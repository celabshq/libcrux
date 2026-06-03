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

/// Performs a 64x64 to 128-bit carry-less multiplication.
#[inline]
fn clmul64(x: u64, y: u64) -> FieldElement {
    // Multiplication in GF(2)[X] with “holes”
    // (sequences of zeroes) to avoid carry spilling.
    //
    // When carries do occur, they wind up in a "hole" and are subsequently
    // masked out of the result.
    // Adapted from https://github.com/RustCrypto/universal-hashes/blob/802b40974a08bbd2663c63780fc87a23ee931868/polyval/src/backend/soft64.rs#L201C1-L227C2
    // Uses the technique described in https://www.bearssl.org/constanttime.html#ghash-for-gcm
    // but directly outputs the 128 bits without needing the Rev trick.
    // This method is constant time and significantly faster than iterating over the
    // bits of x and xoring shifted y.
    let x0 = (x & 0x1111_1111_1111_1111) as FieldElement;
    let x1 = (x & 0x2222_2222_2222_2222) as FieldElement;
    let x2 = (x & 0x4444_4444_4444_4444) as FieldElement;
    let x3 = (x & 0x8888_8888_8888_8888) as FieldElement;
    let y0 = (y & 0x1111_1111_1111_1111) as FieldElement;
    let y1 = (y & 0x2222_2222_2222_2222) as FieldElement;
    let y2 = (y & 0x4444_4444_4444_4444) as FieldElement;
    let y3 = (y & 0x8888_8888_8888_8888) as FieldElement;

    let mut z0 = (x0 * y0) ^ (x1 * y3) ^ (x2 * y2) ^ (x3 * y1);
    let mut z1 = (x0 * y1) ^ (x1 * y0) ^ (x2 * y3) ^ (x3 * y2);
    let mut z2 = (x0 * y2) ^ (x1 * y1) ^ (x2 * y0) ^ (x3 * y3);
    let mut z3 = (x0 * y3) ^ (x1 * y2) ^ (x2 * y1) ^ (x3 * y0);

    z0 &= 0x1111_1111_1111_1111_1111_1111_1111_1111;
    z1 &= 0x2222_2222_2222_2222_2222_2222_2222_2222;
    z2 &= 0x4444_4444_4444_4444_4444_4444_4444_4444;
    z3 &= 0x8888_8888_8888_8888_8888_8888_8888_8888;

    z0 | z1 | z2 | z3
}

/// Performs a 128x128 to 256-bit carry-less multiplication.
///
/// Return (high, low) bits
#[inline]
pub fn mul_wide(&a: &FieldElement, &b: &FieldElement) -> (FieldElement, FieldElement) {
    let (a_low, a_high) = (a as u64, (a >> 64) as u64);
    let (b_low, b_high) = (b as u64, (b >> 64) as u64);

    // Use karatsuba multiplication
    let ab_low = clmul64(a_low, b_low);
    let ab_high = clmul64(a_high, b_high);
    let ab_mid = clmul64(a_low ^ a_high, b_low ^ b_high) ^ ab_low ^ ab_high;
    let low = ab_low ^ (ab_mid << 64);
    let high = ab_high ^ (ab_mid >> 64);
    (high, low)
}

#[inline]
pub fn reduce(&high: &FieldElement, &low: &FieldElement) -> FieldElement {
    let high = (high << 1) ^ (low >> 127);
    let low = low << 1;
    let x0_0 = low << 64;
    let x1_x0 = low ^ (x0_0 << 63) ^ (x0_0 << 62) ^ (x0_0 << 57);
    let x1_x0 = x1_x0 ^ (x1_x0 >> 1) ^ (x1_x0 >> 2) ^ (x1_x0 >> 7);
    x1_x0 ^ high
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
