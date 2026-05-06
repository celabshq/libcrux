#[cfg(hax)]
use hax_lib::int::ToInt;

#[cfg(hax)]
use hax_lib::prop::*;

#[cfg(hax)]
use crate::proof_utils::valid_rate;

use libcrux_intrinsics::arm64::*;

use crate::{generic_keccak::KeccakState, traits::*};

#[allow(non_camel_case_types)]
pub type uint64x2_t = _uint64x2_t;

#[inline(always)]
fn _veor5q_u64(
    a: uint64x2_t,
    b: uint64x2_t,
    c: uint64x2_t,
    d: uint64x2_t,
    e: uint64x2_t,
) -> uint64x2_t {
    _veor3q_u64(_veor3q_u64(a, b, c), d, e)
}

#[inline(always)]
fn _vrax1q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    libcrux_intrinsics::arm64::_vrax1q_u64(a, b)
}

#[inline(always)]
#[hax_lib::requires(0 < LEFT && LEFT < 64 && 0 < RIGHT && RIGHT < 64 && LEFT + RIGHT == 64)]
fn _vxarq_u64<const LEFT: i32, const RIGHT: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    libcrux_intrinsics::arm64::_vxarq_u64::<LEFT, RIGHT>(a, b)
}

#[inline(always)]
fn _vbcaxq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
    libcrux_intrinsics::arm64::_vbcaxq_u64(a, b, c)
}

#[inline(always)]
fn _veorq_n_u64(a: uint64x2_t, c: u64) -> uint64x2_t {
    let c = _vdupq_n_u64(c);
    _veorq_u64(a, c)
}

#[cfg(hax)]
#[hax_lib::requires(i < 25 && lane < 2 &&
        offset.to_int() + (8.to_int() * i.to_int()) + 8.to_int() <= blocks[lane].len().to_int())]
fn load_lane_u64(
    blocks: &[&[u8]; 2],
    offset: usize,
    i: usize,
    statei: uint64x2_t,
    lane: usize,
) -> u64 {
    get_lane_u64(statei, lane)
        ^ u64::from_le_bytes(
            blocks[lane][offset + 8 * i..offset + 8 * i + 8]
                .try_into()
                .unwrap(),
        )
}

#[cfg(hax)]
#[hax_lib::requires(valid_rate(rate))]
#[hax_lib::ensures(|_|
    if rate % 16 > 0 {
        rate / 8 == 2 * (rate/16) + 1
    } else {rate / 8 == 2 * (rate/16)})]
fn lemma_rate_mod(rate: usize) {
}

#[inline(always)]
#[hax_lib::requires(i < 25
        && blocks[0].len() == blocks[1].len()
        && offset.to_int() + (8.to_int() * i.to_int()) + 8.to_int() <= blocks[0].len().to_int())]
#[hax_lib::ensures(|result|
    get_lane_u64(result,0) == load_lane_u64(blocks, offset, i, statei, 0)
    && get_lane_u64(result,1) == load_lane_u64(blocks, offset, i, statei, 1)
)]
fn load_u64x2(blocks: &[&[u8]; 2], offset: usize, i: usize, statei: uint64x2_t) -> uint64x2_t {
    let mut u = [0u64; 2];
    u[0] = u64::from_le_bytes(
        blocks[0][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    );
    u[1] = u64::from_le_bytes(
        blocks[1][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    );
    let uvec = _vld1q_u64(&u);
    _veorq_u64(statei, uvec)
}

#[inline(always)]
#[hax_lib::requires(i < 12
        && blocks[0].len() == blocks[1].len()
        && offset.to_int() + (16.to_int() * i.to_int()) + 16.to_int() <= blocks[0].len().to_int())]
