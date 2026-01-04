use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 6810;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Parse host and port from args
    let mut host = DEFAULT_HOST.to_string();
    let mut port = DEFAULT_PORT;
    let mut cmd_start = 1;

    // Check for -h and -p flags
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" if i + 1 < args.len() => {
                host = args[i + 1].clone();
                cmd_start = i + 2;
                i += 2;
            }
            "-p" if i + 1 < args.len() => {
                port = args[i + 1].parse().unwrap_or(DEFAULT_PORT);
                cmd_start = i + 2;
                i += 2;
            }
            _ => break,
        }
    }

    let addr = format!("{}:{}", host, port);

    // Remaining args are the command
    let command_args: Vec<&str> = args[cmd_start..].iter().map(|s| s.as_str()).collect();

    if command_args.is_empty() {
        // Interactive mode
        interactive_mode(&addr);
    } else {
        // One-shot mode
        let command = command_args.join(" ");
        oneshot_mode(&addr, &command);
    }
}

fn oneshot_mode(addr: &str, command: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            let cmd = format!("{}\n", command);
            if let Err(e) = stream.write_all(cmd.as_bytes()) {
                eprintln!("Failed to send command: {}", e);
                return;
            }

            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            
            // Read all response lines
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if line.trim() == "END" {
                            break; // End of multi-line response
                        }
                        response.push_str(&line);
                        if !command.to_uppercase().starts_with("KEYS") {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            
            print!("{}", response);
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}

fn interactive_mode(addr: &str) {
    let mut stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", addr, e);
            std::process::exit(1);
        }
    };
    
    println!("Connected to ferris-db at {}", addr);
    println!("Type EXIT to quit.\n");

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("FerrisDB> ");
        io::stdout().flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.to_uppercase() == "EXIT" {
            println!("Goodbye!");
            break;
        }

        let cmd = format!("{}\n", trimmed);
        if let Err(e) = stream.write_all(cmd.as_bytes()) {
            eprintln!("Failed to send command: {}", e);
            break; // Connection likely dead, exit
        }

        let mut reader = BufReader::new(&stream);
        
        // Read response lines
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if line.trim() == "END" {
                        break;
                    }
                    print!("{}", line);
                    if !trimmed.to_uppercase().starts_with("KEYS") {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }
}