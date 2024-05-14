use std::path::PathBuf;

pub struct StoreConfig
{
    path: Option<PathBuf>,
    max_open_files: u32,
    cache_size: usize,
    block_size: usize,
}

impl Default for StoreConfig
{
    fn default() -> Self
    {
        Self {
            path: None,
            max_open_files: 1000,
            cache_size: 1024 * 1024 * 1024,
            block_size: 4 * 1024,
        }
    }
}


