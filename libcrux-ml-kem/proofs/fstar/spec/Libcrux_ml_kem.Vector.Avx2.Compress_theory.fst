module Libcrux_ml_kem.Vector.Avx2.Compress_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

(* Hand-written proof theory relocated from src/vector/avx2/compress.rs
   `hax_lib::fstar::before` blocks (byte-exact raw-string contents, verified verbatim
   against the green extracted module).  The mulhi_l_* / lemma_mulhi_mm256_epi32 /
   lemma_compress_half cluster stays in compress.rs: it references the module's own
   `mulhi_mm256_epi32` helper fn (F* module-level dep cycle otherwise, Error 308). *)

module Iavx = Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views

(* AGENT C2: closed via `lemma_mm256_xor_si256` axiom (sibling of
   `lemma_mm256_and_si256`).  Strengthens the per-lane xor characterization. *)
let lemma_mm256_xor_si256_lane (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 16 ==>
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                 (Libcrux_intrinsics.Avx2.mm256_xor_si256 lhs rhs)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) i ^.
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs) i))
  = Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_xor_si256 lhs rhs

(* AGENT C2: closed via `lemma_mm256_srli_epi16` axiom.  Specialises the
   per-lane logical right-shift characterization to SHIFT = 15 (sign bit
   extraction). *)
let lemma_mm256_srli_epi16_15 (vec: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 16 ==>
    v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                    (Libcrux_intrinsics.Avx2.mm256_srli_epi16 (mk_i32 15) vec)) i) ==
    (if v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vec) i) < 0
     then 1 else 0)))
  = Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_srli_epi16 (mk_i32 15) vec;
    let view = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vec in
    let view_shifted = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                         (Libcrux_intrinsics.Avx2.mm256_srli_epi16 (mk_i32 15) vec) in
    introduce forall (i: nat). i < 16 ==>
      v (Seq.index view_shifted i) ==
      (if v (Seq.index view i) < 0 then 1 else 0)
    with begin
      if i < 16 then begin
        let x = Seq.index view i in
        if v x < 0 then begin
          assert (v (cast x <: u16) == v x + pow2 16);
          assert (v ((cast x <: u16) >>! mk_i32 15) == (v x + pow2 16) / pow2 15);
          assert ((v x + pow2 16) / pow2 15 == 1)
        end else begin
          assert (v (cast x <: u16) == v x);
          assert (v ((cast x <: u16) >>! mk_i32 15) == v x / pow2 15);
          assert (v x / pow2 15 == 0)
        end
      end
    end

(* >>! 15 on i16 (arithmetic shift) is sign extension: -1 if negative, else 0 *)
let lemma_i16_arith_shr_15 (x: i16) : Lemma
  (ensures v (x >>! mk_i32 15) == (if v x < 0 then -1 else 0))
  [SMTPat (x >>! mk_i32 15)]
  = ()

(* xor of an i16 with all-ones (-1) is bitwise NOT, i.e. (-x - 1).
   xor with all-zeros is identity.  Proved via Rust_primitives.Integers
   logxor_lemma + lognot_lemma (covers a ^ ones == lognot a and
   v (lognot a) == -1 - v a on signed types). *)
let lemma_i16_xor_neg1 (x: i16) : Lemma
  (ensures v (x ^. mk_i16 (-1)) == -(v x) - 1)
  [SMTPat (x ^. mk_i16 (-1))]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 (-1));
    Rust_primitives.Integers.lognot_lemma x

let lemma_i16_xor_zero (x: i16) : Lemma
  (ensures v (x ^. mk_i16 0) == v x)
  [SMTPat (x ^. mk_i16 0)]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 0)

(* P1: per-lane conditional-not.  When the mask m is all-ones (v m = -1) it
   flips x to lognot x = -x-1; when all-zeros (v m = 0) it is identity.
   Mirror of the portable shifted_to_positive xor reasoning. *)
