use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use rocksdb::Options;

// Database config
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct StoreConfig
{
    // Represents the directory where the database files will be stored.
    // The user does not have to specify a path. Null by default
    path: Option<PathBuf>,
    // Specifies the maximum number of files that can be opened at the same time.
    // It will be a limited value, usually due to a limitation due to the file system.
    // 1000 by default.
    max_open_files: u32,
    // Specifies the size of the database cache in bytes.
    // 160 MB by default.
    cache_size: usize,
    // Specifies the size of database blocks in bytes.
    // 16 KB by default.
    block_size: usize,
}

impl Default for StoreConfig
{
    fn default() -> Self
    {
        Self {
            // Path is set to none because it is not mandatory to specify the path value.
            path: None,
            max_open_files: 1000,
            // 160 MB in bytes 
            cache_size: 160 * 1024 * 1024,
            // 16 KB in bytes
            block_size: 16 * 1024,
        }
    }
}

impl StoreConfig
{
    pub fn to_options(&self) -> Options
    {
        let mut options = Options::default();
        options.set_max_open_files(self.max_open_files as i32);
        let cache_mb = self.cache_size / (1024 * 1024);
        options.set_block_cache(cache_mb as usize);
        options.set_block_size(self.block_size as usize);
        options
    }
}
