use crate::notify::mail::EmailServer;
use crate::proxy::SocketConfig;
use serde::Deserialize;
use simple_log::LogConfig;
use std::fs;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub proxy: SocketConfig,
    log: Option<LogConfig>,
    pub email_server: Option<EmailServer>,
}

pub fn init_conf<S: Into<String>>(path: S) -> anyhow::Result<ServerConfig> {
    let s = fs::read_to_string(path.into())?;
    let conf: ServerConfig = toml::from_str(&s)?;

    if let Some(log) = &conf.log {
        simple_log::new(log.clone()).map_err(|err| anyhow::anyhow!("{}", err))?;
    } else {
        simple_log::quick().map_err(|err| anyhow::anyhow!("{}", err))?;
    }
    Ok(conf)
}
