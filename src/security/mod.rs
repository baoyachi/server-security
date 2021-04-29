mod ip;

use async_trait::async_trait;

use crate::config::init_conf;
use crate::proxy::{CheckType, CondType, Policy, Proxy};
use crate::security::ip::IpServer;
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
struct SecurityPolicy {
    ip_server: IpServer,
}

#[async_trait]
impl Policy for SecurityPolicy {
    async fn check_addr(&mut self, addr: SocketAddr) -> anyhow::Result<CheckType> {
        self.ip_server.check_addr(addr).await
    }
}

pub struct Server {
    proxy: Proxy<SecurityPolicy>,
    path: String,
}

impl Server {
    pub async fn start(path: String, ip_limit: usize) -> anyhow::Result<()> {
        let ip_server = IpServer::new(ip_limit);
        let security_policy = SecurityPolicy { ip_server };
        let proxy = Proxy::new(security_policy);
        let mut server = Server { proxy, path };
        server.start_inner().await
    }

    pub async fn start_inner(&mut self) -> anyhow::Result<()> {
        let config = init_conf(&self.path)?;
        &self.proxy.new_proxy(config.proxy, Self::callback).await?;
        Ok(())
    }

    pub fn callback(err: anyhow::Error) -> anyhow::Result<CondType> {
        Ok(CondType::Continue)
    }
}
