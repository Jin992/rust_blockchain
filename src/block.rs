use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // The index in which the current block is stored
    pub index: u64,
    // The time  the current block is created
    pub timestamp: u64,

    // The blocks proof of work
    pub proof_of_work: u64,
    // The previous block hash
    pub previous_hash: String,
    // Current block hash
    pub hash: String,
    pub nonce: u64
}

impl Block {

    // Create a new block. The hash will be calculated and set automatically.
    pub fn new(index: u64, nonce: u64,  previous_hash: String) -> Self {
        // Current block to be created.
        Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
            nonce
        }
    }
    pub fn generate_genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
            nonce: u64::default()
        }
    }

    pub fn calculate_hash(block: &Block) -> String {
        let serialized_clock_data = serde_json::to_string(&block).unwrap();
        // Calculate and return SHA-256 hash value
        let mut hasher = Sha256::new();
        hasher.update(serialized_clock_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

}