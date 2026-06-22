#![cfg_attr(hax_backend_lean, feature(register_tool))]
#![cfg_attr(hax_backend_lean, register_tool(charon))]
/// Keccak-f[1600] permutation — exposed for cross-spec testing.
pub mod keccak_f;
mod sha3;
/// Sponge construction — exposed for cross-spec testing.
pub mod sponge;

/// Utility function to create an array of size `N` by applying a function `f` to each index.
/// This is needed to inject our custom F* implementation below.
#[hax_lib::fstar::replace(
    r#"
let createi
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
    : t_Array v_T v_N
    = Rust_primitives.Arrays.createi v_N f
"#
)]
#[cfg(not(hax_backend_lean))]
pub(crate) fn createi<T, const N: usize, F: Fn(usize) -> T>(f: F) -> [T; N] {
    core::array::from_fn(f)
}

// For Lean extraction, we need to use this alternative function taking `FnMut` instead of `Fn`.
// This is due to an Aeneas bug: https://github.com/AeneasVerif/aeneas/issues/924
#[cfg(hax_backend_lean)]
pub(crate) fn createi<T, const N: usize, F: FnMut(usize) -> T>(f: F) -> [T; N] {
    core::array::from_fn(f)
}

pub use keccak_f::State;
pub use sha3::{sha3_224, sha3_256, sha3_384, sha3_512, shake128, shake256};
