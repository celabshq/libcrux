#[cfg(hax)]
use hax_lib::int::ToInt;

#[cfg(hax)]
use crate::proof_utils::valid_rate;

use libcrux_intrinsics::arm64::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, set_ij, Absorb};

use super::wrappers::uint64x2_t;

/// Spec function: per-lane semantics of "XOR state element with 8
/// bytes from input block".
//
// Made opaque-to-SMT to suppress body-unfolding cascades in
// `load_block` proof. The functional dependence on `statei` (only via
// `get_lane_u64 statei lane`) is exposed to Z3 via the SMTPat
// extensionality lemma `load_lane_u64_lane_extensionality` injected
// via `fstar::after`, which lets the loop_invariant's per-lane
// equality (provided by `get_lane_u64`) bridge to per-`load_lane_u64`
// equality without unfolding the body. Mirrors the AVX2 cascade
// closure (commit 3b9fc054c).
#[cfg(hax)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::after(
    interface,
    r#"
val load_lane_u64_lane_extensionality
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset i: usize)
      (s1 s2: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (lane: usize)
  : Lemma
    (requires
      (i <. mk_usize 25 && lane <. mk_usize 2 &&
       (((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
           ((Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int) *
             (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
             <:
             Hax_lib.Int.t_Int)
           <:
           Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int)
         <:
         Hax_lib.Int.t_Int) <=
       (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
               (blocks.[ lane ] <: t_Slice u8)
             <:
             usize)
         <:
         Hax_lib.Int.t_Int)) /\
      Libcrux_intrinsics.Arm64_extract.get_lane_u64 s1 lane ==
      Libcrux_intrinsics.Arm64_extract.get_lane_u64 s2 lane)
    (ensures
      load_lane_u64 blocks offset i s1 lane ==
      load_lane_u64 blocks offset i s2 lane)
    [SMTPat (load_lane_u64 blocks offset i s1 lane);
     SMTPat (load_lane_u64 blocks offset i s2 lane)]
"#
)]
#[hax_lib::fstar::after(
    r#"
let load_lane_u64_lane_extensionality
      (blocks: t_Array (t_Slice u8) (mk_usize 2))
      (offset i: usize)
      (s1 s2: Libcrux_intrinsics.Arm64_extract.t_e_uint64x2_t)
      (lane: usize)
  : Lemma
    (requires
      (i <. mk_usize 25 && lane <. mk_usize 2 &&
       (((Rust_primitives.Hax.Int.from_machine offset <: Hax_lib.Int.t_Int) +
           ((Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int) *
             (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
             <:
             Hax_lib.Int.t_Int)
           <:
           Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine (mk_i32 8) <: Hax_lib.Int.t_Int)
         <:
         Hax_lib.Int.t_Int) <=
       (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
               (blocks.[ lane ] <: t_Slice u8)
             <:
             usize)
         <:
         Hax_lib.Int.t_Int)) /\
      Libcrux_intrinsics.Arm64_extract.get_lane_u64 s1 lane ==
      Libcrux_intrinsics.Arm64_extract.get_lane_u64 s2 lane)
    (ensures
      load_lane_u64 blocks offset i s1 lane ==
      load_lane_u64 blocks offset i s2 lane)
    [SMTPat (load_lane_u64 blocks offset i s1 lane);
     SMTPat (load_lane_u64 blocks offset i s2 lane)]
  = reveal_opaque (`%load_lane_u64) load_lane_u64
"#
)]
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
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
fn load_u64x2(blocks: &[&[u8]; 2], offset: usize, i: usize, statei: uint64x2_t) -> uint64x2_t {
    // load_lane_u64 is opaque-to-SMT; reveal here so the body can prove its ensures.
    hax_lib::fstar!(r#"reveal_opaque (`%load_lane_u64) load_lane_u64"#);
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
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(i < 12
        && blocks[0].len() == blocks[1].len()
        && offset.to_int() + (16.to_int() * i.to_int()) + 16.to_int() <= blocks[0].len().to_int())]
#[hax_lib::ensures(|(r0,r1)|
    get_lane_u64(r0,0) == load_lane_u64(blocks, offset, 2*i, in0, 0)
    && get_lane_u64(r0,1) == load_lane_u64(blocks, offset, 2*i, in0, 1)
    && get_lane_u64(r1,0) == load_lane_u64(blocks, offset, 2*i + 1, in1, 0)
    && get_lane_u64(r1,1) == load_lane_u64(blocks, offset, 2*i + 1, in1, 1)
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
fn load_u64x2x2(
    blocks: &[&[u8]; 2],
    offset: usize,
    i: usize,
    in0: uint64x2_t,
    in1: uint64x2_t,
) -> (uint64x2_t, uint64x2_t) {
    // load_lane_u64 is opaque-to-SMT; reveal here so the body can prove its ensures.
    hax_lib::fstar!(r#"reveal_opaque (`%load_lane_u64) load_lane_u64"#);
    let v0 = _vld1q_bytes_u64(&blocks[0][offset + 16 * i..offset + 16 * i + 16]);
    let v1 = _vld1q_bytes_u64(&blocks[1][offset + 16 * i..offset + 16 * i + 16]);
    (
        _veorq_u64(in0, _vtrn1q_u64(v0, v1)),
        _veorq_u64(in1, _vtrn2q_u64(v0, v1)),
    )
}

// Cliff closure 2026-05-07: opacified `load_lane_u64` + `load_u64x2`
// + `load_u64x2x2` and added `load_lane_u64_lane_extensionality`
// SMTPat lemma. Mirrors the AVX2 cascade closure (commits 7bb581f8b
// .. 3b9fc054c) which discharged the same `k!61` /
// `Rust_primitives.Slice.array_from_fn` cascade at q301.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'")]
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
