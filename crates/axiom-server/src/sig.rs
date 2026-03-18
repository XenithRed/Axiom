use tokio::sync::watch;

pub struct Shutdown {
    a: watch::Sender<bool>,
    b: watch::Receiver<bool>,
}

impl Shutdown {
    pub fn new() -> Self {
        let (a, b) = watch::channel(false);
        Self { a, b }
    }

    pub fn subscribe(&self) -> watch::Receiver<bool> {
        self.b.clone()
    }

    pub fn trigger(&self) {
        let _ = self.a.send(true);
    }

    pub async fn wait_for_signal(self) {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sigterm = signal(SignalKind::terminate())
                .expect("failed to register SIGTERM");
            let mut sigint  = signal(SignalKind::interrupt())
                .expect("failed to register SIGINT");

            tokio::select! {
                _ = sigterm.recv() => { tracing::info!("received SIGTERM"); }
                _ = sigint.recv()  => { tracing::info!("received SIGINT");  }
            }
        }
        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for ctrl-c");
            tracing::info!("received ctrl-c");
        }

        self.trigger();
    }
}

impl Default for Shutdown {
    fn default() -> Self { Self::new() }
}
