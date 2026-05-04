module Libcrux_sha3.Neon
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"

/// A SHA3 224 implementation.
let sha224 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires (Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 28)
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
  let dummy:t_Array u8 (mk_usize 28) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 28) in
  let _:Prims.unit =
    let inputs:t_Array (t_Slice u8) (mk_usize 2) =
      let l:list (t_Slice u8) = [data; data] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l
    in
    EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64 (mk_usize 144)
      (mk_u8 6)
      inputs
      digest
      (dummy <: t_Slice u8)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Array u8 (mk_usize 28)) =
    Libcrux_sha3.Generic_keccak.Simd128.keccak2 (mk_usize 144)
      (mk_u8 6)
      (let list = [data; data] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      digest
      dummy
  in
  let digest:t_Slice u8 = tmp0 in
  let dummy:t_Array u8 (mk_usize 28) = tmp1 in
  let _:Prims.unit = () in
  digest

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"

/// A SHA3 256 implementation.
let sha256 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires (Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 32)
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
  let dummy:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
  let _:Prims.unit =
    let inputs:t_Array (t_Slice u8) (mk_usize 2) =
      let l:list (t_Slice u8) = [data; data] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l
    in
    EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64 (mk_usize 136)
      (mk_u8 6)
      inputs
      digest
      (dummy <: t_Slice u8)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Array u8 (mk_usize 32)) =
    Libcrux_sha3.Generic_keccak.Simd128.keccak2 (mk_usize 136)
      (mk_u8 6)
      (let list = [data; data] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      digest
      dummy
  in
  let digest:t_Slice u8 = tmp0 in
  let dummy:t_Array u8 (mk_usize 32) = tmp1 in
  let _:Prims.unit = () in
  digest

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"

/// A SHA3 384 implementation.
let sha384 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires (Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 48)
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
  let dummy:t_Array u8 (mk_usize 48) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 48) in
  let _:Prims.unit =
    let inputs:t_Array (t_Slice u8) (mk_usize 2) =
      let l:list (t_Slice u8) = [data; data] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l
    in
    EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64 (mk_usize 104)
      (mk_u8 6)
      inputs
      digest
      (dummy <: t_Slice u8)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Array u8 (mk_usize 48)) =
    Libcrux_sha3.Generic_keccak.Simd128.keccak2 (mk_usize 104)
      (mk_u8 6)
      (let list = [data; data] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      digest
      dummy
  in
  let digest:t_Slice u8 = tmp0 in
  let dummy:t_Array u8 (mk_usize 48) = tmp1 in
  let _:Prims.unit = () in
  digest

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"

/// A SHA3 512 implementation.
let sha512 (digest data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires (Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 64)
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
  let dummy:t_Array u8 (mk_usize 64) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
  let _:Prims.unit =
    let inputs:t_Array (t_Slice u8) (mk_usize 2) =
      let l:list (t_Slice u8) = [data; data] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l
    in
    EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64 (mk_usize 72)
      (mk_u8 6)
      inputs
      digest
      (dummy <: t_Slice u8)
  in
  let (tmp0: t_Slice u8), (tmp1: t_Array u8 (mk_usize 64)) =
    Libcrux_sha3.Generic_keccak.Simd128.keccak2 (mk_usize 72)
      (mk_u8 6)
      (let list = [data; data] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      digest
      dummy
  in
  let digest:t_Slice u8 = tmp0 in
  let dummy:t_Array u8 (mk_usize 64) = tmp1 in
  let _:Prims.unit = () in
  digest

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"

/// A SHAKE128 implementation.
let shake128 (v_LEN: usize) (digest: t_Array u8 v_LEN) (data: t_Slice u8)
    : Prims.Pure (t_Array u8 v_LEN)
      (requires v_LEN <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Array u8 v_LEN = digest_future in
          (digest_future <: t_Array u8 v_LEN) ==
          (Hacspec_sha3.Sponge.keccak v_LEN (mk_usize 168) (mk_u8 31) data <: t_Array u8 v_LEN)) =
  let dummy:t_Array u8 v_LEN = Rust_primitives.Hax.repeat (mk_u8 0) v_LEN in
  let _:Prims.unit =
    let inputs:t_Array (t_Slice u8) (mk_usize 2) =
      let l:list (t_Slice u8) = [data; data] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l
    in
    EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64 (mk_usize 168)
      (mk_u8 31)
      inputs
      (digest <: t_Slice u8)
      (dummy <: t_Slice u8)
  in
  let (tmp0: t_Array u8 v_LEN), (tmp1: t_Array u8 v_LEN) =
    Libcrux_sha3.Generic_keccak.Simd128.keccak2 (mk_usize 168)
      (mk_u8 31)
      (let list = [data; data] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      digest
      dummy
  in
  let digest:t_Array u8 v_LEN = tmp0 in
  let dummy:t_Array u8 v_LEN = tmp1 in
  let _:Prims.unit = () in
  digest

#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"

/// A SHAKE256 implementation.
let shake256 (v_LEN: usize) (digest: t_Array u8 v_LEN) (data: t_Slice u8)
    : Prims.Pure (t_Array u8 v_LEN)
      (requires v_LEN <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun digest_future ->
          let digest_future:t_Array u8 v_LEN = digest_future in
          (digest_future <: t_Array u8 v_LEN) ==
          (Hacspec_sha3.Sponge.keccak v_LEN (mk_usize 136) (mk_u8 31) data <: t_Array u8 v_LEN)) =
  let dummy:t_Array u8 v_LEN = Rust_primitives.Hax.repeat (mk_u8 0) v_LEN in
  let _:Prims.unit =
    let inputs:t_Array (t_Slice u8) (mk_usize 2) =
      let l:list (t_Slice u8) = [data; data] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l
    in
    EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64 (mk_usize 136)
      (mk_u8 31)
      inputs
      (digest <: t_Slice u8)
      (dummy <: t_Slice u8)
  in
  let (tmp0: t_Array u8 v_LEN), (tmp1: t_Array u8 v_LEN) =
    Libcrux_sha3.Generic_keccak.Simd128.keccak2 (mk_usize 136)
      (mk_u8 31)
      (let list = [data; data] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
        Rust_primitives.Hax.array_of_list 2 list)
      digest
      dummy
  in
  let digest:t_Array u8 v_LEN = tmp0 in
  let dummy:t_Array u8 v_LEN = tmp1 in
  let _:Prims.unit = () in
  digest

#pop-options
