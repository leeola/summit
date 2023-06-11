use axum::{routing::get, Router};
use clap::Parser;
use http::Request;
use hyper::Body;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{error_span, info};

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
        .route("/c/", get(handler::community::handler))
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                error_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        // This layer creates a new id for each request and puts it into the request extensions.
        // Note that it should be added after the Trace layer.
        .layer(RequestIdLayer);

    let ServeConfig { host, port } = config;
    let listen_addr = format!("{host}:{port}");
    info!(listen_addr, "starting server..");
    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal())
        .await
}

async fn root() -> &'static str {
    info!("hello");
    "Hello, World!"
}
