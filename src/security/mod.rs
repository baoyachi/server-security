mod ip;

use crate::config::init_conf;
use crate::proxy::{CondType, Proxy, ValidateType};
use crate::security::ip::IpServer;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Server {
    proxy: Proxy,
    sender: Sender<String>,
    path: String,
    ip_server: IpServer,
}

impl Server {
    pub async fn start(path: String, ip_limit: usize) -> anyhow::Result<()> {
        let (sender, rev) = channel::<String>(10);
        let proxy = Proxy::new();
        let mut server = Server {
            proxy,
            sender,
            path,
            ip_server: IpServer::new(ip_limit),
        };
        server.start_inner(rev).await
    }

    pub async fn start_inner(&self, mut rev: Receiver<String>) -> anyhow::Result<()> {
        let config = init_conf(&self.path)?;
        tokio::spawn(async move {
            while let Some(ip) = rev.recv().await {
                // self.ip_server.add_ip(ip).unwrap();
            }
        });
        &self
            .proxy
            .new_proxy(
                config.proxy,
                |addr| async move {
                    println!("addr:{:?}", addr);
                    let peer_ip = addr.ip().to_string();
                    self.sender.send(peer_ip.clone()).await?;
                    // println!("peer_ip:{:?}",&self.ip_server.get_ip(&peer_ip));
                    Ok(ValidateType::Normal)
                },
                |_| Ok(CondType::Continue),
            )
            .await?;
        Ok(())
    }
}
