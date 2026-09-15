use libcrux_intrinsics::arm64::{
    _uint8x16_t, _vaeseq_u8, _vaesmcq_u8, _vdupq_laneq_u32, _vdupq_n_u32, _vdupq_n_u8, _veorq_u32,
    _veorq_u8, _vextq_u32, _vld1q_u32, _vld1q_u8, _vreinterpretq_u32_u8, _vreinterpretq_u8_u32,
    _vst1q_u8,
};

/// The Neon state
///
/// A `#[repr(transparent)]` newtype around [`_uint8x16_t`]. Under `core-models`
/// both `arm uint8x16_t` and `x86 __m128i` unify to `BitVec<128>`, so without
/// this distinct nominal wrapper the neon and x64 `impl AESState for State`
/// blocks would become conflicting impls for `BitVec<128>` (E0119). The wrapper
/// is zero-cost: `#[repr(transparent)]` guarantees identical layout and the `.0`
/// field access compiles away, so runtime behavior is identical to the former
/// `type State = _uint8x16_t` alias.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub(crate) struct State(_uint8x16_t);

#[inline]
fn new_state() -> _uint8x16_t {
    _vdupq_n_u8(0)
}

#[inline]
fn xor_key1_state(st: &mut _uint8x16_t, k: &_uint8x16_t) {
    *st = _veorq_u8(*st, *k);
}

#[inline]
fn aes_enc(st: &mut _uint8x16_t, key: &_uint8x16_t) {
    *st = _veorq_u8(_vaesmcq_u8(_vaeseq_u8(*st, _vdupq_n_u8(0))), *key);
}

#[inline]
fn aes_enc_last(st: &mut _uint8x16_t, key: &_uint8x16_t) {
    *st = _veorq_u8(_vaeseq_u8(*st, _vdupq_n_u8(0)), *key)
}

#[inline]
fn aes_keygen_assist(next: &mut _uint8x16_t, prev: &_uint8x16_t, rcon: u8) {
    let st = _vaeseq_u8(*prev, _vdupq_n_u8(0));
    let mut tmp = [0u8; 16];
    _vst1q_u8(&mut tmp, st);
    let tmp_new = [
        tmp[4], tmp[1], tmp[14], tmp[11], tmp[1], tmp[14], tmp[11], tmp[4], tmp[12], tmp[9],
        tmp[6], tmp[3], tmp[9], tmp[6], tmp[3], tmp[12],
    ];
    let st_new = _vld1q_u8(&tmp_new);
    let rcon_array = [0, rcon as u32, 0, rcon as u32];
    let rcon_vec = _vreinterpretq_u8_u32(_vld1q_u32(&rcon_array));
    *next = _veorq_u8(st_new, rcon_vec);
}

#[inline]
fn aes_keygen_assist0(next: &mut _uint8x16_t, prev: &_uint8x16_t, rcon: u8) {
    aes_keygen_assist(next, prev, rcon);
    *next = _vreinterpretq_u8_u32(_vdupq_laneq_u32::<3>(_vreinterpretq_u32_u8(*next)))
}

#[inline]
fn aes_keygen_assist1(next: &mut _uint8x16_t, prev: &_uint8x16_t) {
    aes_keygen_assist(next, prev, 0);
    *next = _vreinterpretq_u8_u32(_vdupq_laneq_u32::<2>(_vreinterpretq_u32_u8(*next)));
}

#[inline]
fn key_expansion_step(next: &mut _uint8x16_t, prev: &_uint8x16_t) {
    let zero = _vdupq_n_u32(0);
    let prev0 = _vreinterpretq_u32_u8(*prev);
    let prev1 = _veorq_u32(prev0, _vextq_u32::<3>(zero, prev0));
    let prev2 = _veorq_u32(prev1, _vextq_u32::<3>(zero, prev1));
    let prev3 = _veorq_u32(prev2, _vextq_u32::<3>(zero, prev2));
    *next = _veorq_u8(*next, _vreinterpretq_u8_u32(prev3));
}

impl crate::platform::AESState for State {
    #[inline]
    fn new() -> Self {
        State(new_state())
    }

    #[inline]
    fn load_block(&mut self, b: &[u8]) {
        debug_assert!(b.len() == 16);
        self.0 = _vld1q_u8(b);
    }

    #[inline]
    fn store_block(&self, out: &mut [u8]) {
        debug_assert!(out.len() == 16);
        _vst1q_u8(out, self.0);
    }

    #[inline]
    fn xor_block(&self, input: &[u8], out: &mut [u8]) {
        debug_assert!(input.len() == out.len() && input.len() <= 16);
        // XXX: hot-fix to have enough input and output here.
        // For some reason this doesn't fail even if we don't do this.
        let mut block_in = [0u8; 16];
        let mut block_out = [0u8; 16];
        block_in[0..input.len()].copy_from_slice(input);

        let inp_vec = _vld1q_u8(&block_in);
        let out_vec = _veorq_u8(inp_vec, self.0);
        _vst1q_u8(&mut block_out, out_vec);

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
        aes_keygen_assist0(&mut self.0, &prev.0, RCON as u8);
    }

    #[inline]
    fn aes_keygen_assist1(&mut self, prev: &Self) {
        aes_keygen_assist1(&mut self.0, &prev.0);
    }

    #[inline]
    fn key_expansion_step(&mut self, prev: &Self) {
        key_expansion_step(&mut self.0, &prev.0);
    }
}
