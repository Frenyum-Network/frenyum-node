use rocksdb::{DB, Options, WriteBatch, Direction, IteratorMode, ColumnFamily, ErrorKind, ColumnFamilyDescriptor};
use std::path::Path;
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
    pub fn open(
        path: &Path, 
        db_cf: HashMap<Column, ColumnFamily>
    ) -> Result<Self, rocksdb::Error> {
        let db_opt = Options::default();
        let db = DB::open_default(path)?;
        Ok(Self { db, db_opt, db_cf })
    }

    fn open_with_options(
        path: &Path,
        db_opt: Options,
        db_cf: HashMap<Column, ColumnFamily>
    ) -> Result<Self, rocksdb::Error> {
        let db = DB::open(&db_opt, path)?;
        Ok(Self { db, db_opt, db_cf })
    }

    fn open_with_columns(
        path: &Path,
        db_opt: Options,
        db_cf: HashMap<Column, ColumnFamily>,
        columns: &[Column],
    ) -> Result<Self, rocksdb::Error> {
        let (db, db_opt) = Self::open_db(path, db_opt, columns)?;
        Ok(Self { db, db_opt, db_cf })
    }

    fn open_db(
        path: &Path, 
        db_opt: Options,
        columns: &[Column],
     ) -> Result<(DB, Options), rocksdb::Error> {
        let cf_descriptors: Vec<_> = columns.iter().map(|column| {
            let column = column.to_string(); 
            let options = Options::default();
            ColumnFamilyDescriptor::new(column, options)
        }).collect();   
        let db = DB::open_cf_descriptors(&db_opt, path, cf_descriptors)?;
        Ok((db, db_opt))
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
    pub fn get(
        &self, 
        column: Column, 
        key: &[u8]
    ) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        let cf = self.db_cf.get(&column).expect("Column family not found!");
        self.db.get_cf(cf, key)
    }

    pub fn put(
        &self,
        column: Column,
        key: &[u8],
        value: &[u8]
     ) -> Result<(), rocksdb::Error> {
        let cf = self.db_cf.get(&column).expect("Column family not found!");
        self.db.put_cf(cf, key, value)
    }

    pub fn delete(
        &mut self,
        column: Column,
        key: &[u8]
     ) -> Result<(), rocksdb::Error> {
        let cf = self.db_cf.get(&column).expect("Column family not found!");
        self.db.delete_cf(cf, key)
    } 
}
