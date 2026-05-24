#[cfg(hax)]
use hax_lib::int::ToInt;

#[cfg(hax)]
use hax_lib::prop::*;

#[cfg(hax)]
use crate::proof_utils::valid_rate;

use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, Squeeze4};

/// Per-iteration store wrapper for `store_block_full_avx2`. Given the
/// four state vectors `s0..s3` (= `s[4*i + 0..4*i + 3]` after the
/// composer's linearisation), the four permute2x128 + two
/// unpacklo/unpackhi pass deinterleaves them into four output streams
/// `v_m`, each whose lane `k` corresponds to lane `m` of `s_k`. Four
/// `mm256_storeu_si256_u8` stores then write a 32-byte window per
/// buffer.
///
/// Factored out of `store_block_full_avx2` so the strong per-byte
/// ensures isolates the `update_at_range`/permute/unpack reasoning from
/// the outer loop's heavy invariant. Mirrors `store_u64x2x2` on the
/// AVX2 side (4 lanes instead of 2).
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && out0.len() == out2.len()
    && out0.len() == out3.len()
    && start.to_int() + (32.to_int() * (i.to_int() + 1.to_int())) <= out0.len().to_int()
)]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & hax_lib::forall(|j: usize|
        if j < out0.len() {
            if j < start + 32 * i {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
                    && out2[j] == future(out2)[j] && out3[j] == future(out3)[j]
            } else if j < start + 32 * (i + 1) {
                if (j - start) / 8 == 4 * i {
                    future(out0)[j] == get_lane_u64(s0, 0).to_le_bytes()[(j - start) % 8]
                        && future(out1)[j] == get_lane_u64(s0, 1).to_le_bytes()[(j - start) % 8]
                        && future(out2)[j] == get_lane_u64(s0, 2).to_le_bytes()[(j - start) % 8]
                        && future(out3)[j] == get_lane_u64(s0, 3).to_le_bytes()[(j - start) % 8]
                } else if (j - start) / 8 == 4 * i + 1 {
                    future(out0)[j] == get_lane_u64(s1, 0).to_le_bytes()[(j - start) % 8]
                        && future(out1)[j] == get_lane_u64(s1, 1).to_le_bytes()[(j - start) % 8]
                        && future(out2)[j] == get_lane_u64(s1, 2).to_le_bytes()[(j - start) % 8]
                        && future(out3)[j] == get_lane_u64(s1, 3).to_le_bytes()[(j - start) % 8]
                } else if (j - start) / 8 == 4 * i + 2 {
                    future(out0)[j] == get_lane_u64(s2, 0).to_le_bytes()[(j - start) % 8]
                        && future(out1)[j] == get_lane_u64(s2, 1).to_le_bytes()[(j - start) % 8]
                        && future(out2)[j] == get_lane_u64(s2, 2).to_le_bytes()[(j - start) % 8]
                        && future(out3)[j] == get_lane_u64(s2, 3).to_le_bytes()[(j - start) % 8]
                } else {
                    future(out0)[j] == get_lane_u64(s3, 0).to_le_bytes()[(j - start) % 8]
                        && future(out1)[j] == get_lane_u64(s3, 1).to_le_bytes()[(j - start) % 8]
                        && future(out2)[j] == get_lane_u64(s3, 2).to_le_bytes()[(j - start) % 8]
                        && future(out3)[j] == get_lane_u64(s3, 3).to_le_bytes()[(j - start) % 8]
                }
            } else {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
                    && out2[j] == future(out2)[j] && out3[j] == future(out3)[j]
            }
        } else {
            true
        })
)]
fn store_u64x4x4(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    s0: Vec256,
    s1: Vec256,
    s2: Vec256,
    s3: Vec256,
    start: usize,
    i: usize,
) {
    let v0l = mm256_permute2x128_si256::<0x20>(s0, s2);
    let v1h = mm256_permute2x128_si256::<0x20>(s1, s3);
    let v2l = mm256_permute2x128_si256::<0x31>(s0, s2);
    let v3h = mm256_permute2x128_si256::<0x31>(s1, s3);
    let v0 = mm256_unpacklo_epi64(v0l, v1h);
    let v1 = mm256_unpackhi_epi64(v0l, v1h);
    let v2 = mm256_unpacklo_epi64(v2l, v3h);
    let v3 = mm256_unpackhi_epi64(v2l, v3h);
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let old_out2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let old_out3 = out3.to_vec().as_slice();
    mm256_storeu_si256_u8(&mut out0[start + 32 * i..start + 32 * (i + 1)], v0);
    mm256_storeu_si256_u8(&mut out1[start + 32 * i..start + 32 * (i + 1)], v1);
    mm256_storeu_si256_u8(&mut out2[start + 32 * i..start + 32 * (i + 1)], v2);
    mm256_storeu_si256_u8(&mut out3[start + 32 * i..start + 32 * (i + 1)], v3);
    // Bridge the strengthened `mm256_storeu_si256_u8` per-byte post +
    // `update_at_range` slice posts into the per-absolute-index byte
    // facts the function-level ensures expects, then propagate via
    // `forall_intro` over the abstract index `j`.
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 32 * v i in
        assert (a_pos + 32 <= Seq.length old_out0);
        assert (a_pos + 32 <= Seq.length old_out1);
        assert (a_pos + 32 <= Seq.length old_out2);
        assert (a_pos + 32 <= Seq.length old_out3);
        let bridge_out0 (j_n:nat{j_n < Seq.length old_out0}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + 32 then
                Seq.index out0 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 v0 (mk_usize ((j_n - a_pos) / 8))))
                    ((j_n - a_pos) % 8)
              else
                Seq.index out0 j_n == Seq.index old_out0 j_n)
          = Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call
              old_out0 out0 v0 a_pos j_n
        in
        let bridge_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + 32 then
                Seq.index out1 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 v1 (mk_usize ((j_n - a_pos) / 8))))
                    ((j_n - a_pos) % 8)
              else
                Seq.index out1 j_n == Seq.index old_out1 j_n)
          = Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call
              old_out1 out1 v1 a_pos j_n
        in
        let bridge_out2 (j_n:nat{j_n < Seq.length old_out2}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out2 j_n == Seq.index old_out2 j_n
              else if j_n < a_pos + 32 then
                Seq.index out2 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 v2 (mk_usize ((j_n - a_pos) / 8))))
                    ((j_n - a_pos) % 8)
              else
                Seq.index out2 j_n == Seq.index old_out2 j_n)
          = Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call
              old_out2 out2 v2 a_pos j_n
        in
        let bridge_out3 (j_n:nat{j_n < Seq.length old_out3}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out3 j_n == Seq.index old_out3 j_n
              else if j_n < a_pos + 32 then
                Seq.index out3 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 v3 (mk_usize ((j_n - a_pos) / 8))))
                    ((j_n - a_pos) % 8)
              else
                Seq.index out3 j_n == Seq.index old_out3 j_n)
          = Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.store_block_window_byte_of_storeu_call
              old_out3 out3 v3 a_pos j_n
        in
        Classical.forall_intro bridge_out0;
        Classical.forall_intro bridge_out1;
        Classical.forall_intro bridge_out2;
        Classical.forall_intro bridge_out3
        "#
    );
}

