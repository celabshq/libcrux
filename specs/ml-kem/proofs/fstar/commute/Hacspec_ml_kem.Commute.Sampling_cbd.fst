module Hacspec_ml_kem.Commute.Sampling_cbd
/// CBD (centered binomial distribution) sampling bridge for Track C.
/// Connects the impl's mask-shift-add popcount chunks
/// (sample_from_binomial_distribution_{2,3}) to the Hacspec spec
/// `Hacspec_ml_kem.Sampling.sample_poly_cbd` (bytes_to_bits + sum_coins).
///
/// Layering:
///   1. nat-level digit algebra: carry-free base-2^w addition (`dgt`,
///      `lemma_carry_free_add`) — the mathematical heart: the masked adds
///      (even+odd / first+second+third) add their 2-/3-bit fields without
///      carries.
///   2. literal-mask bit lemmas (`mask_rep`, `lemma_mask_rep_bit`) for
///      0x55555555 / 0x249249 at symbolic bit positions.
///   3. machine-word field lemmas (`lemma_cbd{2,3}_fields`,
///      `lemma_field_extract_{2,3}`, `lemma_w_bit_{4,3}`).
///   4. opaque per-coefficient atoms `cbd_coeff_{2,3}` + producer lemmas
///      (`lemma_cbd{2,3}_coeff`), loop-extension lemmas, and the
///      clean-context finalize (`lemma_cbd{2,3}_value_one`,
///      `lemma_cbd{2,3}_finalize`).
#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
open Rust_primitives.Integers

module ML = FStar.Math.Lemmas
module S  = Hacspec_ml_kem.Serialize
module P  = Hacspec_ml_kem.Parameters
module HS = Hacspec_ml_kem.Sampling
module SB = Hacspec_ml_kem.Commute.Serialize_bits
module SC = Hacspec_ml_kem.Commute.Serialize_compress
module VTS = Libcrux_ml_kem.Vector.Traits.Spec

(* ------------------------------------------------------------------ *)
(* 1. nat-level digit algebra                                          *)
(* ------------------------------------------------------------------ *)

(* boolean-to-int *)
let nb (b: bool) : int = if b then 1 else 0

(* base-2^w digit j of x.  OPAQUE: transparent div/mod-of-pow2 bodies destroy
   the forall triggers in lemma_carry_free_add (pow2 (w*0) never e-matches
   pow2 0) and saturate Z3 with non-linear arithmetic.  All body reasoning
   lives in lemma_dgt0 / lemma_dgt_shift / lemma_dgt_bits{2,3} /
   lemma_field_extract_{2,3} via targeted reveals. *)
[@@ "opaque_to_smt"]
let dgt (x: nat) (w: pos) (j: nat) : nat = (x / pow2 (w * j)) % pow2 w

#push-options "--z3rlimit 150"
let lemma_add_low (a b: nat) (vB: pos)
  : Lemma (requires a % vB + b % vB < vB)
          (ensures (a + b) % vB == a % vB + b % vB /\ (a + b) / vB == a / vB + b / vB)
  = ML.lemma_div_mod a vB;
    ML.lemma_div_mod b vB;
    let r = a % vB + b % vB in
    let q = a / vB + b / vB in
    assert (a + b == r + q * vB);
    ML.lemma_div_plus r q vB;
    ML.lemma_mod_plus r q vB;
    ML.small_div r vB;
    ML.small_mod r vB
#pop-options

#push-options "--z3rlimit 150"
let lemma_dgt_shift (x: nat) (w: pos) (j: nat)
  : Lemma (dgt x w (j + 1) == dgt (x / pow2 w) w j)
  = reveal_opaque (`%dgt) (dgt x w (j + 1));
    reveal_opaque (`%dgt) (dgt (x / pow2 w) w j);
    assert (w * (j + 1) == w + w * j);
    ML.pow2_plus w (w * j);
    ML.division_multiplication_lemma x (pow2 w) (pow2 (w * j))
#pop-options

#push-options "--z3rlimit 100"
let lemma_dgt0 (x: nat) (w: pos)
  : Lemma (dgt x w 0 == x % pow2 w)
  = reveal_opaque (`%dgt) (dgt x w 0);
    assert (w * 0 == 0);
    assert_norm (pow2 0 == 1);
    ML.cancel_mul_div x 1
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let rec lemma_carry_free_add (a b: nat) (w: pos) (n: nat)
  : Lemma
    (requires
      a < pow2 (w * n) /\ b < pow2 (w * n) /\
      (forall (j: nat). j < n ==> dgt a w j + dgt b w j < pow2 w))
    (ensures
      a + b < pow2 (w * n) /\
      (forall (j: nat). j < n ==> dgt (a + b) w j == dgt a w j + dgt b w j))
    (decreases n)
  = if n = 0 then ()
    else begin
      let vB = pow2 w in
      lemma_dgt0 a w; lemma_dgt0 b w; lemma_dgt0 (a + b) w;
      assert (dgt a w 0 + dgt b w 0 < vB);
      lemma_add_low a b vB;
      let aux (j: nat) : Lemma ((j < n - 1) ==> dgt (a / vB) w j + dgt (b / vB) w j < vB) =
        if j < n - 1 then begin lemma_dgt_shift a w j; lemma_dgt_shift b w j end
      in
      FStar.Classical.forall_intro aux;
      assert (w * n == w + w * (n - 1));
      ML.pow2_plus w (w * (n - 1));
      ML.lemma_div_lt_nat a (w * n) w;
      ML.lemma_div_lt_nat b (w * n) w;
      lemma_carry_free_add (a / vB) (b / vB) w (n - 1);
      let aux2 (j: nat) : Lemma ((j < n) ==> dgt (a + b) w j == dgt a w j + dgt b w j) =
        if j = 0 then ()
        else if j < n then begin
          lemma_dgt_shift (a + b) w (j - 1);
          lemma_dgt_shift a w (j - 1);
          lemma_dgt_shift b w (j - 1)
        end
      in
      FStar.Classical.forall_intro aux2;
      ML.lemma_div_mod (a + b) vB;
      ML.lemma_mult_le_left vB ((a + b) / vB) (pow2 (w * (n - 1)) - 1)
    end
#pop-options

(* bit (s+k) of y == bit k of y/2^s *)
#push-options "--z3rlimit 100"
let lemma_get_bit_nat_div (y: nat) (s k: nat)
  : Lemma (get_bit_nat (y / pow2 s) k == get_bit_nat y (s + k))
  = ML.pow2_plus s k;
    ML.division_multiplication_lemma y (pow2 s) (pow2 k)
#pop-options

(* x % 2^{k+1} == x % 2^k + 2^k * bit_k(x) *)
#push-options "--z3rlimit 150"
let lemma_mod_pow2_split (x: nat) (k: nat)
  : Lemma (x % pow2 (k + 1) == x % pow2 k + pow2 k * get_bit_nat x k)
  = ML.lemma_div_mod (x % pow2 (k + 1)) (pow2 k);
    ML.pow2_modulo_division_lemma_1 x k (k + 1);
    ML.pow2_modulo_modulo_lemma_1 x k (k + 1);
    assert_norm (pow2 1 == 2)
#pop-options

#push-options "--z3rlimit 200"
let lemma_dgt_bits2 (y: nat) (j: nat)
  : Lemma (dgt y 2 j == get_bit_nat y (2 * j) + 2 * get_bit_nat y (2 * j + 1))
  = reveal_opaque (`%dgt) (dgt y 2 j);
    let q = y / pow2 (2 * j) in
    assert_norm (pow2 0 == 1);
    assert_norm (pow2 1 == 2);
    assert_norm (pow2 2 == 4);
    lemma_mod_pow2_split q 1;
    ML.cancel_mul_div q 1;
    assert (q % 2 == get_bit_nat q 0);
    lemma_get_bit_nat_div y (2 * j) 0;
    lemma_get_bit_nat_div y (2 * j) 1
