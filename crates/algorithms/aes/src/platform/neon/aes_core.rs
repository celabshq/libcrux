use libcrux_intrinsics::arm64::{
    _uint8x16_t, _vaeseq_u8, _vaesmcq_u8, _vdupq_laneq_u32, _vdupq_n_u32, _vdupq_n_u8, _veorq_u32,
    _veorq_u8, _vextq_u32, _vld1q_u32, _vld1q_u8, _vreinterpretq_u32_u8, _vreinterpretq_u8_u32,
    _vst1q_u8,
};

/// The Neon state
pub(crate) type State = _uint8x16_t;

#[inline]
fn new_state() -> State {
    _vdupq_n_u8(0)
}

#[inline]
fn xor_key1_state(st: &mut State, k: &State) {
    *st = _veorq_u8(*st, *k);
}

#[inline]
fn aes_enc(st: &mut State, key: &State) {
    *st = _veorq_u8(_vaesmcq_u8(_vaeseq_u8(*st, _vdupq_n_u8(0))), *key);
}

#[inline]
fn aes_enc_last(st: &mut State, key: &State) {
    *st = _veorq_u8(_vaeseq_u8(*st, _vdupq_n_u8(0)), *key)
}

#[inline]
fn aes_keygen_assist(next: &mut State, prev: &State, rcon: u8) {
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
fn aes_keygen_assist0(next: &mut State, prev: &State, rcon: u8) {
    aes_keygen_assist(next, prev, rcon);
    *next = _vreinterpretq_u8_u32(_vdupq_laneq_u32::<3>(_vreinterpretq_u32_u8(*next)))
}

#[inline]
fn aes_keygen_assist1(next: &mut State, prev: &State) {
    aes_keygen_assist(next, prev, 0);
    *next = _vreinterpretq_u8_u32(_vdupq_laneq_u32::<2>(_vreinterpretq_u32_u8(*next)));
}

#[inline]
fn key_expansion_step(next: &mut State, prev: &State) {
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
        new_state()
    }

    #[inline]
    fn load_block(&mut self, b: &[u8]) {
        debug_assert!(b.len() == 16);
        *self = _vld1q_u8(b);
    }

    #[inline]
    fn store_block(&self, out: &mut [u8]) {
        debug_assert!(out.len() == 16);
        _vst1q_u8(out, *self);
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
        let out_vec = _veorq_u8(inp_vec, *self);
        _vst1q_u8(&mut block_out, out_vec);

        out.copy_from_slice(&block_out[0..out.len()]);
    }

    #[inline]
    fn xor_key(&mut self, key: &Self) {
        xor_key1_state(self, key);
    }

    #[inline]
    fn aes_enc(&mut self, key: &Self) {
        aes_enc(self, key);
    }

    #[inline]
    fn aes_enc_last(&mut self, key: &Self) {
        aes_enc_last(self, key);
    }

    #[inline]
    fn aes_keygen_assist0<const RCON: i32>(&mut self, prev: &Self) {
        aes_keygen_assist0(self, prev, RCON as u8);
    }

    #[inline]
    fn aes_keygen_assist1(&mut self, prev: &Self) {
        aes_keygen_assist1(self, prev);
    }

    #[inline]
    fn key_expansion_step(&mut self, prev: &Self) {
        key_expansion_step(self, prev);
    }
}
