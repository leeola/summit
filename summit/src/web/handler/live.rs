use crate::{
    date_time::TimeZone,
    uuid::{RequestId, UserId},
    web::{handler::community::CommunityPost, shutdown::ShutdownSignal},
    Summit,
};
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Extension,
};
use futures::stream::Stream;
use kanal::ReceiveError;
use sailfish::{RenderError, TemplateOnce};
use std::{convert::Infallible, pin::pin, sync::Arc, time::Duration};
use thiserror::Error;
use tracing::{error, info, trace, Span};

#[derive(Debug, Error)]
enum EventError {
    #[error("receiving event")]
    Receive(#[from] ReceiveError),
    #[error("rendering event template")]
    Render(#[from] RenderError),
}

pub async fn live_handler(
    State((summit, shutdown_signal)): State<(Arc<Summit>, ShutdownSignal)>,
    Extension(_req_id): Extension<RequestId>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // No users are registered yet. Use a default.
    let user_id = UserId::default();
    let user_tz = TimeZone::default();

    info!(%user_id, "starting sse connection");
    let user_events = summit.user_events(user_id);

    let stream = async_stream::stream! {
        let mut conn_guard = ConnectionGuard {
            span: Span::current(),
            closed_via_shutdown_signal: false,
        };
        loop {
            tokio::select! {
                event_res = &mut pin!(user_events.recv()) => {
                    let res = event_res.map_or_else(
                        |err| Err(EventError::from(err)),
                        |post| -> Result<_, EventError> {
                            // For now "events" are just posts.
                            Ok(CommunityPost::new(user_tz, post).render_once()?)
                        },
                    );
                    match res {
                        Ok(rendered_event_html) => {
                            yield Ok(
                                Event::default()
                                    .event("newCommunityPost")
                                    .data(rendered_event_html)
                            );
                        }
                        Err(err) => {
                            error!(?err, "broadcasting user event failed");
                        }
                    }
                }
                () = &mut pin!(shutdown_signal.recv()) => {
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
/// A guard to report when a SSE Stream has closed, and and metadata we attach to that stream.
pub(super) struct ConnectionGuard {
    pub span: Span,
    pub closed_via_shutdown_signal: bool,
}
impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        // NOTE: Not sure if this is correct usage for async<->sync boundaries, but it preserves
        // the span info like requestId, which is what i'm mostly after.
        let _enter = self.span.enter();
        trace!(
            closed_via_signal = self.closed_via_shutdown_signal,
            "closing sse stream, connection dropped"
        );
    }
}
