use std::collections::HashMap;
use std::fs;
use std::path::Path;
use lazy_static::lazy_static;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{Zero, One, Num};
use rand::thread_rng;

/// BIP39 implementation for encoding/decoding 256-bit big integers to/from mnemonic sentences
pub struct Bip39 {
    word_list: Vec<String>,
    word_index: HashMap<String, usize>,
}

impl Bip39 {
    /// Create a new Bip39 instance by loading the English word list
    pub fn new() -> Result<Self, String> {
        let word_list = Self::load_word_list()?;
        let word_index = Self::build_word_index(&word_list);

        Ok(Self {
            word_list,
            word_index,
        })
    }

    /// Load the English word list from the embedded file
    fn load_word_list() -> Result<Vec<String>, String> {
        let path = Path::new("src/english.txt");
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read word list: {}", e))?;

        let words: Vec<String> = content
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if words.len() != 2048 {
            return Err(format!("Invalid word list length: expected 2048, got {}", words.len()));
        }

        Ok(words)
    }

    /// Build a hash map for quick word lookup
    fn build_word_index(words: &[String]) -> HashMap<String, usize> {
        words.iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect()
    }

    /// Encode a 256-bit big integer to a BIP39 mnemonic sentence
    /// Returns a 24-word mnemonic sentence
    pub fn encode(&self, bigint: &BigUint) -> Result<String, String> {
        // Convert bigint to 256-bit binary representation
        let bits = bigint.bits();
        if bits > 256 {
            return Err(format!("Input too large: {} bits (max 256)", bits));
        }

        // Pad with leading zeros to make exactly 256 bits
        let mut binary = format!("{:0256b}", bigint);

        // Calculate checksum (first 8 bits of SHA256 hash of the binary data)
        // For BIP39, checksum is 1 bit per 32 bits of entropy, so 8 bits for 256 bits
        let checksum = self.calculate_checksum(&binary)?;

        // Combine entropy + checksum
        let entropy_with_checksum = format!("{}{:08b}", binary, checksum);

        // Split into 11-bit chunks (24 words * 11 bits = 264 bits)
        let mut words = Vec::new();
        for chunk in entropy_with_checksum.as_bytes().chunks(11) {
            if chunk.len() < 11 {
                return Err("Invalid chunk size during encoding".to_string());
            }

            // Convert 11-bit binary to index
            let chunk_str = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
            let index = usize::from_str_radix(chunk_str, 2)
                .map_err(|e| format!("Failed to parse binary chunk: {}", e))?;

            if index >= 2048 {
                return Err(format!("Invalid word index: {}", index));
            }

            words.push(self.word_list[index].clone());
        }

        Ok(words.join(" "))
    }

    /// Calculate checksum for BIP39 (first 8 bits of SHA256 hash)
    fn calculate_checksum(&self, binary_data: &str) -> Result<u8, String> {
        use sha2::{Sha256, Digest};

        // Convert binary string to bytes
        let bytes = binary_data
            .as_bytes()
            .chunks(8)
            .map(|chunk| {
                let chunk_str = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(chunk_str, 2).unwrap()
            })
            .collect::<Vec<u8>>();

        // Calculate SHA256 hash
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let result = hasher.finalize();

        // Take first byte (8 bits) as checksum
        Ok(result[0])
    }

    /// Decode a BIP39 mnemonic sentence to a 256-bit big integer
    pub fn decode(&self, mnemonic: &str) -> Result<BigUint, String> {
        let words: Vec<&str> = mnemonic.split_whitespace().collect();

        if words.len() != 24 {
            return Err(format!("Invalid mnemonic length: expected 24 words, got {}", words.len()));
        }

        // Convert words to indices
        let mut indices = Vec::new();
        for word in words {
            let index = self.word_index.get(word)
                .ok_or_else(|| format!("Unknown word in mnemonic: {}", word))?;
            indices.push(*index);
        }

        // Convert indices to 11-bit binary chunks
        let mut binary_chunks = Vec::new();
        for &index in &indices {
            let binary = format!("{:011b}", index);
            binary_chunks.push(binary);
        }

        // Combine all chunks into a single binary string
        let combined_binary = binary_chunks.join("");

        if combined_binary.len() != 264 {
            return Err(format!("Invalid binary length: expected 264 bits, got {}", combined_binary.len()));
        }

        // Split into entropy (256 bits) and checksum (8 bits)
        let entropy_bits = &combined_binary[..256];
        let checksum_bits = &combined_binary[256..];

        // Verify checksum
        self.verify_checksum(entropy_bits, checksum_bits)?;

        // Convert entropy bits to BigUint
        let bigint = BigUint::from_str_radix(entropy_bits, 2)
            .map_err(|e| format!("Failed to parse entropy as bigint: {}", e))?;

        Ok(bigint)
    }

    /// Verify the checksum matches the expected value
    fn verify_checksum(&self, entropy_bits: &str, checksum_bits: &str) -> Result<(), String> {
        let expected_checksum = self.calculate_checksum(entropy_bits)?;
        let expected_binary = format!("{:08b}", expected_checksum);

        if expected_binary != checksum_bits {
            return Err(format!("Checksum mismatch: expected {}, got {}", expected_binary, checksum_bits));
        }

        Ok(())
    }

    /// Get the word list for reference
    pub fn get_word_list(&self) -> &[String] {
        &self.word_list
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;
    use super::*;

    #[test]
    fn test_bip39_roundtrip() {
        let bip39 = Bip39::new().unwrap();

        // Test with a known value
        let test_bigint = BigUint::from(0x1234567890abcdefu64);
        let mnemonic = bip39.encode(&test_bigint).unwrap();
        let decoded = bip39.decode(&mnemonic).unwrap();

        assert_eq!(test_bigint, decoded);
    }

    #[test]
    fn test_bip39_roundtrip_random() {
        let bip39 = Bip39::new().unwrap();

        let mut rng = OsRng::default();
        for _ in 0..10000 {
            let test_bigint = rng.gen_biguint(256);
            let mnemonic = bip39.encode(&test_bigint).unwrap();
            let decoded = bip39.decode(&mnemonic).unwrap();
            assert_eq!(test_bigint, decoded);
        }
    }

    #[test]
    fn test_random_generation() {
        let bip39 = Bip39::new().unwrap();
        let bigint = OsRng::default().gen_biguint(256);

        let mnemonic = bip39.encode(&bigint).unwrap();
        let words: Vec<&str> = mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), 24);
    }
}
