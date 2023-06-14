use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use std::{convert::Infallible, path::PathBuf, time::Duration};
use tokio_stream::StreamExt as _;
use tracing::info;

pub async fn live_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("starting sse connection");

    let start = std::time::Instant::now();

    let stream = stream::repeat_with(move || {
        info!("making event?");
        Event::default().event("newCommunityPost").data(format!(
            "<div>foo {:?}</div>",
            std::time::Instant::now().duration_since(start)
        ))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

// <div hx-sse="swap:newCommunityPost">
// </div>
