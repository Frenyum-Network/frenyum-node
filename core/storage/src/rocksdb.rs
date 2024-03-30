use rocksdb::{DB, Options, WriteBatch, Direction, IteratorMode, ColumnFamily};
use std::collections::HashMap;
use crate::column::Column;

pub struct RocksDB
{
    db: DB,
    db_opt: Options,
    db_cf: HashMap<Column, ColumnFamily>,
}

// Database operations
impl RocksDB 
{
    pub fn new(
        db: DB, 
        db_opt: Options, 
        db_cf: HashMap<Column, ColumnFamily>
    )-> Self {
        RocksDB { db, db_opt, db_cf }
    }

    pub fn open(path: &str, db_cf: HashMap<Column, ColumnFamily>) -> Result<Self, rocksdb::Error>
    {
        let db_opt = Options::default();
        let db = DB::open_default(path)?;
        Ok(RocksDB::new(db, db_opt, db_cf))
    }

    pub fn open_with_options(
        path: &str,
        options: Options,
        db_cf: HashMap<Column, ColumnFamily>
    ) -> Result<Self, rocksdb::Error> {
        let db = DB::open(&options, path)?;
        Ok(RocksDB::new(db, options, db_cf))
    }

    pub fn flush(&self) -> Result<(), rocksdb::Error>
    {
        Ok(self.db.flush()?)
    }

    pub fn compact(&self) -> Result<(), rocksdb::Error>
    {
        Ok(self.db.compact_range(None::<&[u8]>, None::<&[u8]>))
    }
}

// Key operations
impl RocksDB
{
    pub fn get(&self, column: Column) -> &ColumnFamily
    {
        self.db_cf.get(&column).expect("ColumnFamily not found")
    }
}
