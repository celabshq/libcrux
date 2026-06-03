#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]

//! This module contains generated hacl code.

use libcrux_hacl_rs::prelude::*;
use libcrux_macros as krml;
use libcrux_sha2::hacl as hash_sha2;

#[derive(PartialEq, Clone, Copy)]
pub(crate) struct __uint32_t_uint32_t {
    pub fst: u32,
    pub snd: u32,
}

/**
Write the HMAC-SHA-2-256 MAC of a message (`data`) by using a key (`key`) into `dst`.

The key can be any length and will be hashed if it is longer and padded if it is shorter than 64 bytes.
`dst` must point to 32 bytes of memory.
*/
#[inline(always)]
pub fn compute_sha2_256(dst: &mut [u8], key: &[u8], key_len: u32, data: &[u8], data_len: u32) {
    let l: u32 = 64u32;
    let mut key_block: Box<[u8]> = vec![0x00u8; l as usize].into_boxed_slice();
    let nkey: (&mut [u8], &mut [u8]) = key_block.split_at_mut(0usize);
    let ite: u32 = if key_len <= 64u32 { key_len } else { 32u32 };
    let zeroes: (&mut [u8], &mut [u8]) = nkey.1.split_at_mut(ite as usize);
    lowstar::ignore::ignore::<&[u8]>(zeroes.1);
    if key_len <= 64u32 {
        (zeroes.0[0usize..key_len as usize]).copy_from_slice(&key[0usize..key_len as usize])
    } else {
        hash_sha2::hash_256(zeroes.0, key, key_len)
    };
    let mut ipad: Box<[u8]> = vec![0x36u8; l as usize].into_boxed_slice();
    for i in 0u32..l {
        let xi: u8 = (&ipad)[i as usize];
        let yi: u8 = (&key_block)[i as usize];
        (&mut ipad)[i as usize] = xi ^ yi
    }
    let mut opad: Box<[u8]> = vec![0x5cu8; l as usize].into_boxed_slice();
    for i in 0u32..l {
        let xi: u8 = (&opad)[i as usize];
        let yi: u8 = (&key_block)[i as usize];
        (&mut opad)[i as usize] = xi ^ yi
    }
    let mut st: [u32; 8] = [0u32; 8usize];
    krml::unroll_for!(8, "i", 0u32, 1u32, {
        let x: u32 = (&hash_sha2::h256)[i as usize];
        let os: (&mut [u32], &mut [u32]) = st.split_at_mut(0usize);
        os.1[i as usize] = x
    });
    let s: &mut [u32] = &mut st;
    if data_len == 0u32 {
        hash_sha2::sha256_update_last(0u64.wrapping_add(64u32 as u64), 64u32, &ipad, s)
    } else {
        let block_len: u32 = 64u32;
        let n_blocks: u32 = data_len.wrapping_div(block_len);
        let rem: u32 = data_len.wrapping_rem(block_len);
        let scrut: __uint32_t_uint32_t = if n_blocks > 0u32 && rem == 0u32 {
            let n_blocks·: u32 = n_blocks.wrapping_sub(1u32);
            __uint32_t_uint32_t {
                fst: n_blocks·,
                snd: data_len.wrapping_sub(n_blocks·.wrapping_mul(block_len)),
            }
        } else {
            __uint32_t_uint32_t {
                fst: n_blocks,
                snd: rem,
            }
        };
        let n_blocks0: u32 = scrut.fst;
        let rem_len: u32 = scrut.snd;
        let full_blocks_len: u32 = n_blocks0.wrapping_mul(block_len);
        let full_blocks: (&[u8], &[u8]) = data.split_at(0usize);
        let rem0: (&[u8], &[u8]) = full_blocks.1.split_at(full_blocks_len as usize);
        hash_sha2::sha256_update_nblocks(64u32, &ipad, s);
        hash_sha2::sha256_update_nblocks(n_blocks0.wrapping_mul(64u32), rem0.0, s);
        hash_sha2::sha256_update_last(
            (64u32 as u64)
                .wrapping_add(full_blocks_len as u64)
                .wrapping_add(rem_len as u64),
            rem_len,
            rem0.1,
            s,
        )
    };
    let dst1: (&mut [u8], &mut [u8]) = ipad.split_at_mut(0usize);
    hash_sha2::sha256_finish(s, dst1.1);
    let hash1: (&[u8], &[u8]) = dst1.1.split_at(0usize);
    hash_sha2::sha256_init(s);
    let block_len: u32 = 64u32;
    let n_blocks: u32 = 32u32.wrapping_div(block_len);
    let rem: u32 = 32u32.wrapping_rem(block_len);
    let scrut: __uint32_t_uint32_t = if n_blocks > 0u32 && rem == 0u32 {
        let n_blocks·: u32 = n_blocks.wrapping_sub(1u32);
        __uint32_t_uint32_t {
            fst: n_blocks·,
            snd: 32u32.wrapping_sub(n_blocks·.wrapping_mul(block_len)),
        }
    } else {
        __uint32_t_uint32_t {
            fst: n_blocks,
            snd: rem,
        }
    };
    let n_blocks0: u32 = scrut.fst;
    let rem_len: u32 = scrut.snd;
    let full_blocks_len: u32 = n_blocks0.wrapping_mul(block_len);
    let full_blocks: (&[u8], &[u8]) = hash1.1.split_at(0usize);
    let rem0: (&[u8], &[u8]) = full_blocks.1.split_at(full_blocks_len as usize);
    hash_sha2::sha256_update_nblocks(64u32, &opad, s);
    hash_sha2::sha256_update_nblocks(n_blocks0.wrapping_mul(64u32), rem0.0, s);
    hash_sha2::sha256_update_last(
        (64u32 as u64)
            .wrapping_add(full_blocks_len as u64)
            .wrapping_add(rem_len as u64),
        rem_len,
        rem0.1,
        s,
    );
    hash_sha2::sha256_finish(s, dst)
}

