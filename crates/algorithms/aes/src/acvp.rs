//! Internal API exposed for ACVP testing only.
//!
//! AES-GCM is validated together with the AES block cipher (`ACVP-AES-ECB`),
//! which is otherwise not exposed by this crate.

use crate::{
    aes::{block_cipher, AES_128_KEY_LEN, AES_256_KEY_LEN, AES_BLOCK_LEN},
    ctr::{aes128_ctr, aes256_ctr},
    platform::{self, AESState},
};

fn aes128_encrypt_block<T: AESState>(key: &[u8; AES_128_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    let keyex = aes128_ctr::key_expansion::<T>(key);
    let mut st = T::new();
    st.load_block(block);
    block_cipher(&mut st, &keyex);
    st.store_block(block);
}

fn aes256_encrypt_block<T: AESState>(key: &[u8; AES_256_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    let keyex = aes256_ctr::key_expansion::<T>(key);
    let mut st = T::new();
    st.load_block(block);
    block_cipher(&mut st, &keyex);
    st.store_block(block);
}

/// Encrypt a single block with AES-256, using the portable implementation.
pub fn portable_aes256_encrypt_block(key: &[u8; AES_256_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    aes256_encrypt_block::<platform::portable::State>(key, block)
}

/// Encrypt a single block with AES-256, using the ARM Neon implementation.
///
/// Must only be used after checking that the CPU supports the AES extensions.
#[cfg(feature = "simd128")]
pub fn neon_aes256_encrypt_block(key: &[u8; AES_256_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    aes256_encrypt_block::<platform::neon::State>(key, block)
}

/// Encrypt a single block with AES-256, using the x86_64 AES-NI implementation.
///
/// Must only be used after checking that the CPU supports AES-NI.
#[cfg(feature = "simd256")]
pub fn x64_aes256_encrypt_block(key: &[u8; AES_256_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    aes256_encrypt_block::<platform::x64::State>(key, block)
}

/// Encrypt a single block with AES-128, using the portable implementation.
pub fn portable_aes128_encrypt_block(key: &[u8; AES_128_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    aes128_encrypt_block::<platform::portable::State>(key, block)
}

/// Encrypt a single block with AES-128, using the ARM Neon implementation.
///
/// Must only be used after checking that the CPU supports the AES extensions.
#[cfg(feature = "simd128")]
pub fn neon_aes128_encrypt_block(key: &[u8; AES_128_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    aes128_encrypt_block::<platform::neon::State>(key, block)
}

/// Encrypt a single block with AES-128, using the x86_64 AES-NI implementation.
///
/// Must only be used after checking that the CPU supports AES-NI.
#[cfg(feature = "simd256")]
pub fn x64_aes128_encrypt_block(key: &[u8; AES_128_KEY_LEN], block: &mut [u8; AES_BLOCK_LEN]) {
    aes128_encrypt_block::<platform::x64::State>(key, block)
}
