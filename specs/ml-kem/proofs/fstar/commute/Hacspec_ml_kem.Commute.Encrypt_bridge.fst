module Hacspec_ml_kem.Commute.Encrypt_bridge
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P  = Hacspec_ml_kem.Parameters
module HI = Hacspec_ml_kem.Ind_cpa
module HM = Hacspec_ml_kem.Matrix
module HS = Hacspec_ml_kem.Serialize
module U  = Rust_primitives.Hax.Monomorphized_update_at
module R  = Core_models.Ops.Range

(* ════════════════════════════════════════════════════════════════════════
   ENCRYPT composition bridge for Libcrux_ml_kem.Ind_cpa.encrypt_unpacked.

   The impl assembles `c1 ‖ c2`: encrypt_c1 writes ciphertext[0..C1_LEN]
   (= compress_then_serialize_u of compute_vector_u …), encrypt_c2 writes
   ciphertext[C1_LEN..] (= compress_then_serialize_v of compute_ring_element_v …).
   The spec `Hacspec.encrypt_unpacked` builds the SAME two byte-segments and
   concatenates.  `lemma_encrypt_unpacked_finalize` discharges `result == Ok-value`
   from the encrypt_c1/encrypt_c2 functional posts (supplied by the caller).

   Matrix convention: the impl computes `compute_vector_u (matrix_to_spec A)`
   (row-wise), and `build_unpacked_public_key`/`Hacspec.encrypt` feed
   `sample_matrix_A(..,false) == matrix_to_spec A` straight in — so NO transpose.
   ════════════════════════════════════════════════════════════════════════ *)

type fe = P.t_FieldElement
type poly = t_Array fe (mk_usize 256)

(* ── rank_to_params field projections ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 80"
let lemma_rank_to_params (v_K: usize)
  : Lemma (requires P.is_rank v_K)
          (ensures (let p = P.rank_to_params v_K in
                    p.P.f_rank == v_K /\
                    p.P.f_eta1 == P.eta1 v_K /\
                    p.P.f_eta2 == P.eta2 v_K /\
                    p.P.f_du == P.vector_u_compression_factor v_K /\
                    p.P.f_dv == P.vector_v_compression_factor v_K))
  = ()
#pop-options

(* ── prf_input bridge: the impl's `into_padded_array`-based 33-byte PRF input
   (exposed by encrypt_c1's error_2 post) equals the spec's
   `repeat 0 ‖ copy ‖ update_at 32` construction.  Both denote randomness ‖ [d]. ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 60"
let lemma_encrypt_prf_input (randomness: t_Slice u8) (d: u8)
  : Lemma (requires Seq.length randomness == 32)
          (ensures
            U.update_at_usize (Libcrux_ml_kem.Utils.into_padded_array (mk_usize 33) randomness) (mk_usize 32) d
            ==
            U.update_at_usize
              (U.update_at_range_to (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33))
                  ({ R.f_end = mk_usize 32 } <: R.t_RangeTo usize)
                  (Core_models.Slice.impl__copy_from_slice #u8
                      ((Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33)).[ { R.f_end = mk_usize 32 } <: R.t_RangeTo usize ] <: t_Slice u8)
                      randomness))
              (mk_usize 32) d)
  = let base = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 33) in
    let lhs0 = Libcrux_ml_kem.Utils.into_padded_array (mk_usize 33) randomness in
    let cpy = Core_models.Slice.impl__copy_from_slice #u8
                (base.[ { R.f_end = mk_usize 32 } <: R.t_RangeTo usize ] <: t_Slice u8) randomness in
    let rhs0 = U.update_at_range_to base ({ R.f_end = mk_usize 32 } <: R.t_RangeTo usize) cpy in
    let target = Seq.append randomness (Seq.create 1 (mk_u8 0)) in
    assert (lhs0 == target);
    assert (cpy == randomness);
    assert (Seq.slice rhs0 0 32 == cpy);
    assert (Seq.slice rhs0 32 33 == Seq.slice base 32 33);
    let aux (i:nat{i<33}) : Lemma (Seq.index rhs0 i == Seq.index target i) =
      if i < 32 then (Seq.lemma_index_slice rhs0 0 32 i; Seq.lemma_index_slice target 0 32 i)
      else Seq.lemma_index_slice rhs0 32 33 (i-32)
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro rhs0 target
#pop-options

