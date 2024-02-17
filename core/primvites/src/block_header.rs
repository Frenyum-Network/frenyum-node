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

#[derive(Debug)]
pub enum BuilderError
{
    HashMissing,
    ProtocolVersionMissing,
    ParentHashMissing,
    BlockNumberMissing,
    BlockHeightMissing,
    DifficultyMissing,
    TimestampMissing,
    NonceMissing,
    TotalDifficultyMissing,
    GasUsedMissing,
    GasLimitMissing,
    TransactionRootMissing,
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

    pub fn set_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.hash = Some(hash);
        self
    }

    pub fn set_protocol_version(&mut self, version: u32) -> &mut Self
    {
        self.protocol_version = Some(version);
        self
    }

    pub fn set_parent_hash(&mut self, hash: HashDigest) -> &mut Self
    {
        self.parent_hash = Some(hash);
        self
    }

    pub fn set_block_number(&mut self, number: BlockNumber) -> &mut Self
    {
        self.block_number = Some(number);
        self
    }

    pub fn set_block_height(&mut self, height: BlockHeight) -> &mut Self
    {
        self.block_height = Some(height);
        self
    }

    pub fn set_difficulty(&mut self, difficulty: U256) -> &mut Self
    {
        self.difficulty = Some(difficulty);
        self
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self
    {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn set_nonce(&mut self, nonce: U256) -> &mut Self
    {
        self.nonce = Some(nonce);
        self
    }

    pub fn set_total_difficulty(&mut self, total_difficulty: U256) -> &mut Self
    {
        self.total_difficulty = Some(total_difficulty);
        self
    }

    pub fn set_gas_used(&mut self, gas_used: Gas) -> &mut Self
    {
        self.gas_used = Some(gas_used);
        self
    }

    pub fn set_gas_limit(&mut self, gas_limit: Gas) -> &mut Self
    {
        self.gas_limit = Some(gas_limit);
        self
    }

    pub fn set_transaction_root(&mut self, transaction_root: HashDigest) -> &mut Self  
    {
        self.transaction_root = Some(transaction_root);
        self
    }

    pub fn build(&self) -> Result<BlockHeader, BuilderError>
    {
        let hash = self.hash.ok_or(BuilderError::HashMissing)?;
        let protocol_version = self.protocol_version.ok_or(BuilderError::ProtocolVersionMissing)?;
        let parent_hash = self.parent_hash.ok_or(BuilderError::ParentHashMissing)?;
        let block_number = self.block_number.ok_or(BuilderError::BlockNumberMissing)?;
        let block_height = self.block_height.ok_or(BuilderError::BlockHeightMissing)?;
        let difficulty = self.difficulty.ok_or(BuilderError::DifficultyMissing)?;
        let timestamp = self.timestamp.ok_or(BuilderError::TimestampMissing)?;
        let nonce = self.nonce.ok_or(BuilderError::NonceMissing)?;
        let total_difficulty = self.total_difficulty.ok_or(BuilderError::TotalDifficultyMissing)?;
        let gas_used = self.gas_used.ok_or(BuilderError::GasUsedMissing)?;
        let gas_limit = self.gas_limit.ok_or(BuilderError::GasLimitMissing)?;
        let transaction_root = self.transaction_root.ok_or(BuilderError::TransactionRootMissing)?;

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
