use libcrux_intrinsics::arm64::*;

/// A Neon gf128 field element
#[derive(Clone, Copy)]
pub(crate) struct FieldElement(pub(crate) u128);

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
fn store_element(element: &FieldElement, bytes: &mut [u8]) {
    debug_assert!(bytes.len() == 16);

    bytes.copy_from_slice(&element.0.to_be_bytes());
}

#[inline]
fn add(element: &mut FieldElement, other: &FieldElement) {
    element.0 ^= other.0;
}

#[inline]
fn mul_wide(element: &FieldElement, other: &FieldElement) -> (FieldElement, FieldElement) {
    let l0 = element.0 as u64;
    let h0 = (element.0 >> 64) as u64;
    let l1 = other.0 as u64;
    let h1 = (other.0 >> 64) as u64;

    let low: u128 = _vmull_p64(l0, l1);
    let m1: u128 = _vmull_p64(l0, h1);
    let m2: u128 = _vmull_p64(l1, h0);
    let high: u128 = _vmull_p64(h0, h1);

    let mid = m1 ^ m2;
    let m0 = mid << 64;
    let m1 = mid >> 64;
    let low = low ^ m0;
    let high = high ^ m1;

    (FieldElement(high), FieldElement(low))
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
fn mul(x: &mut FieldElement, y: &FieldElement) {
    let (high, low) = mul_wide(x, y);
    *x = reduce(&high, &low);
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
        add(self, other);
    }

    #[inline]
    fn mul(&mut self, other: &Self) {
        mul(self, other)
    }
}
