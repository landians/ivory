use crate::connection::Connection;
use crate::error::Error;
use tokio::sync::{broadcast, mpsc};

/// Per-connection handler.
#[derive(Debug)]
pub(crate) struct Handler {
    connection: Connection,

    shutdown: Shutdown,

    shutdown_complete: mpsc::Sender<()>,
}

impl Handler {
    pub(crate) fn new(
        connection: Connection,
        shutdown: Shutdown,
        shutdown_complete: mpsc::Sender<()>,
    ) -> Self {
        Handler {
            connection,
            shutdown,
            shutdown_complete,
        }
    }

    pub(crate) async fn run(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Shutdown {
    is_shutdown: bool,

    notify: broadcast::Receiver<()>,
}

impl Shutdown {
    pub(crate) fn new(notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown {
            is_shutdown: false,
            notify,
        }
    }

    pub(crate) fn is_shutdown(&self) -> bool {
        self.is_shutdown
    }

    pub(crate) async fn recv(&mut self) {
        if self.is_shutdown {
            return;
        }

        let _ = self.notify.recv().await;

        self.is_shutdown = true;
    }
}
