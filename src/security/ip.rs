use async_trait::async_trait;

use crate::proxy::CheckType;
use chrono::{DateTime, Local, Utc};
use lru::LruCache;
use std::alloc::System;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

#[async_trait]
pub trait Notify: Sync + Send {
    async fn notify(&self, msg: NotifyMsg) -> anyhow::Result<()>;
}

pub struct NotifyMsg {
    pub subject: String,
    pub body: String,
}

#[derive(Debug)]
pub struct IpServer<T> {
    limit: usize,
    cache: LruCache<String, DateTime<Utc>>,
    notify: T,
}

impl<T> IpServer<T>
where
    T: Notify,
{
    pub fn new(limit: usize, notify: T) -> Self {
        IpServer {
            limit,
            cache: LruCache::new(limit),
            notify,
        }
    }

    pub(crate) async fn check_addr(&mut self, addr: &SocketAddr) -> anyhow::Result<CheckType> {
        let ip = addr.ip().to_string();
        &self.add_ip(ip).await?;
        Ok(CheckType::Normal)
    }

    pub async fn add_ip(&mut self, ip: String) -> anyhow::Result<()> {
        if self.cache.len() > self.limit {
            return Err(anyhow!("overflow max ip limit."));
        }
        if self.cache.put(ip.clone(), Utc::now()).is_some() {
            let message = NotifyMsg {
                subject: "add new ip connection".to_string(),
                body: format!("ip:{}", ip),
            };
            self.notify.notify(message).await?;
        }
        Ok(())
    }

    pub fn get_ip(&mut self, ip: &String) -> Option<&DateTime<Utc>>
    where
        T: Notify,
    {
        self.cache.get(ip)
    }
}
