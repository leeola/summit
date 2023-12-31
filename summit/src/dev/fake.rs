use self::{
    text::{locale::Locale, FediHostName, FediUserName},
    user::FakeUserInitConfig,
};
use crate::db::FediAddr;
use clap::Parser;
use fake::{Dummy, Fake};

pub mod text;
pub mod user;
pub mod users;

#[derive(Parser, Debug, Default, Clone)]
pub struct FakeConfig {
    #[command(flatten)]
    pub user: FakeUserInitConfig,
}

impl Dummy<Locale> for FediAddr {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(&locale: &Locale, rng: &mut R) -> Self {
        Self {
            user: FediUserName(locale).fake_with_rng(rng),
            host: FediHostName(locale).fake_with_rng(rng),
        }
    }
}
