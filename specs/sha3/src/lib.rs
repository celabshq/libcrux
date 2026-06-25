/// Keccak-f[1600] permutation — exposed for cross-crate verification.
pub mod keccak_f;
mod sha3;
/// Sponge construction — exposed for cross-crate verification.
pub mod sponge;

/// Utility function to create an array of size `N` by applying a function `f` to each index.
#[hax_lib::fstar::replace(
    r#"
[@@ "opaque_to_smt"]
let createi
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
    : t_Array v_T v_N
    = Rust_primitives.Arrays.createi v_N f

let createi_lemma
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
      (i: usize{i <. v_N})
     : Lemma (Seq.index (createi #v_T v_N #v_F f) (v i) == f i)
       [SMTPat (Seq.index (createi #v_T v_N #v_F f) (v i))]
     = reveal_opaque (`%createi) (createi #v_T v_N #v_F f)
"#
)]
pub(crate) fn createi<T, const N: usize, F: Fn(usize) -> T>(f: F) -> [T; N] {
    core::array::from_fn(f)
}

pub use keccak_f::State;
pub use sha3::{sha3_224, sha3_256, sha3_384, sha3_512, shake128, shake256};