/// Per-iteration store wrapper for the inner `for k in 0..chunks8`
/// loop of `store_block_tail_avx2`. Materializes a 32-byte scratch
/// from a single state vector via `mm256_storeu_si256_u8`, then
/// copies 4 disjoint 8-byte windows into `out0..out3`. Lane `m` of
/// `vec` writes to `out_m[start+8k..start+8(k+1))`.
///
/// Factored out so its per-byte ensures isolates the local
/// `update_at_range`/`copy_from_slice` reasoning from the surrounding
/// loop's heavy invariant. Mirrors `store_u64x4x4` but for the
/// inner 8-byte path.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && out0.len() == out2.len()
    && out0.len() == out3.len()
    && start.to_int() + (8.to_int() * (k.to_int() + 1.to_int())) <= out0.len().to_int()
)]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & hax_lib::forall(|j: usize|
        if j < out0.len() {
            if j < start + 8 * k {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
                    && out2[j] == future(out2)[j] && out3[j] == future(out3)[j]
            } else if j < start + 8 * (k + 1) {
                future(out0)[j] == get_lane_u64(vec, 0).to_le_bytes()[(j - start) % 8]
                    && future(out1)[j] == get_lane_u64(vec, 1).to_le_bytes()[(j - start) % 8]
                    && future(out2)[j] == get_lane_u64(vec, 2).to_le_bytes()[(j - start) % 8]
                    && future(out3)[j] == get_lane_u64(vec, 3).to_le_bytes()[(j - start) % 8]
            } else {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
                    && out2[j] == future(out2)[j] && out3[j] == future(out3)[j]
            }
        } else {
            true
        })
)]
fn store_chunk8x4(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    vec: Vec256,
    start: usize,
    k: usize,
) {
    let mut u8s = [0u8; 32];
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let old_out2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let old_out3 = out3.to_vec().as_slice();
    mm256_storeu_si256_u8(&mut u8s, vec);
    out0[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[0..8]);
    out1[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[8..16]);
    out2[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[16..24]);
    out3[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[24..32]);
    // Bridge: u8s[m*8 + t] == byte t of to_le_bytes(get_lane_u64 vec m)
    // for m<4, t<8 (from the strengthened intrinsic axiom). Each
    // `copy_from_slice` then transfers an 8-byte window into out_m.
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 8 * v k in
        assert (a_pos + 8 <= Seq.length old_out0);
        assert (a_pos + 8 <= Seq.length old_out1);
        assert (a_pos + 8 <= Seq.length old_out2);
        assert (a_pos + 8 <= Seq.length old_out3);
        // Materialise the per-byte facts on u8s lane-by-lane via the
        // explicit intrinsic axiom; this avoids leaning on SMTPat
        // saturation across the 4 outputs.
        Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.mm256_storeu_si256_u8_byte_window
          (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32)) vec;
        let bridge_out_m
              (m_lane: nat{m_lane < 4})
              (out_old out_new: Seq.seq u8)
              (j_n: nat{j_n < Seq.length out_old})
            : Lemma
              (requires
                  a_pos + 8 <= Seq.length out_old /\
                  Seq.length out_new == Seq.length out_old /\
                  Seq.slice out_new 0 a_pos == Seq.slice out_old 0 a_pos /\
                  Seq.slice out_new a_pos (a_pos + 8) ==
                    Seq.slice u8s (m_lane * 8) (m_lane * 8 + 8) /\
                  Seq.slice out_new (a_pos + 8) (Seq.length out_new)
                    == Seq.slice out_old (a_pos + 8) (Seq.length out_old))
              (ensures
                (if j_n < a_pos then
                   Seq.index out_new j_n == Seq.index out_old j_n
                 else if j_n < a_pos + 8 then
                   Seq.index out_new j_n ==
                     Seq.index
                       (Core_models.Num.impl_u64__to_le_bytes
                          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize m_lane)))
                       (j_n - a_pos)
                 else
                   Seq.index out_new j_n == Seq.index out_old j_n))
          = if j_n < a_pos then begin
              assert (Seq.index (Seq.slice out_new 0 a_pos) j_n == Seq.index out_new j_n);
              assert (Seq.index (Seq.slice out_old 0 a_pos) j_n == Seq.index out_old j_n)
            end else if j_n < a_pos + 8 then begin
              let t:nat = j_n - a_pos in
              assert (Seq.index (Seq.slice out_new a_pos (a_pos + 8)) t == Seq.index out_new j_n);
              assert (Seq.index (Seq.slice u8s (m_lane * 8) (m_lane * 8 + 8)) t ==
                      Seq.index u8s (m_lane * 8 + t));
              assert ((m_lane * 8 + t) / 8 == m_lane);
              assert ((m_lane * 8 + t) % 8 == t)
            end else begin
              let t:nat = j_n - (a_pos + 8) in
              assert (Seq.index (Seq.slice out_new (a_pos + 8) (Seq.length out_new)) t ==
                      Seq.index out_new j_n);
              assert (Seq.index (Seq.slice out_old (a_pos + 8) (Seq.length out_old)) t ==
                      Seq.index out_old j_n)
            end
        in
        let bridge_call_out0 (j_n:nat{j_n < Seq.length old_out0}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + 8 then
                Seq.index out0 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 0)))
                    (j_n - a_pos)
              else
                Seq.index out0 j_n == Seq.index old_out0 j_n)
          = bridge_out_m 0 old_out0 out0 j_n
        in
        let bridge_call_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + 8 then
                Seq.index out1 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 1)))
                    (j_n - a_pos)
              else
                Seq.index out1 j_n == Seq.index old_out1 j_n)
          = bridge_out_m 1 old_out1 out1 j_n
        in
        let bridge_call_out2 (j_n:nat{j_n < Seq.length old_out2}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out2 j_n == Seq.index old_out2 j_n
              else if j_n < a_pos + 8 then
                Seq.index out2 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 2)))
                    (j_n - a_pos)
              else
                Seq.index out2 j_n == Seq.index old_out2 j_n)
          = bridge_out_m 2 old_out2 out2 j_n
        in
        let bridge_call_out3 (j_n:nat{j_n < Seq.length old_out3}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out3 j_n == Seq.index old_out3 j_n
              else if j_n < a_pos + 8 then
                Seq.index out3 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 3)))
                    (j_n - a_pos)
              else
                Seq.index out3 j_n == Seq.index old_out3 j_n)
          = bridge_out_m 3 old_out3 out3 j_n
        in
        Classical.forall_intro bridge_call_out0;
        Classical.forall_intro bridge_call_out1;
        Classical.forall_intro bridge_call_out2;
        Classical.forall_intro bridge_call_out3
        "#
    );
}

