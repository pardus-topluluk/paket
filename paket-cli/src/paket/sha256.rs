use sha2::{Digest, Sha256};

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
