use std::error::Error;
use std::env;
use std::process;
use serde_json;
use hex; 

use self::transaction::{Decodable, Transaction};
mod transaction;

pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(&transaction_hex)
        .map_err(|e| format!("Hex decode error: {}", e))?;
    let transaction = Transaction::consensus_decode(&mut transaction_bytes.as_slice())
        .map_err(|e| format!("Consensus decode error: {}", e))?;
    let json_output = serde_json::to_string_pretty(&transaction)
        .map_err(|e| format!("Serialization error: {}", e))?;
    Ok(json_output)
}

fn main() {
    let transaction_hex = env::args().nth(1).expect("Please provide a transaction hex string");
    
    match decode(transaction_hex) {
        Ok(result) => println!("Decoded transaction:\n{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
