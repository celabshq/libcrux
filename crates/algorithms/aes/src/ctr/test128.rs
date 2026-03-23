use crate::{
    aes_gcm_128::GCM_KEY_LEN,
    ctr::{AesCtrContext, AES_GCM_CTR_LEN, AES_GCM_NONCE_START},
    platform::{self, AESState},
    NONCE_LEN,
};

type AesGcm128CtrContext<T> = AesCtrContext<T, 11, AES_GCM_CTR_LEN, AES_GCM_NONCE_START>;

pub(crate) fn aes128_ctr_xor_block<T: AESState>(
    ctx: &AesGcm128CtrContext<T>,
    ctr: u32,
    inp: &[u8],
    out: &mut [u8],
) {
    debug_assert!(inp.len() == out.len() && inp.len() <= 16);
    ctx.aes_ctr_xor_block(ctr, inp, out);
}

pub(crate) fn aes128_ctr_encrypt<T: AESState>(
    key: &[u8],
    nonce: &[u8],
    ctr: u32,
    inp: &[u8],
    out: &mut [u8],
) {
    debug_assert!(nonce.len() == NONCE_LEN);
    debug_assert!(key.len() == GCM_KEY_LEN);
    debug_assert!(inp.len() == out.len());
    let ctx = AesGcm128CtrContext::<T>::init(key, nonce);
    ctx.aes_ctr_update(ctr, inp, out);
}

const INPUT: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
];
const KEY: [u8; 16] = [
    0x7E, 0x24, 0x06, 0x78, 0x17, 0xFA, 0xE0, 0xD7, 0x43, 0xD6, 0xCE, 0x1F, 0x32, 0x53, 0x91, 0x63,
];
const NONCE: [u8; 12] = [
    0x00, 0x6C, 0xB6, 0xDB, 0xC0, 0x54, 0x3B, 0x59, 0xDA, 0x48, 0xD9, 0x0B,
];
const EXPECTED: [u8; 32] = [
    0x51, 0x04, 0xA1, 0x06, 0x16, 0x8A, 0x72, 0xD9, 0x79, 0x0D, 0x41, 0xEE, 0x8E, 0xDA, 0xD3, 0x88,
    0xEB, 0x2E, 0x1E, 0xFC, 0x46, 0xDA, 0x57, 0xC8, 0xFC, 0xE6, 0x30, 0xDF, 0x91, 0x41, 0xBE, 0x28,
];

#[test]
fn test_ctr_block() {
    let mut computed: [u8; 32] = [0u8; 32];
    let ctx = AesGcm128CtrContext::<platform::portable::State>::init(&KEY, &NONCE);
    aes128_ctr_xor_block(&ctx, 1, &INPUT[0..16], &mut computed[0..16]);
    aes128_ctr_xor_block(&ctx, 2, &INPUT[16..32], &mut computed[16..32]);
    for i in 0..32 {
        if computed[i] != EXPECTED[i] {
            #[cfg(feature = "std")]
            std::eprintln!(
                "mismatch at {}: expected is {}, computed is {}",
                i,
                EXPECTED[i],
                computed[i]
            );
            panic!();
        }
    }
}

#[cfg(feature = "simd128")]
#[test]
fn test_ctr_block_neon() {
    let mut computed: [u8; 32] = [0u8; 32];
    let ctx = AesGcm128CtrContext::<platform::neon::State>::init(&KEY, &NONCE);
    aes128_ctr_xor_block(&ctx, 1, &INPUT[0..16], &mut computed[0..16]);
    aes128_ctr_xor_block(&ctx, 2, &INPUT[16..32], &mut computed[16..32]);
    for i in 0..32 {
        if computed[i] != EXPECTED[i] {
            #[cfg(feature = "std")]
            std::eprintln!(
                "mismatch at {}: expected is {}, computed is {}",
                i,
                EXPECTED[i],
                computed[i]
            );
            panic!();
        }
    }
}

#[test]
fn test_ctr_encrypt() {
    let mut computed: [u8; 32] = [0u8; 32];
    aes128_ctr_encrypt::<platform::portable::State>(&KEY, &NONCE, 1, &INPUT, &mut computed);
    for i in 0..32 {
        if computed[i] != EXPECTED[i] {
            #[cfg(feature = "std")]
            std::eprintln!(
                "mismatch at {}: expected is {}, computed is {}",
                i,
                EXPECTED[i],
                computed[i]
            );
            panic!();
        }
    }
}

#[cfg(feature = "simd128")]
#[test]
fn test_ctr_encrypt_neon() {
    let mut computed: [u8; 32] = [0u8; 32];
    aes128_ctr_encrypt::<platform::neon::State>(&KEY, &NONCE, 1, &INPUT, &mut computed);
    for i in 0..32 {
        if computed[i] != EXPECTED[i] {
            #[cfg(feature = "std")]
            std::eprintln!(
                "mismatch at {}: expected is {}, computed is {}",
                i,
                EXPECTED[i],
                computed[i]
            );
            panic!();
        }
    }
}

#[cfg(all(feature = "simd256", feature = "std"))]
#[test]
fn test_ctr_encrypt_intel() {
    let mut computed: [u8; 32] = [0u8; 32];
    aes128_ctr_encrypt::<platform::x64::State>(&KEY, &NONCE, 1, &INPUT, &mut computed);
    for i in 0..32 {
        if computed[i] != EXPECTED[i] {
            std::eprintln!(
                "mismatch at {}: expected is {}, computed is {}",
                i,
                EXPECTED[i],
                computed[i]
            );
            panic!();
        }
    }
}
