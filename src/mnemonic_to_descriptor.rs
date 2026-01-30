use bip39::{Language, Mnemonic};
use miniscript::{
    bitcoin::{bip32::Xpriv, NetworkKind},
    Descriptor,
};

#[derive(serde::Serialize)]
pub struct DescriptorsFromMnemonic {
    pub bip44: Descriptor<String>,
    pub bip49: Descriptor<String>,
    pub bip84: Descriptor<String>,
    pub bip86: Descriptor<String>,
}

pub fn from_mnemonic_to_descriptor(
    mnemonic_input: &str,
    passphrase_input: &str,
    network: NetworkKind,
) -> Option<DescriptorsFromMnemonic> {
    let mnemonic = match Mnemonic::parse_in_normalized(Language::English, &mnemonic_input) {
        Ok(mnemonic) => mnemonic,
        Err(error) => panic!("Failed to parse mnemonic: {error:?}"),
    };
    let word_count = mnemonic.word_count();
    println!("Parsed {word_count} word mnemonic.");

    // Trim \n from passphrase
    let passphrase = passphrase_input.trim_end();

    let seed = mnemonic.to_seed(passphrase);
    let root_xprv = Xpriv::new_master(network, &seed).unwrap();
    let network_der = if network == NetworkKind::Main {
        "0h"
    } else {
        "1h"
    };

    let bip44: Descriptor<String> =
        Descriptor::new_pkh(format!("{root_xprv}/44h/{network_der}/0h/<0;1>/*"))
            .expect("valid pkh() descriptor");
    let bip49: Descriptor<String> =
        Descriptor::new_sh_wpkh(format!("{root_xprv}/49h/{network_der}/0h/<0;1>/*"))
            .expect("valid sh(wpkh()) descriptor");
    let bip84: Descriptor<String> =
        Descriptor::new_wpkh(format!("{root_xprv}/84h/{network_der}/0h/<0;1>/*"))
            .expect("valid wpkh() descriptor");
    let bip86: Descriptor<String> =
        Descriptor::new_tr(format!("{root_xprv}/86h/{network_der}/0h/<0;1>/*"), None)
            .expect("valid tr() descriptor");
    let descriptors = DescriptorsFromMnemonic {
        bip44,
        bip49,
        bip84,
        bip86,
    };
    Some(descriptors)
}

pub fn import_desc(descriptor: Descriptor<String>) -> String {
    let desc = descriptor.to_string(); // includes a checksum
    return format!("{{\"desc\": \"{desc}\", \"timestamp\": \"now\", \"active\": true}}");
}
