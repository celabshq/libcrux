// From https://github.com/hoxxep/MACs/blob/kmac-submission/kmac/src/encoding.rs
fn zero_bytes(num: usize) -> u8 {
    let zero_bits = (num | 1usize).leading_zeros();
    let zero_bits = (zero_bits % 64) as u8;
    zero_bits / 8
}

#[inline(always)]
/// Left-encode a single byte.
pub fn left_encode_byte(num: u8) -> [u8; 2] {
    [1u8, num]
}

#[hax_lib::ensures(|result| result.len() <= 9)]
#[inline(always)]
pub(crate) fn encode(num: usize, buffer: &mut [u8; 9], left_encode: bool) -> &[u8] {
    let zero_size = zero_bytes(num);
    let zero_length = zero_size as usize;
    let be_bytes = num.to_be_bytes();
    let encoding_length = be_bytes.len() - zero_length;
    let encoding_size = encoding_length as u8;
    debug_assert!(0 < encoding_length);
    debug_assert!(encoding_length <= 8);
    let output_length = encoding_length + 1;
    if left_encode {
        buffer[0] = encoding_size;
        buffer[1..output_length].copy_from_slice(&be_bytes[zero_length..]);
    } else {
        buffer[0..encoding_length].copy_from_slice(&be_bytes[zero_length..]);
        buffer[encoding_length] = encoding_size;
    }

    &buffer[..output_length]
}

#[hax_lib::ensures(|result| result.len() <= 9)]
#[inline(always)]
/// Left-encode any `num < u64::MAX`.
pub fn left_encode(num: usize, buffer: &mut [u8; 9]) -> &[u8] {
    encode(num, buffer, true)
}

#[hax_lib::ensures(|result| result.len() <= 9)]
#[inline(always)]
/// Right-encode any `num < u64::MAX`.
pub fn right_encode(num: usize, buffer: &mut [u8; 9]) -> &[u8] {
    encode(num, buffer, false)
}
