//! # BlockHeader
//!
//! This module defines the `BlockHeader` struct, which represents the header of a blockchain block.
//! It contains various metadata about the block, such as the hash, protocol version, parent hash,
//! block number, block height, difficulty, timestamp, nonce, total difficulty, gas used, gas limit,
//! and transaction root.
//!
//! ## Example
//!
//! ```
//! use crate::{BlockHeader, BlockHeaderBuilder};
//! use crypto::hash::HashDigest;
//! use core_utils::timestamp::Timestamp;
//! use crate::{BlockNumber, BlockHeight, U256};
//!
//! // Create a new BlockHeader using the builder pattern
//! let header = BlockHeaderBuilder::new()
//!     .set_hash(HashDigest::default())
//!     .set_protocol_version(1)
//!     .set_parent_hash(HashDigest::default())
//!     .set_block_number(123)
//!     .set_block_height(456)
//!     .set_difficulty(U256::from(1000))
//!     .set_timestamp(Timestamp::now())
//!     .set_nonce(U256::from(12345))
//!     .set_total_difficulty(U256::from(5000))
//!     .set_transaction_root(HashDigest::default())
//!     .build();
//!
//! // Now you can use the `header` object
//! assert_eq!(header.protocol_version(), 1);
//! assert_eq!(header.block_number(), 123);
//! ```
//!
//! Note: This is just an example. Actual values may vary depending on the use case.

use crypto::hash::HashDigest;
use core_utils::timestamp::Timestamp;
use crate::{BlockNumber, BlockHeight, U256};

// An Block header
pub struct BlockHeader
{  
    // Hash of the block
    hash: HashDigest,
    // Protocol version used
    protocol_version: u32,
    // Parent hash
    parent_hash: HashDigest,
    // Block number
    block_number: BlockNumber,
    // Block height
    block_height: BlockHeight,
    // Difficulty level
    difficulty: U256,
    // Block timestamp
    timestamp: Timestamp,
    // Nonce of the block
    nonce: U256,
    // Total block difficulty
    total_difficulty: U256,
    // Transaction root
    transaction_root: HashDigest,
}

impl BlockHeader
{
    // The 'hash' function returns the hash value of the block
    pub fn hash(&self) -> &HashDigest { &self.hash }
    
    // The `protocol_version` function returns the protocol version of the block
    pub fn protocol_version(&self) -> u32 { self.protocol_version }

    // The `parent_hash` function returns the parent hash of the block
    pub fn parent_hash(&self) -> &HashDigest { &self.parent_hash }

    // The `block_number` function returns the block number
    pub fn block_number(&self) -> BlockNumber { self.block_number }

    // The `block_height` function returns the block height
    pub fn block_height(&self) -> BlockHeight { self.block_height }

    // The `difficulty` function returns the difficulty of the block
    pub fn difficulty(&self) -> &U256 { &self.difficulty }

    // The `timestamp` function returns the timestamp of the block
    pub fn timestamp(&self) -> &Timestamp { &self.timestamp }

    // The `nonce` function returns the nonce of the block
    pub fn nonce(&self) -> &U256 { &self.nonce }

    // The `total_difficulty` function returns the total difficulty of the block
    pub fn total_difficulty(&self) -> &U256 { &self.total_difficulty }

    // The `transaction_root` function returns the transaction root of the block
    pub fn transaction_root(&self) -> &HashDigest { &self.transaction_root }
}



pub struct BlockHeaderBuilder 
{
    hash: HashDigest,
    protocol_version: u32,
    parent_hash: HashDigest,
    block_number: BlockNumber,
    block_height: BlockHeight,
    difficulty: U256,
    timestamp: Timestamp,
    nonce: U256,
    total_difficulty: U256,
    transaction_root: HashDigest,
}

impl Default for BlockHeader
{
    fn default() -> Self 
    {
        Self {
            hash: Default::default(),
            protocol_version: 0,
            parent_hash: Default::default(),
            block_number: 0,
            block_height: 0,
            difficulty: Default::default(),// The `build` function constructs a `BlockHeader` using the provided builder parameters.
            timestamp: Default::default(),
            nonce: Default::default(),
            total_difficulty: Default::default(),
            transaction_root: Default::default(),
        }
    }
}

impl BlockHeaderBuilder
{
    // The 'new' function creates a new block header constructor
    pub fn new() -> Self
    {
        Self {
            hash: Default::default(),
            protocol_version: 0,
            parent_hash: Default::default(),
            block_number: 0,
            block_height: 0,
            difficulty: Default::default(),
            timestamp: Default::default(),
            nonce: Default::default(),
            total_difficulty: Default::default(),
            transaction_root: Default::default(),
        }  
    }

