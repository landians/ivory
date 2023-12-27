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
        // 只要没有接收到来自 server 广播的 shutdown 消息，则一直运行
        while !self.shutdown.is_shutdown() {}
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Shutdown {
    is_shutdown: bool,

    notify_shutdown: broadcast::Receiver<()>,
}

impl Shutdown {
    pub(crate) fn new(notify_shutdown: broadcast::Receiver<()>) -> Shutdown {
        Shutdown {
            is_shutdown: false,
            notify_shutdown,
        }
    }

    pub(crate) fn is_shutdown(&self) -> bool {
        self.is_shutdown
    }

    pub(crate) async fn recv(&mut self) {
        if self.is_shutdown {
            return;
        }

        let _ = self.notify_shutdown.recv().await;

        self.is_shutdown = true;
    }
}