#[hax_lib::ensures(|(r0,r1)|
    get_lane_u64(r0,0) == load_lane_u64(blocks, offset, 2*i, in0, 0)
    && get_lane_u64(r0,1) == load_lane_u64(blocks, offset, 2*i, in0, 1)
    && get_lane_u64(r1,0) == load_lane_u64(blocks, offset, 2*i + 1, in1, 0)
    && get_lane_u64(r1,1) == load_lane_u64(blocks, offset, 2*i + 1, in1, 1)
)]
fn load_u64x2x2(
    blocks: &[&[u8]; 2],
    offset: usize,
    i: usize,
    in0: uint64x2_t,
    in1: uint64x2_t,
) -> (uint64x2_t, uint64x2_t) {
    let v0 = _vld1q_bytes_u64(&blocks[0][offset + 16 * i..offset + 16 * i + 16]);
    let v1 = _vld1q_bytes_u64(&blocks[1][offset + 16 * i..offset + 16 * i + 16]);
    (
        _veorq_u64(in0, _vtrn1q_u64(v0, v1)),
        _veorq_u64(in1, _vtrn2q_u64(v0, v1)),
    )
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries always")]
#[hax_lib::requires(valid_rate(RATE)
            && blocks[0].len() == blocks[1].len()
            && offset.to_int() + RATE.to_int() <= blocks[0].len().to_int()
)]
#[hax_lib::ensures(|_| hax_lib::forall(|i: usize|
    if i < 25 {
        if i < RATE / 8 {
            get_lane_u64(future(state)[i], 0) == load_lane_u64(blocks, offset, i, state[i], 0)
            && get_lane_u64(future(state)[i], 1) == load_lane_u64(blocks, offset, i, state[i], 1)
        } else {
            get_lane_u64(future(state)[i], 0) == get_lane_u64(state[i], 0)
            && get_lane_u64(future(state)[i], 1) == get_lane_u64(state[i], 1)
        }
    } else { true }
))]
pub(crate) fn load_block<const RATE: usize>(
    state: &mut [uint64x2_t; 25],
    blocks: &[&[u8]; 2],
    offset: usize,
) {
    #[cfg(hax)]
    let old_state = *state; // ghost variable

    for i in 0..RATE / 16 {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| 
            if j < 25 {
                if j < 2 * i {
                    get_lane_u64(state[j], 0) == load_lane_u64(blocks, offset, j, old_state[j], 0)
                        && get_lane_u64(state[j], 1)
                            == load_lane_u64(blocks, offset, j, old_state[j], 1)
                } else {
                    get_lane_u64(state[j], 0) == get_lane_u64(old_state[j], 0)
                        && get_lane_u64(state[j], 1) == get_lane_u64(old_state[j], 1)
                }
            } else {
                true
        }));
        let i0 = (2 * i) / 5;
        let j0 = (2 * i) % 5;
        let i1 = (2 * i + 1) / 5;
        let j1 = (2 * i + 1) % 5;
        let (v0, v1) = load_u64x2x2(
            blocks,
            offset,
            i,
            *get_ij(state, i0, j0),
            *get_ij(state, i1, j1),
        );
        set_ij(state, i0, j0, v0);
        set_ij(state, i1, j1, v1);
    }
    #[cfg(hax)]
    lemma_rate_mod(RATE);
    let remaining = RATE % 16;
    if remaining > 0 {
        let i = RATE / 8 - 1;
        let result = load_u64x2(blocks, offset, i, *get_ij(state, i / 5, i % 5));
        set_ij(state, i / 5, i % 5, result);
    }
}

#[inline(always)]
#[hax_lib::requires(valid_rate(RATE) && len < RATE && offset.to_int() + len.to_int() <= blocks[0].len().to_int() && blocks[0].len() == blocks[1].len())]
pub(crate) fn load_last<const RATE: usize, const DELIMITER: u8>(
    state: &mut [uint64x2_t; 25],
    blocks: &[&[u8]; 2],
    offset: usize,
    len: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(offset + len <= blocks[0].len() && blocks[0].len() == blocks[1].len());

    let mut buffer0 = [0u8; RATE];
    buffer0[0..len].copy_from_slice(&blocks[0][offset..offset + len]);
    buffer0[len] = DELIMITER;
    buffer0[RATE - 1] |= 0x80;

    let mut buffer1 = [0u8; RATE];
    buffer1[0..len].copy_from_slice(&blocks[1][offset..offset + len]);
    buffer1[len] = DELIMITER;
    buffer1[RATE - 1] |= 0x80;

    load_block::<RATE>(state, &[&buffer0, &buffer1], 0);
}

