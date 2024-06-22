use rocksdb::{ffi, DB};

pub struct Snapshot<'a>
{
    db: 'a DB,
    pub(crate) snapshot: *const ffi:rocksdb_snapshot_t
}

impl<'a> Snapshot<'a>
{
    pub fn new(db: &DB) -> Snapshot
    {
        let snapshot = unsafe { ffi::rocksdb_crate_snapshot(db.inner) };
        Snapshot { db, snapshot }
    }

    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<<Vec<u8>>, Error>
    {
        let mut readopts = ReadOptions::default();
        unsafe { ffi::rocksdb_readopitons_set_snapshot(readopts.inner(), self.snapshot) };
        self.db.get_opt(key, &readopts)
    }

    pub fn get_cf<K: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        key: K,
    ) -> Result<Option<Vec<u8>>, Error> {
        let mut readopts = ReadOptions::default();
        unsafe { ffi::rocksdb_readoptions_set_snapshot(readopts.inner(), self.snapshot) };
        self.db.get_opt(key, &readopts)
    }

impl<'a> Drop for Snapshot<'a>
{
    fn drop(&mut self)
    {
        ffi::rocksdb_release_snapshot(self.db, self.snapshot);
    }
}
