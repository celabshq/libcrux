//! SHA-3 as FIPS 202 writes it.
//!
//! This crate is a second specification of SHA-3, next to `hacspec_sha3`. The
//! two agree on every input; they differ in what they are for. `hacspec_sha3`
//! is the reference the implementations are verified against, so it is written
//! in the terms an implementation uses: the state is `[u64; 25]`, messages are
//! bytes, the round constants and rotation offsets are tables. This crate is
//! written in the terms the *Standard* uses, so that it can be read next to
//! the document:
//!
//! * the state is the 5-by-5-by-`w` array of bits `A[x, y, z]` of Sec. 3.1,
//!   and messages are bit strings;
//! * every function is one numbered Algorithm, with the Steps in the
//!   Standard's order and the Standard's names;
//! * nothing is precomputed that the Standard computes: `ρ`'s offsets come
//!   from the `(x, y)` walk of Algorithm 2, and `ι`'s round constants from the
//!   `rc` LFSR of Algorithm 5;
//! * the permutation is generic in the width: `KECCAK-p[b, n_r]` for any of
//!   the seven `b` in Table 1, not only `b = 1600`.
//!
//! It is therefore slow, and it is meant to be. The point is that a reader
//! with the PDF open can check it line by line.
//!
//! | Standard | here |
//! |---|---|
//! | Sec. 2.3 basic operations, App. B.1 `h2b`/`b2h` | [`bits`] |
//! | Sec. 3.1 state, state array, conversions | [`state_array`] |
//! | Sec. 3.2 Algorithms 1-6, `θ ρ π χ ι` and `rc` | [`step_mappings`] |
//! | Sec. 3.3-3.4 Algorithm 7, `KECCAK-p` / `KECCAK-f` | [`keccak_p`] |
//! | Sec. 4, 5.1, 5.2 Algorithms 8-9, `SPONGE`, `pad10*1`, `KECCAK[c]` | [`sponge`] |
//! | Sec. 6.1-6.2 the six SHA-3 functions | [`sha3`] |
//! | — (byte-aligned convenience) | [`bytes`] |
//!
//! The document is `specs/sha3/NIST.FIPS.202.pdf`. The crate extracts to Lean
//! (`cargo bin cargo-hax extract hacspec-sha3-pedantic`); `Readme.md` lists the
//! handful of places where the extraction needed something other than the
//! Standard's own phrasing.

// The loops below are the Standard's "For all triples (x, y, z) such that …"
// quantifiers written out; the index variables are exactly the point, so the
// lint that would have us iterate the arrays instead does not apply here.
#![allow(clippy::needless_range_loop)]
// `0^n` and the zeros of `pad10*1` are written as a `push` per bit. The
// Standard says only that the string is `n` zeros and leaves the construction
// open; a `push` loop is what extracts, and `vec![false; n]` and `Vec::extend`
// do not.
#![allow(clippy::same_item_push)]

pub mod bits;
pub mod bytes;
pub mod keccak_p;
pub mod sha3;
pub mod sponge;
pub mod state_array;
pub mod step_mappings;

pub use sha3::{sha3_224, sha3_256, sha3_384, sha3_512, shake128, shake256};
pub use state_array::StateArray;
