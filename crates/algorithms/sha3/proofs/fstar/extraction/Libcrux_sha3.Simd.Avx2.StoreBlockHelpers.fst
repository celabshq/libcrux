module Libcrux_sha3.Simd.Avx2.StoreBlockHelpers
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Rust_primitives
open Libcrux_intrinsics.Avx2_extract

/// Generic per-byte bridge for `update_at_range` composed with
/// `mm256_storeu_si256_u8`. Given the abstract facts that
///
///   - `Seq.slice out' 0 a == Seq.slice out 0 a`           (prefix preserved)
///   - For each `k < 32`: `Seq.index (Seq.slice out' a (a+32)) k`
///       equals byte `k % 8` of `to_le_bytes (get_lane_u64 vec (mk_usize (k / 8)))`
///   - `Seq.slice out' (a+32) (length out') == Seq.slice out (a+32) (length out)`
///                                                          (suffix preserved)
///
/// the lemma derives a **per-absolute-index byte fact** for any `j`.
/// This is the AVX2 analog of `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.store_block_window_byte`
/// generalised from 16-byte / 2-lane to 32-byte / 4-lane.
val store_block_window_byte
    (out: Seq.seq u8)
    (out': Seq.seq u8)
    (vec: t_Vec256)
    (a: nat)
    (j: nat)
  : Lemma
    (requires
        a + 32 <= Seq.length out /\
        Seq.length out' == Seq.length out /\
        j < Seq.length out' /\
        Seq.slice out' 0 a == Seq.slice out 0 a /\
        (forall (k:nat{k < 32}).
            Seq.index (Seq.slice out' a (a + 32)) k ==
            Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k / 8))))
              (k % 8)) /\
        Seq.slice out' (a + 32) (Seq.length out')
          == Seq.slice out (a + 32) (Seq.length out))
    (ensures
        (if j < a then
           Seq.index out' j == Seq.index out j
         else if j < a + 32 then
           Seq.index out' j ==
             Seq.index
               (Core_models.Num.impl_u64__to_le_bytes
                  (get_lane_u64 vec (mk_usize ((j - a) / 8))))
               ((j - a) % 8)
         else
           Seq.index out' j == Seq.index out j))

let store_block_window_byte out out' vec a j =
  if j < a then
    (assert (Seq.index (Seq.slice out' 0 a) j == Seq.index out' j);
     assert (Seq.index (Seq.slice out  0 a) j == Seq.index out  j))
  else if j < a + 32 then
    let k:nat = j - a in
    assert (k < 32);
    assert (Seq.index (Seq.slice out' a (a + 32)) k == Seq.index out' j)
  else
    let k:nat = j - (a + 32) in
    assert (k < Seq.length out' - (a + 32));
    assert (Seq.index (Seq.slice out' (a + 32) (Seq.length out')) k == Seq.index out' j);
    assert (Seq.index (Seq.slice out  (a + 32) (Seq.length out )) k == Seq.index out  j)

/// Convenience wrapper: from the raw `update_at_range`-output slice
/// `out_new` and the raw `mm256_storeu_si256_u8`-output slice
/// `store_res = mm256_storeu_si256_u8 (Seq.slice out a (a+32)) vec`,
/// derive the per-byte fact about `out_new[j]`. This is the form the
/// store_block loop body has in scope.
val store_block_window_byte_of_storeu
    (out out_new store_res: Seq.seq u8)
    (vec: t_Vec256)
    (a: nat)
    (j: nat)
  : Lemma
    (requires
        a + 32 <= Seq.length out /\
        Seq.length store_res == 32 /\
        Seq.length out_new == Seq.length out /\
        j < Seq.length out_new /\
        Seq.slice out_new 0 a == Seq.slice out 0 a /\
        Seq.slice out_new a (a + 32) == store_res /\
        Seq.slice out_new (a + 32) (Seq.length out_new)
          == Seq.slice out (a + 32) (Seq.length out) /\
        (forall (k:nat{k < 32}).
            Seq.index store_res k ==
            Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k / 8))))
              (k % 8)))
    (ensures
        (if j < a then
           Seq.index out_new j == Seq.index out j
         else if j < a + 32 then
           Seq.index out_new j ==
             Seq.index
               (Core_models.Num.impl_u64__to_le_bytes
                  (get_lane_u64 vec (mk_usize ((j - a) / 8))))
               ((j - a) % 8)
         else
           Seq.index out_new j == Seq.index out j))

let store_block_window_byte_of_storeu out out_new store_res vec a j =
  // Re-expose the window-content forall in the form
  //   Seq.index (Seq.slice out_new a (a+32)) k == byte k of to_le_bytes...
  // since Seq.slice out_new a (a+32) == store_res.
  assert (Seq.slice out_new a (a + 32) == store_res);
  introduce forall (k:nat{k < 32}).
              Seq.index (Seq.slice out_new a (a + 32)) k ==
              Seq.index
                (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k / 8))))
                (k % 8)
  with begin
    assert (Seq.index (Seq.slice out_new a (a + 32)) k == Seq.index store_res k)
  end;
  store_block_window_byte out out_new vec a j
