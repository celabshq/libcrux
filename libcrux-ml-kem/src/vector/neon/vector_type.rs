use libcrux_intrinsics::arm64::*;
#[derive(Clone, Copy)]
#[hax_lib::fstar::before(interface, "noeq")]
#[hax_lib::fstar::after(
    interface,
    r#"let repr (x:t_SIMD128Vector) : t_Array i16 (sz 16) =
  Seq.append (Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 x.f_low)
             (Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 x.f_high)

val lemma_repr_index (x: t_SIMD128Vector) (j: nat{j < 16})
    : Lemma
      (Seq.index (repr x) j ==
        (if j < 8
         then Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 x.f_low j
         else Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 x.f_high (j - 8)))
      [SMTPat (Seq.index (repr x) j)]"#
)]
// This module's own repr / lemma_repr_index spec API: the val is in this module's
// interface (above), this is its body — a val's body cannot live in another module.
// proof-residence: spec-host — own repr / lemma_repr_index spec API
#[hax_lib::fstar::after(
    r#"let lemma_repr_index (x: t_SIMD128Vector) (j: nat{j < 16}) =
  let lo = Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 x.f_low in
  let hi = Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 x.f_high in
  if j < 8 then Seq.lemma_index_app1 lo hi j else Seq.lemma_index_app2 lo hi j"#
)]
pub struct SIMD128Vector {
    pub low: _int16x8_t,
    pub high: _int16x8_t,
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!("${result} == repr ${v}"))]
pub(crate) fn to_i16_array(v: SIMD128Vector) -> [i16; 16] {
    let mut out = [0i16; 16];
    _vst1q_s16(&mut out[0..8], v.low);
    _vst1q_s16(&mut out[8..16], v.high);
    // The two `update_at_range` posts are slice-equations; seed the
    // per-index slice/append lemmas so `lemma_eq_intro` can fire.
    proof!(
        r#"let lo = Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 ${v}.f_low in
let hi = Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 ${v}.f_high in
introduce forall (j: nat{j < 16}). Seq.index (${out} <: Seq.seq i16) j == Seq.index (repr ${v}) j
with begin
  if j < 8 then begin
    Seq.lemma_index_slice (${out} <: Seq.seq i16) 0 8 j;
    Seq.lemma_index_app1 lo hi j
  end else begin
    Seq.lemma_index_slice (${out} <: Seq.seq i16) 8 16 (j - 8);
    Seq.lemma_index_app2 lo hi j
  end
end;
Seq.lemma_eq_intro (${out} <: t_Slice i16) (repr ${v})"#
    );
    out
}

