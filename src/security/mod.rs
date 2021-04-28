mod ip;

use crate::config::init_conf;
use crate::proxy::{CondType, Proxy, ValidateType};
use crate::security::ip::IpServer;
use std::net::SocketAddr;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Server {
    proxy: Proxy,
    sender: Sender<String>,
    path: String,
}

impl Server {
    pub async fn start(path: String, ip_limit: usize) -> anyhow::Result<()> {
        let (sender, rev) = channel::<String>(10);
        let ip_server = IpServer::new(ip_limit);
        let proxy = Proxy::new();
        let mut server = Server {
            proxy,
            sender,
            path,
        };
        server.start_inner(rev, ip_server).await
    }

    pub async fn start_inner(
        &self,
        mut rev: Receiver<String>,
        mut ip_server: IpServer,
    ) -> anyhow::Result<()> {
        let config = init_conf(&self.path)?;
        tokio::spawn(async move {
            while let Some(ip) = rev.recv().await {
                ip_server.add_ip(ip).unwrap();
            }
        });
        &self
            .proxy
            .new_proxy(config.proxy, Self::validate, Self::callback)
            .await?;
        Ok(())
    }

    pub async fn validate(addr: SocketAddr) -> anyhow::Result<ValidateType> {
        println!("addr:{:?}", addr);
        let peer_ip = addr.ip().to_string();
        // self.sender.send(peer_ip.clone()).await?;
        // println!("peer_ip:{:?}",&self.ip_server.get_ip(&peer_ip));
        Ok(ValidateType::Normal)
    }

    pub fn callback(err: anyhow::Error) -> anyhow::Result<CondType> {
        Ok(CondType::Continue)
    }
}
