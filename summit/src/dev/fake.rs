use self::user::FakeUserInitConfig;
use clap::Parser;

pub mod user;

#[derive(Parser, Debug, Default, Clone)]
pub struct FakeConfig {
    #[command(flatten)]
    pub user: FakeUserInitConfig,
}
