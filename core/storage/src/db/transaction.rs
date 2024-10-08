use rocksdb::{TransactionDB, DBPinnableSlice, Error, ReadOptions};

pub struct RocksDBTransaction { pub(crate) inner: TransactionDB }

impl RocksDBTransaction
{
    pub fn get_pinned<K: AsRef<[u8]>>(
        &self,
        col: &str,
        key: K,
    ) -> Result<Option<DBPinnableSlice>, Error> {
        let cf_handle = self.inner.cf_handle(col).ok_or(format!("Column family not found"));
        self.inner.get_pinned_cf(cf_handle, key, &ReadOptions::default())
    }
    
    pub fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(
        &self,
        col: &str,
        key: K,
        value: V,
    ) -> Result<(), Error> {
        let cf_handle = self.inner.cf_handle(col).ok_or(format!("Column family not found"));
        self.inner.put_cf(cf_handle, key, value)
    }

    pub fn delete<K: AsRef<[u8]>>(
        &self,
        col: &str,
        key: K,
    ) -> Result<(), Error> {
        let cf_handle = self.inner.cf_handle(col).ok_or(format!("Column family not found"));
        self.inner.delete_cf(cf_handle, key)
    }

    pub fn commit(&self) -> Result<(), Error> 
    {
        self.inner.commit()?;
        Ok(())
    }

    pub fn rollback(&self) -> Result<(), Error>
    {
        self.inner.rollback()?;
        Ok(())
    }
}
