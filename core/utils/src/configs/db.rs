use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use rocksdb::{Options, BlockBasedOptions, Cache};

// Database config
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct StoreConfig
{
    // Represents the directory where the database files will be stored.
    // The user does not have to specify a path. Null by default
    pub path: Option<PathBuf>,
    // Specifies the maximum number of files that can be opened at the same time.
    // It will be a limited value, usually due to a limitation due to the file system.
    // 1000 by default.
    pub max_open_files: u32,
    // Specifies the size of the database cache in bytes.
    // 160 MB by default.
    pub cache_size: usize,
    // Specifies the size of database blocks in bytes.
    // 16 KB by default.
    pub block_size: usize,
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
    // Converts the `StoreConfig` into RocksDB `Options`.
    pub fn to_options(&self) -> Options
    {
        let mut options = Options::default();
        let mut block_options = BlockBasedOptions::default();
        // self.cache_size is in MB, so we convert it to bytes.
        let cache = Cache::new_lru_cache((self.cache_size as usize / (1024 * 1024)) * 1024 * 102);
        options.set_max_open_files(self.max_open_files as i32);
        block_options.set_block_cache(&cache);
        block_options.set_block_size(self.block_size as usize);
        options.set_block_based_table_factory(&block_options);
        options
    }
}
