use crate::rocksdb::RocksDB;
pub use rocksdb::{DBIterator, IteratorMode};

// Structure for RocksDB Iteration
pub struct RocksDBIterator<'a> 
{ 
    iter: rocksdb::DBIterator<'a>, // Iterator for RocksDB
}


impl<'a> RocksDBIterator<'a>
{   
    // Constructor function used to create a new RocksDBIterator instance
    pub fn new(iter: rocksdb::DBIterator<'a>) -> Self
    {
        RocksDBIterator { iter }
    }
}

// Implement the Iterator trait for RocksDBIterator
impl<'a> Iterator for RocksDBIterator<'a>
{
    // The Item relation of the Iterator is a tuple of type Box<[u8]> for both key and value
    type Item = (Box<[u8]>, Box<[u8]>);
    
    // Function to get the next item
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(Ok((key, value))) => Some((key, value)),
            _ => None,
        }
    }
}
