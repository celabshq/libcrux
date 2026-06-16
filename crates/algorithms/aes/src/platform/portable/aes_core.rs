#![allow(clippy::needless_range_loop)]

use crate::aes::AES_BLOCK_LEN;

#[cfg(test)]
mod test;

pub(crate) type State = [u16; 8];

#[inline]
fn new_state() -> State {
    [0u16; 8]
}

#[inline]
fn interleave_u8_1(i0: u8, i1: u8) -> u16 {
    let mut x = i0 as u16;

    x = (x | (x << 4)) & 0x0F0F;
    x = (x | (x << 2)) & 0x3333;
    x = (x | (x << 1)) & 0x5555;

    let mut y = i1 as u16;

    y = (y | (y << 4)) & 0x0F0F;
    y = (y | (y << 2)) & 0x3333;
    y = (y | (y << 1)) & 0x5555;

    x | (y << 1)
}

#[inline]
fn deinterleave_u8_1(i0: u16) -> (u8, u8) {
    let mut x = i0 & 0x5555;

    x = (x | (x >> 1)) & 0x3333;
    x = (x | (x >> 2)) & 0x0F0F;
    x = (x | (x >> 4)) & 0x00FF;

    let mut y = (i0 >> 1) & 0x5555;

    y = (y | (y >> 1)) & 0x3333;
    y = (y | (y >> 2)) & 0x0F0F;
    y = (y | (y >> 4)) & 0x00FF;

    (x as u8, y as u8)
}

#[inline]
fn interleave_u16_2(i0: u16, i1: u16) -> (u16, u16) {
    let x = ((i1 & 0x3333) << 2) | (i0 & 0x3333);
    let y = ((i0 & 0xcccc) >> 2) | (i1 & 0xcccc);
    (x, y)
}

#[inline]
fn interleave_u16_4(i0: u16, i1: u16) -> (u16, u16) {
    let x = ((i1 & 0x0F0F) << 4) | (i0 & 0x0F0F);
    let y = ((i0 & 0xF0F0) >> 4) | (i1 & 0xF0F0);
    (x, y)
}

#[inline]
fn interleave_u16_8(i0: u16, i1: u16) -> (u16, u16) {
    let x = ((i1 & 0x00FF) << 8) | (i0 & 0x00FF);
    let y = ((i0 & 0xFF00) >> 8) | (i1 & 0xFF00);
    (x, y)
}

#[inline]
fn transpose_u8x16(input: &[u8; 16], output: &mut [u16; 8]) {
    let o0 = interleave_u8_1(input[0], input[1]);
    let o1 = interleave_u8_1(input[2], input[3]);
    let o2 = interleave_u8_1(input[4], input[5]);
    let o3 = interleave_u8_1(input[6], input[7]);
    let o4 = interleave_u8_1(input[8], input[9]);
    let o5 = interleave_u8_1(input[10], input[11]);
    let o6 = interleave_u8_1(input[12], input[13]);
    let o7 = interleave_u8_1(input[14], input[15]);

    let (o0, o1) = interleave_u16_2(o0, o1);
    let (o2, o3) = interleave_u16_2(o2, o3);
    let (o4, o5) = interleave_u16_2(o4, o5);
    let (o6, o7) = interleave_u16_2(o6, o7);

    let (o0, o2) = interleave_u16_4(o0, o2);
    let (o1, o3) = interleave_u16_4(o1, o3);
    let (o4, o6) = interleave_u16_4(o4, o6);
    let (o5, o7) = interleave_u16_4(o5, o7);

    let (o0, o4) = interleave_u16_8(o0, o4);
    let (o1, o5) = interleave_u16_8(o1, o5);
    let (o2, o6) = interleave_u16_8(o2, o6);
    let (o3, o7) = interleave_u16_8(o3, o7);

    output[0] = o0;
    output[1] = o1;
    output[2] = o2;
    output[3] = o3;
    output[4] = o4;
    output[5] = o5;
    output[6] = o6;
    output[7] = o7;
}

