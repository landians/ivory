use std::future::Future;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Semaphore};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use tracing::{error, info};
use crate::error::Error;

pub struct Server {
    // Listen address.
    address: String,

    // Limit the max number of connections.
    max_connections: Arc<Semaphore>,
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
            None => Arc::new(Semaphore::new(usize::MAX))
        };

        Server {
            address: self.address,
            max_connections,
        }
    }
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }

    pub async fn serve(&self, shutdown: impl Future) -> Result<(), Error>{
        // When the provided `shutdown` future completes, we must send a shutdown
        // message to all active connections. We use a broadcast channel for this
        // purpose. The call below ignores the receiver of the broadcast pair, and when
        // a receiver is needed, the subscribe() method on the sender is used to create
        // one.
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

    async fn run(&self, notify_shutdown: &broadcast::Sender<()>, shutdown_complete_tx: &mpsc::Sender<()>) -> Result<(), Error>{
        loop {
            // Wait for a permit to become available
            //
            // `acquire_owned` returns a permit that is bound to the semaphore.
            // When the permit value is dropped, it is automatically returned
            // to the semaphore.
            //
            // `acquire_owned()` returns `Err` when the semaphore has been
            // closed. We don't ever close the semaphore, so `unwrap()` is safe.
            let permit = self
                .max_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();
        }

        // Accept a new socket. This will attempt to perform error handling.
        // The `accept` method internally attempts to recover errors, so an
        // error here is non-recoverable.
        let socket = self.accept().await?;



        Ok(())
    }

    async fn accept(&self) -> Result<TcpStream, Error> {
        let listener = TcpListener::bind(&self.address).await?;

        info!("RPC Server is listening on {} ......", self.address);

        let mut backoff = 1;

        // Try to accept a few times
        loop {
            // Perform the accept operation. If a socket is successfully
            // accepted, return it. Otherwise, save the error.
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("Client: {} connected", addr);

                    return Ok(socket)
                },
                Err(err) => {
                    if backoff > 64 {
                        // Accept has failed too many times. Return the error.
                        return Err(err.into());
                    }
                }
            }

            // Pause execution until the back off period elapses.
            time::sleep(Duration::from_secs(backoff)).await;

            // Double the back off
            backoff *= 2;
        }
    }
}