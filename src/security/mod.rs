use crate::proxy::ValidateType;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::RwLock;

static IP_TABLE: Lazy<RwLock<HashMap<String, ()>>> = Lazy::new(Default::default);

pub fn validate(remote_addr: &SocketAddr) -> anyhow::Result<ValidateType> {
    let remote_ip = remote_addr.ip().to_string();
    let mut guard = IP_TABLE.write().unwrap();

    // if guard.len() > 0 {
    //     TODO change configuration
        // return Err(anyhow::anyhow!("{} not exit error", remote_ip));
    // }

    if guard.get(&remote_ip).is_some() {
        return Ok(ValidateType::Normal);
    }

    guard.insert(remote_ip, ());

    Ok(ValidateType::Normal)
}
