use crate::web::graceful_shutdown::GracefulShutdown;
use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use std::{convert::Infallible, time::Duration};
use tracing::{info, trace, Span};

pub async fn live_handler(
    State(signal): State<GracefulShutdown>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("starting sse connection");

    let start = std::time::Instant::now();
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        let mut conn_guard = ConnectionGuard {
            span: Span::current(),
            closed_via_signal: false,
        };
        loop {
            tokio::select! {
                _ = interval.tick() => {
                yield Ok(Event::default().event("newCommunityPost").data(format!(
                    "<div>foo {:?}</div>",
                    std::time::Instant::now().duration_since(start)
                )));
                }
                _ = &mut std::pin::pin!(signal.0.recv()) => {
                    conn_guard.closed_via_signal = true;
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
/// A guard to report when a SSE Stream has closed, and and metadata we attach to that stream.
struct ConnectionGuard {
    span: Span,
    closed_via_signal: bool,
}
impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        // NOTE: Not sure if this is correct usage for async<->sync boundaries, but it preserves
        // the span info like requestId, which is what i'm mostly after.
        let _enter = self.span.enter();
        trace!(
            closed_via_signal = self.closed_via_signal,
            "closing sse stream, connection dropped"
        );
    }
}