use server_security::security::Server;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("lost config path error");
        exit(-1);
    }
    Server::start(args[1].to_string(), 10).await.unwrap();
}
