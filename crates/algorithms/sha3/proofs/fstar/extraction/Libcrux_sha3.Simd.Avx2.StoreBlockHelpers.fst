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

/// Layer 4 per-(j,lane) bridge: from `store_u64x4x4`'s 4-branch `s_k`
/// post at a single absolute byte index `j` in the window
/// `[start+32*i, start+32*(i+1))`, plus the linearisation
/// `s_k == s[4*i+k]`, conclude the unified `s[(j-start)/8]` form.
/// Verified in isolation so the outer-loop body never sees the 4x4
/// cross-product.
val lemma_lane_chain_to_s
    (s: Seq.seq t_Vec256)
    (s0 s1 s2 s3: t_Vec256)
    (start: nat)
    (i: nat)
    (lane_m: nat{lane_m < 4})
    (j: nat)
    (out_byte: u8)
  : Lemma
    (requires
        Seq.length s >= 4 * i + 4 /\
        start + 32 * i <= j /\ j < start + 32 * (i + 1) /\
        s0 == Seq.index s (4 * i + 0) /\
        s1 == Seq.index s (4 * i + 1) /\
        s2 == Seq.index s (4 * i + 2) /\
        s3 == Seq.index s (4 * i + 3) /\
        ((j - start) / 8 == 4 * i + 0 ==>
            out_byte == Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s0 (mk_usize lane_m)))
              ((j - start) % 8)) /\
        ((j - start) / 8 == 4 * i + 1 ==>
            out_byte == Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s1 (mk_usize lane_m)))
              ((j - start) % 8)) /\
        ((j - start) / 8 == 4 * i + 2 ==>
            out_byte == Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s2 (mk_usize lane_m)))
              ((j - start) % 8)) /\
        ((j - start) / 8 == 4 * i + 3 ==>
            out_byte == Seq.index
              (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s3 (mk_usize lane_m)))
              ((j - start) % 8)))
    (ensures
        out_byte == Seq.index
          (Core_models.Num.impl_u64__to_le_bytes
             (get_lane_u64 (Seq.index s ((j - start) / 8)) (mk_usize lane_m)))
          ((j - start) % 8))

#push-options "--z3rlimit 200"
let lemma_lane_chain_to_s s s0 s1 s2 s3 start i lane_m j out_byte =
  let b: nat = j - start in
  let q: nat = b / 8 in
  let r: nat = b % 8 in
  // Euclidean: b == q*8 + r, with 0 <= r < 8.
  FStar.Math.Lemmas.lemma_mod_lt b 8;
  FStar.Math.Lemmas.euclidean_division_definition b 8;
  // Combined with 32*i <= b < 32*(i+1), derive 4*i <= q < 4*(i+1).
  assert (4 * i <= q);
  assert (q < 4 * (i + 1));
  // Case split: q in {4i, 4i+1, 4i+2, 4i+3}.  Each case fires exactly
  // one of the 4 hypothesis implications + the corresponding
  // linearisation step.
  if q = 4 * i then
    assert (Seq.index s q == s0)
  else if q = 4 * i + 1 then
    assert (Seq.index s q == s1)
  else if q = 4 * i + 2 then
    assert (Seq.index s q == s2)
  else (
    assert (q == 4 * i + 3);
    assert (Seq.index s q == s3)
  )
#pop-options

/// Layer 4 per-output bridge: lifts the per-(j,lane) bridge over the
/// whole window via `Classical.forall_intro`, taking a single output
/// buffer's post-`store_u64x4x4` state (4-branch `s_k` form, nested
/// if-else matching the wrapper's post exactly) to the unified
/// `s[(j-start)/8]` form.  Verified in isolated context so the
/// outer-loop body proof is a single call per output.
val lemma_lane_chain_to_s_all_j
    (s: Seq.seq t_Vec256)
    (s0 s1 s2 s3: t_Vec256)
    (start: nat)
    (i: nat)
    (lane_m: nat{lane_m < 4})
    (out_m_new: Seq.seq u8)
  : Lemma
    (requires
      Seq.length s >= 4 * i + 4 /\
      s0 == Seq.index s (4 * i + 0) /\
      s1 == Seq.index s (4 * i + 1) /\
      s2 == Seq.index s (4 * i + 2) /\
      s3 == Seq.index s (4 * i + 3) /\
      (forall (j_n: nat).
        (start + 32 * i <= j_n /\
         j_n < start + 32 * (i + 1) /\
         j_n < Seq.length out_m_new) ==>
        (if (j_n - start) / 8 = 4 * i then
           Seq.index out_m_new j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s0 (mk_usize lane_m)))
             ((j_n - start) % 8)
         else if (j_n - start) / 8 = 4 * i + 1 then
           Seq.index out_m_new j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s1 (mk_usize lane_m)))
             ((j_n - start) % 8)
         else if (j_n - start) / 8 = 4 * i + 2 then
           Seq.index out_m_new j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s2 (mk_usize lane_m)))
             ((j_n - start) % 8)
         else
           Seq.index out_m_new j_n == Seq.index
             (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 s3 (mk_usize lane_m)))
             ((j_n - start) % 8))))
    (ensures
      forall (j_n: nat).
        (start + 32 * i <= j_n /\
         j_n < start + 32 * (i + 1) /\
         j_n < Seq.length out_m_new) ==>
        Seq.index out_m_new j_n == Seq.index
          (Core_models.Num.impl_u64__to_le_bytes
            (get_lane_u64 (Seq.index s ((j_n - start) / 8)) (mk_usize lane_m)))
          ((j_n - start) % 8))

#push-options "--z3rlimit 400"
let lemma_lane_chain_to_s_all_j s s0 s1 s2 s3 start i lane_m out_m_new =
  let aux (j_n: nat{j_n < Seq.length out_m_new}) :
    Lemma
      ((start + 32 * i <= j_n /\
        j_n < start + 32 * (i + 1)) ==>
       Seq.index out_m_new j_n == Seq.index
         (Core_models.Num.impl_u64__to_le_bytes
           (get_lane_u64 (Seq.index s ((j_n - start) / 8)) (mk_usize lane_m)))
         ((j_n - start) % 8))
    = if start + 32 * i <= j_n && j_n < start + 32 * (i + 1) then begin
        let b: nat = j_n - start in
        let q: nat = b / 8 in
        FStar.Math.Lemmas.lemma_mod_lt b 8;
        FStar.Math.Lemmas.euclidean_division_definition b 8;
        assert (4 * i <= q /\ q < 4 * (i + 1));
        lemma_lane_chain_to_s s s0 s1 s2 s3 start i lane_m j_n (Seq.index out_m_new j_n)
      end
      else ()
  in
  Classical.forall_intro aux
#pop-options
