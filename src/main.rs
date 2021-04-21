#[macro_use]
extern crate log;

mod notify;
mod proxy;
mod security;

use crate::proxy::{new_proxy, SocketConfig};
use crate::security::validate;

#[tokio::main]
async fn main() {
    init().unwrap();

    let server_config = SocketConfig {
        listen_addr: "0.0.0.0:8081".to_string(),
        to_addr: "0.0.0.0:8080".to_string(),
    };

    new_proxy(server_config, validate).await.unwrap();
}

fn init() -> anyhow::Result<()> {
    simple_log::quick().unwrap();
    Ok(())
}