let lemma_xor_cond_not (m x: i16) : Lemma
  (requires v m == (if v x < 0 then -1 else 0))
  (ensures v (m ^. x) == (if v x < 0 then - (v x) - 1 else v x))
  = Rust_primitives.Integers.mk_int_v_lemma m;
    Rust_primitives.Integers.logxor_lemma x m;
    Rust_primitives.Integers.lognot_lemma x

(* P0: the compress-1 integer identity, a mirror of the portable
   compress_message_coefficient final case-split.  Pure arithmetic fact:
   for a field element vec_i in [0,3328], floor((vec_i*4+3329)/6658) is
   0 / 1 / 2 on the three ranges [0,832] / [833,2496] / [2497,3328], whose
   parities are 0 / 1 / 0 = (if 833<=vec_i<=2496 then 1 else 0). *)
#push-options "--z3rlimit 200"
let lemma_compress_message_identity (vec_i: int) : Lemma
  (requires vec_i >= 0 /\ vec_i < 3329)
  (ensures ((vec_i * 4 + 3329) / 6658) % 2 == (if 833 <= vec_i && vec_i <= 2496 then 1 else 0))
  = assert (vec_i < 833 ==> (vec_i * 4 + 3329) >= 3329 /\ (vec_i * 4 + 3329) < 6658);
    assert (vec_i < 833 ==> (vec_i * 4 + 3329) / 6658 == 0);
    assert (vec_i < 833 ==> ((vec_i * 4 + 3329) / 6658) % 2 == 0);
    assert ((vec_i >= 833 && vec_i <= 2496) ==> (vec_i * 4 + 3329) >= 6658 /\ (vec_i * 4 + 3329) < 13316);
    assert ((vec_i >= 833 && vec_i <= 2496) ==> (vec_i * 4 + 3329) / 6658 == 1);
    assert ((vec_i >= 833 && vec_i <= 2496) ==> ((vec_i * 4 + 3329) / 6658) % 2 == 1);
    assert (vec_i > 2496 ==> (vec_i * 4 + 3329) >= 13316 /\ (vec_i * 4 + 3329) < 19974);
    assert (vec_i > 2496 ==> (vec_i * 4 + 3329) / 6658 == 2);
    assert (vec_i > 2496 ==> ((vec_i * 4 + 3329) / 6658) % 2 == 0)
#pop-options

(* ───────────────────────────────────────────────────────────────────────
   mulhi composite lemma: lane j of `mulhi_mm256_epi32 lhs rhs` is the high
   32 bits of the unsigned 32x32 product of lane j of lhs and rhs.  Proven
   from the (validated) mul_epu32 / shuffle_epi32 / unpack{lo,hi}_epi32 /
   unpackhi_epi64 lane axioms — NOT itself an axiom.
   ─────────────────────────────────────────────────────────────────────── *)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_mulhi_hi32 (p: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (i: nat{i < 4}) (bigp: int)
  : Lemma
    (requires Libcrux_intrinsics.Avx2_ml_kem_views.lane64u p i == bigp /\ 0 <= bigp /\ bigp / 4294967296 < 2147483648)
    (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 p (2 * i + 1) == bigp / 4294967296)
  = let lo = Libcrux_intrinsics.Avx2_ml_kem_views.lane32 p (2 * i) in
    let hiv = Libcrux_intrinsics.Avx2_ml_kem_views.lane32 p (2 * i + 1) in
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane64u p i == (lo % 4294967296) + 4294967296 * (hiv % 4294967296));
    FStar.Math.Lemmas.lemma_div_plus (lo % 4294967296) (hiv % 4294967296) 4294967296;
    FStar.Math.Lemmas.lemma_div_mod hiv 4294967296

let lemma_shuffle245_even (vec: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (i: nat{i < 4})
  : Lemma (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) vec) (2 * i)
                   == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 vec (2 * i + 1))
  = match i with
    | 0 -> reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 245) 0)
    | 1 -> reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 245) 2)
    | 2 -> reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 245) 4)
    | _ -> reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 245) 6)
#pop-options

