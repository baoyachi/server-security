use chrono::{DateTime, Local, Utc};
use lru::LruCache;
use std::alloc::System;
use std::time::{Duration, Instant};

pub struct IpServer {
    limit: usize,
    cache: LruCache<String, DateTime<Utc>>,
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

    pub fn get_ip(&mut self, ip: &String) -> anyhow::Result<&DateTime<Utc>> {
        let datetime = self
            .cache
            .get(ip)
            .ok_or_else(|| anyhow!("ip:{} not found", ip))?;
        Ok(datetime)
    }
}
