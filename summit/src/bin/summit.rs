use std::sync::Arc;

use clap::Parser;
use summit::{db::DbConfig, server::Summit, web::ServeConfig};
use tracing::{metadata::LevelFilter, subscriber};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    #[command(flatten)]
    pub db: DbConfig,
    #[command(flatten)]
    pub serve: ServeConfig,
    #[cfg(any(test, feature = "dev"))]
    #[command(flatten)]
    pub fake: summit::dev::fake::user::FakeUserInitConfig,
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // TODO: Move logging init to a core utility, for ease of test setup.
    subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .with_env_var("RUST_LOG")
                    .from_env_lossy(),
            )
            .finish(),
    )
    .unwrap();

    let config = CliConfig::parse();
    let summit = Arc::new(Summit::new(config.db.init()));
    #[cfg(any(test, feature = "dev"))]
    {
        tracing::info!("running with dev");
        let fake = config.fake.init(Arc::clone(&summit)).await;
        summit::web::serve(config.serve, summit, fake).await
    }
    #[cfg(not(any(test, feature = "dev")))]
    summit::web::serve(config.serve, summit).await
}