(* Ground per-lane fact lemmas, each isolating ONE intrinsic axiom (minimal
   context, ~ms each).  The mulhi assembly below cites these and EXCLUDES the
   quantified intrinsic posts (`--using_facts_from -...`) so the 7 coexisting
   lane-foralls (mul_epu32 ×2, shuffle ×2, unpack ×3) cannot cross-saturate. *)
(* mul_epu32: the `lane64u`/`lane32` unfolding + the unsigned-product bound is the
   heaviest of the per-lane ground facts (it was riding at 46.5/50 cold, i.e. on
   the edge, and tipped over once the companion's op-facts became proofs rather
   than admits).  Give it its own budget + a fresh solver rather than leaving a
   query that only passes on a lucky Z3 path. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 250"
#restart-solver
let mul_epu32_lane_nn (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (i: nat{i < 4})
  : Lemma (requires 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a (2 * i) /\ 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b (2 * i))
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane64u (Libcrux_intrinsics.Avx2.mm256_mul_epu32 a b) i ==
                   Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a (2 * i) * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b (2 * i)) = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let unpacklo_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 8})
  : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_unpacklo_epi32 a b) k ==
           (match k with | 0 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 0 | 1 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 0
            | 2 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 1 | 3 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 1 | 4 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 4
            | 5 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 4 | 6 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 5 | _ -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 5)) = ()
let unpackhi_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 8})
  : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_unpackhi_epi32 a b) k ==
           (match k with | 0 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 2 | 1 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 2
            | 2 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 3 | 3 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 3 | 4 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 6
            | 5 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 6 | 6 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 7 | _ -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 7)) = ()
let unpackhi64_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_unpackhi_epi64 a b) j ==
           (match j with | 0 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 2 | 1 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 3
            | 2 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 2 | 3 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 3 | 4 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 6
            | 5 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a 7 | 6 -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 6 | _ -> Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b 7)) = ()
#pop-options

(* ───────────────────────────────────────────────────────────────────────
   d-bit compress body: ground per-stage lane lemmas + a per-half spine lemma.
   ─────────────────────────────────────────────────────────────────────── *)

(* a non-negative small lane pins its two i16 lanes *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 60"
let lemma_lane32_to_i16 (a: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) (av: nat)
  : Lemma (requires Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a j == av /\ av < 32768)
          (ensures v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2 * j)) == av /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2 * j + 1)) == 0) = ()
#pop-options

(* @%-into-i32 is the identity on a non-negative value below 2^31 — proved on an
   ABSTRACT int so the nonlinear product it is applied to never enters the @% VC. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 40"
let lemma_atpercent_id (p: int)
  : Lemma (requires 0 <= p /\ p < 2147483648) (ensures p @% 4294967296 == p)
  = FStar.Math.Lemmas.small_mod p 4294967296
#pop-options

(* per-stage ground facts (one intrinsic axiom each) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let cvtepi_lane_nn (c0: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) (j: nat{j < 8})
  : Lemma (requires 0 <= v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) < 3329)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 c0) j == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j)) = ()
let slli_lane_nowrap (c1: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (cb: i32) (j: nat{j < 8})
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
                    0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j < 3329)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_slli_epi32 cb c1) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j * pow2 (v cb))
  = assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb);
    FStar.Math.Lemmas.lemma_mult_le_left (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j) (pow2 (v cb)) 2048;
    FStar.Math.Lemmas.lemma_mult_le_right 2048 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j) 3328;
    lemma_atpercent_id (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j * pow2 (v cb))
let add_lane_1664 (c2 fmh: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (requires Libcrux_intrinsics.Avx2_ml_kem_views.lane32 fmh j == 1664 /\ 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c2 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c2 j < 6815745)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_add_epi32 c2 fmh) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c2 j + 1664) = ()
let srli3_lane (c4: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (requires 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c4 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c4 j < 2147483648)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_srli_epi32 (mk_i32 3) c4) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c4 j / 8) = ()
#pop-options

