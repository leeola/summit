use crate::web::{handler::live::ConnectionGuard, shutdown::ShutdownSignal};
use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use std::{convert::Infallible, pin::pin, time::Duration};
use tracing::{trace, Span};

pub async fn restart_handler(
    State(shutdown_signal): State<ShutdownSignal>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    trace!("opening dev restart sse connection");

    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        let mut conn_guard = ConnectionGuard {
            span: Span::current(),
            closed_via_shutdown_signal: false,
        };
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    yield Ok(Event::default().event("restart heartbeat"));
                }
                _ = &mut pin!(shutdown_signal.recv()) => {
                    conn_guard.closed_via_shutdown_signal = true;
                    return;
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
