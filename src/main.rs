mod store;
mod command;
mod server;

use std::env;

const DEFAULT_PORT: u16 = 6810;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse::<u16>().unwrap_or(DEFAULT_PORT)
    } else {
        DEFAULT_PORT
    };

    let addr = format!("127.0.0.1:{}", port);

    println!("Starting ferris-db server...");

    if let Err(e) = server::run_server(&addr).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
