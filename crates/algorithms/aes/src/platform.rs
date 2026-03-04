//! Traits for platform dependent implementations

pub(crate) mod portable;

#[cfg(feature = "simd128")]
pub(crate) mod neon;

#[cfg(feature = "simd256")]
pub(crate) mod x64;

/// The AES state.
pub(crate) trait AESState: Clone + core::fmt::Debug {
    fn new() -> Self;
    fn load_block(&mut self, b: &[u8]);
    fn store_block(&self, out: &mut [u8]);
    fn xor_block(&self, inp: &[u8], out: &mut [u8]);

    fn xor_key(&mut self, key: &Self);
    fn aes_enc(&mut self, key: &Self);
    fn aes_enc_last(&mut self, key: &Self);
    fn aes_keygen_assist0<const RCON: i32>(&mut self, prev: &Self);
    fn aes_keygen_assist1(&mut self, prev: &Self);
    fn key_expansion_step(&mut self, prev: &Self);
}

/// A gf128 field element.
pub(crate) trait GF128FieldElement {
    fn zero() -> Self;
    fn load_element(bytes: &[u8]) -> Self;
    fn store_element(&self, bytes: &mut [u8]);
    fn add(&mut self, other: &Self);
    fn mul(&mut self, other: &Self);
}
