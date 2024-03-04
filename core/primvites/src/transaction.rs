use utils::{gas::Gas, timestamp::Timestamp};
use crypto::{hash::HashDigest, ed25519::Signature, ed25519::PublicKey, ed25519::PrivateKey};
use crate::{U256, Adress, Bytes};

pub enum TransactionError
{
    HashError,
}

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

impl RawTransaction
{
    pub fn sign(
        &self,
        private_key: PrivateKey, 
        public_key: PublicKey 
    ) -> SignedTransaction {
        let signature = private_key.sign(self);
        let hash_digest = HashDigest::calculate(self.to_byte(), Algorithm::SHA256) {
            Ok(hash) => hash,
            Err(_) => return Err(TransactionError::HashError),
        };
        Ok(SignedTransaction::new(self, public_key, signature, hash_digest))
    }
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
   pub to: Adress,
   pub amount: U256,
}

pub struct SignedTransaction
{
    timestamp: Timestamp,
    raw_transaction: RawTransaction,
    signature: Signature,
    hash: HashDigest,
    size: u32,
}

impl SignedTransaction
{
    pub fn new(
        raw_transaction: RawTransaction,
        public_key: PublicKey,
        signature: Signature
        hash_digest: HashDigest,
    ) -> Self {
        let size = std::mem::size_of::<Self>();
        SignedTransaction {
            timestamp: Timestamp::now(),
            raw_transaction,
            signature,
            hash: hash_digest
            size: size as u32
        }
    }
}