#[inline]
fn transpose_u16x8(input: &[u16; 8], output: &mut [u8]) {
    let (i0, i4) = interleave_u16_8(input[0], input[4]);
    let (i1, i5) = interleave_u16_8(input[1], input[5]);
    let (i2, i6) = interleave_u16_8(input[2], input[6]);
    let (i3, i7) = interleave_u16_8(input[3], input[7]);

    let (i0, i2) = interleave_u16_4(i0, i2);
    let (i1, i3) = interleave_u16_4(i1, i3);
    let (i4, i6) = interleave_u16_4(i4, i6);
    let (i5, i7) = interleave_u16_4(i5, i7);

    let (i0, i1) = interleave_u16_2(i0, i1);
    let (i2, i3) = interleave_u16_2(i2, i3);
    let (i4, i5) = interleave_u16_2(i4, i5);
    let (i6, i7) = interleave_u16_2(i6, i7);

    let (o0, o1) = deinterleave_u8_1(i0);
    let (o2, o3) = deinterleave_u8_1(i1);
    let (o4, o5) = deinterleave_u8_1(i2);
    let (o6, o7) = deinterleave_u8_1(i3);
    let (o8, o9) = deinterleave_u8_1(i4);

    let (o10, o11) = deinterleave_u8_1(i5);
    let (o12, o13) = deinterleave_u8_1(i6);
    let (o14, o15) = deinterleave_u8_1(i7);

    output[0] = o0;
    output[1] = o1;
    output[2] = o2;
    output[3] = o3;
    output[4] = o4;
    output[5] = o5;
    output[6] = o6;
    output[7] = o7;
    output[8] = o8;
    output[9] = o9;
    output[10] = o10;
    output[11] = o11;
    output[12] = o12;
    output[13] = o13;
    output[14] = o14;
    output[15] = o15;
}

#[inline]
fn xnor(a: u16, b: u16) -> u16 {
    !(a ^ b)
}

#[inline]
fn sub_bytes_state(st: &mut State) {
    let u0 = st[7];
    let u1 = st[6];
    let u2 = st[5];
    let u3 = st[4];
    let u4 = st[3];
    let u5 = st[2];
    let u6 = st[1];
    let u7 = st[0];

    let t1 = u6 ^ u4;
    let t2 = u3 ^ u0;
    let t3 = u1 ^ u2;
    let t4 = u7 ^ t3;
    let t5 = t1 ^ t2;
    let t6 = u1 ^ u5;
    let t7 = u0 ^ u6;
    let t8 = t1 ^ t6;
    let t9 = u6 ^ t4;
    let t10 = u3 ^ t4;
    let t11 = u7 ^ t5;
    let t12 = t5 ^ t6;
    let t13 = u2 ^ u5;
    let t14 = t3 ^ t5;
    let t15 = u5 ^ t7;
    let t16 = u0 ^ u5;
    let t17 = u7 ^ t8;
    let t18 = u6 ^ u5;
    let t19 = t2 ^ t18;
    let t20 = t4 ^ t15;
    let t21 = t1 ^ t13;
    let t22 = u0 ^ t4;
    let t39 = t21 ^ t5;
    let t40 = t21 ^ t7;
    let t41 = t7 ^ t19;
    let t42 = t16 ^ t14;
    let t43 = t22 ^ t17;
    let t44 = t19 & t5;
    let t45 = t20 & t11;
    let t46 = t12 ^ t44;
    let t47 = t10 & u7;
    let t48 = t47 ^ t44;
    let t49 = t7 & t21;
    let t50 = t9 & t4;
    let t51 = t40 ^ t49;
    let t52 = t22 & t17;
    let t53 = t52 ^ t49;
    let t54 = t2 & t8;
    let t55 = t41 & t39;
    let t56 = t55 ^ t54;
    let t57 = t16 & t14;
    let t58 = t57 ^ t54;
    let t59 = t46 ^ t45;
    let t60 = t48 ^ t42;
    let t61 = t51 ^ t50;
    let t62 = t53 ^ t58;
    let t63 = t59 ^ t56;
    let t64 = t60 ^ t58;
    let t65 = t61 ^ t56;
    let t66 = t62 ^ t43;
    let t67 = t65 ^ t66;
    let t68 = t65 & t63;
    let t69 = t64 ^ t68;
    let t70 = t63 ^ t64;
    let t71 = t66 ^ t68;
    let t72 = t71 & t70;
    let t73 = t69 & t67;
    let t74 = t63 & t66;
    let t75 = t70 & t74;
    let t76 = t70 ^ t68;
    let t77 = t64 & t65;
    let t78 = t67 & t77;
    let t79 = t67 ^ t68;
    let t80 = t64 ^ t72;
    let t81 = t75 ^ t76;
    let t82 = t66 ^ t73;
    let t83 = t78 ^ t79;
    let t84 = t81 ^ t83;
    let t85 = t80 ^ t82;
    let t86 = t80 ^ t81;
    let t87 = t82 ^ t83;
    let t88 = t85 ^ t84;
    let t89 = t87 & t5;
    let t90 = t83 & t11;
    let t91 = t82 & u7;
    let t92 = t86 & t21;
    let t93 = t81 & t4;
    let t94 = t80 & t17;
    let t95 = t85 & t8;
    let t96 = t88 & t39;
    let t97 = t84 & t14;
    let t98 = t87 & t19;
    let t99 = t83 & t20;
    let t100 = t82 & t10;
    let t101 = t86 & t7;
    let t102 = t81 & t9;
    let t103 = t80 & t22;
    let t104 = t85 & t2;
    let t105 = t88 & t41;
    let t106 = t84 & t16;
    let t107 = t104 ^ t105;
    let t108 = t93 ^ t99;
    let t109 = t96 ^ t107;
    let t110 = t98 ^ t108;
    let t111 = t91 ^ t101;
    let t112 = t89 ^ t92;
    let t113 = t107 ^ t112;
    let t114 = t90 ^ t110;
    let t115 = t89 ^ t95;
    let t116 = t94 ^ t102;
    let t117 = t97 ^ t103;
    let t118 = t91 ^ t114;
    let t119 = t111 ^ t117;
    let t120 = t100 ^ t108;
    let t121 = t92 ^ t95;
    let t122 = t110 ^ t121;
    let t123 = t106 ^ t119;
    let t124 = t104 ^ t115;
    let t125 = t111 ^ t116;

    let t128 = t94 ^ t107;

    let t131 = t93 ^ t101;
    let t132 = t112 ^ t120;

    let t134 = t97 ^ t116;
    let t135 = t131 ^ t134;
    let t136 = t93 ^ t115;

    let t138 = t119 ^ t132;
    let t140 = t114 ^ t136;

    let s0 = t109 ^ t122;
    let s2 = xnor(t123, t124);
    let s3 = t113 ^ t114;
    let s4 = t118 ^ t128;
    let s7 = xnor(t113, t125);
    let s6 = xnor(t109, t135);
    let s5 = t109 ^ t138;
    let s1 = xnor(t109, t140);

    st[0] = s7;
    st[1] = s6;
    st[2] = s5;
    st[3] = s4;
    st[4] = s3;
    st[5] = s2;
    st[6] = s1;
    st[7] = s0;
}

