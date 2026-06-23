module Hacspec_ml_kem.Ind_cca
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

#push-options "--z3rlimit 1500"

/// Algorithm 16: ML-KEM.KeyGen_internal
/// ```plaintext
/// Input: d ∈ 𝔹³², z ∈ 𝔹³².
/// Output: encapsulation key ek ∈ 𝔹^{384k+32}.
/// Output: decapsulation key dk ∈ 𝔹^{768k+96}.
/// (ekₚₖₑ, dkₚₖₑ) ← K-PKE.KeyGen(d)
/// ek ← ekₚₖₑ
/// dk ← (dkₚₖₑ ‖ ek ‖ H(ek) ‖ z)
/// return (ek, dk)
/// ```
let keygen_internal
      (v_RANK v_EK_SIZE v_DK_PKE_SIZE v_DK_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (d z: t_Array u8 (mk_usize 32))
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_SIZE)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        v_DK_PKE_SIZE =. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) &&
        v_DK_SIZE =.
        (((v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
            Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
            <:
            usize) +!
          mk_usize 32
          <:
          usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  match
    Hacspec_ml_kem.Ind_cpa.generate_keypair v_RANK v_EK_SIZE v_DK_PKE_SIZE params (d <: t_Slice u8)
    <:
    Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_PKE_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok (ek, dk_pke) ->
    let dk:t_Array u8 v_DK_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_DK_SIZE in
    let dk:t_Array u8 v_DK_SIZE =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to dk
        ({ Core_models.Ops.Range.f_end = v_DK_PKE_SIZE } <: Core_models.Ops.Range.t_RangeTo usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (dk.[ { Core_models.Ops.Range.f_end = v_DK_PKE_SIZE }
                <:
                Core_models.Ops.Range.t_RangeTo usize ]
              <:
              t_Slice u8)
            (dk_pke <: t_Slice u8)
          <:
          t_Slice u8)
    in
    let dk:t_Array u8 v_DK_SIZE =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range dk
        ({
            Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
            Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize
          }
          <:
          Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (dk.[ {
                  Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
                  Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize ]
              <:
              t_Slice u8)
            (ek <: t_Slice u8)
          <:
          t_Slice u8)
    in
    let dk:t_Array u8 v_DK_SIZE =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range dk
        ({
            Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
            Core_models.Ops.Range.f_end
            =
            (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
            Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
            <:
            usize
          }
          <:
          Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (dk.[ {
                  Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
                  Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
                  <:
                  usize
                }
                <:
                Core_models.Ops.Range.t_Range usize ]
              <:
              t_Slice u8)
            (Hacspec_ml_kem.Parameters.Hash_functions.v_H (ek <: t_Slice u8) <: t_Slice u8)
          <:
          t_Slice u8)
    in
    let dk:t_Array u8 v_DK_SIZE =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from dk
        ({
            Core_models.Ops.Range.f_start
            =
            (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
            Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
            <:
            usize
          }
          <:
          Core_models.Ops.Range.t_RangeFrom usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (dk.[ {
                  Core_models.Ops.Range.f_start
                  =
                  (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
                  Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
                  <:
                  usize
                }
                <:
                Core_models.Ops.Range.t_RangeFrom usize ]
              <:
              t_Slice u8)
            (z <: t_Slice u8)
          <:
          t_Slice u8)
    in
    Core_models.Result.Result_Ok (ek, dk <: (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_SIZE))
    <:
    Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 1500"

/// Algorithm 17: ML-KEM.Encaps_internal
/// ```plaintext
/// Input: encapsulation key ek ∈ 𝔹^{384k+32}.
/// Input: m ∈ 𝔹³².
/// Output: shared key K ∈ 𝔹³².
/// Output: ciphertext c ∈ 𝔹^{32(dᵤk+dᵥ)}.
/// (K, r) ← G(m ‖ H(ek))
/// c ← K-PKE.Encrypt(ek, m, r)
/// return (K, c)
/// ```
let encaps_internal
      (v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (ek: t_Slice u8)
      (m: t_Array u8 (mk_usize 32))
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
            params.Hacspec_ml_kem.Parameters.f_du
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
            params.Hacspec_ml_kem.Parameters.f_dv
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_CT_SIZE =. (v_U_SIZE +! v_V_SIZE <: usize) &&
        (Core_models.Slice.impl__len #u8 ek <: usize) =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let to_hash:t_Array u8 (mk_usize 64) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to to_hash
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (m <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = mk_usize 32 } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (Hacspec_ml_kem.Parameters.Hash_functions.v_H ek <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let hashed:t_Array u8 (mk_usize 64) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_G (to_hash <: t_Slice u8)
  in
  let (shared_secret: t_Slice u8), (pseudorandomness: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8 (hashed <: t_Slice u8) (mk_usize 32)
  in
  let (r: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8)
          #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (pseudorandomness.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
        <:
        Core_models.Result.t_Result (t_Array u8 (mk_usize 32)) Core_models.Array.t_TryFromSliceError
      )
  in
  match
    Hacspec_ml_kem.Ind_cpa.encrypt v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE params ek m (r <: t_Slice u8)
    <:
    Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok c ->
    let k:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
    let k:t_Array u8 (mk_usize 32) = Core_models.Slice.impl__copy_from_slice #u8 k shared_secret in
    Core_models.Result.Result_Ok (k, c <: (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE))
    <:
    Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 1500"

/// Algorithm 18: ML-KEM.Decaps_internal
/// ```plaintext
/// Input: decapsulation key dk ∈ 𝔹^{768k+96}.
/// Input: ciphertext c ∈ 𝔹^{32(dᵤk+dᵥ)}.
/// Output: shared key K ∈ 𝔹³².
/// dkₚₖₑ ← dk[0 : 384k]
/// ekₚₖₑ ← dk[384k : 768k + 32]
/// h ← dk[768k + 32 : 768k + 64]
/// z ← dk[768k + 64 : 768k + 96]
/// m′ ← K-PKE.Decrypt(dkₚₖₑ, c)
/// (K′, r′) ← G(m′ ‖ h)
/// K\u{303} ← J(z ‖ c)
/// c′ ← K-PKE.Encrypt(ekₚₖₑ, m′, r′)
/// if c ≠ c′ then
///     K′ ← K\u{303}
/// end if
/// return K′
/// ```
let decaps_internal
      (v_RANK v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE v_U_SIZE v_V_SIZE v_CT_SIZE v_J_INPUT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (dk: t_Array u8 v_DK_SIZE)
      (c: t_Array u8 v_CT_SIZE)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        v_DK_PKE_SIZE =. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) &&
        v_DK_SIZE =.
        (((v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
            Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
            <:
            usize) +!
          mk_usize 32
          <:
          usize) &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
            params.Hacspec_ml_kem.Parameters.f_du
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
            params.Hacspec_ml_kem.Parameters.f_dv
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_CT_SIZE =. (v_U_SIZE +! v_V_SIZE <: usize) &&
        v_J_INPUT_SIZE =. (mk_usize 32 +! v_CT_SIZE <: usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let dk_pke:t_Slice u8 =
    dk.[ { Core_models.Ops.Range.f_end = v_DK_PKE_SIZE } <: Core_models.Ops.Range.t_RangeTo usize ]
  in
  let ek:t_Slice u8 =
    dk.[ {
        Core_models.Ops.Range.f_start = v_DK_PKE_SIZE;
        Core_models.Ops.Range.f_end = v_DK_PKE_SIZE +! v_EK_SIZE <: usize
      }
      <:
      Core_models.Ops.Range.t_Range usize ]
  in
  let h:t_Slice u8 =
    dk.[ {
        Core_models.Ops.Range.f_start = v_DK_PKE_SIZE +! v_EK_SIZE <: usize;
        Core_models.Ops.Range.f_end
        =
        (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
        Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
        <:
        usize
      }
      <:
      Core_models.Ops.Range.t_Range usize ]
  in
  let z:t_Slice u8 =
    dk.[ {
        Core_models.Ops.Range.f_start
        =
        (v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
        Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
        <:
        usize
      }
      <:
      Core_models.Ops.Range.t_RangeFrom usize ]
  in
  let m_prime:t_Array u8 (mk_usize 32) =
    Hacspec_ml_kem.Ind_cpa.decrypt v_RANK params dk_pke (c <: t_Slice u8)
  in
  let to_hash:t_Array u8 (mk_usize 64) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to to_hash
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (m_prime <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = mk_usize 32 } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          h
        <:
        t_Slice u8)
  in
  let hashed:t_Array u8 (mk_usize 64) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_G (to_hash <: t_Slice u8)
  in
  let (success_shared_secret: t_Slice u8), (pseudorandomness: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8 (hashed <: t_Slice u8) (mk_usize 32)
  in
  let (r_prime: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8)
          #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (pseudorandomness.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
        <:
        Core_models.Result.t_Result (t_Array u8 (mk_usize 32)) Core_models.Array.t_TryFromSliceError
      )
  in
  let j_input:t_Array u8 v_J_INPUT_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_J_INPUT_SIZE in
  let j_input:t_Array u8 v_J_INPUT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to j_input
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (j_input.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          z
        <:
        t_Slice u8)
  in
  let j_input:t_Array u8 v_J_INPUT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from j_input
      ({ Core_models.Ops.Range.f_start = mk_usize 32 } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (j_input.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (c <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let (rejection_shared_secret: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_J (mk_usize 32) (j_input <: t_Slice u8)
  in
  match
    Hacspec_ml_kem.Ind_cpa.encrypt v_RANK
      v_U_SIZE
      v_V_SIZE
      v_CT_SIZE
      params
      ek
      m_prime
      (r_prime <: t_Slice u8)
    <:
    Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok c_prime ->
    if
      (c.[ Core_models.Ops.Range.RangeFull <: Core_models.Ops.Range.t_RangeFull ] <: t_Slice u8) =.
      (c_prime.[ Core_models.Ops.Range.RangeFull <: Core_models.Ops.Range.t_RangeFull ]
        <:
        t_Slice u8)
    then
      let k:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
      let k:t_Array u8 (mk_usize 32) =
        Core_models.Slice.impl__copy_from_slice #u8 k success_shared_secret
      in
      Core_models.Result.Result_Ok k
      <:
      Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
        Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
    else
      Core_models.Result.Result_Ok rejection_shared_secret
      <:
      Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
        Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 1500"

/// Algorithm 19: ML-KEM.KeyGen
/// Generates an encapsulation key and a corresponding decapsulation key.
let generate_keypair
      (v_RANK v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (randomness: t_Array u8 (mk_usize 64))
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_SIZE)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        v_DK_PKE_SIZE =. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) &&
        v_DK_SIZE =.
        (((v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
            Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
            <:
            usize) +!
          mk_usize 32
          <:
          usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let (d: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8)
          #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (randomness.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
        <:
        Core_models.Result.t_Result (t_Array u8 (mk_usize 32)) Core_models.Array.t_TryFromSliceError
      )
  in
  let (z: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8)
          #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (randomness.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
        <:
        Core_models.Result.t_Result (t_Array u8 (mk_usize 32)) Core_models.Array.t_TryFromSliceError
      )
  in
  keygen_internal v_RANK v_EK_SIZE v_DK_PKE_SIZE v_DK_SIZE params d z

#pop-options

#push-options "--z3rlimit 1500"

/// Modulus check for encapsulation key validation (FIPS 203 Section 7.2).
/// Verifies that ByteEncode₁₂(ByteDecode₁₂(ek[..384k])) == ek[..384k].
let public_key_modulus_check
      (v_EK_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (ek: t_Array u8 v_EK_SIZE)
    : Prims.Pure bool
      (requires
        params.Hacspec_ml_kem.Parameters.f_rank <=. mk_usize 4 &&
        v_EK_SIZE =.
        ((params.Hacspec_ml_kem.Parameters.f_rank *!
            Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT
            <:
            usize) +!
          mk_usize 32
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let tt_size:usize = Hacspec_ml_kem.Parameters.impl_MlKemParams__tt_as_ntt_encoded_size params in
  let encoded_ring_elements:t_Slice u8 =
    ek.[ { Core_models.Ops.Range.f_end = tt_size } <: Core_models.Ops.Range.t_RangeTo usize ]
  in
  let valid:bool = true in
  let valid:bool =
    Rust_primitives.Hax.Folds.fold_chunked_slice Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT
      encoded_ring_elements
      (fun valid temp_1_ ->
          let valid:bool = valid in
          let _:usize = temp_1_ in
          true)
      valid
      (fun valid chunk ->
          let valid:bool = valid in
          let chunk:t_Slice u8 = chunk in
          let decoded:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
            Hacspec_ml_kem.Serialize.byte_decode (mk_usize 384)
              (mk_usize 3072)
              (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 384))
                  #Core_models.Array.t_TryFromSliceError
                  (Core_models.Convert.f_try_into #(t_Slice u8)
                      #(t_Array u8 (mk_usize 384))
                      #FStar.Tactics.Typeclasses.solve
                      chunk
                    <:
                    Core_models.Result.t_Result (t_Array u8 (mk_usize 384))
                      Core_models.Array.t_TryFromSliceError)
                <:
                t_Array u8 (mk_usize 384))
              (mk_usize 12)
          in
          let re_encoded:t_Array u8 (mk_usize 384) =
            Hacspec_ml_kem.Serialize.byte_encode (mk_usize 384)
              (mk_usize 3072)
              decoded
              (mk_usize 12)
          in
          if
            chunk <>.
            (Core_models.Array.impl_23__as_slice #u8 (mk_usize 384) re_encoded <: t_Slice u8)
          then
            let valid:bool = false in
            valid
          else valid)
  in
  valid

#pop-options

#push-options "--z3rlimit 1500"

/// Algorithm 20: ML-KEM.Encaps
/// Uses the encapsulation key to generate a shared key and ciphertext.
/// Includes modulus check on ek per FIPS 203 Section 7.2.
let encapsulate
      (v_RANK v_EK_SIZE v_U_SIZE v_V_SIZE v_CT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (ek: t_Array u8 v_EK_SIZE)
      (m: t_Array u8 (mk_usize 32))
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
            params.Hacspec_ml_kem.Parameters.f_du
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
            params.Hacspec_ml_kem.Parameters.f_dv
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_CT_SIZE =. (v_U_SIZE +! v_V_SIZE <: usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let _:Prims.unit = () <: Prims.unit in
  encaps_internal v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE params (ek <: t_Slice u8) m

#pop-options

#push-options "--z3rlimit 1500"

/// Algorithm 21: ML-KEM.Decaps
/// Uses the decapsulation key to produce a shared key from a ciphertext.
let decapsulate
      (v_RANK v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE v_U_SIZE v_V_SIZE v_CT_SIZE v_J_INPUT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (dk: t_Array u8 v_DK_SIZE)
      (c: t_Array u8 v_CT_SIZE)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        v_DK_PKE_SIZE =. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) &&
        v_DK_SIZE =.
        (((v_DK_PKE_SIZE +! v_EK_SIZE <: usize) +!
            Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
            <:
            usize) +!
          mk_usize 32
          <:
          usize) &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
            params.Hacspec_ml_kem.Parameters.f_du
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
            params.Hacspec_ml_kem.Parameters.f_dv
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_CT_SIZE =. (v_U_SIZE +! v_V_SIZE <: usize) &&
        v_J_INPUT_SIZE =. (mk_usize 32 +! v_CT_SIZE <: usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  decaps_internal v_RANK v_EK_SIZE v_DK_SIZE v_DK_PKE_SIZE v_U_SIZE v_V_SIZE v_CT_SIZE
    v_J_INPUT_SIZE params dk c

#pop-options

#push-options "--z3rlimit 1500"

/// ML-KEM.KeyGen — unpacked variant.  Returns the components used by
/// `MlKemKeyPairUnpacked` in the libcrux impl, instead of the
/// serialized `(ek, dk)` pair.
let ind_cca_unpack_generate_keypair
      (v_RANK v_EK_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (randomness: t_Array u8 (mk_usize 64))
    : Prims.Pure
      (Core_models.Result.t_Result
          (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
            t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
            t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK &
            t_Array u8 (mk_usize 32) &
            t_Array u8 (mk_usize 32) &
            t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let (d: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8)
          #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (randomness.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
        <:
        Core_models.Result.t_Result (t_Array u8 (mk_usize 32)) Core_models.Array.t_TryFromSliceError
      )
  in
  let (z: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
      #Core_models.Array.t_TryFromSliceError
      (Core_models.Convert.f_try_into #(t_Slice u8)
          #(t_Array u8 (mk_usize 32))
          #FStar.Tactics.Typeclasses.solve
          (randomness.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
        <:
        Core_models.Result.t_Result (t_Array u8 (mk_usize 32)) Core_models.Array.t_TryFromSliceError
      )
  in
  match
    Hacspec_ml_kem.Ind_cpa.generate_keypair_unpacked v_RANK params (d <: t_Slice u8)
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok (secret_as_ntt, tt_as_ntt, v_A_as_ntt, seed_for_A) ->
    let
    (m_A:
      t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
        v_RANK):t_Array
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK) v_RANK =
      Hacspec_ml_kem.Matrix.transpose v_RANK v_A_as_ntt
    in
    let (ek: t_Array u8 v_EK_SIZE):t_Array u8 v_EK_SIZE =
      Hacspec_ml_kem.Serialize.serialize_public_key v_RANK
        v_EK_SIZE
        tt_as_ntt
        (seed_for_A <: t_Slice u8)
    in
    let (public_key_hash: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
      Hacspec_ml_kem.Parameters.Hash_functions.v_H (ek <: t_Slice u8)
    in
    let implicit_rejection_value:t_Array u8 (mk_usize 32) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32)
    in
    let implicit_rejection_value:t_Array u8 (mk_usize 32) =
      Core_models.Slice.impl__copy_from_slice #u8 implicit_rejection_value (z <: t_Slice u8)
    in
    Core_models.Result.Result_Ok
    (secret_as_ntt, tt_as_ntt, m_A, seed_for_A, public_key_hash, implicit_rejection_value
      <:
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32) &
        t_Array u8 (mk_usize 32) &
        t_Array u8 (mk_usize 32)))
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32) &
        t_Array u8 (mk_usize 32) &
        t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32) &
        t_Array u8 (mk_usize 32) &
        t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 1500"

/// ML-KEM.Encaps — unpacked variant.  Skips the `H(ek)` and
/// `ByteDecode₁₂(ek)` decoding steps; consumes the precomputed
/// `public_key_hash`, `t_as_ntt`, and `m_A` (libcrux-transposed form)
/// directly.
let ind_cca_unpack_encapsulate
      (v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (public_key_hash: t_Array u8 (mk_usize 32))
      (tt_as_ntt: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (m_A:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
      (randomness: t_Array u8 (mk_usize 32))
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
            params.Hacspec_ml_kem.Parameters.f_du
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
            params.Hacspec_ml_kem.Parameters.f_dv
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_CT_SIZE =. (v_U_SIZE +! v_V_SIZE <: usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let to_hash:t_Array u8 (mk_usize 64) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to to_hash
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (randomness <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = mk_usize 32 } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (public_key_hash <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let hashed:t_Array u8 (mk_usize 64) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_G (to_hash <: t_Slice u8)
  in
  let (shared_secret: t_Slice u8), (pseudorandomness: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8 (hashed <: t_Slice u8) (mk_usize 32)
  in
  let
  (v_A_as_ntt:
    t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK):t_Array
    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK) v_RANK =
    Hacspec_ml_kem.Matrix.transpose v_RANK m_A
  in
  match
    Hacspec_ml_kem.Ind_cpa.encrypt_unpacked v_RANK
      v_U_SIZE
      v_V_SIZE
      v_CT_SIZE
      params
      tt_as_ntt
      v_A_as_ntt
      randomness
      pseudorandomness
    <:
    Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok c ->
    let k:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
    let k:t_Array u8 (mk_usize 32) = Core_models.Slice.impl__copy_from_slice #u8 k shared_secret in
    Core_models.Result.Result_Ok (k, c <: (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE))
    <:
    Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 (mk_usize 32) & t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 1500"

/// ML-KEM.Decaps — unpacked variant.  Skips the `dk` deconstruction
/// (`dkₚₖₑ`, `ekₚₖₑ`, `h`, `z`) and the `ByteDecode₁₂(dkₚₖₑ)` step;
/// consumes the components precomputed by `ind_cca_unpack_generate_keypair`
/// directly.  Re-encrypts via `ind_cpa::encrypt_unpacked` to check the
/// FO transform\'s c == c′ condition.
let ind_cca_unpack_decapsulate
      (v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE v_J_INPUT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (public_key_hash implicit_rejection_value: t_Array u8 (mk_usize 32))
      (ciphertext: t_Array u8 v_CT_SIZE)
      (secret_as_ntt tt_as_ntt:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (m_A:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
            params.Hacspec_ml_kem.Parameters.f_du
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
            params.Hacspec_ml_kem.Parameters.f_dv
            <:
            usize) /!
          mk_usize 8
          <:
          usize) &&
        v_CT_SIZE =. (v_U_SIZE +! v_V_SIZE <: usize) &&
        v_J_INPUT_SIZE =. (mk_usize 32 +! v_CT_SIZE <: usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3))
      (fun _ -> Prims.l_True) =
  let m_prime:t_Array u8 (mk_usize 32) =
    Hacspec_ml_kem.Ind_cpa.decrypt_unpacked v_RANK params secret_as_ntt (ciphertext <: t_Slice u8)
  in
  let to_hash:t_Array u8 (mk_usize 64) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to to_hash
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (m_prime <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = mk_usize 32 } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (public_key_hash <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let hashed:t_Array u8 (mk_usize 64) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_G (to_hash <: t_Slice u8)
  in
  let (success_shared_secret: t_Slice u8), (pseudorandomness: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8 (hashed <: t_Slice u8) (mk_usize 32)
  in
  let j_input:t_Array u8 v_J_INPUT_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_J_INPUT_SIZE in
  let j_input:t_Array u8 v_J_INPUT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to j_input
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (j_input.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (implicit_rejection_value <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let j_input:t_Array u8 v_J_INPUT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from j_input
      ({ Core_models.Ops.Range.f_start = mk_usize 32 } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (j_input.[ { Core_models.Ops.Range.f_start = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (ciphertext <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let (rejection_shared_secret: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_J (mk_usize 32) (j_input <: t_Slice u8)
  in
  let
  (v_A_as_ntt:
    t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK):t_Array
    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK) v_RANK =
    Hacspec_ml_kem.Matrix.transpose v_RANK m_A
  in
  match
    Hacspec_ml_kem.Ind_cpa.encrypt_unpacked v_RANK
      v_U_SIZE
      v_V_SIZE
      v_CT_SIZE
      params
      tt_as_ntt
      v_A_as_ntt
      m_prime
      pseudorandomness
    <:
    Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok c_prime ->
    if
      (ciphertext.[ Core_models.Ops.Range.RangeFull <: Core_models.Ops.Range.t_RangeFull ]
        <:
        t_Slice u8) =.
      (c_prime.[ Core_models.Ops.Range.RangeFull <: Core_models.Ops.Range.t_RangeFull ]
        <:
        t_Slice u8)
    then
      let k:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
      let k:t_Array u8 (mk_usize 32) =
        Core_models.Slice.impl__copy_from_slice #u8 k success_shared_secret
      in
      Core_models.Result.Result_Ok k
      <:
      Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
        Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
    else
      Core_models.Result.Result_Ok rejection_shared_secret
      <:
      Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
        Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options
