use crate::{server::Summit, web::graceful_shutdown::GracefulShutdown};
use axum::{http::Request, routing::get, Router};
use clap::Parser;
use hyper::Body;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{error_span, info};

mod graceful_shutdown;
pub mod handler;
pub mod template;

#[derive(Parser, Debug, Default)]
pub struct ServeConfig {
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
}

pub async fn serve(
    config: ServeConfig,
    summit: Arc<Summit>,
    // TODO: Probably worth it to make a server instance and include `with_fake()` to modify the
    // router, avoiding this nonsense.
    #[cfg(any(test, feature = "dev"))] fake: Arc<crate::dev::fake::user::FakeUsers>,
) -> Result<(), hyper::Error> {
    let (receiver, signal) = GracefulShutdown::new();
    let app = Router::new()
        .route("/c/", get(handler::community::handler))
        .route(
            "/live",
            get(handler::live::live_handler).with_state(receiver),
        )
        .route("/static/*key", get(handler::static_assets::serve_asset))
        .with_state(summit);
    #[cfg(any(test, feature = "dev"))]
    let app = app.with_state(fake);
    let app = app
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
        .with_graceful_shutdown(signal)
        .await
}