#[inline]
fn shift_row_u16(input: u16) -> u16 {
    (input & 0x1111)
        | ((input & 0x2220) >> 4)
        | ((input & 0x0002) << 12)
        | ((input & 0x4400) >> 8)
        | ((input & 0x0044) << 8)
        | ((input & 0x8000) >> 12)
        | ((input & 0x0888) << 4)
}

#[inline]
fn shift_rows_state(st: &mut State) {
    st[0] = shift_row_u16(st[0]);
    st[1] = shift_row_u16(st[1]);
    st[2] = shift_row_u16(st[2]);
    st[3] = shift_row_u16(st[3]);
    st[4] = shift_row_u16(st[4]);
    st[5] = shift_row_u16(st[5]);
    st[6] = shift_row_u16(st[6]);
    st[7] = shift_row_u16(st[7]);
}

#[inline]
fn mix_columns_state(st: &mut State) {
    let mut last_col: u16 = 0;

    for i in 0..8 {
        let col = st[i] ^ (((st[i] & 0xeeee) >> 1) | ((st[i] & 0x1111) << 3));
        st[i] = st[i] ^ last_col ^ col ^ (((col & 0xcccc) >> 2) | ((col & 0x3333) << 2));
        last_col = col;
    }

    st[0] ^= last_col;
    st[1] ^= last_col;
    st[3] ^= last_col;
    st[4] ^= last_col;
}

#[inline]
fn xor_key1_state(st: &mut State, k: &State) {
    st[0] ^= k[0];
    st[1] ^= k[1];
    st[2] ^= k[2];
    st[3] ^= k[3];
    st[4] ^= k[4];
    st[5] ^= k[5];
    st[6] ^= k[6];
    st[7] ^= k[7];
}

#[inline]
fn aes_enc(st: &mut State, key: &State) {
    sub_bytes_state(st);
    shift_rows_state(st);
    mix_columns_state(st);
    xor_key1_state(st, key)
}

