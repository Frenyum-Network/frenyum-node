pub mod rocksdb;
pub mod iter;
pub mod snapshot;
pub mod transaction;
pub mod write_batch;

use utils::configs::db::StoreConfig;
use rocksdb::db::{DB, Options, WriteBatch, ColumnFamily, TransactionDB, Error};
use std::path::PathBuf;

pub struct DBManager
{
    db: RocksDB,
}

impl DBManager 
{
    pub fn new(config: StoreConfig) -> Result<Self, rocksdb::error>
    {
        let options = config.to_options();
        let path = config.path.unwrap_or_else(|| PathBuf::from("default_db_path"));
        
        let db = DB::open(&options, path);
        Ok(Self {
            db,
        })
    }
}
