use crate::{
    db::{CreatePost, FediAddr, User},
    Summit,
};
use clap::Parser;
use fake::{faker::lorem::en::Words, Dummy, Fake};
use std::{sync::Arc, time::Duration};
use tracing::warn;

use super::text::{Locale, Name};

#[derive(Parser, Debug, Default, Clone)]
pub struct FakeUserInitConfig {
    /// The number of fake users to create at startup.
    ///
    /// The higher this number is the more direct user load there will be, so balance this with
    /// other types of fake activity.
    ///
    /// See also [`FakeConfig`](crate::dev::fake).
    #[arg(long, default_value_t = 1)]
    pub fake_user_count: u16,
    /// Start fake users on creation, after [`Self::start_on_init_delay`].
    #[arg(long, default_value_t = true)]
    pub start_on_init: bool,
    #[arg(long, default_value_t = 3)]
    pub start_on_init_delay_secs: u64,
}
impl FakeUserInitConfig {
    /// Initialize fake users over the given Summit instance.
    //
    // TODO: Include stop channel.
    pub async fn init(&self, summit: Arc<Summit>) -> Arc<FakeUsers> {
        let f = Arc::new(FakeUsers::new(summit));
        if self.start_on_init && self.fake_user_count > 0 {
            let f = Arc::clone(&f);
            tokio::spawn(async move { f.run().await });
        }
        f
    }
}

#[derive(Debug)]
pub struct FakeUsers {
    summit: Arc<Summit>,
}
impl FakeUsers {
    pub fn new(summit: Arc<Summit>) -> Self {
        Self { summit }
    }
    pub async fn run(&self) {
        warn!("starting fake user..");
        loop {
            // TODO: randomize delay. Probably centralize some random callback behavior?
            tokio::time::sleep(Duration::from_secs(5)).await;
            if let Err(err) = self.fake_user_action().await {
                warn!(?err, "encountered error faking user action");
            }
        }
    }
    async fn fake_user_action(&self) -> crate::Result<()> {
        self.summit
            .create_post(FakeCreatePost(Locale::En.fake()).fake())
            .await?;
        Ok(())
    }
}
impl Dummy<Locale> for User {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(&locale: &Locale, rng: &mut R) -> Self {
        Self {
            fedi_addr: locale.fake_with_rng(rng),
        }
    }
}
#[derive(Debug, Default)]
pub struct FakeUser {
    pub locale: Locale,
    pub user: User,
}
impl Dummy<Locale> for FakeUser {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(&locale: &Locale, rng: &mut R) -> Self {
        Self {
            locale,
            user: locale.fake_with_rng(rng),
        }
    }
}

pub struct FakeCreatePost(pub FakeUser);
impl Dummy<FakeCreatePost> for CreatePost {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FakeCreatePost, rng: &mut R) -> Self {
        Self {
            author: config.0.user.clone(),
            title: Words(1..10).fake_with_rng::<Vec<String>, _>(rng).join(" "),
            body: Words(2..1_000)
                .fake_with_rng::<Vec<String>, _>(rng)
                .join(" "),
        }
    }
}
