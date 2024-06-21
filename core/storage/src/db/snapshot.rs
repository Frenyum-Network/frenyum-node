use rocksdb::{ffi, DB};

pub struct Snapshot<'a>
{
    db: 'a DB,
    snapshot: *const ffi:rocksdb_snapshot_t
}
