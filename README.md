# server-security-proxy
server security proxy write by Rust

# how to use
* 1. config toml file
```toml
[proxy]
server_addr = "0.0.0.0:8081"
to_addr = "127.0.0.1:8080"

[log]
# see how to configrution simple-log:https://github.com/baoyachi/simple-log
path = "./var/log/server_security/server_security.log"
level = "INFO"
size = 200
out_kind = ["file"]
roll_count = 300
```

* run exec:./server_security ./config/server_config.toml
```rust
use server_security::start;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().map(|x| format!("{}", x)).collect();
    if args.len() < 2 {
        println!("lost config path error");
        exit(-1);
    }
    start(format!("{}", args[1])).await.unwrap();
}
```

## TODO
- [ ] security validate
- [ ] notify 
- [ ] monitor
- [ ] tls
