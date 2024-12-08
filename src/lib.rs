use wasm_bindgen::prelude::*;
mod transaction;
use self::transaction::{Decodable, Transaction};

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
