use crate::{
    web::{
        extension::request_id::{self, RequestIdLayer},
        shutdown::ShutdownSignal,
    },
    Summit,
};
use axum::{routing::get, Router};
use clap::Parser;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

mod extension;
pub mod handler;
mod shutdown;
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
    let shutdown_signal = ShutdownSignal::new().await;
    let app = Router::new()
        .route("/c/", get(handler::community::handler))
        .route(
            "/live",
            get(handler::live::live_handler).with_state((summit.clone(), shutdown_signal.clone())),
        )
        .route("/static/*key", get(handler::static_assets::serve_asset))
        .with_state(summit);
    #[cfg(feature = "local_dev")]
    let app = app.route(
        "/dev/watch-restart",
        get(handler::dev::restart::restart_handler).with_state(shutdown_signal.clone()),
    );
    // FIXME: I think this doesn't work with Axum handlers. Need to move this to whatever handler
    // consumes this in the future, most likely some admin endpoint.
    #[cfg(any(test, feature = "dev"))]
    let app = app.with_state(fake);
    let app = app
        .layer(TraceLayer::new_for_http().make_span_with(request_id::trace_layer_span_with))
        .layer(RequestIdLayer);

    let ServeConfig { host, port } = config;
    let listen_addr = format!("{host}:{port}");
    info!(listen_addr, "starting server..");
    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move { shutdown_signal.recv().await })
        .await
}
