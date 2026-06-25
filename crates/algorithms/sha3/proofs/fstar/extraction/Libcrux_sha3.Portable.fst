module Libcrux_sha3.Portable
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// The Keccak state for the incremental API.
type t_KeccakState = { f_state:Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 }

let impl: Core_models.Clone.t_Clone t_KeccakState =
  { f_clone = (fun x -> x); f_clone_pre = (fun _ -> True); f_clone_post = (fun _ _ -> True) }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1': Core_models.Marker.t_Copy t_KeccakState

unfold
let impl_1 = impl_1'

/// A portable SHA3 224 implementation.
let sha224 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 digest <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 digest)
              (mk_usize 144)
              (mk_u8 6)
              data
            <:
            t_Slice u8)) =
  let digest:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.keccak1 (mk_usize 144) (mk_u8 6) data digest
  in
  digest

/// A portable SHA3 256 implementation.
let sha256 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 digest <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 digest)
              (mk_usize 136)
              (mk_u8 6)
              data
            <:
            t_Slice u8)) =
  let digest:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.keccak1 (mk_usize 136) (mk_u8 6) data digest
  in
  digest

/// A portable SHA3 384 implementation.
let sha384 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 digest <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 digest)
              (mk_usize 104)
              (mk_u8 6)
              data
            <:
            t_Slice u8)) =
  let digest:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.keccak1 (mk_usize 104) (mk_u8 6) data digest
  in
  digest

/// A portable SHA3 512 implementation.
let sha512 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 digest <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 digest)
              (mk_usize 72)
              (mk_u8 6)
              data
            <:
            t_Slice u8)) =
  let digest:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.keccak1 (mk_usize 72) (mk_u8 6) data digest
  in
  digest

/// A portable SHAKE128 implementation.
let shake128 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 digest <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 digest)
              (mk_usize 168)
              (mk_u8 31)
              data
            <:
            t_Slice u8)) =
  let digest:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.keccak1 (mk_usize 168) (mk_u8 31) data digest
  in
  digest

/// A portable SHAKE256 implementation.
let shake256 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 digest <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 digest)
              (mk_usize 136)
              (mk_u8 31)
              data
            <:
            t_Slice u8)) =
  let digest:t_Slice u8 =
    Libcrux_sha3.Generic_keccak.Portable.keccak1 (mk_usize 136) (mk_u8 31) data digest
  in
  digest
