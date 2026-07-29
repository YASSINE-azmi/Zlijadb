pub fn crc32(data: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn verify(data: &[u8], expected: u32) -> bool {
    crc32(data) == expected
}
