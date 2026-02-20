use sha2::{Sha256, Digest};

/// Helper struct to create hashes
pub struct Hash;

impl Hash {
	/// Returns a SHA256 hash from a string.
	fn from_string(data: &str) -> [u8; 32] {
		let mut hasher = Sha256::new();

		hasher.update(data.as_bytes());

		return hasher.finalize().into();
	}

	/// Converts a SHA256 hash into a hexadecimal string representation.
	fn to_hex_string(data: &[u8]) -> String {
		data.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
	}
}
