use hex;
use miniscript::bitcoin;
use serde_json;
use std::env;
use std::error::Error;
use std::process;

mod transaction;
use self::transaction::{Decodable, Transaction};

mod bip32;
use self::bip32::get_wallet;

mod policy;
use self::policy::{get_policy_info, MiniscriptKeyType};

mod mnemonic_to_descriptor;

pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes =
        hex::decode(&transaction_hex).map_err(|e| format!("Hex decode error: {}", e))?;
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
        }
        Err(e) => {
            // Print the error to standard error
            eprintln!("Failed to derive wallet: {}", e);
        }
    }
}

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
        }
        "derive" => {
            if args.len() < 3 {
                eprintln!("Usage: {} derive <extended_private_key>", args[0]);
                process::exit(1);
            }
            let extended_private_key = args[2].clone();
            derive_keys_from_xpriv(extended_private_key);
        }
        "policy" => {
            if args.len() < 3 {
                eprintln!("Usage: {} policy <policy_str>", args[0]);
                process::exit(1);
            }
            let policy_str = args[2].as_str();

            if let Ok(info) =
                get_policy_info::<bitcoin::PublicKey>(policy_str, MiniscriptKeyType::PublicKey)
            {
                println!("Policy info: {:?}", info);
            } else {
                // Then try with strings.
                match get_policy_info::<String>(policy_str, MiniscriptKeyType::String) {
                    Ok(info) => println!("Policy info: {:?}", info),
                    Err(e) => eprintln!("Error retrieving policy info: {}", e),
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}
