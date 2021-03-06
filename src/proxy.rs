use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use async_trait::async_trait;

use chrono::{Date, DateTime, Utc};
use futures::{Future, FutureExt};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Deserialize)]
pub struct SocketConfig {
    pub server_addr: String,
    pub to_addr: String,
}

pub enum CheckType {
    Normal,
    Stop,
}

pub enum CondType {
    Continue,
    Stop,
}

trait AutoValue {}

#[async_trait]
pub trait Policy {
    async fn check_addr(&mut self, addr: &SocketAddr) -> anyhow::Result<CheckType>;
}

#[derive(Debug)]
pub struct Proxy<T> {
    policy: Option<T>,
}

impl<T> Proxy<T> {
    pub fn new(policy: Option<T>) -> Self {
        Proxy { policy }
    }

    pub async fn new_proxy<C>(&mut self, config: SocketConfig, callback: C) -> anyhow::Result<()>
    where
        T: Policy,
        C: Fn(anyhow::Error) -> anyhow::Result<CondType> + Sync + Send + Copy + 'static,
    {
        let listen_addr = config.server_addr.clone();
        let to_addr = config.to_addr;
        let listener = TcpListener::bind(listen_addr).await?;

        while let Ok((inbound, remote_addr)) = listener.accept().await {
            if let Some(policy) = self.policy.as_mut() {
                if let Err(err) = policy.check_addr(&remote_addr).await {
                    error!("validate error:{}", err);
                }
            } else {
                debug!("remote_addr:{}", remote_addr.to_string());
                let transfer = transfer(inbound, to_addr.clone()).map(move |r| {
                    if let Err(e) = r {
                        match callback(e) {
                            Ok(CondType::Stop) => {
                                warn!("system call stop");
                                return;
                            }
                            Err(e) => {
                                error!("Failed to transfer error:{}", e);
                            }
                            _ => {}
                        }
                    }
                });

                tokio::spawn(transfer);
            }
        }
        Ok(())
    }
}

async fn transfer(mut inbound: TcpStream, to_addr: String) -> anyhow::Result<()> {
    let mut outbound = TcpStream::connect(to_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
