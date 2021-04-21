use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct SocketConfig {
    pub server_addr: String,
    pub to_addr: String,
}

pub enum ValidateType {
    Normal,
    Warning,
    Forbidden,
}

pub async fn new_proxy<F>(config: SocketConfig, validate: F) -> anyhow::Result<()>
where
    F: Fn(&SocketAddr) -> anyhow::Result<ValidateType>,
{
    let listen_addr = config.server_addr.clone();
    let to_addr = config.to_addr;
    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, remote_addr)) = listener.accept().await {
        if let Err(err) = validate(&remote_addr) {
            error!("validate error={}", err);
        } else {
            debug!("remote_addr:{}", remote_addr.to_string());
            let transfer = transfer(inbound, to_addr.clone()).map(|r| {
                if let Err(e) = r {
                    error!("Failed to transfer; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
    }

    Ok(())
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
