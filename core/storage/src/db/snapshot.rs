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
}

impl<'a> Drop for Snapshot<'a>
{
    fn drop(&mut self)
    {
        ffi::rocksdb_release_snapshot(self.db, self.snapshot);
    }
}
