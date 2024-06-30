#[inline(always)]
pub fn read_u32_le(data: &[u8]) -> u32 {
    (data[0] as u32) |
    ((data[1] as u32) << 8) |
    ((data[2] as u32) << 16) |
    ((data[3] as u32) << 24)
}

#[inline(always)]
pub fn read_u64_le(data: &[u8]) -> u64 {
    (data[0] as u64) |
    ((data[1] as u64) << 8) |
    ((data[2] as u64) << 16) |
    ((data[3] as u64) << 24) |
    ((data[4] as u64) << 32) |
    ((data[5] as u64) << 40) |
    ((data[6] as u64) << 48) |
    ((data[7] as u64) << 56)
}
