// RocksDB snapshot wrapper
use rocksdb::{DB, ColumnFamily, ReadOptions, Error, IteratorMode, DBIterator, DBRawIterator, Snapshot as RocksDBSnapshot};

// Represents a snapshot taken from RocksDB.
pub struct Snapshot<'a>
{
    db: &'a DB,
    pub(crate) snapshot: RocksDBSnapshot<'a>,
}

impl<'a> Snapshot<'a>
{
    // Takes a new snapshot and freezes the current state.
    pub fn new(db: &'a DB) -> Snapshot<'a>
    {
        let snapshot = db.snapshot();
        Snapshot { db, snapshot }
    }

    // Reads the value for a specific key using Snapshot.
    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error>
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        self.db.get_opt(key, &readopts)
    }

    // Reads data according to a specific column family and key.
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
    
    // Creates an iterator in the database using the snapshot.
    // This iterator traverses the database as it was at the time of the snapshot.
    pub fn iterator(&self, mode: IteratorMode) -> DBIterator<'a>
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        self.db.iterator_opt(mode, readopts)
    }
    
    // This iterator can be used to iterate over more raw data.
    pub fn raw_iterator(&self) -> DBRawIterator
    {
        let mut readopts = ReadOptions::default();
        readopts.set_snapshot(&self.snapshot);
        self.db.raw_iterator_opt(readopts)
    }
}

// To specify that the Snapshot structure is thread-safe, we set the Sync and Send traits to
// must be marked manually. Because snapshots are only read and not modified.
unsafe impl<'a> Sync for Snapshot<'a> {}
unsafe impl<'a> Send for Snapshot<'a> {}
