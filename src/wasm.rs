use ::miniscript::bitcoin;
use wasm_bindgen::prelude::*;
mod transaction;
use self::transaction::{Decodable, Transaction};
use self::xor::{get_bip39_index_by_word, xor_bits, Xor};
mod mnemonic_to_descriptor;
mod policy;
mod xor;
use self::mnemonic_to_descriptor::{from_mnemonic_to_descriptor, import_desc};
use self::policy::{get_policy_info, MiniscriptKeyType};

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Welcome to Decode Bitcoin Dot Com!");

    // body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn decode(transaction_hex: String) -> Result<String, JsValue> {
    let transaction_bytes = hex::decode(&transaction_hex)
        .map_err(|e| JsValue::from_str(&format!("Hex decode error: {}", e)))?;
    let transaction = Transaction::consensus_decode(&mut transaction_bytes.as_slice())
        .map_err(|e| JsValue::from_str(&format!("Consensus decode error: {}", e)))?;
    Ok(serde_json::to_string_pretty(&transaction)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
}

#[wasm_bindgen]
pub fn xor(word_a: &str, word_b: &str) -> Result<String, JsValue> {
    let bip39_words = include_str!("assets/bip39-english.txt")
        .lines()
        .collect::<Vec<_>>();

    let bits1 = get_bip39_index_by_word(&bip39_words, &word_a);
    let bits2 = get_bip39_index_by_word(&bip39_words, &word_b);

    let xor_result = xor_bits(bits1, bits2);

    let xor_word = match bip39_words.get(xor_result as usize) {
        Some(&word) => word,
        None => {
            eprintln!("XOR not possible in Bip 39 range.");
            std::process::exit(1);
        }
    };

    let mut xor_result = vec![];
    xor_result.push(Xor {
        word_a: word_a.to_string(),
        word_b: word_b.to_string(),
        xor_word: xor_word.to_string(),
    });
    Ok(serde_json::to_string_pretty(&xor_result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
}

#[wasm_bindgen]
pub fn policy(policy_str: &str) -> Result<String, JsValue> {
    if let Ok(info) =
        get_policy_info::<bitcoin::PublicKey>(policy_str, MiniscriptKeyType::PublicKey)
    {
        Ok(serde_json::to_string_pretty(&info).map_err(|e| {
            let msg = format!("Serialization error: {}", e);
            web_sys::console::error_1(&JsValue::from_str(&msg));
            JsValue::from_str(&msg)
        })?)
    } else {
        match get_policy_info::<String>(policy_str, MiniscriptKeyType::String) {
            Ok(info) => match serde_json::to_string_pretty(&info) {
                Ok(json_str) => Ok(json_str),
                Err(e) => {
                    let msg = format!("Serialization error: {}", e);
                    web_sys::console::error_1(&JsValue::from_str(&msg));
                    Err(JsValue::from_str(&msg))
                }
            },
            Err(e) => {
                let msg = format!("Policy parse error: {}", e);
                web_sys::console::error_1(&JsValue::from_str(&msg));
                Err(JsValue::from_str(&msg))
            }
        }
    }
}

#[wasm_bindgen(js_name = mnemonicToDescriptor)]
pub fn mnemonic_to_descriptor(
    mnemonic: &str,
    passphrase: &str,
    network: &str,
) -> Result<String, JsValue> {
    let network_kind = match network {
        "main" => bitcoin::NetworkKind::Main,
        "test" => bitcoin::NetworkKind::Test,
        _ => return Err(JsValue::from_str("Invalid network: use \"main\" or \"test\"")),
    };

    let descriptors = from_mnemonic_to_descriptor(mnemonic, passphrase, network_kind)
        .ok_or_else(|| JsValue::from_str("Failed to derive descriptors from mnemonic"))?;

    serde_json::to_string_pretty(&descriptors)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen(js_name = importDescriptor)]
pub fn import_descriptor(descriptor_str: &str) -> Result<String, JsValue> {
    let descriptor: miniscript::Descriptor<String> = descriptor_str
        .parse()
        .map_err(|e| JsValue::from_str(&format!("Invalid descriptor: {}", e)))?;

    Ok(import_desc(descriptor))
}
