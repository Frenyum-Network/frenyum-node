use crypto::hash::HashDigest;
use utils::{gas::Gas, timestamp::Timestamp};
use crate::{BlockNumber, BlockHeight, U256};

pub struct BlockHeader
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
    gas_used: Gas,
    gas_limit: Gas,
    transaction_root: HashDigest,
}

impl BlockHeader
{
    pub fn hash(&self) -> &HashDigest { &self.hash }

    pub fn protocol_version(&self) -> u32 { self.protocol_version }

    pub fn parent_hash(&self) -> &HashDigest { &self.parent_hash }

    pub fn block_number(&self) -> BlockNumber { self.block_number }

    pub fn block_height(&self) -> BlockHeight { self.block_height }

    pub fn difficulty(&self) -> &U256 { &self.difficulty }

    pub fn timestamp(&self) -> &Timestamp { &self.timestamp }

    pub fn nonce(&self) -> &U256 { &self.nonce }

    pub fn total_difficulty(&self) -> &U256 { &self.total_difficulty }

    pub fn gas_used(&self) -> &Gas { &self.gas_used }

    pub fn gas_limit(&self) -> &Gas { &self.gas_limit }

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
    gas_used: Gas,
    gas_limit: Gas,
    transaction_root: HashDigest,
}

impl BlockHeaderBuilder
{
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
            gas_used: Default::default(),
            gas_limit: Default::default(),
            transaction_root: Default::default(),
        }  
    }

    pub fn set_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.hash = hash;
        self
    }

    pub fn set_protocol_version(&mut self, version: u32) -> &mut Self
    {
        self.protocol_version = version;
        self
    }

    pub fn set_parent_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.parent_hash = hash;
        self
    }

    pub fn set_block_number(&mut self, number: BlockNumber) -> &mut Self
    {
        self.block_number = number;
        self
    }

    pub fn set_block_height(&mut self, height: BlockHeight) -> &mut Self
    {
        self.block_height = height;
        self
    }

    pub fn set_difficulty(&mut self, difficulty: U256) -> &mut Self
    {
        self.difficulty = difficulty;
        self
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self
    {
        self.timestamp = timestamp;
        self
    }

    pub fn set_nonce(&mut self, nonce: U256) -> &mut Self
    {
        self.nonce = nonce;
        self
    }

    pub fn set_total_difficulty(&mut self, total_difficulty: U256) -> &mut Self
    {
        self.total_difficulty = total_difficulty;
        self
    }

    pub fn set_gas_used(&mut self, gas_used: Gas) -> &mut Self
    {
        self.gas_used = gas_used;
        self
    }

    pub fn set_gas_limit(&mut self, gas_limit: Gas) -> &mut Self
    {
        self.gas_limit = gas_limit;
        self
    }

    pub fn set_transaction_root(&mut self, transaction_root: HashDigest) -> &mut Self  
    {
        self.transaction_root = transaction_root;
        self
    }

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
        let gas_used = self.gas_used;
        let gas_limit = self.gas_limit;
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
            gas_used,
            gas_limit,
            transaction_root,
        }
    }
    
    pub fn clear(&mut self)
    {
        self.hash = Default::default();
        self.protocol_version = 0;
        self.parent_hash = Default::default();
        self.block_number = 0;
        self.block_height = 0;
        self.difficulty = Default::default();
        self.timestamp = Default::default();
        self.nonce = Default::default();
        self.total_difficulty = Default::default();
        self.gas_used = Default::default();
        self.gas_limit = Default::default();
        self.transaction_root = Default::default();
    }
    
}
