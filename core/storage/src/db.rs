pub mod rocksdb;
pub mod iter;
pub mod snapshot;
pub mod transaction;
pub mod write_batch;

use rocksdb::{DB, Options, WriteBatch, Snapshot, ColumnFamily, TransactionDB, Transaction, TransactionOptions, WriteOptions};
use std::path::Path;

pub struct DBManager
{
    db: TransactionDB,
    write_batch: WriteBatch,
}

