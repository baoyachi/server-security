use crate::proxy::CheckType;
use chrono::{DateTime, Local, Utc};
use lru::LruCache;
use std::alloc::System;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct IpServer {
    limit: usize,
    cache: LruCache<String, DateTime<Utc>>,
}

impl IpServer {
    pub(crate) async fn check_addr(&mut self, addr: SocketAddr) -> anyhow::Result<CheckType> {
        let ip = addr.ip().to_string();
        &self.add_ip(ip)?;
        Ok(CheckType::Normal)
    }
}

impl IpServer {
    pub fn new(limit: usize) -> Self {
        IpServer {
            limit,
            cache: LruCache::new(limit),
        }
    }

    pub fn add_ip(&mut self, ip: String) -> anyhow::Result<()> {
        if self.cache.len() > self.limit {
            return Err(anyhow!("overflow max ip limit."));
        }
        self.cache.put(ip, Utc::now());
        Ok(())
    }

    pub fn get_ip(&mut self, ip: &String) -> Option<&DateTime<Utc>> {
        self.cache.get(ip)
    }
}
