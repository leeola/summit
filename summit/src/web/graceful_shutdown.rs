use futures::Future;
use kanal::{AsyncReceiver, AsyncSender};
use tokio::signal;

#[derive(Clone)]
pub struct GracefulShutdown(pub AsyncReceiver<()>);
impl GracefulShutdown {
    pub fn new() -> (Self, impl Future<Output = ()>) {
        let (snd, rcv) = kanal::bounded_async::<()>(2);
        (Self(rcv), shutdown_signal(snd))
    }
}
async fn shutdown_signal(sender: AsyncSender<()>) {
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
    tokio::spawn(async move { sender.send(()).await });
    println!("signal sent, actually starting graceful shutdown");
}
