#[cfg(hax)]
use hax_lib::int::ToInt;

#[cfg(hax)]
use crate::proof_utils::valid_rate;

use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, set_ij, Absorb};

/// Spec function (mirrors arm64::load_lane_u64 at N=4): per-lane
/// semantics of "XOR state element with 8 bytes from input block".
//
// Made opaque-to-SMT to suppress body-unfolding cascades in
// `load_block` proof. The functional dependence on `statei` (only
// via `get_lane_u64 statei lane`) is exposed to Z3 via the SMTPat
// extensionality lemma `load_lane_u64_lane_extensionality` injected
// via `fstar::after`, which lets the loop_invariant's per-lane
// equality (provided by `get_lane_u64`) bridge to per-`load_lane_u64`
// equality without unfolding the body.
#[cfg(hax)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::after(
    interface,
    r#"
val load_lane_u64_lane_extensionality
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (s1 s2: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (lane: usize)
  : Lemma
    (requires
      (i <. mk_usize 25 && lane <. mk_usize 4 &&
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
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s1 lane ==
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 lane)
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
      (blocks: t_Array (t_Slice u8) (mk_usize 4))
      (offset i: usize)
      (s1 s2: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (lane: usize)
  : Lemma
    (requires
      (i <. mk_usize 25 && lane <. mk_usize 4 &&
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
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s1 lane ==
      Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 lane)
    (ensures
      load_lane_u64 blocks offset i s1 lane ==
      load_lane_u64 blocks offset i s2 lane)
    [SMTPat (load_lane_u64 blocks offset i s1 lane);
     SMTPat (load_lane_u64 blocks offset i s2 lane)]
  = reveal_opaque (`%load_lane_u64) load_lane_u64
"#
)]
#[hax_lib::requires(i < 25 && lane < 4 &&
        offset.to_int() + (8.to_int() * i.to_int()) + 8.to_int() <= blocks[lane].len().to_int())]
fn load_lane_u64(
    blocks: &[&[u8]; 4],
    offset: usize,
    i: usize,
    statei: Vec256,
    lane: usize,
) -> u64 {
    get_lane_u64(statei, lane)
        ^ u64::from_le_bytes(
            blocks[lane][offset + 8 * i..offset + 8 * i + 8]
                .try_into()
                .unwrap(),
        )
}

/// Bulk-block load helper (mirrors arm64::load_u64x2x2 at N=4).
/// Loads 32 bytes from each of the 4 blocks at `offset + 32*i`,
/// gathers them via unpack/permute into 4 Vec256s, each holding the
/// `(4*i + idx)`th u64 from each block in lane `lane`, then XORs
/// with the corresponding state inputs `inK`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(i < 6
        && blocks[0].len() == blocks[1].len()
        && blocks[0].len() == blocks[2].len()
        && blocks[0].len() == blocks[3].len()
        && offset.to_int() + (32.to_int() * i.to_int()) + 32.to_int() <= blocks[0].len().to_int())]
#[hax_lib::ensures(|(r0, r1, r2, r3)|
    get_lane_u64(r0, 0) == load_lane_u64(blocks, offset, 4*i, in0, 0)
    && get_lane_u64(r0, 1) == load_lane_u64(blocks, offset, 4*i, in0, 1)
    && get_lane_u64(r0, 2) == load_lane_u64(blocks, offset, 4*i, in0, 2)
    && get_lane_u64(r0, 3) == load_lane_u64(blocks, offset, 4*i, in0, 3)
    && get_lane_u64(r1, 0) == load_lane_u64(blocks, offset, 4*i + 1, in1, 0)
    && get_lane_u64(r1, 1) == load_lane_u64(blocks, offset, 4*i + 1, in1, 1)
    && get_lane_u64(r1, 2) == load_lane_u64(blocks, offset, 4*i + 1, in1, 2)
    && get_lane_u64(r1, 3) == load_lane_u64(blocks, offset, 4*i + 1, in1, 3)
    && get_lane_u64(r2, 0) == load_lane_u64(blocks, offset, 4*i + 2, in2, 0)
    && get_lane_u64(r2, 1) == load_lane_u64(blocks, offset, 4*i + 2, in2, 1)
    && get_lane_u64(r2, 2) == load_lane_u64(blocks, offset, 4*i + 2, in2, 2)
    && get_lane_u64(r2, 3) == load_lane_u64(blocks, offset, 4*i + 2, in2, 3)
    && get_lane_u64(r3, 0) == load_lane_u64(blocks, offset, 4*i + 3, in3, 0)
    && get_lane_u64(r3, 1) == load_lane_u64(blocks, offset, 4*i + 3, in3, 1)
    && get_lane_u64(r3, 2) == load_lane_u64(blocks, offset, 4*i + 3, in3, 2)
    && get_lane_u64(r3, 3) == load_lane_u64(blocks, offset, 4*i + 3, in3, 3)
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
fn load_u64x4x4(
    blocks: &[&[u8]; 4],
    offset: usize,
    i: usize,
    in0: Vec256,
    in1: Vec256,
    in2: Vec256,
    in3: Vec256,
) -> (Vec256, Vec256, Vec256, Vec256) {
    // load_lane_u64 is opaque-to-SMT (to suppress cascade in load_block);
    // reveal it here so this body can prove its ensures.
    hax_lib::fstar!(r#"reveal_opaque (`%load_lane_u64) load_lane_u64"#);
    let start = offset + 32 * i;
    let v0 = mm256_loadu_si256_u8(&blocks[0][start..start + 32]);
    let v1 = mm256_loadu_si256_u8(&blocks[1][start..start + 32]);
    let v2 = mm256_loadu_si256_u8(&blocks[2][start..start + 32]);
    let v3 = mm256_loadu_si256_u8(&blocks[3][start..start + 32]);

    let v0l = mm256_unpacklo_epi64(v0, v1);
    let v1h = mm256_unpackhi_epi64(v0, v1);
    let v2l = mm256_unpacklo_epi64(v2, v3);
    let v3h = mm256_unpackhi_epi64(v2, v3);

    let g0 = mm256_permute2x128_si256::<0x20>(v0l, v2l);
    let g1 = mm256_permute2x128_si256::<0x20>(v1h, v3h);
    let g2 = mm256_permute2x128_si256::<0x31>(v0l, v2l);
    let g3 = mm256_permute2x128_si256::<0x31>(v1h, v3h);

    (
        mm256_xor_si256(in0, g0),
        mm256_xor_si256(in1, g1),
        mm256_xor_si256(in2, g2),
        mm256_xor_si256(in3, g3),
    )
}

/// Partial-block load helper (mirrors arm64::load_u64x2 at N=4).
/// Loads 8 bytes from each of the 4 blocks at `offset + 8*i`,
/// gathers them into a Vec256, and XORs with `statei`.
#[inline(always)]
#[hax_lib::requires(i < 25
        && blocks[0].len() == blocks[1].len()
        && blocks[0].len() == blocks[2].len()
        && blocks[0].len() == blocks[3].len()
        && offset.to_int() + (8.to_int() * i.to_int()) + 8.to_int() <= blocks[0].len().to_int())]
#[hax_lib::ensures(|result|
    get_lane_u64(result, 0) == load_lane_u64(blocks, offset, i, statei, 0)
    && get_lane_u64(result, 1) == load_lane_u64(blocks, offset, i, statei, 1)
    && get_lane_u64(result, 2) == load_lane_u64(blocks, offset, i, statei, 2)
    && get_lane_u64(result, 3) == load_lane_u64(blocks, offset, i, statei, 3)
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
fn load_u64x4(blocks: &[&[u8]; 4], offset: usize, i: usize, statei: Vec256) -> Vec256 {
    // load_lane_u64 is opaque-to-SMT; reveal here so the body can prove its ensures.
    hax_lib::fstar!(r#"reveal_opaque (`%load_lane_u64) load_lane_u64"#);
    let v0 = u64::from_le_bytes(
        blocks[0][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let v1 = u64::from_le_bytes(
        blocks[1][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let v2 = u64::from_le_bytes(
        blocks[2][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let v3 = u64::from_le_bytes(
        blocks[3][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let u = mm256_set_epi64x(v3, v2, v1, v0);
    mm256_xor_si256(statei, u)
}

#[cfg(hax)]
#[hax_lib::requires(valid_rate(rate))]
#[hax_lib::ensures(|_|
    (rate % 32 == 8 || rate % 32 == 16) &&
    if rate % 32 == 16 {
        rate / 8 == 4 * (rate/32) + 2
    } else {rate / 8 == 4 * (rate/32) + 1})]
fn lemma_rate_mod(rate: usize) {
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'")]
#[hax_lib::requires(valid_rate(RATE)
            && blocks[0].len() == blocks[1].len()
            && blocks[0].len() == blocks[2].len()
            && blocks[0].len() == blocks[3].len()
            && offset.to_int() + RATE.to_int() <= blocks[0].len().to_int()
)]
#[hax_lib::ensures(|_| hax_lib::forall(|i: usize|
    if i < 25 {
        if i < RATE / 8 {
            get_lane_u64(future(state)[i], 0) == load_lane_u64(blocks, offset, i, state[i], 0)
            && get_lane_u64(future(state)[i], 1) == load_lane_u64(blocks, offset, i, state[i], 1)
            && get_lane_u64(future(state)[i], 2) == load_lane_u64(blocks, offset, i, state[i], 2)
            && get_lane_u64(future(state)[i], 3) == load_lane_u64(blocks, offset, i, state[i], 3)
        } else {
            get_lane_u64(future(state)[i], 0) == get_lane_u64(state[i], 0)
            && get_lane_u64(future(state)[i], 1) == get_lane_u64(state[i], 1)
            && get_lane_u64(future(state)[i], 2) == get_lane_u64(state[i], 2)
            && get_lane_u64(future(state)[i], 3) == get_lane_u64(state[i], 3)
        }
    } else { true }
))]
pub(crate) fn load_block<const RATE: usize>(
    state: &mut [Vec256; 25],
    blocks: &[&[u8]; 4],
    offset: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(RATE <= blocks[0].len() && RATE / 32 <= 6 && 32 * (RATE/32 - 1) + 32 <= RATE &&
                  RATE % 8 == 0 && (RATE % 32 == 8 || RATE % 32 == 16));
    #[cfg(hax)]
    let old_state = *state;
    for i in 0..RATE / 32 {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize|
            if j < 25 {
                if j < 4 * i {
                    get_lane_u64(state[j], 0) == load_lane_u64(blocks, offset, j, old_state[j], 0)
                        && get_lane_u64(state[j], 1) == load_lane_u64(blocks, offset, j, old_state[j], 1)
                        && get_lane_u64(state[j], 2) == load_lane_u64(blocks, offset, j, old_state[j], 2)
                        && get_lane_u64(state[j], 3) == load_lane_u64(blocks, offset, j, old_state[j], 3)
                } else {
                    get_lane_u64(state[j], 0) == get_lane_u64(old_state[j], 0)
                        && get_lane_u64(state[j], 1) == get_lane_u64(old_state[j], 1)
                        && get_lane_u64(state[j], 2) == get_lane_u64(old_state[j], 2)
                        && get_lane_u64(state[j], 3) == get_lane_u64(old_state[j], 3)
                }
            } else {
                true
            }));
        let i0 = (4 * i) / 5;
        let j0 = (4 * i) % 5;
        let i1 = (4 * i + 1) / 5;
        let j1 = (4 * i + 1) % 5;
        let i2 = (4 * i + 2) / 5;
        let j2 = (4 * i + 2) % 5;
        let i3 = (4 * i + 3) / 5;
        let j3 = (4 * i + 3) % 5;
        hax_lib::fstar!(r#"
          assert(v $RATE / 32 > 0);
          assert (v $i <= v $RATE / 32 - 1);
          assert (v $i < 6);
          assert (v $i + 1 <= v $RATE / 32);
          assert ((v $RATE / 32) * 32 <= v $RATE);
          assert (32 * (v $i + 1) <= v $RATE);
          assert (32 * v $i + 32 <= v $RATE);
          assert (sz 32 *! $i +! sz 32 <=. $RATE)
        "#);
        let (g0, g1, g2, g3) = load_u64x4x4(
            blocks,
            offset,
            i,
            *get_ij(state, i0, j0),
            *get_ij(state, i1, j1),
            *get_ij(state, i2, j2),
            *get_ij(state, i3, j3),
        );
        set_ij(state, i0, j0, g0);
        set_ij(state, i1, j1, g1);
        set_ij(state, i2, j2, g2);
        set_ij(state, i3, j3, g3);
        hax_lib::assert!(get_lane_u64(state[4*i], 0) == load_lane_u64(blocks, offset, 4*i, old_state[4*i], 0)
                        && get_lane_u64(state[4*i], 1) == load_lane_u64(blocks, offset, 4*i, old_state[4*i], 1)
                        && get_lane_u64(state[4*i], 2) == load_lane_u64(blocks, offset, 4*i, old_state[4*i], 2)
                        && get_lane_u64(state[4*i], 3) == load_lane_u64(blocks, offset, 4*i, old_state[4*i], 3));
        hax_lib::assert!(get_lane_u64(state[4*i+1], 0) == load_lane_u64(blocks, offset, 4*i+1, old_state[4*i+1], 0)
                        && get_lane_u64(state[4*i+1], 1) == load_lane_u64(blocks, offset, 4*i+1, old_state[4*i+1], 1)
                        && get_lane_u64(state[4*i+1], 2) == load_lane_u64(blocks, offset, 4*i+1, old_state[4*i+1], 2)
                        && get_lane_u64(state[4*i+1], 3) == load_lane_u64(blocks, offset, 4*i+1, old_state[4*i+1], 3));
        hax_lib::assert!(get_lane_u64(state[4*i+2], 0) == load_lane_u64(blocks, offset, 4*i+2, old_state[4*i+2], 0)
                        && get_lane_u64(state[4*i+2], 1) == load_lane_u64(blocks, offset, 4*i+2, old_state[4*i+2], 1)
                        && get_lane_u64(state[4*i+2], 2) == load_lane_u64(blocks, offset, 4*i+2, old_state[4*i+2], 2)
                        && get_lane_u64(state[4*i+2], 3) == load_lane_u64(blocks, offset, 4*i+2, old_state[4*i+2], 3));
        hax_lib::assert!(get_lane_u64(state[4*i+3], 0) == load_lane_u64(blocks, offset, 4*i+3, old_state[4*i+3], 0)
                        && get_lane_u64(state[4*i+3], 1) == load_lane_u64(blocks, offset, 4*i+3, old_state[4*i+3], 1)
                        && get_lane_u64(state[4*i+3], 2) == load_lane_u64(blocks, offset, 4*i+3, old_state[4*i+3], 2)
                        && get_lane_u64(state[4*i+3], 3) == load_lane_u64(blocks, offset, 4*i+3, old_state[4*i+3], 3));
    }
    lemma_rate_mod(RATE);
    let rem = RATE % 32; // has to be 8 or 16
    let i = 4 * (RATE / 32);
    let result = load_u64x4(blocks, offset, i, *get_ij(state, i / 5, i % 5));
    set_ij(state, i / 5, i % 5, result);
    if rem == 16 {
        let i = 4 * (RATE / 32) + 1;
        let result = load_u64x4(blocks, offset, i, *get_ij(state, i / 5, i % 5));
        set_ij(state, i / 5, i % 5, result);
    }
}

#[inline(always)]
#[hax_lib::requires(valid_rate(RATE)
    && len < RATE
    && start.to_int() + len.to_int() <= blocks[0].len().to_int()
    && blocks[0].len() == blocks[1].len()
    && blocks[0].len() == blocks[2].len()
    && blocks[0].len() == blocks[3].len()
)]
pub(crate) fn load_last<const RATE: usize, const DELIMITER: u8>(
    state: &mut [Vec256; 25],
    blocks: &[&[u8]; 4],
    start: usize,
    len: usize,
) {
    // Loop unrolled to mirror simd/arm64.rs::load_last so the F*
    // bridge [lemma_load_last_eq_xor_block_into_state_avx2] can
    // reconstruct each buffer in scope without reasoning about a
    // fold_range over [buffers].
    let mut buffer0 = [0u8; RATE];
    buffer0[0..len].copy_from_slice(&blocks[0][start..start + len]);
    buffer0[len] = DELIMITER;
    buffer0[RATE - 1] |= 0x80;

    let mut buffer1 = [0u8; RATE];
    buffer1[0..len].copy_from_slice(&blocks[1][start..start + len]);
    buffer1[len] = DELIMITER;
    buffer1[RATE - 1] |= 0x80;

    let mut buffer2 = [0u8; RATE];
    buffer2[0..len].copy_from_slice(&blocks[2][start..start + len]);
    buffer2[len] = DELIMITER;
    buffer2[RATE - 1] |= 0x80;

    let mut buffer3 = [0u8; RATE];
    buffer3[0..len].copy_from_slice(&blocks[3][start..start + len]);
    buffer3[len] = DELIMITER;
    buffer3[RATE - 1] |= 0x80;

    load_block::<RATE>(state, &[&buffer0, &buffer1, &buffer2, &buffer3], 0);
}

#[hax_lib::attributes]
impl Absorb<4> for KeccakState<4, Vec256> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= input[0].len().to_int() &&
        input[0].len() == input[1].len() &&
        input[0].len() == input[2].len() &&
        input[0].len() == input[3].len()
    )]
    fn load_block<const RATE: usize>(&mut self, input: &[&[u8]; 4], start: usize) {
        load_block::<RATE>(&mut self.st, input, start);
    }

    #[hax_lib::requires(
        valid_rate(RATE) &&
        len < RATE &&
        start.to_int() + len.to_int() <= input[0].len().to_int() &&
        input[0].len() == input[1].len() &&
        input[0].len() == input[2].len() &&
        input[0].len() == input[3].len()
    )]
    fn load_last<const RATE: usize, const DELIMITER: u8>(
        &mut self,
        input: &[&[u8]; 4],
        start: usize,
        len: usize,
    ) {
        load_last::<RATE, DELIMITER>(&mut self.st, input, start, len)
    }
}
