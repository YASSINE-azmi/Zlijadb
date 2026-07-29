use std::path::PathBuf;

use crate::compression::CompressionType;
use crate::error::Result;

const MB: u64 = 1024 * 1024;
const KB: u32 = 1024;

#[derive(Debug, Clone)]
pub struct Config {
    pub wal_max_segment_size: u64,
    pub memtable_max_size: u64,
    pub sst_block_size: u32,
    pub compression_type: CompressionType,
    pub data_dir: PathBuf,
    pub sync_on_write: bool,
    pub cache_size: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wal_max_segment_size: 4 * MB,
            memtable_max_size: 64 * MB,
            sst_block_size: 4 * KB,
            compression_type: CompressionType::Lz4,
            data_dir: PathBuf::from("zlija_data"),
            sync_on_write: true,
            cache_size: 32 * MB,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.wal_max_segment_size == 0 {
            return Err(crate::error::EdgeDbError::InvalidFormat(
                "wal_max_segment_size must be > 0".into(),
            ));
        }
        if self.sst_block_size < 256 {
            return Err(crate::error::EdgeDbError::InvalidFormat(
                "sst_block_size must be >= 256".into(),
            ));
        }
        if self.memtable_max_size == 0 {
            return Err(crate::error::EdgeDbError::InvalidFormat(
                "memtable_max_size must be > 0".into(),
            ));
        }
        if self.data_dir.as_os_str().is_empty() {
            return Err(crate::error::EdgeDbError::InvalidFormat(
                "data_dir must not be empty".into(),
            ));
        }
        Ok(())
    }

    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        if let Ok(val) = std::env::var("ZLIJA_WAL_MAX_SEGMENT_SIZE") {
            config.wal_max_segment_size = val.parse().map_err(|_| {
                crate::error::EdgeDbError::InvalidFormat(format!(
                    "invalid ZLIJA_WAL_MAX_SEGMENT_SIZE: {val}"
                ))
            })?;
        }
        if let Ok(val) = std::env::var("ZLIJA_MEMTABLE_MAX_SIZE") {
            config.memtable_max_size = val.parse().map_err(|_| {
                crate::error::EdgeDbError::InvalidFormat(format!(
                    "invalid ZLIJA_MEMTABLE_MAX_SIZE: {val}"
                ))
            })?;
        }
        if let Ok(val) = std::env::var("ZLIJA_SST_BLOCK_SIZE") {
            config.sst_block_size = val.parse().map_err(|_| {
                crate::error::EdgeDbError::InvalidFormat(format!(
                    "invalid ZLIJA_SST_BLOCK_SIZE: {val}"
                ))
            })?;
        }
        if let Ok(val) = std::env::var("ZLIJA_COMPRESSION_TYPE") {
            config.compression_type = match val.to_lowercase().as_str() {
                "none" => CompressionType::None,
                "lz4" => CompressionType::Lz4,
                "zstd" => CompressionType::Zstd,
                other => {
                    return Err(crate::error::EdgeDbError::InvalidFormat(format!(
                        "unknown compression type: {other}"
                    )));
                }
            };
        }
        if let Ok(val) = std::env::var("ZLIJA_DATA_DIR") {
            config.data_dir = PathBuf::from(val);
        }
        if let Ok(val) = std::env::var("ZLIJA_SYNC_ON_WRITE") {
            config.sync_on_write = val.parse().map_err(|_| {
                crate::error::EdgeDbError::InvalidFormat(format!(
                    "invalid ZLIJA_SYNC_ON_WRITE: {val}"
                ))
            })?;
        }
        if let Ok(val) = std::env::var("ZLIJA_CACHE_SIZE") {
            config.cache_size = val.parse().map_err(|_| {
                crate::error::EdgeDbError::InvalidFormat(format!(
                    "invalid ZLIJA_CACHE_SIZE: {val}"
                ))
            })?;
        }

        config.validate()?;
        Ok(config)
    }
}