(* AND with a broadcast (2^dd - 1) mask reduces a small non-negative lane mod 2^dd *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_and_mask_lane (c5 mask: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) (cval: nat) (dd: nat)
  : Lemma (requires 0 < dd /\ dd <= 11 /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c5 j == cval /\ cval < 32768 /\
                    Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mask (2 * j) == mk_i16 (pow2 dd - 1) /\
                    Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mask (2 * j + 1) == mk_i16 0)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_and_si256 c5 mask) j == cval % pow2 dd)
  = lemma_lane32_to_i16 c5 j cval;
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    Rust_primitives.Integers.logand_mask_lemma (mk_i16 cval) dd;
    assert (mk_i16 (pow2 dd - 1) ==
            Rust_primitives.Integers.sub #i16_inttype (mk_i16 (pow2 dd)) (mk_i16 1));
    Rust_primitives.Integers.logand_lemma (mk_i16 0) (mk_i16 0)
#pop-options

(* The per-half spine, symbolic in lane j: the cvtepi/slli/add/mulhi/srli/and
   chain computes the Barrett `((x*2^d+1664)*10321340)>>35 & (2^d-1)`.  Constant
   vectors fmh/cf/mask are passed in with their lane facts so the recomputed
   spine matches the body's exact `set1` expressions definitionally. *)
(* nonlinear bounds, proven in CLEAN context (no SIMD terms) so the heavy
   half-lemma below only consumes them as ground facts. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 60"
let lemma_compress_nn_bounds (xv dd: nat)
  : Lemma (requires xv < 3329 /\ (dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11))
          (ensures (let nn = xv * pow2 dd + 1664 in
                    nn <= 6817408 /\
                    (nn * 10321340) / pow2 32 < 2147483648 /\
                    (nn * 10321340) / pow2 35 < 32768))
  = assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    FStar.Math.Lemmas.lemma_mult_le_right (pow2 dd) xv 3328;
    FStar.Math.Lemmas.lemma_mult_le_left 3328 (pow2 dd) 2048;
    let nn = xv * pow2 dd + 1664 in
    FStar.Math.Lemmas.lemma_mult_le_right 10321340 nn 6817408;
    assert_norm (6817408 * 10321340 == 70364785886720);
    assert_norm (pow2 47 == 140737488355328);
    assert (nn * 10321340 < pow2 47);
    FStar.Math.Lemmas.lemma_div_lt (nn * 10321340) 47 32;
    FStar.Math.Lemmas.lemma_div_lt (nn * 10321340) 47 35;
    assert_norm (pow2 15 == 32768); assert_norm (pow2 12 == 4096);
    assert_norm (pow2 31 == 2147483648)
#pop-options

(* pack + permute<0b11_01_10_00=0xD8>: ground per-lane facts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let packs_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16})
  : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_packs_epi32 a b) k ==
           (if k < 4 then Libcrux_intrinsics.Avx2_ml_kem_views.sat_i16 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a k)
            else if k < 8 then Libcrux_intrinsics.Avx2_ml_kem_views.sat_i16 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b (k - 4))
            else if k < 12 then Libcrux_intrinsics.Avx2_ml_kem_views.sat_i16 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a (k - 4))
            else Libcrux_intrinsics.Avx2_ml_kem_views.sat_i16 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b (k - 8)))) = ()
let permute_lane_0xD8 (vec: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16})
  : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) vec) k ==
           Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vec (4 * Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216) (k / 4) + k % 4)) = ()
#pop-options

