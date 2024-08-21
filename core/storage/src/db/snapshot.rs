use rocksdb::{ffi, DB, ColumnFamily, ReadOptions, Error};

pub struct Snapshot<'a>
{
    db: &'a DB,
    pub(crate) snapshot: *const ffi::rocksdb_snapshot_t,
}

impl<'a> Snapshot<'a>
{
    pub fn new(db: &'a DB) -> Snapshot<'a>
    {
        let snapshot = unsafe { ffi::rocksdb_create_snapshot(db.inner()) };
        Snapshot { db, snapshot }
    }

    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error>
    {
        let mut readopts = ReadOptions::default();
        unsafe { ffi::rocksdb_readoptions_set_snapshot(readopts.inner(), self.snapshot) };
        self.db.get_opt(key, &readopts)
    }

    pub fn get_cf<K: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        key: K,
    ) -> Result<Option<Vec<u8>>, Error>
    {
        let mut readopts = ReadOptions::default();
        unsafe { ffi::rocksdb_readoptions_set_snapshot(readopts.inner(), self.snapshot) };
        self.db.get_cf_opt(cf, key, &readopts)
    }

    pub fn iterator(&self, mode: IteratorMode) -> DBIterator<'a>
    {
        let mut readopts = ReadOptions::default();
        unsafe { ffi::rocksdb_readoptions_set_snapshot(readopts.inner(), self.snapshot) };
        self.db.iterator_opt(mode, readopts)
    }

    pub fn raw_iterator(&self) -> DBRawIterator
    {
        let mut readopts = ReadOptions::default();
        unsafe { ffi::rocksdb_readoptions_set_snapshot(readopts.inner(), self.snapshot) };
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

unsafe impl Sync for Snapshot<'a> {}
unsafe impl Send for Snapshot<'a> {}
