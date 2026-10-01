//! AVX2 SIMD backend for SHA-3.
//!
//! Pure module-declaration shim. All bodies live in:
//! - [`wrappers`] (`Libcrux_sha3.Simd.Avx2.Wrappers`) — math
//!   wrappers + `KeccakItem<4>` impl.
//! - [`load`] (`Libcrux_sha3.Simd.Avx2.Load`) — `load_block`,
//!   `load_last`, helpers, and the `Absorb<4>` impl.
//! - [`store`] (`Libcrux_sha3.Simd.Avx2.Store`) — `store_block` and
//!   the `Squeeze4` impl.
//!
//! Keeping this file content-free is what tells hax NOT to emit a
//! `Libcrux_sha3.Simd.Avx2.Bundle.fst`.

pub(crate) mod load;
pub(crate) mod store;
pub(crate) mod wrappers;

// `Libcrux_sha3.Simd.Avx2.fst` is referenced by the equivalence proofs
// (`EquivImplSpec.{Keccakf,Sponge}.Avx2.*`) and by
// `Libcrux_sha3.Generic_keccak.Simd256` via
// `let open Libcrux_sha3.Simd.Avx2 in ...`. A body-less ghost lemma
// forces hax to emit this parent module, and injected `include`
// directives re-export the items of `.Wrappers`, `.Load` and `.Store`.
#[cfg(hax)]
#[cfg_attr(
    hax,
    hax_lib::fstar::after(
        r#"
include Libcrux_sha3.Simd.Avx2.Wrappers
include Libcrux_sha3.Simd.Avx2.Load
include Libcrux_sha3.Simd.Avx2.Store
"#
    )
)]
#[cfg_attr(hax, hax_lib::ensures(|_| true))]
fn module_anchor() {}