(* result.[i] of permute<0xD8>(packs cl6 ch6) == lane32 of cl6 (i<8) / ch6 (i-8),
   when those lanes are < 2^15 (so the i16 saturation is the identity). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 60 --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lane32 -Libcrux_intrinsics.Avx2.mm256_packs_epi32 -Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64'"
let lemma_result_lane (cl6 ch6: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (i: nat{i < 16})
  : Lemma
    (requires (forall (k: nat). k < 8 ==>
                 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 cl6 k /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 cl6 k < 32768 /\
                 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 ch6 k /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 ch6 k < 32768))
    (ensures v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216)
                                 (Libcrux_intrinsics.Avx2.mm256_packs_epi32 cl6 ch6)) i) ==
             (if i < 8 then Libcrux_intrinsics.Avx2_ml_kem_views.lane32 cl6 i else Libcrux_intrinsics.Avx2_ml_kem_views.lane32 ch6 (i - 8)))
  = let packed = Libcrux_intrinsics.Avx2.mm256_packs_epi32 cl6 ch6 in
    permute_lane_0xD8 packed i;
    let r0 = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216) 0) in
    let r1 = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216) 1) in
    let r2 = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216) 2) in
    let r3 = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216) 3) in
    (match i with
     | 0 -> packs_lane cl6 ch6 0  | 1 -> packs_lane cl6 ch6 1
     | 2 -> packs_lane cl6 ch6 2  | 3 -> packs_lane cl6 ch6 3
     | 4 -> packs_lane cl6 ch6 8  | 5 -> packs_lane cl6 ch6 9
     | 6 -> packs_lane cl6 ch6 10 | 7 -> packs_lane cl6 ch6 11
     | 8 -> packs_lane cl6 ch6 4  | 9 -> packs_lane cl6 ch6 5
     | 10 -> packs_lane cl6 ch6 6 | 11 -> packs_lane cl6 ch6 7
     | 12 -> packs_lane cl6 ch6 12 | 13 -> packs_lane cl6 ch6 13
     | 14 -> packs_lane cl6 ch6 14 | _ -> packs_lane cl6 ch6 15)
#pop-options

(* set1_epi32 lane facts: lane32 == the broadcast constant; for a < 2^16
   constant, the per-i16 decomposition (low = constant, high = 0). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80 --split_queries always"
let set1_lane32 (c: i32) (j: nat{j < 8})
  : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_set1_epi32 c) j == v c) = ()
let set1_mask_i16 (c: i32) (dd: nat{0 < dd /\ dd <= 11}) (j: nat{j < 8})
  : Lemma (requires v c == pow2 dd - 1)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_set1_epi32 c) (2 * j) == mk_i16 (pow2 dd - 1) /\
                   Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_set1_epi32 c) (2 * j + 1) == mk_i16 0)
  = assert_norm (pow2 11 == 2048); FStar.Math.Lemmas.pow2_le_compat 11 dd
(* the d-bit mask constant `(1 <<! cb) - 1` has value `2^d - 1` (clean context so
   the `@%.` in shift_left_positive_lemma evaluates). *)
let lemma_mask_val (cb: i32)
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11))
          (ensures v ((mk_i32 1 <<! cb <: i32) -! mk_i32 1) == pow2 (v cb) - 1)
  = assert_norm (pow2 11 == 2048); assert_norm (pow2 31 == 2147483648);
    assert_norm (pow2 32 == 4294967296);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb)
#pop-options

(* local copies of the cast bridges (Vector.Avx2.Ntt has the originals, but it
   is not a dependency of this module). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let compress_castsi256_lemma (vc: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256)
  : Lemma (ensures (forall (i: nat). i < 8 ==>
            Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vc)) i ==
            Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vc) i))
  = let aux (i: nat{i < 8})
      : Lemma (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vc)) i ==
               Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vc) i) =
      let a = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vc)) i in
      let b = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vc) i in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        let k : nat = 16 * i + nthv in
        assert (k < 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vc) 16 k;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma vc 16 k;
        assert (k / 16 == i); assert (k % 16 == nthv);
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_castsi256_si128 vc k
      in Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in Classical.forall_intro aux

let compress_extracti128_lemma (vc: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256)
  : Lemma (ensures (forall (i: nat). i < 8 ==>
            Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vc)) i ==
            Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vc) (i + 8)))
  = let aux (i: nat{i < 8})
      : Lemma (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vc)) i ==
               Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vc) (i + 8)) =
      let a = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vc)) i in
      let b = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vc) (i + 8) in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        FStar.Math.Lemmas.lemma_mult_le_left 16 (i + 8) 15;
        let k : nat = 16 * i + nthv in
        let k' : nat = 16 * (i + 8) + nthv in
        assert (k < 128); assert (k' < 256); assert (k' == k + 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        FStar.Math.Lemmas.lemma_div_plus nthv (i + 8) 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv (i + 8) 16;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vc) 16 k;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma vc 16 k';
        assert (k / 16 == i); assert (k % 16 == nthv);
        assert (k' / 16 == i + 8); assert (k' % 16 == nthv);
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_extracti128_si256_1 vc k
      in Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in Classical.forall_intro aux
