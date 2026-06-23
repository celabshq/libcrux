module Hacspec_ml_kem.Ind_cpa
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

#push-options "--z3rlimit 150"

/// Helper to sample a polynomial from CBD with dynamic eta.
let sample_secret (eta: usize) (prf_input: t_Array u8 (mk_usize 33))
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires eta =. mk_usize 2 || eta =. mk_usize 3)
      (fun _ -> Prims.l_True) =
  match eta <: usize with
  | Rust_primitives.Integers.MkInt 2 ->
    let (out: t_Array u8 (mk_usize 128)):t_Array u8 (mk_usize 128) =
      Hacspec_ml_kem.Parameters.Hash_functions.v_PRF (mk_usize 128) (prf_input <: t_Slice u8)
    in
    Hacspec_ml_kem.Sampling.sample_poly_cbd (mk_usize 128) (mk_usize 1024) (mk_usize 2) out
  | Rust_primitives.Integers.MkInt 3 ->
    let (out: t_Array u8 (mk_usize 192)):t_Array u8 (mk_usize 192) =
      Hacspec_ml_kem.Parameters.Hash_functions.v_PRF (mk_usize 192) (prf_input <: t_Slice u8)
    in
    Hacspec_ml_kem.Sampling.sample_poly_cbd (mk_usize 192) (mk_usize 1536) (mk_usize 3) out
  | _ ->
    let args:usize = eta <: usize in
    let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
      let list = [Core_models.Fmt.Rt.impl__new_display #usize args] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
      Rust_primitives.Hax.array_of_list 1 list
    in
    Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic_fmt (Core_models.Fmt.Rt.impl_1__new_v1
              (mk_usize 1)
              (mk_usize 1)
              (let list = ["unsupported eta="] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
              args
            <:
            Core_models.Fmt.t_Arguments)
        <:
        Rust_primitives.Hax.t_Never)

#pop-options

let concat_byte (v_N v_N1: usize) (a: t_Array u8 v_N) (b: u8)
    : Prims.Pure (t_Array u8 v_N1)
      (requires v_N1 >. mk_usize 0 && v_N =. (v_N1 -! mk_usize 1 <: usize))
      (fun _ -> Prims.l_True) =
  let result:t_Array u8 v_N1 = Rust_primitives.Hax.repeat (mk_u8 0) v_N1 in
  let result:t_Array u8 v_N1 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to result
      ({ Core_models.Ops.Range.f_end = v_N } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (result.[ { Core_models.Ops.Range.f_end = v_N } <: Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (a <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let result:t_Array u8 v_N1 =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize result v_N b
  in
  result

/// FIPS 203 inner loop: sample `RANK` polynomials from CBD_η using
/// PRF_η(seed ‖ domain_separator + i) for i ∈ {0, …, RANK-1}.
/// Captures the \"for i ∈ {0, …, k-1}: v[i] ← SamplePolyCBD_η(PRF_η(seed, N))\"
/// pattern that appears in K-PKE.KeyGen (Alg. 13) for s/e and in
/// K-PKE.Encrypt (Alg. 14) for r/e₁.
let sample_vector_cbd (v_RANK eta: usize) (seed: t_Slice u8) (domain_separator: u8)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (requires
        (Core_models.Slice.impl__len #u8 seed <: usize) =. mk_usize 32 &&
        (eta =. mk_usize 2 || eta =. mk_usize 3) &&
        v_RANK <=. mk_usize 4 &&
        ((cast (domain_separator <: u8) <: usize) +! v_RANK <: usize) <. mk_usize 256)
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        let (prf_input: t_Array u8 (mk_usize 33)):t_Array u8 (mk_usize 33) =
          concat_byte (mk_usize 32)
            (mk_usize 33)
            (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
                #Core_models.Array.t_TryFromSliceError
                (Core_models.Convert.f_try_into #(t_Slice u8)
                    #(t_Array u8 (mk_usize 32))
                    #FStar.Tactics.Typeclasses.solve
                    seed
                  <:
                  Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
                    Core_models.Array.t_TryFromSliceError)
              <:
              t_Array u8 (mk_usize 32))
            (domain_separator +! (cast (i <: usize) <: u8) <: u8)
        in
        sample_secret eta prf_input)

/// `sample_vector_cbd` followed by NTT.  Captures the \"ŝ ← NTT(s)\" /
/// \"ê ← NTT(e)\" / \"r\u{302} ← NTT(r)\" steps in K-PKE.{KeyGen,Encrypt}.
let sample_vector_cbd_then_ntt (v_RANK eta: usize) (seed: t_Slice u8) (domain_separator: u8)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (requires
        (Core_models.Slice.impl__len #u8 seed <: usize) =. mk_usize 32 &&
        (eta =. mk_usize 2 || eta =. mk_usize 3) &&
        v_RANK <=. mk_usize 4 &&
        ((cast (domain_separator <: u8) <: usize) +! v_RANK <: usize) <. mk_usize 256)
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Ntt.vector_ntt v_RANK
    (sample_vector_cbd v_RANK eta seed domain_separator
      <:
      t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)

#push-options "--z3rlimit 150"

/// Algorithm 13: K-PKE.KeyGen — unpacked variant.  Returns the four
/// components separately instead of the serialized `(ek, dk)` byte
/// pair.  The packed `generate_keypair` is a thin serialization
/// wrapper around this function.
let generate_keypair_unpacked
      (v_RANK: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (key_generation_seed: t_Slice u8)
    : Prims.Pure
      (Core_models.Result.t_Result
          (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
            t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
            t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK &
            t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (Core_models.Slice.impl__len #u8 key_generation_seed <: usize) =. mk_usize 32)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let g_input:t_Array u8 (mk_usize 33) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33) in
  let g_input:t_Array u8 (mk_usize 33) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to g_input
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (g_input.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          key_generation_seed
        <:
        t_Slice u8)
  in
  let g_input:t_Array u8 (mk_usize 33) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize g_input
      (mk_usize 32)
      (cast (v_RANK <: usize) <: u8)
  in
  let hashed:t_Array u8 (mk_usize 64) =
    Hacspec_ml_kem.Parameters.Hash_functions.v_G (g_input <: t_Slice u8)
  in
  let (seed_for_A_slice: t_Slice u8), (seed_for_secret_and_error: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8 (hashed <: t_Slice u8) (mk_usize 32)
  in
  match
    Hacspec_ml_kem.Matrix.sample_matrix_A v_RANK seed_for_A_slice false
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok
    (v_A_as_ntt:
      t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
        v_RANK) ->
    let secret_as_ntt:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      v_RANK =
      sample_vector_cbd_then_ntt v_RANK
        params.Hacspec_ml_kem.Parameters.f_eta1
        seed_for_secret_and_error
        (mk_u8 0)
    in
    let error_as_ntt:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      v_RANK =
      sample_vector_cbd_then_ntt v_RANK
        params.Hacspec_ml_kem.Parameters.f_eta1
        seed_for_secret_and_error
        (cast (v_RANK <: usize) <: u8)
    in
    let tt_as_ntt:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
      Hacspec_ml_kem.Matrix.compute_As_plus_e v_RANK v_A_as_ntt secret_as_ntt error_as_ntt
    in
    let seed_for_A:t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
    let seed_for_A:t_Array u8 (mk_usize 32) =
      Core_models.Slice.impl__copy_from_slice #u8 seed_for_A seed_for_A_slice
    in
    Core_models.Result.Result_Ok
    (secret_as_ntt, tt_as_ntt, v_A_as_ntt, seed_for_A
      <:
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32)))
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 150"

/// Algorithm 13: K-PKE.KeyGen
/// Generates an encryption key and a corresponding decryption key.
/// ```plaintext
/// Output: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Output: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
/// d ←$ B
/// (ρ,σ) ← G(d)
/// N ← 0
/// for (i ← 0; i < k; i++)
///     for(j ← 0; j < k; j++)
///         Â[i,j] ← SampleNTT(XOF(ρ, j, i))
///     end for
/// end for
/// for(i ← 0; i < k; i++)
///     s[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
///     N ← N + 1
/// end for
/// for(i ← 0; i < k; i++)
///     e[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
///     N ← N + 1
/// end for
/// ŝ ← NTT(s)
/// ê ← NTT(e)
/// t\u{302} ← Â◦ŝ + ê
/// ekₚₖₑ ← ByteEncode₁₂(t\u{302}) ‖ ρ
/// dkₚₖₑ ← ByteEncode₁₂(ŝ)
/// ```
let generate_keypair
      (v_RANK v_EK_SIZE v_DK_PKE_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (key_generation_seed: t_Slice u8)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_PKE_SIZE)
          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        v_DK_PKE_SIZE =. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) &&
        (params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 2 ||
        params.Hacspec_ml_kem.Parameters.f_eta1 =. mk_usize 3) &&
        (Core_models.Slice.impl__len #u8 key_generation_seed <: usize) =. mk_usize 32)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  match
    generate_keypair_unpacked v_RANK params key_generation_seed
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK &
        t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 32)) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok (secret_as_ntt, tt_as_ntt, e_A_as_ntt, seed_for_A) ->
    let (tt_encoded: t_Array u8 v_DK_PKE_SIZE):t_Array u8 v_DK_PKE_SIZE =
      Hacspec_ml_kem.Serialize.serialize_secret_key v_RANK v_DK_PKE_SIZE tt_as_ntt
    in
    let ek:t_Array u8 v_EK_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_EK_SIZE in
    let ek:t_Array u8 v_EK_SIZE =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to ek
        ({ Core_models.Ops.Range.f_end = v_DK_PKE_SIZE } <: Core_models.Ops.Range.t_RangeTo usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (ek.[ { Core_models.Ops.Range.f_end = v_DK_PKE_SIZE }
                <:
                Core_models.Ops.Range.t_RangeTo usize ]
              <:
              t_Slice u8)
            (tt_encoded <: t_Slice u8)
          <:
          t_Slice u8)
    in
    let ek:t_Array u8 v_EK_SIZE =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from ek
        ({ Core_models.Ops.Range.f_start = v_DK_PKE_SIZE }
          <:
          Core_models.Ops.Range.t_RangeFrom usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (ek.[ { Core_models.Ops.Range.f_start = v_DK_PKE_SIZE }
                <:
                Core_models.Ops.Range.t_RangeFrom usize ]
              <:
              t_Slice u8)
            (seed_for_A <: t_Slice u8)
          <:
          t_Slice u8)
    in
    let (dk: t_Array u8 v_DK_PKE_SIZE):t_Array u8 v_DK_PKE_SIZE =
      Hacspec_ml_kem.Serialize.serialize_secret_key v_RANK v_DK_PKE_SIZE secret_as_ntt
    in
    Core_models.Result.Result_Ok (ek, dk <: (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_PKE_SIZE))
    <:
    Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_PKE_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 v_EK_SIZE & t_Array u8 v_DK_PKE_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 150"

/// K-PKE.Encrypt — unpacked variant.  Skips the
/// `ByteDecode₁₂(ek)` / `sample_matrix_A(seed_for_A)` decoding step
/// and consumes the already-decoded `t_as_ntt` and `A_as_ntt`
/// directly.  The packed `encrypt` is a thin decoding wrapper.
let encrypt_unpacked
      (v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (tt_as_ntt: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (v_A_as_ntt:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
      (message: t_Array u8 (mk_usize 32))
      (randomness: t_Slice u8)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
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
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3) &&
        (Core_models.Slice.impl__len #u8 randomness <: usize) =. mk_usize 32)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let r_as_ntt:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    sample_vector_cbd_then_ntt v_RANK params.Hacspec_ml_kem.Parameters.f_eta1 randomness (mk_u8 0)
  in
  let error_1_:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    sample_vector_cbd v_RANK
      params.Hacspec_ml_kem.Parameters.f_eta2
      randomness
      (cast (v_RANK <: usize) <: u8)
  in
  let prf_input:t_Array u8 (mk_usize 33) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33) in
  let prf_input:t_Array u8 (mk_usize 33) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to prf_input
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (prf_input.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          randomness
        <:
        t_Slice u8)
  in
  let prf_input:t_Array u8 (mk_usize 33) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize prf_input
      (mk_usize 32)
      (cast (v_RANK *! mk_usize 2 <: usize) <: u8)
  in
  let error_2_:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    sample_secret params.Hacspec_ml_kem.Parameters.f_eta2 prf_input
  in
  let u:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    Hacspec_ml_kem.Matrix.compute_vector_u v_RANK v_A_as_ntt r_as_ntt error_1_
  in
  let message_as_ring_element:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Hacspec_ml_kem.Serialize.deserialize_then_decompress_message message
  in
  let v:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Hacspec_ml_kem.Matrix.compute_ring_element_v v_RANK
      tt_as_ntt
      r_as_ntt
      error_2_
      message_as_ring_element
  in
  let (c1: t_Array u8 v_U_SIZE):t_Array u8 v_U_SIZE =
    Hacspec_ml_kem.Serialize.compress_then_serialize_u v_RANK
      v_U_SIZE
      u
      params.Hacspec_ml_kem.Parameters.f_du
  in
  let (c2: t_Array u8 v_V_SIZE):t_Array u8 v_V_SIZE =
    Hacspec_ml_kem.Serialize.compress_then_serialize_v v_V_SIZE
      v
      params.Hacspec_ml_kem.Parameters.f_dv
  in
  let c:t_Array u8 v_CT_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_CT_SIZE in
  let c:t_Array u8 v_CT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to c
      ({ Core_models.Ops.Range.f_end = v_U_SIZE } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (c.[ { Core_models.Ops.Range.f_end = v_U_SIZE } <: Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          (c1 <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let c:t_Array u8 v_CT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from c
      ({ Core_models.Ops.Range.f_start = v_U_SIZE } <: Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (c.[ { Core_models.Ops.Range.f_start = v_U_SIZE }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (c2 <: t_Slice u8)
        <:
        t_Slice u8)
  in
  Core_models.Result.Result_Ok c
  <:
  Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
    Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 150"

/// Algorithm 14: K-PKE.Encrypt
/// Uses the encryption key to encrypt a plaintext message using the randomness r.
/// ```plaintext
/// Input: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Input: message m ∈ 𝔹^{32}.
/// Input: encryption randomness r ∈ 𝔹^{32}.
/// Output: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
/// N ← 0
/// t\u{302} ← ByteDecode₁₂(ekₚₖₑ[0:384k])
/// ρ ← ekₚₖₑ[384k: 384k + 32]
/// for (i ← 0; i < k; i++)
///     for(j ← 0; j < k; j++)
///         Â[i,j] ← SampleNTT(XOF(ρ, j, i))
///     end for
/// end for
/// for(i ← 0; i < k; i++)
///     r[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(r,N))
///     N ← N + 1
/// end for
/// for(i ← 0; i < k; i++)
///     e₁[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
///     N ← N + 1
/// end for
/// e₂ ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
/// r\u{302} ← NTT(r)
/// u ← NTT⁻¹(Âᵀ ◦ r\u{302}) + e₁
/// μ ← Decompress₁(ByteDecode₁(m))
/// v ← NTT⁻¹(t\u{302}ᵀ ◦ r\u{302}) + e₂ + μ
/// c₁ ← ByteEncode_{dᵤ}(Compress_{dᵤ}(u))
/// c₂ ← ByteEncode_{dᵥ}(Compress_{dᵥ}(v))
/// return c ← (c₁ ‖ c₂)
/// ```
let encrypt
      (v_RANK v_U_SIZE v_V_SIZE v_CT_SIZE: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (ek: t_Slice u8)
      (message: t_Array u8 (mk_usize 32))
      (randomness: t_Slice u8)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
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
        params.Hacspec_ml_kem.Parameters.f_eta2 =. mk_usize 3) &&
        (Core_models.Slice.impl__len #u8 randomness <: usize) =. mk_usize 32)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let tt_encoded_size:usize =
    Hacspec_ml_kem.Parameters.impl_MlKemParams__tt_as_ntt_encoded_size params
  in
  let (tt_as_ntt: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK):t_Array
    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    Hacspec_ml_kem.Serialize.deserialize_ring_elements_reduced v_RANK
      (ek.[ { Core_models.Ops.Range.f_end = tt_encoded_size }
          <:
          Core_models.Ops.Range.t_RangeTo usize ]
        <:
        t_Slice u8)
  in
  let seed_for_A:t_Slice u8 =
    ek.[ { Core_models.Ops.Range.f_start = tt_encoded_size }
      <:
      Core_models.Ops.Range.t_RangeFrom usize ]
  in
  match
    Hacspec_ml_kem.Matrix.sample_matrix_A v_RANK seed_for_A false
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
  with
  | Core_models.Result.Result_Ok
    (v_A_as_ntt:
      t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
        v_RANK) ->
    encrypt_unpacked v_RANK
      v_U_SIZE
      v_V_SIZE
      v_CT_SIZE
      params
      tt_as_ntt
      v_A_as_ntt
      message
      randomness
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u8 v_CT_SIZE)
      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

#pop-options

#push-options "--z3rlimit 150"

/// K-PKE.Decrypt — unpacked variant.  Skips the
/// `ByteDecode₁₂(dk)` decoding step and consumes the already-decoded
/// `secret_as_ntt` directly.  The packed `decrypt` is a thin
/// decoding wrapper.
let decrypt_unpacked
      (v_RANK: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (secret_as_ntt:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (ciphertext: t_Slice u8)
    : Prims.Pure (t_Array u8 (mk_usize 32))
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        (Core_models.Slice.impl__len #u8 ciphertext <: usize) =.
        ((((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
              params.Hacspec_ml_kem.Parameters.f_du
              <:
              usize) +!
            (Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
              params.Hacspec_ml_kem.Parameters.f_dv
              <:
              usize)
            <:
            usize) /!
          mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let u_encoded_size:usize = Hacspec_ml_kem.Parameters.impl_MlKemParams__u_encoded_size params in
  let (u: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK):t_Array
    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    Hacspec_ml_kem.Serialize.deserialize_then_decompress_u v_RANK
      (ciphertext.[ {
            Core_models.Ops.Range.f_start = mk_usize 0;
            Core_models.Ops.Range.f_end = u_encoded_size
          }
          <:
          Core_models.Ops.Range.t_Range usize ]
        <:
        t_Slice u8)
      params.Hacspec_ml_kem.Parameters.f_du
  in
  let v:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Hacspec_ml_kem.Serialize.deserialize_then_decompress_v (ciphertext.[ {
            Core_models.Ops.Range.f_start = u_encoded_size
          }
          <:
          Core_models.Ops.Range.t_RangeFrom usize ]
        <:
        t_Slice u8)
      params.Hacspec_ml_kem.Parameters.f_dv
  in
  let u_as_ntt:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    Hacspec_ml_kem.Ntt.vector_ntt v_RANK u
  in
  let w:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Hacspec_ml_kem.Matrix.compute_message v_RANK v secret_as_ntt u_as_ntt
  in
  Hacspec_ml_kem.Serialize.compress_then_serialize_message w

#pop-options

#push-options "--z3rlimit 150"

/// Algorithm 15: K-PKE.Decrypt
/// Uses the decryption key to decrypt a ciphertext.
/// ```plaintext
/// Input: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
/// Input: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
/// Output: message m ∈ 𝔹^{32}.
/// c₁ ← c[0 : 32dᵤk]
/// c₂ ← c[32dᵤk : 32(dᵤk + dᵥ)]
/// u ← Decompress_{dᵤ}(ByteDecode_{dᵤ}(c₁))
/// v ← Decompress_{dᵥ}(ByteDecode_{dᵥ}(c₂))
/// ŝ ← ByteDecode₁₂(dkₚₖₑ)
/// w ← v - NTT⁻¹(ŝᵀ ◦ NTT(u))
/// m ← ByteEncode₁(Compress₁(w))
/// return m
/// ```
let decrypt
      (v_RANK: usize)
      (params: Hacspec_ml_kem.Parameters.t_MlKemParams)
      (dk ciphertext: t_Slice u8)
    : Prims.Pure (t_Array u8 (mk_usize 32))
      (requires
        v_RANK <=. mk_usize 4 && params.Hacspec_ml_kem.Parameters.f_rank =. v_RANK &&
        (Core_models.Slice.impl__len #u8 dk <: usize) =.
        (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) &&
        (Core_models.Slice.impl__len #u8 ciphertext <: usize) =.
        ((((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *!
              params.Hacspec_ml_kem.Parameters.f_du
              <:
              usize) +!
            (Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
              params.Hacspec_ml_kem.Parameters.f_dv
              <:
              usize)
            <:
            usize) /!
          mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let
  (secret_as_ntt: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK):t_Array
    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    Hacspec_ml_kem.Serialize.deserialize_ring_elements_reduced v_RANK dk
  in
  decrypt_unpacked v_RANK params secret_as_ntt ciphertext

#pop-options
