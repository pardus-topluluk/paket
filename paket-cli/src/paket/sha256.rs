use sha2::{Digest, Sha256};

/// Calculate SHA256 of the given bytes.
///
/// Example:
/// ```rust
/// let text = String::from("Selam");
/// let sha256sum = crate::paket_cli::paket::sha256::calculate_sha256(text.as_bytes());
///
/// assert_eq!(sha256sum, "26fb1ab0ca8483866f03ca66e2018b0685f3e1e84caca77b3f5643ae799d9eb4");
/// ```
pub fn calculate_sha256(input: &[u8]) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input);

    let hash_result = hasher.finalize();

    let hash_hex_string = hash_result
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    hash_hex_string
}
