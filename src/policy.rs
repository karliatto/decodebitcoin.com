use serde::{Deserialize, Serialize};

use miniscript::{policy, BareCtx, FromStrKey, Legacy, MiniscriptKey, Segwitv0};

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct Miniscripts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bare: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwitv0: Option<String>,
    // Taproot to come
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MiniscriptKeyType {
    PublicKey,
    String,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct PolicyInfo {
    pub is_concrete: bool,
    pub key_type: MiniscriptKeyType,
    pub is_trivial: bool,
    pub is_unsatisfiable: bool,
    pub relative_timelocks: Vec<u32>,
    pub n_keys: usize,
    pub minimum_n_keys: usize,
    pub sorted: String,
    pub normalized: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniscript: Option<Miniscripts>,
}

pub fn get_policy_info<Pk: MiniscriptKey + FromStrKey>(
    policy_str: &str,
    key_type: MiniscriptKeyType,
) -> Result<PolicyInfo, miniscript::Error>
where
    Pk: std::str::FromStr,
    <Pk as std::str::FromStr>::Err: std::fmt::Display,
    <Pk as MiniscriptKey>::Sha256: std::str::FromStr,
    <Pk as MiniscriptKey>::Hash256: std::str::FromStr,
    <Pk as MiniscriptKey>::Ripemd160: std::str::FromStr,
    <Pk as MiniscriptKey>::Hash160: std::str::FromStr,
    <<Pk as MiniscriptKey>::Sha256 as std::str::FromStr>::Err: std::fmt::Display,
    <<Pk as MiniscriptKey>::Hash256 as std::str::FromStr>::Err: std::fmt::Display,
    <<Pk as MiniscriptKey>::Ripemd160 as std::str::FromStr>::Err: std::fmt::Display,
    <<Pk as MiniscriptKey>::Hash160 as std::str::FromStr>::Err: std::fmt::Display,
{
    let concrete_pol: Option<policy::Concrete<Pk>> = policy_str.parse().ok();
    let policy = match concrete_pol {
        Some(ref concrete) => policy::Liftable::lift(concrete)?,
        None => policy_str.parse()?,
    };
    Ok(PolicyInfo {
        is_concrete: concrete_pol.is_some(),
        key_type: key_type,
        is_trivial: policy.is_trivial(),
        is_unsatisfiable: policy.is_unsatisfiable(),
        relative_timelocks: policy.relative_timelocks(),
        n_keys: policy.n_keys(),
        minimum_n_keys: policy
            .minimum_n_keys()
            .ok_or(miniscript::Error::CouldNotSatisfy)?,
        sorted: policy.clone().sorted().to_string(),
        normalized: policy.clone().normalized().to_string(),
        miniscript: concrete_pol.map(|p| Miniscripts {
            bare: match policy::compiler::best_compilation::<Pk, BareCtx>(&p) {
                Ok(ms) => Some(ms.to_string()),
                Err(e) => {
                    // debug!("Compiler error: {}", e); // from crate log
                    None
                }
            },
            p2sh: match policy::compiler::best_compilation::<Pk, Legacy>(&p) {
                Ok(ms) => Some(ms.to_string()),
                Err(e) => {
                    // debug!("Compiler error: {}", e); // from crate log
                    None
                }
            },
            segwitv0: match policy::compiler::best_compilation::<Pk, Segwitv0>(&p) {
                Ok(ms) => Some(ms.to_string()),
                Err(e) => {
                    // debug!("Compiler error: {}", e); // from crate log
                    None
                }
            },
        }),
    })
}
