use crabgrind::memcheck::{self, MemState};
use std::ffi::c_void;
use std::hint::black_box;

#[inline(never)]
fn test_sha256() {
    let message = black_box([0x42u8; 64]);
    let _ = memcheck::mark_memory(
        message.as_ptr() as *const c_void,
        message.len(),
        MemState::Undefined,
    );
    let mut digest = libcrux_sha3::sha256(&message);
    let _ = memcheck::mark_memory(
        digest.as_mut_ptr() as *mut c_void,
        digest.len(),
        MemState::Defined,
    );
    println!("sha256: {:02x?}", &digest[..4]);
}

#[inline(never)]
fn test_sha512() {
    let message = black_box([0x42u8; 64]);
    let _ = memcheck::mark_memory(
        message.as_ptr() as *const c_void,
        message.len(),
        MemState::Undefined,
    );
    let mut digest = libcrux_sha3::sha512(&message);
    let _ = memcheck::mark_memory(
        digest.as_mut_ptr() as *mut c_void,
        digest.len(),
        MemState::Defined,
    );
    println!("sha512: {:02x?}", &digest[..4]);
}

#[inline(never)]
fn test_shake256() {
    let message = black_box([0x42u8; 64]);
    let _ = memcheck::mark_memory(
        message.as_ptr() as *const c_void,
        message.len(),
        MemState::Undefined,
    );
    let mut digest = libcrux_sha3::shake256::<64>(&message);
    let _ = memcheck::mark_memory(
        digest.as_mut_ptr() as *mut c_void,
        digest.len(),
        MemState::Defined,
    );
    println!("shake256: {:02x?}", &digest[..4]);
}

pub fn main() {
    test_sha256();
    test_sha512();
    test_shake256();
}