/// Tail wrapper for the rem8>0 block of store_block_tail_avx2.
/// Identical structure to store_chunk8x4 but with a partial window
/// of rem8<8 bytes per output. The window covers
/// [start + 8*q_inner, start + 8*q_inner + rem8); lane m of vec maps to out_m.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && out0.len() == out2.len()
    && out0.len() == out3.len()
    && rem8 > 0
    && rem8 < 8
    && start.to_int() + (8.to_int() * q_inner.to_int()) + rem8.to_int() <= out0.len().to_int()
)]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & hax_lib::forall(|j: usize|
        if j < out0.len() {
            if j < start + 8 * q_inner {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
                    && out2[j] == future(out2)[j] && out3[j] == future(out3)[j]
            } else if j < start + 8 * q_inner + rem8 {
                future(out0)[j] == get_lane_u64(vec, 0).to_le_bytes()[(j - start) % 8]
                    && future(out1)[j] == get_lane_u64(vec, 1).to_le_bytes()[(j - start) % 8]
                    && future(out2)[j] == get_lane_u64(vec, 2).to_le_bytes()[(j - start) % 8]
                    && future(out3)[j] == get_lane_u64(vec, 3).to_le_bytes()[(j - start) % 8]
            } else {
                out0[j] == future(out0)[j] && out1[j] == future(out1)[j]
                    && out2[j] == future(out2)[j] && out3[j] == future(out3)[j]
            }
        } else {
            true
        })
)]
fn store_tail_ragged_avx2(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    vec: Vec256,
    start: usize,
    q_inner: usize,
    rem8: usize,
) {
    let mut u8s = [0u8; 32];
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let old_out2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let old_out3 = out3.to_vec().as_slice();
    mm256_storeu_si256_u8(&mut u8s, vec);
    out0[start + 8 * q_inner..start + 8 * q_inner + rem8].copy_from_slice(&u8s[0..rem8]);
    out1[start + 8 * q_inner..start + 8 * q_inner + rem8].copy_from_slice(&u8s[8..8 + rem8]);
    out2[start + 8 * q_inner..start + 8 * q_inner + rem8].copy_from_slice(&u8s[16..16 + rem8]);
    out3[start + 8 * q_inner..start + 8 * q_inner + rem8].copy_from_slice(&u8s[24..24 + rem8]);
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 8 * v q_inner in
        let r:nat = v rem8 in
        assert (r < 8);
        assert (a_pos + r <= Seq.length old_out0);
        assert (a_pos + r <= Seq.length old_out1);
        assert (a_pos + r <= Seq.length old_out2);
        assert (a_pos + r <= Seq.length old_out3);
        Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.mm256_storeu_si256_u8_byte_window
          (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32)) vec;
        let bridge_partial_out_m
              (m_lane: nat{m_lane < 4})
              (out_old out_new: Seq.seq u8)
              (j_n: nat{j_n < Seq.length out_old})
            : Lemma
              (requires
                  a_pos + r <= Seq.length out_old /\
                  Seq.length out_new == Seq.length out_old /\
                  Seq.slice out_new 0 a_pos == Seq.slice out_old 0 a_pos /\
                  Seq.slice out_new a_pos (a_pos + r) ==
                    Seq.slice u8s (m_lane * 8) (m_lane * 8 + r) /\
                  Seq.slice out_new (a_pos + r) (Seq.length out_new)
                    == Seq.slice out_old (a_pos + r) (Seq.length out_old))
              (ensures
                (if j_n < a_pos then
                   Seq.index out_new j_n == Seq.index out_old j_n
                 else if j_n < a_pos + r then
                   Seq.index out_new j_n ==
                     Seq.index
                       (Core_models.Num.impl_u64__to_le_bytes
                          (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize m_lane)))
                       (j_n - a_pos)
                 else
                   Seq.index out_new j_n == Seq.index out_old j_n))
          = if j_n < a_pos then begin
              assert (Seq.index (Seq.slice out_new 0 a_pos) j_n == Seq.index out_new j_n);
              assert (Seq.index (Seq.slice out_old 0 a_pos) j_n == Seq.index out_old j_n)
            end else if j_n < a_pos + r then begin
              let t:nat = j_n - a_pos in
              assert (t < r /\ t < 8);
              assert (Seq.index (Seq.slice out_new a_pos (a_pos + r)) t == Seq.index out_new j_n);
              assert (Seq.index (Seq.slice u8s (m_lane * 8) (m_lane * 8 + r)) t ==
                      Seq.index u8s (m_lane * 8 + t));
              assert ((m_lane * 8 + t) / 8 == m_lane);
              assert ((m_lane * 8 + t) % 8 == t)
            end else begin
              let t:nat = j_n - (a_pos + r) in
              assert (Seq.index (Seq.slice out_new (a_pos + r) (Seq.length out_new)) t ==
                      Seq.index out_new j_n);
              assert (Seq.index (Seq.slice out_old (a_pos + r) (Seq.length out_old)) t ==
                      Seq.index out_old j_n)
            end
        in
        let bridge_call_out0 (j_n:nat{j_n < Seq.length old_out0}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + r then
                Seq.index out0 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 0)))
                    (j_n - a_pos)
              else
                Seq.index out0 j_n == Seq.index old_out0 j_n)
          = bridge_partial_out_m 0 old_out0 out0 j_n
        in
        let bridge_call_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + r then
                Seq.index out1 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 1)))
                    (j_n - a_pos)
              else
                Seq.index out1 j_n == Seq.index old_out1 j_n)
          = bridge_partial_out_m 1 old_out1 out1 j_n
        in
        let bridge_call_out2 (j_n:nat{j_n < Seq.length old_out2}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out2 j_n == Seq.index old_out2 j_n
              else if j_n < a_pos + r then
                Seq.index out2 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 2)))
                    (j_n - a_pos)
              else
                Seq.index out2 j_n == Seq.index old_out2 j_n)
          = bridge_partial_out_m 2 old_out2 out2 j_n
        in
        let bridge_call_out3 (j_n:nat{j_n < Seq.length old_out3}) :
            Lemma (
              if j_n < a_pos then
                Seq.index out3 j_n == Seq.index old_out3 j_n
              else if j_n < a_pos + r then
                Seq.index out3 j_n ==
                  Seq.index
                    (Core_models.Num.impl_u64__to_le_bytes
                       (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 3)))
                    (j_n - a_pos)
              else
                Seq.index out3 j_n == Seq.index old_out3 j_n)
          = bridge_partial_out_m 3 old_out3 out3 j_n
        in
        Classical.forall_intro bridge_call_out0;
        Classical.forall_intro bridge_call_out1;
        Classical.forall_intro bridge_call_out2;
        Classical.forall_intro bridge_call_out3
        "#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(valid_rate(RATE)
    && len <= RATE
    && start.to_int() + len.to_int() <= out0.len().to_int()
    && out0.len() == out1.len()
    && out0.len() == out2.len()
    && out0.len() == out3.len()
)]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & hax_lib::forall(|i: usize| if i < out0.len() {
        if i < start {
            out0[i] == future(out0)[i]
        } else if i < start + len {
            future(out0)[i] == get_lane_u64(s[(i - start) / 8], 0).to_le_bytes()[(i - start) % 8]
        } else {
            out0[i] == future(out0)[i]
        }
    } else { true })
    & hax_lib::forall(|i: usize| if i < out1.len() {
        if i < start {
            out1[i] == future(out1)[i]
        } else if i < start + len {
            future(out1)[i] == get_lane_u64(s[(i - start) / 8], 1).to_le_bytes()[(i - start) % 8]
        } else {
            out1[i] == future(out1)[i]
        }
    } else { true })
    & hax_lib::forall(|i: usize| if i < out2.len() {
        if i < start {
            out2[i] == future(out2)[i]
        } else if i < start + len {
            future(out2)[i] == get_lane_u64(s[(i - start) / 8], 2).to_le_bytes()[(i - start) % 8]
        } else {
            out2[i] == future(out2)[i]
        }
    } else { true })
    & hax_lib::forall(|i: usize| if i < out3.len() {
        if i < start {
            out3[i] == future(out3)[i]
        } else if i < start + len {
            future(out3)[i] == get_lane_u64(s[(i - start) / 8], 3).to_le_bytes()[(i - start) % 8]
        } else {
            out3[i] == future(out3)[i]
        }
    } else { true })
)]
pub(crate) fn store_block<const RATE: usize>(
    s: &[Vec256; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    start: usize,
    len: usize,
) {
    hax_lib::fstar!("admit()");
    let chunks = len / 32;
    for i in 0..chunks {
        let i0 = (4 * i) / 5;
        let j0 = (4 * i) % 5;
        let i1 = (4 * i + 1) / 5;
        let j1 = (4 * i + 1) % 5;
        let i2 = (4 * i + 2) / 5;
        let j2 = (4 * i + 2) % 5;
        let i3 = (4 * i + 3) / 5;
        let j3 = (4 * i + 3) % 5;

        let v0l = mm256_permute2x128_si256::<0x20>(*get_ij(s, i0, j0), *get_ij(s, i2, j2));
        // 0 0 2 2
        let v1h = mm256_permute2x128_si256::<0x20>(*get_ij(s, i1, j1), *get_ij(s, i3, j3)); // 1 1 3 3
        let v2l = mm256_permute2x128_si256::<0x31>(*get_ij(s, i0, j0), *get_ij(s, i2, j2)); // 0 0 2 2
        let v3h = mm256_permute2x128_si256::<0x31>(*get_ij(s, i1, j1), *get_ij(s, i3, j3)); // 1 1 3 3

        let v0 = mm256_unpacklo_epi64(v0l, v1h); // 0 1 2 3
        let v1 = mm256_unpackhi_epi64(v0l, v1h); // 0 1 2 3
        let v2 = mm256_unpacklo_epi64(v2l, v3h); // 0 1 2 3
        let v3 = mm256_unpackhi_epi64(v2l, v3h); // 0 1 2 3

        mm256_storeu_si256_u8(&mut out0[start + 32 * i..start + 32 * (i + 1)], v0);
        mm256_storeu_si256_u8(&mut out1[start + 32 * i..start + 32 * (i + 1)], v1);
        mm256_storeu_si256_u8(&mut out2[start + 32 * i..start + 32 * (i + 1)], v2);
        mm256_storeu_si256_u8(&mut out3[start + 32 * i..start + 32 * (i + 1)], v3);
    }

    let rem = len % 32;
    if rem > 0 {
        let start = start + 32 * chunks;
        let mut u8s = [0u8; 32];
        let chunks8 = rem / 8;
        for k in 0..chunks8 {
            let i = (4 * chunks + k) / 5;
            let j = (4 * chunks + k) % 5;
            mm256_storeu_si256_u8(&mut u8s, *get_ij(s, i, j));
            out0[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[0..8]);
            out1[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[8..16]);
            out2[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[16..24]);
            out3[start + 8 * k..start + 8 * (k + 1)].copy_from_slice(&u8s[24..32]);
        }
        let rem8 = rem % 8;
        if rem8 > 0 {
            let i = (4 * chunks + chunks8) / 5;
            let j = (4 * chunks + chunks8) % 5;
            mm256_storeu_si256_u8(&mut u8s, *get_ij(s, i, j));
            out0[start + rem - rem8..start + rem].copy_from_slice(&u8s[0..rem8]);
            out1[start + rem - rem8..start + rem].copy_from_slice(&u8s[8..8 + rem8]);
            out2[start + rem - rem8..start + rem].copy_from_slice(&u8s[16..16 + rem8]);
            out3[start + rem - rem8..start + rem].copy_from_slice(&u8s[24..24 + rem8]);
        }
    }
}

#[hax_lib::attributes]
impl Squeeze4<Vec256> for KeccakState<4, Vec256> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        len <= RATE &&
        start.to_int() + len.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
    fn squeeze4<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
        start: usize,
        len: usize,
    ) {
        store_block::<RATE>(&self.st, out0, out1, out2, out3, start, len)
    }
}