#[inline(always)]
#[hax_lib::requires(array.len() == 16)]
#[hax_lib::ensures(|result| fstar!("repr ${result} == $array"))]
pub(crate) fn from_i16_array(array: &[i16]) -> SIMD128Vector {
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    // Seed the per-index append/slice lemmas for `lemma_eq_intro`.
    proof!(
        r#"let lo = Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 ${result}.f_low in
let hi = Libcrux_intrinsics.Arm64_ml_kem_views.vec128_as_i16x8 ${result}.f_high in
introduce forall (j: nat{j < 16}). Seq.index (repr ${result}) j == Seq.index ${array} j
with begin
  if j < 8 then begin
    Seq.lemma_index_app1 lo hi j;
    Seq.lemma_index_slice ${array} 0 8 j
  end else begin
    Seq.lemma_index_app2 lo hi j;
    Seq.lemma_index_slice ${array} 8 16 (j - 8)
  end
end;
Seq.lemma_eq_intro (repr ${result}) ${array}"#
    );
    result
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300"))]
#[hax_lib::requires(bytes.len() >= 32)]
#[hax_lib::ensures(|_| fstar!(r#"
    Core_models.Slice.impl__len #u8 (bytes_future <: t_Slice u8) ==
      Core_models.Slice.impl__len #u8 ${bytes} /\
    (Core_models.Slice.impl__len #u8 ${bytes} >=. mk_usize 32 ==>
     (let head : t_Slice u8 = Seq.slice bytes_future 0 32 in
      Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N
        #(mk_usize 16) (repr ${v}) head))
"#))]
pub(crate) fn to_bytes(v: SIMD128Vector, bytes: &mut [u8]) {
    // Proof-only snapshots: the two stores rebind `bytes`, so without these the
    // store bridge has no way to name its pre-state operands (`ser0` before the
    // first store, `ser1` between the two — mirrors AVX2's `bytes_pre`).
    #[cfg(hax)]
    let ser0: &[u8] = &*bytes;
    _vst1q_bytes(&mut bytes[0..16], v.low);
    #[cfg(hax)]
    let ser1: &[u8] = &*bytes;
    _vst1q_bytes(&mut bytes[16..32], v.high);
    // The two `_vst1q_bytes` are LENGTH-ONLY over core-models, so re-supply the
    // per-half store bit-fact with `lemma_e_vst1q_bytes`, then thread the two
    // `update_at_range` frames with `lemma_store_glue_two_writes_neon`; the per-i
    // forall then only carries the i16x8 lane-bit decomposition (LHS view).
    proof!(
        r#"
let range0:Core_models.Ops.Range.t_Range usize =
  { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 16 } in
let range1:Core_models.Ops.Range.t_Range usize =
  { Core_models.Ops.Range.f_start = mk_usize 16; Core_models.Ops.Range.f_end = mk_usize 32 } in
let src_lo:t_Slice u8 = ${ser0}.[ range0 ] in
let src_hi:t_Slice u8 = ${ser1}.[ range1 ] in
assert (Seq.length src_lo == 16);
assert (Seq.length src_hi == 16);
let stored_lo:t_Slice u8 = Libcrux_intrinsics.Arm64.e_vst1q_bytes src_lo ${v}.f_low in
let stored_hi:t_Slice u8 = Libcrux_intrinsics.Arm64.e_vst1q_bytes src_hi ${v}.f_high in
Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_vst1q_bytes src_lo ${v}.f_low;
Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_vst1q_bytes src_hi ${v}.f_high;
Libcrux_intrinsics.Arm64_ml_kem_views.lemma_store_glue_two_writes_neon ${ser0} ${ser1} ${bytes} stored_lo stored_hi ${v}.f_low ${v}.f_high range0 range1;
let head : t_Array u8 (sz 32) = Seq.slice ${bytes} 0 32 in
introduce forall (i: nat{i < 256}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array (repr ${v}) 16 i ==
    Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i
with
  (lemma_repr_index ${v} (i / 16);
   FStar.Math.Lemmas.euclidean_division_definition i 16;
   if i < 128
   then
     Libcrux_intrinsics.Arm64_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma ${v}.f_low 16 i
   else
     (Libcrux_intrinsics.Arm64_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma ${v}.f_high 16 (i - 128);
      FStar.Math.Lemmas.euclidean_division_definition (i - 128) 16));
BitVecEq.bit_vec_equal_intro
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array (repr ${v}) 16)
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array (Seq.slice ${bytes} 0 32 <: t_Array u8 (sz 32)) 8)
"#
    );
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300"))]
#[hax_lib::requires(array.len() >= 32)]
#[hax_lib::ensures(|result| fstar!(r#"
    Core_models.Slice.impl__len #u8 ${array} >=. mk_usize 32 ==>
    (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
     Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
       #(mk_usize 16) head (repr ${result}))
"#))]
pub(crate) fn from_bytes(array: &[u8]) -> SIMD128Vector {
    let result = SIMD128Vector {
        low: _vld1q_bytes(&array[0..16]),
        high: _vld1q_bytes(&array[16..32]),
    };
    // The two `_vld1q_bytes` are total (length-only) over core-models, so the
    // per-half load bit-fact comes from `lemma_bv_bit_e_vld1q_bytes`; the i16x8
    // lane-bit decomposition on the LHS then discharges `from_le_bytes_post_N`.
    proof!(
        r#"
let range0:Core_models.Ops.Range.t_Range usize =
  { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 16 } in
let range1:Core_models.Ops.Range.t_Range usize =
  { Core_models.Ops.Range.f_start = mk_usize 16; Core_models.Ops.Range.f_end = mk_usize 32 } in
assert (Seq.length (${array}.[ range0 ] <: t_Slice u8) == 16);
assert (Seq.length (${array}.[ range1 ] <: t_Slice u8) == 16);
let head : t_Array u8 (sz 32) = Seq.slice ${array} 0 32 in
introduce forall (i: nat{i < 256}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array (repr ${result}) 16 i ==
    Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i
with
  (lemma_repr_index ${result} (i / 16);
   FStar.Math.Lemmas.euclidean_division_definition i 16;
   if i < 128
   then
     (Libcrux_intrinsics.Arm64_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma ${result}.f_low 16 i;
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_bv_bit_e_vld1q_bytes (${array}.[ range0 ] <: t_Slice u8) i;
      Seq.lemma_index_slice ${array} 0 16 (i / 8);
      Seq.lemma_index_slice ${array} 0 32 (i / 8))
   else
     (Libcrux_intrinsics.Arm64_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma ${result}.f_high 16 (i - 128);
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_bv_bit_e_vld1q_bytes (${array}.[ range1 ] <: t_Slice u8) (i - 128);
      FStar.Math.Lemmas.euclidean_division_definition (i - 128) 16;
      Seq.lemma_index_slice ${array} 16 32 ((i / 8) - 16);
      Seq.lemma_index_slice ${array} 0 32 (i / 8)));
BitVecEq.bit_vec_equal_intro
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array (repr ${result}) 16)
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array (Seq.slice ${array} 0 32 <: t_Array u8 (sz 32)) 8)
"#
    );
    result
}

#[allow(non_snake_case)]
#[inline(always)]
#[hax_lib::ensures(|result| fstar!("repr result == Seq.create 16 (mk_i16 0)"))]
pub(crate) fn ZERO() -> SIMD128Vector {
    let result = SIMD128Vector {
        low: _vdupq_n_s16(0),
        high: _vdupq_n_s16(0),
    };
    proof!(r#"Seq.lemma_eq_intro (repr ${result}) (Seq.create 16 (mk_i16 0))"#);
    result
}
