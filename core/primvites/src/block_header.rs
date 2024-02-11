use crate::crypto::HashDigest;
use crate::utils::Timestamp;

pub struct BlockHeader
{
    // Version
    // Parent Hash
    // Block Number
    // Difficulty
    hash: HashDigest,
    timestamp: Timestamp,
    // Nonce
    // Total Difficulty
    // Gas Price
    // Gas Limit
    // Transaction Root
}
