use super::user::FakeUserRt;
use crate::Summit;
use anyhow::anyhow;
use clap::Parser;
use fake::{Fake, Faker};
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tracing::warn;

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
    #[arg(long, default_value_t = 2)]
    pub start_on_init_delay_secs: u64,
    /// The maximum delay in seconds that fake users will end up with.
    #[arg(long, default_value_t = 60)]
    pub rate_of_actions_secs_max: u64,
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
                tokio::time::sleep(Duration::from_secs(config.start_on_init_delay_secs)).await;
                for fake_user_index in 0..config.fake_count {
                    if let Err(err) = f.new_and_start().await {
                        warn!(
                            fake_user_index,
                            out_of = config.fake_count,
                            ?err,
                            "failed to create fake user at startup"
                        );
                    }
                    // A slight pause to make initial user creation log readable.. ish.
                    tokio::time::sleep(Duration::from_millis(100)).await;
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
            }
            .into(),
        )
    }
    pub async fn new_and_start(&self) -> anyhow::Result<()> {
        // No reason to do any "fancy" locking here, a simple try-lock will suffice i imagine,
        // failing to spin up a fake user in the (currently impossible) occurance of contention on
        // inner.
        let fake_user = {
            self.0
                .try_lock()
                // nuke the mutex error lifetime.
                .map_err(|err| anyhow!("{err:?}"))?
                .new_user()
        };
        tokio::spawn(async move { fake_user.start().await });
        Ok(())
    }
}
struct FakeUsersInner {
    user_creation_rng: Box<dyn RngCore + Send + Sync>,
    config: FakeUserInitConfig,
    count: u64,
    summit: Arc<Summit>,
}
impl FakeUsersInner {
    pub fn new_user(&mut self) -> FakeUserRt<impl Rng> {
        // TODO: Track users for management. For now just spinning them up and wishing them well.
        self.count += 1;
        let new_user_seed: u64 = Faker.fake_with_rng(&mut self.user_creation_rng);
        FakeUserRt::new(
            StdRng::seed_from_u64(new_user_seed),
            Arc::clone(&self.summit),
            NewFakeUser {
                config: self.config.clone(),
                fake_user_count: self.count,
            },
        )
    }
}

pub struct NewFakeUser {
    pub config: FakeUserInitConfig,
    pub fake_user_count: u64,
}
