#![allow(unused)]
use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;
use ripemd::Ripemd160;
use secp256k1::{Message, PublicKey, Scalar, Secp256k1, SecretKey};
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};
use sha2::{Digest, Sha256, Sha512};
use std::io::Read;
use std::{fmt, path::PathBuf, process::Command};
type HmacSha512 = Hmac<Sha512>;
use hex::FromHex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum DerivationError {
    ParseError(String),
    DerivationError(String),
}

impl fmt::Display for DerivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DerivationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DerivationError::DerivationError(msg) => write!(f, "Derivation error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub struct Wallet {
    pub witness_programs: HashSet<String>,
    pub private_keys: HashMap<u16, String>,
    pub witness_program_private_key_map: HashMap<String, String>,
}

#[derive(Debug)]
struct ExKey {
    version: [u8; 4],
    depth: [u8; 1],
    finger_print: [u8; 4],
    child_number: [u8; 4],
    chaincode: [u8; 32],
    key: [u8; 33],
}

impl Clone for ExKey {
    fn clone(&self) -> Self {
        ExKey {
            version: self.version,
            depth: self.depth,
            finger_print: self.finger_print,
            child_number: self.child_number,
            chaincode: self.chaincode,
            key: self.key,
        }
    }
}

impl Serialize for ExKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut ex_key = serializer.serialize_struct("ExKey", 6)?;
        ex_key.serialize_field("version", &self.version)?;
        ex_key.serialize_field("depth", &self.depth)?;
        ex_key.serialize_field("finger_print", &self.finger_print)?;
        ex_key.serialize_field("child_number", &self.child_number)?;
        ex_key.serialize_field("chaincode", &self.chaincode)?;
        // ex_key.serialize_field("key", &self.key)?;
        ex_key.end()
    }
}

#[derive(Debug)]
enum DerivationType {
    Normal(u32),
    Hardened(u32),
}

#[derive(Debug)]
struct DerivationPath {
    path: Vec<DerivationType>,
}

