use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CipherType {
    pub types: Vec<String>,
    pub subtypes: Vec<String>,
    pub subtypes2: Vec<String>,
    pub table: Vec<String>,
    pub size: String,
    pub notes: String,
}

pub fn load_cipher_types(path: &str) -> Result<HashMap<String, CipherType>, std::io::Error> {
    let file = std::fs::File::open(path)?;
    let cipher_types: HashMap<String, CipherType> = serde_json::from_reader(file)?;
    Ok(cipher_types)
}

pub fn get_cipher_primary_type(cipher_types: &HashMap<String, CipherType>, cipher: &str) -> String {
    cipher_types
        .get(cipher)
        .and_then(|ct| ct.types.first())
        .cloned()
        .unwrap_or_else(|| "unknown".to_string())
} 