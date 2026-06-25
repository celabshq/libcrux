#[cfg(hax)]
use hax_lib::int::ToInt;

#[cfg(hax)]
use hax_lib::prop::*;

#[cfg(hax)]
use crate::proof_utils::{modifies_range, valid_rate};

use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, Squeeze4};

/// `stored s out start lane lo hi`: every byte index `k` in the
/// half-open range `[lo, hi)` of `out` already holds the correct
/// squeezed output byte — namely byte `(k-start) % 8` of the
/// little-endian encoding of lane `lane` of state word
/// `s[(k-start)/8]`. The AVX2 analog of the user's
/// `store_block_output`, lifted to a range. Opaque so the outer-loop
/// and composer never unfold the per-byte content; only the producer
/// (`store_u64x4x4`) reveals it.
#[cfg(hax)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
pub(crate) fn stored(
    s: &[Vec256; 25],
    out: &[u8],
    start: usize,
    lane: usize,
    lo: usize,
    hi: usize,
) -> hax_lib::Prop {
    hax_lib::forall(|k: usize| {
        hax_lib::implies(
            lane < 4 && start <= k && lo <= k && k < hi && k < out.len() && (k - start) / 8 < 25,
            out[k] == get_lane_u64(s[(k - start) / 8], lane).to_le_bytes()[(k - start) % 8],
        )
    })
}

/// `stored` over `[lo,hi)` is preserved when the buffer is modified
/// only on a strictly-later disjoint range `[mlo,mhi)` (`hi <= mlo`):
/// content is per-index and the prefix bytes are untouched. Used by
/// the producer to carry the already-written prefix past the new
/// window. Reveals both opaque predicates internally.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_stored_frame
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (a b: t_Slice u8)
      (start lane lo hi mlo mhi: usize)
  : Lemma
    (requires
      stored s a start lane lo hi /\
      Libcrux_sha3.Proof_utils.modifies_range a b mlo mhi /\
      v hi <= v mlo)
    (ensures stored s b start lane lo hi)
  = reveal_opaque (`%stored) stored;
    reveal_opaque (`%Libcrux_sha3.Proof_utils.modifies_range)
      Libcrux_sha3.Proof_utils.modifies_range
"#
)]
fn lemma_stored_frame(
    _s: &[Vec256; 25],
    _a: &[u8],
    _b: &[u8],
    _start: usize,
    _lane: usize,
    _lo: usize,
    _hi: usize,
    _mlo: usize,
    _mhi: usize,
) {
}

/// `stored` is additive over adjacent ranges: correct on `[lo,mid)`
/// and `[mid,hi)` gives correct on `[lo,hi)`. Reveals `stored`.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_stored_union
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out: t_Slice u8)
      (start lane lo mid hi: usize)
  : Lemma
    (requires
      stored s out start lane lo mid /\ stored s out start lane mid hi /\
      v lo <= v mid /\ v mid <= v hi)
    (ensures stored s out start lane lo hi)
  = reveal_opaque (`%stored) stored
"#
)]
fn lemma_stored_union(
    _s: &[Vec256; 25],
    _out: &[u8],
    _start: usize,
    _lane: usize,
    _lo: usize,
    _mid: usize,
    _hi: usize,
) {
}

/// `stored` over an empty range `[lo,lo)` holds vacuously. Seeds the
/// loop-invariant base case. Reveals `stored`.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_stored_empty
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (out: t_Slice u8)
      (start lane lo: usize)
  : Lemma (ensures stored s out start lane lo lo)
  = reveal_opaque (`%stored) stored
"#
)]
fn lemma_stored_empty(_s: &[Vec256; 25], _out: &[u8], _start: usize, _lane: usize, _lo: usize) {}