impl DerivationPath {
    fn parse(path: &str) -> Result<Self, String> {
        if !path.starts_with("m/") {
            return Err("Path must start with 'm/'".to_string());
        }

        let path = path.strip_prefix("m/").unwrap_or(path);

        let segments = path.split('/').collect::<Vec<&str>>();
        let mut path = Vec::new();

        for segment in segments {
            if segment.contains('\'') || segment.contains('h') {
                let index = segment.trim_end_matches(&['h', '\''][..]);
                match index.parse::<u32>() {
                    Ok(number) => path.push(DerivationType::Hardened(number)),
                    Err(e) => {
                        println!("Failed to parse the number: {}", e);
                    }
                }
            } else {
                match segment.parse::<u32>() {
                    Ok(number) => {
                        path.push(DerivationType::Normal(number));
                    }
                    Err(e) => {
                        println!("Failed to parse the number: {}", e);
                    }
                }
            }
        }

        Ok(DerivationPath { path })
    }
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn hash160(input: &[u8]) -> Vec<u8> {
    // Step 1: Compute SHA-256
    let mut hasher = Sha256::new();
    hasher.update(input);
    let sha256_result = hasher.finalize();

    // Step 2: Compute RIPEMD-160
    let mut ripemd160_hasher = Ripemd160::new();
    ripemd160_hasher.update(&sha256_result);
    let ripemd160_result = ripemd160_hasher.finalize();

    // Convert to Vec<u8>
    ripemd160_result.to_vec()
}

// Decode a base58 string into an array of bytes
fn base58_decode(base58_string: &str) -> Vec<u8> {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let base58_len = base58_alphabet.len();

    // Convert Base58 string to a big integer
    let mut result = BigUint::zero();
    for (i, c) in base58_string.chars().enumerate() {
        let index = base58_alphabet.find(c).expect("Invalid Base58 character");
        result = result * base58_len + BigUint::from(index);
    }

    // Convert the integer to bytes
    let mut bytes = result.to_bytes_be();

    // Handle leading zeros (Base58 encoding uses leading '1's for leading zeros)
    for c in base58_string.chars() {
        if c == '1' {
            bytes.insert(0, 0);
        } else {
            break;
        }
    }

    // Check if the length of the bytes is greater than 4 for checksum
    if bytes.len() > 4 {
        // Chop off the last 4 bytes for the checksum
        let (data, checksum) = bytes.split_at(bytes.len() - 4);

        // BONUS POINTS: Verify the checksum!
        return data.to_vec();
    }
    bytes
}

fn read_version(extended_key_bytes: &mut &[u8]) -> [u8; 4] {
    let mut buffer = [0; 4];
    extended_key_bytes.read(&mut buffer).unwrap();

    buffer
}

fn read_depth(extended_key_bytes: &mut &[u8]) -> [u8; 1] {
    let mut buffer = [0; 1];
    extended_key_bytes.read(&mut buffer).unwrap();

    buffer
}

fn read_finger_print(extended_key_bytes: &mut &[u8]) -> [u8; 4] {
    let mut buffer = [0; 4];
    extended_key_bytes.read(&mut buffer).unwrap();

    buffer
}

fn read_child_number(extended_key_bytes: &mut &[u8]) -> [u8; 4] {
    let mut buffer = [0; 4];
    extended_key_bytes.read(&mut buffer).unwrap();

    buffer
}

fn read_chaincode(extended_key_bytes: &mut &[u8]) -> [u8; 32] {
    let mut buffer = [0; 32];
    extended_key_bytes.read(&mut buffer).unwrap();

    buffer
}

fn read_key(extended_key_bytes: &mut &[u8]) -> [u8; 33] {
    let mut buffer = [0; 33];
    extended_key_bytes.read(&mut buffer).unwrap();

    buffer
}

// Bip32 Serialization format: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
fn deserialize_key(bytes: &[u8]) -> ExKey {
    let mut bytes_slice = bytes;

    let version = read_version(&mut bytes_slice);
    let depth = read_depth(&mut bytes_slice);
    let finger_print = read_finger_print(&mut bytes_slice);
    let child_number = read_child_number(&mut bytes_slice);
    let chaincode = read_chaincode(&mut bytes_slice);
    let key = read_key(&mut bytes_slice);

    ExKey {
        version,
        depth,
        finger_print,
        child_number,
        chaincode,
        key,
    }
}

fn get_child_key_at_path(key: &ExKey, derivation_path: &str) -> Result<ExKey, DerivationError> {
    match DerivationPath::parse(derivation_path) {
        Ok(derivation_path) => {
            let mut previous_ex_key: ExKey = key.clone();
            for segment in derivation_path.path.iter() {
                previous_ex_key = match segment {
                    DerivationType::Hardened(value) => {
                        let child_ex_key = derive_priv_child(&previous_ex_key, 0x80000000 + value);
                        child_ex_key
                    }
                    DerivationType::Normal(value) => {
                        let child_ex_key = derive_priv_child(&previous_ex_key, 0x00000000 + value);
                        child_ex_key
                    }
                };
            }
            let hex_private_key = bytes_to_hex(&previous_ex_key.key[1..]);
            Ok(previous_ex_key)
        }
        Err(e) => Err(DerivationError::ParseError(e.to_string())),
    }
}

fn derive_public_key_from_private(key: &[u8]) -> Vec<u8> {
    // unimplemented!("implement the logic")
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&key).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    public_key.serialize().to_vec()
}