/**
Write the HMAC-SHA-2-384 MAC of a message (`data`) by using a key (`key`) into `dst`.

The key can be any length and will be hashed if it is longer and padded if it is shorter than 128 bytes.
`dst` must point to 48 bytes of memory.
*/
#[inline(always)]
pub fn compute_sha2_384(dst: &mut [u8], key: &[u8], key_len: u32, data: &[u8], data_len: u32) {
    let l: u32 = 128u32;
    let mut key_block: Box<[u8]> = vec![0x00u8; l as usize].into_boxed_slice();
    let nkey: (&mut [u8], &mut [u8]) = key_block.split_at_mut(0usize);
    let ite: u32 = if key_len <= 128u32 { key_len } else { 48u32 };
    let zeroes: (&mut [u8], &mut [u8]) = nkey.1.split_at_mut(ite as usize);
    lowstar::ignore::ignore::<&[u8]>(zeroes.1);
    if key_len <= 128u32 {
        (zeroes.0[0usize..key_len as usize]).copy_from_slice(&key[0usize..key_len as usize])
    } else {
        hash_sha2::hash_384(zeroes.0, key, key_len)
    };
    let mut ipad: Box<[u8]> = vec![0x36u8; l as usize].into_boxed_slice();
    for i in 0u32..l {
        let xi: u8 = (&ipad)[i as usize];
        let yi: u8 = (&key_block)[i as usize];
        (&mut ipad)[i as usize] = xi ^ yi
    }
    let mut opad: Box<[u8]> = vec![0x5cu8; l as usize].into_boxed_slice();
    for i in 0u32..l {
        let xi: u8 = (&opad)[i as usize];
        let yi: u8 = (&key_block)[i as usize];
        (&mut opad)[i as usize] = xi ^ yi
    }
    let mut st: [u64; 8] = [0u64; 8usize];
    krml::unroll_for!(8, "i", 0u32, 1u32, {
        let x: u64 = (&hash_sha2::h384)[i as usize];
        let os: (&mut [u64], &mut [u64]) = st.split_at_mut(0usize);
        os.1[i as usize] = x
    });
    let s: &mut [u64] = &mut st;
    if data_len == 0u32 {
        hash_sha2::sha384_update_last(
            fstar::uint128::add(
                fstar::uint128::uint64_to_uint128(0u64),
                fstar::uint128::uint64_to_uint128(128u32 as u64),
            ),
            128u32,
            &ipad,
            s,
        )
    } else {
        let block_len: u32 = 128u32;
        let n_blocks: u32 = data_len.wrapping_div(block_len);
        let rem: u32 = data_len.wrapping_rem(block_len);
        let scrut: __uint32_t_uint32_t = if n_blocks > 0u32 && rem == 0u32 {
            let n_blocks·: u32 = n_blocks.wrapping_sub(1u32);
            __uint32_t_uint32_t {
                fst: n_blocks·,
                snd: data_len.wrapping_sub(n_blocks·.wrapping_mul(block_len)),
            }
        } else {
            __uint32_t_uint32_t {
                fst: n_blocks,
                snd: rem,
            }
        };
        let n_blocks0: u32 = scrut.fst;
        let rem_len: u32 = scrut.snd;
        let full_blocks_len: u32 = n_blocks0.wrapping_mul(block_len);
        let full_blocks: (&[u8], &[u8]) = data.split_at(0usize);
        let rem0: (&[u8], &[u8]) = full_blocks.1.split_at(full_blocks_len as usize);
        hash_sha2::sha384_update_nblocks(128u32, &ipad, s);
        hash_sha2::sha384_update_nblocks(n_blocks0.wrapping_mul(128u32), rem0.0, s);
        hash_sha2::sha384_update_last(
            fstar::uint128::add(
                fstar::uint128::add(
                    fstar::uint128::uint64_to_uint128(128u32 as u64),
                    fstar::uint128::uint64_to_uint128(full_blocks_len as u64),
                ),
                fstar::uint128::uint64_to_uint128(rem_len as u64),
            ),
            rem_len,
            rem0.1,
            s,
        )
    };
    let dst1: (&mut [u8], &mut [u8]) = ipad.split_at_mut(0usize);
    hash_sha2::sha384_finish(s, dst1.1);
    let hash1: (&[u8], &[u8]) = dst1.1.split_at(0usize);
    hash_sha2::sha384_init(s);
    let block_len: u32 = 128u32;
    let n_blocks: u32 = 48u32.wrapping_div(block_len);
    let rem: u32 = 48u32.wrapping_rem(block_len);
    let scrut: __uint32_t_uint32_t = if n_blocks > 0u32 && rem == 0u32 {
        let n_blocks·: u32 = n_blocks.wrapping_sub(1u32);
        __uint32_t_uint32_t {
            fst: n_blocks·,
            snd: 48u32.wrapping_sub(n_blocks·.wrapping_mul(block_len)),
        }
    } else {
        __uint32_t_uint32_t {
            fst: n_blocks,
            snd: rem,
        }
    };
    let n_blocks0: u32 = scrut.fst;
    let rem_len: u32 = scrut.snd;
    let full_blocks_len: u32 = n_blocks0.wrapping_mul(block_len);
    let full_blocks: (&[u8], &[u8]) = hash1.1.split_at(0usize);
    let rem0: (&[u8], &[u8]) = full_blocks.1.split_at(full_blocks_len as usize);
    hash_sha2::sha384_update_nblocks(128u32, &opad, s);
    hash_sha2::sha384_update_nblocks(n_blocks0.wrapping_mul(128u32), rem0.0, s);
    hash_sha2::sha384_update_last(
        fstar::uint128::add(
            fstar::uint128::add(
                fstar::uint128::uint64_to_uint128(128u32 as u64),
                fstar::uint128::uint64_to_uint128(full_blocks_len as u64),
            ),
            fstar::uint128::uint64_to_uint128(rem_len as u64),
        ),
        rem_len,
        rem0.1,
        s,
    );
    hash_sha2::sha384_finish(s, dst)
}