/// Confined-reveal producer for `stored` over one window. From the
/// per-output `s_k`-discriminator byte facts (which `store_u64x4x4`'s
/// `bridge_out_m` establishes) + the linearisation `s_k == s[4i+k]`,
/// concludes the opaque `stored s out_m start lane_m window`. The
/// `reveal` of `stored` is confined HERE (verified once in clean
/// context) so the caller's bridge sub-queries are never polluted.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
#push-options "--z3rlimit 400"
let lemma_window_stored
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (s0 s1 s2 s3: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (start i: usize)
      (lane_m: nat{lane_m < 4})
      (out_m: t_Slice u8)
  : Lemma
    (requires
      v i < 6 /\
      v start + 32 * (v i + 1) <= Seq.length out_m /\
      s0 == Seq.index s (4 * v i + 0) /\
      s1 == Seq.index s (4 * v i + 1) /\
      s2 == Seq.index s (4 * v i + 2) /\
      s3 == Seq.index s (4 * v i + 3) /\
      (forall (j_n: nat).
        (v start + 32 * v i <= j_n /\ j_n < v start + 32 * (v i + 1) /\ j_n < Seq.length out_m) ==>
        (if (j_n - v start) / 8 = 4 * v i then
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s0 (mk_usize lane_m))) ((j_n - v start) % 8)
         else if (j_n - v start) / 8 = 4 * v i + 1 then
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s1 (mk_usize lane_m))) ((j_n - v start) % 8)
         else if (j_n - v start) / 8 = 4 * v i + 2 then
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 (mk_usize lane_m))) ((j_n - v start) % 8)
         else
           Seq.index out_m j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s3 (mk_usize lane_m))) ((j_n - v start) % 8))))
    (ensures
      stored s out_m start (mk_usize lane_m)
        (start +! (mk_usize 32 *! i)) (start +! (mk_usize 32 *! (i +! mk_usize 1))))
  = Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.lemma_lane_chain_to_s_all_j
      s s0 s1 s2 s3 (v start) (v i) lane_m out_m;
    reveal_opaque (`%stored) stored
#pop-options
"#
)]
fn lemma_window_stored(
    _s: &[Vec256; 25],
    _s0: Vec256,
    _s1: Vec256,
    _s2: Vec256,
    _s3: Vec256,
    _start: usize,
    _i: usize,
    _lane_m: usize,
    _out_m: &[u8],
) {
}

/// Confined-reveal producer for `modifies_range` over one window: from
/// the frame facts (`out_new` equals `out_old` outside the window) +
/// equal lengths, concludes the opaque `modifies_range`. `reveal`
/// confined here.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
let lemma_window_modifies (out_old out_new: t_Slice u8) (start i: usize)
  : Lemma
    (requires
      v start + 32 * (v i + 1) <= Seq.length out_old /\
      Seq.length out_new == Seq.length out_old /\
      (forall (j_n: nat).
        (j_n < Seq.length out_old /\
         (j_n < v start + 32 * v i \/ j_n >= v start + 32 * (v i + 1))) ==>
        Seq.index out_new j_n == Seq.index out_old j_n))
    (ensures
      Libcrux_sha3.Proof_utils.modifies_range out_old out_new
        (start +! (mk_usize 32 *! i)) (start +! (mk_usize 32 *! (i +! mk_usize 1))))
  = reveal_opaque (`%Libcrux_sha3.Proof_utils.modifies_range)
      Libcrux_sha3.Proof_utils.modifies_range
"#
)]
fn lemma_window_modifies(_out_old: &[u8], _out_new: &[u8], _start: usize, _i: usize) {}

/// Confined-reveal producer of `stored` for a single-vector window
/// `[off, off+w)` (`w <= 8`) written from lane `lane_m` of `vec`, where
/// `vec == s[base]` and `base == (off-start)/8` (8-aligned). Used by the
/// tail's `store_chunk8x4` / `store_tail_ragged_avx2` leaves. Reveals
/// `stored` here so those leaves never reveal in their own body.
#[cfg(hax)]
#[hax_lib::fstar::replace(
    r#"
