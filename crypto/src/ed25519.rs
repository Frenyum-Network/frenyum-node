extern crate ed25519_dalek;

use rand::rngs::OsRng;
use ed25519_dalek::*;
use anyhow::Result;
use crate::hash::*;
use serde::Serialize;
use bincode::serialize_into;

pub struct PrivateKey(ed25519_dalek::SecretKey);

impl PrivateKey {
    pub const LENGTH: usize = ed25519_dalek::SECRET_KEY_LENGTH;

    pub fn generate() -> Self 
    {
        let mut csprng = OsRng;
        let secret_key = ed25519_dalek::SecretKey::generate(&mut csprng);
        PrivateKey(secret_key)
    }

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] 
    {
        self.0.to_bytes()
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature 
    {
        let secret_key: &SecretKey = &self.0;
        let expanded_secret_key: ExpandedSecretKey = ExpandedSecretKey::from(secret_key);
        let public_key: PublicKey = expanded_secret_key.into();
        let sig = expanded_secret_key.sign(message, &public_key.0);
        Signature(sig)
    }
}

pub struct PublicKey(ed25519_dalek::PublicKey);

impl From<ExpandedSecretKey> for PublicKey
{
    fn from(expanded_secret_key: ExpandedSecretKey) -> Self
    {
        PublicKey((&expanded_secret_key).into())
    }
}

impl From<ed25519_dalek::PublicKey> for PublicKey
{
    fn from(public_key: ed25519_dalek::PublicKey) -> Self
    {
        PublicKey(public_key)
    }
}

impl PublicKey {
    pub const LENGTH: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] 
    {
        self.0.to_bytes()
    }
    
    pub fn from_bytes(bytes: &[u8; Self::LENGTH]) -> Result<Self, anyhow::Error>
    {
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
        T: CryptoHash + Serialize + std::hash::BuildHasher,
    {
        let mut bytes = Vec::new();
        serialize_into(&mut bytes, &message)
            .map_err(|_| anyhow::anyhow!("SerializationError"))?;
        
        let hash = hex_digest(Algorithm::SHA256, &bytes);
        let hash_bytes = hex::decode(hash)?;

        if public_key.0.verify(&hash_bytes, &self.0).is_ok() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Signature verification failed!"))
        }
    }
}