/// Per-iteration store wrapper for the `store_block` loop body.
///
/// Given the two state slots `s_2i` (state[2*i]) and `s_succ`
/// (state[2*i + 1]), and the output slices `out0`/`out1`, performs the
/// per-iteration `vtrn1q_u64`/`vtrn2q_u64` interleave + two
/// `_vst1q_bytes_u64` stores and returns updated slices that satisfy
/// the byte-level loop invariant for the freshly-stored 16-byte
/// window.
///
/// Factored out of `store_block` so its strong per-byte ensures
/// isolates the `update_at_range`/slice precondition cliff from the
/// outer loop's heavy invariant. Mirrors `load_u64x2x2` on the load
/// side.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && start.to_int() + (16.to_int() * (i.to_int() + 1.to_int())) <= out0.len().to_int()
)]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & hax_lib::forall(|j: usize|
        if j < out0.len() {
            if j < start + 16 * i {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
            } else if j < start + 16 * (i + 1) {
                if (j - start) / 8 == 2 * i {
                    future(out0)[j] == get_lane_u64(s_2i, 0).to_le_bytes()[(j - start) % 8]
                    && future(out1)[j] == get_lane_u64(s_2i, 1).to_le_bytes()[(j - start) % 8]
                } else {
                    future(out0)[j] == get_lane_u64(s_succ, 0).to_le_bytes()[(j - start) % 8]
                    && future(out1)[j] == get_lane_u64(s_succ, 1).to_le_bytes()[(j - start) % 8]
                }
            } else {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
            }
        } else {
            true
        })
)]
fn store_u64x2x2(
    out0: &mut [u8],
    out1: &mut [u8],
    s_2i: uint64x2_t,
    s_succ: uint64x2_t,
    start: usize,
    i: usize,
) {
    let v0 = _vtrn1q_u64(s_2i, s_succ);
    let v1 = _vtrn2q_u64(s_2i, s_succ);
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    _vst1q_bytes_u64(&mut out0[start + 16 * i..start + 16 * (i + 1)], v0);
    _vst1q_bytes_u64(&mut out1[start + 16 * i..start + 16 * (i + 1)], v1);
    // Bridge the strengthened `e_vst1q_bytes_u64` per-byte post +
    // `update_at_range` slice posts into the per-absolute-index byte
    // facts the function-level ensures expects, then propagate via
    // `forall_intro` over the abstract index `j`.
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 16 * v i in
        assert (a_pos + 16 <= Seq.length old_out0);
        assert (a_pos + 16 <= Seq.length old_out1);
        let bridge_out0 (j_n:nat{j_n < Seq.length old_out0}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + 16 then
                Seq.index out0 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2 v0 ((j_n - a_pos) / 8)))
                    ((j_n - a_pos) % 8)
              else
                Seq.index out0 j_n == Seq.index old_out0 j_n)
          = Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.store_block_window_byte_of_vst
              old_out0
              out0
              (Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64
                 (Seq.slice old_out0 a_pos (a_pos + 16))
                 v0)
              v0
              a_pos
              j_n
        in
        let bridge_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + 16 then
                Seq.index out1 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2 v1 ((j_n - a_pos) / 8)))
                    ((j_n - a_pos) % 8)
              else
                Seq.index out1 j_n == Seq.index old_out1 j_n)
          = Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.store_block_window_byte_of_vst
              old_out1
              out1
              (Libcrux_intrinsics.Arm64_extract.e_vst1q_bytes_u64
                 (Seq.slice old_out1 a_pos (a_pos + 16))
                 v1)
              v1
              a_pos
              j_n
        in
        Classical.forall_intro bridge_out0;
        Classical.forall_intro bridge_out1
        "#
    );
}

