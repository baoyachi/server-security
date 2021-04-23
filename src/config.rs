use crate::proxy::SocketConfig;
use serde::Deserialize;
use simple_log::LogConfig;
use std::fs;
use crate::notify::mail::EmailServer;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub proxy: SocketConfig,
    log: LogConfig,
    pub email_server: EmailServer,
}

pub fn init_conf(path: String) -> anyhow::Result<ServerConfig> {
    let s = fs::read_to_string(path)?;
    let conf: ServerConfig = toml::from_str(&s)?;

    if shadow_rs::is_debug() {
        simple_log::quick().map_err(|err| anyhow::anyhow!("{}", err))?;
    } else {
        simple_log::new(conf.log.clone()).map_err(|err| anyhow::anyhow!("{}", err))?;
    }
    Ok(conf)
}
