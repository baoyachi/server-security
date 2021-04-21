use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use arc_swap::access::DynAccess;
use arc_swap::ArcSwap;
use futures::FutureExt;
use once_cell::sync::Lazy;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

pub struct SocketConfig {
    pub listen_addr: String,
    pub to_addr: String,
}

static IP_TABLE: Lazy<RwLock<HashMap<String, ()>>> = Lazy::new(|| Default::default());

pub fn validate(remote_addr: &SocketAddr) -> anyhow::Result<()> {
    let remote_ip = remote_addr.ip().to_string();
    let mut guard = IP_TABLE.write().unwrap();
    
    if guard.get(&remote_ip).is_some() {
        return Ok(());
    }
    if guard.len() > 0 {
        //TODO change configuration
        return anyhow::bail!("{} not exit error", remote_ip);
    }
    guard.insert(remote_ip, ());

    Ok(())
}

pub async fn new_proxy(config: SocketConfig) -> anyhow::Result<()> {
    let listen_addr = config.listen_addr.clone();
    let to_addr = config.to_addr;
    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, remote_addr)) = listener.accept().await {
        if let Err(err) = validate(&remote_addr) {
            error!("validate error={}", err);
        } else {
            info!("remote_addr:{}",remote_addr.to_string());
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

