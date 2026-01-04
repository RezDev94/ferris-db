use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::core::{Command, Store};
use crate::persistence::FilePersistence;

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("ferris-db server listening on {}", addr);

    let persistence = Arc::new(FilePersistence::new("data.json"));
    let store = Arc::new(Mutex::new(Store::new(persistence)));

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        println!("New connection from: {}", peer_addr);

        let store = Arc::clone(&store);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, store).await {
                eprintln!("Error handling client {}: {}", peer_addr, e);
            }
            println!("Connection closed: {}", peer_addr);
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    store: Arc<Mutex<Store>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            break;
        }

        let response = execute_command(&line, &store).await;
        writer.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn execute_command(input: &str, store: &Arc<Mutex<Store>>) -> String {
    let command = match Command::parse(input) {
        Ok(cmd) => cmd,
        Err(e) => return format!("ERROR: {}\n", e),
    };
    
    let mut store = store.lock().await;

    match command {
        Command::PING => "PONG\nEND\n".to_string(),
        Command::Get { key } => match store.get(&key) {
            Ok(value) => format!("{}\n", value),
            Err(e) => format!("ERROR: {}\n", e),
        },
        Command::Set { key, value, ttl } => {
            match store.set(key, value, ttl) {
                Ok(()) => "OK\n".to_string(),
                Err(e) => format!("ERROR: {}\n", e),
            }
        },
        Command::Delete { key } => match store.delete(&key) {
            Ok(()) => "OK\n".to_string(),
            Err(e) => format!("ERROR: {}\n", e),
        },
        Command::Rename { old_key, new_key } => match store.rename(old_key, new_key) {
            Ok(()) => "OK\n".to_string(),
            Err(e) => format!("ERROR: {}\n", e),
        },
        Command::Expire { key, ttl } => match store.expire(key, ttl) {
            Ok(()) => "OK\n".to_string(),
            Err(e) => format!("ERROR: {}\n", e),
        },
        Command::TTL { key } => match store.ttl(&key) {
            Ok(ttl) => match ttl {
                Some(ttl) => format!("{}\n", ttl),
                None => "(no ttl)\nEND\n".to_string(),
            },
            Err(e) => format!("ERROR: {}\n", e),
        },
        Command::Keys => {
            let keys = store.keys();
            if keys.is_empty() {
                "(empty)\nEND\n".to_string()
            } else {
                let mut result = String::new();
                for k in keys.iter() {
                    result.push_str(&format!("{}\n", k));
                }
                result.push_str("END\n");
                result
            }
        }
        Command::Count => format!("{}\n", store.count()),
        Command::Clear => match store.clear() {
            Ok(()) => "OK\n".to_string(),
            Err(e) => format!("ERROR: {}\n", e),
        },
    }
}