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

// fn main() {
//     let mut store = Store::new();
//     let mut input = String::new();

//     println!("Welcome to ferris-db!");
//     println!("Commands: GET <key>, SET <key> <value> <ttl>, DELETE <key>, RENAME <old_key> <new_key>, EXPIRE <key> <ttl>, KEYS, COUNT, CLEAR, EXIT");

//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();

//         input.clear();
//         io::stdin().read_line(&mut input).unwrap();

//         match Command::parse(&input) {
//             Command::Get { key } => {
//                 match store.get(&key) {
//                     Some(value) => println!("{}", value),
//                     None => println!("Error: Key not found"),
//                 }
//             }
//             Command::Set { key, value, ttl } => {
//                 match store.set(key, value, ttl) {
//                     Ok(()) => println!("OK"),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//             Command::Delete { key } => {
//                 match store.delete(&key) {
//                     Ok(()) => println!("OK"),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//             Command::Rename { old_key, new_key } => {
//                 match store.rename(old_key, new_key) {
//                     Ok(()) => println!("OK"),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//             Command::Expire { key, ttl } => {
//                 match store.expire(key, ttl) {
//                     Ok(()) => println!("OK"),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//             Command::TTL { key } => {
//                 match store.ttl(key) {
//                     Ok(ttl) => println!("{}", ttl),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//             Command::Keys => {
//                 for key in store.keys() {
//                     println!("{}", key);
//                 }
//             }
//             Command::Count => {
//                 println!("{}", store.count());
//             }
//             Command::Clear => {
//                 match store.clear() {
//                     Ok(()) => println!("OK"),
//                     Err(e) => println!("Error: {}", e),
//                 }
//             }
//             Command::Exit => {
//                 println!("Goodbye!");
//                 break;
//             }
//             Command::Unknown(cmd) => {
//                 println!("Error: <{}>", cmd.trim());
//             }
//         }
//     }
// }
