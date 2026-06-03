use crabgrind::memcheck::{self, MemState};
use std::ffi::c_void;
use std::hint::black_box;

/* VULNERABLE: Returns early if a mismatch is found.
 * Execution time depends on the secret data. */
#[inline(never)]
fn compare_insecure(secret: &[u8], public: &[u8]) -> bool {
    for i in 0..secret.len() {
        if secret[i] != public[i] {
            // Valgrind will flag this!
            return false;
        }
    }
    true
}

/* SECURE: Always iterates through the entire array.
 * Execution time is completely independent of the secret data. */
#[inline(never)]
fn compare_secure(secret: &[u8], public: &[u8]) -> bool {
    let mut diff = 0u8;
    for i in 0..secret.len() {
        diff |= secret[i] ^ public[i];
    }
    diff == 0
}

pub fn main() {
    // We use black_box to prevent LLVM from pre-calculating the result at compile time
    let secret = black_box([0xDE, 0xAD, 0xBE, 0xEF]);
    let public = black_box([0xDE, 0xAD, 0xBE, 0x00]);

    // 1. CLASSIFY THE SECRET: Tell Valgrind to treat this memory as uninitialized
    // We use `let _ =` because `mark_memory` returns the number of unaddressable bytes (usize)
    let _ = memcheck::mark_memory(
        secret.as_ptr() as *const c_void,
        secret.len(),
        MemState::Undefined,
    );
    // --- TEST SECURE FUNCTION ---
    let mut res_sec = compare_secure(&secret, &public);

    // Unclassify the result so we can safely branch/print it
    let _ = memcheck::mark_memory(
        &mut res_sec as *mut _ as *mut c_void,
        std::mem::size_of_val(&res_sec),
        MemState::Defined,
    );
    println!("Secure result: {}", res_sec);

    // --- TEST INSECURE FUNCTION ---
    let mut res_insec = compare_insecure(&secret, &public);

    // Declassify the result
    let _ = memcheck::mark_memory(
        &mut res_insec as *mut _ as *mut c_void,
        std::mem::size_of_val(&res_insec),
        MemState::Defined,
    );
    println!("Insecure result: {}", res_insec);
}