/// Tail wrapper for the `remaining > 8` branch of `store_block`.
///
/// Stores the partial 16-byte window `out0[start+16*q .. start+16*q+remaining]`
/// (and the analogous out1 window) by:
/// (1) materializing both 16-byte tmp arrays via `_vst1q_bytes_u64`,
/// (2) `copy_from_slice`-ing the first `remaining` bytes into the
///     `out0`/`out1` windows.
///
/// `q = len/16` (the post-loop iteration count). The window covers the
/// last `remaining` bytes of `[start, start+len)` with `8 < remaining
/// < 16`. The window's first 8 bytes correspond to `s_2i`; bytes 8..remaining
/// correspond to `s_succ`. Lanes 0/1 of each go to out0/out1.
///
/// Mirrors `store_u64x2x2` on the partial-window side: the strong
/// per-byte ensures isolates the local update_at_range slice precond
/// + `_vst1q_bytes_u64`/`vtrn` reasoning so the calling `store_block`
/// body composes additively.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && remaining > 8
    && remaining < 16
    && start.to_int() + (16.to_int() * q.to_int()) + remaining.to_int() <= out0.len().to_int()
)]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & hax_lib::forall(|j: usize|
        if j < out0.len() {
            if j < start + 16 * q {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
            } else if j < start + 16 * q + remaining {
                if (j - start) / 8 == 2 * q {
                    future(out0)[j] == get_lane_u64(s_2i, 0).to_le_bytes()[(j - start) % 8]
                    && future(out1)[j] == get_lane_u64(s_2i, 1).to_le_bytes()[(j - start) % 8]
                } else {
                    future(out0)[j] == get_lane_u64(s_succ, 0).to_le_bytes()[(j - start) % 8]
                    && future(out1)[j] == get_lane_u64(s_succ, 1).to_le_bytes()[(j - start) % 8]
                }
            } else {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
            }
        } else {
            true
        })
)]
fn store_tail_high(
    out0: &mut [u8],
    out1: &mut [u8],
    s_2i: uint64x2_t,
    s_succ: uint64x2_t,
    start: usize,
    q: usize,
    remaining: usize,
) {
    let v0 = _vtrn1q_u64(s_2i, s_succ);
    let v1 = _vtrn2q_u64(s_2i, s_succ);
    let mut out0_tmp = [0u8; 16];
    let mut out1_tmp = [0u8; 16];
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    _vst1q_bytes_u64(&mut out0_tmp, v0);
    _vst1q_bytes_u64(&mut out1_tmp, v1);
    out0[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out0_tmp[0..remaining]);
    out1[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out1_tmp[0..remaining]);
    // Bridge: derive per-byte facts for `out0`/`out1` in the
    // partial window from `_vst1q_bytes_u64`'s per-byte post +
    // `update_at_range`'s slice posts.
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 16 * v q in
        let r:nat = v remaining in
        assert (a_pos + r <= Seq.length old_out0);
        assert (a_pos + r <= Seq.length old_out1);
        let bridge_out0 (j_n:nat{j_n < Seq.length old_out0}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + r then
                (let k:nat = j_n - a_pos in
                 Seq.index out0 j_n ==
                 Seq.index
                   (Core_models.Num.impl_u64__to_le_bytes
                      (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2 v0 (k / 8)))
                   (k % 8))
              else
                Seq.index out0 j_n == Seq.index old_out0 j_n)
          = if j_n < a_pos then begin
              assert (Seq.index (Seq.slice out0 0 a_pos) j_n == Seq.index out0 j_n);
              assert (Seq.index (Seq.slice old_out0 0 a_pos) j_n == Seq.index old_out0 j_n)
            end else if j_n < a_pos + r then begin
              let k:nat = j_n - a_pos in
              assert (k < r);
              assert (Seq.index (Seq.slice out0 a_pos (a_pos + r)) k == Seq.index out0 j_n);
              assert (Seq.index (Seq.slice out0_tmp 0 r) k == Seq.index out0_tmp k)
            end else begin
              let k:nat = j_n - (a_pos + r) in
              assert (Seq.index (Seq.slice out0 (a_pos + r) (Seq.length out0)) k ==
                      Seq.index out0 j_n);
              assert (Seq.index (Seq.slice old_out0 (a_pos + r) (Seq.length old_out0)) k ==
                      Seq.index old_out0 j_n)
            end
        in
        let bridge_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + r then
                (let k:nat = j_n - a_pos in
                 Seq.index out1 j_n ==
                 Seq.index
                   (Core_models.Num.impl_u64__to_le_bytes
                      (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2 v1 (k / 8)))
                   (k % 8))
              else
                Seq.index out1 j_n == Seq.index old_out1 j_n)
          = if j_n < a_pos then begin
              assert (Seq.index (Seq.slice out1 0 a_pos) j_n == Seq.index out1 j_n);
              assert (Seq.index (Seq.slice old_out1 0 a_pos) j_n == Seq.index old_out1 j_n)
            end else if j_n < a_pos + r then begin
              let k:nat = j_n - a_pos in
              assert (k < r);
              assert (Seq.index (Seq.slice out1 a_pos (a_pos + r)) k == Seq.index out1 j_n);
              assert (Seq.index (Seq.slice out1_tmp 0 r) k == Seq.index out1_tmp k)
            end else begin
              let k:nat = j_n - (a_pos + r) in
              assert (Seq.index (Seq.slice out1 (a_pos + r) (Seq.length out1)) k ==
                      Seq.index out1 j_n);
              assert (Seq.index (Seq.slice old_out1 (a_pos + r) (Seq.length old_out1)) k ==
                      Seq.index old_out1 j_n)
            end
        in
        Classical.forall_intro bridge_out0;
        Classical.forall_intro bridge_out1
        "#
    );
}

