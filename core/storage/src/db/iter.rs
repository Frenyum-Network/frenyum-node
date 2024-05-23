pub struct RocksDBIterator<'a> 
{ 
    iter: rocksdb::DBIterator<'a>,
}

impl<'a> RocksDBIterator<'a>
{
    pub fn new(iter: rocksdb::DBIterator<'a>) -> Self
    {
        RocksDBIterator { iter }
    }
}

impl<'a> Iterator for RocksDBIterator<'a>
{
    type Item = (Box<[u8]>, Box<[u8]>);

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(Ok((key, value))) => Some((key, value)),
            _ => None,
        }
    }
}
