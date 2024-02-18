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
            difficulty: Default::default(),
            timestamp: Default::default(),
            nonce: Default::default(),
            total_difficulty: Default::default(),
            gas_used: Default::default(),
            gas_limit: Default::default(),
            transaction_root: Default::default(),
        }
    }
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
        let gas_used = Gas::new(100);
        let gas_limit = Gas::new(1000);
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
            .set_gas_used(gas_used.clone())
            .set_gas_limit(gas_limit.clone())
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
        assert_eq!(header.gas_used(), &gas_used);
        assert_eq!(header.gas_limit(), &gas_limit);
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