#[inline]
fn aes_enc_last(st: &mut State, key: &State) {
    sub_bytes_state(st);
    shift_rows_state(st);
    xor_key1_state(st, key)
}

#[inline]
fn aes_keygen_assisti(rcon: u8, i: usize, u: u16) -> u16 {
    let u3 = u & 0xf000;
    let n = u3 >> 12;
    let n = ((n >> 1) | (n << 3)) & 0x000f;
    let ri = ((rcon >> i) & 1) as u16;
    let n = n ^ ri;
    let n = n << 12;
    n ^ (u3 >> 4)
}

#[inline]
fn aes_keygen_assist(next: &mut State, prev: &State, rcon: u8) {
    next.copy_from_slice(prev);
    sub_bytes_state(next);

    next[0] = aes_keygen_assisti(rcon, 0, next[0]);
    next[1] = aes_keygen_assisti(rcon, 1, next[1]);
    next[2] = aes_keygen_assisti(rcon, 2, next[2]);
    next[3] = aes_keygen_assisti(rcon, 3, next[3]);
    next[4] = aes_keygen_assisti(rcon, 4, next[4]);
    next[5] = aes_keygen_assisti(rcon, 5, next[5]);
    next[6] = aes_keygen_assisti(rcon, 6, next[6]);
    next[7] = aes_keygen_assisti(rcon, 7, next[7]);
}

#[inline]
fn aes_keygen_assist0(next: &mut State, prev: &State, rcon: u8) {
    aes_keygen_assist(next, prev, rcon);

    #[inline]
    fn aux(mut n: u16) -> u16 {
        n &= 0xf000;
        n ^= n >> 4;
        n ^= n >> 8;
        n
    }

    next[0] = aux(next[0]);
    next[1] = aux(next[1]);
    next[2] = aux(next[2]);
    next[3] = aux(next[3]);
    next[4] = aux(next[4]);
    next[5] = aux(next[5]);
    next[6] = aux(next[6]);
    next[7] = aux(next[7]);
}

#[inline]
fn aes_keygen_assist1(next: &mut State, prev: &State) {
    aes_keygen_assist(next, prev, 0);

    #[inline]
    fn aux(mut n: u16) -> u16 {
        n &= 0x0f00;
        n ^= n << 4;
        n ^= n >> 8;
        n
    }

    next[0] = aux(next[0]);
    next[1] = aux(next[1]);
    next[2] = aux(next[2]);
    next[3] = aux(next[3]);
    next[4] = aux(next[4]);
    next[5] = aux(next[5]);
    next[6] = aux(next[6]);
    next[7] = aux(next[7]);
}

#[inline]
fn key_expand1(p: u16, n: u16) -> u16 {
    let p = p ^ ((p & 0x0fff) << 4) ^ ((p & 0x00ff) << 8) ^ ((p & 0x000f) << 12);
    n ^ p
}

#[inline]
fn key_expansion_step(next: &mut State, prev: &State) {
    next[0] = key_expand1(prev[0], next[0]);
    next[1] = key_expand1(prev[1], next[1]);
    next[2] = key_expand1(prev[2], next[2]);
    next[3] = key_expand1(prev[3], next[3]);
    next[4] = key_expand1(prev[4], next[4]);
    next[5] = key_expand1(prev[5], next[5]);
    next[6] = key_expand1(prev[6], next[6]);
    next[7] = key_expand1(prev[7], next[7]);
}

impl crate::platform::AESState for State {
    #[inline]
    fn new() -> Self {
        new_state()
    }

    #[inline]
    fn load_block(&mut self, b: &[u8]) {
        debug_assert!(b.len() == 16);

        transpose_u8x16(b.try_into().unwrap(), self);
    }

    #[inline]
    fn store_block(&self, out: &mut [u8]) {
        debug_assert!(out.len() == AES_BLOCK_LEN, "out.len() = {}", out.len());

        transpose_u16x8(self, out);
    }

    #[inline]
    fn xor_block(&self, input: &[u8], out: &mut [u8]) {
        debug_assert!(input.len() == out.len() && input.len() <= AES_BLOCK_LEN);

        let mut block = [0u8; AES_BLOCK_LEN];
        self.store_block(&mut block);

        for i in 0..input.len() {
            out[i] = input[i] ^ block[i];
        }
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
        key_expansion_step(self, prev)
    }
}