#push-options "--z3rlimit 400"
let lemma_window_stored_single
      (s: t_Array Libcrux_intrinsics.Avx2_extract.t_Vec256 (mk_usize 25))
      (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256)
      (out_m: t_Slice u8)
      (start: usize)
      (lane_m: nat{lane_m < 4})
      (off w: usize)
      (base: nat)
  : Lemma
    (requires
      base < 25 /\ vec == Seq.index s base /\
      v start <= v off /\ v w <= 8 /\ v off + v w <= Seq.length out_m /\
      (v off - v start) / 8 == base /\ (v off - v start) % 8 == 0 /\
      (forall (j_n: nat).
        (v off <= j_n /\ j_n < v off + v w /\ j_n < Seq.length out_m) ==>
        Seq.index out_m j_n == Seq.index
          (Core_models.Num.impl_u64__to_le_bytes (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize lane_m)))
          (j_n - v off)))
    (ensures stored s out_m start (mk_usize lane_m) off (off +! w))
  = reveal_opaque (`%stored) stored;
    let aux (j_n: nat{j_n < Seq.length out_m}) :
      Lemma ((v off <= j_n /\ j_n < v off + v w) ==>
        Seq.index out_m j_n == Seq.index
          (Core_models.Num.impl_u64__to_le_bytes
            (Libcrux_intrinsics.Avx2_extract.get_lane_u64 (Seq.index s ((j_n - v start) / 8)) (mk_usize lane_m)))
          ((j_n - v start) % 8))
      = if v off <= j_n && j_n < v off + v w then begin
          // off - start = 8 * base; j_n - start = (j_n - off) + 8 * base,
          // with 0 <= j_n - off < 8, so /8 == base and %8 == j_n - off.
          FStar.Math.Lemmas.lemma_div_plus (j_n - v off) base 8;
          FStar.Math.Lemmas.lemma_mod_plus (j_n - v off) base 8
        end else ()
    in
    Classical.forall_intro aux
#pop-options
"#
)]
fn lemma_window_stored_single(
    _s: &[Vec256; 25],
    _vec: Vec256,
    _out_m: &[u8],
    _start: usize,
    _lane_m: usize,
    _off: usize,
    _w: usize,
    _base: usize,
) {
}

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
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"
    Seq.length out0 == Seq.length out1 /\
    Seq.length out0 == Seq.length out2 /\
    Seq.length out0 == Seq.length out3 /\
    v i < 6 /\
    v start + 32 * (v i + 1) <= Seq.length out0 /\
    s0 == Seq.index s (4 * v i + 0) /\
    s1 == Seq.index s (4 * v i + 1) /\
    s2 == Seq.index s (4 * v i + 2) /\
    s3 == Seq.index s (4 * v i + 3)
"#))]
#[hax_lib::ensures(|_|
    (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & modifies_range(out0, future(out0), start + 32 * i, start + 32 * (i + 1))
    & modifies_range(out1, future(out1), start + 32 * i, start + 32 * (i + 1))
    & modifies_range(out2, future(out2), start + 32 * i, start + 32 * (i + 1))
    & modifies_range(out3, future(out3), start + 32 * i, start + 32 * (i + 1))
    & stored(s, future(out0), start, 0, start + 32 * i, start + 32 * (i + 1))
    & stored(s, future(out1), start, 1, start + 32 * i, start + 32 * (i + 1))
    & stored(s, future(out2), start, 2, start + 32 * i, start + 32 * (i + 1))
    & stored(s, future(out3), start, 3, start + 32 * i, start + 32 * (i + 1))
)]
fn store_u64x4x4(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    s: &[Vec256; 25],
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
        Classical.forall_intro bridge_out3;
        // Linearise s_k == s[4*i+k] (Euclidean 5*(n/5)+(n%5)=n). The
        // per-output `s_k`-form window facts follow from the bridge
        // (`v_m` form) + the permute/unpack SMTPats. Hand each output to
        // the confined-reveal producers, which package into the opaque
        // `stored` / `modifies_range` predicates WITHOUT revealing here
        // (so these bridge sub-queries stay trivial).
        FStar.Math.Lemmas.lemma_div_mod (4 * v i) 5;
        FStar.Math.Lemmas.lemma_div_mod (4 * v i + 1) 5;
        FStar.Math.Lemmas.lemma_div_mod (4 * v i + 2) 5;
        FStar.Math.Lemmas.lemma_div_mod (4 * v i + 3) 5;
        assert (Seq.index s (4 * v i + 0) == s0);
        assert (Seq.index s (4 * v i + 1) == s1);
        assert (Seq.index s (4 * v i + 2) == s2);
        assert (Seq.index s (4 * v i + 3) == s3);
        lemma_window_stored s s0 s1 s2 s3 start i 0 out0;
        lemma_window_stored s s0 s1 s2 s3 start i 1 out1;
        lemma_window_stored s s0 s1 s2 s3 start i 2 out2;
        lemma_window_stored s s0 s1 s2 s3 start i 3 out3;
        lemma_window_modifies old_out0 out0 start i;
        lemma_window_modifies old_out1 out1 start i;
        lemma_window_modifies old_out2 out2 start i;
        lemma_window_modifies old_out3 out3 start i
        "#
    );
}

/// Inner-loop leaf producer (8-byte chunk) for the tail. Writes the
/// 8-byte window `[off, off+8)` (`off = start+32*q+8*k`) of each output
/// from lane `m` of `vec` (= `s[4*q+k]`, supplied + linked by the
/// caller). The per-byte storeu/copy facts are bridged here, then
/// packaged into the opaque `stored` / `modifies_range` via the
/// confined-reveal lemmas.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"
    Seq.length out0 == Seq.length out1 /\
    Seq.length out0 == Seq.length out2 /\
    Seq.length out0 == Seq.length out3 /\
    4 * v q + v k < 25 /\
    v start + 32 * v q + 8 * (v k + 1) <= Seq.length out0 /\
    vec == Seq.index s (4 * v q + v k)
"#))]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & modifies_range(out0, future(out0), start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & modifies_range(out1, future(out1), start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & modifies_range(out2, future(out2), start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & modifies_range(out3, future(out3), start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & stored(s, future(out0), start, 0, start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & stored(s, future(out1), start, 1, start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & stored(s, future(out2), start, 2, start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
    & stored(s, future(out3), start, 3, start + 32 * q + 8 * k, start + 32 * q + 8 * (k + 1))
)]
fn store_chunk8x4(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    vec: Vec256,
    s: &[Vec256; 25],
    start: usize,
    q: usize,
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
    let off = start + 32 * q + 8 * k;
    out0[off..off + 8].copy_from_slice(&u8s[0..8]);
    out1[off..off + 8].copy_from_slice(&u8s[8..16]);
    out2[off..off + 8].copy_from_slice(&u8s[16..24]);
    out3[off..off + 8].copy_from_slice(&u8s[24..32]);
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 32 * v q + 8 * v k in
        assert (a_pos + 8 <= Seq.length old_out0);
        assert (a_pos + 8 <= Seq.length old_out1);
        assert (a_pos + 8 <= Seq.length old_out2);
        assert (a_pos + 8 <= Seq.length old_out3);
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
              if j_n < a_pos then Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + 8 then
                Seq.index out0 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 0))) (j_n - a_pos)
              else Seq.index out0 j_n == Seq.index old_out0 j_n)
          = bridge_out_m 0 old_out0 out0 j_n
        in
        let bridge_call_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + 8 then
                Seq.index out1 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 1))) (j_n - a_pos)
              else Seq.index out1 j_n == Seq.index old_out1 j_n)
          = bridge_out_m 1 old_out1 out1 j_n
        in
        let bridge_call_out2 (j_n:nat{j_n < Seq.length old_out2}) :
            Lemma (
              if j_n < a_pos then Seq.index out2 j_n == Seq.index old_out2 j_n
              else if j_n < a_pos + 8 then
                Seq.index out2 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 2))) (j_n - a_pos)
              else Seq.index out2 j_n == Seq.index old_out2 j_n)
          = bridge_out_m 2 old_out2 out2 j_n
        in
        let bridge_call_out3 (j_n:nat{j_n < Seq.length old_out3}) :
            Lemma (
              if j_n < a_pos then Seq.index out3 j_n == Seq.index old_out3 j_n
              else if j_n < a_pos + 8 then
                Seq.index out3 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 3))) (j_n - a_pos)
              else Seq.index out3 j_n == Seq.index old_out3 j_n)
          = bridge_out_m 3 old_out3 out3 j_n
        in
        Classical.forall_intro bridge_call_out0;
        Classical.forall_intro bridge_call_out1;
        Classical.forall_intro bridge_call_out2;
        Classical.forall_intro bridge_call_out3;
        // Package per-output window facts into opaque `stored` and the
        // frames into opaque `modifies_range` (reveals confined to the
        // lemmas; base = 4*q+k = (off-start)/8).
        lemma_window_stored_single s vec out0 start 0 off (mk_usize 8) (4 * v q + v k);
        lemma_window_stored_single s vec out1 start 1 off (mk_usize 8) (4 * v q + v k);
        lemma_window_stored_single s vec out2 start 2 off (mk_usize 8) (4 * v q + v k);
        lemma_window_stored_single s vec out3 start 3 off (mk_usize 8) (4 * v q + v k);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out0 out0 off (off +! mk_usize 8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out1 out1 off (off +! mk_usize 8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out2 out2 off (off +! mk_usize 8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out3 out3 off (off +! mk_usize 8)
        "#
    );
}

/// Ragged leaf producer for the tail's final `rem8 < 8` bytes. Writes
/// `[off, off+rem8)` (`off = start+32*q+8*chunks8`) of each output from
/// lane `m` of `vec` (= `s[4*q+chunks8]`, supplied + linked). Per-byte
/// bridge then packaged into opaque `stored` / `modifies_range`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"
    Seq.length out0 == Seq.length out1 /\
    Seq.length out0 == Seq.length out2 /\
    Seq.length out0 == Seq.length out3 /\
    v rem8 > 0 /\ v rem8 < 8 /\
    4 * v q + v chunks8 < 25 /\
    v start + 32 * v q + 8 * v chunks8 + v rem8 <= Seq.length out0 /\
    vec == Seq.index s (4 * v q + v chunks8)
"#))]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & modifies_range(out0, future(out0), start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & modifies_range(out1, future(out1), start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & modifies_range(out2, future(out2), start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & modifies_range(out3, future(out3), start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & stored(s, future(out0), start, 0, start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & stored(s, future(out1), start, 1, start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & stored(s, future(out2), start, 2, start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
    & stored(s, future(out3), start, 3, start + 32 * q + 8 * chunks8, start + 32 * q + 8 * chunks8 + rem8)
)]
fn store_tail_ragged_avx2(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    vec: Vec256,
    s: &[Vec256; 25],
    start: usize,
    q: usize,
    chunks8: usize,
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
    let off = start + 32 * q + 8 * chunks8;
    out0[off..off + rem8].copy_from_slice(&u8s[0..rem8]);
    out1[off..off + rem8].copy_from_slice(&u8s[8..8 + rem8]);
    out2[off..off + rem8].copy_from_slice(&u8s[16..16 + rem8]);
    out3[off..off + rem8].copy_from_slice(&u8s[24..24 + rem8]);
    hax_lib::fstar!(
        r#"
        let a_pos:nat = v start + 32 * v q + 8 * v chunks8 in
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
              if j_n < a_pos then Seq.index out0 j_n == Seq.index old_out0 j_n
              else if j_n < a_pos + r then
                Seq.index out0 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 0))) (j_n - a_pos)
              else Seq.index out0 j_n == Seq.index old_out0 j_n)
          = bridge_partial_out_m 0 old_out0 out0 j_n
        in
        let bridge_call_out1 (j_n:nat{j_n < Seq.length old_out1}) :
            Lemma (
              if j_n < a_pos then Seq.index out1 j_n == Seq.index old_out1 j_n
              else if j_n < a_pos + r then
                Seq.index out1 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 1))) (j_n - a_pos)
              else Seq.index out1 j_n == Seq.index old_out1 j_n)
          = bridge_partial_out_m 1 old_out1 out1 j_n
        in
        let bridge_call_out2 (j_n:nat{j_n < Seq.length old_out2}) :
            Lemma (
              if j_n < a_pos then Seq.index out2 j_n == Seq.index old_out2 j_n
              else if j_n < a_pos + r then
                Seq.index out2 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 2))) (j_n - a_pos)
              else Seq.index out2 j_n == Seq.index old_out2 j_n)
          = bridge_partial_out_m 2 old_out2 out2 j_n
        in
        let bridge_call_out3 (j_n:nat{j_n < Seq.length old_out3}) :
            Lemma (
              if j_n < a_pos then Seq.index out3 j_n == Seq.index old_out3 j_n
              else if j_n < a_pos + r then
                Seq.index out3 j_n == Seq.index
                  (Core_models.Num.impl_u64__to_le_bytes
                     (Libcrux_intrinsics.Avx2_extract.get_lane_u64 vec (mk_usize 3))) (j_n - a_pos)
              else Seq.index out3 j_n == Seq.index old_out3 j_n)
          = bridge_partial_out_m 3 old_out3 out3 j_n
        in
        Classical.forall_intro bridge_call_out0;
        Classical.forall_intro bridge_call_out1;
        Classical.forall_intro bridge_call_out2;
        Classical.forall_intro bridge_call_out3;
        lemma_window_stored_single s vec out0 start 0 off rem8 (4 * v q + v chunks8);
        lemma_window_stored_single s vec out1 start 1 off rem8 (4 * v q + v chunks8);
        lemma_window_stored_single s vec out2 start 2 off rem8 (4 * v q + v chunks8);
        lemma_window_stored_single s vec out3 start 3 off rem8 (4 * v q + v chunks8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out0 out0 off (off +! rem8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out1 out1 off (off +! rem8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out2 out2 off (off +! rem8);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_intro old_out3 out3 off (off +! rem8)
        "#
    );
}

/// Outer-loop half of `store_block`: writes the full 32-byte windows
/// `[start, start+32*q)` by calling `store_u64x4x4` per iteration.
/// Verified via an opaque-`stored`/`modifies_range` loop invariant with
/// per-iteration frame/union carryover.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && out0.len() == out2.len()
    && out0.len() == out3.len()
    && q <= 6
    && start.to_int() + (32.to_int() * q.to_int()) <= out0.len().to_int()
)]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & modifies_range(out0, future(out0), start, start + 32 * q)
    & modifies_range(out1, future(out1), start, start + 32 * q)
    & modifies_range(out2, future(out2), start, start + 32 * q)
    & modifies_range(out3, future(out3), start, start + 32 * q)
    & stored(s, future(out0), start, 0, start, start + 32 * q)
    & stored(s, future(out1), start, 1, start, start + 32 * q)
    & stored(s, future(out2), start, 2, start, start + 32 * q)
    & stored(s, future(out3), start, 3, start, start + 32 * q)
)]
fn store_block_full_avx2(
    s: &[Vec256; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    start: usize,
    q: usize,
) {
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let old_out2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let old_out3 = out3.to_vec().as_slice();
    hax_lib::fstar!(
        r#"
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
        assert (old_out0 == out0); assert (old_out1 == out1);
        assert (old_out2 == out2); assert (old_out3 == out3);
        // Seed the loop-invariant base case (i = 0): empty `stored`
        // ranges and reflexive `modifies_range` (out_m unchanged so far).
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out0 start start;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out1 start start;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out2 start start;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out3 start start;
        lemma_stored_empty s out0 start (mk_usize 0) start;
        lemma_stored_empty s out1 start (mk_usize 1) start;
        lemma_stored_empty s out2 start (mk_usize 2) start;
        lemma_stored_empty s out3 start (mk_usize 3) start
        "#
    );
    for i in 0..q {
        hax_lib::loop_invariant!(|i: usize| (out0.len() == old_out0.len()).to_prop()
            & (out1.len() == old_out1.len()).to_prop()
            & (out2.len() == old_out2.len()).to_prop()
            & (out3.len() == old_out3.len()).to_prop()
            & modifies_range(old_out0, out0, start, start + 32 * i)
            & modifies_range(old_out1, out1, start, start + 32 * i)
            & modifies_range(old_out2, out2, start, start + 32 * i)
            & modifies_range(old_out3, out3, start, start + 32 * i)
            & stored(s, out0, start, 0, start, start + 32 * i)
            & stored(s, out1, start, 1, start, start + 32 * i)
            & stored(s, out2, start, 2, start, start + 32 * i)
            & stored(s, out3, start, 3, start, start + 32 * i));
        #[cfg(hax)]
        let p0 = out0.to_vec().as_slice();
        #[cfg(hax)]
        let p1 = out1.to_vec().as_slice();
        #[cfg(hax)]
        let p2 = out2.to_vec().as_slice();
        #[cfg(hax)]
        let p3 = out3.to_vec().as_slice();
        hax_lib::fstar!(
            r#"
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
            assert (p0 == out0); assert (p1 == out1); assert (p2 == out2); assert (p3 == out3);
            // Discharge store_u64x4x4's `s_k == s[4*i+k]` link: get_ij
            // linearises 5*((4i+k)/5)+(4i+k)%5 == 4*i+k (Euclidean).
            FStar.Math.Lemmas.lemma_div_mod (4 * v i) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 1) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 2) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 3) 5
            "#
        );
        store_u64x4x4(
            out0,
            out1,
            out2,
            out3,
            s,
            *get_ij(s, (4 * i) / 5, (4 * i) % 5),
            *get_ij(s, (4 * i + 1) / 5, (4 * i + 1) % 5),
            *get_ij(s, (4 * i + 2) / 5, (4 * i + 2) % 5),
            *get_ij(s, (4 * i + 3) / 5, (4 * i + 3) % 5),
            start,
            i,
        );
        // Extend the invariant from [start, start+32*i) to
        // [start, start+32*(i+1)): carry the prefix past this window's
        // modification, union the two `stored` ranges, union the frames.
        // All opaque (no reveal).
        hax_lib::fstar!(
            r#"
            let lo:usize = start in
            let mid:usize = start +! (mk_usize 32 *! i) in
            let hi:usize = start +! (mk_usize 32 *! (i +! mk_usize 1)) in
            assert (v lo <= v mid /\ v mid <= v hi);
            lemma_stored_frame s p0 out0 start (mk_usize 0) lo mid mid hi;
            lemma_stored_frame s p1 out1 start (mk_usize 1) lo mid mid hi;
            lemma_stored_frame s p2 out2 start (mk_usize 2) lo mid mid hi;
            lemma_stored_frame s p3 out3 start (mk_usize 3) lo mid mid hi;
            lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
            lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
            lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
            lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out0 p0 out0 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out1 p1 out1 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out2 p2 out2 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out3 p3 out3 lo mid hi
            "#
        );
    }
}

/// Tail half of `store_block`: writes the partial window
/// `[start+32*q, start+32*q+rem)` (`rem < 32`) via the inner 8-byte
/// loop (`store_chunk8x4`) and the ragged remainder
/// (`store_tail_ragged_avx2`), composing their opaque posts.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'")]
#[hax_lib::requires(
    out0.len() == out1.len()
    && out0.len() == out2.len()
    && out0.len() == out3.len()
    && q <= 6
    && rem < 32
    && 4 * q + rem / 8 < 25
    && start.to_int() + (32.to_int() * q.to_int()) + rem.to_int() <= out0.len().to_int()
)]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()).to_prop()
    & (future(out1).len() == out1.len()).to_prop()
    & (future(out2).len() == out2.len()).to_prop()
    & (future(out3).len() == out3.len()).to_prop()
    & modifies_range(out0, future(out0), start + 32 * q, start + 32 * q + rem)
    & modifies_range(out1, future(out1), start + 32 * q, start + 32 * q + rem)
    & modifies_range(out2, future(out2), start + 32 * q, start + 32 * q + rem)
    & modifies_range(out3, future(out3), start + 32 * q, start + 32 * q + rem)
    & stored(s, future(out0), start, 0, start + 32 * q, start + 32 * q + rem)
    & stored(s, future(out1), start, 1, start + 32 * q, start + 32 * q + rem)
    & stored(s, future(out2), start, 2, start + 32 * q, start + 32 * q + rem)
    & stored(s, future(out3), start, 3, start + 32 * q, start + 32 * q + rem)
)]
fn store_block_tail_avx2(
    s: &[Vec256; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    start: usize,
    q: usize,
    rem: usize,
) {
    #[cfg(hax)]
    let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let old_out1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let old_out2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let old_out3 = out3.to_vec().as_slice();
    hax_lib::fstar!(
        r#"
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
        assert (old_out0 == out0); assert (old_out1 == out1);
        assert (old_out2 == out2); assert (old_out3 == out3);
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out0 (start +! (mk_usize 32 *! q)) (start +! (mk_usize 32 *! q));
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out1 (start +! (mk_usize 32 *! q)) (start +! (mk_usize 32 *! q));
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out2 (start +! (mk_usize 32 *! q)) (start +! (mk_usize 32 *! q));
        Libcrux_sha3.Proof_utils.lemma_modifies_range_refl out3 (start +! (mk_usize 32 *! q)) (start +! (mk_usize 32 *! q));
        lemma_stored_empty s out0 start (mk_usize 0) (start +! (mk_usize 32 *! q));
        lemma_stored_empty s out1 start (mk_usize 1) (start +! (mk_usize 32 *! q));
        lemma_stored_empty s out2 start (mk_usize 2) (start +! (mk_usize 32 *! q));
        lemma_stored_empty s out3 start (mk_usize 3) (start +! (mk_usize 32 *! q))
        "#
    );
    let chunks8 = rem / 8;
    for k in 0..chunks8 {
        hax_lib::loop_invariant!(|k: usize| (out0.len() == old_out0.len()).to_prop()
            & (out1.len() == old_out1.len()).to_prop()
            & (out2.len() == old_out2.len()).to_prop()
            & (out3.len() == old_out3.len()).to_prop()
            & modifies_range(old_out0, out0, start + 32 * q, start + 32 * q + 8 * k)
            & modifies_range(old_out1, out1, start + 32 * q, start + 32 * q + 8 * k)
            & modifies_range(old_out2, out2, start + 32 * q, start + 32 * q + 8 * k)
            & modifies_range(old_out3, out3, start + 32 * q, start + 32 * q + 8 * k)
            & stored(s, out0, start, 0, start + 32 * q, start + 32 * q + 8 * k)
            & stored(s, out1, start, 1, start + 32 * q, start + 32 * q + 8 * k)
            & stored(s, out2, start, 2, start + 32 * q, start + 32 * q + 8 * k)
            & stored(s, out3, start, 3, start + 32 * q, start + 32 * q + 8 * k));
        #[cfg(hax)]
        let p0 = out0.to_vec().as_slice();
        #[cfg(hax)]
        let p1 = out1.to_vec().as_slice();
        #[cfg(hax)]
        let p2 = out2.to_vec().as_slice();
        #[cfg(hax)]
        let p3 = out3.to_vec().as_slice();
        hax_lib::fstar!(
            r#"
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
            assert (p0 == out0); assert (p1 == out1); assert (p2 == out2); assert (p3 == out3);
            FStar.Math.Lemmas.lemma_div_mod (4 * v q + v k) 5
            "#
        );
        store_chunk8x4(
            out0,
            out1,
            out2,
            out3,
            *get_ij(s, (4 * q + k) / 5, (4 * q + k) % 5),
            s,
            start,
            q,
            k,
        );
        hax_lib::fstar!(
            r#"
            let lo:usize = start +! (mk_usize 32 *! q) in
            let mid:usize = (start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! k) in
            let hi:usize = (start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! (k +! mk_usize 1)) in
            assert (v lo <= v mid /\ v mid <= v hi);
            lemma_stored_frame s p0 out0 start (mk_usize 0) lo mid mid hi;
            lemma_stored_frame s p1 out1 start (mk_usize 1) lo mid mid hi;
            lemma_stored_frame s p2 out2 start (mk_usize 2) lo mid mid hi;
            lemma_stored_frame s p3 out3 start (mk_usize 3) lo mid mid hi;
            lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
            lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
            lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
            lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out0 p0 out0 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out1 p1 out1 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out2 p2 out2 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out3 p3 out3 lo mid hi
            "#
        );
    }
    let rem8 = rem % 8;
    if rem8 > 0 {
        #[cfg(hax)]
        let r0 = out0.to_vec().as_slice();
        #[cfg(hax)]
        let r1 = out1.to_vec().as_slice();
        #[cfg(hax)]
        let r2 = out2.to_vec().as_slice();
        #[cfg(hax)]
        let r3 = out3.to_vec().as_slice();
        hax_lib::fstar!(
            r#"
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
            assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
            assert (r0 == out0); assert (r1 == out1); assert (r2 == out2); assert (r3 == out3);
            FStar.Math.Lemmas.lemma_div_mod (4 * v q + v chunks8) 5
            "#
        );
        store_tail_ragged_avx2(
            out0,
            out1,
            out2,
            out3,
            *get_ij(s, (4 * q + chunks8) / 5, (4 * q + chunks8) % 5),
            s,
            start,
            q,
            chunks8,
            rem8,
        );
        hax_lib::fstar!(
            r#"
            FStar.Math.Lemmas.lemma_div_mod (v rem) 8;
            let lo:usize = start +! (mk_usize 32 *! q) in
            let mid:usize = (start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! chunks8) in
            let hi:usize = (start +! (mk_usize 32 *! q)) +! rem in
            assert (v mid + v rem8 == v hi);
            assert (mid +! rem8 == hi);
            assert (v lo <= v mid /\ v mid <= v hi);
            lemma_stored_frame s r0 out0 start (mk_usize 0) lo mid mid hi;
            lemma_stored_frame s r1 out1 start (mk_usize 1) lo mid mid hi;
            lemma_stored_frame s r2 out2 start (mk_usize 2) lo mid mid hi;
            lemma_stored_frame s r3 out3 start (mk_usize 3) lo mid mid hi;
            lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
            lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
            lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
            lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out0 r0 out0 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out1 r1 out1 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out2 r2 out2 lo mid hi;
            Libcrux_sha3.Proof_utils.lemma_modifies_range_union old_out3 r3 out3 lo mid hi
            "#
        );
    } else {
        hax_lib::fstar!(
            r#"
            FStar.Math.Lemmas.lemma_div_mod (v rem) 8;
            assert (v rem == 8 * v chunks8);
            assert ((start +! (mk_usize 32 *! q)) +! (mk_usize 8 *! chunks8)
                    == (start +! (mk_usize 32 *! q)) +! rem)
            "#
        );
    }
}

/// Composer (top of the store_block proof). Splits `len = 32*chunks + rem`,
/// calls the full and tail halves, and composes their opaque
/// `stored` / `modifies_range` posts over the adjacent ranges
/// `[start, start+32*chunks)` and `[start+32*chunks, start+len)` into a
/// single `stored` / `modifies_range` over `[start, start+len)` via the
/// frame lemmas. No `reveal` here — the byte content stays opaque.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
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
    & modifies_range(out0, future(out0), start, start + len)
    & modifies_range(out1, future(out1), start, start + len)
    & modifies_range(out2, future(out2), start, start + len)
    & modifies_range(out3, future(out3), start, start + len)
    & stored(s, future(out0), start, 0, start, start + len)
    & stored(s, future(out1), start, 1, start, start + len)
    & stored(s, future(out2), start, 2, start, start + len)
    & stored(s, future(out3), start, 3, start, start + len)
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
    let chunks = len / 32;
    let rem = len % 32;
    #[cfg(hax)]
    let e0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let e1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let e2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let e3 = out3.to_vec().as_slice();
    hax_lib::fstar!(
        r#"
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
        assert (e0 == out0); assert (e1 == out1); assert (e2 == out2); assert (e3 == out3);
        assert (v len == 32 * v chunks + v rem);
        assert (v chunks <= 6);
        assert (v rem < 32);
        assert (v start + 32 * v chunks + v rem == v start + v len);
        assert (v start + 32 * v chunks <= v start + v len);
        // Discharge the tail's state-word index bound 4*chunks + rem/8 < 25
        // (= len/8 < 25): len/8 = rem/8 + 4*chunks (lemma_div_plus, since
        // 32*chunks = 8*(4*chunks)), and len <= RATE < 200.
        assert (v len == v rem + (4 * v chunks) * 8);
        FStar.Math.Lemmas.lemma_div_plus (v rem) (4 * v chunks) 8;
        assert (v len / 8 == v rem / 8 + 4 * v chunks);
        assert (v len < 200);
        assert (4 * v chunks + v rem / 8 < 25)
        "#
    );
    store_block_full_avx2(s, out0, out1, out2, out3, start, chunks);
    #[cfg(hax)]
    let mid0 = out0.to_vec().as_slice();
    #[cfg(hax)]
    let mid1 = out1.to_vec().as_slice();
    #[cfg(hax)]
    let mid2 = out2.to_vec().as_slice();
    #[cfg(hax)]
    let mid3 = out3.to_vec().as_slice();
    hax_lib::fstar!(
        r#"
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out0) == out0);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out1) == out1);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out2) == out2);
        assert_norm (Alloc.Vec.impl_1__as_slice (Alloc.Slice.impl__to_vec out3) == out3);
        assert (mid0 == out0); assert (mid1 == out1); assert (mid2 == out2); assert (mid3 == out3)
        "#
    );
    store_block_tail_avx2(s, out0, out1, out2, out3, start, chunks, rem);
    // Compose: full covers [start, start+32*chunks), tail covers
    // [start+32*chunks, start+len). Carry full's `stored` past tail's
    // window-modification (frame), then union the two `stored` ranges
    // and the two `modifies_range`s. All opaque; no reveal.
    hax_lib::fstar!(
        r#"
        let lo:usize = start in
        let mid:usize = start +! (mk_usize 32 *! chunks) in
        let hi:usize = start +! len in
        assert (v mid + v rem == v hi);
        assert (mid +! rem == hi);
        assert (v lo <= v mid /\ v mid <= v hi);
        lemma_stored_frame s mid0 out0 start (mk_usize 0) lo mid mid hi;
        lemma_stored_frame s mid1 out1 start (mk_usize 1) lo mid mid hi;
        lemma_stored_frame s mid2 out2 start (mk_usize 2) lo mid mid hi;
        lemma_stored_frame s mid3 out3 start (mk_usize 3) lo mid mid hi;
        lemma_stored_union s out0 start (mk_usize 0) lo mid hi;
        lemma_stored_union s out1 start (mk_usize 1) lo mid hi;
        lemma_stored_union s out2 start (mk_usize 2) lo mid hi;
        lemma_stored_union s out3 start (mk_usize 3) lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union e0 mid0 out0 lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union e1 mid1 out1 lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union e2 mid2 out2 lo mid hi;
        Libcrux_sha3.Proof_utils.lemma_modifies_range_union e3 mid3 out3 lo mid hi
        "#
    );
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
