//! AVX2 SIMD backend for SHA-3.
//!
//! Pure module-declaration shim. All bodies live in:
//! - [`wrappers`] (`Libcrux_sha3.Simd.Avx2.Wrappers`) — math
//!   wrappers + `KeccakItem<4>` impl.
//! - [`load`] (`Libcrux_sha3.Simd.Avx2.Load`) — `load_block`,
//!   `load_last`, helpers, and the `Absorb<4>` impl.
//! - [`store`] (`Libcrux_sha3.Simd.Avx2.Store`) — `store_block` and
//!   the `Squeeze4` impl. (The store_block body is currently
//!   admitted; a follow-up sprint discharges it.)
//!
//! Keeping this file content-free is what tells hax NOT to emit a
//! `Libcrux_sha3.Simd.Avx2.Bundle.fst`.

pub(crate) mod wrappers;
pub(crate) mod load;
pub(crate) mod store;

// `Libcrux_sha3.Simd.Avx2.fst` is referenced by the (frozen)
// equivalence proofs (`EquivImplSpec.{Keccakf,Sponge}.Avx2.*`) and
// by `Libcrux_sha3.Generic_keccak.Simd256` via
// `let open Libcrux_sha3.Simd.Avx2 in ...`. To preserve that
// behaviour after the load/store split (where bodies moved into
// `.Wrappers`, `.Load`, `.Store`), we use a body-less ghost lemma
// to force hax to emit a parent `Libcrux_sha3.Simd.Avx2.fst`, then
// inject `include` directives to re-export the moved items.
#[cfg(hax)]
#[hax_lib::fstar::after(
    r#"
include Libcrux_sha3.Simd.Avx2.Wrappers
include Libcrux_sha3.Simd.Avx2.Load
include Libcrux_sha3.Simd.Avx2.Store
"#
)]
#[hax_lib::ensures(|_| true)]
fn module_anchor() {}
