module Hacspec_kyber
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open Core
open FStar.Mul

unfold
let t_Ciphertext = t_Array u8 (sz 1088)

unfold
let t_PrivateKey = t_Array u8 (sz 2400)

unfold
let t_PublicKey = t_Array u8 (sz 1184)

unfold
let t_SharedSecret = t_Array u8 (sz 32)

let v_KYBER768_CIPHERTEXT_SIZE: usize = Hacspec_kyber.Parameters.v_CPA_PKE_CIPHERTEXT_SIZE

let v_KYBER768_PUBLIC_KEY_SIZE: usize = Hacspec_kyber.Parameters.v_CPA_PKE_PUBLIC_KEY_SIZE

let v_KYBER768_SHARED_SECRET_SIZE: usize = Hacspec_kyber.Parameters.v_CPA_PKE_MESSAGE_SIZE

let v_KYBER768_KEY_GENERATION_SEED_SIZE: usize =
  Hacspec_kyber.Parameters.v_CPA_PKE_KEY_GENERATION_SEED_SIZE +! v_KYBER768_SHARED_SECRET_SIZE

let v_KYBER768_SECRET_KEY_SIZE: usize =
  ((Hacspec_kyber.Parameters.v_CPA_PKE_SECRET_KEY_SIZE +!
      Hacspec_kyber.Parameters.v_CPA_PKE_PUBLIC_KEY_SIZE
      <:
      usize) +!
    Hacspec_kyber.Parameters.Hash_functions.v_H_DIGEST_SIZE
    <:
    usize) +!
  v_KYBER768_SHARED_SECRET_SIZE

let public_key_modulus_check (public_key: t_Array u8 (sz 1184)) : bool =
  let encoded_ring_elements:t_Slice u8 =
    public_key.[ {
        Core.Ops.Range.f_start = sz 0;
        Core.Ops.Range.f_end = v_KYBER768_PUBLIC_KEY_SIZE -! sz 32 <: usize
      }
      <:
      Core.Ops.Range.t_Range usize ]
  in
  let decoded_ring_elements:Hacspec_lib.Vector.t_Vector
    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
    Hacspec_kyber.Serialize.vector_decode_12_ (Core.Result.impl__unwrap (Core.Convert.f_try_into encoded_ring_elements

            <:
            Core.Result.t_Result (t_Array u8 (sz 1152)) Core.Array.t_TryFromSliceError)
        <:
        t_Array u8 (sz 1152))
  in
  encoded_ring_elements =.
  (Hacspec_kyber.Serialize.vector_encode_12_ decoded_ring_elements <: t_Array u8 (sz 1152))

type t_BadRejectionSamplingRandomnessError =
  | BadRejectionSamplingRandomnessError : t_BadRejectionSamplingRandomnessError

type t_KeyPair = {
  f_pk:t_Array u8 (sz 1184);
  f_sk:t_Array u8 (sz 2400)
}

let impl__KeyPair__new (pk: t_Array u8 (sz 1184)) (sk: t_Array u8 (sz 2400)) : t_KeyPair =
  { f_pk = pk; f_sk = sk } <: t_KeyPair

