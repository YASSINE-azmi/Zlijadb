use thiserror::Error;

#[derive(Error, Debug)]
pub enum EdgeDbError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("data corruption: {0}")]
    Corruption(String),

    #[error("checksum mismatch: CRC32 validation failed")]
    ChecksumMismatch,

    #[error("invalid format: {0}")]
    InvalidFormat(String),

    #[error("key not found")]
    KeyNotFound,

    #[error("compression failed")]
    CompressionFailed,
}

pub type Result<T> = std::result::Result<T, EdgeDbError>;
