use rocksdb::{DB, ColumnFamily, ReadOptions, Error, IteratorMode, DBIterator, DBRawIterator};

pub struct Snapshot<'a>
{
    db: &'a DB,
    pub(crate) snapshot: *const ffi::rocksdb_snapshot_t,
}

impl<'a> Snapshot<'a>
{
    pub fn new(db: &'a DB) -> Snapshot<'a>
    {
        let snapshot = db.snapshot();
        Snapshot { db, snapshot }
    }

    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error>
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        self.db.get_opt(key, &readopts)
    }

    pub fn get_cf<K: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        key: K,
    ) -> Result<Option<Vec<u8>>, Error>
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        self.db.get_cf_opt(cf, key, &readopts)
    }

    pub fn iterator(&self, mode: IteratorMode) -> DBIterator<'a>
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        self.db.iterator_opt(mode, readopts)
    }

    pub fn raw_iterator(&self) -> DBRawIterator
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        DBRawIterator::new(self.db, readopts)
    }
}

impl<'a> Drop for Snapshot<'a>
{
    fn drop(&mut self)
    {
        unsafe {
            ffi::rocksdb_release_snapshot(self.db.inner(), self.snapshot);
        }
    }
}

unsafe impl<'a> Sync for Snapshot<'a> {}
unsafe impl<'a> Send for Snapshot<'a> {}