#pop-options

(* ───────────────────────────────────────────────────────────────────────
   d-bit decompress body: ground per-stage lane lemmas + a per-half spine.
   EXACT division (no Barrett/mulhi) — the spine cvtepi/mullo/slli<1>/add/
   srli<d>/srli<1> computes `(2*x*3329 + 2^d) / (2^d*2)`, matching the scalar
   bridge `lemma_decompress_ciphertext_coefficient_fe_commute` directly.
   Reuses the compress helpers `cvtepi_lane_nn`, `lemma_atpercent_id`,
   `packs_lane`, `permute_lane_0xD8`, `lemma_result_lane`, `set1_lane32`,
   `compress_castsi256_lemma`, `compress_extracti128_lemma` (defined above).
   ─────────────────────────────────────────────────────────────────────── *)

(* nonlinear bounds, proven in CLEAN context (no SIMD terms). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 80"
let lemma_decompress_nn_bounds (xv dd: nat)
  : Lemma (requires xv < pow2 dd /\ (dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11))
          (ensures (xv < 2048 /\
                    xv * 3329 < 1073741824 /\
                    2 * xv * 3329 < 2147481600 /\
                    2 * xv * 3329 + pow2 dd < 2147483648 /\
                    (2 * xv * 3329 + pow2 dd) / pow2 dd <= 6657 /\
                    (2 * xv * 3329 + pow2 dd) / (pow2 dd * 2) < 3329))
  = assert_norm (pow2 4 == 16); assert_norm (pow2 5 == 32);
    assert_norm (pow2 10 == 1024); assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    FStar.Math.Lemmas.lemma_mult_le_right 3329 xv 2047;
    FStar.Math.Lemmas.lemma_mult_le_right 3329 xv (pow2 dd - 1);
    let d3 = 2 * xv * 3329 + pow2 dd in
    assert (d3 < 6658 * pow2 dd);
    FStar.Math.Lemmas.lemma_div_plus (-1) 6658 (pow2 dd);
    FStar.Math.Lemmas.lemma_div_le d3 (6658 * pow2 dd - 1) (pow2 dd);
    assert (d3 / pow2 dd <= 6657);
    FStar.Math.Lemmas.division_multiplication_lemma d3 (pow2 dd) 2;
    FStar.Math.Lemmas.lemma_div_le (d3 / pow2 dd) 6657 2
#pop-options

(* per-stage ground facts (one intrinsic axiom each). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let mullo_lane_nowrap (c1 fm: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (requires Libcrux_intrinsics.Avx2_ml_kem_views.lane32 fm j == 3329 /\ 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j < 2048)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_mullo_epi32 c1 fm) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j * 3329)
  = FStar.Math.Lemmas.lemma_mult_le_right 3329 (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j) 2047;
    lemma_atpercent_id (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c1 j * 3329)
let slli1_lane_nowrap (d1: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (requires 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j < 1073741824)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 1) d1) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j * 2)
  = assert_norm (pow2 1 == 2);
    lemma_atpercent_id (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j * 2)
let add_lane_2cb (d2 twocb: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) (pcb: nat)
  : Lemma (requires Libcrux_intrinsics.Avx2_ml_kem_views.lane32 twocb j == pcb /\ pcb <= 2048 /\
                    0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d2 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d2 j < 2147481600)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_add_epi32 d2 twocb) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d2 j + pcb)
  = lemma_atpercent_id (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d2 j + pcb)
let srli_d_lane (c: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (cb: i32) (j: nat{j < 8})
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
                    0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c j < 2147483648)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_srli_epi32 cb c) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c j / pow2 (v cb)) = ()
