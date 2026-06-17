//! # KMAC
//!
//! This crate implements KMAC as defined in [NIST SP 800-185](https://csrc.nist.gov/pubs/sp/800/185/final).
#![no_std]

mod internals {
    pub(super) mod kmac;
}
use internals::kmac::*;
use libcrux_sha3::portable::incremental::{CShake128, CShake256};

/// Write the KMAC128 MAC of `data` under `key` into `tag`, using
/// `customization` if provided.
///
/// C.f. [NIST SP 800-185](https://csrc.nist.gov/pubs/sp/800/185/final),
/// section 4.
#[hax_lib::requires(key.len() >= 16)]
pub fn kmac_128<'a>(tag: &'a mut [u8], key: &[u8], data: &[u8], customization: &[u8]) -> &'a [u8] {
    // Assert that key is long enough, i.e. at least 128 bits
    assert!(key.len() >= 16);
    compute_kmac::<168, CShake128>(tag, tag.len(), key, key.len(), data, customization)
}

/// Write the KMAC256 MAC of `data` under `key` into `tag`, using
/// `customization` if provided.
///
/// C.f. [NIST SP 800-185](https://csrc.nist.gov/pubs/sp/800/185/final),
/// section 4.
#[hax_lib::requires(key.len() >= 32)]
pub fn kmac_256<'a>(tag: &'a mut [u8], key: &[u8], data: &[u8], customization: &[u8]) -> &'a [u8] {
    // Assert that key is long enough, i.e. at least 256 bits
    assert!(key.len() >= 32);
    compute_kmac::<136, CShake256>(tag, tag.len(), key, key.len(), data, customization)
}
