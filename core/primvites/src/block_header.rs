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

#[derive(Debug)]
pub enum BuilderError
{
    MissingField(&'static str),
}

impl std::fmt::Display for BuilderError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            BuilderError::MissingField(field) => write!(f, "{} is missing", field),
        }
    }
}

impl std::error::Error for BuilderError {}


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
            difficulty: Default::default();,
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

    pub fn build(&self) -> Result<BlockHeader, BuilderError>
    {
        let fields = [
            (self.hash.as_ref(), "Hash"),
            (self.protocol_version, "Protocol version"),
            (self.parent_hash.as_ref(), "Parent hash"),
            (self.block_number, "Block number"),
            (self.block_height, "Block height"),
            (self.difficulty.as_ref(), "Difficulty"),
            (self.timestamp.as_ref(), "Timestamp"),
            (self.nonce.as_ref(), "Nonce"),
            (self.total_difficulty.as_ref(), "Total difficulty"),
            (self.gas_used.as_ref(), "Gas used"),
            (self.gas_limit.as_ref(), "Gas limit"),
            (self.transaction_root.as_ref(), "Transaction root"),
        ];

        for (field_value, field_name) in &fields
        {
            if field_value.is_none()
            {
                return Err(BuilderError::MissingField(*field_name));
            }
        }

        Ok(BlockHeader {
            hash: self.hash.clone().unwrap(),
            protocol_version: self.protocol_version.unwrap(),
            parent_hash: self.parent_hash.clone().unwrap(),
            block_number: self.block_number.unwrap(),
            block_height: self.block_height.unwrap(),
            difficulty: self.difficulty.clone().unwrap(),
            timestamp: self.timestamp.clone().unwrap(),
            nonce: self.nonce.clone().unwrap(),
            total_difficulty: self.total_difficulty.clone().unwrap(),
            gas_used: self.gas_used.clone().unwrap(),
            gas_limit: self.gas_limit.clone().unwrap(),
            transaction_root: self.transaction_root.clone().unwrap(),
        })
    }
    
    pub fn clear(&mut self)
    {
        self.hash = None;
        self.protocol_version = None;
        self.parent_hash = None;
        self.block_number = None;
        self.block_height = None;
        self.difficulty = None;
        self.timestamp = None;
        self.nonce = None;
        self.total_difficulty = None;
        self.gas_used = None;
        self.gas_limit = None;
        self.transaction_root = None;
    }
    
}
