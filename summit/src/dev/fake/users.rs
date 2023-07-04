use super::user::FakeUserRt;
use crate::Summit;
use anyhow::anyhow;
use clap::Parser;
use fake::{Fake, Faker};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing::{debug_span, error, info, warn, Instrument};

#[derive(Parser, Debug, Default, Clone)]
pub struct FakeUserInitConfig {
    /// The number of fake users to create at startup.
    ///
    /// The higher this number is the more direct user load there will be, so balance this with
    /// other types of fake activity.
    ///
    /// See also [`FakeConfig`](crate::dev::fake).
    #[arg(long, default_value_t = 1)]
    pub fake_count: u16,
    /// The delay on startup user creation, if [`Self::fake_user_startup_count`] is > 0.
    #[arg(long, default_value_t = 0)]
    pub start_on_init_delay_ms: u64,
    /// The maximum delay in seconds that fake users will end up with.
    #[arg(long, default_value_t = 60)]
    pub rate_of_actions_secs_max: u64,
    /// Advance the ticks of any active fake users at startup. This can be delayed via
    /// [`Self::start_on_init_delay_ms`].
    #[arg(long, default_value_t = 0)]
    pub ff_ticks: u64,
    /// In milliseconds, interval duration each tick must take to run, at a minimum. Ticks may
    /// exceed this time.
    #[arg(long, default_value_t = 2500)]
    pub tick_dur: u64,
    /// Don't start the fake user runtime, but users may still be created or advanced.
    #[arg(long)]
    pub dont_start_runtime: bool,
}
impl FakeUserInitConfig {
    /// Initialize fake users over the given Summit instance.
    //
    // TODO: Include stop channel.
    pub async fn init(&self, summit: Arc<Summit>) -> Arc<FakeUsers> {
        let f = Arc::new(FakeUsers::new(summit, &self));
        if self.fake_count > 0 {
            let config = self.clone();
            let f = Arc::clone(&f);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(config.start_on_init_delay_ms)).await;
                for fake_user_index in 0..config.fake_count {
                    if let Err(err) = f.new_user().await {
                        warn!(
                            fake_user_index,
                            out_of = config.fake_count,
                            ?err,
                            "failed to create fake user at startup"
                        );
                    }
                }
                if let Err(err) = f.fastforward_runtime(config.ff_ticks).await {
                    error!(?err, "advancing fake user runtime failed");
                }
                if !config.dont_start_runtime {
                    if let Err(err) = f.start_runtime(config.tick_dur).await {
                        error!(?err, "fake user runtime exited");
                    } else {
                        error!("fake user runtime exited");
                    }
                }
            });
        }
        f
    }
}

pub struct FakeUsers(Mutex<FakeUsersInner>);
impl FakeUsers {
    pub fn new(summit: Arc<Summit>, config: &FakeUserInitConfig) -> Self {
        Self(
            FakeUsersInner {
                user_creation_rng: Box::new(StdRng::seed_from_u64(0xDEADBEEF)),
                config: config.clone(),
                count: 0,
                summit,
                users: Default::default(),
            }
            .into(),
        )
    }
    pub async fn new_user(&self) -> anyhow::Result<()> {
        self.0
            .try_lock()
            // nuke the mutex error lifetime.
            .map_err(|err| anyhow!("{err:?}"))?
            .new_user();
        Ok(())
    }
    pub async fn fastforward_runtime(&self, ff_by_ticks: u64) -> anyhow::Result<()> {
        if ff_by_ticks == 0 {
            return Ok(());
        }
        info!(ff_by_ticks, "fast forwarding fake user runtime");
        let mut users = self.0.lock().await;
        for tick in 0..ff_by_ticks {
            users
                .tick_users(tick)
                .instrument(debug_span!("ff tick", tick))
                .await?;
        }
        Ok(())
    }
    // NIT: Make this runtime move..? I don't want it to start twice, iirc it was shared to pass
    // state to the web server for remote control, but a shutdown channel is probably all that's
    // necesary.
    pub async fn start_runtime(&self, tick_rate_ms: u64) -> anyhow::Result<()> {
        warn!(tick_rate_ms, "starting fake user runtime");
        let tick_rate = Duration::from_millis(tick_rate_ms);
        let mut prev_tick = Instant::now();
        for tick in 0.. {
            let res: anyhow::Result<()> = async {
                self.0.lock().await.tick_users(tick).await?;
                let now = Instant::now();
                let elapsed = now.duration_since(prev_tick);
                if let Some(wait_for) = tick_rate.checked_sub(elapsed) {
                    tokio::time::sleep(wait_for).await;
                }
                prev_tick = now;
                Ok(())
            }
            .instrument(debug_span!("rt tick", tick))
            .await;
            res?;
        }
        Ok(())
    }
}
struct FakeUsersInner {
    user_creation_rng: Box<dyn RngCore + Send + Sync>,
    config: FakeUserInitConfig,
    count: u64,
    summit: Arc<Summit>,
    users: Vec<FakeUserRt<StdRng>>,
}
impl FakeUsersInner {
    pub fn new_user(&mut self) {
        // TODO: Track users for management. For now just spinning them up and wishing them well.
        let fake_user_index = self.count;
        self.count += 1;
        let new_user_seed: u64 = Faker.fake_with_rng(&mut self.user_creation_rng);
        let user = FakeUserRt::new(
            StdRng::seed_from_u64(new_user_seed),
            Arc::clone(&self.summit),
            NewFakeUser {
                config: self.config.clone(),
                fake_user_index,
            },
        );
        warn!(
            fake_user = ?user.fake_user.user.fedi_addr.format(),
            tick_rate = user.fake_user.tick_rate,
            "creating fake user",
        );
        self.users.push(user);
    }
    pub async fn tick_users(&mut self, tick: u64) -> anyhow::Result<()> {
        for user in self.users.iter_mut() {
            user.tick(tick).await?;
        }
        Ok(())
    }
}

pub struct NewFakeUser {
    pub config: FakeUserInitConfig,
    pub fake_user_index: u64,
}
