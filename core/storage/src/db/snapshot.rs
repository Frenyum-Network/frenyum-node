use rocksdb::{DB, Options, Snapshot as RocksSnapshot};

pub struct Snapshot<'a> 
{
    db: &'a DB,
    snapshot: RocksSnapshot<'a>,
}

