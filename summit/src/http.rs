use axum::{routing::get, Router};
use clap::Parser;
use tracing::info;

mod graceful_shutdown;
pub mod handler;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct ServeConfig {
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
}

pub async fn serve(config: ServeConfig) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(root))
        .route("/c/", get(handler::community::handler));

    let ServeConfig { host, port } = config;
    let listen_addr = format!("{host}:{port}");
    info!(listen_addr, "starting server..");
    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal())
        .await
}

async fn root() -> &'static str {
    "Hello, World!"
}
