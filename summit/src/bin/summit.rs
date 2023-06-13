use clap::Parser;
use summit::http::ServeConfig;
use tracing::{metadata::LevelFilter, subscriber};
use tracing_subscriber::EnvFilter;

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

    let config = ServeConfig::parse();
    summit::serve(config).await
}
