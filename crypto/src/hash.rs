use sha3::{Digest, Sha3_256};
use ring::digest::*;
use anyhow::Error as AnyhowError;

#[derive(PartialEq)]
pub struct HashDigest(pub [u8; HashDigest::LENGTH]);

#[derive(Debug, PartialEq)]
pub enum HashError
{
    HexError,
    SerializationError,
    DataTooLong,
    EmptyData,
    UnsupportedAlgorithm,
}

pub enum Algorithm
{
    Keccak,
    SHA256,
    SHA512,
}


#[allow(dead_code)]
impl HashDigest 
{
    pub const LENGTH: usize = 32;
    
    pub fn calculate(bytes: &[u8], algorithm: Algorithm) -> Result<HashDigest, HashError>
    {   
        if bytes.len() > HashDigest::LENGTH 
        {
           return Err(HashError::DataTooLong);
        }

        if bytes.is_empty()
        {
            return Err(HashError::EmptyData);
        }
        
        let mut hash_digest = [0u8; HashDigest::LENGTH];

        match algorithm 
        {
            Algorithm::SHA256 => {
                let mut context = Context::new(&SHA256);
                context.update(bytes);
                let result = context.finish();
                hash_digest.copy_from_slice(result.as_ref());
                Ok(HashDigest(hash_digest))
            }
            Algorithm::SHA512 => {
                let mut context = Context::new(&SHA512);
                context.update(bytes);
                let result = context.finish();
                hash_digest.copy_from_slice(result.as_ref());
                Ok(HashDigest(hash_digest))
            }
            Algorithm::Keccak =>
            {
                 let hash_digest: [u8; HashDigest::LENGTH] = Sha3_256::digest(bytes).into();
                 Ok(HashDigest(hash_digest))
            }
            _ => return Err(HashError::UnsupportedAlgorithm),
        }
    }

    pub fn eq(&self, hash: &HashDigest) -> bool
    {
        &self.0 == &hash.0
    }

    pub fn to_string(&self) -> String
    {
        format!("{:02X?}", self.0)
    }

    pub fn to_byte(&self) -> [u8; HashDigest::LENGTH]
    {
        self.0
    }
}

impl From<Vec<u8>> for HashDigest
{
    fn from(vec: Vec<u8>) -> Self
    {
        let mut hash_digest = [0u8; HashDigest::LENGTH];
        hash_digest.copy_from_slice(&vec[..]);
        HashDigest(hash_digest)
    }
}

impl AsRef<[u8]> for HashDigest
{
    fn as_ref(&self) -> &[u8]
    {
        &self.0
    }
}

impl std::fmt::Debug for HashDigest
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "HashDigest({})", self.to_string())
    }
}

impl std::fmt::Display for HashError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            HashError::DataTooLong => write!(f, "Data is too long!"),
            HashError::HexError => write!(f, "Hex failed!"),
            HashError::SerializationError => write!(f, "Serialization failed!"),
            HashError::EmptyData => write!(f, "Data is empty!"),
            HashError::UnsupportedAlgorithm => write!(f, "Unsupported algorithm!"),
        }
    }
}

impl From<HashError> for AnyhowError
{
    fn from(err: HashError) -> Self
    {
        AnyhowError::msg(err.to_string())
    }
}

impl From<[u8; HashDigest::LENGTH]> for HashDigest
{
    fn from(bytes: [u8; HashDigest::LENGTH]) -> Self
    {
        HashDigest(bytes)
    }
}

pub fn hex_digest(algorithm: Algorithm, data: &[u8]) -> Result<HashDigest, HashError>
{
    match algorithm {
        Algorithm::SHA256 => HashDigest::calculate(data, Algorithm::SHA256),
        Algorithm::Keccak => HashDigest::calculate(data, Algorithm::Keccak),
        Algorithm::SHA512 => HashDigest::calculate(data, Algorithm::SHA512),
    }
}

pub trait CryptoHash 
{
    fn hash(&self) -> Result<HashDigest, HashError>;
}

impl CryptoHash for [u8]
{
    fn hash(&self) -> Result<HashDigest, HashError>
    {
        let hash = hex_digest(Algorithm::SHA256, self)?;
        let hash_bytes = hex::decode(hash.as_ref())
            .map_err(|_| HashError::HexError)?;

        Ok(hash_bytes.into())
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_keccak_calculate_hash()
    {
        let hashv1 = HashDigest::calculate(b"FRENYUM", Algorithm::Keccak);
        let hashv2 = [
         0x32, 0x1A, 0x2C, 0xBA, 0xE1, 0x4F, 0xD6, 0x2C, 0x8B, 0xD9, 0x82, 0x83, 0x75, 0x8A, 0x38, 0xBD,
         0xB6, 0x0F, 0xD1, 0x06, 0x10, 0x97, 0xF0, 0xA4, 0x50, 0x13, 0x38, 0x6A, 0x7F, 0x3D, 0x2B, 0x2B,
        ];

        
        assert_eq!(hashv1, Ok(HashDigest::from(hashv2)));
    }

    #[test]
    fn test_keccak_eq_hash()
    {
        let hashv1 = HashDigest::calculate(b"FRENYUM", Algorithm::Keccak);
        let hashv2 = HashDigest::calculate(b"FRENYUM", Algorithm::Keccak);
        let hashv3 = HashDigest::calculate(b"NOT_EQ", Algorithm::Keccak);

        assert_eq!(hashv1, hashv2);
        assert_ne!(hashv1, hashv3);
    }

}






