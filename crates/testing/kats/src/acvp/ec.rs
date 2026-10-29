/// Left-pad a big-endian integer to `len` bytes, or `None` if it does not fit.
pub fn pad(bytes: &[u8], len: usize) -> Option<Vec<u8>> {
    let mut out = vec![0u8; len];
    out[len.checked_sub(bytes.len())?..].copy_from_slice(bytes);
    Some(out)
}

/// `X || Y`, with each coordinate left-padded to `len` bytes, or `None` if a coordinate does not
/// fit.
pub fn raw_point(x: &[u8], y: &[u8], len: usize) -> Option<Vec<u8>> {
    Some([pad(x, len)?, pad(y, len)?].concat())
}

/// The SEC1 uncompressed encoding `0x04 || X || Y`, with each coordinate left-padded to `len`
/// bytes, or `None` if a coordinate does not fit.
pub fn uncompressed_point(x: &[u8], y: &[u8], len: usize) -> Option<Vec<u8>> {
    Some([&[0x04], raw_point(x, y, len)?.as_slice()].concat())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding() {
        assert_eq!(pad(&[1, 2], 3), Some(vec![0, 1, 2]));
        assert_eq!(pad(&[1, 2], 1), None);
        assert_eq!(raw_point(&[1], &[2], 2), Some(vec![0, 1, 0, 2]));
        assert_eq!(uncompressed_point(&[1], &[2], 2), Some(vec![4, 0, 1, 0, 2]));
        assert_eq!(uncompressed_point(&[1, 2, 3], &[2], 2), None);
    }
}
