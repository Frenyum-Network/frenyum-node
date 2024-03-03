use utils::{gas::Gas, timestamp::Timestamp};
use crypto::{hash::HashDigest, ed25519::Signature};
use crate::{U256, Bytes};

pub struct RawTransaction
{
    chain_id: u32,
    nonce: U256,
    // Action
    gas_price: Gas,
    gas: Gas,
    value: U256,
    data: Bytes,
}

pub struct SignedTransaction
{
    timestamp: Timestamp,
    raw_transaction: RawTransaction,
    signature: Signature,
    hash: HashDigest,
    size: u32,
}
