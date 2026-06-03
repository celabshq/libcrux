use crate::generic_keccak::{self, portable::keccak1};
use hax_lib;

#[cfg(hax)]
use hax_lib::int::*;

use generic_keccak::KeccakState as GenericState;

/// The Keccak state for the incremental API.
#[derive(Clone, Copy)]
pub struct KeccakState {
    state: GenericState<1, u64>,
}

impl KeccakState {
    /// Initialize a fresh portable Keccak state.
    pub fn init() -> Self {
        KeccakState {
            state: GenericState::<1, u64>::new(),
        }
    }
}

/// A portable SHA3 224 implementation.
#[inline(always)]
pub fn sha224(digest: &mut [u8], data: &[u8]) {
    keccak1::<144, 0x06u8>(data, digest);
}

/// A portable SHA3 256 implementation.
#[inline(always)]
pub fn sha256(digest: &mut [u8], data: &[u8]) {
    keccak1::<136, 0x06u8>(data, digest);
}

/// A portable SHA3 384 implementation.
#[inline(always)]
pub fn sha384(digest: &mut [u8], data: &[u8]) {
    keccak1::<104, 0x06u8>(data, digest);
}

/// A portable SHA3 512 implementation.
#[inline(always)]
pub fn sha512(digest: &mut [u8], data: &[u8]) {
    keccak1::<72, 0x06u8>(data, digest);
}

/// A portable SHAKE128 implementation.
#[inline(always)]
pub fn shake128(digest: &mut [u8], data: &[u8]) {
    keccak1::<168, 0x1fu8>(data, digest);
}

/// A portable SHAKE256 implementation.
#[inline(always)]
pub fn shake256(digest: &mut [u8], data: &[u8]) {
    keccak1::<136, 0x1fu8>(data, digest);
}

/// An incremental API for SHAKE
pub mod incremental;
