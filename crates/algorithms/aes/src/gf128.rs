//! Generic Gf128 implementation.
//!
//! Generic over platform dependent [`GF128FieldElement`].

use crate::{aes::AES_BLOCK_LEN, platform::*};

#[cfg(test)]
mod test;

/// Generic Gf128 state.
pub(crate) struct GF128State<T: GF128FieldElement> {
    accumulator: T,
    r: T,
}

const KEY_LEN: usize = AES_BLOCK_LEN;

impl<T: GF128FieldElement> GF128State<T> {
    #[inline]
    pub(crate) fn init(key: &[u8]) -> Self {
        debug_assert!(key.len() == KEY_LEN);

        Self {
            accumulator: T::zero(),
            r: T::load_element(key),
        }
    }

    #[inline]
    pub(crate) fn update(&mut self, block: &[u8]) {
        debug_assert!(block.len() == KEY_LEN);

        let block_elem = T::load_element(block);
        self.accumulator.add(&block_elem);
        self.accumulator.mul(&self.r);
    }

    #[inline]
    pub(crate) fn update_blocks(&mut self, input: &[u8]) {
        debug_assert!(input.len().is_multiple_of(AES_BLOCK_LEN));

        let blocks = input.len() / AES_BLOCK_LEN;
        for i in 0..blocks {
            let offset = i * AES_BLOCK_LEN;
            self.update(&input[offset..offset + AES_BLOCK_LEN]);
        }
    }

    #[inline]
    pub(crate) fn update_last(&mut self, partial_block: &[u8]) {
        debug_assert!(partial_block.len() < 16);

        let mut block = [0u8; 16];
        block[0..partial_block.len()].copy_from_slice(partial_block);
        self.update(&block);
    }

    #[inline]
    pub(crate) fn update_padded(&mut self, input: &[u8]) {
        let blocks = input.len() / AES_BLOCK_LEN;
        self.update_blocks(&input[0..blocks * AES_BLOCK_LEN]);

        let last = input.len() - input.len() % AES_BLOCK_LEN;
        if last < input.len() {
            self.update_last(&input[last..]);
        }
    }

    #[inline]
    pub(crate) fn emit(&self, out: &mut [u8]) {
        debug_assert!(out.len() == 16);

        self.accumulator.store_element(out);
    }
}
