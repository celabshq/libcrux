/// A portable gf128 field element.
pub(crate) type FieldElement = u128;

#[inline]
fn zero() -> FieldElement {
    0
}

#[inline]
fn load_element(bytes: &[u8]) -> FieldElement {
    debug_assert!(bytes.len() == 16);

    u128::from_be_bytes(bytes.try_into().unwrap())
}

#[inline]
fn store_element(element: &FieldElement, bytes: &mut [u8]) {
    debug_assert!(bytes.len() == 16);
    bytes.copy_from_slice(&u128::to_be_bytes(*element));
}

#[inline]
fn add(element: &FieldElement, other: &FieldElement) -> FieldElement {
    element ^ other
}

#[inline]
fn ith_bit_mask(elem: &FieldElement, i: usize) -> FieldElement {
    debug_assert!(i < 128);

    let bit: u16 = ((elem >> (127 - i)) as u16) & 0x1;
    let bit_mask16 = (!bit).wrapping_add(1);
    let bit_mask32 = (bit_mask16 as u32) ^ ((bit_mask16 as u32) << 16);
    let bit_mask64 = (bit_mask32 as u64) ^ ((bit_mask32 as u64) << 32);

    (bit_mask64 as u128) ^ ((bit_mask64 as u128) << 64)
}

const IRRED: FieldElement = 0xE100_0000_0000_0000_0000_0000_0000_0000;

#[inline]
fn mul_x(elem: &mut FieldElement) {
    let mask = ith_bit_mask(elem, 127);
    *elem = (*elem >> 1) ^ (IRRED & mask)
}

#[inline]
fn mul_step(x: &FieldElement, y: &mut FieldElement, i: usize, result: &mut FieldElement) {
    debug_assert!(i < 128);
    let mask = ith_bit_mask(x, i);
    *result ^= *y & mask;
    mul_x(y);
}

#[inline]
fn mul(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let mut result = 0;
    let mut multiplicand = *y;
    for i in 0..128 {
        mul_step(x, &mut multiplicand, i, &mut result)
    }
    result
}

impl crate::platform::GF128FieldElement for FieldElement {
    #[inline]
    fn zero() -> Self {
        zero()
    }

    #[inline]
    fn load_element(bytes: &[u8]) -> Self {
        load_element(bytes)
    }

    #[inline]
    fn store_element(&self, bytes: &mut [u8]) {
        store_element(self, bytes);
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
