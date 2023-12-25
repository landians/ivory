use crate::connection::Connection;
use crate::error::Error;
use crate::handler::{Handler, Shutdown};
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Semaphore};
use tokio::time::{self, Duration};
use tracing::{error, info};

#[derive(Debug)]
pub struct Server {
    // Listen address.
    address: String,

    // Limit the max number of connections.
    max_connections: Arc<Semaphore>,

    current_connections: AtomicUsize,
}

#[derive(Debug)]
pub struct ServerBuilder {
    address: String,
    max_connections: Option<usize>,
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        ServerBuilder {
            address: "127.0.0.1:8989".to_string(),
            max_connections: None,
        }
    }

    pub fn address(mut self, address: &str) -> ServerBuilder {
        self.address = address.to_string();
        self
    }

    pub fn max_connections(mut self, n: usize) -> ServerBuilder {
        self.max_connections = Some(n);
        self
    }

    pub fn build(self) -> Server {
        let max_connections = match self.max_connections {
            Some(n) => Arc::new(Semaphore::new(n)),
            None => Arc::new(Semaphore::new(usize::MAX)),
        };

        Server {
            address: self.address,
            max_connections,
            current_connections: AtomicUsize::new(0),
        }
    }
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }

    pub async fn serve(&self, shutdown: impl Future) -> Result<(), Error> {
        let (notify_shutdown, _) = broadcast::channel(1);
        let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

        tokio::select! {
            res = self.run(&notify_shutdown, &shutdown_complete_tx) => {
                if let Err(err) = res {
                    error!(cause = %err, "failed to accept");
                }
            },
            // 接收Ctrl+c SIGINT
            _ = shutdown => {
                info!("RPC Server is shutting down!!!");
            }
        }

        drop(notify_shutdown);
        drop(shutdown_complete_tx);

        let _ = shutdown_complete_rx.recv().await;

        Ok(())
    }

    async fn run(
        &self,
        notify_shutdown: &broadcast::Sender<()>,
        shutdown_complete_tx: &mpsc::Sender<()>,
    ) -> Result<(), Error> {
        loop {
            let permit = self.max_connections.clone().acquire_owned().await.unwrap();

            let socket = self.accept().await?;

            let mut handler = Handler::new(
                Connection::new(socket),
                Shutdown::new(notify_shutdown.subscribe()),
                shutdown_complete_tx.clone(),
            );

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!(cause = ?err, "connection error");
                }

                drop(permit);
            });
        }
    }

    async fn accept(&self) -> Result<TcpStream, Error> {
        let listener = TcpListener::bind(&self.address).await?;

        info!("RPC Server is listening on {} ......", self.address);

        let mut backoff = 1;

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("Client: {} connected", addr);

                    return Ok(socket);
                }
                Err(err) => {
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;

            backoff *= 2;
        }
    }

    pub fn current_connections(&self) -> usize {
        self.current_connections.load(Ordering::Relaxed)
    }
}
