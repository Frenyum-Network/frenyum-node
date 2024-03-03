use utils::{gas::Gas, timestamp::Timestamp};
use crypto::{hash::HashDigest, ed25519::Signature};
use crate::{U256, Bytes};

pub struct RawTransaction
{
    chain_id: u32,
    nonce: U256,
    action: Action,
    gas_price: Gas,
    gas: Gas,
    value: U256,
    data: Bytes,
}

pub enum Action
{
    Transfer(TransferAction),
    // TokenTransfer
    // CreateAccount
    // ContractCall
    // ContractDeploy
    // DelegateCall
    // Swap
}

pub struct TransferAction
{
    // to
    amount: U256,
}

pub struct SignedTransaction
{
    timestamp: Timestamp,
    raw_transaction: RawTransaction,
    signature: Signature,
    hash: HashDigest,
    size: u32,
}
