pub mod ip;

use async_trait::async_trait;

use crate::config::{init_conf, ServerConfig};
use crate::notify::mail::EmailServer;
use crate::proxy::{CheckType, CondType, Policy, Proxy};
use crate::security::ip::{IpServer, Notify};
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
struct SecurityPolicy<T> {
    ip_server: IpServer<T>,
}

#[async_trait]
impl<T> Policy for SecurityPolicy<T>
where
    T: Notify,
{
    async fn check_addr(&mut self, addr: &SocketAddr) -> anyhow::Result<CheckType> {
        self.ip_server.check_addr(addr).await
    }
}

pub struct Server {
    proxy: Proxy<SecurityPolicy<EmailServer>>,
}

impl Server {
    pub async fn start(path: String, ip_limit: usize) -> anyhow::Result<()> {
        let config = init_conf(path)?;

        let policy = config
            .email_server
            .clone()
            .map(|s| IpServer::new(ip_limit, s))
            .map(|ip_server| Some(SecurityPolicy { ip_server }))
            .unwrap_or_default();

        let proxy = Proxy::new(policy);
        let mut server = Server { proxy };
        server.start_inner(config).await
    }

    pub async fn start_inner(&mut self, config: ServerConfig) -> anyhow::Result<()> {
        &self.proxy.new_proxy(config.proxy, Self::callback).await?;
        Ok(())
    }

    pub fn callback(err: anyhow::Error) -> anyhow::Result<CondType> {
        Ok(CondType::Continue)
    }
}
