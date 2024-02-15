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
    hash: Option<HashDigest>,
    protocol_version: Option<u32>,
    parent_hash: Option<HashDigest>,
    block_number: Option<BlockNumber>,
    block_height: Option<BlockHeight>,
    difficulty: Option<U256>,
    timestamp: Option<Timestamp>,
    nonce: Option<U256>,
    total_difficulty: Option<U256>,
    gas_used: Option<Gas>,
    gas_limit: Option<Gas>,
    transaction_root: Option<HashDigest>,
}

impl BlockHeaderBuilder
{
    pub fn new() -> Self
    {
        Self {
            hash: None,
            protocol_version: None,
            parent_hash: None,
            block_number: None,
            block_height: None,
            difficulty: None,
            timestamp: None,
            nonce: None,
            total_difficulty: None,
            gas_used: None,
            gas_limit: None,
            transaction_root: None,
        }  
    }

    pub fn hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.hash = Some(hash);
        self
    }

    pub fn protocol_version(&mut self, version: u32) -> &mut Self
    {
        self.protocol_version = Some(version);
        self
    }

    pub fn parent_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.parent_hash = Some(hash);
        self
    }

    pub fn block_number(&mut self, number: BlockNumber) -> &mut Self
    {
        self.block_number = Some(number);
        self
    }

    pub fn block_height(&mut self, height: BlockHeight) -> &mut Self
    {
        self.block_height = Some(height);
        self
    }

    pub fn difficulty(&mut self, difficulty: U256) -> &mut Self
    {
        self.difficulty = Some(difficulty);
        self
    }

    pub fn timestamp(&mut self, timestamp: Timestamp) -> &mut Self
    {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn nonce(&mut self, nonce: U256) -> &mut Self
    {
        self.nonce = Some(nonce);
        self
    }

    pub fn total_difficulty(&mut self, total_difficulty: U256) -> &mut Self
    {
        self.total_difficulty = Some(total_difficulty);
        self
    }

    pub fn gas_used(&mut self, gas_used: Gas) -> &mut Self
    {
        self.gas_used = Some(gas_used);
        self
    }

    pub fn gas_limit(&mut self, gas_limit: Gas) -> &mut Self
    {
        self.gas_limit = Some(gas_limit);
        self
    }

    pub fn transaction_root(&mut self, transaction_root: HashDigest) -> &mut Self  
    {
        self.transaction_root = Some(transaction_root);
        self
    }
}
