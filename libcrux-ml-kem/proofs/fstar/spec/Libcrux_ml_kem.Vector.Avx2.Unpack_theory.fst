module Libcrux_ml_kem.Vector.Avx2.Unpack_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views

module RI = Rust_primitives.Integers

(* ============================================================================
   DESERIALIZE unpack spine — the `set_epi16 (duplicated bytes)` -> `mullo by
   2^k` -> `srli` -> `and mask` shape shared by the AVX2 deserialize widths.

   Structured per the session-7 keystone lesson: the PURE i16/u16 BIT ARITHMETIC
   (`lemma_mul_pow2_bit`, `lemma_srli4_and15_bits`, `lemma_deser4_lane`) is
   proven with no vector terms in scope; the lane view only appears in the final
   16-arm ground dispatch, whose arms do nothing but pin two literal lane values
   and call the arithmetic lemma.
   ========================================================================== *)

(* ── pure bit arithmetic ──────────────────────────────────────────────────── *)

(* Generalisation of the companion's `lemma_mul_pow2_bit15` to an arbitrary bit
   position: multiplying by (2^k mod 2^16) moves bit n-k of x to bit n. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400"
let lemma_mul_pow2_bit (x m: i16) (k: nat) (n: nat{k <= n /\ n <= 15})
  : Lemma (requires (v m) % pow2 16 == pow2 k)
          (ensures RI.get_bit (RI.mul_mod x m) (sz n) == RI.get_bit x (sz (n - k))) =
  let y: i16 = RI.mul_mod x m in
  let n16 = pow2 16 in
  assert_norm (pow2 16 == 65536);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I16);
  let x16 = (v x) % n16 in
  let y16 = (v y) % n16 in
  assert ((v y) % n16 == (v x * v m) % n16);
  FStar.Math.Lemmas.lemma_mod_mul_distr_r (v x) (v m) n16;
  FStar.Math.Lemmas.lemma_mod_mul_distr_l (v x) (pow2 k) n16;
  assert (y16 == (x16 * pow2 k) % n16);
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 (x16 * pow2 k) n 16;
  FStar.Math.Lemmas.pow2_plus k (n - k);
  FStar.Math.Lemmas.division_multiplication_lemma (x16 * pow2 k) (pow2 k) (pow2 (n - k));
  FStar.Math.Lemmas.cancel_mul_div x16 (pow2 k);
  FStar.Math.Lemmas.modulo_modulo_lemma ((x16 * pow2 k) / pow2 n) 2 (pow2 0);
  assert ((y16 / pow2 n) % 2 == (x16 / pow2 (n - k)) % 2)
#pop-options

(* bit c of the low-nibble mask constant. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_bit_of_15 (c: nat{c < 16})
  : Lemma (RI.get_bit (mk_i16 15) (sz c) == (if c < 4 then 1 else 0)) =
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I16);
  assert_norm (pow2 16 == 65536);
  if c < 4 then begin
    assert_norm (pow2 0 == 1); assert_norm (pow2 1 == 2);
    assert_norm (pow2 2 == 4); assert_norm (pow2 3 == 8);
    assert (c == 0 \/ c == 1 \/ c == 2 \/ c == 3)
  end
  else begin
    assert_norm (pow2 4 == 16);
    FStar.Math.Lemmas.pow2_le_compat c 4;
    FStar.Math.Lemmas.small_division_lemma_1 15 (pow2 c)
  end
#pop-options

(* the nibble extract: `(y >>u 4) & 15` keeps bits 4..7 of y in positions 0..3. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_srli4_and15_bits (y: i16) (c: nat{c < 16})
  : Lemma (RI.get_bit ((cast ((cast y <: u16) >>! mk_i32 4 <: u16) <: i16) &. mk_i16 15) (sz c) ==
           (if c < 4 then RI.get_bit y (sz (4 + c)) else 0)) =
  assert_norm (pow2 4 == 16); assert_norm (pow2 16 == 65536);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I16);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.U16);
  let yu: u16 = cast y <: u16 in
  let sh: u16 = yu >>! mk_i32 4 in
  assert (v yu == (v y) % pow2 16);
  assert (v sh == (v yu) / pow2 4);
  let r: i16 = cast sh <: i16 in
  assert (v r == v sh);
  lemma_bit_of_15 c;
  RI.get_bit_and r (mk_i16 15) (sz c);
  if c < 4 then begin
    FStar.Math.Lemmas.pow2_plus 4 c;
    FStar.Math.Lemmas.division_multiplication_lemma (v yu) (pow2 4) (pow2 c);
    FStar.Math.Lemmas.small_mod (v r) (pow2 16)
  end
#pop-options

