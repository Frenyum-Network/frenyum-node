pub struct RocksDBTransaction { pub(crate) inner: TransactionDB }

impl RocksDBTransaction
{
    pub fn get_pinned<K: AsRef<[u8]>(
        &self,
        col: &str,
        key: K,
    ) -> Result<Option<DBPinnableSlice>, Error> {
        let cf_handle = self.inner.get_cf_handle(column)?;
        self.inner.get_pinned_cf(cf_handle, key, &ReadOptions::default())
    }

    
}
