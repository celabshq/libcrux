use generic_keccak::KeccakState as GenericState;
use hax_lib;
#[cfg(hax)]
use hax_lib::int::*;
#[cfg(hax)]
use hax_lib::prop::*;

use crate::generic_keccak::{self, portable::keccak1};

/// The Keccak state for the incremental API.
// The `.fsti` interface (generated when this module is extracted with
// `--interfaces`) exposes the concrete `t_KeccakState` record, whose field type
// `GenericState<1, u64>` carries a `KeccakItem u64 1` typeclass constraint.  hax
// emits the implicit-dependency `let _` that resolves that instance into the
// `.fst` but not the `.fsti`, so name it explicitly for the interface here.
#[cfg_attr(hax, hax_lib::fstar::before(interface, r#"let _ =
  let open Libcrux_sha3.Simd.Portable in
  let open Libcrux_sha3.Traits in
  ()"#))]
#[derive(Clone, Copy)]
pub struct KeccakState {
    state: GenericState<1, u64>,
}

/// A portable SHA3 224 implementation.
#[inline(always)]
#[hax_lib::requires(digest.len() < usize::MAX - 200)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 144) (mk_u8 6) $data <: t_Slice u8)"#)
})]
pub fn sha224(digest: &mut [u8], data: &[u8]) {
    keccak1::<144, 0x06u8>(data, digest);
}

/// A portable SHA3 256 implementation.
#[inline(always)]
#[hax_lib::requires(digest.len() < usize::MAX - 200)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 136) (mk_u8 6) $data <: t_Slice u8)"#)
})]
pub fn sha256(digest: &mut [u8], data: &[u8]) {
    keccak1::<136, 0x06u8>(data, digest);
}

/// A portable SHA3 384 implementation.
#[inline(always)]
#[hax_lib::requires(digest.len() < usize::MAX - 200)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 104) (mk_u8 6) $data <: t_Slice u8)"#)
})]
pub fn sha384(digest: &mut [u8], data: &[u8]) {
    keccak1::<104, 0x06u8>(data, digest);
}

/// A portable SHA3 512 implementation.
#[inline(always)]
#[hax_lib::requires(digest.len() < usize::MAX - 200)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 72) (mk_u8 6) $data <: t_Slice u8)"#)
})]
pub fn sha512(digest: &mut [u8], data: &[u8]) {
    keccak1::<72, 0x06u8>(data, digest);
}

/// A portable SHAKE128 implementation.
#[inline(always)]
#[hax_lib::requires(digest.len() < usize::MAX - 200)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 168) (mk_u8 31) $data <: t_Slice u8)"#)
})]
pub fn shake128(digest: &mut [u8], data: &[u8]) {
    keccak1::<168, 0x1fu8>(data, digest);
}

/// A portable SHAKE256 implementation.
#[inline(always)]
#[hax_lib::requires(digest.len() < usize::MAX - 200)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 136) (mk_u8 31) $data <: t_Slice u8)"#)
})]
pub fn shake256(digest: &mut [u8], data: &[u8]) {
    keccak1::<136, 0x1fu8>(data, digest);
}

/// An incremental API for SHAKE
pub mod incremental;
