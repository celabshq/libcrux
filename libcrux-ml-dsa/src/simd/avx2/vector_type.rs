/// The vector type
#[hax_lib::fstar::before("noeq")]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct Vec256 {
    pub(super) value: libcrux_intrinsics::avx2::Vec256,
}

/// An avx2 encoded ring element
pub(crate) type AVX2RingElement = [Vec256; super::SIMD_UNITS_IN_RING_ELEMENT];

/// Create an all-zero vector coefficient
pub(crate) fn zero() -> Vec256 {
    Vec256 {
        value: libcrux_intrinsics::avx2::mm256_setzero_si256(),
    }
}

/// Create a coefficient from an `i32` array
pub(crate) fn from_coefficient_array(coefficient_array: &[i32], out: &mut Vec256) {
    out.value = libcrux_intrinsics::avx2::mm256_loadu_si256_i32(coefficient_array)
}

/// Write out the coefficient to an `i32` array
#[inline(always)]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${out}_future == Seq.length ${out} /\
    (Seq.length ${out} == 8 ==>
      (forall (i: nat). i < 8 ==>
        Seq.index ${out}_future i ==
        Spec.Intrinsics.to_i32x8 ${value}.f_value (mk_u64 i)))"#))]
pub(crate) fn to_coefficient_array(value: &Vec256, out: &mut [i32]) {
    libcrux_intrinsics::avx2::mm256_storeu_si256_i32(out, value.value);
}
