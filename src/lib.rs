#[macro_use]
extern crate log;

pub mod config;
pub mod notify;
pub mod proxy;
pub mod security;

use crate::config::init_conf;
use crate::proxy::{new_proxy, CondType};
use crate::security::validate;

pub async fn start(path: String) -> anyhow::Result<()> {
    let config = init_conf(path)?;
    new_proxy(config.proxy, validate, |_|Ok(CondType::Continue)).await
}
