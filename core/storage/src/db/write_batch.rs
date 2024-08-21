pub struct RocksDBWriteBatch
{
    pub(crate) db: Arc<TransactionDB>,
    pub(crate) inner: WriteBatch,
}
