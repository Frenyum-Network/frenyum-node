use std::path::PathBuf;

pub struct StoreConfig
{
    path: PathBuf,
    max_open_files: u32,
    cache_size: usize,
    block_size: usize,
}




