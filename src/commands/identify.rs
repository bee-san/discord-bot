use crate::error::CommandError;
use cipher_identifier::identify_cipher as identify;

pub fn identify_cipher(text: &str, top_n: usize, highlight_cipher: Option<&str>) -> Result<Vec<(String, f64)>, CommandError> {
    // Use the cipher_identifier crate's identify function
    let results = identify(text, top_n, highlight_cipher)
        .map_err(|e| CommandError::Other(format!("Cipher identification failed: {}", e)))?;
    
    Ok(results)
} 