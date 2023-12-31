use ring::digest;

#[derive(PartialEq)]
struct HashDigest([u8; HashDigest::LENGTH]);

#[derive(Debug, PartialEq)]
pub enum HashError
{
    DataTooLong,
    EmptyData,
}

#[allow(dead_code)]
impl HashDigest 
{
    pub const LENGTH: usize = 32;
    
    pub fn calculate(bytes: &[u8]) -> Result<HashDigest, HashError>
    {   
        if bytes.len() > HashDigest::LENGTH 
        {
           return Err(HashError::DataTooLong);
        }

        if bytes.is_empty()
        {
            return Err(HashError::EmptyData);
        }

        let mut hasher = digest::Context::new(&digest::SHA256);        
        hasher.update(bytes);
        let result = hasher.finish();

        let mut hash_digest = [0u8; HashDigest::LENGTH];
        hash_digest.copy_from_slice(result.as_ref());
        Ok(HashDigest(hash_digest))
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
            HashError::EmptyData => write!(f, "Data is empty!")
        }
    }
}

impl From<[u8; HashDigest::LENGTH]> for HashDigest
{
    fn from(bytes: [u8; HashDigest::LENGTH]) -> Self
    {
        HashDigest(bytes)
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_calculate_hash()
    {
        let hashv1 = HashDigest::calculate(b"FRENYUM");
        let hashv2 = [0xF0, 0xD2, 0x36, 0x19, 0x3D, 0xDE, 0x9D, 0xA8, 0x13, 0x37, 0x98, 0xBD, 0xDA, 0xDC, 0x17, 0x66,
0x59, 0x42, 0x54, 0xD3, 0x56, 0xB0, 0x01, 0x37, 0x62, 0x03, 0x3C, 0xC2, 0x6A, 0xF7, 0xD1, 0x06,
        ];
        
        assert_eq!(hashv1, Ok(HashDigest::from(hashv2)));
    }

    #[test]
    fn test_eq_hash()
    {
        let hashv1 = HashDigest::calculate(b"FRENYUM");
        let hashv2 = HashDigest::calculate(b"FRENYUM");
        let hashv3 = HashDigest::calculate(b"NOT_EQ");

        assert_eq!(hashv1, hashv2);
        assert_ne!(hashv1, hashv3);
    }

}






