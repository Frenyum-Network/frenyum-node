use utils::{gas::Gas, timestamp::Timestamp};
use crypto::{hash::HashDigest, ed25519::Signature, ed25519::PublicKey, ed25519::PrivateKey};
use crate::{U256, Adress, Bytes};

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
            gas_price,
            gas,
            value,
            data,
        };
    }
    pub fn sign(
        &self,
        private_key: PrivateKey, 
        public_key: PublicKey 
    ) -> SignedTransaction {
        let signature = private_key.sign(self);
        let hash_digest = HashDigest::calculate(self.to_byte(), Algorithm::SHA256).expect("Failed to hash.");
        SignedTransaction::new(self.clone(), public_key.clone(), signature, hash_digest)
        }
   }

pub enum Action
{
    Transfer(TransferAction),
    // Other actions yet to be designed
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
        let size = std::mem::size_of::<SignedTransaction>() as 32;
        SignedTransaction {
            timestamp: Timestamp::now(),
            raw_transaction,
            signature,
            hash_digest,
            size,
        }
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
    use rand_core::os::OsRng;
    use utils::ed25519::*;

    #[test]
    fn test_sign_transaction()
    { 
        let mut csprng: OsRng = OsRng;
        let private_key = PrivateKey::generate(&mut csprng);
        let public_key = private_key.to_public_key();

        let raw_transaction = RawTransaction {
            chain_id: 1,
            nonce: U256::from(12345),
            action: Action::Tranasfer(TransferAction { 
                to: Adress::new([0; 20]), 
                amount: U256::from(100), 
            }),
            gas_price: Gas::from(10),
            gas: Gas::from(1000),
            value: U256::from(),
            data: Bytes::from(&[1, 2, 3, 4]),
        };

        let signed_transaction = raw_transaction.sign(private_key, public_key);

        assert_eq!(signed_transaction.raw_transaction.chain_id, 1);
        assert_eq!(signed_transaction.raw_transaction.nonce, U256::from(12345));
        assert_eq!(
            signed_transaction.raw_transaction.action,
            Action::Transfer(TransferAction {
                to: Address::new([0; 20]),
                amount: U256::from(100),
            })
        );
        assert_eq!(signed_transaction.raw_transaction.gas_price, Gas::from(10));
        assert_eq!(signed_transaction.raw_transaction.gas, Gas::from(1000));
        assert_eq!(signed_transaction.raw_transaction.value, U256::from(500));
        assert_eq!(signed_transaction.raw_transaction.data, Bytes::from(&[1, 2, 3, 4]));
    }
}
