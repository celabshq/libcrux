module Libcrux_sha3
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// Size in bytes of a SHA3 224 digest.
let v_SHA3_224_DIGEST_SIZE: usize = mk_usize 28

/// Size in bytes of a SHA3 256 digest.
let v_SHA3_256_DIGEST_SIZE: usize = mk_usize 32

/// Size in bytes of a SHA3 2384 digest.
let v_SHA3_384_DIGEST_SIZE: usize = mk_usize 48

/// Size in bytes of a SHA3 512 digest.
let v_SHA3_512_DIGEST_SIZE: usize = mk_usize 64

/// The Digest Algorithm.
type t_Algorithm =
  | Algorithm_Sha224 : t_Algorithm
  | Algorithm_Sha256 : t_Algorithm
  | Algorithm_Sha384 : t_Algorithm
  | Algorithm_Sha512 : t_Algorithm

let anon_const_Algorithm_Sha224__anon_const_0: u32 = mk_u32 1

let anon_const_Algorithm_Sha256__anon_const_0: u32 = mk_u32 2

let anon_const_Algorithm_Sha384__anon_const_0: u32 = mk_u32 3

let anon_const_Algorithm_Sha512__anon_const_0: u32 = mk_u32 4

let t_Algorithm_cast_to_repr (x: t_Algorithm) : u32 =
  match x <: t_Algorithm with
  | Algorithm_Sha224  -> anon_const_Algorithm_Sha224__anon_const_0
  | Algorithm_Sha256  -> anon_const_Algorithm_Sha256__anon_const_0
  | Algorithm_Sha384  -> anon_const_Algorithm_Sha384__anon_const_0
  | Algorithm_Sha512  -> anon_const_Algorithm_Sha512__anon_const_0

let impl_1: Core_models.Clone.t_Clone t_Algorithm =
  { f_clone = (fun x -> x); f_clone_pre = (fun _ -> True); f_clone_post = (fun _ _ -> True) }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_2': Core_models.Marker.t_Copy t_Algorithm

unfold
let impl_2 = impl_2'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_3': Core_models.Fmt.t_Debug t_Algorithm

unfold
let impl_3 = impl_3'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_4': Core_models.Marker.t_StructuralPartialEq t_Algorithm

unfold
let impl_4 = impl_4'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_5': Core_models.Cmp.t_PartialEq t_Algorithm t_Algorithm

unfold
let impl_5 = impl_5'

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Core_models.Convert.t_From u32 t_Algorithm =
  {
    f_from_pre = (fun (v: t_Algorithm) -> true);
    f_from_post = (fun (v: t_Algorithm) (out: u32) -> true);
    f_from
    =
    fun (v: t_Algorithm) ->
      match v <: t_Algorithm with
      | Algorithm_Sha224  -> mk_u32 1
      | Algorithm_Sha256  -> mk_u32 2
      | Algorithm_Sha384  -> mk_u32 3
      | Algorithm_Sha512  -> mk_u32 4
  }

/// Returns the output size of a digest.
let digest_size (mode: t_Algorithm) : usize =
  match mode <: t_Algorithm with
  | Algorithm_Sha224  -> v_SHA3_224_DIGEST_SIZE
  | Algorithm_Sha256  -> v_SHA3_256_DIGEST_SIZE
  | Algorithm_Sha384  -> v_SHA3_384_DIGEST_SIZE
  | Algorithm_Sha512  -> v_SHA3_512_DIGEST_SIZE

#push-options "--split_queries always"

/// SHA3
let hash (v_LEN: usize) (algorithm: t_Algorithm) (payload: t_Slice u8)
    : Prims.Pure (t_Array u8 v_LEN)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 payload <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int) &&
        v_LEN <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 payload <: usize) <=.
            (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
            <:
            bool)
      in
      ()
  in
  let out:t_Array u8 v_LEN = Rust_primitives.Hax.repeat (mk_u8 0) v_LEN in
  let out:t_Array u8 v_LEN =
    match algorithm <: t_Algorithm with
    | Algorithm_Sha224  -> Libcrux_sha3.Portable.sha224 out payload
    | Algorithm_Sha256  -> Libcrux_sha3.Portable.sha256 out payload
    | Algorithm_Sha384  -> Libcrux_sha3.Portable.sha384 out payload
    | Algorithm_Sha512  -> Libcrux_sha3.Portable.sha512 out payload
  in
  out

