use utils::db::StoreConfig;
use rocksdb::{
    DB, Options, WriteBatch, Direction, IteratorMode, 
    ColumnFamily, ErrorKind, ColumnFamilyDescriptor,
    SingleThreaded, Snapshot, DBWithThreadMode,
};
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
        config: &StoreConfig,
        db_cf: HashMap<Column, ColumnFamily>,
        columns: &[Column],
    ) -> Result<Self, rocksdb::Error> {
        let (db, db_opt) = Self::open_db(path, config, columns)?;
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

    pub fn create_cf(&mut self, column: &Column) -> Result<(), rocksdb::Error>
    {
        let cf_desc = ColumnFamilyDescriptor::new(column.to_string(), Options::default());
        self.db.create_cf(&cf_desc)?;
        Ok(())
    }

    pub fn write(&self, batch: WriteBatch) -> Result<(), rocksdb::Error>
    {
        self.db.write(batch)
    }    
}