let decapsulate (ciphertext: t_Array u8 (sz 1088)) (secret_key: t_Array u8 (sz 2400))
    : t_Array u8 (sz 32) =
  let ind_cpa_secret_key, secret_key:(t_Slice u8 & t_Slice u8) =
    Core.Slice.impl__split_at (Rust_primitives.unsize secret_key <: t_Slice u8)
      Hacspec_kyber.Parameters.v_CPA_PKE_SECRET_KEY_SIZE
  in
  let ind_cpa_public_key, secret_key:(t_Slice u8 & t_Slice u8) =
    Core.Slice.impl__split_at secret_key Hacspec_kyber.Parameters.v_CPA_PKE_PUBLIC_KEY_SIZE
  in
  let ind_cpa_public_key_hash, implicit_rejection_value:(t_Slice u8 & t_Slice u8) =
    Core.Slice.impl__split_at secret_key Hacspec_kyber.Parameters.Hash_functions.v_H_DIGEST_SIZE
  in
  let decrypted:t_Array u8 (sz 32) =
    Hacspec_kyber.Ind_cpa.decrypt (Hacspec_lib.f_as_len_array (sz 1152) ind_cpa_secret_key
        <:
        t_Array u8 (sz 1152))
      ciphertext
  in
  let (to_hash: t_Array u8 (sz 64)):t_Array u8 (sz 64) =
    Hacspec_lib.f_push (sz 64) decrypted ind_cpa_public_key_hash
  in
  let hashed:t_Array u8 (sz 64) =
    Hacspec_kyber.Parameters.Hash_functions.v_G (Rust_primitives.unsize to_hash <: t_Slice u8)
  in
  let success_shared_secret, pseudorandomness:(t_Slice u8 & t_Slice u8) =
    Core.Slice.impl__split_at (Rust_primitives.unsize hashed <: t_Slice u8)
      v_KYBER768_SHARED_SECRET_SIZE
  in
  let (to_hash: t_Array u8 (sz 1120)):t_Array u8 (sz 1120) =
    Hacspec_lib.f_push (sz 1120)
      implicit_rejection_value
      (Rust_primitives.unsize ciphertext <: t_Slice u8)
  in
  let (rejection_shared_secret: t_Array u8 (sz 32)):t_Array u8 (sz 32) =
    Hacspec_kyber.Parameters.Hash_functions.v_J (sz 32)
      (Rust_primitives.unsize to_hash <: t_Slice u8)
  in
  let reencrypted_ciphertext:Core.Result.t_Result (t_Array u8 (sz 1088))
    t_BadRejectionSamplingRandomnessError =
    Hacspec_kyber.Ind_cpa.encrypt (Hacspec_lib.f_as_len_array (sz 1184) ind_cpa_public_key
        <:
        t_Array u8 (sz 1184))
      decrypted
      (Hacspec_lib.f_as_len_array (sz 32) pseudorandomness <: t_Array u8 (sz 32))
  in
  match reencrypted_ciphertext with
  | Core.Result.Result_Ok reencrypted ->
    if ciphertext =. reencrypted
    then Hacspec_lib.f_as_len_array (sz 32) success_shared_secret
    else rejection_shared_secret
  | _ -> rejection_shared_secret

let encapsulate (public_key: t_Array u8 (sz 1184)) (randomness: t_Array u8 (sz 32))
    : Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
      t_BadRejectionSamplingRandomnessError =
  Rust_primitives.Hax.Control_flow_monad.Mexception.run (let _:Prims.unit =
        if ~.(public_key_modulus_check public_key <: bool)
        then
          Rust_primitives.Hax.never_to_any (Core.Panicking.panic "assertion failed: public_key_modulus_check(&public_key)"

              <:
              Rust_primitives.Hax.t_Never)
      in
      let (to_hash: t_Array u8 (sz 64)):t_Array u8 (sz 64) =
        Hacspec_lib.f_push (sz 64)
          randomness
          (Rust_primitives.unsize (Hacspec_kyber.Parameters.Hash_functions.v_H (Rust_primitives.unsize
                      public_key
                    <:
                    t_Slice u8)
                <:
                t_Array u8 (sz 32))
            <:
            t_Slice u8)
      in
      let hashed:t_Array u8 (sz 64) =
        Hacspec_kyber.Parameters.Hash_functions.v_G (Rust_primitives.unsize to_hash <: t_Slice u8)
      in
      let shared_secret, pseudorandomness:(t_Slice u8 & t_Slice u8) =
        Core.Slice.impl__split_at (Rust_primitives.unsize hashed <: t_Slice u8)
          v_KYBER768_SHARED_SECRET_SIZE
      in
      let* ciphertext:t_Array u8 (sz 1088) =
        match
          Core.Ops.Try_trait.f_branch (Hacspec_kyber.Ind_cpa.encrypt public_key
                randomness
                (Hacspec_lib.f_as_len_array (sz 32) pseudorandomness <: t_Array u8 (sz 32))
              <:
              Core.Result.t_Result (t_Array u8 (sz 1088)) t_BadRejectionSamplingRandomnessError)
        with
        | Core.Ops.Control_flow.ControlFlow_Break residual ->
          let* hoist17:Rust_primitives.Hax.t_Never =
            Core.Ops.Control_flow.ControlFlow.v_Break (Core.Ops.Try_trait.f_from_residual residual
                <:
                Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
                  t_BadRejectionSamplingRandomnessError)
          in
          Core.Ops.Control_flow.ControlFlow_Continue (Rust_primitives.Hax.never_to_any hoist17)
          <:
          Core.Ops.Control_flow.t_ControlFlow
            (Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
                t_BadRejectionSamplingRandomnessError) (t_Array u8 (sz 1088))
        | Core.Ops.Control_flow.ControlFlow_Continue v_val ->
          Core.Ops.Control_flow.ControlFlow_Continue v_val
          <:
          Core.Ops.Control_flow.t_ControlFlow
            (Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
                t_BadRejectionSamplingRandomnessError) (t_Array u8 (sz 1088))
      in
      Core.Ops.Control_flow.ControlFlow_Continue
      (Core.Result.Result_Ok
        (ciphertext, Hacspec_lib.f_as_len_array (sz 32) shared_secret
          <:
          (t_Array u8 (sz 1088) & t_Array u8 (sz 32)))
        <:
        Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
          t_BadRejectionSamplingRandomnessError)
      <:
      Core.Ops.Control_flow.t_ControlFlow
        (Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
            t_BadRejectionSamplingRandomnessError)
        (Core.Result.t_Result (t_Array u8 (sz 1088) & t_Array u8 (sz 32))
            t_BadRejectionSamplingRandomnessError))

