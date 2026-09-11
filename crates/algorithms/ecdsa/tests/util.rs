#![allow(dead_code)]

use std::num::ParseIntError;

pub(crate) fn hex_str_to_bytes(val: &str) -> Vec<u8> {
    let b: Result<Vec<u8>, ParseIntError> = (0..val.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
        .collect();
    b.expect("Error parsing hex string")
}

pub(crate) fn hex_str_to_array<A>(val: &str) -> A
where
    A: Default + AsMut<[u8]>,
{
    let b: Result<Vec<u8>, ParseIntError> = (0..val.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
        .collect();
    let b = b.expect("Error parsing hex string");
    let mut out = A::default();
    A::as_mut(&mut out).clone_from_slice(&b);
    out
}
