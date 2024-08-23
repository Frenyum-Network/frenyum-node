use rocksdb::{TransactionDB, WriteBatch};
use std::sync::Arc;

pub struct RocksDBWriteBatch
{
    pub(crate) db: Arc<TransactionDB>,
    pub(crate) inner: WriteBatch,
}

impl RocksDBWriteBatch
{
    pub fn len(&self) -> usize
    {
        self.inner.len()
    }

    pub fn size_in_bytes(&self) -> usize
    {
        self.inner.size_in_bytes()
    }

    pub fn is_empty(&self) -> bool
    {
        self.inner.is_empty()
    }

    pub fn put<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<(), rocksdb::Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.inner.put(key, value)?;
        Ok(())
    }

    pub fn delete<K>(
        &mut self,
        key: K,
    ) -> Result<(), rocksdb::Error>
    where
        K: AsRef<[u8]>,
    {
        self.inner.put(key)?;
        Ok(())
    }
}

