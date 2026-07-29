use crate::error::{EdgeDbError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None = 0,
    Lz4 = 1,
    Zstd = 2,
}

impl CompressionType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Lz4),
            2 => Some(Self::Zstd),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

pub fn compress(data: &[u8], algo: CompressionType) -> Result<Vec<u8>> {
    match algo {
        CompressionType::None => Ok(data.to_vec()),
        CompressionType::Lz4 => {
            lz4::block::compress(data, None, false).map_err(|_| EdgeDbError::CompressionFailed)
        }
        CompressionType::Zstd => {
            zstd::stream::encode_all(std::io::Cursor::new(data), 0)
                .map_err(|_| EdgeDbError::CompressionFailed)
        }
    }
}

pub fn decompress(data: &[u8], algo: CompressionType) -> Result<Vec<u8>> {
    match algo {
        CompressionType::None => Ok(data.to_vec()),
        CompressionType::Lz4 => {
            lz4::block::decompress(data, None)
                .map_err(|_| EdgeDbError::CompressionFailed)
        }
        CompressionType::Zstd => {
            zstd::stream::decode_all(std::io::Cursor::new(data))
                .map_err(|_| EdgeDbError::CompressionFailed)
        }
    }
}
