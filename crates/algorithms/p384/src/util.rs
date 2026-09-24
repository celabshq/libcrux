//! # Utility functions

use crate::constants::FP_NUM_BYTES;

#[inline(never)]
/// Big-endian byte-array comparison: is `a < b`? Used both to reject
/// non-canonical (unreduced) field element encodings and to rejection-sample
/// private keys against the curve order.
pub(crate) fn be_bytes_lt(a: &[u8; FP_NUM_BYTES], b: &[u8; FP_NUM_BYTES]) -> bool {
    let mut a_gt_b = false;
    let mut preceeding_bytes_lt = false;

    let mut a_eq_b = true;

    for (a_i, b_i) in a.iter().zip(b.iter()) {
        let diff_bytes = a_i ^ b_i;
        a_eq_b &= diff_bytes == 0;

        let current_byte_gt = core::hint::black_box(b_i.overflowing_sub(*a_i).1);

        // In case of a_i > b_i, we can still have a < b, if any of the
        // preceeding bytes of a was strictly less than the
        // corresponding byte of b.
        a_gt_b |= current_byte_gt & !preceeding_bytes_lt;
        preceeding_bytes_lt |= core::hint::black_box(a_i.overflowing_sub(*b_i).1);
    }

    !(a_gt_b | a_eq_b)
}

#[inline(never)]
pub(crate) fn be_bytes_nonzero(x: &[u8; FP_NUM_BYTES]) -> bool {
    let mut check = 0u8;
    for byte in x {
        check |= *byte;
    }
    check != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lt() {
        let mut a = [0u8; 48];
        let mut b = [0u8; 48];

        // a == b == 0
        assert!(!be_bytes_lt(&a, &b));

        // b >= a
        b[47] = 1;
        assert!(be_bytes_lt(&a, &b));
        b[0] = 1;
        assert!(be_bytes_lt(&a, &b));

        // a bytes can be larger than b bytes if preceeding b bytes
        // were larger than preceeding b bytes.
        a[1] = 2;
        assert!(be_bytes_lt(&a, &b));

        a[47] = 1;
        b[1] = 2;
        assert!(be_bytes_lt(&a, &b));

        // a == b != 0
        a[0] = 1;
        assert!(!be_bytes_lt(&a, &b));

        // a > b != 0
        a[0] = 2;
        assert!(!be_bytes_lt(&a, &b));
    }
}
