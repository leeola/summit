use axum::{routing::get, Router};
use clap::Parser;
use tokio::signal;
use tracing::info;

/// Simple program to greet a person
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct ServeConfig {
    // #[arg(long, default_value_t = String::from("127.0.0.1"))]
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
}

pub async fn serve(config: ServeConfig) -> Result<(), hyper::Error> {
    // build our application with a single route
    let app = Router::new().route("/", get(root));

    let ServeConfig { host, port } = config;
    let listen_addr = format!("{host}:{port}");
    info!(listen_addr, "starting server..");
    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    println!("signal received, starting graceful shutdown");
}
