use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Xor {
    pub word_a: String,
    pub word_b: String,
    pub xor_word: String,
}

fn create_word_to_index_map(words: &[&str]) -> HashMap<String, u16> {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| (word.to_string(), i as u16))
        .collect()
}

fn word_to_bits(word_to_index: &HashMap<String, u16>, word: &str) -> u16 {
    match word_to_index.get(word) {
        Some(&index) => index,
        None => {
            eprintln!("Word '{}' not found in BIP39 list", word);
            std::process::exit(1);
        }
    }
}

pub fn xor_bits(bits1: u16, bits2: u16) -> u16 {
    bits1 ^ bits2
}

pub fn get_bip39_index_by_word(words: &[&str], word: &str) -> u16 {
    let word_to_index = create_word_to_index_map(&words);
    let bits = word_to_bits(&word_to_index, &word);
    bits
}
