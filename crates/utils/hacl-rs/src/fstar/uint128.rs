#![allow(non_camel_case_types)]

pub type uint128 = u128;

pub fn add(x: uint128, y: uint128) -> uint128 {
    x.wrapping_add(y)
}
pub fn add_mod(x: uint128, y: uint128) -> uint128 {
    x.wrapping_add(y)
}
pub fn shift_left(x: uint128, y: u32) -> uint128 {
    x.wrapping_shl(y)
}
pub fn shift_right(x: uint128, y: u32) -> uint128 {
    x.wrapping_shr(y)
}
pub fn uint64_to_uint128(x: u64) -> uint128 {
    x as u128
}
pub fn uint128_to_uint64(x: uint128) -> u64 {
    x as u64
}
pub fn mul_wide(x: u64, y: u64) -> uint128 {
    (x as u128) * (y as u128)
}