#pop-options

#push-options "--z3rlimit 200"
let lemma_dgt_bits3 (y: nat) (j: nat)
  : Lemma (dgt y 3 j ==
           get_bit_nat y (3 * j) + 2 * get_bit_nat y (3 * j + 1) + 4 * get_bit_nat y (3 * j + 2))
  = reveal_opaque (`%dgt) (dgt y 3 j);
    let q = y / pow2 (3 * j) in
    assert_norm (pow2 0 == 1);
    assert_norm (pow2 1 == 2);
    assert_norm (pow2 2 == 4);
    assert_norm (pow2 3 == 8);
    lemma_mod_pow2_split q 2;
    lemma_mod_pow2_split q 1;
    ML.cancel_mul_div q 1;
    assert (q % 2 == get_bit_nat q 0);
    lemma_get_bit_nat_div y (3 * j) 0;
    lemma_get_bit_nat_div y (3 * j) 1;
    lemma_get_bit_nat_div y (3 * j) 2
#pop-options

(* bit m of (lo + 2^s * hi), lo < 2^s *)
#push-options "--z3rlimit 300"
let lemma_get_bit_nat_split (lo hi: nat) (s m: nat)
  : Lemma (requires lo < pow2 s)
          (ensures get_bit_nat (lo + pow2 s * hi) m ==
                   (if m < s then get_bit_nat lo m else get_bit_nat hi (m - s)))
  = if m < s then begin
      ML.pow2_plus (s - m) m;
      assert (lo + pow2 s * hi == lo + (pow2 (s - m) * hi) * pow2 m);
      ML.lemma_div_plus lo (pow2 (s - m) * hi) (pow2 m);
      (* (lo + 2^s hi)/2^m == lo/2^m + 2^{s-m} * hi *)
      ML.pow2_plus 1 (s - m - 1);
      assert (pow2 (s - m) * hi == (pow2 (s - m - 1) * hi) * 2);
      ML.lemma_mod_plus (lo / pow2 m) (pow2 (s - m - 1) * hi) 2
    end
    else begin
      ML.pow2_plus s (m - s);
      ML.division_multiplication_lemma (lo + pow2 s * hi) (pow2 s) (pow2 (m - s));
      ML.lemma_div_plus lo hi (pow2 s);
      ML.small_div lo (pow2 s)
    end
#pop-options

(* ------------------------------------------------------------------ *)
(* 2. literal masks: 0x55555555 = mask_rep 4 16, 0x249249 = mask_rep 8 8 *)
(* ------------------------------------------------------------------ *)

let rec mask_rep (base: pos) (n: nat) : nat =
  if n = 0 then 0 else 1 + base * mask_rep base (n - 1)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let rec lemma_mask_rep_bit (wd: pos) (n: nat) (m: nat)
  : Lemma (ensures get_bit_nat (mask_rep (pow2 wd) n) m ==
                   (if m < wd * n && m % wd = 0 then 1 else 0))
          (decreases n)
  = if n = 0 then begin
      assert (mask_rep (pow2 wd) 0 == 0);
      ML.small_div 0 (pow2 m)
    end
    else begin
      let rest = mask_rep (pow2 wd) (n - 1) in
      assert (mask_rep (pow2 wd) n == 1 + pow2 wd * rest);
      ML.pow2_le_compat wd 1;
      lemma_get_bit_nat_split 1 rest wd m;
      if m < wd then begin
        if m = 0 then begin
          assert_norm (pow2 0 == 1);
          ML.cancel_mul_div 1 1;
          assert (get_bit_nat 1 0 == 1);
          ML.lemma_mult_le_left wd 1 n
        end
        else begin
          ML.pow2_le_compat m 1;
          ML.small_div 1 (pow2 m);
          assert (get_bit_nat 1 m == 0)
        end
      end
      else begin
        lemma_mask_rep_bit wd (n - 1) (m - wd);
        ML.lemma_mod_plus (m - wd) 1 wd;
        assert (wd * n == wd + wd * (n - 1))
      end
    end
#pop-options

(* ------------------------------------------------------------------ *)
(* 3. machine-word field lemmas                                        *)
(* ------------------------------------------------------------------ *)

(* field extraction: (ct >> os) & (2^w - 1) is digit os/w *)
#push-options "--z3rlimit 200"
let lemma_field_extract_2 (ct: u32) (os: u32)
  : Lemma (requires v os < 32 /\ v os % 2 == 0)
          (ensures v ((ct >>! os) &. mk_u32 3) == dgt (v ct) 2 (v os / 2))
  = reveal_opaque (`%dgt) (dgt (v ct) 2 (v os / 2));
    let sh = ct >>! os in
    shift_right_lemma ct os;
    logand_mask_lemma sh 2;
    assert_norm (sub #U32 (mk_int #U32 (pow2 2)) (mk_int #U32 1) == mk_u32 3);
    assert (2 * (v os / 2) == v os);
    assert_norm (pow2 2 == 4)
#pop-options

#push-options "--z3rlimit 200"
let lemma_field_extract_3 (ct: u32) (os: i32)
  : Lemma (requires v os >= 0 /\ v os < 32 /\ v os % 3 == 0)
          (ensures v ((ct >>! os) &. mk_u32 7) == dgt (v ct) 3 (v os / 3))
  = reveal_opaque (`%dgt) (dgt (v ct) 3 (v os / 3));
    let sh = ct >>! os in
    shift_right_lemma ct os;
    logand_mask_lemma sh 3;
    assert_norm (sub #U32 (mk_int #U32 (pow2 3)) (mk_int #U32 1) == mk_u32 7);
    assert (3 * (v os / 3) == v os);
    assert_norm (pow2 3 == 8)
#pop-options

(* digit j of coin_toss == bit (2j) + bit (2j+1) of the source word (eta = 2) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd2_fields (w ct: u32) (j: nat{j < 16})
  : Lemma
    (requires v ct == v (w &. mk_u32 1431655765) + v ((w >>! mk_i32 1) &. mk_u32 1431655765))
    (ensures dgt (v ct) 2 j == get_bit_nat (v w) (2 * j) + get_bit_nat (v w) (2 * j + 1))
  = let m32 = mk_u32 1431655765 in
    let ev = w &. m32 in
    let od = (w >>! mk_i32 1) &. m32 in
    assert_norm (mask_rep (pow2 2) 16 == 1431655765);
    assert_norm (pow2 2 == 4);
    assert_norm (pow2 (2 * 16) == 4294967296);
    let evbit (p: nat{p < 32}) : Lemma
      (get_bit_nat (v ev) p == (if p % 2 = 0 then get_bit_nat (v w) p else 0)) =
      let pp = mk_usize p in
      get_bit_and w m32 pp;
      SB.lemma_get_bit_nat_eq ev pp;
      SB.lemma_get_bit_nat_eq w pp;
      SB.lemma_get_bit_nat_eq m32 pp;
      lemma_mask_rep_bit 2 16 p
    in
    let odbit (p: nat{p < 32}) : Lemma
      (get_bit_nat (v od) p ==
       (if p % 2 = 0 && p < 31 then get_bit_nat (v w) (p + 1) else 0)) =
      let pp = mk_usize p in
      get_bit_and (w >>! mk_i32 1) m32 pp;
      get_bit_shr w (mk_i32 1) pp;
      SB.lemma_get_bit_nat_eq od pp;
      SB.lemma_get_bit_nat_eq (w >>! mk_i32 1) pp;
      SB.lemma_get_bit_nat_eq m32 pp;
      (if p < 31 then SB.lemma_get_bit_nat_eq w (mk_usize (p + 1)));
      lemma_mask_rep_bit 2 16 p
    in
    let evd (jj: nat{jj < 16}) : Lemma (dgt (v ev) 2 jj == get_bit_nat (v w) (2 * jj)) =
      lemma_dgt_bits2 (v ev) jj; evbit (2 * jj); evbit (2 * jj + 1)
    in
    let odd_ (jj: nat{jj < 16}) : Lemma (dgt (v od) 2 jj == get_bit_nat (v w) (2 * jj + 1)) =
      lemma_dgt_bits2 (v od) jj; odbit (2 * jj); odbit (2 * jj + 1)
    in
    let auxh (jj: nat) : Lemma ((jj < 16) ==> dgt (v ev) 2 jj + dgt (v od) 2 jj < pow2 2) =
      if jj < 16 then begin evd jj; odd_ jj end
    in
    FStar.Classical.forall_intro auxh;
    assert (v ev < pow2 (2 * 16));
    assert (v od < pow2 (2 * 16));
    lemma_carry_free_add (v ev) (v od) 2 16;
    evd j; odd_ j
#pop-options

(* digit j of coin_toss == bit (3j) + bit (3j+1) + bit (3j+2) of source (eta = 3) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd3_fields (w ct: u32) (j: nat{j < 8})
  : Lemma
    (requires v ct == (v (w &. mk_u32 2396745) + v ((w >>! mk_i32 1) &. mk_u32 2396745)) +
                      v ((w >>! mk_i32 2) &. mk_u32 2396745))
    (ensures dgt (v ct) 3 j ==
             get_bit_nat (v w) (3 * j) + get_bit_nat (v w) (3 * j + 1) + get_bit_nat (v w) (3 * j + 2))
  = let m24 = mk_u32 2396745 in
    let f1 = w &. m24 in
    let f2 = (w >>! mk_i32 1) &. m24 in
    let f3 = (w >>! mk_i32 2) &. m24 in
    assert_norm (mask_rep (pow2 3) 8 == 2396745);
    assert_norm (pow2 3 == 8);
    assert_norm (pow2 (3 * 8) == 16777216);
    logand_lemma w m24;
    logand_lemma (w >>! mk_i32 1) m24;
    logand_lemma (w >>! mk_i32 2) m24;
    assert (v f1 <= 2396745 /\ v f2 <= 2396745 /\ v f3 <= 2396745);
    (* NOTE: the 0x249249 mask covers only bits < 24 (= 3*8); all digit uses
       below stay at p <= 23, so the helpers take p < 24. *)
    let f1bit (p: nat{p < 24}) : Lemma
      (get_bit_nat (v f1) p == (if p % 3 = 0 then get_bit_nat (v w) p else 0)) =
      let pp = mk_usize p in
      get_bit_and w m24 pp;
      SB.lemma_get_bit_nat_eq f1 pp;
      SB.lemma_get_bit_nat_eq w pp;
      SB.lemma_get_bit_nat_eq m24 pp;
      lemma_mask_rep_bit 3 8 p
    in
    let f2bit (p: nat{p < 24}) : Lemma
      (get_bit_nat (v f2) p == (if p % 3 = 0 then get_bit_nat (v w) (p + 1) else 0)) =
      let pp = mk_usize p in
      get_bit_and (w >>! mk_i32 1) m24 pp;
      get_bit_shr w (mk_i32 1) pp;
      SB.lemma_get_bit_nat_eq (w >>! mk_i32 1) pp;
      SB.lemma_get_bit_nat_eq f2 pp;
      SB.lemma_get_bit_nat_eq m24 pp;
      SB.lemma_get_bit_nat_eq w (mk_usize (p + 1));
      lemma_mask_rep_bit 3 8 p
    in
    let f3bit (p: nat{p < 24}) : Lemma
      (get_bit_nat (v f3) p == (if p % 3 = 0 then get_bit_nat (v w) (p + 2) else 0)) =
      let pp = mk_usize p in
      get_bit_and (w >>! mk_i32 2) m24 pp;
      get_bit_shr w (mk_i32 2) pp;
      SB.lemma_get_bit_nat_eq (w >>! mk_i32 2) pp;
      SB.lemma_get_bit_nat_eq f3 pp;
      SB.lemma_get_bit_nat_eq m24 pp;
      SB.lemma_get_bit_nat_eq w (mk_usize (p + 2));
      lemma_mask_rep_bit 3 8 p
    in
    let f1d (jj: nat{jj < 8}) : Lemma (dgt (v f1) 3 jj == get_bit_nat (v w) (3 * jj)) =
      lemma_dgt_bits3 (v f1) jj;
      f1bit (3 * jj); f1bit (3 * jj + 1); f1bit (3 * jj + 2)
    in
    let f2d (jj: nat{jj < 8}) : Lemma (dgt (v f2) 3 jj == get_bit_nat (v w) (3 * jj + 1)) =
      lemma_dgt_bits3 (v f2) jj;
      f2bit (3 * jj); f2bit (3 * jj + 1); f2bit (3 * jj + 2)
    in
    let f3d (jj: nat{jj < 8}) : Lemma (dgt (v f3) 3 jj == get_bit_nat (v w) (3 * jj + 2)) =
      lemma_dgt_bits3 (v f3) jj;
      f3bit (3 * jj); f3bit (3 * jj + 1); f3bit (3 * jj + 2)
    in
    let auxh (jj: nat) : Lemma ((jj < 8) ==> dgt (v f1) 3 jj + dgt (v f2) 3 jj < pow2 3) =
      if jj < 8 then begin f1d jj; f2d jj end
    in
    FStar.Classical.forall_intro auxh;
    assert (v f1 < pow2 (3 * 8));
    assert (v f2 < pow2 (3 * 8));
    lemma_carry_free_add (v f1) (v f2) 3 8;
    let ab : nat = v f1 + v f2 in
    let auxh2 (jj: nat) : Lemma ((jj < 8) ==> dgt ab 3 jj + dgt (v f3) 3 jj < pow2 3) =
      if jj < 8 then begin f1d jj; f2d jj; f3d jj end
    in
    FStar.Classical.forall_intro auxh2;
    assert (v f3 < pow2 (3 * 8));
    lemma_carry_free_add ab (v f3) 3 8;
    f1d j; f2d j; f3d j
#pop-options

(* bit m of x is 0 when x < 2^m (clean-context helper: get_bit_nat's
   definitional unfold is flaky inside heavy lemma bodies) *)
#push-options "--z3rlimit 100"
let lemma_get_bit_nat_zero (x: nat) (m: nat)
  : Lemma (requires x < pow2 m) (ensures get_bit_nat x m == 0)
  = ML.small_div x (pow2 m)
#pop-options

(* byte-assembly: bit m of the 4-byte LE word.
   NOTE: the casts MUST be the `Rust_primitives.cast` typeclass method
   (= cast_mod) — that is what hax extraction elaborates `as`-casts to;
   the bare `cast` here resolves to Rust_primitives.Integers.cast (a
   DIFFERENT term) and the call-site hypothesis would not match. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_w_bit_4 (b0 b1 b2 b3: u8) (w: u32) (m: nat{m < 32})
  : Lemma
    (requires
      w == ((((Rust_primitives.cast #u8 #u32 b0 <: u32) |. ((Rust_primitives.cast #u8 #u32 b1 <: u32) <<! mk_i32 8 <: u32) <: u32) |.
             ((Rust_primitives.cast #u8 #u32 b2 <: u32) <<! mk_i32 16 <: u32) <: u32) |.
            ((Rust_primitives.cast #u8 #u32 b3 <: u32) <<! mk_i32 24 <: u32)))
    (ensures get_bit_nat (v w) m ==
             (if m < 8 then get_bit_nat (v b0) m
              else if m < 16 then get_bit_nat (v b1) (m - 8)
              else if m < 24 then get_bit_nat (v b2) (m - 16)
              else get_bit_nat (v b3) (m - 24)))
  = let c0:u32 = Rust_primitives.cast #u8 #u32 b0 in
    let c1:u32 = Rust_primitives.cast #u8 #u32 b1 in
    let c2:u32 = Rust_primitives.cast #u8 #u32 b2 in
    let c3:u32 = Rust_primitives.cast #u8 #u32 b3 in
    let s1:u32 = c1 <<! mk_i32 8 in
    let s2:u32 = c2 <<! mk_i32 16 in
    let s3:u32 = c3 <<! mk_i32 24 in
    let t1:u32 = c0 |. s1 in
    let t2:u32 = t1 |. s2 in
    let mm = mk_usize m in
    assert_norm (pow2 8 == 256);
    assert_norm (pow2 32 == 4294967296);
    ML.small_mod (v b0) 4294967296;
    ML.small_mod (v b1) 4294967296;
    ML.small_mod (v b2) 4294967296;
    ML.small_mod (v b3) 4294967296;
    assert (v b0 >= 0 /\ v b1 >= 0 /\ v b2 >= 0 /\ v b3 >= 0);
    assert (v b0 < 256 /\ v b1 < 256 /\ v b2 < 256 /\ v b3 < 256);
    assert (v c0 == v b0 /\ v c1 == v b1 /\ v c2 == v b2 /\ v c3 == v b3);
    SB.lemma_get_bit_nat_eq w mm;
    get_bit_or t2 s3 mm;
    get_bit_or t1 s2 mm;
    get_bit_or c0 s1 mm;
    get_bit_shl c1 (mk_i32 8) mm;
    get_bit_shl c2 (mk_i32 16) mm;
    get_bit_shl c3 (mk_i32 24) mm;
    SB.lemma_get_bit_nat_eq t2 mm;
    SB.lemma_get_bit_nat_eq t1 mm;
    SB.lemma_get_bit_nat_eq c0 mm;
    (* low byte zero above 8 bits *)
    if m >= 8 then begin
      ML.pow2_le_compat m 8;
      lemma_get_bit_nat_zero (v b0) m
    end;
    if m >= 8 && m < 16 then SB.lemma_get_bit_nat_eq c1 (mk_usize (m - 8));
    if m >= 16 && m < 24 then begin
      SB.lemma_get_bit_nat_eq c2 (mk_usize (m - 16));
      ML.pow2_le_compat (m - 8) 8;
      lemma_get_bit_nat_zero (v b1) (m - 8)
    end;
    if m >= 24 then begin
      SB.lemma_get_bit_nat_eq c3 (mk_usize (m - 24));
      ML.pow2_le_compat (m - 8) 8;
      lemma_get_bit_nat_zero (v b1) (m - 8);
      ML.pow2_le_compat (m - 16) 8;
      lemma_get_bit_nat_zero (v b2) (m - 16)
    end
#pop-options

(* byte-assembly: bit m of the 3-byte LE word (eta = 3) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_w_bit_3 (b0 b1 b2: u8) (w: u32) (m: nat{m < 24})
  : Lemma
    (requires
      w == (((Rust_primitives.cast #u8 #u32 b0 <: u32) |. ((Rust_primitives.cast #u8 #u32 b1 <: u32) <<! mk_i32 8 <: u32) <: u32) |.
            ((Rust_primitives.cast #u8 #u32 b2 <: u32) <<! mk_i32 16 <: u32)))
    (ensures get_bit_nat (v w) m ==
             (if m < 8 then get_bit_nat (v b0) m
              else if m < 16 then get_bit_nat (v b1) (m - 8)
              else get_bit_nat (v b2) (m - 16)))
  = let c0:u32 = Rust_primitives.cast #u8 #u32 b0 in
    let c1:u32 = Rust_primitives.cast #u8 #u32 b1 in
    let c2:u32 = Rust_primitives.cast #u8 #u32 b2 in
    let s1:u32 = c1 <<! mk_i32 8 in
    let s2:u32 = c2 <<! mk_i32 16 in
    let t1:u32 = c0 |. s1 in
    let mm = mk_usize m in
    assert_norm (pow2 8 == 256);
    assert_norm (pow2 32 == 4294967296);
    ML.small_mod (v b0) 4294967296;
    ML.small_mod (v b1) 4294967296;
    ML.small_mod (v b2) 4294967296;
    assert (v b0 >= 0 /\ v b1 >= 0 /\ v b2 >= 0);
    assert (v b0 < 256 /\ v b1 < 256 /\ v b2 < 256);
    assert (v c0 == v b0 /\ v c1 == v b1 /\ v c2 == v b2);
    SB.lemma_get_bit_nat_eq w mm;
    get_bit_or t1 s2 mm;
    get_bit_or c0 s1 mm;
    get_bit_shl c1 (mk_i32 8) mm;
    get_bit_shl c2 (mk_i32 16) mm;
    SB.lemma_get_bit_nat_eq t1 mm;
    SB.lemma_get_bit_nat_eq c0 mm;
    if m >= 8 then begin
      ML.pow2_le_compat m 8;
      lemma_get_bit_nat_zero (v b0) m
    end;
    if m >= 8 && m < 16 then SB.lemma_get_bit_nat_eq c1 (mk_usize (m - 8));
    if m >= 16 then begin
      SB.lemma_get_bit_nat_eq c2 (mk_usize (m - 16));
      ML.pow2_le_compat (m - 8) 8;
      lemma_get_bit_nat_zero (v b1) (m - 8)
    end
#pop-options

(* ------------------------------------------------------------------ *)
(* 4. opaque per-coefficient atoms + producers                         *)
(* ------------------------------------------------------------------ *)

[@@ "opaque_to_smt"]
let cbd_coeff_2 (randomness: t_Slice u8{Seq.length randomness == 128})
                (c: i16) (k: nat{k < 256}) : prop
  = let bits = S.bytes_to_bits (mk_usize 128) (mk_usize 1024) randomness in
    v c == nb (Seq.index bits (4 * k)) + nb (Seq.index bits (4 * k + 1))
         - nb (Seq.index bits (4 * k + 2)) - nb (Seq.index bits (4 * k + 3))

[@@ "opaque_to_smt"]
let cbd_coeff_3 (randomness: t_Slice u8{Seq.length randomness == 192})
                (c: i16) (k: nat{k < 256}) : prop
  = let bits = S.bytes_to_bits (mk_usize 192) (mk_usize 1536) randomness in
    v c == nb (Seq.index bits (6 * k)) + nb (Seq.index bits (6 * k + 1)) + nb (Seq.index bits (6 * k + 2))
         - nb (Seq.index bits (6 * k + 3)) - nb (Seq.index bits (6 * k + 4)) - nb (Seq.index bits (6 * k + 5))

(* producer: the impl's per-(chunk, outcome_set) value satisfies the atom (eta = 2) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd2_coeff
    (randomness: t_Slice u8)
    (b0 b1 b2 b3: u8) (w ct: u32) (os: u32) (c: i16) (n: nat{n < 32})
  : Lemma
    (requires
      Seq.length randomness == 128 /\
      b0 == Seq.index randomness (4 * n) /\
      b1 == Seq.index randomness (4 * n + 1) /\
      b2 == Seq.index randomness (4 * n + 2) /\
      b3 == Seq.index randomness (4 * n + 3) /\
      w == ((((Rust_primitives.cast #u8 #u32 b0 <: u32) |. ((Rust_primitives.cast #u8 #u32 b1 <: u32) <<! mk_i32 8 <: u32) <: u32) |.
             ((Rust_primitives.cast #u8 #u32 b2 <: u32) <<! mk_i32 16 <: u32) <: u32) |.
            ((Rust_primitives.cast #u8 #u32 b3 <: u32) <<! mk_i32 24 <: u32)) /\
      v ct == v (w &. mk_u32 1431655765) + v ((w >>! mk_i32 1) &. mk_u32 1431655765) /\
      v os % 4 == 0 /\ v os < 32 /\
      v c == v ((ct >>! os) &. mk_u32 3) - v ((ct >>! (os +! mk_u32 2 <: u32)) &. mk_u32 3))
    (ensures cbd_coeff_2 randomness c (8 * n + v os / 4))
  = let t : nat = v os / 4 in
    let k : nat = 8 * n + t in
    assert (t < 8);
    (* field values of ct *)
    lemma_cbd2_fields w ct (2 * t);
    lemma_cbd2_fields w ct (2 * t + 1);
    lemma_field_extract_2 ct os;
    lemma_field_extract_2 ct (os +! mk_u32 2);
    assert (v os / 2 == 2 * t);
    assert (v (os +! mk_u32 2) / 2 == 2 * t + 1);
    assert (v c == (get_bit_nat (v w) (4 * t) + get_bit_nat (v w) (4 * t + 1))
                 - (get_bit_nat (v w) (4 * t + 2) + get_bit_nat (v w) (4 * t + 3)));
    (* spec bits *)
    assert (mk_usize (32 * v (sz 4)) == mk_usize 128);
    assert (mk_usize (256 * v (sz 4)) == mk_usize 1024);
    let bitm (j: nat{j < 4}) : Lemma
      (Seq.index (S.bytes_to_bits (mk_usize 128) (mk_usize 1024) randomness) (4 * k + j)
       == (get_bit_nat (v w) (4 * t + j) = 1)) =
      let m = 4 * k + j in
      assert (m == 32 * n + (4 * t + j));
      SC.lemma_bytes_to_bits_index_d (sz 4) randomness m;
      ML.lemma_div_plus (4 * t + j) (4 * n) 8;
      ML.lemma_mod_plus (4 * t + j) (4 * n) 8;
      assert (m / 8 == 4 * n + (4 * t + j) / 8);
      assert (m % 8 == (4 * t + j) % 8);
      lemma_w_bit_4 b0 b1 b2 b3 w (4 * t + j);
      assert ((4 * t + j) / 8 == (if 4 * t + j < 8 then 0 else if 4 * t + j < 16 then 1 else if 4 * t + j < 24 then 2 else 3));
      assert ((4 * t + j) % 8 == 4 * t + j - 8 * ((4 * t + j) / 8))
    in
    bitm 0; bitm 1; bitm 2; bitm 3;
    reveal_opaque (`%cbd_coeff_2) (cbd_coeff_2 randomness c k)
#pop-options

(* producer: eta = 3 *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd3_coeff
    (randomness: t_Slice u8)
    (b0 b1 b2: u8) (w ct: u32) (os: i32) (c: i16) (n: nat{n < 64})
  : Lemma
    (requires
      Seq.length randomness == 192 /\
      b0 == Seq.index randomness (3 * n) /\
      b1 == Seq.index randomness (3 * n + 1) /\
      b2 == Seq.index randomness (3 * n + 2) /\
      w == (((Rust_primitives.cast #u8 #u32 b0 <: u32) |. ((Rust_primitives.cast #u8 #u32 b1 <: u32) <<! mk_i32 8 <: u32) <: u32) |.
            ((Rust_primitives.cast #u8 #u32 b2 <: u32) <<! mk_i32 16 <: u32)) /\
      v ct == (v (w &. mk_u32 2396745) + v ((w >>! mk_i32 1) &. mk_u32 2396745)) +
              v ((w >>! mk_i32 2) &. mk_u32 2396745) /\
      v os >= 0 /\ v os % 6 == 0 /\ v os < 24 /\
      v c == v ((ct >>! os) &. mk_u32 7) - v ((ct >>! (os +! mk_i32 3 <: i32)) &. mk_u32 7))
    (ensures cbd_coeff_3 randomness c (4 * n + v os / 6))
  = let t : nat = v os / 6 in
    let k : nat = 4 * n + t in
    assert (t < 4);
    lemma_cbd3_fields w ct (2 * t);
    lemma_cbd3_fields w ct (2 * t + 1);
    lemma_field_extract_3 ct os;
    lemma_field_extract_3 ct (os +! mk_i32 3);
    assert (v os / 3 == 2 * t);
    assert (v (os +! mk_i32 3) / 3 == 2 * t + 1);
    assert (v c == (get_bit_nat (v w) (6 * t) + get_bit_nat (v w) (6 * t + 1) + get_bit_nat (v w) (6 * t + 2))
                 - (get_bit_nat (v w) (6 * t + 3) + get_bit_nat (v w) (6 * t + 4) + get_bit_nat (v w) (6 * t + 5)));
    assert (mk_usize (32 * v (sz 6)) == mk_usize 192);
    assert (mk_usize (256 * v (sz 6)) == mk_usize 1536);
    let bitm (j: nat{j < 6}) : Lemma
      (Seq.index (S.bytes_to_bits (mk_usize 192) (mk_usize 1536) randomness) (6 * k + j)
       == (get_bit_nat (v w) (6 * t + j) = 1)) =
      let m = 6 * k + j in
      assert (m == 24 * n + (6 * t + j));
      SC.lemma_bytes_to_bits_index_d (sz 6) randomness m;
      ML.lemma_div_plus (6 * t + j) (3 * n) 8;
      ML.lemma_mod_plus (6 * t + j) (3 * n) 8;
      assert (m / 8 == 3 * n + (6 * t + j) / 8);
      assert (m % 8 == (6 * t + j) % 8);
      lemma_w_bit_3 b0 b1 b2 w (6 * t + j);
      assert ((6 * t + j) / 8 == (if 6 * t + j < 8 then 0 else if 6 * t + j < 16 then 1 else 2));
      assert ((6 * t + j) % 8 == 6 * t + j - 8 * ((6 * t + j) / 8))
    in
    bitm 0; bitm 1; bitm 2; bitm 3; bitm 4; bitm 5;
    reveal_opaque (`%cbd_coeff_3) (cbd_coeff_3 randomness c k)
#pop-options

(* loop-extension lemmas (standalone — inline forall-extension over an
   opaque atom saturates in heavy composer contexts) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_cbd2_extend
    (randomness: t_Slice u8{Seq.length randomness == 128})
    (arr_old arr_new: t_Array i16 (mk_usize 256))
    (bound: nat{bound < 256}) (c: i16)
  : Lemma
    (requires
      (forall (j: nat). j < bound ==> cbd_coeff_2 randomness (Seq.index arr_old j) j) /\
      arr_new == Seq.upd arr_old bound c /\
      cbd_coeff_2 randomness c bound)
    (ensures forall (j: nat). j < bound + 1 ==> cbd_coeff_2 randomness (Seq.index arr_new j) j)
  = let aux (j: nat) : Lemma ((j < bound + 1) ==> cbd_coeff_2 randomness (Seq.index arr_new j) j) =
      if j < bound then assert (Seq.index arr_new j == Seq.index arr_old j)
      else if j = bound then assert (Seq.index arr_new j == c)
    in
    FStar.Classical.forall_intro aux

let lemma_cbd3_extend
    (randomness: t_Slice u8{Seq.length randomness == 192})
    (arr_old arr_new: t_Array i16 (mk_usize 256))
    (bound: nat{bound < 256}) (c: i16)
  : Lemma
    (requires
      (forall (j: nat). j < bound ==> cbd_coeff_3 randomness (Seq.index arr_old j) j) /\
      arr_new == Seq.upd arr_old bound c /\
      cbd_coeff_3 randomness c bound)
    (ensures forall (j: nat). j < bound + 1 ==> cbd_coeff_3 randomness (Seq.index arr_new j) j)
  = let aux (j: nat) : Lemma ((j < bound + 1) ==> cbd_coeff_3 randomness (Seq.index arr_new j) j) =
      if j < bound then assert (Seq.index arr_new j == Seq.index arr_old j)
      else if j = bound then assert (Seq.index arr_new j == c)
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ------------------------------------------------------------------ *)
(* 5. spec-side reductions: sum_coins at concrete eta                  *)
(* ------------------------------------------------------------------ *)

#push-options "--fuel 4 --ifuel 1 --z3rlimit 200"
let lemma_sum_coins_2 (coins: t_Slice bool)
  : Lemma (requires Seq.length coins == 2)
          (ensures v (HS.sum_coins (mk_usize 2) coins).P.f_val
                   == nb (Seq.index coins 0) + nb (Seq.index coins 1))
  = assert (v (HS.sum_coins (mk_usize 2) coins).P.f_val
            == nb (Seq.index coins 0) + nb (Seq.index coins 1))
      by (FStar.Tactics.norm [delta_only [`%HS.sum_coins; `%Rust_primitives.Hax.Folds.fold_range;
                                          `%nb]; zeta; iota; primops];
          FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 5 --ifuel 1 --z3rlimit 200"
let lemma_sum_coins_3 (coins: t_Slice bool)
  : Lemma (requires Seq.length coins == 3)
          (ensures v (HS.sum_coins (mk_usize 3) coins).P.f_val
                   == nb (Seq.index coins 0) + nb (Seq.index coins 1) + nb (Seq.index coins 2))
  = assert (v (HS.sum_coins (mk_usize 3) coins).P.f_val
            == nb (Seq.index coins 0) + nb (Seq.index coins 1) + nb (Seq.index coins 2))
      by (FStar.Tactics.norm [delta_only [`%HS.sum_coins; `%Rust_primitives.Hax.Folds.fold_range;
                                          `%nb]; zeta; iota; primops];
          FStar.Tactics.smt ())
#pop-options

(* ------------------------------------------------------------------ *)
(* 6. per-coefficient value match against sample_poly_cbd              *)
(* ------------------------------------------------------------------ *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd2_value_one
    (randomness: t_Slice u8{Seq.length randomness == 128})
    (c: i16) (k: nat{k < 256})
  : Lemma
    (requires cbd_coeff_2 randomness c k)
    (ensures
      v c >= -2 /\ v c <= 2 /\
      VTS.i16_to_spec_fe c ==
      Seq.index (HS.sample_poly_cbd (mk_usize 128) (mk_usize 1024) (mk_usize 2) randomness) k)
  = reveal_opaque (`%cbd_coeff_2) (cbd_coeff_2 randomness c k);
    let kk = mk_usize k in
    assert (k == v kk);
    (* spec index via createi *)
    assert (Seq.index (HS.sample_poly_cbd (mk_usize 128) (mk_usize 1024) (mk_usize 2) randomness) (v kk)
            == (let bits = S.bytes_to_bits (mk_usize 128) (mk_usize 1024) randomness in
                let x = HS.sum_coins (mk_usize 2)
                  (bits.[ { Core_models.Ops.Range.f_start = (mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize;
                            Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize) +! mk_usize 2 <: usize }
                          <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
                let y = HS.sum_coins (mk_usize 2)
                  (bits.[ { Core_models.Ops.Range.f_start = ((mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize) +! mk_usize 2 <: usize;
                            Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize) +! (mk_usize 2 *! mk_usize 2 <: usize) <: usize }
                          <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
                P.impl_FieldElement__new (((x.P.f_val +! P.v_FIELD_MODULUS <: u16) -! y.P.f_val <: u16) %!
                                          P.v_FIELD_MODULUS <: u16)))
      by (FStar.Tactics.norm [delta_only [`%HS.sample_poly_cbd]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    let bits = S.bytes_to_bits (mk_usize 128) (mk_usize 1024) randomness in
    let xs = (bits.[ { Core_models.Ops.Range.f_start = (mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize;
                       Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize) +! mk_usize 2 <: usize }
                     <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
    let ys = (bits.[ { Core_models.Ops.Range.f_start = ((mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize) +! mk_usize 2 <: usize;
                       Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 2 <: usize) +! (mk_usize 2 *! mk_usize 2 <: usize) <: usize }
                     <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
    assert (xs == Seq.slice bits (4 * k) (4 * k + 2));
    assert (ys == Seq.slice bits (4 * k + 2) (4 * k + 4));
    Seq.lemma_index_slice bits (4 * k) (4 * k + 2) 0;
    Seq.lemma_index_slice bits (4 * k) (4 * k + 2) 1;
    Seq.lemma_index_slice bits (4 * k + 2) (4 * k + 4) 0;
    Seq.lemma_index_slice bits (4 * k + 2) (4 * k + 4) 1;
    lemma_sum_coins_2 xs;
    lemma_sum_coins_2 ys;
    let x = HS.sum_coins (mk_usize 2) xs in
    let y = HS.sum_coins (mk_usize 2) ys in
    let xv : nat = v x.P.f_val in
    let yv : nat = v y.P.f_val in
    assert (xv <= 2 /\ yv <= 2);
    assert (v c == xv - yv);
    let rhs = P.impl_FieldElement__new (((x.P.f_val +! P.v_FIELD_MODULUS <: u16) -! y.P.f_val <: u16) %!
                                        P.v_FIELD_MODULUS <: u16) in
    assert (v rhs.P.f_val == (xv + 3329 - yv) % 3329);
    let r1 = VTS.i16_to_spec_fe c in
    assert (v r1.P.f_val == v c % 3329);
    ML.lemma_mod_plus (xv - yv) 1 3329;
    assert (v r1.P.f_val == v rhs.P.f_val);
    assert (r1.P.f_val == rhs.P.f_val)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd3_value_one
    (randomness: t_Slice u8{Seq.length randomness == 192})
    (c: i16) (k: nat{k < 256})
  : Lemma
    (requires cbd_coeff_3 randomness c k)
    (ensures
      v c >= -3 /\ v c <= 3 /\
      VTS.i16_to_spec_fe c ==
      Seq.index (HS.sample_poly_cbd (mk_usize 192) (mk_usize 1536) (mk_usize 3) randomness) k)
  = reveal_opaque (`%cbd_coeff_3) (cbd_coeff_3 randomness c k);
    let kk = mk_usize k in
    assert (k == v kk);
    assert (Seq.index (HS.sample_poly_cbd (mk_usize 192) (mk_usize 1536) (mk_usize 3) randomness) (v kk)
            == (let bits = S.bytes_to_bits (mk_usize 192) (mk_usize 1536) randomness in
                let x = HS.sum_coins (mk_usize 3)
                  (bits.[ { Core_models.Ops.Range.f_start = (mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize;
                            Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize) +! mk_usize 3 <: usize }
                          <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
                let y = HS.sum_coins (mk_usize 3)
                  (bits.[ { Core_models.Ops.Range.f_start = ((mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize) +! mk_usize 3 <: usize;
                            Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize) +! (mk_usize 2 *! mk_usize 3 <: usize) <: usize }
                          <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
                P.impl_FieldElement__new (((x.P.f_val +! P.v_FIELD_MODULUS <: u16) -! y.P.f_val <: u16) %!
                                          P.v_FIELD_MODULUS <: u16)))
      by (FStar.Tactics.norm [delta_only [`%HS.sample_poly_cbd]; zeta; iota; primops];
          FStar.Tactics.l_to_r [`P.createi_lemma];
          FStar.Tactics.trefl ());
    let bits = S.bytes_to_bits (mk_usize 192) (mk_usize 1536) randomness in
    let xs = (bits.[ { Core_models.Ops.Range.f_start = (mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize;
                       Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize) +! mk_usize 3 <: usize }
                     <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
    let ys = (bits.[ { Core_models.Ops.Range.f_start = ((mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize) +! mk_usize 3 <: usize;
                       Core_models.Ops.Range.f_end = ((mk_usize 2 *! kk <: usize) *! mk_usize 3 <: usize) +! (mk_usize 2 *! mk_usize 3 <: usize) <: usize }
                     <: Core_models.Ops.Range.t_Range usize ] <: t_Slice bool) in
    assert (xs == Seq.slice bits (6 * k) (6 * k + 3));
    assert (ys == Seq.slice bits (6 * k + 3) (6 * k + 6));
    Seq.lemma_index_slice bits (6 * k) (6 * k + 3) 0;
    Seq.lemma_index_slice bits (6 * k) (6 * k + 3) 1;
    Seq.lemma_index_slice bits (6 * k) (6 * k + 3) 2;
    Seq.lemma_index_slice bits (6 * k + 3) (6 * k + 6) 0;
    Seq.lemma_index_slice bits (6 * k + 3) (6 * k + 6) 1;
    Seq.lemma_index_slice bits (6 * k + 3) (6 * k + 6) 2;
    lemma_sum_coins_3 xs;
    lemma_sum_coins_3 ys;
    let x = HS.sum_coins (mk_usize 3) xs in
    let y = HS.sum_coins (mk_usize 3) ys in
    let xv : nat = v x.P.f_val in
    let yv : nat = v y.P.f_val in
    assert (xv <= 3 /\ yv <= 3);
    assert (v c == xv - yv);
    let rhs = P.impl_FieldElement__new (((x.P.f_val +! P.v_FIELD_MODULUS <: u16) -! y.P.f_val <: u16) %!
                                        P.v_FIELD_MODULUS <: u16) in
    assert (v rhs.P.f_val == (xv + 3329 - yv) % 3329);
    let r1 = VTS.i16_to_spec_fe c in
    assert (v r1.P.f_val == v c % 3329);
    ML.lemma_mod_plus (xv - yv) 1 3329;
    assert (v r1.P.f_val == v rhs.P.f_val);
    assert (r1.P.f_val == rhs.P.f_val)
#pop-options

(* ------------------------------------------------------------------ *)
(* 7. composer finalize                                                *)
(* ------------------------------------------------------------------ *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_is_bounded_poly_3_of_array
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (arr: t_Array i16 (mk_usize 256))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (k: nat). k < 256 ==> v (Seq.index arr k) >= -3 /\ v (Seq.index arr k) <= 3) /\
      (forall (i: nat). i < 16 ==>
        Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
          (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients i)
        == Seq.slice arr (16 * i) (16 * i + 16)))
    (ensures Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3) re)
  = reveal_opaque (`%Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly)
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3) re);
    let aux (i: nat{i < 16}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector (mk_usize 3)
        (re.Libcrux_ml_kem.Vector.f_coefficients.[ sz i ])) =
      let vec = re.Libcrux_ml_kem.Vector.f_coefficients.[ sz i ] in
      let a16 = Libcrux_ml_kem.Vector.Traits.f_to_i16_array #v_Vector vec in
      assert (a16 == Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector vec);
      reveal_opaque (`%VTS.is_i16b_array_opaque) (VTS.is_i16b_array_opaque 3 a16);
      let aux2 (l: nat) : Lemma ((l < 16) ==> VTS.is_i16b 3 (Seq.index a16 l)) =
        if l < 16 then Seq.lemma_index_slice arr (16 * i) (16 * i + 16) l
      in
      FStar.Classical.forall_intro aux2
    in
    FStar.Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd2_finalize
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (randomness: t_Slice u8{Seq.length randomness == 128})
    (arr: t_Array i16 (mk_usize 256))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (k: nat). k < 256 ==> cbd_coeff_2 randomness (Seq.index arr k) k) /\
      (forall (i: nat). i < 16 ==>
        Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
          (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients i)
        == Seq.slice arr (16 * i) (16 * i + 16)))
    (ensures
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3) re /\
      Libcrux_ml_kem.Vector.Spec.poly_to_spec #v_Vector re
      == HS.sample_poly_cbd (mk_usize 128) (mk_usize 1024) (mk_usize 2) randomness)
  = let spec = HS.sample_poly_cbd (mk_usize 128) (mk_usize 1024) (mk_usize 2) randomness in
    let lift = Libcrux_ml_kem.Vector.Spec.poly_to_spec #v_Vector re in
    assert (Seq.length lift == 256);
    assert (Seq.length spec == 256);
    let aux (k: nat{k < 256}) : Lemma
      (Seq.index lift k == Seq.index spec k /\
       v (Seq.index arr k) >= -3 /\ v (Seq.index arr k) <= 3) =
      assert (k / 16 < 16);
      Libcrux_ml_kem.Vector.Spec.poly_to_spec_index #v_Vector re k;
      Seq.lemma_index_slice arr (16 * (k / 16)) (16 * (k / 16) + 16) (k % 16);
      assert (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients (k / 16))) (k % 16)
              == Seq.index arr k);
      lemma_cbd2_value_one randomness (Seq.index arr k) k
    in
    FStar.Classical.forall_intro aux;
    Seq.lemma_eq_intro lift spec;
    lemma_is_bounded_poly_3_of_array #v_Vector arr re
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_cbd3_finalize
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (randomness: t_Slice u8{Seq.length randomness == 192})
    (arr: t_Array i16 (mk_usize 256))
    (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires
      (forall (k: nat). k < 256 ==> cbd_coeff_3 randomness (Seq.index arr k) k) /\
      (forall (i: nat). i < 16 ==>
        Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
          (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients i)
        == Seq.slice arr (16 * i) (16 * i + 16)))
    (ensures
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3) re /\
      Libcrux_ml_kem.Vector.Spec.poly_to_spec #v_Vector re
      == HS.sample_poly_cbd (mk_usize 192) (mk_usize 1536) (mk_usize 3) randomness)
  = let spec = HS.sample_poly_cbd (mk_usize 192) (mk_usize 1536) (mk_usize 3) randomness in
    let lift = Libcrux_ml_kem.Vector.Spec.poly_to_spec #v_Vector re in
    assert (Seq.length lift == 256);
    assert (Seq.length spec == 256);
    let aux (k: nat{k < 256}) : Lemma
      (Seq.index lift k == Seq.index spec k /\
       v (Seq.index arr k) >= -3 /\ v (Seq.index arr k) <= 3) =
      assert (k / 16 < 16);
      Libcrux_ml_kem.Vector.Spec.poly_to_spec_index #v_Vector re k;
      Seq.lemma_index_slice arr (16 * (k / 16)) (16 * (k / 16) + 16) (k % 16);
      assert (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients (k / 16))) (k % 16)
              == Seq.index arr k);
      lemma_cbd3_value_one randomness (Seq.index arr k) k
    in
    FStar.Classical.forall_intro aux;
    Seq.lemma_eq_intro lift spec;
    lemma_is_bounded_poly_3_of_array #v_Vector arr re
#pop-options
