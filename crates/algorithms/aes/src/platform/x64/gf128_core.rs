use libcrux_intrinsics::avx2::{
    mm_clmulepi64_si128, mm_loadu_si128_u128, mm_slli_si128, mm_srli_si128, mm_storeu_si128_u128,
    mm_unpackhi_epi64, mm_unpacklo_epi64, mm_xor_si128, Vec128,
};

// XXX: A lot of the code below is shared with NEON. Refactor!

/// An avx2 gf128 field element.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct FieldElement(pub(super) u128);

impl FieldElement {
    /// Transmute `u128` and `Vec128`.
    #[inline]
    #[allow(unsafe_code)]
    fn transmute(&self) -> Vec128 {
        mm_loadu_si128_u128(&self.0)
    }

    /// Convert a vec to self.
    #[inline]
    #[allow(unsafe_code)]
    fn from_vec128(vec: Vec128) -> Self {
        let mut out = 0;
        mm_storeu_si128_u128(&mut out, vec);
        Self(out)
    }
}

#[inline]
fn zero() -> FieldElement {
    FieldElement(0)
}

#[inline]
fn load_element(b: &[u8]) -> FieldElement {
    debug_assert!(b.len() == 16);

    FieldElement(u128::from_be_bytes(b.try_into().unwrap()))
}

#[inline]
fn store_element(elem: &FieldElement, b: &mut [u8]) {
    debug_assert!(b.len() == 16);

    b.copy_from_slice(&elem.0.to_be_bytes());
}

#[inline]
fn add(elem: &FieldElement, other: &FieldElement) -> FieldElement {
    FieldElement(elem.0 ^ other.0)
}

/// Performs a 128x128 to 256-bit carry-less multiplication.
///
/// This implementation uses the Karatsuba algorithm to reduce the number of expensive
/// PCLMULQDQ instructions from 4 to 3. On most modern x64 CPUs (Intel Sandy
/// Bridge and newer, AMD Zen and newer), this results in higher performance due to
/// better utilization of execution ports and potentially lower overall latency.
///
/// @param elem The first 128-bit field element.
/// @param other The second 128-bit field element.
/// @returns A tuple `(high, low)` containing the 256-bit result.
#[inline]
fn mul_wide(elem: &FieldElement, other: &FieldElement) -> (FieldElement, FieldElement) {
    // Let the inputs be a = (a_hi << 64) | a_lo and b = (b_hi << 64) | b_lo.
    // The product is (a_hi*b_hi << 128) + ((a_lo*b_hi ^ a_hi*b_lo) << 64) + a_lo*b_lo.
    // The Karatsuba trick computes the middle term using the other two products:
    // (a_lo*b_hi ^ a_hi*b_lo) = (a_lo^a_hi)*(b_lo^b_hi) ^ a_lo*b_lo ^ a_hi*b_hi

    let a: Vec128 = elem.transmute();
    let b: Vec128 = other.transmute();

    // 1. Calculate the low and high 128-bit parts of the product in parallel.
    //    p_lo = a_lo * b_lo
    let p_lo = mm_clmulepi64_si128::<0x00>(a, b);
    //    p_hi = a_hi * b_hi
    let p_hi = mm_clmulepi64_si128::<0x11>(a, b);

    // 2. Calculate the middle term using the third multiplication.
    //    First, prepare the operands (a_lo^a_hi) and (b_lo^b_hi).
    //    Using unpack instructions is an alternative to shuffling.
    let a_xor = mm_xor_si128(mm_unpackhi_epi64(a, a), mm_unpacklo_epi64(a, a));
    let b_xor = mm_xor_si128(mm_unpackhi_epi64(b, b), mm_unpacklo_epi64(b, b));

    // Multiply the low 64-bit parts of the XORed results.
    // p_mid_prod = (a_lo^a_hi) * (b_lo^b_hi)
    let p_mid_prod = mm_clmulepi64_si128::<0x00>(a_xor, b_xor);

    // Finish computing the middle term by XORing with p_lo and p_hi.
    let p_mid = mm_xor_si128(mm_xor_si128(p_mid_prod, p_lo), p_hi);

    // 3. Combine the parts to get the final 256-bit result.
    //    The middle part is XORed at a 64-bit offset.
    //    res_low  = p_lo ^ (p_mid << 64)
    //    res_high = p_hi ^ (p_mid >> 64)
    let res_low = mm_xor_si128(p_lo, mm_slli_si128::<8>(p_mid));
    let res_high = mm_xor_si128(p_hi, mm_srli_si128::<8>(p_mid));

    // The original function returned (high_part, low_part). We maintain that order.
    (
        FieldElement::from_vec128(res_high),
        FieldElement::from_vec128(res_low),
    )
}

#[inline]
fn reduce(high: &FieldElement, low: &FieldElement) -> FieldElement {
    let high = (high.0 << 1) ^ (low.0 >> 127);
    let low = low.0 << 1;
    let x0_0 = low << 64;
    let x1_x0 = low ^ (x0_0 << 63) ^ (x0_0 << 62) ^ (x0_0 << 57);
    let x1_x0 = x1_x0 ^ (x1_x0 >> 1) ^ (x1_x0 >> 2) ^ (x1_x0 >> 7);
    FieldElement(x1_x0 ^ high)
}

#[inline]
fn mul(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let (high, low) = mul_wide(x, y);
    reduce(&high, &low)
}

impl crate::platform::GF128FieldElement for FieldElement {
    #[inline]
    fn zero() -> Self {
        zero()
    }

    #[inline]
    fn load_element(b: &[u8]) -> Self {
        load_element(b)
    }

    #[inline]
    fn store_element(&self, b: &mut [u8]) {
        store_element(self, b);
    }

    #[inline]
    fn add(&mut self, other: &Self) {
        *self = add(self, other);
    }

    #[inline]
    fn mul(&mut self, other: &Self) {
        *self = mul(self, other)
    }
}

#[allow(unsafe_code)]
#[test]
fn test_transmute() {
    let x = FieldElement((1u128 << 64) + 698234);
    let xv = x.transmute();
    let xu128 = FieldElement::from_vec128(xv).0;
    assert_eq!(x.0, xu128);

    let x = FieldElement(1u128 << 64 ^ 2u128);
    let xv = x.transmute();
    let xv = mm_slli_si128::<8>(xv);
    let x_l8 = FieldElement::from_vec128(xv);

    assert_eq!(x_l8.0, x.0 << 64);
}
