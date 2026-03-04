//! The AES block cipher function.

use crate::platform::*;

pub(crate) type ExtendedKey<T, const NUM_KEYS: usize> = [T; NUM_KEYS];

/// AES block size
pub(crate) const AES_BLOCK_LEN: usize = 16;

/// AES 128 block cipher key length in bytes.
pub const AES_128_KEY_LEN: usize = 16;

/// AES 256 block cipher key length in bytes.
pub const AES_256_KEY_LEN: usize = 32;

/// The AES block cipher function.
#[inline]
pub(crate) fn block_cipher<T: AESState, const NUM_KEYS: usize>(
    st: &mut T,
    keyex: &ExtendedKey<T, NUM_KEYS>,
) {
    st.xor_key(&keyex[0]);

    #[allow(clippy::needless_range_loop)]
    for i in 1..NUM_KEYS - 1 {
        st.aes_enc(&keyex[i]);
    }

    st.aes_enc_last(&keyex[NUM_KEYS - 1]);
}
