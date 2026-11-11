//! The `fstar::uintX` mask functions from libcrux-hacl-rs, in the two shapes the HACL code uses
//! them: as a select, and accumulated over a buffer followed by a comparison of the result.

use ctgrind_test::{classify, declassify};
use libcrux_hacl_rs::fstar::{uint64, uint8};
use std::hint::black_box;

#[inline(never)]
fn select_eq_u8(a: u8, b: u8, x: u8, y: u8) -> u8 {
    let m = uint8::eq_mask(a, b);
    (m & x) | (!m & y)
}

#[inline(never)]
fn all_eq_u8(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut m = 0xFFu8;
    for i in 0..32 {
        m &= uint8::eq_mask(a[i], b[i]);
    }
    m == 0xFF
}

#[inline(never)]
fn select_eq_u64(a: u64, b: u64, x: u64, y: u64) -> u64 {
    let m = uint64::eq_mask(a, b);
    (m & x) | (!m & y)
}

#[inline(never)]
fn all_eq_u64(a: &[u64; 32], b: &[u64; 32]) -> bool {
    let mut m = u64::MAX;
    for i in 0..32 {
        m &= uint64::eq_mask(a[i], b[i]);
    }
    m == u64::MAX
}

#[inline(never)]
fn select_gte_u64(a: u64, b: u64, x: u64, y: u64) -> u64 {
    let m = uint64::gte_mask(a, b);
    (m & x) | (!m & y)
}

#[inline(never)]
fn all_gte_u64(a: &[u64; 32], b: &[u64; 32]) -> bool {
    let mut m = u64::MAX;
    for i in 0..32 {
        m &= uint64::gte_mask(a[i], b[i]);
    }
    m == u64::MAX
}

#[inline(never)]
fn test_u8() {
    let a = black_box([0x42u8; 32]);
    let b = black_box([0x42u8; 32]);
    classify(&a);

    let selected = select_eq_u8(a[0], b[0], 1, 2);
    declassify(&selected);
    let eq = all_eq_u8(&a, &b);
    declassify(&eq);
    println!("uint8: select {selected}, all_eq {eq}");
}

#[inline(never)]
fn test_u64() {
    let a = black_box([0x4242u64; 32]);
    let b = black_box([0x4241u64; 32]);
    classify(&a);

    let selected_eq = select_eq_u64(a[0], b[0], 1, 2);
    declassify(&selected_eq);
    let eq = all_eq_u64(&a, &b);
    declassify(&eq);
    let selected_gte = select_gte_u64(a[0], b[0], 1, 2);
    declassify(&selected_gte);
    let gte = all_gte_u64(&a, &b);
    declassify(&gte);
    println!(
        "uint64: select_eq {selected_eq}, all_eq {eq}, select_gte {selected_gte}, all_gte {gte}"
    );
}

pub fn main() {
    test_u8();
    test_u64();
}
