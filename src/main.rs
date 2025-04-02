use std::error::Error;
use std::env;
use std::process;
use serde_json;
use hex; 

mod transaction;
use self::transaction::{Decodable, Transaction};

mod bip32;
use self::bip32::get_wallet;

pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(&transaction_hex)
        .map_err(|e| format!("Hex decode error: {}", e))?;
    let transaction = Transaction::consensus_decode(&mut transaction_bytes.as_slice())
        .map_err(|e| format!("Consensus decode error: {}", e))?;
    let json_output = serde_json::to_string_pretty(&transaction)
        .map_err(|e| format!("Serialization error: {}", e))?;
    Ok(json_output)
}

pub fn derive_keys_from_xpriv(extended_private_key: String) {
    let wallet = get_wallet(extended_private_key);
    
    match wallet {
        Ok(wallet) => {
            // Print the derived wallet to standard output
            println!("Derived wallet: {:?}", wallet);
        },
        Err(e) => {
            // Print the error to standard error
            eprintln!("Failed to derive wallet: {}", e);
        },
    }}

fn main() {
    // Check if at least one argument is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> <args>", args[0]);
        eprintln!("Commands:");
        eprintln!("  decode <transaction_hex>");
        eprintln!("  derive <extended_private_key>");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "decode" => {
            if args.len() < 3 {
                eprintln!("Usage: {} decode <transaction_hex>", args[0]);
                process::exit(1);
            }
            let transaction_hex = args[2].clone();
            match decode(transaction_hex) {
                Ok(result) => println!("Decoded transaction:\n{}", result),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        },
        "derive" => {
            if args.len() < 3 {
                eprintln!("Usage: {} derive <extended_private_key>", args[0]);
                process::exit(1);
            }
            let extended_private_key = args[2].clone();
            derive_keys_from_xpriv(extended_private_key);
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}
