use super::text::{
    locale::Locale,
    markdown::{Paragraph, Sentence},
};
use crate::{
    db::{Author, CreatePost},
    Summit,
};
use fake::{Dummy, Fake, Faker};
use std::sync::Arc;
use tracing::warn;

// temp compat.
pub use super::users::*;

impl Dummy<Locale> for Author {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(&locale: &Locale, rng: &mut R) -> Self {
        Self {
            fedi_addr: locale.fake_with_rng(rng),
        }
    }
}
#[derive(Debug)]
pub struct FakeUserRt<R> {
    pub rng: R,
    pub summit: Arc<Summit>,
    pub fake_user: FakeUser,
    pub next_tick: u64,
}
impl<R: rand::Rng> FakeUserRt<R> {
    /// Construct a new user with fake parameters generated from the given seed.
    pub fn new(mut rng: R, summit: Arc<Summit>, new_fake_user: NewFakeUser) -> Self {
        let fake_user: FakeUser = new_fake_user.fake_with_rng(&mut rng);
        let next_tick = fake_user.tick_rate;
        Self {
            rng,
            summit,
            fake_user,
            next_tick,
        }
    }
    pub async fn tick(&mut self, tick: u64) -> crate::Result<()> {
        if tick < self.next_tick {
            return Ok(());
        }
        self.next_tick += self.fake_user.tick_rate;
        if let Err(err) = self.action().await {
            warn!(?err, "encountered error faking user action");
        }
        return Ok(());
    }
    async fn action(&mut self) -> crate::Result<()> {
        self.summit
            .create_post(FakeCreatePost(&self.fake_user).fake_with_rng(&mut self.rng))
            .await?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct FakeUser {
    pub locale: Locale,
    pub user: Author,
    pub tick_rate: u64,
}
impl Dummy<NewFakeUser> for FakeUser {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &NewFakeUser, rng: &mut R) -> Self {
        let locale: Locale = {
            // Always gen the locale, keeping rng consistent despite config.
            let locale = Faker.fake_with_rng(rng);
            if config.fake_user_index == 0 {
                // The default for testing, should prob make this configurable but this works for
                // now.
                Locale::EnBob
            } else {
                locale
            }
        };
        let user = locale.fake_with_rng(rng);
        let rate_of_actions_frac: f32 = (0.01..1.0).fake_with_rng(rng);
        // First, calculate the secs based on the above fraction range. This makes it so that you
        // can configure (CLI/etc) the upper bound, ie the amount of spammy, without affecting which
        // users are spammy, which are slow, etc. Slow and spammy is all relative to the range.
        let rate_of_actions_secs =
            (config.config.rate_of_actions_secs_max as f32 * rate_of_actions_frac).clamp(
                // ensure we never go below 1s spam
                1.,
                // Next, to help ensure the first users are spammy for testing, we apply a cap
                // where as each user is created they're affected less and
                // less by the cap.
                //
                // +1 to one-index the math.
                5.0 * (config.fake_user_index + 1) as f32,
            );
        let tick_rate = (rate_of_actions_secs as f32 * 1000. / config.config.tick_dur.max(1) as f32)
            .round() as u64;
        Self {
            locale,
            user,
            tick_rate,
        }
    }
}

pub struct FakeCreatePost<'a>(&'a FakeUser);
impl Dummy<FakeCreatePost<'_>> for CreatePost {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(
        &FakeCreatePost(fake_user): &FakeCreatePost<'_>,
        rng: &mut R,
    ) -> Self {
        let &FakeUser { locale, .. } = fake_user;
        Self {
            author: fake_user.user.clone(),
            title: Sentence(locale, 1..2).fake_with_rng::<String, _>(rng),
            body: Paragraph(locale, 1..=10).fake_with_rng::<String, _>(rng),
        }
    }
}
