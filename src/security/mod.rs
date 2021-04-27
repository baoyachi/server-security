use crate::config::init_conf;
use crate::proxy::{CondType, Proxy, ValidateType};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Server {
    proxy: Proxy,
    sender: Sender<String>,
    path: String,
}

impl Server {
    pub async fn start(path: String) -> anyhow::Result<()> {
        let (sender, rev) = channel::<String>(10);
        let proxy = Proxy::new();
        let server = Server {
            proxy,
            sender,
            path,
        };
        server.start_inner(rev).await
    }

    pub async fn start_inner(&self, mut rev: Receiver<String>) -> anyhow::Result<()> {
        let config = init_conf(&self.path)?;
        tokio::spawn(async move {
            while let Some(i) = rev.recv().await {
                info!("got = {:?}", i);
            }
        });
        self.proxy
            .new_proxy(
                config.proxy,
                |addr| async move {
                    println!("addr:{:?}", addr);
                    self.sender.send(addr.to_string()).await?;
                    Ok(ValidateType::Normal)
                },
                |_| Ok(CondType::Continue),
            )
            .await
    }
}