#pop-options

/// SHA3 224
/// Preconditions:
/// - `digest.len() == 28`
let sha224_ema (digest payload: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 payload <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int) &&
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 digest <: usize)
          <:
          Hax_lib.Int.t_Int) =
        (28 <: Hax_lib.Int.t_Int))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (mk_usize 28) (mk_usize 144) (mk_u8 6) payload <: t_Slice u8)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 payload <: usize) <=.
            (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
            <:
            bool)
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 28 <: bool)
      in
      ()
  in
  let digest:t_Slice u8 = Libcrux_sha3.Portable.sha224 digest payload in
  digest

/// SHA3 224
let sha224 (data: t_Slice u8)
    : Prims.Pure (t_Array u8 (mk_usize 28))
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int))
      (ensures
        fun result ->
          let result:t_Array u8 (mk_usize 28) = result in
          (result <: t_Array u8 (mk_usize 28)) ==
          Hacspec_sha3.Sponge.keccak (mk_usize 28) (mk_usize 144) (mk_u8 6) data) =
  let out:t_Array u8 (mk_usize 28) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 28) in
  let out:t_Array u8 (mk_usize 28) = sha224_ema out data in
  out

/// SHA3 256
let sha256_ema (digest payload: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 payload <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int) &&
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 digest <: usize)
          <:
          Hax_lib.Int.t_Int) =
        (32 <: Hax_lib.Int.t_Int))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (mk_usize 32) (mk_usize 136) (mk_u8 6) payload <: t_Slice u8)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 payload <: usize) <=.
            (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
            <:
            bool)
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 32 <: bool)
      in
      ()
  in
  let digest:t_Slice u8 = Libcrux_sha3.Portable.sha256 digest payload in
  digest

/// SHA3 256
let sha256 (data: t_Slice u8)
    : Prims.Pure (t_Array u8 (mk_usize 32))
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int))
      (ensures
        fun result ->
          let result:t_Array u8 (mk_usize 32) = result in
          (result <: t_Array u8 (mk_usize 32)) ==
          Hacspec_sha3.Sponge.keccak (mk_usize 32) (mk_usize 136) (mk_u8 6) data) =
  let out:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
  let out:t_Array u8 (mk_usize 32) = sha256_ema out data in
  out

/// SHA3 384
let sha384_ema (digest payload: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 payload <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int) &&
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 digest <: usize)
          <:
          Hax_lib.Int.t_Int) =
        (48 <: Hax_lib.Int.t_Int))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (mk_usize 48) (mk_usize 104) (mk_u8 6) payload <: t_Slice u8)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 payload <: usize) <=.
            (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
            <:
            bool)
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 48 <: bool)
      in
      ()
  in
  let digest:t_Slice u8 = Libcrux_sha3.Portable.sha384 digest payload in
  digest

/// SHA3 384
let sha384 (data: t_Slice u8)
    : Prims.Pure (t_Array u8 (mk_usize 48))
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int))
      (ensures
        fun result ->
          let result:t_Array u8 (mk_usize 48) = result in
          (result <: t_Array u8 (mk_usize 48)) ==
          Hacspec_sha3.Sponge.keccak (mk_usize 48) (mk_usize 104) (mk_u8 6) data) =
  let out:t_Array u8 (mk_usize 48) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 48) in
  let out:t_Array u8 (mk_usize 48) = sha384_ema out data in
  out

