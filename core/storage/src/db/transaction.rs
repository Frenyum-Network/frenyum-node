// RocksDB transaction wrapper
use rocksdb::{TransactionDB, DBPinnableSlice, Error, ReadOptions};

pub struct RocksDBTransaction { pub(crate) inner: TransactionDB }

// 'RocksDBTransaction' structure, transaction-based with TransactionDB
// provides data management. Can perform atomic operations in the database.
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
    
    // The 'put' method writes data based on a specific column family and key.
    pub fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(
        &self,
        col: &str,
        key: K,
        value: V,
    ) -> Result<(), Error> {
        let cf_handle = self.inner.cf_handle(col).ok_or(format!("Column family not found"));
        self.inner.put_cf(cf_handle, key, value)
    }

    // The 'delete' method deletes data based on a specific column family and key.
    pub fn delete<K: AsRef<[u8]>>(
        &self,
        col: &str,
        key: K,
    ) -> Result<(), Error> {
        let cf_handle = self.inner.cf_handle(col).ok_or(format!("Column family not found"));
        self.inner.delete_cf(cf_handle, key)
    }

    // The 'commit' method will commit all changes made in the transaction.
    // makes it permanent. If not committed, the changes are rolled back.
    pub fn commit(&self) -> Result<(), Error> 
    {
        self.inner.commit()?;
        Ok(())
    }

    // The 'rollback' method rolls back transactions that have been committed.
    // Uncommitted transactions are canceled.
    pub fn rollback(&self) -> Result<(), Error>
    {
        self.inner.rollback()?;
        Ok(())
    }
}
