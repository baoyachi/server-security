use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::RwLock;
use crate::proxy::ValidateType;

static IP_TABLE: Lazy<RwLock<HashMap<String, ()>>> = Lazy::new(|| Default::default());

pub fn validate(remote_addr: &SocketAddr) -> anyhow::Result<ValidateType> {
    let remote_ip = remote_addr.ip().to_string();
    let mut guard = IP_TABLE.write().unwrap();

    if guard.get(&remote_ip).is_some() {
        return Ok(ValidateType::Normal);
    }
    if guard.len() > 0 {
        //TODO change configuration
        return anyhow::bail!("{} not exit error", remote_ip);
    }
    guard.insert(remote_ip, ());

    Ok(ValidateType::Normal)
}
