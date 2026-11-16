//! Vectors for libcrux using aarch64 (neon) intrinsics

use super::{Operations, FIELD_MODULUS};

// mod sampling;
mod arithmetic;
mod compress;
mod ntt;
mod serialize;
mod vector_type;

use arithmetic::*;
use compress::*;
use ntt::*;
use serialize::*;
pub(crate) use vector_type::SIMD128Vector;
use vector_type::*;

#[cfg(hax)]
impl crate::vector::traits::Repr for SIMD128Vector {
    fn repr(&self) -> [i16; 16] {
        to_i16_array(self.clone())
    }
}

#[cfg(any(eurydice, not(hax)))]
impl crate::vector::traits::Repr for SIMD128Vector {}

#[cfg_attr(hax, hax_lib::attributes)]
impl Operations for SIMD128Vector {
    #[inline(always)]
    #[cfg_attr(hax, hax_lib::ensures(|out| fstar!(r#"impl.f_repr out == Seq.create 16 (mk_i16 0)"#)))]
    fn ZERO() -> Self {
        ZERO()
    }

    #[cfg_attr(hax, hax_lib::requires(array.len() == 16))]
    #[cfg_attr(hax, hax_lib::ensures(|out| fstar!(r#"impl.f_repr out == $array"#)))]
    fn from_i16_array(array: &[i16]) -> Self {
        from_i16_array(array)
    }

    #[cfg_attr(hax, hax_lib::ensures(|out| fstar!(r#"out == impl.f_repr $x"#)))]
    fn to_i16_array(x: Self) -> [i16; 16] {
        to_i16_array(x)
    }

    #[cfg_attr(hax, hax_lib::requires(array.len() >= 32))]
    fn from_bytes(array: &[u8]) -> Self {
        from_bytes(array)
    }

    #[cfg_attr(hax, hax_lib::requires(bytes.len() >= 32))]
    fn to_bytes(x: Self, bytes: &mut [u8]) {
        to_bytes(x, bytes)
    }

    fn add(lhs: Self, rhs: &Self) -> Self {
        add(lhs, rhs)
    }

    fn sub(lhs: Self, rhs: &Self) -> Self {
        sub(lhs, rhs)
    }

    fn multiply_by_constant(v: Self, c: i16) -> Self {
        multiply_by_constant(v, c)
    }

    fn to_unsigned_representative(a: Self) -> Self {
        to_unsigned_representative(a)
    }

    fn cond_subtract_3329(v: Self) -> Self {
        cond_subtract_3329(v)
    }

    fn barrett_reduce(v: Self) -> Self {
        barrett_reduce(v)
    }

    fn montgomery_multiply_by_constant(v: Self, c: i16) -> Self {
        montgomery_multiply_by_constant(v, c)
    }

    fn compress_1(v: Self) -> Self {
        compress_1(v)
    }

    fn compress<const COEFFICIENT_BITS: i32>(v: Self) -> Self {
        compress::<COEFFICIENT_BITS>(v)
    }

    fn decompress_1(a: Self) -> Self {
        decompress_1(a)
    }

    fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(v: Self) -> Self {
        decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(v)
    }

    fn ntt_layer_1_step(a: Self, zeta1: i16, zeta2: i16, zeta3: i16, zeta4: i16) -> Self {
        ntt_layer_1_step(a, zeta1, zeta2, zeta3, zeta4)
    }

    fn ntt_layer_2_step(a: Self, zeta1: i16, zeta2: i16) -> Self {
        ntt_layer_2_step(a, zeta1, zeta2)
    }

    fn ntt_layer_3_step(a: Self, zeta: i16) -> Self {
        ntt_layer_3_step(a, zeta)
    }

    fn inv_ntt_layer_1_step(a: Self, zeta1: i16, zeta2: i16, zeta3: i16, zeta4: i16) -> Self {
        inv_ntt_layer_1_step(a, zeta1, zeta2, zeta3, zeta4)
    }

    fn inv_ntt_layer_2_step(a: Self, zeta1: i16, zeta2: i16) -> Self {
        inv_ntt_layer_2_step(a, zeta1, zeta2)
    }

    fn inv_ntt_layer_3_step(a: Self, zeta: i16) -> Self {
        inv_ntt_layer_3_step(a, zeta)
    }

    fn ntt_multiply(
        lhs: &Self,
        rhs: &Self,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
        zeta4: i16,
    ) -> Self {
        ntt_multiply(lhs, rhs, zeta1, zeta2, zeta3, zeta4)
    }

    fn serialize_1(a: Self) -> [u8; 2] {
        serialize_1(a)
    }

    fn deserialize_1(a: &[u8]) -> Self {
        deserialize_1(a)
    }

    fn serialize_4(a: Self) -> [u8; 8] {
        serialize_4(a)
    }

    fn deserialize_4(a: &[u8]) -> Self {
        deserialize_4(a)
    }

    fn serialize_5(a: Self) -> [u8; 10] {
        serialize_5(a)
    }

    fn deserialize_5(a: &[u8]) -> Self {
        deserialize_5(a)
    }

    fn serialize_10(a: Self) -> [u8; 20] {
        serialize_10(a)
    }

    fn deserialize_10(a: &[u8]) -> Self {
        deserialize_10(a)
    }

    fn serialize_11(a: Self) -> [u8; 22] {
        serialize_11(a)
    }

    fn deserialize_11(a: &[u8]) -> Self {
        deserialize_11(a)
    }

    fn serialize_12(a: Self) -> [u8; 24] {
        serialize_12(a)
    }

    fn deserialize_12(a: &[u8]) -> Self {
        deserialize_12(a)
    }

    fn rej_sample(a: &[u8], out: &mut [i16]) -> usize {
        // FIXME: The code in rejsample fails on the CI machines.
        // We need to understand why and fix it before using it.
        // We use the portable version in the meantime.
        rej_sample(a, out)
    }
}

#[inline(always)]
pub(crate) fn rej_sample(a: &[u8], result: &mut [i16]) -> usize {
    let mut sampled = 0;
    for bytes in a.chunks(3) {
        let b1 = bytes[0] as i16;
        let b2 = bytes[1] as i16;
        let b3 = bytes[2] as i16;

        let d1 = ((b2 & 0xF) << 8) | b1;
        let d2 = (b3 << 4) | (b2 >> 4);

        if d1 < FIELD_MODULUS && sampled < 16 {
            result[sampled] = d1;
            sampled += 1
        }
        if d2 < FIELD_MODULUS && sampled < 16 {
            result[sampled] = d2;
            sampled += 1
        }
    }
    sampled
}
