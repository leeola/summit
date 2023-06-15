use futures::{Future, FutureExt};
use kanal::AsyncReceiver;
use tokio::signal;
use tracing::{error, info};

/// A shutdown signal indicating that the server itself is being shutdown, so connections should be
/// closed, state should be persisted (if needed), etc. Of course, do not rely on this signal to
/// ensure validity of state, as it is only an indicator to a request. The server may still die at
/// any moment.
#[derive(Clone)]
pub struct ShutdownSignal(AsyncReceiver<()>);
impl ShutdownSignal {
    /// Construct a new [`ShutdownSignal`] and **start a background task** which listens for
    /// standard OS signals.
    ///
    /// # Panics
    /// If [`tokio::signal`] is not able to construct various terminator signal watchers.
    pub async fn new() -> Self {
        let (sender, receiver) = kanal::bounded_async::<()>(2);
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(async move {
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
            info!("shutdown signal received, propagating..");
            if let Err(err) = sender.send(()).await {
                error!(?err, "failed to propagate shutdown signal");
            }
        });
        Self(receiver)
    }
    /// Return a future that can be `await`ed, where the output indicates a shutdown signal was
    /// sent.
    pub fn recv(&self) -> impl Future<Output = ()> + '_ {
        // NIT: This feels really bad, but many consumers don't have a way to deal with a failed
        // receiving channel. It's possible that we may want to simply suppress errors here,
        // but not actually signal shutdown. Maybe return a never resolving future? Not
        // sure.
        self.0.recv().map(|res| res.unwrap_or(()))
    }
}
