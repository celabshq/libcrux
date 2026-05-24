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

/// One-call wrapper that fuses the per-byte forall (from the
/// strengthened `mm256_storeu_si256_u8` SMTPat) with
/// `store_block_window_byte_of_storeu`. The bridge call site never
/// has to discharge the 32-byte `forall` precondition itself; the
/// SMTPat fires only inside this helper's body, isolated from any
/// other storeu calls in the caller's scope. This breaks the
/// 4-storeus-in-one-scope quantifier cliff seen in `store_u64x4x4`.
val store_block_window_byte_of_storeu_call
    (out out_new: Seq.seq u8)
    (vec: t_Vec256)
    (a: nat)
    (j: nat)
  : Lemma
    (requires
        a + 32 <= Seq.length out /\
        Seq.length out_new == Seq.length out /\
        j < Seq.length out_new /\
        Seq.slice out_new 0 a == Seq.slice out 0 a /\
        Seq.slice out_new a (a + 32) ==
          mm256_storeu_si256_u8 (Seq.slice out a (a + 32)) vec /\
        Seq.slice out_new (a + 32) (Seq.length out_new)
          == Seq.slice out (a + 32) (Seq.length out))
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

let store_block_window_byte_of_storeu_call out out_new vec a j =
  let store_res = mm256_storeu_si256_u8 (Seq.slice out a (a + 32)) vec in
  // Sanity: the intrinsic's length post.
  assert (Seq.length store_res == 32);
  // Drive the per-byte SMTPat instantiation manually so it fires
  // here against the single concrete `store_res` in scope; the
  // caller may have several other `mm256_storeu_si256_u8` calls
  // active, and an unguided e-matching saturation across all of
  // them is what previously cliffed `store_u64x4x4`.
  introduce forall (k:nat{k < 32}).
              Seq.index store_res k ==
              Seq.index
                (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k / 8))))
                (k % 8)
  with begin
    Libcrux_intrinsics.Avx2_extract.lemma_mm256_storeu_si256_u8_byte
      (Seq.slice out a (a + 32)) vec k
  end;
  store_block_window_byte_of_storeu out out_new store_res vec a j

/// Materialises the per-byte forall over a 32-byte scratch buffer
/// produced directly by `mm256_storeu_si256_u8` (e.g. when the
/// store_block tail body writes into a local `[0u8; 32]`). Packages
/// 32 SMTPat firings of `lemma_mm256_storeu_si256_u8_byte` into one
/// `forall` so the caller does not need to coax e-matching.
val mm256_storeu_si256_u8_byte_window
    (init: Seq.seq u8)
    (vec: t_Vec256)
  : Lemma
    (requires Seq.length init == 32)
    (ensures
        Seq.length (mm256_storeu_si256_u8 init vec) == 32 /\
        (forall (k:nat{k < 32}).
            Seq.index (mm256_storeu_si256_u8 init vec) k ==
            Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k / 8))))
              (k % 8)))

let mm256_storeu_si256_u8_byte_window init vec =
  let u8s = mm256_storeu_si256_u8 init vec in
  assert (Seq.length u8s == 32);
  introduce forall (k:nat{k < 32}).
              Seq.index u8s k ==
              Seq.index
                (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k / 8))))
                (k % 8)
  with begin
    Libcrux_intrinsics.Avx2_extract.lemma_mm256_storeu_si256_u8_byte init vec k
  end