let srli1_lane (c: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma (requires 0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c j < 2147483648)
          (ensures Libcrux_intrinsics.Avx2_ml_kem_views.lane32 (Libcrux_intrinsics.Avx2.mm256_srli_epi32 (mk_i32 1) c) j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 c j / 2)
  = assert_norm (pow2 1 == 2)
#pop-options

(* the d-bit two_pow constant `1 <<! cb` has value `2^d`. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_twopow_val (cb: i32)
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11))
          (ensures v ((mk_i32 1 <<! cb) <: i32) == pow2 (v cb))
  = assert_norm (pow2 11 == 2048); assert_norm (pow2 31 == 2147483648);
    assert_norm (pow2 32 == 4294967296);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb)
#pop-options

(* per-half spine, symbolic in lane j.  Exclude lane32's DEFINITION + the chain
   intrinsic posts so the products stay atomic; all lane32 equalities supplied
   by the per-stage lemmas + the clean bounds helper. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Libcrux_intrinsics.Avx2_ml_kem_views.lane32 -Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 -Libcrux_intrinsics.Avx2.mm256_mullo_epi32 -Libcrux_intrinsics.Avx2.mm256_slli_epi32 -Libcrux_intrinsics.Avx2.mm256_add_epi32 -Libcrux_intrinsics.Avx2.mm256_srli_epi32 -Libcrux_intrinsics.Avx2.mm256_set1_epi32'"
let lemma_decompress_half (c0: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) (cb: i32) (fm twocb: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8})
  : Lemma
    (requires
       (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
       0 <= v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) < pow2 (v cb) /\
       Libcrux_intrinsics.Avx2_ml_kem_views.lane32 fm j == 3329 /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 twocb j == pow2 (v cb))
    (ensures
       (let c1 = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 c0 in
        let d1 = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 c1 fm in
        let d2 = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 1) d1 in
        let d3 = Libcrux_intrinsics.Avx2.mm256_add_epi32 d2 twocb in
        let d4 = Libcrux_intrinsics.Avx2.mm256_srli_epi32 cb d3 in
        let d5 = Libcrux_intrinsics.Avx2.mm256_srli_epi32 (mk_i32 1) d4 in
        let xv = v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) in
        let dd = v cb in
        0 <= Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d5 j /\ Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d5 j < 3329 /\
        Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d5 j == (2 * xv * 3329 + pow2 dd) / (pow2 dd * 2)))
  = let dd = v cb in
    let xv = v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 c0 j) in
    let c1 = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 c0 in
    let d1 = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 c1 fm in
    let d2 = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 1) d1 in
    let d3 = Libcrux_intrinsics.Avx2.mm256_add_epi32 d2 twocb in
    let d4 = Libcrux_intrinsics.Avx2.mm256_srli_epi32 cb d3 in
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    lemma_decompress_nn_bounds xv dd;
    cvtepi_lane_nn c0 j;
    mullo_lane_nowrap c1 fm j;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j == xv * 3329);
    slli1_lane_nowrap d1 j;
    (* hand Z3 the (xv*3329)*2 == 2*xv*3329 reassociation explicitly: without it
       Z3 reports "unknown because unknown" on this nonlinear step under
       the fact-filter, even though the stage lemma supplies the equality. *)
    FStar.Math.Lemmas.paren_mul_right 2 xv 3329;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d2 j ==
            Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j * 2);
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d1 j * 2 == (xv * 3329) * 2);
    (* and the commutation too: under the fact-filter the products are ATOMS, so
       Z3 does not get `(xv*3329)*2 == 2*(xv*3329)` for free — without this the
       step below saturates (rlimit 100.000, canceled). *)
    FStar.Math.Lemmas.swap_mul (xv * 3329) 2;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d2 j == 2 * xv * 3329);
    add_lane_2cb d2 twocb j (pow2 dd);
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d3 j == 2 * xv * 3329 + pow2 dd);
    srli_d_lane d3 cb j;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 d4 j == (2 * xv * 3329 + pow2 dd) / pow2 dd);
    srli1_lane d4 j;
    FStar.Math.Lemmas.division_multiplication_lemma (2 * xv * 3329 + pow2 dd) (pow2 dd) 2
#pop-options