/**
Write the HMAC-SHA-2-512 MAC of a message (`data`) by using a key (`key`) into `dst`.

The key can be any length and will be hashed if it is longer and padded if it is shorter than 128 bytes.
`dst` must point to 64 bytes of memory.
*/
#[inline(always)]
pub fn compute_sha2_512(dst: &mut [u8], key: &[u8], key_len: u32, data: &[u8], data_len: u32) {
    let l: u32 = 128u32;
    let mut key_block: Box<[u8]> = vec![0x00u8; l as usize].into_boxed_slice();
    let nkey: (&mut [u8], &mut [u8]) = key_block.split_at_mut(0usize);
    let ite: u32 = if key_len <= 128u32 { key_len } else { 64u32 };
    let zeroes: (&mut [u8], &mut [u8]) = nkey.1.split_at_mut(ite as usize);
    lowstar::ignore::ignore::<&[u8]>(zeroes.1);
    if key_len <= 128u32 {
        (zeroes.0[0usize..key_len as usize]).copy_from_slice(&key[0usize..key_len as usize])
    } else {
        hash_sha2::hash_512(zeroes.0, key, key_len)
    };
    let mut ipad: Box<[u8]> = vec![0x36u8; l as usize].into_boxed_slice();
    for i in 0u32..l {
        let xi: u8 = (&ipad)[i as usize];
        let yi: u8 = (&key_block)[i as usize];
        (&mut ipad)[i as usize] = xi ^ yi
    }
    let mut opad: Box<[u8]> = vec![0x5cu8; l as usize].into_boxed_slice();
    for i in 0u32..l {
        let xi: u8 = (&opad)[i as usize];
        let yi: u8 = (&key_block)[i as usize];
        (&mut opad)[i as usize] = xi ^ yi
    }
    let mut st: [u64; 8] = [0u64; 8usize];
    krml::unroll_for!(8, "i", 0u32, 1u32, {
        let x: u64 = (&hash_sha2::h512)[i as usize];
        let os: (&mut [u64], &mut [u64]) = st.split_at_mut(0usize);
        os.1[i as usize] = x
    });
    let s: &mut [u64] = &mut st;
    if data_len == 0u32 {
        hash_sha2::sha512_update_last(
            fstar::uint128::add(
                fstar::uint128::uint64_to_uint128(0u64),
                fstar::uint128::uint64_to_uint128(128u32 as u64),
            ),
            128u32,
            &ipad,
            s,
        )
    } else {
        let block_len: u32 = 128u32;
        let n_blocks: u32 = data_len.wrapping_div(block_len);
        let rem: u32 = data_len.wrapping_rem(block_len);
        let scrut: __uint32_t_uint32_t = if n_blocks > 0u32 && rem == 0u32 {
            let n_blocks·: u32 = n_blocks.wrapping_sub(1u32);
            __uint32_t_uint32_t {
                fst: n_blocks·,
                snd: data_len.wrapping_sub(n_blocks·.wrapping_mul(block_len)),
            }
        } else {
            __uint32_t_uint32_t {
                fst: n_blocks,
                snd: rem,
            }
        };
        let n_blocks0: u32 = scrut.fst;
        let rem_len: u32 = scrut.snd;
        let full_blocks_len: u32 = n_blocks0.wrapping_mul(block_len);
        let full_blocks: (&[u8], &[u8]) = data.split_at(0usize);
        let rem0: (&[u8], &[u8]) = full_blocks.1.split_at(full_blocks_len as usize);
        hash_sha2::sha512_update_nblocks(128u32, &ipad, s);
        hash_sha2::sha512_update_nblocks(n_blocks0.wrapping_mul(128u32), rem0.0, s);
        hash_sha2::sha512_update_last(
            fstar::uint128::add(
                fstar::uint128::add(
                    fstar::uint128::uint64_to_uint128(128u32 as u64),
                    fstar::uint128::uint64_to_uint128(full_blocks_len as u64),
                ),
                fstar::uint128::uint64_to_uint128(rem_len as u64),
            ),
            rem_len,
            rem0.1,
            s,
        )
    };
    let dst1: (&mut [u8], &mut [u8]) = ipad.split_at_mut(0usize);
    hash_sha2::sha512_finish(s, dst1.1);
    let hash1: (&[u8], &[u8]) = dst1.1.split_at(0usize);
    hash_sha2::sha512_init(s);
    let block_len: u32 = 128u32;
    let n_blocks: u32 = 64u32.wrapping_div(block_len);
    let rem: u32 = 64u32.wrapping_rem(block_len);
    let scrut: __uint32_t_uint32_t = if n_blocks > 0u32 && rem == 0u32 {
        let n_blocks·: u32 = n_blocks.wrapping_sub(1u32);
        __uint32_t_uint32_t {
            fst: n_blocks·,
            snd: 64u32.wrapping_sub(n_blocks·.wrapping_mul(block_len)),
        }
    } else {
        __uint32_t_uint32_t {
            fst: n_blocks,
            snd: rem,
        }
    };
    let n_blocks0: u32 = scrut.fst;
    let rem_len: u32 = scrut.snd;
    let full_blocks_len: u32 = n_blocks0.wrapping_mul(block_len);
    let full_blocks: (&[u8], &[u8]) = hash1.1.split_at(0usize);
    let rem0: (&[u8], &[u8]) = full_blocks.1.split_at(full_blocks_len as usize);
    hash_sha2::sha512_update_nblocks(128u32, &opad, s);
    hash_sha2::sha512_update_nblocks(n_blocks0.wrapping_mul(128u32), rem0.0, s);
    hash_sha2::sha512_update_last(
        fstar::uint128::add(
            fstar::uint128::add(
                fstar::uint128::uint64_to_uint128(128u32 as u64),
                fstar::uint128::uint64_to_uint128(full_blocks_len as u64),
            ),
            fstar::uint128::uint64_to_uint128(rem_len as u64),
        ),
        rem_len,
        rem0.1,
        s,
    );
    hash_sha2::sha512_finish(s, dst)
}
