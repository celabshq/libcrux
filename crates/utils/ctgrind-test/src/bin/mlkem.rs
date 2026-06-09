use crabgrind::memcheck::{self, MemState};
use libcrux_ml_kem::mlkem512;
use std::ffi::c_void;
use std::hint::black_box;

#[inline(never)]
fn test_decapsulate() {
    // Generate key pair and ciphertext with known randomness — not secret yet.
    // KEY_GENERATION_SEED_SIZE = CPA_PKE_KEY_GENERATION_SEED_SIZE + SHARED_SECRET_SIZE = 64
    let key_pair = mlkem512::generate_key_pair(black_box([0u8; 64]));
    let (ct, _) = mlkem512::encapsulate(key_pair.public_key(), black_box([0u8; 32]));

    // The IND-CCA private key packs: [cpa_sk | pk | H(pk) | z]
    // Only cpa_sk and z are secret; the embedded pk and H(pk) are public.
    // Derive layout from public type sizes rather than internal constants.
    // Marking everything as secret will fail us in `rej_sample`.
    let sk_bytes: &[u8] = key_pair.private_key().as_ref();
    let pk_len = key_pair.public_key().as_ref().len(); // 800 for mlkem512
    const H_DIGEST_LEN: usize = 32;
    const Z_LEN: usize = 32;
    let cpa_sk_len = sk_bytes.len() - pk_len - H_DIGEST_LEN - Z_LEN; // 768
    let z_offset = cpa_sk_len + pk_len + H_DIGEST_LEN; // 1600

    // Classify CPA secret key
    let _ = memcheck::mark_memory(
        sk_bytes.as_ptr() as *const c_void,
        cpa_sk_len,
        MemState::Undefined,
    );
    // Classify implicit rejection value z
    let _ = memcheck::mark_memory(
        sk_bytes[z_offset..].as_ptr() as *const c_void,
        Z_LEN,
        MemState::Undefined,
    );

    let mut ss = mlkem512::decapsulate(key_pair.private_key(), &ct);

    // Declassify shared secret before use.
    let _ = memcheck::mark_memory(ss.as_mut_ptr() as *mut c_void, ss.len(), MemState::Defined);
    println!("mlkem512 shared secret: {:02x?}", &ss[..4]);
}

pub fn main() {
    test_decapsulate();
}
