//! AES ctr mode implementation.
//!
//! This implementation is generic over the [`AESState`], which has different,
//! platform dependent implementations.
//!
//! This get's instantiated in [`aes128_ctr`] and [`aes256_ctr`].

use crate::{aes::*, platform::AESState};

#[cfg(test)]
mod test128;

mod aes128_ctr;
mod aes256_ctr;

/// The ctr nonce length. This is different from the AES nonce length
/// [`crate::NONCE_LEN`].
const CTR_NONCE_LEN: usize = 16;

pub(crate) const AES_GCM_CTR_LEN: usize = 4;
pub(crate) const AES_CCM_CTR_LEN: usize = 3;
pub(crate) const AES_GCM_NONCE_START: usize = 0;
pub(crate) const AES_CCM_NONCE_START: usize = 1;

/// Generic AES CTR context.
///
/// - `NUM_KEYS` is the number of sub-keys that are expanded in `extended_key`, i.e. 11 for AES-128, 15 for AES-256.
/// - `CTR_LEN` is how many bytes at the end of `ctr_nonce` are used for the counter
/// - `NONCE_START` is the index in `ctr_nonce`, where the AEAD nonce begins, i.e. 0 in AES-GCM and 1 in AES-CCM (because the first byte is for flags CCM)
pub(crate) struct AesCtrContext<
    T: AESState,
    const NUM_KEYS: usize,
    const CTR_LEN: usize,
    const NONCE_START: usize,
> {
    pub(crate) extended_key: ExtendedKey<T, NUM_KEYS>,
    pub(crate) ctr_nonce: [u8; CTR_NONCE_LEN],
}

impl<T: AESState, const NUM_KEYS: usize, const CTR_LEN: usize, const NONCE_START: usize>
    AesCtrContext<T, NUM_KEYS, CTR_LEN, NONCE_START>
{
    #[inline]
    pub(crate) fn aes_ctr_set_nonce(&mut self, nonce: &[u8]) {
        debug_assert!(nonce.len() == crate::NONCE_LEN);

        self.ctr_nonce[NONCE_START..crate::NONCE_LEN + NONCE_START].copy_from_slice(nonce);
    }

    #[inline]
    pub(crate) fn aes_ctr_key_block(&self, ctr: u32, out: &mut [u8]) {
        debug_assert!(out.len() == AES_BLOCK_LEN);

        let mut st_init = self.ctr_nonce;
        st_init[CTR_NONCE_LEN - CTR_LEN..].copy_from_slice(&ctr.to_be_bytes()[4 - CTR_LEN..]);
        let mut st = T::new();

        st.load_block(&st_init);

        block_cipher(&mut st, &self.extended_key);

        st.store_block(out);
    }

    #[inline]
    fn aes_ctr_xor_block(&self, ctr: u32, input: &[u8], out: &mut [u8]) {
        debug_assert!(input.len() == out.len() && input.len() <= AES_BLOCK_LEN);

        let mut st_init = self.ctr_nonce;
        st_init[CTR_NONCE_LEN - CTR_LEN..].copy_from_slice(&ctr.to_be_bytes()[4 - CTR_LEN..]);
        let mut st = T::new();
        st.load_block(&st_init);

        block_cipher(&mut st, &self.extended_key);

        st.xor_block(input, out);
    }

    #[inline]
    fn aes_ctr_xor_blocks(&self, ctr: u32, input: &[u8], out: &mut [u8]) {
        debug_assert!(input.len() == out.len() && input.len().is_multiple_of(AES_BLOCK_LEN));
        // If input.len() / AES_BLOCK_LEN == u32::MAX - 1 and we start with
        // ctr == 2 then we'll wrap to 0 below and we'll repeat the initial key
        // block
        // Note that every entry point checks for the input length. Hence we
        // only have a debug assert here.
        debug_assert!(input.len() / AES_BLOCK_LEN < (u32::MAX - 1) as usize);

        let blocks = input.len() / AES_BLOCK_LEN;
        for i in 0..blocks {
            let offset = i * AES_BLOCK_LEN;
            self.aes_ctr_xor_block(
                ctr.wrapping_add(i as u32),
                &input[offset..offset + AES_BLOCK_LEN],
                &mut out[offset..offset + AES_BLOCK_LEN],
            );
        }
    }

    #[inline]
    pub(crate) fn aes_ctr_update(&self, ctr: u32, input: &[u8], out: &mut [u8]) {
        debug_assert!(input.len() == out.len());
        debug_assert!(input.len() / AES_BLOCK_LEN < u32::MAX as usize);

        let blocks = input.len() / AES_BLOCK_LEN;
        self.aes_ctr_xor_blocks(
            ctr,
            &input[0..blocks * AES_BLOCK_LEN],
            &mut out[0..blocks * AES_BLOCK_LEN],
        );

        let last = input.len() - input.len() % AES_BLOCK_LEN;
        if last < input.len() {
            self.aes_ctr_xor_block(
                ctr.wrapping_add(blocks as u32),
                &input[last..],
                &mut out[last..],
            );
        }
    }
}

/// Trait for constructing an [`AesCtrContext`] from a CCM key.
///
/// Implemented for AES-128 (`NUM_KEYS = 11`) and AES-256 (`NUM_KEYS = 15`).
pub(crate) trait CcmInit: Sized {
    fn ccm_init(key: &[u8]) -> Self;
}

/// Trait for constructing an [`AesCtrContext`] from a GCM key.
///
/// Implemented for AES-128 (`NUM_KEYS = 11`) and AES-256 (`NUM_KEYS = 15`).
pub(crate) trait GcmInit: Sized {
    fn gcm_init(key: &[u8]) -> Self;
}
