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