(* one lane: source byte `x`, multiplier `m == 2^k` with k in {0,4}.  Bit c of
   the unpacked lane is bit (4 - k + c) of the source byte, for c < 4. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_deser4_lane (x m: i16) (k: nat{k == 0 \/ k == 4}) (c: nat{c < 16})
  : Lemma (requires (v m) % pow2 16 == pow2 k)
          (ensures
            RI.get_bit ((cast ((cast (RI.mul_mod x m) <: u16) >>! mk_i32 4 <: u16) <: i16)
                        &. mk_i16 15) (sz c) ==
            (if c < 4 then RI.get_bit x (sz (4 - k + c)) else 0)) =
  lemma_srli4_and15_bits (RI.mul_mod x m) c;
  if c < 4 then lemma_mul_pow2_bit x m k (4 + c)
#pop-options

(* the two multiplier constants, as ground pow2 images. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_deser4_mults ()
  : Lemma ((v ((mk_i16 1 <<! mk_i32 0 <: i16) <: i16)) % pow2 16 == pow2 0 /\
           (v ((mk_i16 1 <<! mk_i32 4 <: i16) <: i16)) % pow2 16 == pow2 4 /\
           ((mk_i16 1 <<! mk_i32 4 <: i16) -! mk_i16 1 <: i16) == mk_i16 15) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 4 == 16); assert_norm (pow2 16 == 65536)
#pop-options

(* ── the per-index deserialize_4 bit obligation ───────────────────────────────
   Mirror of the companion's `lemma_deserialize_1_bits` at width 4.  The lane
   view appears ONLY here; each of the 16 arms pins two literal lane values and
   calls `lemma_deser4_lane`, which carries all of the bit arithmetic. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deserialize_4_bits (b0 b1 b2 b3 b4 b5 b6 b7: i16) (i: nat{i < 256})
  : Lemma
      (let coeff = mm256_set_epi16 b7 b7 b6 b6 b5 b5 b4 b4 b3 b3 b2 b2 b1 b1 b0 b0 in
       let mults = mm256_set_epi16 (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) in
       let r = mm256_and_si256 (mm256_srli_epi16 (mk_i32 4) (mm256_mullo_epi16 coeff mults))
                 (mm256_set1_epi16 ((mk_i16 1 <<! mk_i32 4 <: i16) -! mk_i16 1 <: i16)) in
       bv_bit r i = (if i % 16 < 4
                     then (let j = (i / 16) * 4 + i % 16 in
                           match i / 32 with
                           | 0 -> RI.get_bit b0 (sz j)
                           | 1 -> RI.get_bit b1 (sz (j - 8))
                           | 2 -> RI.get_bit b2 (sz (j - 16))
                           | 3 -> RI.get_bit b3 (sz (j - 24))
                           | 4 -> RI.get_bit b4 (sz (j - 32))
                           | 5 -> RI.get_bit b5 (sz (j - 40))
                           | 6 -> RI.get_bit b6 (sz (j - 48))
                           | 7 -> RI.get_bit b7 (sz (j - 56)))
                     else 0)) =
  let coeff = mm256_set_epi16 b7 b7 b6 b6 b5 b5 b4 b4 b3 b3 b2 b2 b1 b1 b0 b0 in
  let mults = mm256_set_epi16 (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) in
  let msb = mm256_mullo_epi16 coeff mults in
  let lsb = mm256_srli_epi16 (mk_i32 4) msb in
  let mask = mm256_set1_epi16 ((mk_i16 1 <<! mk_i32 4 <: i16) -! mk_i16 1 <: i16) in
  let r = mm256_and_si256 lsb mask in
  let l = i / 16 in
  let bb = i % 16 in
  lemma_deser4_mults ();
  lemma_mm256_set_epi16_lanes b7 b7 b6 b6 b5 b5 b4 b4 b3 b3 b2 b2 b1 b1 b0 b0;
  lemma_mm256_set_epi16_lanes (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16);
  lemma_mm256_set1_epi16 ((mk_i16 1 <<! mk_i32 4 <: i16) -! mk_i16 1 <: i16);
  bit_vec_of_int_t_array_vec256_as_i16x16_lemma r 16 i;
  assert (get_lane msb l == RI.mul_mod (get_lane coeff l) (get_lane mults l));
  assert (get_lane lsb l ==
          (cast ((cast (get_lane msb l) <: u16) >>! mk_i32 4 <: u16) <: i16));
  assert (get_lane mask l == mk_i16 15);
  assert (get_lane r l == (get_lane lsb l &. get_lane mask l));
  (if false then ()
   else if l = 0 then begin
     assert (get_lane coeff 0 == b0);
     assert (get_lane mults 0 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b0 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 1 then begin
     assert (get_lane coeff 1 == b0);
     assert (get_lane mults 1 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b0 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 2 then begin
     assert (get_lane coeff 2 == b1);
     assert (get_lane mults 2 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b1 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 3 then begin
     assert (get_lane coeff 3 == b1);
     assert (get_lane mults 3 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b1 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 4 then begin
     assert (get_lane coeff 4 == b2);
     assert (get_lane mults 4 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b2 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 5 then begin
     assert (get_lane coeff 5 == b2);
     assert (get_lane mults 5 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b2 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 6 then begin
     assert (get_lane coeff 6 == b3);
     assert (get_lane mults 6 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b3 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 7 then begin
     assert (get_lane coeff 7 == b3);
     assert (get_lane mults 7 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b3 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 8 then begin
     assert (get_lane coeff 8 == b4);
     assert (get_lane mults 8 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b4 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 9 then begin
     assert (get_lane coeff 9 == b4);
     assert (get_lane mults 9 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b4 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 10 then begin
     assert (get_lane coeff 10 == b5);
     assert (get_lane mults 10 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b5 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 11 then begin
     assert (get_lane coeff 11 == b5);
     assert (get_lane mults 11 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b5 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 12 then begin
     assert (get_lane coeff 12 == b6);
     assert (get_lane mults 12 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b6 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 13 then begin
     assert (get_lane coeff 13 == b6);
     assert (get_lane mults 13 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b6 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else if l = 14 then begin
     assert (get_lane coeff 14 == b7);
     assert (get_lane mults 14 == (mk_i16 1 <<! mk_i32 4 <: i16));
     lemma_deser4_lane b7 (mk_i16 1 <<! mk_i32 4 <: i16) 4 bb
   end
   else if l = 15 then begin
     assert (get_lane coeff 15 == b7);
     assert (get_lane mults 15 == (mk_i16 1 <<! mk_i32 0 <: i16));
     lemma_deser4_lane b7 (mk_i16 1 <<! mk_i32 0 <: i16) 0 bb
   end
   else ());
  assert (l < 16)
#pop-options

module Canon = Libcrux_core_models.Intrinsics_views
module IVi   = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module IV    = Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
module Funarr = Libcrux_core_models.Abstractions.Funarr

(* ── `mm256_si256_from_two_si128` — the 128+128 -> 256 concatenation ─────────
   `mm256_castsi128_si256` zero-extends into the low half and
   `mm256_inserti128_si256 1` replaces the HIGH 128-bit lane, so the
   composition is a pure concatenation.  Over core-models both ops ARE modelled,
   so the wrapper gets a contract proven from its actual code.

   Trust accounting: `castsi128_si256` rests on the tested lift axiom
   `Canon.lemma_castsi128_si256_lift` (same class as xor / setzero — a
   differential-tested raw-op identity); `inserti128_si256` on the i128x2 lane
   view plus `Canon.lemma_readback` at I128, exactly the route
   `lemma_mm_storeu_bytes_si128` takes at U8.  Net: no `fstar::replace` stub; the
   contract rests on one already-present tested identity.

   Developed here per `feedback_develop_locally_upstream_once`; belongs next to
   `lemma_bv_bit_castsi256_si128` / `lemma_bv_bit_extracti128_si256_1` in
   `Avx2_ml_kem_views` once the deserialize widths have exercised it. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_castsi128_si256 (a: t_Vec128) (k: nat{k < 256})
  : Lemma (bv_bit (mm256_castsi128_si256 a) k == (if k < 128 then bv_bit a k else 0)) =
  reveal_opaque (`%mm256_castsi128_si256) mm256_castsi128_si256;
  Canon.lemma_castsi128_si256_lift a
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_bv_bit_inserti128_si256_1 (a: t_Vec256) (b: t_Vec128) (k: nat{k < 256})
  : Lemma (bv_bit (mm256_inserti128_si256 (mk_i32 1) a b) k ==
           (if k < 128 then bv_bit a k else bv_bit b (k - 128))) =
  reveal_opaque (`%mm256_inserti128_si256) mm256_inserti128_si256;
  let r = mm256_inserti128_si256 (mk_i32 1) a b in
  Canon.lemma_mm256_inserti128_si256 (mk_i32 1) a b;
  assert (Canon.to_i128x2 r ==
          IV.e_mm256_inserti128_si256 (mk_i32 1) (Canon.to_i128x2 a) (Canon.to_i128x1 b));
  if k < 128 then begin
    Canon.lemma_readback RI.I128 (mk_u64 256) (mk_u64 2) r (mk_u64 0) k;
    Canon.lemma_readback RI.I128 (mk_u64 256) (mk_u64 2) a (mk_u64 0) k;
    lemma_bv_bit_reader #(mk_u64 256) 128 r 0 k;
    lemma_bv_bit_reader #(mk_u64 256) 128 a 0 k;
    assert (Funarr.impl_5__get (mk_u64 2) #i128 (Canon.to_i128x2 r) (mk_u64 0) ==
            Funarr.impl_5__get (mk_u64 2) #i128 (Canon.to_i128x2 a) (mk_u64 0))
  end
  else begin
    Canon.lemma_readback RI.I128 (mk_u64 256) (mk_u64 2) r (mk_u64 1) (k - 128);
    Canon.lemma_readback RI.I128 (mk_u64 128) (mk_u64 1) b (mk_u64 0) (k - 128);
    lemma_bv_bit_reader #(mk_u64 256) 128 r 1 (k - 128);
    lemma_bv_bit_reader #(mk_u64 128) 128 b 0 (k - 128);
    assert (Funarr.impl_5__get (mk_u64 2) #i128 (Canon.to_i128x2 r) (mk_u64 1) ==
            Funarr.impl_5__get (mk_u64 1) #i128 (Canon.to_i128x1 b) (mk_u64 0))
  end
#pop-options

(* the wrapper's contract, in the form its consumers use *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_si256_from_two_si128 (lo hi: t_Vec128) (k: nat{k < 256})
  : Lemma (bv_bit (mm256_inserti128_si256 (mk_i32 1) (mm256_castsi128_si256 lo) hi) k ==
           (if k < 128 then bv_bit lo k else bv_bit hi (k - 128))) =
  lemma_bv_bit_inserti128_si256_1 (mm256_castsi128_si256 lo) hi k;
  if k < 128 then lemma_bv_bit_castsi128_si256 lo k
#pop-options

(* ── the width-generic unpack arithmetic, shared by deserialize_10 / _12 ────
   `lemma_srli4_and15_bits` / `lemma_deser4_lane` above are the width-4
   instances of exactly this shape; these generalise the shift amount and the
   mask width so widths 10 (srli 6, mask 1023) and 12 (srli 4, mask 4095) need
   no new bit arithmetic at all.  Pure i16/u16 — no vector terms in scope, per
   `feedback_split_simd_lemma_three_contexts`. *)

(* bit c of a low-w mask constant 2^w - 1. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_bit_of_low_mask (mask: i16) (w: nat{1 <= w /\ w <= 15}) (c: nat{c < 16})
  : Lemma (requires v mask == pow2 w - 1)
          (ensures RI.get_bit mask (sz c) == (if c < w then 1 else 0)) =
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I16);
  assert_norm (pow2 16 == 65536);
  FStar.Math.Lemmas.pow2_le_compat 15 w;
  FStar.Math.Lemmas.small_mod (v mask) (pow2 16);
  if c < w then begin
    (* 2^w - 1 == 2^c * (2^(w-c) - 1) + (2^c - 1), and 2^(w-c) - 1 is odd *)
    FStar.Math.Lemmas.pow2_plus c (w - c);
    FStar.Math.Lemmas.small_division_lemma_1 (pow2 c - 1) (pow2 c);
    FStar.Math.Lemmas.lemma_div_plus (pow2 c - 1) (pow2 (w - c) - 1) (pow2 c);
    FStar.Math.Lemmas.pow2_plus 1 (w - c - 1)
  end
  else begin
    FStar.Math.Lemmas.pow2_le_compat c w;
    FStar.Math.Lemmas.small_division_lemma_1 (pow2 w - 1) (pow2 c)
  end
#pop-options

(* the width-w extract: `(y >>u sh) & (2^w - 1)` keeps bits sh..sh+w-1 of y in
   positions 0..w-1.  (Width-4 instance: `lemma_srli4_and15_bits`.) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_srli_and_mask_bits (y mask: i16) (sh: nat{1 <= sh /\ sh <= 15})
      (w: nat{1 <= w /\ w <= 15}) (c: nat{c < 16})
  : Lemma (requires v mask == pow2 w - 1 /\ sh + w <= 16)
          (ensures
            RI.get_bit ((cast ((cast y <: u16) >>! mk_i32 sh <: u16) <: i16) &. mask) (sz c) ==
            (if c < w then RI.get_bit y (sz (sh + c)) else 0)) =
  assert_norm (pow2 16 == 65536);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I16);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.U16);
  let yu: u16 = cast y <: u16 in
  let s: u16 = yu >>! mk_i32 sh in
  assert (v yu == (v y) % pow2 16);
  assert (v s == (v yu) / pow2 sh);
  FStar.Math.Lemmas.pow2_le_compat sh 1;
  FStar.Math.Lemmas.lemma_div_lt_nat (v yu) 16 sh;
  let r: i16 = cast s <: i16 in
  assert (v r == v s);
  lemma_bit_of_low_mask mask w c;
  RI.get_bit_and r mask (sz c);
  if c < w then begin
    FStar.Math.Lemmas.pow2_plus sh c;
    FStar.Math.Lemmas.division_multiplication_lemma (v yu) (pow2 sh) (pow2 c);
    FStar.Math.Lemmas.small_mod (v r) (pow2 16)
  end
#pop-options

(* one lane of the deserialize unpack: source lane `x`, multiplier `m == 2^k`,
   right shift `sh`, mask `2^w - 1`.  Bit c of the unpacked lane is bit
   (sh - k + c) of the source lane, for c < w, and 0 above. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_deser_lane (x m mask: i16) (k: nat) (sh: nat{1 <= sh /\ sh <= 15})
      (w: nat{1 <= w /\ w <= 15}) (c: nat{c < 16})
  : Lemma (requires (v m) % pow2 16 == pow2 k /\ v mask == pow2 w - 1 /\
                    k <= sh /\ sh + w <= 16)
          (ensures
            RI.get_bit ((cast ((cast (RI.mul_mod x m) <: u16) >>! mk_i32 sh <: u16) <: i16) &. mask)
                       (sz c) ==
            (if c < w then RI.get_bit x (sz (sh - k + c)) else 0)) =
  lemma_srli_and_mask_bits (RI.mul_mod x m) mask sh w c;
  if c < w then lemma_mul_pow2_bit x m k (sh + c)
#pop-options

(* ── VPSHUFB, select branch, 128-bit, in bv_bit form ───────────────────────
   The 128-bit twin of `Byteperm_theory.lemma_bv_bit_mm256_shuffle_epi8_sel`;
   deserialize_10 / _12 gather their bytes with two 128-bit PSHUFBs before the
   concatenation, and only the 256-bit sel form existed.  The `16 * (nth / 16)`
   lane term of the 256-bit version drops out — a 128-bit shuffle has a single
   lane. *)
let vec128_byte (bv: t_Vec128) (k: nat{k < 16}) : i8 =
  Funarr.impl_5__get (mk_u64 16) #i8 (Canon.to_i8x16 bv) (mk_u64 k)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm_shuffle_epi8_sel (a b: t_Vec128) (i: nat{i < 128}) (sel: nat{sel < 16})
  : Lemma (requires v (vec128_byte b (i / 8)) >= 0 /\
                    sel == (v (vec128_byte b (i / 8))) % 16)
          (ensures bv_bit (mm_shuffle_epi8 a b) i == bv_bit a (8 * sel + i % 8)) =
  reveal_opaque (`%mm_shuffle_epi8) mm_shuffle_epi8;
  Canon.lemma_mm_shuffle_epi8 a b;
  let nth = i / 8 in
  let sb = i % 8 in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  let r = mm_shuffle_epi8 a b in
  Canon.lemma_iv_mm_shuffle_epi8_sel (Canon.to_i8x16 a) (Canon.to_i8x16 b) nth;
  Canon.lemma_readback RI.I8 (mk_u64 128) (mk_u64 16) r (mk_u64 nth) sb;
  lemma_bv_bit_reader 8 r nth sb;
  Canon.lemma_readback RI.I8 (mk_u64 128) (mk_u64 16) a (mk_u64 sel) sb;
  lemma_bv_bit_reader 8 a sel sb
#pop-options

(* ── deserialize_10: the two 128-bit gather shuffles ────────────────────────
   `deserialize_10_vec` gathers with two 128-bit PSHUFBs before concatenating.
   Both masks are the same byte map up to the +6 offset of the high half:

     lo  9,8,8,7,7,6,6,5, 4,3,3,2,2,1,1,0   (byte 15 first, as `mm_set_epi8`)
     hi  15,14,14,13,13,12,12,11, 10,9,9,8,8,7,7,6

   i.e. byte b reads source byte `(b+1)/2` within its 8-byte group, plus one
   group-carry — the 10-bits-per-coefficient stride.  Stating it in that closed
   form (rather than 16 literal arms) is what lets the downstream index algebra
   go through: the gather step below is then a pure INDEX SHIFT, per
   `feedback_split_simd_lemma_three_contexts`. *)
unfold let deser10_bytemap (b: nat{b < 16}) : nat = (b + 1) / 2 + (if b < 8 then 0 else 1)

unfold let deser10_lo_mask =
  mm_set_epi8 (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 7) (mk_i8 6) (mk_i8 6) (mk_i8 5)
              (mk_i8 4) (mk_i8 3) (mk_i8 3) (mk_i8 2) (mk_i8 2) (mk_i8 1) (mk_i8 1) (mk_i8 0)

unfold let deser10_hi_mask =
  mm_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 14) (mk_i8 13) (mk_i8 13) (mk_i8 12) (mk_i8 12)
              (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 7)
              (mk_i8 6)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_deser10_lo_mask_bytes (b: nat{b < 16})
  : Lemma (ensures v (vec128_byte deser10_lo_mask b) == deser10_bytemap b) =
  reveal_opaque (`%mm_set_epi8) mm_set_epi8;
  Canon.lemma_mm_set_epi8 (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 7) (mk_i8 6) (mk_i8 6)
    (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 3) (mk_i8 2) (mk_i8 2) (mk_i8 1) (mk_i8 1) (mk_i8 0);
  Canon.lemma_iv_mm_set_epi8 (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 7) (mk_i8 6) (mk_i8 6)
    (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 3) (mk_i8 2) (mk_i8 2) (mk_i8 1) (mk_i8 1) (mk_i8 0) b
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_deser10_hi_mask_bytes (b: nat{b < 16})
  : Lemma (ensures v (vec128_byte deser10_hi_mask b) == deser10_bytemap b + 6) =
  reveal_opaque (`%mm_set_epi8) mm_set_epi8;
  Canon.lemma_mm_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 14) (mk_i8 13) (mk_i8 13) (mk_i8 12)
    (mk_i8 12) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 7)
    (mk_i8 6);
  Canon.lemma_iv_mm_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 14) (mk_i8 13) (mk_i8 13) (mk_i8 12)
    (mk_i8 12) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 7)
    (mk_i8 6) b
#pop-options

(* the two gathers, each as a pure index shift *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser10_shuffle_lo_bit (a: t_Vec128) (i: nat{i < 128})
  : Lemma (ensures bv_bit (mm_shuffle_epi8 a deser10_lo_mask) i ==
                   bv_bit a (8 * deser10_bytemap (i / 8) + i % 8)) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_deser10_lo_mask_bytes (i / 8);
  FStar.Math.Lemmas.small_mod (deser10_bytemap (i / 8)) 16;
  lemma_bv_bit_mm_shuffle_epi8_sel a deser10_lo_mask i (deser10_bytemap (i / 8))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser10_shuffle_hi_bit (a: t_Vec128) (i: nat{i < 128})
  : Lemma (ensures bv_bit (mm_shuffle_epi8 a deser10_hi_mask) i ==
                   bv_bit a (8 * (deser10_bytemap (i / 8) + 6) + i % 8)) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_deser10_hi_mask_bytes (i / 8);
  FStar.Math.Lemmas.small_mod (deser10_bytemap (i / 8) + 6) 16;
  lemma_bv_bit_mm_shuffle_epi8_sel a deser10_hi_mask i (deser10_bytemap (i / 8) + 6)
#pop-options

(* ── deserialize_10: the unpack multipliers and the low-10 mask ─────────────
   `mm256_set_epi16` lists lane 15 first, so lane l carries 2^(6 - 2*(l%4)).
   After the `srli 6` that makes bit c of the result lane read bit 2*(l%4) + c
   of the shuffled lane — a pure INDEX SHIFT, which is the shape the rest of
   the chain composes with. *)
unfold let deser10_mults =
  mm256_set_epi16 (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
                  (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
                  (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
                  (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
                  (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)

unfold let deser10_mask = mm256_set1_epi16 ((mk_i16 1 <<! mk_i32 10 <: i16) -! mk_i16 1 <: i16)

(* the four multiplier literals + the mask literal, as ground pow2 images.
   Pure integer arithmetic, no vector terms. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_deser10_consts ()
  : Lemma ((v (mk_i16 1 <<! mk_i32 0 <: i16)) % pow2 16 == pow2 0 /\
           (v (mk_i16 1 <<! mk_i32 2 <: i16)) % pow2 16 == pow2 2 /\
           (v (mk_i16 1 <<! mk_i32 4 <: i16)) % pow2 16 == pow2 4 /\
           (v (mk_i16 1 <<! mk_i32 6 <: i16)) % pow2 16 == pow2 6 /\
           v ((mk_i16 1 <<! mk_i32 10 <: i16) -! mk_i16 1 <: i16) == pow2 10 - 1) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 2 == 4); assert_norm (pow2 4 == 16);
  assert_norm (pow2 6 == 64); assert_norm (pow2 10 == 1024); assert_norm (pow2 16 == 65536)
#pop-options

(* the ground 16-arm lane dispatch, in its own context: only the constant
   vector is in scope. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_deser10_mult_lane (l: nat{l < 16})
  : Lemma (ensures (v (get_lane deser10_mults l)) % pow2 16 == pow2 (6 - 2 * (l % 4))) =
  lemma_deser10_consts ();
  lemma_mm256_set_epi16_lanes
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
    (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
    (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
    (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 2 <: i16)
    (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 6 <: i16);
  (if l = 0       then assert (get_lane deser10_mults 0  == (mk_i16 1 <<! mk_i32 6 <: i16))
   else if l = 1  then assert (get_lane deser10_mults 1  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 2  then assert (get_lane deser10_mults 2  == (mk_i16 1 <<! mk_i32 2 <: i16))
   else if l = 3  then assert (get_lane deser10_mults 3  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 4  then assert (get_lane deser10_mults 4  == (mk_i16 1 <<! mk_i32 6 <: i16))
   else if l = 5  then assert (get_lane deser10_mults 5  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 6  then assert (get_lane deser10_mults 6  == (mk_i16 1 <<! mk_i32 2 <: i16))
   else if l = 7  then assert (get_lane deser10_mults 7  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 8  then assert (get_lane deser10_mults 8  == (mk_i16 1 <<! mk_i32 6 <: i16))
   else if l = 9  then assert (get_lane deser10_mults 9  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 10 then assert (get_lane deser10_mults 10 == (mk_i16 1 <<! mk_i32 2 <: i16))
   else if l = 11 then assert (get_lane deser10_mults 11 == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 12 then assert (get_lane deser10_mults 12 == (mk_i16 1 <<! mk_i32 6 <: i16))
   else if l = 13 then assert (get_lane deser10_mults 13 == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 14 then assert (get_lane deser10_mults 14 == (mk_i16 1 <<! mk_i32 2 <: i16))
   else assert (get_lane deser10_mults 15 == (mk_i16 1 <<! mk_i32 0 <: i16)))
#pop-options

(* ── the unpack spine, as a pure index shift on the (still opaque) input ──── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser10_unpack_bit (co: t_Vec256) (i: nat{i < 256})
  : Lemma (ensures
      (let r = mm256_and_si256
                 (mm256_srli_epi16 (mk_i32 6) (mm256_mullo_epi16 co deser10_mults)) deser10_mask in
       bv_bit r i == (if i % 16 >= 10 then 0
                      else bv_bit co ((i / 16) * 16 + 2 * ((i / 16) % 4) + i % 16)))) =
  let msb = mm256_mullo_epi16 co deser10_mults in
  let lsb = mm256_srli_epi16 (mk_i32 6) msb in
  let r = mm256_and_si256 lsb deser10_mask in
  let l = i / 16 in
  let b = i % 16 in
  lemma_deser10_consts ();
  lemma_deser10_mult_lane l;
  bit_vec_of_int_t_array_vec256_as_i16x16_lemma r 16 i;
  assert (get_lane msb l == RI.mul_mod (get_lane co l) (get_lane deser10_mults l));
  assert (get_lane lsb l == (cast ((cast (get_lane msb l) <: u16) >>! mk_i32 6 <: u16) <: i16));
  assert (get_lane deser10_mask l == ((mk_i16 1 <<! mk_i32 10 <: i16) -! mk_i16 1 <: i16));
  assert (get_lane r l == (get_lane lsb l &. get_lane deser10_mask l));
  lemma_deser_lane (get_lane co l) (get_lane deser10_mults l)
                   ((mk_i16 1 <<! mk_i32 10 <: i16) -! mk_i16 1 <: i16)
                   (6 - 2 * (l % 4)) 6 10 b;
  if b < 10 then bit_vec_of_int_t_array_vec256_as_i16x16_lemma co 16 (16 * l + 2 * (l % 4) + b)
#pop-options

(* ── the index identity that closes the gather, pure integer arithmetic ─────
   Result lane l bit b reads shuffled flat bit d = 16*l + 2*(l%4) + b, and the
   gather byte map sends that back to source bit 10*l + b.  Eight ground arms
   on l (q = l/4 in {0,1} x s = l%4 in {0..3}), each with the ONE b-split at
   the point where (18*(l%4) + b) crosses a multiple of 8. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser10_index (l: nat{l < 8}) (b: nat{b < 10})
  : Lemma (ensures (let d = 16 * l + 2 * (l % 4) + b in
                    8 * deser10_bytemap (d / 8) + d % 8 == 10 * l + b)) =
  let d = 16 * l + 2 * (l % 4) + b in
  FStar.Math.Lemmas.euclidean_division_definition d 8;
  (if l = 0      then (if b < 8 then assert (d / 8 == 0)  else assert (d / 8 == 1))
   else if l = 1 then (if b < 6 then assert (d / 8 == 2)  else assert (d / 8 == 3))
   else if l = 2 then (if b < 4 then assert (d / 8 == 4)  else assert (d / 8 == 5))
   else if l = 3 then (if b < 2 then assert (d / 8 == 6)  else assert (d / 8 == 7))
   else if l = 4 then (if b < 8 then assert (d / 8 == 8)  else assert (d / 8 == 9))
   else if l = 5 then (if b < 6 then assert (d / 8 == 10) else assert (d / 8 == 11))
   else if l = 6 then (if b < 4 then assert (d / 8 == 12) else assert (d / 8 == 13))
   else               (if b < 2 then assert (d / 8 == 14) else assert (d / 8 == 15)))
#pop-options

(* ── the gather: both halves, via the two 128-bit shuffles + the concat ───── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser10_gather_bit (lo0 up0: t_Vec128) (co: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             (forall (k: nat{k < 256}).
                bv_bit co k ==
                (if k < 128 then bv_bit (mm_shuffle_epi8 lo0 deser10_lo_mask) k
                 else bv_bit (mm_shuffle_epi8 up0 deser10_hi_mask) (k - 128))) /\
             i % 16 < 10)
          (ensures
             bv_bit co ((i / 16) * 16 + 2 * ((i / 16) % 4) + i % 16) ==
             (let j = (i / 16) * 10 + i % 16 in
              if i < 128 then bv_bit lo0 j else bv_bit up0 (j - 32))) =
  let l = i / 16 in
  let b = i % 16 in
  if i < 128 then begin
    assert (l < 8);
    let d = 16 * l + 2 * (l % 4) + b in
    assert (d < 128);
    lemma_deser10_index l b;
    lemma_deser10_shuffle_lo_bit lo0 d
  end
  else begin
    assert (l >= 8 /\ l < 16);
    let l' = l - 8 in
    FStar.Math.Lemmas.lemma_mod_plus l' 2 4;
    assert (l % 4 == l' % 4);
    let d' = 16 * l' + 2 * (l' % 4) + b in
    assert (16 * l + 2 * (l % 4) + b == d' + 128);
    assert (d' < 128);
    lemma_deser10_index l' b;
    lemma_deser10_shuffle_hi_bit up0 d'
  end
#pop-options

(* ── the whole `deserialize_10_vec` obligation, one index at a time ─────────
   `co` is the 128+128 concatenation; it enters as a FREE parameter carrying
   exactly the post `mm256_si256_from_two_si128` supplies, so the wrapper (a
   Serialize-module function, invisible here) never has to be named. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deserialize_10_bits (lo0 up0: t_Vec128) (co: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             forall (k: nat{k < 256}).
               bv_bit co k ==
               (if k < 128 then bv_bit (mm_shuffle_epi8 lo0 deser10_lo_mask) k
                else bv_bit (mm_shuffle_epi8 up0 deser10_hi_mask) (k - 128)))
          (ensures
             (let r = mm256_and_si256
                        (mm256_srli_epi16 (mk_i32 6) (mm256_mullo_epi16 co deser10_mults))
                        deser10_mask in
              bv_bit r i == (if i % 16 >= 10 then 0
                             else let j = (i / 16) * 10 + i % 16 in
                                  if i < 128 then bv_bit lo0 j else bv_bit up0 (j - 32)))) =
  lemma_deser10_unpack_bit co i;
  if i % 16 < 10 then lemma_deser10_gather_bit lo0 up0 co i
#pop-options

(* ── deserialize_12: the two 128-bit gather shuffles ────────────────────────
   Same shape as deserialize_10, one width up.  Both masks are the same byte
   map up to the +4 offset of the high half:

     lo  11,10,10,9, 8,7,7,6, 5,4,4,3, 2,1,1,0   (byte 15 first, as `mm_set_epi8`)
     hi  15,14,14,13, 12,11,11,10, 9,8,8,7, 6,5,5,4

   i.e. within each group of FOUR bytes the map is (base, base+1, base+1,
   base+2) with base = 3 * (b / 4) — the 12-bits-per-coefficient stride, three
   source bytes per four gathered bytes. *)
unfold let deser12_bytemap (b: nat{b < 16}) : nat = 3 * (b / 4) + (b % 4 + 1) / 2

unfold let deser12_lo_mask =
  mm_set_epi8 (mk_i8 11) (mk_i8 10) (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 7) (mk_i8 7) (mk_i8 6)
              (mk_i8 5) (mk_i8 4) (mk_i8 4) (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 1) (mk_i8 0)

unfold let deser12_hi_mask =
  mm_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 14) (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 11)
              (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 5)
              (mk_i8 4)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_deser12_lo_mask_bytes (b: nat{b < 16})
  : Lemma (ensures v (vec128_byte deser12_lo_mask b) == deser12_bytemap b) =
  reveal_opaque (`%mm_set_epi8) mm_set_epi8;
  Canon.lemma_mm_set_epi8 (mk_i8 11) (mk_i8 10) (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 7) (mk_i8 7)
    (mk_i8 6) (mk_i8 5) (mk_i8 4) (mk_i8 4) (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 1) (mk_i8 0);
  Canon.lemma_iv_mm_set_epi8 (mk_i8 11) (mk_i8 10) (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 7)
    (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 4) (mk_i8 4) (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 1)
    (mk_i8 0) b
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_deser12_hi_mask_bytes (b: nat{b < 16})
  : Lemma (ensures v (vec128_byte deser12_hi_mask b) == deser12_bytemap b + 4) =
  reveal_opaque (`%mm_set_epi8) mm_set_epi8;
  Canon.lemma_mm_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 14) (mk_i8 13) (mk_i8 12) (mk_i8 11)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 5)
    (mk_i8 4);
  Canon.lemma_iv_mm_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 14) (mk_i8 13) (mk_i8 12) (mk_i8 11)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 8) (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 5)
    (mk_i8 4) b
#pop-options

(* the two gathers, each as a pure index shift *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser12_shuffle_lo_bit (a: t_Vec128) (i: nat{i < 128})
  : Lemma (ensures bv_bit (mm_shuffle_epi8 a deser12_lo_mask) i ==
                   bv_bit a (8 * deser12_bytemap (i / 8) + i % 8)) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_deser12_lo_mask_bytes (i / 8);
  FStar.Math.Lemmas.small_mod (deser12_bytemap (i / 8)) 16;
  lemma_bv_bit_mm_shuffle_epi8_sel a deser12_lo_mask i (deser12_bytemap (i / 8))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser12_shuffle_hi_bit (a: t_Vec128) (i: nat{i < 128})
  : Lemma (ensures bv_bit (mm_shuffle_epi8 a deser12_hi_mask) i ==
                   bv_bit a (8 * (deser12_bytemap (i / 8) + 4) + i % 8)) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_deser12_hi_mask_bytes (i / 8);
  FStar.Math.Lemmas.small_mod (deser12_bytemap (i / 8) + 4) 16;
  lemma_bv_bit_mm_shuffle_epi8_sel a deser12_hi_mask i (deser12_bytemap (i / 8) + 4)
#pop-options

(* ── deserialize_12: the unpack multipliers and the low-12 mask ─────────────
   Lane l carries 2^(4 - 4*(l%2)); after the `srli 4` bit c of the result lane
   reads bit 4*(l%2) + c of the shuffled lane. *)
unfold let deser12_mults =
  mm256_set_epi16 (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)

unfold let deser12_mask = mm256_set1_epi16 ((mk_i16 1 <<! mk_i32 12 <: i16) -! mk_i16 1 <: i16)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_deser12_consts ()
  : Lemma ((v (mk_i16 1 <<! mk_i32 0 <: i16)) % pow2 16 == pow2 0 /\
           (v (mk_i16 1 <<! mk_i32 4 <: i16)) % pow2 16 == pow2 4 /\
           v ((mk_i16 1 <<! mk_i32 12 <: i16) -! mk_i16 1 <: i16) == pow2 12 - 1) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 4 == 16);
  assert_norm (pow2 12 == 4096); assert_norm (pow2 16 == 65536)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_deser12_mult_lane (l: nat{l < 16})
  : Lemma (ensures (v (get_lane deser12_mults l)) % pow2 16 == pow2 (4 - 4 * (l % 2))) =
  lemma_deser12_consts ();
  lemma_mm256_set_epi16_lanes
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 4 <: i16);
  (if l = 0       then assert (get_lane deser12_mults 0  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 1  then assert (get_lane deser12_mults 1  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 2  then assert (get_lane deser12_mults 2  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 3  then assert (get_lane deser12_mults 3  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 4  then assert (get_lane deser12_mults 4  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 5  then assert (get_lane deser12_mults 5  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 6  then assert (get_lane deser12_mults 6  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 7  then assert (get_lane deser12_mults 7  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 8  then assert (get_lane deser12_mults 8  == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 9  then assert (get_lane deser12_mults 9  == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 10 then assert (get_lane deser12_mults 10 == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 11 then assert (get_lane deser12_mults 11 == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 12 then assert (get_lane deser12_mults 12 == (mk_i16 1 <<! mk_i32 4 <: i16))
   else if l = 13 then assert (get_lane deser12_mults 13 == (mk_i16 1 <<! mk_i32 0 <: i16))
   else if l = 14 then assert (get_lane deser12_mults 14 == (mk_i16 1 <<! mk_i32 4 <: i16))
   else assert (get_lane deser12_mults 15 == (mk_i16 1 <<! mk_i32 0 <: i16)))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser12_unpack_bit (co: t_Vec256) (i: nat{i < 256})
  : Lemma (ensures
      (let r = mm256_and_si256
                 (mm256_srli_epi16 (mk_i32 4) (mm256_mullo_epi16 co deser12_mults)) deser12_mask in
       bv_bit r i == (if i % 16 >= 12 then 0
                      else bv_bit co ((i / 16) * 16 + 4 * ((i / 16) % 2) + i % 16)))) =
  let msb = mm256_mullo_epi16 co deser12_mults in
  let lsb = mm256_srli_epi16 (mk_i32 4) msb in
  let r = mm256_and_si256 lsb deser12_mask in
  let l = i / 16 in
  let b = i % 16 in
  lemma_deser12_consts ();
  lemma_deser12_mult_lane l;
  bit_vec_of_int_t_array_vec256_as_i16x16_lemma r 16 i;
  assert (get_lane msb l == RI.mul_mod (get_lane co l) (get_lane deser12_mults l));
  assert (get_lane lsb l == (cast ((cast (get_lane msb l) <: u16) >>! mk_i32 4 <: u16) <: i16));
  assert (get_lane deser12_mask l == ((mk_i16 1 <<! mk_i32 12 <: i16) -! mk_i16 1 <: i16));
  assert (get_lane r l == (get_lane lsb l &. get_lane deser12_mask l));
  lemma_deser_lane (get_lane co l) (get_lane deser12_mults l)
                   ((mk_i16 1 <<! mk_i32 12 <: i16) -! mk_i16 1 <: i16)
                   (4 - 4 * (l % 2)) 4 12 b;
  if b < 12 then bit_vec_of_int_t_array_vec256_as_i16x16_lemma co 16 (16 * l + 4 * (l % 2) + b)
#pop-options

(* the index identity: eight ground arms on l, each with the ONE b-split at the
   point where (4*(l%2) + b) crosses a multiple of 8. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser12_index (l: nat{l < 8}) (b: nat{b < 12})
  : Lemma (ensures (let d = 16 * l + 4 * (l % 2) + b in
                    8 * deser12_bytemap (d / 8) + d % 8 == 12 * l + b)) =
  let d = 16 * l + 4 * (l % 2) + b in
  FStar.Math.Lemmas.euclidean_division_definition d 8;
  (if l = 0      then (if b < 8 then assert (d / 8 == 0)  else assert (d / 8 == 1))
   else if l = 1 then (if b < 4 then assert (d / 8 == 2)  else assert (d / 8 == 3))
   else if l = 2 then (if b < 8 then assert (d / 8 == 4)  else assert (d / 8 == 5))
   else if l = 3 then (if b < 4 then assert (d / 8 == 6)  else assert (d / 8 == 7))
   else if l = 4 then (if b < 8 then assert (d / 8 == 8)  else assert (d / 8 == 9))
   else if l = 5 then (if b < 4 then assert (d / 8 == 10) else assert (d / 8 == 11))
   else if l = 6 then (if b < 8 then assert (d / 8 == 12) else assert (d / 8 == 13))
   else               (if b < 4 then assert (d / 8 == 14) else assert (d / 8 == 15)))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser12_gather_bit (lo0 up0: t_Vec128) (co: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             (forall (k: nat{k < 256}).
                bv_bit co k ==
                (if k < 128 then bv_bit (mm_shuffle_epi8 lo0 deser12_lo_mask) k
                 else bv_bit (mm_shuffle_epi8 up0 deser12_hi_mask) (k - 128))) /\
             i % 16 < 12)
          (ensures
             bv_bit co ((i / 16) * 16 + 4 * ((i / 16) % 2) + i % 16) ==
             (let j = (i / 16) * 12 + i % 16 in
              if i < 128 then bv_bit lo0 j else bv_bit up0 (j - 64))) =
  let l = i / 16 in
  let b = i % 16 in
  if i < 128 then begin
    assert (l < 8);
    let d = 16 * l + 4 * (l % 2) + b in
    assert (d < 128);
    lemma_deser12_index l b;
    lemma_deser12_shuffle_lo_bit lo0 d
  end
  else begin
    assert (l >= 8 /\ l < 16);
    let l' = l - 8 in
    FStar.Math.Lemmas.lemma_mod_plus l' 4 2;
    assert (l % 2 == l' % 2);
    let d' = 16 * l' + 4 * (l' % 2) + b in
    assert (16 * l + 4 * (l % 2) + b == d' + 128);
    assert (d' < 128);
    lemma_deser12_index l' b;
    lemma_deser12_shuffle_hi_bit up0 d'
  end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deserialize_12_bits (lo0 up0: t_Vec128) (co: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             forall (k: nat{k < 256}).
               bv_bit co k ==
               (if k < 128 then bv_bit (mm_shuffle_epi8 lo0 deser12_lo_mask) k
                else bv_bit (mm_shuffle_epi8 up0 deser12_hi_mask) (k - 128)))
          (ensures
             (let r = mm256_and_si256
                        (mm256_srli_epi16 (mk_i32 4) (mm256_mullo_epi16 co deser12_mults))
                        deser12_mask in
              bv_bit r i == (if i % 16 >= 12 then 0
                             else let j = (i / 16) * 12 + i % 16 in
                                  if i < 128 then bv_bit lo0 j else bv_bit up0 (j - 64)))) =
  lemma_deser12_unpack_bit co i;
  if i % 16 < 12 then lemma_deser12_gather_bit lo0 up0 co i
#pop-options

module BP = Libcrux_ml_kem.Vector.Avx2.Byteperm_theory

(* ── deserialize_5: the srli-ONLY unpack ─────────────────────────────────────
   Width 5 is the one width with NO and-mask: a logical shift right by 11
   already leaves exactly the 5 live bits in positions 0..4 and zeroes above,
   so `lemma_srli_and_mask_bits` does not apply.  This is its mask-free twin;
   `lemma_deser5_lane` then composes it with `lemma_mul_pow2_bit` exactly the
   way `lemma_deser_lane` does at widths 4 / 10 / 12. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_srli_bits (y: i16) (sh: nat{1 <= sh /\ sh <= 15}) (c: nat{c < 16})
  : Lemma (ensures
            RI.get_bit ((cast ((cast y <: u16) >>! mk_i32 sh <: u16) <: i16)) (sz c) ==
            (if c < 16 - sh then RI.get_bit y (sz (sh + c)) else 0)) =
  assert_norm (pow2 16 == 65536);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I16);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.U16);
  let yu: u16 = cast y <: u16 in
  let s: u16 = yu >>! mk_i32 sh in
  assert (v yu == (v y) % pow2 16);
  assert (v s == (v yu) / pow2 sh);
  FStar.Math.Lemmas.lemma_div_lt_nat (v yu) 16 sh;
  let r: i16 = cast s <: i16 in
  assert (v r == v s);
  FStar.Math.Lemmas.small_mod (v r) (pow2 16);
  if c < 16 - sh then begin
    FStar.Math.Lemmas.pow2_plus sh c;
    FStar.Math.Lemmas.division_multiplication_lemma (v yu) (pow2 sh) (pow2 c)
  end
  else begin
    FStar.Math.Lemmas.pow2_le_compat c (16 - sh);
    FStar.Math.Lemmas.small_division_lemma_1 (v r) (pow2 c)
  end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_deser5_lane (x m: i16) (k: nat) (sh: nat{1 <= sh /\ sh <= 15}) (c: nat{c < 16})
  : Lemma (requires (v m) % pow2 16 == pow2 k /\ k <= sh)
          (ensures
            RI.get_bit ((cast ((cast (RI.mul_mod x m) <: u16) >>! mk_i32 sh <: u16) <: i16))
                       (sz c) ==
            (if c < 16 - sh then RI.get_bit x (sz (sh - k + c)) else 0)) =
  lemma_srli_bits (RI.mul_mod x m) sh c;
  if c < 16 - sh then lemma_mul_pow2_bit x m k (sh + c)
#pop-options

(* the multipliers.  Lane l carries 2^(11 - shift_inv l) with
   shift_inv l = 5*(l%2) + 2*((l%8)/2) — the same expression the function's own
   `ensures` names, so after the `srli 11` bit c of the result lane reads bit
   shift_inv l + c of the shuffled lane and no algebra survives into the post. *)
unfold let deser5_mults =
  mm256_set_epi16 (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 5 <: i16)
                  (mk_i16 1 <<! mk_i32 2 <: i16) (mk_i16 1 <<! mk_i32 7 <: i16)
                  (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 9 <: i16)
                  (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1 <<! mk_i32 11 <: i16)
                  (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 5 <: i16)
                  (mk_i16 1 <<! mk_i32 2 <: i16) (mk_i16 1 <<! mk_i32 7 <: i16)
                  (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 9 <: i16)
                  (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1 <<! mk_i32 11 <: i16)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_deser5_consts ()
  : Lemma ((v (mk_i16 1 <<! mk_i32 0 <: i16)) % pow2 16 == pow2 0 /\
           (v (mk_i16 1 <<! mk_i32 2 <: i16)) % pow2 16 == pow2 2 /\
           (v (mk_i16 1 <<! mk_i32 4 <: i16)) % pow2 16 == pow2 4 /\
           (v (mk_i16 1 <<! mk_i32 5 <: i16)) % pow2 16 == pow2 5 /\
           (v (mk_i16 1 <<! mk_i32 6 <: i16)) % pow2 16 == pow2 6 /\
           (v (mk_i16 1 <<! mk_i32 7 <: i16)) % pow2 16 == pow2 7 /\
           (v (mk_i16 1 <<! mk_i32 9 <: i16)) % pow2 16 == pow2 9 /\
           (v (mk_i16 1 <<! mk_i32 11 <: i16)) % pow2 16 == pow2 11) =
  assert_norm (pow2 0 == 1); assert_norm (pow2 2 == 4); assert_norm (pow2 4 == 16);
  assert_norm (pow2 5 == 32); assert_norm (pow2 6 == 64); assert_norm (pow2 7 == 128);
  assert_norm (pow2 9 == 512); assert_norm (pow2 11 == 2048); assert_norm (pow2 16 == 65536)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_deser5_mult_lane (l: nat{l < 16})
  : Lemma (ensures (v (get_lane deser5_mults l)) % pow2 16 ==
                   pow2 (11 - (5 * (l % 2) + 2 * ((l % 8) / 2)))) =
  lemma_deser5_consts ();
  lemma_mm256_set_epi16_lanes
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 5 <: i16)
    (mk_i16 1 <<! mk_i32 2 <: i16) (mk_i16 1 <<! mk_i32 7 <: i16)
    (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 9 <: i16)
    (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1 <<! mk_i32 11 <: i16)
    (mk_i16 1 <<! mk_i32 0 <: i16) (mk_i16 1 <<! mk_i32 5 <: i16)
    (mk_i16 1 <<! mk_i32 2 <: i16) (mk_i16 1 <<! mk_i32 7 <: i16)
    (mk_i16 1 <<! mk_i32 4 <: i16) (mk_i16 1 <<! mk_i32 9 <: i16)
    (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1 <<! mk_i32 11 <: i16);
  (if l = 0       then assert (get_lane deser5_mults 0  == (mk_i16 1 <<! mk_i32 11 <: i16))
   else if l = 1  then assert (get_lane deser5_mults 1  == (mk_i16 1 <<! mk_i32 6  <: i16))
   else if l = 2  then assert (get_lane deser5_mults 2  == (mk_i16 1 <<! mk_i32 9  <: i16))
   else if l = 3  then assert (get_lane deser5_mults 3  == (mk_i16 1 <<! mk_i32 4  <: i16))
   else if l = 4  then assert (get_lane deser5_mults 4  == (mk_i16 1 <<! mk_i32 7  <: i16))
   else if l = 5  then assert (get_lane deser5_mults 5  == (mk_i16 1 <<! mk_i32 2  <: i16))
   else if l = 6  then assert (get_lane deser5_mults 6  == (mk_i16 1 <<! mk_i32 5  <: i16))
   else if l = 7  then assert (get_lane deser5_mults 7  == (mk_i16 1 <<! mk_i32 0  <: i16))
   else if l = 8  then assert (get_lane deser5_mults 8  == (mk_i16 1 <<! mk_i32 11 <: i16))
   else if l = 9  then assert (get_lane deser5_mults 9  == (mk_i16 1 <<! mk_i32 6  <: i16))
   else if l = 10 then assert (get_lane deser5_mults 10 == (mk_i16 1 <<! mk_i32 9  <: i16))
   else if l = 11 then assert (get_lane deser5_mults 11 == (mk_i16 1 <<! mk_i32 4  <: i16))
   else if l = 12 then assert (get_lane deser5_mults 12 == (mk_i16 1 <<! mk_i32 7  <: i16))
   else if l = 13 then assert (get_lane deser5_mults 13 == (mk_i16 1 <<! mk_i32 2  <: i16))
   else if l = 14 then assert (get_lane deser5_mults 14 == (mk_i16 1 <<! mk_i32 5  <: i16))
   else assert (get_lane deser5_mults 15 == (mk_i16 1 <<! mk_i32 0 <: i16)))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser5_unpack_bit (sh256: t_Vec256) (i: nat{i < 256})
  : Lemma (ensures
      (let r = mm256_srli_epi16 (mk_i32 11) (mm256_mullo_epi16 sh256 deser5_mults) in
       bv_bit r i ==
       (if i % 16 >= 5 then 0
        else bv_bit sh256 ((i / 16) * 16 + (5 * ((i / 16) % 2) + 2 * (((i / 16) % 8) / 2))
                           + i % 16)))) =
  let msb = mm256_mullo_epi16 sh256 deser5_mults in
  let r = mm256_srli_epi16 (mk_i32 11) msb in
  let l = i / 16 in
  let b = i % 16 in
  let si = 5 * (l % 2) + 2 * ((l % 8) / 2) in
  lemma_deser5_consts ();
  lemma_deser5_mult_lane l;
  bit_vec_of_int_t_array_vec256_as_i16x16_lemma r 16 i;
  assert (get_lane msb l == RI.mul_mod (get_lane sh256 l) (get_lane deser5_mults l));
  assert (get_lane r l == (cast ((cast (get_lane msb l) <: u16) >>! mk_i32 11 <: u16) <: i16));
  lemma_deser5_lane (get_lane sh256 l) (get_lane deser5_mults l) (11 - si) 11 b;
  if b < 5 then bit_vec_of_int_t_array_vec256_as_i16x16_lemma sh256 16 (16 * l + si + b)
#pop-options

(* ── the whole `deserialize_5_vec` obligation, one index at a time ───────────
   `co` is the 128+128 duplication `mm256_si256_from_two_si128 c c`; it enters
   as a FREE parameter carrying exactly the post the wrapper supplies. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deserialize_5_bits (c: t_Vec128) (co: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             forall (k: nat{k < 256}).
               bv_bit co k == (if k < 128 then bv_bit c k else bv_bit c (k - 128)))
          (ensures
             (let r = mm256_srli_epi16 (mk_i32 11)
                        (mm256_mullo_epi16 (mm256_shuffle_epi8 co BP.deser5_mask) deser5_mults) in
              bv_bit r i ==
              (if i % 16 >= 5 then 0
               else let shift_inv = ((i / 16) % 2) * 5 + (((i / 16) % 8) / 2) * 2 in
                    let j = i + shift_inv in
                    let byte_pos = j / 8 in
                    let c_byte = if byte_pos < 16
                                 then (byte_pos / 4) * 2 + byte_pos % 2
                                 else ((byte_pos - 16) / 4) * 2 + (byte_pos - 16) % 2 + 8 in
                    bv_bit c (c_byte * 8 + j % 8)))) =
  lemma_deser5_unpack_bit (mm256_shuffle_epi8 co BP.deser5_mask) i;
  if i % 16 < 5 then begin
    let l = i / 16 in
    let si = 5 * (l % 2) + 2 * ((l % 8) / 2) in
    BP.lemma_deser5_gather_bit c co (16 * l + si + i % 16)
  end
#pop-options

(* ── the outer deserialize_5 byte bridge ─────────────────────────────────────
   `deserialize_5` loads its 128-bit operand with a `mm_set_epi8` of ten
   `bytes[k] as i8`, duplicating the seven straddling bytes.  Byte n of that
   vector is source byte `deser10_bytemap n` — the SAME closed form the
   deserialize_10 gather uses, because both are the "one extra byte per 8-byte
   group" stride of a sub-byte-aligned code.

   This is a single lemma covering all k, rather than a per-k family. *)

(* bit (8n + t) of a 128-bit vector IS bit t of its byte n. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_vec128_byte (a: t_Vec128) (n: nat{n < 16}) (t: nat{t < 8})
  : Lemma (ensures bv_bit a (8 * n + t) == RI.get_bit (vec128_byte a n) (sz t)) =
  Canon.lemma_readback RI.I8 (mk_u64 128) (mk_u64 16) a (mk_u64 n) t;
  lemma_bv_bit_reader 8 a n t
#pop-options

(* `x as i8` keeps the eight bits of `x: u8` — the two's-complement branch of
   `get_bit` adds back exactly the 256 the cast subtracted. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
let lemma_get_bit_cast_u8_i8 (x: u8) (t: nat{t < 8})
  : Lemma (ensures RI.get_bit (cast x <: i8) (sz t) == RI.get_bit x (sz t)) =
  assert_norm (pow2 8 == 256);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I8);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.U8)
#pop-options

unfold let deser5_src (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9: u8) (k: nat{k < 10}) : u8 =
  if k = 0 then b0 else if k = 1 then b1 else if k = 2 then b2 else if k = 3 then b3
  else if k = 4 then b4 else if k = 5 then b5 else if k = 6 then b6 else if k = 7 then b7
  else if k = 8 then b8 else b9

unfold let deser5_load (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9: u8) =
  mm_set_epi8 (cast b9 <: i8) (cast b8 <: i8) (cast b8 <: i8) (cast b7 <: i8) (cast b7 <: i8)
              (cast b6 <: i8) (cast b6 <: i8) (cast b5 <: i8) (cast b4 <: i8) (cast b3 <: i8)
              (cast b3 <: i8) (cast b2 <: i8) (cast b2 <: i8) (cast b1 <: i8) (cast b1 <: i8)
              (cast b0 <: i8)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_deser5_load_bytes (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9: u8) (n: nat{n < 16})
  : Lemma (ensures vec128_byte (deser5_load b0 b1 b2 b3 b4 b5 b6 b7 b8 b9) n ==
                   (cast (deser5_src b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 (deser10_bytemap n)) <: i8)) =
  reveal_opaque (`%mm_set_epi8) mm_set_epi8;
  Canon.lemma_mm_set_epi8 (cast b9 <: i8) (cast b8 <: i8) (cast b8 <: i8) (cast b7 <: i8)
    (cast b7 <: i8) (cast b6 <: i8) (cast b6 <: i8) (cast b5 <: i8) (cast b4 <: i8)
    (cast b3 <: i8) (cast b3 <: i8) (cast b2 <: i8) (cast b2 <: i8) (cast b1 <: i8)
    (cast b1 <: i8) (cast b0 <: i8);
  Canon.lemma_iv_mm_set_epi8 (cast b9 <: i8) (cast b8 <: i8) (cast b8 <: i8) (cast b7 <: i8)
    (cast b7 <: i8) (cast b6 <: i8) (cast b6 <: i8) (cast b5 <: i8) (cast b4 <: i8)
    (cast b3 <: i8) (cast b3 <: i8) (cast b2 <: i8) (cast b2 <: i8) (cast b1 <: i8)
    (cast b1 <: i8) (cast b0 <: i8) n
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 300"
let lemma_deser5_src_index (bytes: t_Slice u8) (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9: u8)
      (k: nat{k < 10})
  : Lemma (requires Seq.length bytes == 10 /\
                    b0 == Seq.index bytes 0 /\ b1 == Seq.index bytes 1 /\
                    b2 == Seq.index bytes 2 /\ b3 == Seq.index bytes 3 /\
                    b4 == Seq.index bytes 4 /\ b5 == Seq.index bytes 5 /\
                    b6 == Seq.index bytes 6 /\ b7 == Seq.index bytes 7 /\
                    b8 == Seq.index bytes 8 /\ b9 == Seq.index bytes 9)
          (ensures deser5_src b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 k == Seq.index bytes k) = ()
#pop-options

(* THE index identity, verified by hand over all 16 lanes.  Result lane l bit b
   reads shuffled flat bit jj = 16*l + shift_inv(l) + b; the gather sends that
   to byte `deser5_bytemap (jj/8)` of the loaded vector, and the load sends THAT
   to source byte `deser10_bytemap` of it — which is exactly (5*l+b)/8, with the
   bit offset jj%8 already equal to (5*l+b)%8.  Sixteen ground arms on l, eight
   of which need the ONE b-split where jj crosses a multiple of 8.  Pure integer
   arithmetic: no vector term is in scope. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser5_outer_index (l: nat{l < 16}) (b: nat{b < 5})
  : Lemma (ensures
      (let jj = 16 * l + (5 * (l % 2) + 2 * ((l % 8) / 2)) + b in
       let j5 = 5 * l + b in
       jj < 256 /\ jj % 8 == j5 % 8 /\
       deser10_bytemap (BP.deser5_bytemap (jj / 8)) == j5 / 8)) =
  let jj = 16 * l + (5 * (l % 2) + 2 * ((l % 8) / 2)) + b in
  FStar.Math.Lemmas.euclidean_division_definition jj 8;
  (if l = 0       then assert (jj / 8 == 0)
   else if l = 1  then (if b < 3 then assert (jj / 8 == 2)  else assert (jj / 8 == 3))
   else if l = 2  then assert (jj / 8 == 4)
   else if l = 3  then (if b < 1 then assert (jj / 8 == 6)  else assert (jj / 8 == 7))
   else if l = 4  then (if b < 4 then assert (jj / 8 == 8)  else assert (jj / 8 == 9))
   else if l = 5  then assert (jj / 8 == 11)
   else if l = 6  then (if b < 2 then assert (jj / 8 == 12) else assert (jj / 8 == 13))
   else if l = 7  then assert (jj / 8 == 15)
   else if l = 8  then assert (jj / 8 == 16)
   else if l = 9  then (if b < 3 then assert (jj / 8 == 18) else assert (jj / 8 == 19))
   else if l = 10 then assert (jj / 8 == 20)
   else if l = 11 then (if b < 1 then assert (jj / 8 == 22) else assert (jj / 8 == 23))
   else if l = 12 then (if b < 4 then assert (jj / 8 == 24) else assert (jj / 8 == 25))
   else if l = 13 then assert (jj / 8 == 27)
   else if l = 14 then (if b < 2 then assert (jj / 8 == 28) else assert (jj / 8 == 29))
   else assert (jj / 8 == 31))
#pop-options

(* the whole outer `deserialize_5` obligation, one index at a time.  The ten
   byte locals enter as free parameters pinned to `Seq.index bytes k` by
   EQUATIONAL requires, so the caller discharges them from its own
   let-equations and no slice reasoning enters this context — the
   `lemma_store_glue_two_writes` shape. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deserialize_5_outer_bits
      (bytes: t_Slice u8) (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9: u8) (r: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             Seq.length bytes == 10 /\
             b0 == Seq.index bytes 0 /\ b1 == Seq.index bytes 1 /\
             b2 == Seq.index bytes 2 /\ b3 == Seq.index bytes 3 /\
             b4 == Seq.index bytes 4 /\ b5 == Seq.index bytes 5 /\
             b6 == Seq.index bytes 6 /\ b7 == Seq.index bytes 7 /\
             b8 == Seq.index bytes 8 /\ b9 == Seq.index bytes 9 /\
             (forall (k: nat{k < 256}).
                bv_bit r k ==
                (if k % 16 >= 5 then 0
                 else let shift_inv = ((k / 16) % 2) * 5 + (((k / 16) % 8) / 2) * 2 in
                      let j = k + shift_inv in
                      let byte_pos = j / 8 in
                      let c_byte = if byte_pos < 16
                                   then (byte_pos / 4) * 2 + byte_pos % 2
                                   else ((byte_pos - 16) / 4) * 2 + (byte_pos - 16) % 2 + 8 in
                      bv_bit (deser5_load b0 b1 b2 b3 b4 b5 b6 b7 b8 b9)
                             (c_byte * 8 + j % 8))))
          (ensures
             bv_bit r i ==
             (if i % 16 >= 5 then 0
              else Rust_primitives.BitVectors.bit_vec_of_int_t_array
                     (bytes <: t_Array u8 (sz 10)) 8 ((i / 16) * 5 + i % 16))) =
  if i % 16 < 5 then begin
    let l = i / 16 in
    let b = i % 16 in
    let jj = 16 * l + (5 * (l % 2) + 2 * ((l % 8) / 2)) + b in
    let n = jj / 8 in
    let t = jj % 8 in
    let m = BP.deser5_bytemap n in
    let src = deser10_bytemap m in
    lemma_deser5_outer_index l b;
    lemma_deser5_load_bytes b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 m;
    lemma_bv_bit_vec128_byte (deser5_load b0 b1 b2 b3 b4 b5 b6 b7 b8 b9) m t;
    lemma_deser5_src_index bytes b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 src;
    lemma_get_bit_cast_u8_i8 (deser5_src b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 src) t
  end
#pop-options