let generate_keypair (randomness: t_Array u8 (sz 64))
    : Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError =
  Rust_primitives.Hax.Control_flow_monad.Mexception.run (let ind_cpa_keypair_randomness:t_Slice u8 =
        randomness.[ {
            Core.Ops.Range.f_start = sz 0;
            Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_CPA_PKE_KEY_GENERATION_SEED_SIZE
          }
          <:
          Core.Ops.Range.t_Range usize ]
      in
      let implicit_rejection_value:t_Slice u8 =
        randomness.[ {
            Core.Ops.Range.f_start = Hacspec_kyber.Parameters.v_CPA_PKE_KEY_GENERATION_SEED_SIZE
          }
          <:
          Core.Ops.Range.t_RangeFrom usize ]
      in
      let* ind_cpa_key_pair:Hacspec_kyber.Ind_cpa.t_KeyPair =
        match
          Core.Ops.Try_trait.f_branch (Hacspec_kyber.Ind_cpa.generate_keypair (Hacspec_lib.f_as_len_array
                    (sz 32)
                    ind_cpa_keypair_randomness
                  <:
                  t_Array u8 (sz 32))
              <:
              Core.Result.t_Result Hacspec_kyber.Ind_cpa.t_KeyPair
                t_BadRejectionSamplingRandomnessError)
        with
        | Core.Ops.Control_flow.ControlFlow_Break residual ->
          let* hoist18:Rust_primitives.Hax.t_Never =
            Core.Ops.Control_flow.ControlFlow.v_Break (Core.Ops.Try_trait.f_from_residual residual
                <:
                Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError)
          in
          Core.Ops.Control_flow.ControlFlow_Continue (Rust_primitives.Hax.never_to_any hoist18)
          <:
          Core.Ops.Control_flow.t_ControlFlow
            (Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError)
            Hacspec_kyber.Ind_cpa.t_KeyPair
        | Core.Ops.Control_flow.ControlFlow_Continue v_val ->
          Core.Ops.Control_flow.ControlFlow_Continue v_val
          <:
          Core.Ops.Control_flow.t_ControlFlow
            (Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError)
            Hacspec_kyber.Ind_cpa.t_KeyPair
      in
      Core.Ops.Control_flow.ControlFlow_Continue
      (let secret_key_serialized:t_Array u8 (sz 2400) =
          Hacspec_kyber.Ind_cpa.impl__KeyPair__serialize_secret_key ind_cpa_key_pair
            (Hacspec_lib.f_as_len_array (sz 32) implicit_rejection_value <: t_Array u8 (sz 32))
        in
        let key_pair:t_KeyPair =
          impl__KeyPair__new (Hacspec_kyber.Ind_cpa.impl__KeyPair__pk ind_cpa_key_pair
              <:
              t_Array u8 (sz 1184))
            secret_key_serialized
        in
        Core.Result.Result_Ok key_pair
        <:
        Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError)
      <:
      Core.Ops.Control_flow.t_ControlFlow
        (Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError)
        (Core.Result.t_Result t_KeyPair t_BadRejectionSamplingRandomnessError))
