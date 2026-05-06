module Libcrux_sha3.Simd.Arm64.StoreBlockHelpers
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Rust_primitives
open Libcrux_intrinsics.Arm64_extract

/// Generic per-byte bridge for `update_at_range` composed with
/// `e_vst1q_bytes_u64`. Given the abstract facts that
///
///   - `Seq.slice out' 0 a == Seq.slice out 0 a`           (prefix preserved)
///   - For each `k < 16`: `Seq.index (Seq.slice out' a (a+16)) k`
///       equals byte `k % 8` of `to_le_bytes (get_lane_u64x2 v (k / 8))`
///   - `Seq.slice out' (a+16) (length out') == Seq.slice out (a+16) (length out)`
///                                                          (suffix preserved)
///
/// the lemma derives a **per-absolute-index byte fact** for any `j`.
/// This is exactly the shape the store_block loop body needs to advance
/// its byte-level invariant by 16 bytes per iteration: outside the
/// window, `out'[j] == out[j]`; inside the window, `out'[j]` equals the
/// `(j-a)%8`-th byte of `to_le_bytes (get_lane_u64x2 v ((j-a)/8))`.
val store_block_window_byte
    (out: Seq.seq u8)
    (out': Seq.seq u8)
    (v: t_e_uint64x2_t)
    (a: nat)
    (j: nat)
  : Lemma
    (requires
        a + 16 <= Seq.length out /\
        Seq.length out' == Seq.length out /\
        j < Seq.length out' /\
        Seq.slice out' 0 a == Seq.slice out 0 a /\
        (forall (k:nat{k < 16}).
            Seq.index (Seq.slice out' a (a + 16)) k ==
            Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x2 v (k / 8)))
              (k % 8)) /\
        Seq.slice out' (a + 16) (Seq.length out')
          == Seq.slice out (a + 16) (Seq.length out))
    (ensures
        (if j < a then
           Seq.index out' j == Seq.index out j
         else if j < a + 16 then
           Seq.index out' j ==
             Seq.index
               (Core_models.Num.impl_u64__to_le_bytes
                  (get_lane_u64x2 v ((j - a) / 8)))
               ((j - a) % 8)
         else
           Seq.index out' j == Seq.index out j))

let store_block_window_byte out out' v a j =
  if j < a then
    (assert (Seq.index (Seq.slice out' 0 a) j == Seq.index out' j);
     assert (Seq.index (Seq.slice out  0 a) j == Seq.index out  j))
  else if j < a + 16 then
    let k:nat = j - a in
    assert (k < 16);
    assert (Seq.index (Seq.slice out' a (a + 16)) k == Seq.index out' j)
  else
    let k:nat = j - (a + 16) in
    assert (k < Seq.length out' - (a + 16));
    assert (Seq.index (Seq.slice out' (a + 16) (Seq.length out')) k == Seq.index out' j);
    assert (Seq.index (Seq.slice out  (a + 16) (Seq.length out )) k == Seq.index out  j)

/// Convenience wrapper: from the raw `update_at_range`-output slice
/// `out_new` and the raw `e_vst1q_bytes_u64`-output slice
/// `vst_res = e_vst1q_bytes_u64 (Seq.slice out a (a+16)) v`,
/// derive the per-byte fact about `out_new[j]`. This is the form the
/// store_block loop body has in scope.
val store_block_window_byte_of_vst
    (out out_new vst_res: Seq.seq u8)
    (v: t_e_uint64x2_t)
    (a: nat)
    (j: nat)
  : Lemma
    (requires
        a + 16 <= Seq.length out /\
        Seq.length vst_res == 16 /\
        Seq.length out_new == Seq.length out /\
        j < Seq.length out_new /\
        // From update_at_range's post:
        Seq.slice out_new 0 a == Seq.slice out 0 a /\
        Seq.slice out_new a (a + 16) == vst_res /\
        Seq.slice out_new (a + 16) (Seq.length out_new)
          == Seq.slice out (a + 16) (Seq.length out) /\
        // From e_vst1q_bytes_u64's post:
        (forall (k:nat{k < 16}).
            Seq.index vst_res k ==
            Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x2 v (k / 8)))
              (k % 8)))
    (ensures
        (if j < a then
           Seq.index out_new j == Seq.index out j
         else if j < a + 16 then
           Seq.index out_new j ==
             Seq.index
               (Core_models.Num.impl_u64__to_le_bytes
                  (get_lane_u64x2 v ((j - a) / 8)))
               ((j - a) % 8)
         else
           Seq.index out_new j == Seq.index out j))

let store_block_window_byte_of_vst out out_new vst_res v a j =
  // Re-expose the window-content forall in the form
  //   Seq.index (Seq.slice out_new a (a+16)) k == byte k of to_le_bytes...
  // since Seq.slice out_new a (a+16) == vst_res.
  assert (Seq.slice out_new a (a + 16) == vst_res);
  introduce forall (k:nat{k < 16}).
              Seq.index (Seq.slice out_new a (a + 16)) k ==
              Seq.index
                (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x2 v (k / 8)))
                (k % 8)
  with begin
    assert (Seq.index (Seq.slice out_new a (a + 16)) k == Seq.index vst_res k)
  end;
  store_block_window_byte out out_new v a j

/// Synthetic test: demonstrates that the convenience wrapper fires on a
/// realistic call-site shape (the store_block loop body emits exactly
/// these intermediate facts after one `update_at_range` / `e_vst1q_bytes_u64`
/// pair). The lemma below is the Z3-side check that helper 2 closes the
/// gap from a single iteration to the per-byte view of the resulting slice.
val test_store_block_loop_step
    (out out_new vst_res: Seq.seq u8)
    (v: t_e_uint64x2_t)
    (start: nat)
    (i: nat)
    (j: nat)
  : Lemma
    (requires
        start + 16 * (i + 1) <= Seq.length out /\
        Seq.length vst_res == 16 /\
        Seq.length out_new == Seq.length out /\
        j < Seq.length out_new /\
        Seq.slice out_new 0 (start + 16 * i) == Seq.slice out 0 (start + 16 * i) /\
        Seq.slice out_new (start + 16 * i) (start + 16 * i + 16) == vst_res /\
        Seq.slice out_new (start + 16 * i + 16) (Seq.length out_new)
          == Seq.slice out (start + 16 * i + 16) (Seq.length out) /\
        (forall (k:nat{k < 16}).
            Seq.index vst_res k ==
            Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64x2 v (k / 8)))
              (k % 8)))
    (ensures
        (let a = start + 16 * i in
         if j < a then
           Seq.index out_new j == Seq.index out j
         else if j < a + 16 then
           Seq.index out_new j ==
             Seq.index
               (Core_models.Num.impl_u64__to_le_bytes
                  (get_lane_u64x2 v ((j - a) / 8)))
               ((j - a) % 8)
         else
           Seq.index out_new j == Seq.index out j))

let test_store_block_loop_step out out_new vst_res v start i j =
  let a:nat = start + 16 * i in
  store_block_window_byte_of_vst out out_new vst_res v a j
