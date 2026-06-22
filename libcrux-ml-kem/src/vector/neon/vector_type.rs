use libcrux_intrinsics::arm64::*;
#[derive(Clone, Copy)]
#[hax_lib::fstar::before(interface, "noeq")]
#[hax_lib::fstar::after(
    interface,
    r#"let repr (x:t_SIMD128Vector) : t_Array i16 (sz 16) =
  Seq.append (Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 x.f_low)
             (Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 x.f_high)

val lemma_repr_index (x: t_SIMD128Vector) (j: nat{j < 16})
    : Lemma
      (Seq.index (repr x) j ==
        (if j < 8
         then Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 x.f_low j
         else Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 x.f_high (j - 8)))
      [SMTPat (Seq.index (repr x) j)]"#
)]
#[hax_lib::fstar::after(
    r#"let lemma_repr_index (x: t_SIMD128Vector) (j: nat{j < 16}) =
  let lo = Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 x.f_low in
  let hi = Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 x.f_high in
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
    hax_lib::fstar!(
        r#"let lo = Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 ${v}.f_low in
let hi = Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 ${v}.f_high in
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
    hax_lib::fstar!(
        r#"let lo = Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 ${result}.f_low in
let hi = Libcrux_intrinsics.Arm64_extract.vec128_as_i16x8 ${result}.f_high in
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
#[hax_lib::fstar::verification_status(panic_free)]
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
    _vst1q_bytes(&mut bytes[0..16], v.low);
    _vst1q_bytes(&mut bytes[16..32], v.high);
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(array.len() >= 32)]
#[hax_lib::ensures(|result| fstar!(r#"
    Core_models.Slice.impl__len #u8 ${array} >=. mk_usize 32 ==>
    (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
     Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
       #(mk_usize 16) head (repr ${result}))
"#))]
pub(crate) fn from_bytes(array: &[u8]) -> SIMD128Vector {
    SIMD128Vector {
        low: _vld1q_bytes(&array[0..16]),
        high: _vld1q_bytes(&array[16..32]),
    }
}

#[allow(non_snake_case)]
#[inline(always)]
#[hax_lib::ensures(|result| fstar!("repr result == Seq.create 16 (mk_i16 0)"))]
pub(crate) fn ZERO() -> SIMD128Vector {
    let result = SIMD128Vector {
        low: _vdupq_n_s16(0),
        high: _vdupq_n_s16(0),
    };
    hax_lib::fstar!(r#"Seq.lemma_eq_intro (repr ${result}) (Seq.create 16 (mk_i16 0))"#);
    result
}
