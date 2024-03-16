//! # Hash Algorithms Scheme
//!
//! This module provides hash computation capabilities using SHA-256, SHA-512 and Keccak algorithms.
//! The HashDigest type represents a digest based on an array.
//!
//! # Examples
//!
//! ```rust
//! use crate::crypto::{Algorithm, HashDigest};
//! 
//! let hash_result = HashDigest::calculate(b"FRENYUM_TEST", Algorithm::SHA256);
//! match hash_result {
//!     Ok(hash) => println!("SHA-256 hash: {:?}", hash),
//!     Err(e) => eprintln!("Error: {:?}", e),
//! }
//! ```

use sha3::{Digest, Sha3_256};
use ring::digest::*;
use anyhow::Error as AnyhowError;
use std::default::Default;

// Output value of our hash function.
#[derive(Clone, Copy, PartialEq, Default)]
pub struct HashDigest(pub [u8; HashDigest::LENGTH]);

// Represents errors that can occur during hash operations.
#[derive(Debug, PartialEq)]
pub enum HashError
{
    HexError,
    SerializationError,
    DataTooLong,
    EmptyData,
    UnsupportedAlgorithm,
}

// Represents supported hashing algorithms.
pub enum Algorithm
{
    Keccak,
    SHA256,
    SHA512,
}


#[allow(dead_code)]
impl HashDigest 
{
    // The length of the HashDigest
    pub const LENGTH: usize = 32;
    
    // The 'calculate' function calculates a summary using the specified algorithm on the specified data.
    pub fn calculate(bytes: &[u8], algorithm: Algorithm) -> Result<HashDigest, HashError>
    {   
         // Check the data length and return an error if necessary.
        if bytes.len() > HashDigest::LENGTH 
        {
           return Err(HashError::DataTooLong);
        }

        // Check if the data is empty and return an error if necessary.
        if bytes.is_empty()
        {
            return Err(HashError::EmptyData);
        }
        
        let mut hash_digest = [0u8; HashDigest::LENGTH];
        
        // Calculate the hash using the specified algorithm.
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

    // The 'eq' function checks if two `HashDigests` are equal.
    pub fn eq(&self, hash: &HashDigest) -> bool
    {
        &self.0 == &hash.0
    }

    // The 'to_string' function converts `HashDigest` to hexadecimal.
    pub fn to_string(&self) -> String
    {
        format!("{:02X?}", self.0)
    }

    // The 'to_bytes' function converts `HashDigest` to a byte array.
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

impl std::fmt::Display for HashDigest
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
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

// The 'hex_digest' function calculates a summary of data with the specified algorithm.
pub fn hex_digest(algorithm: Algorithm, data: &[u8]) -> Result<HashDigest, HashError>
{
    match algorithm {
        Algorithm::SHA256 => HashDigest::calculate(data, Algorithm::SHA256),
        Algorithm::Keccak => HashDigest::calculate(data, Algorithm::Keccak),
        Algorithm::SHA512 => HashDigest::calculate(data, Algorithm::SHA512),
    }
}

// 'CryptoHash` defines the functions required to compute a hash of data.
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

    #[test]
    fn test_sha256_calculate_hash()
    {
        let hashv1 = HashDigest::calculate(b"FRENYUM", Algorithm::SHA256);
        let hashv2 =  [
        0xF0, 0xD2, 0x36, 0x19, 0x3D, 0xDE, 0x9D, 0xA8, 0x13, 0x37, 0x98, 0xBD, 0xDA, 0xDC,
        0x17, 0x66, 0x59, 0x42, 0x54, 0xD3, 0x56, 0xB0, 0x01, 0x37, 0x62, 0x03, 0x3C, 0xC2,
        0x6A, 0xF7, 0xD1, 0x06
        ];

        assert_eq!(hashv1, Ok(HashDigest::from(hashv2)));
    }
    
    #[test]
    fn test_sha256_eq_hash()
    {
        let hashv1 = HashDigest::calculate(b"FRENYUM", Algorithm::SHA256);
        let hashv2 = HashDigest::calculate(b"FRENYUM", Algorithm::SHA256);
        let hashv3 = HashDigest::calculate(b"NOT_EQ", Algorithm::SHA256);

        assert_eq!(hashv1, hashv2);
        assert_ne!(hashv2, hashv3);

    }

}






