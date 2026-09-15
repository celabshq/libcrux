use libcrux_intrinsics::avx2::{
    mm_aesenc_si128, mm_aesenclast_si128, mm_aeskeygenassist_si128, mm_loadu_si128,
    mm_setzero_si128, mm_shuffle_epi32, mm_slli_si128, mm_storeu_si128_u8, mm_xor_si128, Vec128,
};

/// The avx2 state.
///
/// A `#[repr(transparent)]` newtype around [`Vec128`]. Under `core-models` both
/// `x86 __m128i` and `arm uint8x16_t` unify to `BitVec<128>`, so without this
/// distinct nominal wrapper the neon and x64 `impl AESState for State` blocks
/// would become conflicting impls for `BitVec<128>` (E0119). The wrapper is
/// zero-cost: `#[repr(transparent)]` guarantees identical layout and the `.0`
/// field access compiles away, so runtime behavior is identical to the former
/// `type State = Vec128` alias.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub(crate) struct State(Vec128);

#[inline]
fn new_state() -> Vec128 {
    mm_setzero_si128()
}

#[inline]
fn xor_key1_state(st: &mut Vec128, k: &Vec128) {
    *st = mm_xor_si128(*st, *k);
}

#[inline]
fn aes_enc(st: &mut Vec128, key: &Vec128) {
    *st = mm_aesenc_si128(*st, *key);
}

#[inline]
fn aes_enc_last(st: &mut Vec128, key: &Vec128) {
    *st = mm_aesenclast_si128(*st, *key);
}

#[inline]
fn aes_keygen_assist<const RCON: i32>(next: &mut Vec128, prev: &Vec128) {
    *next = mm_aeskeygenassist_si128::<RCON>(*prev);
}

#[inline]
fn aes_keygen_assist0<const RCON: i32>(next: &mut Vec128, prev: &Vec128) {
    aes_keygen_assist::<RCON>(next, prev);
    *next = mm_shuffle_epi32::<0xff>(*next);
}

#[inline]
fn aes_keygen_assist1(next: &mut Vec128, prev: &Vec128) {
    aes_keygen_assist::<0>(next, prev);
    *next = mm_shuffle_epi32::<0xaa>(*next);
}

#[inline]
fn key_expansion_step(next: &mut Vec128, prev: &Vec128) {
    let p0 = mm_xor_si128(*prev, mm_slli_si128::<4>(*prev));
    let p1 = mm_xor_si128(p0, mm_slli_si128::<4>(p0));
    let p2 = mm_xor_si128(p1, mm_slli_si128::<4>(p1));
    *next = mm_xor_si128(*next, p2);
}

impl crate::platform::AESState for State {
    #[inline]
    fn new() -> Self {
        State(new_state())
    }

    #[inline]
    fn load_block(&mut self, b: &[u8]) {
        debug_assert!(b.len() == 16);

        self.0 = mm_loadu_si128(b);
    }

    #[inline]
    fn store_block(&self, out: &mut [u8]) {
        debug_assert!(out.len() == 16);

        mm_storeu_si128_u8(out, self.0);
    }

    #[inline]
    fn xor_block(&self, input: &[u8], out: &mut [u8]) {
        debug_assert!(input.len() == out.len() && input.len() <= 16);
        // XXX: hot-fix to have enough input and output here.
        let mut block_in = [0u8; 16];
        let mut block_out = [0u8; 16];
        block_in[0..input.len()].copy_from_slice(input);

        let inp_vec = mm_loadu_si128(&block_in);
        let out_vec = mm_xor_si128(inp_vec, self.0);
        mm_storeu_si128_u8(&mut block_out, out_vec);

        out.copy_from_slice(&block_out[0..out.len()]);
    }

    #[inline]
    fn xor_key(&mut self, key: &Self) {
        xor_key1_state(&mut self.0, &key.0);
    }

    #[inline]
    fn aes_enc(&mut self, key: &Self) {
        aes_enc(&mut self.0, &key.0);
    }

    #[inline]
    fn aes_enc_last(&mut self, key: &Self) {
        aes_enc_last(&mut self.0, &key.0);
    }

    #[inline]
    fn aes_keygen_assist0<const RCON: i32>(&mut self, prev: &Self) {
        aes_keygen_assist0::<RCON>(&mut self.0, &prev.0);
    }

    #[inline]
    fn aes_keygen_assist1(&mut self, prev: &Self) {
        aes_keygen_assist1(&mut self.0, &prev.0);
    }

    #[inline]
    fn key_expansion_step(&mut self, prev: &Self) {
        key_expansion_step(&mut self.0, &prev.0)
    }
}

#[cfg(feature = "std")]
#[test]
fn test() {
    use libcrux_intrinsics::avx2::{mm_set_epi32, mm_storeu_si128_i32};

    let x = mm_set_epi32(3, 2, 1, 0);
    let y = mm_shuffle_epi32::<0xaa>(x);
    let w = mm_slli_si128::<4>(x);
    let mut z: [i32; 4] = [0; 4];
    mm_storeu_si128_i32(&mut z, x);

    std::eprintln!("{:?}", z);
    mm_storeu_si128_i32(&mut z, w);

    std::eprintln!("shift right 4 {:?}", z);
    mm_storeu_si128_i32(&mut z, y);

    std::eprintln!("shuffle aa {:?}", z);
}
