use crypto::hash::HashDigest;
use utils::gas::Gas;
use utils::timestamp::Timestamp;
use crate::BlockNumber;
use crate::BlockHeight;

pub struct BlockHeader
{
    // Version
    // Parent Hash
    block_number: BlockNumber,
    block_height: BlockHeight,
    // Difficulty
    hash: HashDigest,
    timestamp: Timestamp,
    // Nonce
    // Total Difficulty
    gas_used: Gas,
    gas_limit: Gas,
    // Transaction Root
}