fn derive_priv_child(key: &ExKey, child_num: u32) -> ExKey {
    // Create a mutable vector to hold the composed message
    let mut message = Vec::new();

    if child_num >= 0x80000000 {
        // Extend the message with the byte representation
        message.extend_from_slice(&[0u8]);

        // Extend the message with the key
        message.extend_from_slice(&key.key[1..]);
    } else {
        let secp = Secp256k1::new();
        message.extend_from_slice(&derive_public_key_from_private(&key.key[1..]));
    }

    // Extend the message with the child number as bytes.
    message.extend_from_slice(&u32::from(child_num).to_be_bytes());

    // Create an HMAC instance using SHA-512 and chaincode as key.
    let mut hmac = Hmac::<Sha512>::new_from_slice(&key.chaincode).expect("Invalid key length");
    // Update the HMAC with the composed message.
    hmac.update(&message);
    let hmac_result = hmac.finalize();
    let code = hmac_result.into_bytes();
    let hex_code = bytes_to_hex(&code);

    let child_secret_key: SecretKey =
        secp256k1::SecretKey::from_slice(&code[..32]).expect("statistically impossible to hit");
    let parent_secret_key: SecretKey =
        secp256k1::SecretKey::from_slice(&key.key[1..]).expect("statistically impossible to hit");
    let parent_secrte_key_scalar = Scalar::from(parent_secret_key);
    // Add the tweak to the child key
    let tweaked: SecretKey = child_secret_key
        .add_tweak(&parent_secrte_key_scalar)
        .expect("statistically impossible to hit");

    let mut key_bytes: [u8; 33] = [0; 33];
    key_bytes[0] = 0;
    key_bytes[1..33].copy_from_slice(tweaked.as_ref());

    let test_key_bytes = bytes_to_hex(&key_bytes);
    let chaincode_slice: &[u8] = &code[32..];
    let chaincode: [u8; 32] = chaincode_slice.try_into().expect("Slice length must be 32");

    // Convert u32 to [u8; 32]
    let child_number: [u8; 4] = [
        (child_num >> 24) as u8, // Extract the most significant byte.
        (child_num >> 16) as u8, // Extract the second byte.
        (child_num >> 8) as u8,  // Extract the third byte.
        child_num as u8,         // Extract the least significant byte.
    ];

    let mut depth: [u8; 1] = key.depth;

    match depth[0].checked_add(1) {
        Some(new_value) => depth[0] = new_value,
        none => println!("Overflow occurred!"),
    }

    ExKey {
        version: key.version,
        depth,
        finger_print: key.finger_print,
        child_number,
        chaincode,
        key: key_bytes,
    }
}

// https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#user-content-P2WPKH
fn get_p2wpkh_program(pubkey: &[u8]) -> Vec<u8> {
    let key_hash = hash160(pubkey);
    // 0x00 is the version byte for P2WPKH
    let mut script_pub_key = vec![0x00];
    // Push the length byte (0x14)
    script_pub_key.push(0x14);
    // Extend with the key_hash slice
    script_pub_key.extend_from_slice(&key_hash);
    script_pub_key
}

pub fn get_wallet(extended_private_key: String) -> Result<Wallet, DerivationError> {
    let decoded_extended_private_key = base58_decode(&extended_private_key);
    let extended_key = deserialize_key(&decoded_extended_private_key);
    let mut private_keys: HashMap<u16, String> = HashMap::new();
    let mut public_keys: HashSet<String> = HashSet::new();
    let mut witness_programs: HashSet<String> = HashSet::new();
    let mut witness_program_private_key_map: HashMap<String, String> = HashMap::new();

    for index in 0..10 {
        let derivation_path = format!("m/84h/1h/0h/0/{}", index);
        let child_extended_key = get_child_key_at_path(&extended_key, &derivation_path).unwrap();
        let hex_child_private_key = bytes_to_hex(&child_extended_key.key[1..]);
        let child_public_key = derive_public_key_from_private(&child_extended_key.key[1..]);
        let witness_program = get_p2wpkh_program(&child_public_key);

        private_keys.insert(index, hex_child_private_key.clone());
        witness_programs.insert(bytes_to_hex(&witness_program));
        witness_program_private_key_map.insert(
            bytes_to_hex(&witness_program),
            hex_child_private_key.clone(),
        );
    }

    Ok(Wallet {
        private_keys,
        witness_programs,
        witness_program_private_key_map,
    })
}
