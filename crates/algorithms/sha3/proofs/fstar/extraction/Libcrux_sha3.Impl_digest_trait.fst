module Libcrux_sha3.Impl_digest_trait
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let v_SHA3_224_LEN: usize = mk_usize 28

let v_SHA3_256_LEN: usize = mk_usize 32

let v_SHA3_384_LEN: usize = mk_usize 48

let v_SHA3_512_LEN: usize = mk_usize 64

///A struct that implements [`libcrux_traits::digest`] traits.
///\n\n
///[`Sha3_224Hasher`] is a convenient hasher for this struct.
type t_Sha3_224_ = | Sha3_224_ : t_Sha3_224_

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_traits.Digest.Arrayref.t_Hash t_Sha3_224_ (mk_usize 28) =
  {
    f_hash_pre = (fun (digest: t_Array u8 (mk_usize 28)) (payload: t_Slice u8) -> true);
    f_hash_post
    =
    (fun
        (digest: t_Array u8 (mk_usize 28))
        (payload: t_Slice u8)
        (out:
          (t_Array u8 (mk_usize 28) &
            Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError))
        ->
        true);
    f_hash
    =
    fun (digest: t_Array u8 (mk_usize 28)) (payload: t_Slice u8) ->
      if
        (Core_models.Slice.impl__len #u8 payload <: usize) >.
        (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
      then
        digest,
        (Core_models.Result.Result_Err
          (Libcrux_traits.Digest.Arrayref.HashError_InvalidPayloadLength
            <:
            Libcrux_traits.Digest.Arrayref.t_HashError)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
        <:
        (t_Array u8 (mk_usize 28) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
      else
        let digest:t_Array u8 (mk_usize 28) = Libcrux_sha3.Portable.sha224 digest payload in
        let hax_temp_output:Core_models.Result.t_Result Prims.unit
          Libcrux_traits.Digest.Arrayref.t_HashError =
          Core_models.Result.Result_Ok (() <: Prims.unit)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError
        in
        digest, hax_temp_output
        <:
        (t_Array u8 (mk_usize 28) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
  }

///A struct that implements [`libcrux_traits::digest`] traits.
///\n\n
///[`Sha3_256Hasher`] is a convenient hasher for this struct.
type t_Sha3_256_ = | Sha3_256_ : t_Sha3_256_

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: Libcrux_traits.Digest.Arrayref.t_Hash t_Sha3_256_ (mk_usize 32) =
  {
    f_hash_pre = (fun (digest: t_Array u8 (mk_usize 32)) (payload: t_Slice u8) -> true);
    f_hash_post
    =
    (fun
        (digest: t_Array u8 (mk_usize 32))
        (payload: t_Slice u8)
        (out:
          (t_Array u8 (mk_usize 32) &
            Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError))
        ->
        true);
    f_hash
    =
    fun (digest: t_Array u8 (mk_usize 32)) (payload: t_Slice u8) ->
      if
        (Core_models.Slice.impl__len #u8 payload <: usize) >.
        (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
      then
        digest,
        (Core_models.Result.Result_Err
          (Libcrux_traits.Digest.Arrayref.HashError_InvalidPayloadLength
            <:
            Libcrux_traits.Digest.Arrayref.t_HashError)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
        <:
        (t_Array u8 (mk_usize 32) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
      else
        let digest:t_Array u8 (mk_usize 32) = Libcrux_sha3.Portable.sha256 digest payload in
        let hax_temp_output:Core_models.Result.t_Result Prims.unit
          Libcrux_traits.Digest.Arrayref.t_HashError =
          Core_models.Result.Result_Ok (() <: Prims.unit)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError
        in
        digest, hax_temp_output
        <:
        (t_Array u8 (mk_usize 32) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
  }

///A struct that implements [`libcrux_traits::digest`] traits.
///\n\n
///[`Sha3_384Hasher`] is a convenient hasher for this struct.
type t_Sha3_384_ = | Sha3_384_ : t_Sha3_384_

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_2: Libcrux_traits.Digest.Arrayref.t_Hash t_Sha3_384_ (mk_usize 48) =
  {
    f_hash_pre = (fun (digest: t_Array u8 (mk_usize 48)) (payload: t_Slice u8) -> true);
    f_hash_post
    =
    (fun
        (digest: t_Array u8 (mk_usize 48))
        (payload: t_Slice u8)
        (out:
          (t_Array u8 (mk_usize 48) &
            Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError))
        ->
        true);
    f_hash
    =
    fun (digest: t_Array u8 (mk_usize 48)) (payload: t_Slice u8) ->
      if
        (Core_models.Slice.impl__len #u8 payload <: usize) >.
        (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
      then
        digest,
        (Core_models.Result.Result_Err
          (Libcrux_traits.Digest.Arrayref.HashError_InvalidPayloadLength
            <:
            Libcrux_traits.Digest.Arrayref.t_HashError)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
        <:
        (t_Array u8 (mk_usize 48) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
      else
        let digest:t_Array u8 (mk_usize 48) = Libcrux_sha3.Portable.sha384 digest payload in
        let hax_temp_output:Core_models.Result.t_Result Prims.unit
          Libcrux_traits.Digest.Arrayref.t_HashError =
          Core_models.Result.Result_Ok (() <: Prims.unit)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError
        in
        digest, hax_temp_output
        <:
        (t_Array u8 (mk_usize 48) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
  }

///A struct that implements [`libcrux_traits::digest`] traits.
///\n\n
///[`Sha3_512Hasher`] is a convenient hasher for this struct.
type t_Sha3_512_ = | Sha3_512_ : t_Sha3_512_

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_3: Libcrux_traits.Digest.Arrayref.t_Hash t_Sha3_512_ (mk_usize 64) =
  {
    f_hash_pre = (fun (digest: t_Array u8 (mk_usize 64)) (payload: t_Slice u8) -> true);
    f_hash_post
    =
    (fun
        (digest: t_Array u8 (mk_usize 64))
        (payload: t_Slice u8)
        (out:
          (t_Array u8 (mk_usize 64) &
            Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError))
        ->
        true);
    f_hash
    =
    fun (digest: t_Array u8 (mk_usize 64)) (payload: t_Slice u8) ->
      if
        (Core_models.Slice.impl__len #u8 payload <: usize) >.
        (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
      then
        digest,
        (Core_models.Result.Result_Err
          (Libcrux_traits.Digest.Arrayref.HashError_InvalidPayloadLength
            <:
            Libcrux_traits.Digest.Arrayref.t_HashError)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
        <:
        (t_Array u8 (mk_usize 64) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
      else
        let digest:t_Array u8 (mk_usize 64) = Libcrux_sha3.Portable.sha512 digest payload in
        let hax_temp_output:Core_models.Result.t_Result Prims.unit
          Libcrux_traits.Digest.Arrayref.t_HashError =
          Core_models.Result.Result_Ok (() <: Prims.unit)
          <:
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError
        in
        digest, hax_temp_output
        <:
        (t_Array u8 (mk_usize 64) &
          Core_models.Result.t_Result Prims.unit Libcrux_traits.Digest.Arrayref.t_HashError)
  }
