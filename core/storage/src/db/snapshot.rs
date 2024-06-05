use rocksdb::{DB, Options, Snapshot as RocksSnapshot};

pub struct Snapshot<'a> 
{
    db: &'a DB,
    snapshot: RocksSnapshot<'a>,
}

impl<'a> Snapshot<'a>
{
    pub fn new(db: &'a DB) -> Self
    {
        let snapshot = db.snapshot();
        Snapshot { db, snapshot }
    }
}
