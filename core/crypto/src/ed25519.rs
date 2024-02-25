extern crate ed25519_dalek;

use std::convert::TryFrom;
use rand::rngs::OsRng;
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
        let mut csprng = OsRng;
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
        let expanded_secret_key: ExpandedSecretKey = ExpandedSecretKey::from(secret_key);
        let public_key: PublicKey = self.into();
        let sign = expanded_secret_key.sign(message, &public_key.0);

        Signature(sign)
    }
}

impl From<&PrivateKey> for PublicKey
{
    fn from(private_key: &PrivateKey) -> Self 
    {
        PublicKey(ed25519_dalek::PublicKey::from(&private_key.0))
    }
}

impl From<ExpandedSecretKey> for PublicKey 
{
    fn from(expanded_secret_key: ExpandedSecretKey) -> Self
    {
        PublicKey(ed25519_dalek::PublicKey::from(&expanded_secret_key))
    }
}

impl From<&ed25519_dalek::SecretKey> for PublicKey
{
    fn from(secret_key: &ed25519_dalek::SecretKey) -> Self
    {
        PublicKey::from(ExpandedSecretKey::from(secret_key))
    }
}

#[derive(PartialEq, Debug)]
pub struct PublicKey(ed25519_dalek::PublicKey);

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

#[derive(PartialEq, Debug)]
pub struct Signature(ed25519_dalek::Signature);

impl Signature {
    pub const LENGTH: usize = ed25519_dalek::SIGNATURE_LENGTH;

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

        let signature_bytes = bytes.to_vec();
        let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes)
            .map_err(|_| anyhow::anyhow!("Failed to create signature"))?;

        Ok(Signature(signature))
    }

    pub fn verify<T>(
        &self,
        message: &T,
        public_key: &PublicKey,
    ) -> Result<(), anyhow::Error>
    where
        T: CryptoHash + Serialize + ?Sized,
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

    #[test]
    fn test_public_key_from_bytes_invalid_input()
    {
        let mut invalid_bytes = [0u8; PublicKey::LENGTH];
        invalid_bytes[0] = 1;
        let result = PublicKey::from_bytes(&invalid_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_signature_to_bytes_serialization()
    {
        let private_key = PrivateKey::generate();
        let message = "FRENYUM";
        let signature = private_key.sign_message(message.as_bytes());
        let bytes = signature.to_bytes();
        let restored_signature = Signature::from_bytes(&bytes).unwrap();
        assert_eq!(signature, restored_signature);
    }

    #[test]
    fn test_signature_verify()
    {
        let private_key = PrivateKey::generate();
        let message = "FRENYUM_OK"; 
        let signature = private_key.sign_message(message.as_bytes());

        let public_key = private_key.to_public_key();

        assert!(signature.verify(message.as_bytes(), &public_key).is_ok());
    }
}