    // The 'set_hash' function sets the hash value of the block
    pub fn set_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.hash = hash;
        self// The `build` function constructs a `BlockHeader` using the provided builder parameters.
    }
    
    // The `set_protocol_version` function sets the protocol version of the block
    pub fn set_protocol_version(&mut self, version: u32) -> &mut Self
    {
        self.protocol_version = version;
        self
    }

    // The `set_parent_hash` function sets the parent hash value of the block
    pub fn set_parent_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.parent_hash = hash;
        self
    }

    // The `set_block_number` function sets the block number
    pub fn set_block_number(&mut self, number: BlockNumber) -> &mut Self
    {
        self.block_number = number;
        self
    }

    // The `set_block_height` function sets the block height
    pub fn set_block_height(&mut self, height: BlockHeight) -> &mut Self
    {
        self.block_height = height;
        self
    }

    // The `set_difficulty` function sets the difficulty of the block
    pub fn set_difficulty(&mut self, difficulty: U256) -> &mut Self
    {
        self.difficulty = difficulty;
        self
    }

    // The `set_timestamp` function sets the timestamp of the block
    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self
    {
        self.timestamp = timestamp;
        self
    }

    // The `set_nonce` function sets the nonce value of the block
    pub fn set_nonce(&mut self, nonce: U256) -> &mut Self
    {
        self.nonce = nonce;
        self
    }

    // The `set_total_difficulty` function sets the total difficulty of the block
    pub fn set_total_difficulty(&mut self, total_difficulty: U256) -> &mut Self
    {
        self.total_difficulty = total_difficulty;
        self
    }

    // The `set_transaction_root` function sets the transaction root of the block
    pub fn set_transaction_root(&mut self, transaction_root: HashDigest) -> &mut Self  
    {
        self.transaction_root = transaction_root;
        self
    }
    
    // The `build` function constructs a `BlockHeader` using the provided builder parameters
    pub fn build(&self) -> BlockHeader
    {
        let hash = self.hash;
        let protocol_version = self.protocol_version;
        let parent_hash = self.parent_hash;
        let block_number = self.block_number;
        let block_height = self.block_height;
        let difficulty = self.difficulty;
        let timestamp = self.timestamp;
        let nonce = self.nonce;
        let total_difficulty = self.total_difficulty;
        let transaction_root = self.transaction_root;

        
        BlockHeader {
            hash,
            protocol_version,
            parent_hash,
            block_number,
            block_height,
            difficulty,
            timestamp,
            nonce,
            total_difficulty,
            transaction_root,
        }
    } 
}

#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn test_block_header_builder()
    {
        let hash = HashDigest::default();
        let protocol_version = 1;
        let parent_hash = HashDigest::default();
        let block_number = 123;
        let block_height = 456;
        let difficulty = U256::from(1000);
        let timestamp = Timestamp::now();
        let nonce = U256::from(12345);
        let total_difficulty = U256::from(5000);
        let transaction_root = HashDigest::default();

        let header = BlockHeaderBuilder::new()
            .set_hash(hash.clone())
            .set_protocol_version(protocol_version)
            .set_parent_hash(parent_hash.clone())
            .set_block_number(block_number)
            .set_block_height(block_height)
            .set_difficulty(difficulty.clone())
            .set_timestamp(timestamp.clone())
            .set_nonce(nonce.clone())
            .set_total_difficulty(total_difficulty.clone())
            .set_transaction_root(transaction_root.clone())
            .build();

        assert_eq!(header.hash(), &hash);
        assert_eq!(header.protocol_version(), protocol_version);
        assert_eq!(header.parent_hash(), &parent_hash);
        assert_eq!(header.block_number(), block_number);
        assert_eq!(header.block_height(), block_height);
        assert_eq!(header.difficulty(), &difficulty);
        assert_eq!(header.timestamp(), &timestamp);
        assert_eq!(header.nonce(), &nonce);
        assert_eq!(header.total_difficulty(), &total_difficulty);
        assert_eq!(header.transaction_root(), &transaction_root);
    }
    
    #[test]
    fn test_block_header_default_values()
    {
        let header = BlockHeader::default();
        assert_eq!(header.hash(), &HashDigest::default());
        assert_eq!(header.protocol_version(), 0);
        assert_eq!(header.parent_hash(), &HashDigest::default());
    }
}
