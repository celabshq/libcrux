//! Arm64 (NEON) SIMD backend for SHA-3.
//!
//! Pure module-declaration shim. All bodies live in:
//! - [`wrappers`] (`Libcrux_sha3.Simd.Arm64.Wrappers`) — math
//!   wrappers + `KeccakItem<2>` impl + the `uint64x2_t` type alias.
//! - [`load`] (`Libcrux_sha3.Simd.Arm64.Load`) — `load_block`,
//!   `load_last`, helpers, and the `Absorb<2>` impl.
//! - [`store`] (`Libcrux_sha3.Simd.Arm64.Store`) — `store_block` (and
//!   `_full` / `_tail` decomposition) plus the `Squeeze2` impl.
//!
//! Keeping this file content-free is what tells hax NOT to emit a
//! `Libcrux_sha3.Simd.Arm64.Bundle.fst`: when there are no top-level
//! items here that cross-refer into the submodules, each submodule
//! gets its own clean `.fst` and the per-module SMT context is
//! actually small.

pub(crate) mod load;
pub(crate) mod store;
pub(crate) mod wrappers;

// Re-export `uint64x2_t` so callers (`neon.rs`, `generic_keccak`) can
// reference `crate::simd::arm64::uint64x2_t`.
pub use wrappers::uint64x2_t;

// `Libcrux_sha3.Simd.Arm64.fst` is referenced by the equivalence proofs
// (`EquivImplSpec.{Keccakf,Sponge}.Arm64.*`) and by
// `Libcrux_sha3.Generic_keccak.Simd128` via
// `let open Libcrux_sha3.Simd.Arm64 in ...`. A body-less ghost lemma
// forces hax to emit this parent module, and injected `include`
// directives re-export the items of `.Wrappers`, `.Load` and `.Store`.
#[cfg(hax)]
#[cfg_attr(
    hax,
    hax_lib::fstar::after(
        r#"
include Libcrux_sha3.Simd.Arm64.Wrappers
include Libcrux_sha3.Simd.Arm64.Load
include Libcrux_sha3.Simd.Arm64.Store
"#
    )
)]
#[cfg_attr(hax, hax_lib::ensures(|_| true))]
fn module_anchor() {}