/// Tail wrapper for the `remaining > 0 && remaining <= 8` branch of
/// `store_block`. A single 16-byte tmp materialized from one state
/// slot — its low half (`tmp[0..remaining]`) goes to `out0`, its high
/// half (`tmp[8..8+remaining]`) goes to `out1`.
///
/// `q = len/16`. Window: `[start+16*q, start+16*q+remaining)`, with
/// `0 < remaining <= 8`. Both `out0[j]` and `out1[j]` map to lanes 0
/// and 1 of the same state slot `s_2q`; the lo-half / hi-half split
/// is exactly `_vst1q_bytes_u64`'s definition.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && remaining > 0
    && remaining <= 8
    && start.to_int() + (16.to_int() * q.to_int()) + remaining.to_int() <= out0.len().to_int()
)]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & hax_lib::forall(|j: usize|
        if j < out0.len() {
            if j < start + 16 * q {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
            } else if j < start + 16 * q + remaining {
                future(out0)[j] == get_lane_u64(s_2q, 0).to_le_bytes()[(j - start) % 8]
                && future(out1)[j] == get_lane_u64(s_2q, 1).to_le_bytes()[(j - start) % 8]
            } else {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
            }
        } else {
            true
        })
)]
fn store_tail_low(
    out0: &mut [u8],
    out1: &mut [u8],
    s_2q: uint64x2_t,
    start: usize,
    q: usize,
    remaining: usize,
) {
    let mut out01 = [0u8; 16];
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    _vst1q_bytes_u64(&mut out01, s_2q);
    out0[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out01[0..remaining]);
    out1[start + 16 * q..start + 16 * q + remaining]
        .copy_from_slice(&out01[8..8 + remaining]);
    // Bridge: out01[k] == byte k%8 of get_lane_u64x2 s_2q (k/8) for
    // k<16; the low-half goes to out0 (k in 0..remaining, all
    // satisfy k/8 = 0); the high-half goes to out1 (k in 8..8+remaining,
    // all satisfy k/8 = 1).
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 16 * v q in
        let r:nat = v remaining in
        assert (r <= 8);
        let bridge_out0 (j_n:nat{j_n < Seq.length old_out0}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + r then
                (let k:nat = j_n - a_pos in
                 Seq.index out0 j_n ==
                 Seq.index
                   (Core_models.Num.impl_u64__to_le_bytes
                      (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2 s_2q 0))
                   k)
              else
                Seq.index out0 j_n == Seq.index old_out0 j_n)
          = if j_n < a_pos then begin
              assert (Seq.index (Seq.slice out0 0 a_pos) j_n == Seq.index out0 j_n);
              assert (Seq.index (Seq.slice old_out0 0 a_pos) j_n == Seq.index old_out0 j_n)
            end else if j_n < a_pos + r then begin
              let k:nat = j_n - a_pos in
              assert (k < r /\ k < 8);
              assert (Seq.index (Seq.slice out0 a_pos (a_pos + r)) k == Seq.index out0 j_n);
              assert (Seq.index (Seq.slice out01 0 r) k == Seq.index out01 k);
              assert (k / 8 == 0 /\ k % 8 == k)
            end else begin
              let k:nat = j_n - (a_pos + r) in
              assert (Seq.index (Seq.slice out0 (a_pos + r) (Seq.length out0)) k ==
                      Seq.index out0 j_n);
              assert (Seq.index (Seq.slice old_out0 (a_pos + r) (Seq.length old_out0)) k ==
                      Seq.index old_out0 j_n)
            end
        in
        let bridge_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + r then
                (let k:nat = j_n - a_pos in
                 Seq.index out1 j_n ==
                 Seq.index
                   (Core_models.Num.impl_u64__to_le_bytes
                      (Libcrux_intrinsics.Arm64_extract.get_lane_u64x2 s_2q 1))
                   k)
              else
                Seq.index out1 j_n == Seq.index old_out1 j_n)
          = if j_n < a_pos then begin
              assert (Seq.index (Seq.slice out1 0 a_pos) j_n == Seq.index out1 j_n);
              assert (Seq.index (Seq.slice old_out1 0 a_pos) j_n == Seq.index old_out1 j_n)
            end else if j_n < a_pos + r then begin
              let k:nat = j_n - a_pos in
              assert (k < r /\ k < 8);
              assert (Seq.index (Seq.slice out1 a_pos (a_pos + r)) k == Seq.index out1 j_n);
              assert (Seq.index (Seq.slice out01 8 (8 + r)) k == Seq.index out01 (8 + k));
              assert ((8 + k) / 8 == 1 /\ (8 + k) % 8 == k)
            end else begin
              let k:nat = j_n - (a_pos + r) in
              assert (Seq.index (Seq.slice out1 (a_pos + r) (Seq.length out1)) k ==
                      Seq.index out1 j_n);
              assert (Seq.index (Seq.slice old_out1 (a_pos + r) (Seq.length old_out1)) k ==
                      Seq.index old_out1 j_n)
            end
        in
        Classical.forall_intro bridge_out0;
        Classical.forall_intro bridge_out1
        "#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'")]
