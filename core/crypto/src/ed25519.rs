extern crate ed25519_dalek;

use std::convert::TryFrom;
use rand::RngCore;
use ed25519_dalek::*;
use anyhow::Result;
use crate::hash::*;
use serde::Serialize;
use bincode::serialize_into;
use ring::agreement::PublicKey as RingPublicKey;

pub struct PrivateKey(ed25519_dalek::SecretKey);

impl PrivateKey {
    pub const LENGTH: usize = ed25519_dalek::SECRET_KEY_LENGTH;

    pub fn generate() -> Self 
    {
        let mut csprng = rand::thread_rng();
        let secret_key = ed25519_dalek::SecretKey::generate(&mut csprng);
        PrivateKey(secret_key)
    }

    pub fn to_public_key(&self) -> PublicKey
    {
        PublicKey::from(&self.0)
    }


    pub fn to_bytes(&self) -> [u8; Self::LENGTH] 
    {
        self.0.to_bytes()
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature 
    {
        let secret_key: &SecretKey = &self.0;
        let signer = Signer::from_secret_key(secret_key);
        let sign = signer.sign(message);
        
        Signature(sign)
    }
}

impl From<&PrivateKey> for PublicKey
{
    fn from(private_key: &PrivateKey) -> Self 
    {
        PublicKey(PublicKey::from(&private_key.0))
    }
}

impl From<&ed25519_dalek::SecretKey> for PublicKey
{
    fn from(secret_key: &ed25519_dalek::SecretKey) -> Self
    {
        PublicKey::from(secret_key)
    }
}

#[derive(PartialEq, Debug)]
pub struct PublicKey(RingPublicKey);

impl PublicKey {
    pub const LENGTH: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] 
    {
        self.0.to_bytes()
    }
    
    pub fn from_bytes(bytes: &[u8; Self::LENGTH]) -> Result<Self, anyhow::Error>
    {
        if bytes.len() != Self::LENGTH
        {
            return Err(anyhow::anyhow!("Invalid byte array length"));
        }

        PublicKey::from_bytes(bytes).map_err(|_| anyhow::anyhow!("Failed to create public key"))
    }
}



pub struct Signature(ed25519_dalek::Signature);

impl Signature {
    pub const LENGTH: usize = ed25519_dalek::SIGNATURE_LENGTH;

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] 
    {
        self.0.to_bytes()
    }

    pub fn verify<T>(
        &self,
        message: &T,
        public_key: &PublicKey,
    ) -> Result<(), anyhow::Error>
    where
        T: CryptoHash + Serialize,
    {
        let mut bytes = Vec::new();
        serialize_into(&mut bytes, &message)
            .map_err(|_| anyhow::anyhow!("SerializationError"))?;
        
        let binding = bytes.hash()?;
        let hash_bytes = binding.as_ref();

        if public_key.0.verify(&hash_bytes, &self.0).is_ok() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Signature verification failed!"))
        }
    }
}

mod test {
    use super::*;
    use crate::hash::HashDigest;

    #[test]
    fn test_private_key_generate_creates_valid_key()
    {
        let private_key = PrivateKey::generate();
        assert_eq!(private_key.0.to_bytes().len(), PrivateKey::LENGTH);
    }

    #[test]
    fn test_private_key_to_public_key_conversion()
    {
        let private_key = PrivateKey::generate();
        let public_key = private_key.to_public_key();
    }

    #[test]
    fn test_public_key_from_bytes_serialization()
    {
        let private_key = PrivateKey::generate();
        let public_key = private_key.to_public_key();
        let bytes = public_key.to_bytes();
        let deserialized_public_key = PublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(public_key, deserialized_public_key);
    }
}
