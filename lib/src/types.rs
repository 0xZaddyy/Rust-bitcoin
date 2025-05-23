#[allow(unused_imports)]
use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
#[allow(unused_imports)]
use crate::util::MerkleRoot;
use chrono::{DateTime, Utc};
#[allow(unused_imports)]
use k256::Secp256k1;
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use std::vec;
#[allow(unused_imports)]
use uuid::timestamp;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header: header,
            transactions: transactions,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    // Timestamp of the block
    pub timestamp: DateTime<Utc>,
    // Nonce used to mine the block
    pub nonce: u64,
    // Hash of the previous transaction
    pub prev_block_hash: [u8; 32],
    // Merkle root of the block's transactions
    pub merkle_root: [u8; 32],
    // target
    pub target: U256,
}

impl BlockHeader {
    fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