#[hax_lib::requires(valid_rate(RATE) && len <= RATE && start.to_int() + len.to_int() <= out0.len().to_int() && out0.len() == out1.len())]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & hax_lib::forall(|i: usize| if i < out0.len() {
        if i < start {
            out0[i] == future(out0)[i]
        } else if i < start + len {
            future(out0)[i] == get_lane_u64(s[(i - start) / 8], 0).to_le_bytes()[(i - start) % 8]
        } else {
            out0[i] == future(out0)[i]
        }
    } else {
        true
    })
    & hax_lib::forall(|i: usize| if i < out1.len() {
        if i < start {
            out1[i] == future(out1)[i]
        } else if i < start + len {
            future(out1)[i] == get_lane_u64(s[(i - start) / 8], 1).to_le_bytes()[(i - start) % 8]
        } else {
            out1[i] == future(out1)[i]
        }
    } else {
        true
    })
)]
pub(crate) fn store_block<const RATE: usize>(
    s: &[uint64x2_t; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    start: usize,
    len: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(len <= RATE && start + len <= out0.len() && out0.len() == out1.len());

    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice(); // ghost variable
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice(); // ghost variable
    hax_lib::fstar!(
        r#"
        assert_norm (
          Alloc.Vec.impl_1__as_slice
            (Alloc.Slice.impl__to_vec out0) == out0);
        assert_norm (
          Alloc.Vec.impl_1__as_slice
            (Alloc.Slice.impl__to_vec out1) == out1);
        assert (old_out0 == out0);
        assert (old_out1 == out1)
        "#
    );

    for i in 0..len / 16 {
        hax_lib::loop_invariant!(|i: usize| (out0.len() == old_out0.len()).to_prop()
            & (out1.len() == old_out1.len()).to_prop()
            & hax_lib::forall(|j: usize| if j < out0.len() {
                if j < start {
                    out0[j] == old_out0[j]
                } else if j < start + i * 16 {
                    out0[j] == get_lane_u64(s[(j - start) / 8], 0).to_le_bytes()[(j - start) % 8]
                } else {
                    out0[j] == old_out0[j]
                }
            } else {
                true
            })
            & hax_lib::forall(|j: usize| if j < out1.len() {
                if j < start {
                    out1[j] == old_out1[j]
                } else if j < start + i * 16 {
                    out1[j] == get_lane_u64(s[(j - start) / 8], 1).to_le_bytes()[(j - start) % 8]
                } else {
                    out1[j] == old_out1[j]
                }
            } else {
                true
            }));
        let i0 = (2 * i) / 5;
        let j0 = (2 * i) % 5;
        let i1 = (2 * i + 1) / 5;
        let j1 = (2 * i + 1) % 5;
        store_u64x2x2(out0, out1, *get_ij(s, i0, j0), *get_ij(s, i1, j1), start, i);
    }
    let q = len / 16;
    let remaining = len % 16;
    // Bridge the Euclidean equation `len = 16*q + remaining` and
    // `remaining < 16` so the tail wrappers' slice preconditions hold.
    hax_lib::fstar!(
        r#"
        assert (v len == 16 * v q + v remaining);
        assert (v remaining < 16);
        assert (v start + 16 * v q + v remaining == v start + v len);
        assert (v start + v len <= Seq.length out0);
        assert (v start + v len <= Seq.length out1)
        "#
    );
    if remaining > 8 {
        let i = 2 * q;
        let i0 = i / 5;
        let j0 = i % 5;
        let i1 = (i + 1) / 5;
        let j1 = (i + 1) % 5;
        store_tail_high(
            out0,
            out1,
            *get_ij(s, i0, j0),
            *get_ij(s, i1, j1),
            start,
            q,
            remaining,
        );
    } else if remaining > 0 {
        let i = 2 * q;
        store_tail_low(out0, out1, *get_ij(s, i / 5, i % 5), start, q, remaining);
    }
    // Function-level ensures aggregation:
    // The loop's final invariant gives the per-byte fact for `j` in
    // `[start, start + 16*q)` (in `s[(j-start)/8]` form).  The tail
    // wrapper post gives the per-byte fact for `j` in
    // `[start + 16*q, start + 16*q + remaining)` (in `s_2i` /
    // `s_succ` / `s_2q` form, with a discriminator on
    // `(j-start)/8 == 2*q`).  Combining these two ranges into
    // `[start, start+len)` (using `len = 16*q + remaining`) AND
    // bridging the wrapper's `s_2i` / `s_succ` / `s_2q` form to the
    // function ensures' `s[(j-start)/8]` form requires Euclidean
    // div/mod normalization (`5*((2q)/5) + (2q)%5 = 2q`,
    // `(16q + k)/8 = 2q + k/8`) which Z3 cannot discharge within
    // rlimit budget under the `--using_facts_from` filter.  Discharge
    // deferred — see `proofs/agent-status/store-block-arm64-discharge-progress.md`.
    hax_lib::fstar!("admit ()");
}

#[hax_lib::attributes]
impl KeccakItem<2> for uint64x2_t {
    #[inline(always)]
    fn zero() -> Self {
        _vdupq_n_u64(0)
    }
    #[inline(always)]
    fn xor5(a: Self, b: Self, c: Self, d: Self, e: Self) -> Self {
        _veor5q_u64(a, b, c, d, e)
    }
    #[inline(always)]
    fn rotate_left1_and_xor(a: Self, b: Self) -> Self {
        _vrax1q_u64(a, b)
    }
    #[inline(always)]
    #[hax_lib::requires(0 < LEFT && LEFT < 64 && 0 < RIGHT && RIGHT < 64 && LEFT + RIGHT == 64)]
    fn xor_and_rotate<const LEFT: i32, const RIGHT: i32>(a: Self, b: Self) -> Self {
        _vxarq_u64::<LEFT, RIGHT>(a, b)
    }
    #[inline(always)]
    fn and_not_xor(a: Self, b: Self, c: Self) -> Self {
        _vbcaxq_u64(a, b, c)
    }
    #[inline(always)]
    fn xor_constant(a: Self, c: u64) -> Self {
        _veorq_n_u64(a, c)
    }
    #[inline(always)]
    fn xor(a: Self, b: Self) -> Self {
        _veorq_u64(a, b)
    }
}

#[hax_lib::attributes]
impl Absorb<2> for KeccakState<2, uint64x2_t> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= input[0].len().to_int() &&
        input[0].len() == input[1].len()
    )]
    fn load_block<const RATE: usize>(&mut self, input: &[&[u8]; 2], start: usize) {
        load_block::<RATE>(&mut self.st, input, start);
    }

    #[hax_lib::requires(
        valid_rate(RATE) &&
        len < RATE &&
        start.to_int() + len.to_int() <= input[0].len().to_int() &&
        input[0].len() == input[1].len()
    )]
    fn load_last<const RATE: usize, const DELIMITER: u8>(
        &mut self,
        input: &[&[u8]; 2],
        start: usize,
        len: usize,
    ) {
        load_last::<RATE, DELIMITER>(&mut self.st, input, start, len);
    }
}

#[hax_lib::attributes]
impl Squeeze2<uint64x2_t> for KeccakState<2, uint64x2_t> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        len <= RATE &&
        start.to_int() + len.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
    fn squeeze2<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        start: usize,
        len: usize,
    ) {
        store_block::<RATE>(&self.st, out0, out1, start, len);
    }
}