/// SHA3 512
let sha512_ema (digest payload: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 payload <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int) &&
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 digest <: usize)
          <:
          Hax_lib.Int.t_Int) =
        (64 <: Hax_lib.Int.t_Int))
      (ensures
        fun digest_future ->
          let digest_future:t_Slice u8 = digest_future in
          b2t
          ((Core_models.Slice.impl__len #u8 digest_future <: usize) =.
            (Core_models.Slice.impl__len #u8 digest <: usize)
            <:
            bool) /\
          (digest_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (mk_usize 64) (mk_usize 72) (mk_u8 6) payload <: t_Slice u8)) =
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 payload <: usize) <=.
            (cast (Core_models.Num.impl_u32__MAX <: u32) <: usize)
            <:
            bool)
      in
      ()
  in
  let _:Prims.unit =
    if true
    then
      let _:Prims.unit =
        Hax_lib.v_assert ((Core_models.Slice.impl__len #u8 digest <: usize) =. mk_usize 64 <: bool)
      in
      ()
  in
  let digest:t_Slice u8 = Libcrux_sha3.Portable.sha512 digest payload in
  digest

/// SHA3 512
let sha512 (data: t_Slice u8)
    : Prims.Pure (t_Array u8 (mk_usize 64))
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data <: usize)
          <:
          Hax_lib.Int.t_Int) <=
        (Rust_primitives.Hax.Int.from_machine Core_models.Num.impl_u32__MAX <: Hax_lib.Int.t_Int))
      (ensures
        fun result ->
          let result:t_Array u8 (mk_usize 64) = result in
          (result <: t_Array u8 (mk_usize 64)) ==
          Hacspec_sha3.Sponge.keccak (mk_usize 64) (mk_usize 72) (mk_u8 6) data) =
  let out:t_Array u8 (mk_usize 64) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
  let out:t_Array u8 (mk_usize 64) = sha512_ema out data in
  out

/// SHAKE 128
/// Note that the output length `BYTES` must fit into 32 bit. If it is longer,
/// the output will only return `u32::MAX` bytes.
let shake128 (v_BYTES: usize) (data: t_Slice u8)
    : Prims.Pure (t_Array u8 v_BYTES)
      (requires v_BYTES <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun result ->
          let result:t_Array u8 v_BYTES = result in
          (result <: t_Array u8 v_BYTES) ==
          Hacspec_sha3.Sponge.keccak v_BYTES (mk_usize 168) (mk_u8 31) data) =
  let out:t_Array u8 v_BYTES = Rust_primitives.Hax.repeat (mk_u8 0) v_BYTES in
  let out:t_Array u8 v_BYTES = Libcrux_sha3.Portable.shake128 out data in
  out

/// SHAKE 128
/// Writes `out.len()` bytes.
let shake128_ema (out data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          b2t
          ((Core_models.Slice.impl__len #u8 out_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out <: usize)
            <:
            bool) /\
          (out_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 out)
              (mk_usize 168)
              (mk_u8 31)
              data
            <:
            t_Slice u8)) =
  let out:t_Slice u8 = Libcrux_sha3.Portable.shake128 out data in
  out

/// SHAKE 256
/// Note that the output length `BYTES` must fit into 32 bit. If it is longer,
/// the output will only return `u32::MAX` bytes.
let shake256 (v_BYTES: usize) (data: t_Slice u8)
    : Prims.Pure (t_Array u8 v_BYTES)
      (requires v_BYTES <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun result ->
          let result:t_Array u8 v_BYTES = result in
          (result <: t_Array u8 v_BYTES) ==
          Hacspec_sha3.Sponge.keccak v_BYTES (mk_usize 136) (mk_u8 31) data) =
  let out:t_Array u8 v_BYTES = Rust_primitives.Hax.repeat (mk_u8 0) v_BYTES in
  let out:t_Array u8 v_BYTES = Libcrux_sha3.Portable.shake256 out data in
  out

/// SHAKE 256
/// Writes `out.len()` bytes.
let shake256_ema (out data: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #u8 out <: usize) <.
        (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          b2t
          ((Core_models.Slice.impl__len #u8 out_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out <: usize)
            <:
            bool) /\
          (out_future <: t_Slice u8) ==
          (Hacspec_sha3.Sponge.keccak (Core_models.Slice.impl__len #u8 out)
              (mk_usize 136)
              (mk_u8 31)
              data
            <:
            t_Slice u8)) =
  let out:t_Slice u8 = Libcrux_sha3.Portable.shake256 out data in
  out
