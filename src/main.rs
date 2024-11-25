use clap::{Parser, Subcommand};
use simple_kv_store::KvStore;
use std::error::Error;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "kv")]
#[command(about = "A simple key-value store CLI", long_about = None)]
struct Cli {}

#[derive(Subcommand)]
enum Command {
    /// Get a value by key
    Get {
        /// The key to look up
        key: String,
    },
    /// Set a key-value pair
    Set {
        /// The key to set
        key: String,
        /// The value to set
        value: String,
        /// Optional TTL in seconds
        #[arg(short, long)]
        ttl: Option<u64>,
    },
    /// Delete a key-value pair
    Delete {
        /// The key to delete
        key: String,
    },
    /// List all key-value pairs
    List,
    /// Get TTL for a key
    GetTtl {
        /// The key to check TTL for
        key: String,
    },
    /// Get TTL for a key
    Ttl {
        /// The key to check TTL for
        key: String,
    },
    /// Exit the shell
    Exit,
    /// Show help message
    Help,
}

fn print_help() {
    println!("Available commands:");
    println!("  get <key>                     Get a value by key");
    println!("  set <key> <value> [--ttl <seconds>]  Set a key-value pair with optional TTL");
    println!("  delete <key>                  Delete a key-value pair");
    println!("  list                          List all key-value pairs");
    println!("  ttl <key>                     Get TTL for a key");
    println!("  getttl <key>                  Get TTL for a key (verbose)");
    println!("  exit                          Exit the shell");
    println!("  help                          Show this help message");
}

fn parse_input(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        "get" if parts.len() == 2 => Some(Command::Get {
            key: parts[1].to_string(),
        }),
        "set" => {
            if parts.len() >= 3 {
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                let mut ttl = None;

                if parts.len() >= 5 && parts[3] == "--ttl" {
                    if let Ok(ttl_val) = parts[4].parse() {
                        ttl = Some(ttl_val);
                    }
                }

                Some(Command::Set { key, value, ttl })
            } else {
                println!("Usage: set <key> <value> [--ttl <seconds>]");
                None
            }
        }
        "delete" if parts.len() == 2 => Some(Command::Delete {
            key: parts[1].to_string(),
        }),
        "list" if parts.len() == 1 => Some(Command::List),
        "getttl" if parts.len() == 2 => Some(Command::GetTtl {
            key: parts[1].to_string(),
        }),
        "ttl" if parts.len() == 2 => Some(Command::Ttl {
            key: parts[1].to_string(),
        }),
        "exit" | "quit" => Some(Command::Exit),
        "help" => Some(Command::Help),
        _ => {
            println!("Unknown command. Type 'help' for available commands.");
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _cli = Cli::parse();
    let mut store = KvStore::new()?;
    
    println!("Welcome to the key-value store shell. Type 'help' for available commands.");
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let command = match parse_input(&input) {
            Some(cmd) => cmd,
            None => continue,
        };

        match command {
            Command::Get { key } => {
                match store.get(&key) {
                    Some(value) => println!("{}", value),
                    None => println!("Key not found"),
                }
            }
            Command::Set { key, value, ttl } => {
                store.set_with_ttl(key.clone(), value, ttl)?;
                println!("Key '{}' has been set.", key);
            }
            Command::Delete { key } => {
                match store.delete(&key)? {
                    Some(_) => println!("Key '{}' has been deleted.", key),
                    None => println!("Key not found"),
                }
            }
            Command::List => {
                let pairs = store.list();
                if pairs.is_empty() {
                    println!("Store is empty");
                } else {
                    for (key, value) in pairs {
                        println!("{}: {}", key, value);
                    }
                }
            }
            Command::GetTtl { key } => {
                match store.get(&key) {
                    Some(_) => {
                        match store.get_ttl(&key) {
                            Some(ttl) => println!("TTL for key '{}': {} seconds", key, ttl),
                            None => println!("Key '{}' has no TTL set", key),
                        }
                    }
                    None => println!("Key not found"),
                }
            }
            Command::Ttl { key } => {
                match store.get_ttl(&key) {
                    Some(ttl) => println!("{}", ttl),
                    None => println!("Key not found or no TTL set"),
                }
            }
            Command::Help => print_help(),
            Command::Exit => break,
        }
    }

    Ok(())
} 