(* ── impl-side concat: result == c1_bytes ‖ c2_bytes ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_impl_concat (v_CT_SIZE v_C1_LEN: usize) (c1_bytes c2_bytes: t_Slice u8)
  : Lemma (requires
            v v_C1_LEN <= v v_CT_SIZE /\
            Seq.length c1_bytes == v v_C1_LEN /\
            Seq.length c2_bytes == v v_CT_SIZE - v v_C1_LEN)
          (ensures
            U.update_at_range_from
              (U.update_at_range (Rust_primitives.Hax.repeat (mk_u8 0) v_CT_SIZE)
                 ({ R.f_start = mk_usize 0; R.f_end = v_C1_LEN } <: R.t_Range usize) c1_bytes)
              ({ R.f_start = v_C1_LEN } <: R.t_RangeFrom usize) c2_bytes
            == Seq.append c1_bytes c2_bytes)
  = let r0 = Rust_primitives.Hax.repeat (mk_u8 0) v_CT_SIZE in
    let r1 = U.update_at_range r0 ({ R.f_start = mk_usize 0; R.f_end = v_C1_LEN } <: R.t_Range usize) c1_bytes in
    let r2 = U.update_at_range_from r1 ({ R.f_start = v_C1_LEN } <: R.t_RangeFrom usize) c2_bytes in
    assert (Seq.slice r1 (v (mk_usize 0)) (v v_C1_LEN) == c1_bytes);
    assert (Seq.slice r2 0 (v v_C1_LEN) == Seq.slice r1 0 (v v_C1_LEN));
    assert (Seq.slice r2 0 (v v_C1_LEN) == c1_bytes);
    assert (Seq.slice r2 (v v_C1_LEN) (Seq.length r2) == c2_bytes);
    Rust_primitives.Arrays.lemma_slice_append r2 c1_bytes c2_bytes
#pop-options

(* ── spec-side concat: the spec's c == c1_spec ‖ c2_spec ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_spec_concat (v_CT_SIZE v_U_SIZE: usize) (c1_spec c2_spec: t_Slice u8)
  : Lemma (requires
            v v_U_SIZE <= v v_CT_SIZE /\
            Seq.length c1_spec == v v_U_SIZE /\
            Seq.length c2_spec == v v_CT_SIZE - v v_U_SIZE)
          (ensures
            (let c0 = Rust_primitives.Hax.repeat (mk_u8 0) v_CT_SIZE in
             let c1 = U.update_at_range_to c0 ({ R.f_end = v_U_SIZE } <: R.t_RangeTo usize)
                        (Core_models.Slice.impl__copy_from_slice #u8
                          (c0.[ { R.f_end = v_U_SIZE } <: R.t_RangeTo usize ] <: t_Slice u8) c1_spec) in
             let c2 = U.update_at_range_from c1 ({ R.f_start = v_U_SIZE } <: R.t_RangeFrom usize)
                        (Core_models.Slice.impl__copy_from_slice #u8
                          (c1.[ { R.f_start = v_U_SIZE } <: R.t_RangeFrom usize ] <: t_Slice u8) c2_spec) in
             c2 == Seq.append c1_spec c2_spec))
  = let c0 = Rust_primitives.Hax.repeat (mk_u8 0) v_CT_SIZE in
    let cp1 = Core_models.Slice.impl__copy_from_slice #u8
                (c0.[ { R.f_end = v_U_SIZE } <: R.t_RangeTo usize ] <: t_Slice u8) c1_spec in
    let c1 = U.update_at_range_to c0 ({ R.f_end = v_U_SIZE } <: R.t_RangeTo usize) cp1 in
    let cp2 = Core_models.Slice.impl__copy_from_slice #u8
                (c1.[ { R.f_start = v_U_SIZE } <: R.t_RangeFrom usize ] <: t_Slice u8) c2_spec in
    let c2 = U.update_at_range_from c1 ({ R.f_start = v_U_SIZE } <: R.t_RangeFrom usize) cp2 in
    assert (cp1 == c1_spec);
    assert (cp2 == c2_spec);
    assert (Seq.slice c1 0 (v v_U_SIZE) == cp1);
    assert (Seq.slice c2 0 (v v_U_SIZE) == Seq.slice c1 0 (v v_U_SIZE));
    assert (Seq.slice c2 0 (v v_U_SIZE) == c1_spec);
    assert (Seq.slice c2 (v v_U_SIZE) (Seq.length c2) == c2_spec);
    Rust_primitives.Arrays.lemma_slice_append c2 c1_spec c2_spec
#pop-options

(* ── the finalize: result == Ok-value of the spec ──
   `result` is the impl's final ciphertext; its two byte-segments are pinned to
   the encrypt_c1/encrypt_c2 functional posts (as `Seq.slice` facts, so the
   caller need not name the hax-internal in-place `&mut` tmp binders). ── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_encrypt_unpacked_finalize
      (v_K v_C1_LEN v_C2_LEN v_CT_SIZE v_U_COMP v_V_COMP: usize)
      (tt_spec: t_Array poly v_K)
      (a_spec: t_Array (t_Array poly v_K) v_K)
      (message: t_Array u8 (mk_usize 32))
      (randomness: t_Slice u8)
      (result: t_Array u8 v_CT_SIZE)
    : Lemma
      (requires
        P.is_rank v_K /\
        v_C1_LEN == P.c1_size v_K /\ v_C2_LEN == P.c2_size v_K /\
        v_CT_SIZE == P.cpa_ciphertext_size v_K /\
        v_U_COMP == P.vector_u_compression_factor v_K /\
        v_V_COMP == P.vector_v_compression_factor v_K /\
        Seq.length randomness == 32 /\
        v v_C1_LEN + v v_C2_LEN == v v_CT_SIZE /\
        Seq.slice result 0 (v v_C1_LEN) ==
          HS.compress_then_serialize_u v_K v_C1_LEN
            (HM.compute_vector_u v_K a_spec
               (HI.sample_vector_cbd_then_ntt v_K (P.eta1 v_K) randomness (mk_u8 0))
               (HI.sample_vector_cbd v_K (P.eta2 v_K) randomness (cast v_K <: u8)))
            v_U_COMP /\
        Seq.slice result (v v_C1_LEN) (v v_CT_SIZE) ==
          HS.compress_then_serialize_v v_C2_LEN
            (HM.compute_ring_element_v v_K tt_spec
               (HI.sample_vector_cbd_then_ntt v_K (P.eta1 v_K) randomness (mk_u8 0))
               (HI.sample_secret (P.eta2 v_K)
                  (U.update_at_usize (Libcrux_ml_kem.Utils.into_padded_array (mk_usize 33) randomness)
                     (mk_usize 32) (cast (v_K *! mk_usize 2) <: u8)))
               (HS.deserialize_then_decompress_message message))
            v_V_COMP)
      (ensures
        (match HI.encrypt_unpacked v_K v_C1_LEN v_C2_LEN v_CT_SIZE (P.rank_to_params v_K)
                 tt_spec a_spec message randomness with
         | Core_models.Result.Result_Ok ct -> result == ct
         | Core_models.Result.Result_Err _ -> true))
  = lemma_rank_to_params v_K;
    lemma_encrypt_prf_input randomness (cast (v_K *! mk_usize 2) <: u8);
    let c1_spec = Seq.slice result 0 (v v_C1_LEN) in
    let c2_spec = Seq.slice result (v v_C1_LEN) (v v_CT_SIZE) in
    Rust_primitives.Arrays.lemma_slice_append result c1_spec c2_spec;
    lemma_spec_concat v_CT_SIZE v_C1_LEN c1_spec c2_spec
#pop-options
