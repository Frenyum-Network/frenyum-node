use utils::{gas::Gas, timestamp::Timestamp};
use crypto::{hash::HashDigest, hash::Algorithm, ed25519::Signature, ed25519::PublicKey, ed25519::PrivateKey};
use crate::{U256, Address, Bytes};
use serde::Serialize;
use anyhow::anyhow;
use bincode;

// Struct representing a raw transaction
#[derive(Serialize, Clone, Debug)]
pub struct RawTransaction 
{
    // Chain identifier
    chain_id: u32,
    // Nonce for transaction ordering
    nonce: U256,
    // Action to be performed in the transaction
    action: Action,
    // Gas price in the transaction
    gas_price: Gas,
    // Gas limit for the transaction
    gas: Gas,
    // Value (amount) to be transferred in the transaction
    value: U256,
    // Additional data payload for the transaction
    data: Bytes,
}

impl RawTransaction 
{
    // The 'new' function creates a new raw transaction constructor
    pub fn new(
        chain_id: u32,
        nonce: U256,
        action: Action,
        gas_price: Gas,
        gas: Gas,
        value: U256,
        data: Bytes,
    ) -> Self {
        RawTransaction {
            chain_id,
            nonce,
            action,
            gas_price,
            gas,
            value,
            data,
        }
    }

    // The 'sign' function sign the raw transaction
    pub fn sign(
        &self,
        private_key: PrivateKey,
        public_key: PublicKey,
    ) -> SignedTransaction {
        // Serialize raw transaction to bytes
        let raw_tx_bytes = bincode::serialize(self)
            .map_err(|e| anyhow::anyhow!("SerializationError: {}", e)).unwrap();
        // Sign the serialized transaction bytes
        let signature = private_key.sign_message(&raw_tx_bytes);
        // Calculate hash digest of the serialized transaction bytes
        let hash_digest = HashDigest::calculate(&raw_tx_bytes, Algorithm::SHA256).expect("Failed to hash.");
        // Create a signed transaction
        SignedTransaction::new(
            self.clone(),
            public_key.clone(),
            signature,
            hash_digest,
        )
    }

    // The 'to_bytes' function// The 'new' function creates a new raw transaction constructor 
    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error>
    {
        bincode::serialize(self)
    }
}

// Enum representing different types of actions in a transaction
#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum Action
{
    Transfer(TransferAction),
    // Other actions yet to be designed
}

// Struct representing a transfer action
#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct TransferAction
{
   pub to: Address,
   pub amount: U256,
}

// Struct representing a signed transaction
#[derive(Debug)]
pub struct SignedTransaction
{
    // Timestamp of the transaction
    timestamp: Timestamp,
    // Raw transaction being signed
    raw_transaction: RawTransaction,
    // Signature of the transaction
    signature: Signature,
    // Hash digest of the transaction
    hash: HashDigest,
    // Size of the transaction
    size: u32,
}

impl SignedTransaction 
{
    // The 'new' function creates a new signed transaction constructor
    pub fn new(
        raw_transaction: RawTransaction,
        public_key: PublicKey,
        signature: Signature,
        hash: HashDigest,
    ) -> Self {
        let size = std::mem::size_of::<SignedTransaction>() as u32;
        SignedTransaction {
            timestamp: Timestamp::now(),
            raw_transaction,
            signature,
            hash,
            size,
        }
    }

    // The 'get_size' function get the size of the transaction
    pub fn get_size(&self) -> u32
    {
        self.size
    }
    
    // The 'get_hash' function get the hash digest of the transaction
    pub fn get_hash(&self) -> &HashDigest
    {
        &self.hash
    }
}

/// pub enum ExecutionStatus
/// {   
///     Failure,
///     Succes,
///     ...
/// }
///
/// pub struct ExecutionOutcome
/// {
///     Status
///     Gas
///     ...
/// }
///

#[cfg(test)]
mod test
{
    use super::*;
    use rand::rngs::OsRng;
    use anyhow::anyhow;
    use crypto::ed25519::{PrivateKey, PublicKey, Signature};
    
    // Failed Test
    #[test]
    fn test_sign_transaction()
    { 
        let mut csprng: OsRng = OsRng;
        let private_key = PrivateKey::generate(&mut csprng);
        let public_key = private_key.to_public_key();

        let raw_transaction = RawTransaction {
            chain_id: 1,
            nonce: U256::from(12345),
            action: Action::Transfer(TransferAction { 
                to: Address::from([0; 20]), 
                amount: U256::from(100), 
            }),
            gas_price: Gas::from(10),
            gas: Gas::from(1000),
            value: U256::from(500),
            data: Bytes::from(&[1, 2, 3, 4]),
        };

        let signed_transaction = raw_transaction.sign(private_key, public_key);

        assert_eq!(signed_transaction.raw_transaction.chain_id, 1);
        assert_eq!(signed_transaction.raw_transaction.nonce, U256::from(12345));
        assert_eq!(
            signed_transaction.raw_transaction.action,
            Action::Transfer(TransferAction {
                to: Address::from([0; 20]),
                amount: U256::from(100),
            })
        );
        assert_eq!(signed_transaction.raw_transaction.gas_price, Gas::from(10));
        assert_eq!(signed_transaction.raw_transaction.gas, Gas::from(1000));
        assert_eq!(signed_transaction.raw_transaction.value, U256::from(500));
        assert_eq!(signed_transaction.raw_transaction.data, Bytes::from(&[1, 2, 3, 4]));
    }
}
