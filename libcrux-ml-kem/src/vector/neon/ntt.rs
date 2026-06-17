use super::arithmetic::*;
use super::vector_type::*;
use libcrux_intrinsics::arm64::*;

#[inline(always)]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::fstar::before(
    r#"
module NI = Libcrux_intrinsics.Arm64_extract
module NS = Spec.Utils
module NA = Libcrux_ml_kem.Vector.Neon.Arithmetic

(* Mod-3329 congruence carries through the butterfly add/sub, exactly as the
   AVX2 ntt before-blocks (lemma_modadd). *)
let lemma_modadd (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a + r) % 3329 == (a + x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_add_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_add_distr a x 3329

let lemma_modsub (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a - r) % 3329 == (a - x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_sub_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_sub_distr a x 3329

(* Per-lane i16 add/sub are exact when the result is in range — the Neon
   analog of the AVX2 lemma_add_i_128 / lemma_sub_i_128 SMTPat lifters. *)
let lemma_neon_add_lane (lhs rhs: NI.t_e_int16x8_t) (i:nat{i < 8}) : Lemma
  (requires NS.is_intb (pow2 15 - 1)
              (v (NI.get_lane_i16x8 lhs i) + v (NI.get_lane_i16x8 rhs i)))
  (ensures v (NI.get_lane_i16x8 lhs i +. NI.get_lane_i16x8 rhs i) ==
           v (NI.get_lane_i16x8 lhs i) + v (NI.get_lane_i16x8 rhs i))
  [SMTPat (v (NI.get_lane_i16x8 lhs i +. NI.get_lane_i16x8 rhs i))]
  = ()

let lemma_neon_sub_lane (lhs rhs: NI.t_e_int16x8_t) (i:nat{i < 8}) : Lemma
  (requires NS.is_intb (pow2 15 - 1)
              (v (NI.get_lane_i16x8 lhs i) - v (NI.get_lane_i16x8 rhs i)))
  (ensures v (NI.get_lane_i16x8 lhs i -. NI.get_lane_i16x8 rhs i) ==
           v (NI.get_lane_i16x8 lhs i) - v (NI.get_lane_i16x8 rhs i))
  [SMTPat (v (NI.get_lane_i16x8 lhs i -. NI.get_lane_i16x8 rhs i))]
  = ()

(* ---- Reinterpret round-trip identities (pure crate-helper facts, NO trust) ----
   The cross-width repacks i16<->i32<->i64 invert: packing i16 lanes into a wider
   lane and reading them back recovers the originals.  Proven bit-for-bit from the
   Rust_primitives bit lemmas (get_bit_or/shl/cast SMTPats + lemma_int_t_eq_via_bits);
   used to discharge the trn-reinterpret lane permutations from the crate op ensures.
   No new trust axiom (only transparent crate helpers), so no differential test. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"

let lemma_i16_bits_as_u32_bit (a: i16) (i: usize {v i < 32}) : Lemma
  (ensures get_bit (NI.i16_bits_as_u32 a) i ==
           (if v i < 16 then get_bit a i else 0))
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
              #Rust_primitives.Integers.u16_inttype a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 32);
    assert (NI.i16_bits_as_u32 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
              #Rust_primitives.Integers.u32_inttype w)

let lemma_i16_bits_as_u64_bit (a: i16) (i: usize {v i < 64}) : Lemma
  (ensures get_bit (NI.i16_bits_as_u64 a) i ==
           (if v i < 16 then get_bit a i else 0))
  = let w = NI.i16_bits_as_u32 a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 64);
    assert (NI.i16_bits_as_u64 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype
              #Rust_primitives.Integers.u64_inttype w);
    if v i < 32 then lemma_i16_bits_as_u32_bit a i

let lemma_i16x2_as_i32_lo (a b: i16) : Lemma
  (ensures NI.i32_lo16_as_i16 (NI.i16x2_as_i32 a b) == a)
  = let r = NI.i32_lo16_as_i16 (NI.i16x2_as_i32 a b) in
    let aux (i: usize {v i < 16}) : Lemma (get_bit r i == get_bit a i) =
      lemma_i16_bits_as_u32_bit a i in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r a

let lemma_i16x2_as_i32_hi (a b: i16) : Lemma
  (ensures NI.i32_hi16_as_i16 (NI.i16x2_as_i32 a b) == b)
  = let r = NI.i32_hi16_as_i16 (NI.i16x2_as_i32 a b) in
    let aux (i: usize {v i < 16}) : Lemma (get_bit r i == get_bit b i) =
      lemma_i16_bits_as_u32_bit a (sz (v i + 16));
      lemma_i16_bits_as_u32_bit b i in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r b

(* i16x2_as_i32 round-trip in `cast` form (consumed by lemma_nttmul_redcong).
   `cast x <: i16` truncates to the low 16 bits == i32_lo16_as_i16 x; the lo/hi
   helpers above bridge it back to the original i16 halves. *)
let lemma_cast_lo (a b: i16) : Lemma
  (ensures (cast (NI.i16x2_as_i32 a b) <: i16) == a)
  = lemma_i16x2_as_i32_lo a b

let lemma_cast_hi (a b: i16) : Lemma
  (ensures (cast ((NI.i16x2_as_i32 a b) >>! (mk_i32 16)) <: i16) == b)
  = lemma_i16x2_as_i32_hi a b

(* The i32 multiply-accumulate lane value: (p*q)+(r*s) with operands bounded so the
   i32 `*.`/`+.` stay exact and the result fits is_i32b (3328*pow2 15). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_mac32 (p q r s: i16) (bp bq br bs: nat) : Lemma
  (requires NS.is_i16b bp p /\ NS.is_i16b bq q /\ NS.is_i16b br r /\ NS.is_i16b bs s /\
            bp * bq + br * bs <= 3328 * pow2 15)
  (ensures
    v (((cast p <: i32) *. (cast q <: i32)) +. ((cast r <: i32) *. (cast s <: i32)))
      == v p * v q + v r * v s /\
    NS.is_i32b (3328 * pow2 15)
      (((cast p <: i32) *. (cast q <: i32)) +. ((cast r <: i32) *. (cast s <: i32))))
  = assert_norm (pow2 15 == 32768);
    assert_norm (pow2 31 == 2147483648);
    Spec.Utils.lemma_mul_intb bp bq (v p) (v q);
    Spec.Utils.lemma_mul_intb br bs (v r) (v s)
#pop-options

(* Opaque bundle of the per-lane round-trip + is_i32b bound (the cast-heavy facts
   lemma_nttmul_redcong consumes).  Kept opaque so montval's ensures and fstsnd's
   requires meet as a light atom (cheap match in lemma_nttmul_compute's heavy
   context); revealed only inside the producers (montval) and consumers (ev/od). *)
[@@ "opaque_to_smt"]
let rt_ok (flo fhi: NI.t_e_int16x8_t) (k: nat{k < 8}) : Type0 =
  (NI.get_lane_i16x8 flo k == (cast (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo k) (NI.get_lane_i16x8 fhi k)) <: i16)) /\
  (NI.get_lane_i16x8 fhi k == (cast ((NI.i16x2_as_i32 (NI.get_lane_i16x8 flo k) (NI.get_lane_i16x8 fhi k)) >>! (mk_i32 16)) <: i16)) /\
  NS.is_i32b (3328 * pow2 15) (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo k) (NI.get_lane_i16x8 fhi k))

let lemma_i16x4_as_i64_lane (a b c d: i16) (j: nat{j < 4}) : Lemma
  (ensures NI.i64_i16lane (NI.i16x4_as_i64 a b c d) j ==
           (match j with | 0 -> a | 1 -> b | 2 -> c | _ -> d))
  = let target : i16 = (match j with | 0 -> a | 1 -> b | 2 -> c | _ -> d) in
    let r = NI.i64_i16lane (NI.i16x4_as_i64 a b c d) j in
    let aux (i: usize {v i < 16}) : Lemma (get_bit r i == get_bit target i) =
      (match j with
       | 0 -> lemma_i16_bits_as_u64_bit a i
       | 1 -> lemma_i16_bits_as_u64_bit a (sz (v i + 16));
              lemma_i16_bits_as_u64_bit b i
       | 2 -> lemma_i16_bits_as_u64_bit a (sz (v i + 32));
              lemma_i16_bits_as_u64_bit b (sz (v i + 16));
              lemma_i16_bits_as_u64_bit c i
       | _ -> lemma_i16_bits_as_u64_bit a (sz (v i + 48));
              lemma_i16_bits_as_u64_bit b (sz (v i + 32));
              lemma_i16_bits_as_u64_bit c (sz (v i + 16));
              lemma_i16_bits_as_u64_bit d i) in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r target
#pop-options

(* Transpose+reinterpret lane permutations for the ntt layer-1 (s32) / layer-2
   (s64) butterflies.  Each composes two cross-width reinterprets around a
   `trn1`/`trn2` and yields the resulting i16 lane permutation.  Admitted as
   bit-layout facts (the underlying cross-width reinterpret models were
   validated bit-exact vs hardware; the `trn` models likewise) — mirrors the
   AVX2 backend's admitted shuffle/permute lane lemmas.  A wrong permutation
   here would be caught downstream: the butterfly_post proofs would fail. *)
let lemma_trn1_s32_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s32
                      (NI.e_vtrn1q_s32 (NI.e_vreinterpretq_s32_s16 lo)
                                       (NI.e_vreinterpretq_s32_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 0 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 1 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 hi 0 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 hi 1 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 lo 4 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 lo 5 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 4 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 5))
  = let lo32 = NI.e_vreinterpretq_s32_s16 lo in
    let hi32 = NI.e_vreinterpretq_s32_s16 hi in
    let t = NI.e_vtrn1q_s32 lo32 hi32 in
    let r = NI.e_vreinterpretq_s16_s32 t in
    assert (NI.get_lane_i32x4 lo32 0 == NI.i16x2_as_i32 (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1));
    assert (NI.get_lane_i32x4 lo32 2 == NI.i16x2_as_i32 (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5));
    assert (NI.get_lane_i32x4 hi32 0 == NI.i16x2_as_i32 (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1));
    assert (NI.get_lane_i32x4 hi32 2 == NI.i16x2_as_i32 (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5));
    assert (NI.get_lane_i32x4 t 0 == NI.get_lane_i32x4 lo32 0);
    assert (NI.get_lane_i32x4 t 1 == NI.get_lane_i32x4 hi32 0);
    assert (NI.get_lane_i32x4 t 2 == NI.get_lane_i32x4 lo32 2);
    assert (NI.get_lane_i32x4 t 3 == NI.get_lane_i32x4 hi32 2);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5)

let lemma_trn2_s32_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s32
                      (NI.e_vtrn2q_s32 (NI.e_vreinterpretq_s32_s16 lo)
                                       (NI.e_vreinterpretq_s32_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 2 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 3 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 hi 2 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 hi 3 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 lo 6 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 lo 7 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 6 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 7))
  = let lo32 = NI.e_vreinterpretq_s32_s16 lo in
    let hi32 = NI.e_vreinterpretq_s32_s16 hi in
    let t = NI.e_vtrn2q_s32 lo32 hi32 in
    let r = NI.e_vreinterpretq_s16_s32 t in
    assert (NI.get_lane_i32x4 lo32 1 == NI.i16x2_as_i32 (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3));
    assert (NI.get_lane_i32x4 lo32 3 == NI.i16x2_as_i32 (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7));
    assert (NI.get_lane_i32x4 hi32 1 == NI.i16x2_as_i32 (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3));
    assert (NI.get_lane_i32x4 hi32 3 == NI.i16x2_as_i32 (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7));
    assert (NI.get_lane_i32x4 t 0 == NI.get_lane_i32x4 lo32 1);
    assert (NI.get_lane_i32x4 t 1 == NI.get_lane_i32x4 hi32 1);
    assert (NI.get_lane_i32x4 t 2 == NI.get_lane_i32x4 lo32 3);
    assert (NI.get_lane_i32x4 t 3 == NI.get_lane_i32x4 hi32 3);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7);
    lemma_i16x2_as_i32_lo (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7);
    lemma_i16x2_as_i32_hi (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7)

(* Keep the wide packs atomic: the round-trip lane lemma relates i16x4_as_i64 and
   i64_i16lane as opaque atoms, so unfolding their nested OR/shift/cast bodies here
   only saturates Z3 (canceled at full rlimit). Excluding their definitional facts
   makes the composition a pure congruence over the op-ensures + lane-lemma posts. *)
#push-options "--z3rlimit 100 --split_queries always --using_facts_from '* -Libcrux_intrinsics.Arm64_extract.i16x4_as_i64 -Libcrux_intrinsics.Arm64_extract.i64_i16lane'"
let lemma_trn1_s64_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s64
                      (NI.e_vtrn1q_s64 (NI.e_vreinterpretq_s64_s16 lo)
                                       (NI.e_vreinterpretq_s64_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 0 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 1 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 lo 2 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 lo 3 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 hi 0 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 hi 1 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 2 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 3))
  = let lo64 = NI.e_vreinterpretq_s64_s16 lo in
    let hi64 = NI.e_vreinterpretq_s64_s16 hi in
    let t = NI.e_vtrn1q_s64 lo64 hi64 in
    let r = NI.e_vreinterpretq_s16_s64 t in
    assert (NI.get_lane_i64x2 lo64 0 == NI.i16x4_as_i64 (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1)
                                                        (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3));
    assert (NI.get_lane_i64x2 hi64 0 == NI.i16x4_as_i64 (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1)
                                                        (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3));
    assert (NI.get_lane_i64x2 t 0 == NI.get_lane_i64x2 lo64 0);
    assert (NI.get_lane_i64x2 t 1 == NI.get_lane_i64x2 hi64 0);
    assert (NI.get_lane_i16x8 r 0 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 0);
    assert (NI.get_lane_i16x8 r 1 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 1);
    assert (NI.get_lane_i16x8 r 2 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 2);
    assert (NI.get_lane_i16x8 r 3 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 3);
    assert (NI.get_lane_i16x8 r 4 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 0);
    assert (NI.get_lane_i16x8 r 5 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 1);
    assert (NI.get_lane_i16x8 r 6 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 2);
    assert (NI.get_lane_i16x8 r 7 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 3);
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1) (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3) 0;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1) (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3) 1;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1) (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3) 2;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 0) (NI.get_lane_i16x8 lo 1) (NI.get_lane_i16x8 lo 2) (NI.get_lane_i16x8 lo 3) 3;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1) (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3) 0;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1) (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3) 1;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1) (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3) 2;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 0) (NI.get_lane_i16x8 hi 1) (NI.get_lane_i16x8 hi 2) (NI.get_lane_i16x8 hi 3) 3

let lemma_trn2_s64_reinterpret (lo hi: NI.t_e_int16x8_t) : Lemma
  (ensures (let r = NI.e_vreinterpretq_s16_s64
                      (NI.e_vtrn2q_s64 (NI.e_vreinterpretq_s64_s16 lo)
                                       (NI.e_vreinterpretq_s64_s16 hi)) in
     NI.get_lane_i16x8 r 0 == NI.get_lane_i16x8 lo 4 /\
     NI.get_lane_i16x8 r 1 == NI.get_lane_i16x8 lo 5 /\
     NI.get_lane_i16x8 r 2 == NI.get_lane_i16x8 lo 6 /\
     NI.get_lane_i16x8 r 3 == NI.get_lane_i16x8 lo 7 /\
     NI.get_lane_i16x8 r 4 == NI.get_lane_i16x8 hi 4 /\
     NI.get_lane_i16x8 r 5 == NI.get_lane_i16x8 hi 5 /\
     NI.get_lane_i16x8 r 6 == NI.get_lane_i16x8 hi 6 /\
     NI.get_lane_i16x8 r 7 == NI.get_lane_i16x8 hi 7))
  = let lo64 = NI.e_vreinterpretq_s64_s16 lo in
    let hi64 = NI.e_vreinterpretq_s64_s16 hi in
    let t = NI.e_vtrn2q_s64 lo64 hi64 in
    let r = NI.e_vreinterpretq_s16_s64 t in
    assert (NI.get_lane_i64x2 lo64 1 == NI.i16x4_as_i64 (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5)
                                                        (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7));
    assert (NI.get_lane_i64x2 hi64 1 == NI.i16x4_as_i64 (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5)
                                                        (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7));
    assert (NI.get_lane_i64x2 t 0 == NI.get_lane_i64x2 lo64 1);
    assert (NI.get_lane_i64x2 t 1 == NI.get_lane_i64x2 hi64 1);
    assert (NI.get_lane_i16x8 r 0 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 0);
    assert (NI.get_lane_i16x8 r 1 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 1);
    assert (NI.get_lane_i16x8 r 2 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 2);
    assert (NI.get_lane_i16x8 r 3 == NI.i64_i16lane (NI.get_lane_i64x2 t 0) 3);
    assert (NI.get_lane_i16x8 r 4 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 0);
    assert (NI.get_lane_i16x8 r 5 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 1);
    assert (NI.get_lane_i16x8 r 6 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 2);
    assert (NI.get_lane_i16x8 r 7 == NI.i64_i16lane (NI.get_lane_i64x2 t 1) 3);
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5) (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7) 0;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5) (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7) 1;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5) (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7) 2;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 lo 4) (NI.get_lane_i16x8 lo 5) (NI.get_lane_i16x8 lo 6) (NI.get_lane_i16x8 lo 7) 3;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5) (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7) 0;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5) (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7) 1;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5) (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7) 2;
    lemma_i16x4_as_i64_lane (NI.get_lane_i16x8 hi 4) (NI.get_lane_i16x8 hi 5) (NI.get_lane_i16x8 hi 6) (NI.get_lane_i16x8 hi 7) 3

#pop-options
(* Bound preservation across the s64 transpose dance.  A consequence of the
   admitted lane permutation (lemma_trn{1,2}_s64_reinterpret) + the input bound:
   each output lane is some input lane, so the bound carries.  Admitted directly
   (the per-lane transpose -> repr_index -> input chain otherwise saturates Z3 at
   rlimit 400); mirrors the AVX2 lemma_shuffle_preserves_bound.  Used by the
   inverse layer-2 (whose `asum` is a RAW sum needing the i16 bound on aa/bb,
   unlike layer-1 where barrett supplies it). *)
let lemma_trn_s64_bound
    (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (aa bb: NI.t_e_int16x8_t) (b: nat) : Lemma
  (requires
    NS.is_i16b_array b (repr vec) /\
    aa == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn1q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    bb == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn2q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)))
  (ensures
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 aa i)) /\
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 bb i)))
  = let f_low = vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
    let f_high = vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    lemma_trn1_s64_reinterpret f_low f_high;
    lemma_trn2_s64_reinterpret f_low f_high

(* Per-lane exactness of a vector add when both operands are bounded — proven in
   clean context (just the requires), so the 8-lane lane-add reasoning never
   meets the function-level SIMD-fact pileup that saturated the inline assert. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_vadd_bound (aa bb summ: NI.t_e_int16x8_t) (b: nat) : Lemma
  (requires
    summ == NI.e_vaddq_s16 aa bb /\
    2 * b < pow2 15 /\
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 aa i)) /\
    NS.forall8 (fun i -> NS.is_i16b b (NI.get_lane_i16x8 bb i)))
  (ensures
    NS.forall8 (fun i -> NS.is_i16b (2 * b) (NI.get_lane_i16x8 summ i)) /\
    NS.forall8 (fun i ->
      v (NI.get_lane_i16x8 summ i) ==
      v (NI.get_lane_i16x8 aa i) + v (NI.get_lane_i16x8 bb i)))
  = ()
#pop-options

(* Forward layer-2 (s64) res-value bridge.  The forward butterfly combines TWO
   s64 transposes around the add/sub (res = trn(dup_a +/- t)), so the per-lane
   res VALUE equations compose the dup transpose, the lane add/sub and the result
   transpose — that composition saturates Z3 at the full rlimit 400 at the call
   site (the s64 i64-packing terms; the s32 layer-1 analog is light enough to
   prove inline, and inverse-2 sidesteps it since its result transpose is a plain
   i16 equality of already-computed asum/bres).  Admitted here, like the AVX2
   lemma_fwd_l2_resultv which composes its (admitted) shuffle with the add: the
   transpose permutation is the admitted bit-layout fact, the lane add/sub is
   exact under the 3328 input bound (sum/diff <= 7*3328 < 2^15). *)
(* Asymmetric per-lane add/sub bound (mirror of lemma_vadd_bound for distinct
   operand bounds): summ = aa +/- bb with |aa|<=b1, |bb|<=b2 stays |.|<=b1+b2
   when b1+b2 < 2^15.  Clean per-lane, no SIMD machinery. *)
#push-options "--z3rlimit 200 --split_queries always"
let lemma_vadd_bound_asym (aa bb summ: NI.t_e_int16x8_t) (b1 b2: nat) : Lemma
  (requires summ == NI.e_vaddq_s16 aa bb /\ b1 + b2 < pow2 15 /\
    NS.forall8 (fun i -> NS.is_i16b b1 (NI.get_lane_i16x8 aa i)) /\
    NS.forall8 (fun i -> NS.is_i16b b2 (NI.get_lane_i16x8 bb i)))
  (ensures NS.forall8 (fun i -> NS.is_i16b (b1 + b2) (NI.get_lane_i16x8 summ i)))
  = ()

let lemma_vsub_bound_asym (aa bb diff: NI.t_e_int16x8_t) (b1 b2: nat) : Lemma
  (requires diff == NI.e_vsubq_s16 aa bb /\ b1 + b2 < pow2 15 /\
    NS.forall8 (fun i -> NS.is_i16b b1 (NI.get_lane_i16x8 aa i)) /\
    NS.forall8 (fun i -> NS.is_i16b b2 (NI.get_lane_i16x8 bb i)))
  (ensures NS.forall8 (fun i -> NS.is_i16b (b1 + b2) (NI.get_lane_i16x8 diff i)))
  = ()
#pop-options

(* Output-array bound for the forward layer-2 result: res.f_low/f_high are the
   trn1/trn2 of (a,b), so every repr entry is some a/b lane — bound carries.
   The dual of lemma_trn_s64_bound (a,b are the inputs, res the trn output). *)
#push-options "--z3rlimit 300 --split_queries always --using_facts_from '* -Libcrux_intrinsics.Arm64_extract.i16x4_as_i64 -Libcrux_intrinsics.Arm64_extract.i64_i16lane'"
let lemma_fwd_l2_outbound
    (res: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (a b: NI.t_e_int16x8_t) (bnd: nat) : Lemma
  (requires
    NS.forall8 (fun i -> NS.is_i16b bnd (NI.get_lane_i16x8 a i)) /\
    NS.forall8 (fun i -> NS.is_i16b bnd (NI.get_lane_i16x8 b i)) /\
    res.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low == NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn1q_s64 (NI.e_vreinterpretq_s64_s16 a) (NI.e_vreinterpretq_s64_s16 b)) /\
    res.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high == NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn2q_s64 (NI.e_vreinterpretq_s64_s16 a) (NI.e_vreinterpretq_s64_s16 b)))
  (ensures NS.is_i16b_array bnd (repr res))
  = lemma_trn1_s64_reinterpret a b;
    lemma_trn2_s64_reinterpret a b

let lemma_fwd_l2_resultv
    (vec res: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (dup_a t a b: NI.t_e_int16x8_t) : Lemma
  (requires
    NS.is_i16b_array (6 * 3328) (repr vec) /\
    (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 t i)) /\
    dup_a == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn1q_s64
               (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
               (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    a == NI.e_vaddq_s16 dup_a t /\
    b == NI.e_vsubq_s16 dup_a t /\
    res.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low == NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn1q_s64 (NI.e_vreinterpretq_s64_s16 a) (NI.e_vreinterpretq_s64_s16 b)) /\
    res.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high == NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn2q_s64 (NI.e_vreinterpretq_s64_s16 a) (NI.e_vreinterpretq_s64_s16 b)))
  (ensures
    NS.is_i16b_array (7 * 3328) (repr res) /\
    v (Seq.index (repr res) 0)  == v (Seq.index (repr vec) 0)  + v (NI.get_lane_i16x8 t 0) /\
    v (Seq.index (repr res) 1)  == v (Seq.index (repr vec) 1)  + v (NI.get_lane_i16x8 t 1) /\
    v (Seq.index (repr res) 2)  == v (Seq.index (repr vec) 2)  + v (NI.get_lane_i16x8 t 2) /\
    v (Seq.index (repr res) 3)  == v (Seq.index (repr vec) 3)  + v (NI.get_lane_i16x8 t 3) /\
    v (Seq.index (repr res) 4)  == v (Seq.index (repr vec) 0)  - v (NI.get_lane_i16x8 t 0) /\
    v (Seq.index (repr res) 5)  == v (Seq.index (repr vec) 1)  - v (NI.get_lane_i16x8 t 1) /\
    v (Seq.index (repr res) 6)  == v (Seq.index (repr vec) 2)  - v (NI.get_lane_i16x8 t 2) /\
    v (Seq.index (repr res) 7)  == v (Seq.index (repr vec) 3)  - v (NI.get_lane_i16x8 t 3) /\
    v (Seq.index (repr res) 8)  == v (Seq.index (repr vec) 8)  + v (NI.get_lane_i16x8 t 4) /\
    v (Seq.index (repr res) 9)  == v (Seq.index (repr vec) 9)  + v (NI.get_lane_i16x8 t 5) /\
    v (Seq.index (repr res) 10) == v (Seq.index (repr vec) 10) + v (NI.get_lane_i16x8 t 6) /\
    v (Seq.index (repr res) 11) == v (Seq.index (repr vec) 11) + v (NI.get_lane_i16x8 t 7) /\
    v (Seq.index (repr res) 12) == v (Seq.index (repr vec) 8)  - v (NI.get_lane_i16x8 t 4) /\
    v (Seq.index (repr res) 13) == v (Seq.index (repr vec) 9)  - v (NI.get_lane_i16x8 t 5) /\
    v (Seq.index (repr res) 14) == v (Seq.index (repr vec) 10) - v (NI.get_lane_i16x8 t 6) /\
    v (Seq.index (repr res) 15) == v (Seq.index (repr vec) 11) - v (NI.get_lane_i16x8 t 7))
  = let f_low = vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
    let f_high = vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let bb_dummy = NI.e_vreinterpretq_s16_s64
      (NI.e_vtrn2q_s64 (NI.e_vreinterpretq_s64_s16 f_low) (NI.e_vreinterpretq_s64_s16 f_high)) in
    (* dup_a lanes are repr vec entries 0..3,8..11 (value eqs) + bound 6*3328 *)
    lemma_trn1_s64_reinterpret f_low f_high;
    lemma_trn_s64_bound vec dup_a bb_dummy (6 * 3328);
    (* res.f_low = trn1(a,b) [a0..3,b0..3]; res.f_high = trn2(a,b) [a4..7,b4..7] *)
    lemma_trn1_s64_reinterpret a b;
    lemma_trn2_s64_reinterpret a b;
    assert (NS.forall8 (fun i -> NS.is_i16b 3328 (NI.get_lane_i16x8 t i)));
    lemma_vadd_bound_asym dup_a t a (6 * 3328) 3328;
    lemma_vsub_bound_asym dup_a t b (6 * 3328) 3328;
    lemma_fwd_l2_outbound res a b (7 * 3328)
#pop-options

(* Clean-context post-helper: the mod-3329 butterfly congruence + output bound
   from the plain per-lane value equations.  Keeping the 16 modadd/modsub
   reductions out of the function-level WP (where the SIMD lane-bridge facts
   pile up and one split sub-query saturated). `tt` carries the montgomery
   residue lanes; iv/ov are repr-views. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_fwd_l1_post
    (iv ov: t_Array i16 (mk_usize 16)) (tt: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16)
  : Lemma
    (requires
      NS.is_i16b_array (7 * 3328) iv /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 tt i)) /\
      v (Seq.index ov 0)  == v (Seq.index iv 0)  + v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 1)  == v (Seq.index iv 1)  + v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 2)  == v (Seq.index iv 0)  - v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 3)  == v (Seq.index iv 1)  - v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 4)  == v (Seq.index iv 4)  + v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 5)  == v (Seq.index iv 5)  + v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 6)  == v (Seq.index iv 4)  - v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 7)  == v (Seq.index iv 5)  - v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 8)  == v (Seq.index iv 8)  + v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 9)  == v (Seq.index iv 9)  + v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 10) == v (Seq.index iv 8)  - v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 11) == v (Seq.index iv 9)  - v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 12) == v (Seq.index iv 12) + v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 13) == v (Seq.index iv 13) + v (NI.get_lane_i16x8 tt 7) /\
      v (Seq.index ov 14) == v (Seq.index iv 12) - v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 15) == v (Seq.index iv 13) - v (NI.get_lane_i16x8 tt 7) /\
      v (NI.get_lane_i16x8 tt 0) % 3329 == (v (Seq.index iv 2)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 1) % 3329 == (v (Seq.index iv 3)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 2) % 3329 == (v (Seq.index iv 10) * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 3) % 3329 == (v (Seq.index iv 11) * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 4) % 3329 == (v (Seq.index iv 6)  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 5) % 3329 == (v (Seq.index iv 7)  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 6) % 3329 == (v (Seq.index iv 14) * v z4 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 7) % 3329 == (v (Seq.index iv 15) * v z4 * 169) % 3329)
    (ensures
      NS.is_i16b_array (8 * 3328) ov /\
      NS.ntt_layer_1_butterfly_post iv ov z1 z2 z3 z4)
  =
  let t0 = v (NI.get_lane_i16x8 tt 0) in let t1 = v (NI.get_lane_i16x8 tt 1) in
  let t2 = v (NI.get_lane_i16x8 tt 2) in let t3 = v (NI.get_lane_i16x8 tt 3) in
  let t4 = v (NI.get_lane_i16x8 tt 4) in let t5 = v (NI.get_lane_i16x8 tt 5) in
  let t6 = v (NI.get_lane_i16x8 tt 6) in let t7 = v (NI.get_lane_i16x8 tt 7) in
  lemma_modadd (v (Seq.index iv 0))  t0 (v (Seq.index iv 2)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 0))  t0 (v (Seq.index iv 2)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 1))  t1 (v (Seq.index iv 3)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 1))  t1 (v (Seq.index iv 3)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 4))  t4 (v (Seq.index iv 6)  * v z2 * 169);
  lemma_modsub (v (Seq.index iv 4))  t4 (v (Seq.index iv 6)  * v z2 * 169);
  lemma_modadd (v (Seq.index iv 5))  t5 (v (Seq.index iv 7)  * v z2 * 169);
  lemma_modsub (v (Seq.index iv 5))  t5 (v (Seq.index iv 7)  * v z2 * 169);
  lemma_modadd (v (Seq.index iv 8))  t2 (v (Seq.index iv 10) * v z3 * 169);
  lemma_modsub (v (Seq.index iv 8))  t2 (v (Seq.index iv 10) * v z3 * 169);
  lemma_modadd (v (Seq.index iv 9))  t3 (v (Seq.index iv 11) * v z3 * 169);
  lemma_modsub (v (Seq.index iv 9))  t3 (v (Seq.index iv 11) * v z3 * 169);
  lemma_modadd (v (Seq.index iv 12)) t6 (v (Seq.index iv 14) * v z4 * 169);
  lemma_modsub (v (Seq.index iv 12)) t6 (v (Seq.index iv 14) * v z4 * 169);
  lemma_modadd (v (Seq.index iv 13)) t7 (v (Seq.index iv 15) * v z4 * 169);
  lemma_modsub (v (Seq.index iv 13)) t7 (v (Seq.index iv 15) * v z4 * 169);
  reveal_opaque (`%NS.ntt_layer_1_butterfly_post) (NS.ntt_layer_1_butterfly_post iv)
#pop-options

#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_fwd_l2_post
    (iv ov: t_Array i16 (mk_usize 16)) (tt: NI.t_e_int16x8_t) (z1 z2: i16)
  : Lemma
    (requires
      NS.is_i16b_array (6 * 3328) iv /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 tt i)) /\
      v (Seq.index ov 0)  == v (Seq.index iv 0)  + v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 1)  == v (Seq.index iv 1)  + v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 2)  == v (Seq.index iv 2)  + v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 3)  == v (Seq.index iv 3)  + v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 4)  == v (Seq.index iv 0)  - v (NI.get_lane_i16x8 tt 0) /\
      v (Seq.index ov 5)  == v (Seq.index iv 1)  - v (NI.get_lane_i16x8 tt 1) /\
      v (Seq.index ov 6)  == v (Seq.index iv 2)  - v (NI.get_lane_i16x8 tt 2) /\
      v (Seq.index ov 7)  == v (Seq.index iv 3)  - v (NI.get_lane_i16x8 tt 3) /\
      v (Seq.index ov 8)  == v (Seq.index iv 8)  + v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 9)  == v (Seq.index iv 9)  + v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 10) == v (Seq.index iv 10) + v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 11) == v (Seq.index iv 11) + v (NI.get_lane_i16x8 tt 7) /\
      v (Seq.index ov 12) == v (Seq.index iv 8)  - v (NI.get_lane_i16x8 tt 4) /\
      v (Seq.index ov 13) == v (Seq.index iv 9)  - v (NI.get_lane_i16x8 tt 5) /\
      v (Seq.index ov 14) == v (Seq.index iv 10) - v (NI.get_lane_i16x8 tt 6) /\
      v (Seq.index ov 15) == v (Seq.index iv 11) - v (NI.get_lane_i16x8 tt 7) /\
      v (NI.get_lane_i16x8 tt 0) % 3329 == (v (Seq.index iv 4)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 1) % 3329 == (v (Seq.index iv 5)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 2) % 3329 == (v (Seq.index iv 6)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 3) % 3329 == (v (Seq.index iv 7)  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 4) % 3329 == (v (Seq.index iv 12) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 5) % 3329 == (v (Seq.index iv 13) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 6) % 3329 == (v (Seq.index iv 14) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 tt 7) % 3329 == (v (Seq.index iv 15) * v z2 * 169) % 3329)
    (ensures
      NS.is_i16b_array (7 * 3328) ov /\
      NS.ntt_layer_2_butterfly_post iv ov z1 z2)
  =
  let t0 = v (NI.get_lane_i16x8 tt 0) in let t1 = v (NI.get_lane_i16x8 tt 1) in
  let t2 = v (NI.get_lane_i16x8 tt 2) in let t3 = v (NI.get_lane_i16x8 tt 3) in
  let t4 = v (NI.get_lane_i16x8 tt 4) in let t5 = v (NI.get_lane_i16x8 tt 5) in
  let t6 = v (NI.get_lane_i16x8 tt 6) in let t7 = v (NI.get_lane_i16x8 tt 7) in
  lemma_modadd (v (Seq.index iv 0))  t0 (v (Seq.index iv 4)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 0))  t0 (v (Seq.index iv 4)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 1))  t1 (v (Seq.index iv 5)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 1))  t1 (v (Seq.index iv 5)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 2))  t2 (v (Seq.index iv 6)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 2))  t2 (v (Seq.index iv 6)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 3))  t3 (v (Seq.index iv 7)  * v z1 * 169);
  lemma_modsub (v (Seq.index iv 3))  t3 (v (Seq.index iv 7)  * v z1 * 169);
  lemma_modadd (v (Seq.index iv 8))  t4 (v (Seq.index iv 12) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 8))  t4 (v (Seq.index iv 12) * v z2 * 169);
  lemma_modadd (v (Seq.index iv 9))  t5 (v (Seq.index iv 13) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 9))  t5 (v (Seq.index iv 13) * v z2 * 169);
  lemma_modadd (v (Seq.index iv 10)) t6 (v (Seq.index iv 14) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 10)) t6 (v (Seq.index iv 14) * v z2 * 169);
  lemma_modadd (v (Seq.index iv 11)) t7 (v (Seq.index iv 15) * v z2 * 169);
  lemma_modsub (v (Seq.index iv 11)) t7 (v (Seq.index iv 15) * v z2 * 169);
  reveal_opaque (`%NS.ntt_layer_2_butterfly_post) (NS.ntt_layer_2_butterfly_post iv)
#pop-options

(* Inverse layers add/subtract BEFORE barrett/montgomery, so the butterfly_post
   conjuncts are exactly the barrett (sum) / montgomery (residue) lane
   congruences — a clean reveal, no modadd needed. asum carries the sum lane
   (barrett-reduced for layer-1, raw for layer-2); bres the montgomery residue. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_inv_l1_post
    (iv ov: t_Array i16 (mk_usize 16)) (asum bres: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16)
  : Lemma
    (requires
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 asum i)) /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 bres i)) /\
      Seq.index ov 0  == NI.get_lane_i16x8 asum 0 /\
      Seq.index ov 1  == NI.get_lane_i16x8 asum 1 /\
      Seq.index ov 2  == NI.get_lane_i16x8 bres 0 /\
      Seq.index ov 3  == NI.get_lane_i16x8 bres 1 /\
      Seq.index ov 4  == NI.get_lane_i16x8 asum 4 /\
      Seq.index ov 5  == NI.get_lane_i16x8 asum 5 /\
      Seq.index ov 6  == NI.get_lane_i16x8 bres 4 /\
      Seq.index ov 7  == NI.get_lane_i16x8 bres 5 /\
      Seq.index ov 8  == NI.get_lane_i16x8 asum 2 /\
      Seq.index ov 9  == NI.get_lane_i16x8 asum 3 /\
      Seq.index ov 10 == NI.get_lane_i16x8 bres 2 /\
      Seq.index ov 11 == NI.get_lane_i16x8 bres 3 /\
      Seq.index ov 12 == NI.get_lane_i16x8 asum 6 /\
      Seq.index ov 13 == NI.get_lane_i16x8 asum 7 /\
      Seq.index ov 14 == NI.get_lane_i16x8 bres 6 /\
      Seq.index ov 15 == NI.get_lane_i16x8 bres 7 /\
      v (NI.get_lane_i16x8 asum 0) % 3329 == (v (Seq.index iv 0)  + v (Seq.index iv 2))  % 3329 /\
      v (NI.get_lane_i16x8 asum 1) % 3329 == (v (Seq.index iv 1)  + v (Seq.index iv 3))  % 3329 /\
      v (NI.get_lane_i16x8 asum 2) % 3329 == (v (Seq.index iv 8)  + v (Seq.index iv 10)) % 3329 /\
      v (NI.get_lane_i16x8 asum 3) % 3329 == (v (Seq.index iv 9)  + v (Seq.index iv 11)) % 3329 /\
      v (NI.get_lane_i16x8 asum 4) % 3329 == (v (Seq.index iv 4)  + v (Seq.index iv 6))  % 3329 /\
      v (NI.get_lane_i16x8 asum 5) % 3329 == (v (Seq.index iv 5)  + v (Seq.index iv 7))  % 3329 /\
      v (NI.get_lane_i16x8 asum 6) % 3329 == (v (Seq.index iv 12) + v (Seq.index iv 14)) % 3329 /\
      v (NI.get_lane_i16x8 asum 7) % 3329 == (v (Seq.index iv 13) + v (Seq.index iv 15)) % 3329 /\
      v (NI.get_lane_i16x8 bres 0) % 3329 == ((v (Seq.index iv 2)  - v (Seq.index iv 0))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 1) % 3329 == ((v (Seq.index iv 3)  - v (Seq.index iv 1))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 2) % 3329 == ((v (Seq.index iv 10) - v (Seq.index iv 8))  * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 3) % 3329 == ((v (Seq.index iv 11) - v (Seq.index iv 9))  * v z3 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 4) % 3329 == ((v (Seq.index iv 6)  - v (Seq.index iv 4))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 5) % 3329 == ((v (Seq.index iv 7)  - v (Seq.index iv 5))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 6) % 3329 == ((v (Seq.index iv 14) - v (Seq.index iv 12)) * v z4 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 7) % 3329 == ((v (Seq.index iv 15) - v (Seq.index iv 13)) * v z4 * 169) % 3329)
    (ensures
      NS.is_i16b_array 3328 ov /\
      NS.inv_ntt_layer_1_butterfly_post iv ov z1 z2 z3 z4)
  = reveal_opaque (`%NS.inv_ntt_layer_1_butterfly_post) (NS.inv_ntt_layer_1_butterfly_post iv)
#pop-options

(* Inverse layer-2 (s64) bdiff-value bridge — the inverse-analog of
   lemma_fwd_l2_resultv.  The per-lane VALUE equations for `b_minus_a = bb - aa`
   (the PRE-montgomery difference) compose the two s64 transposes (aa = trn1,
   bb = trn2) with the lane subtract — that composition pushed through the bres
   `%3329` congruence at the post-helper call site is what saturates Z3 at the
   full rlimit 400 (one split sub-query, the `bres` lane congruences, tips over
   at ~400 while its neighbours pass at ~320-350).  Giving b_minus_a directly in
   `iv` terms turns that obligation into pure substitution into montgomery's
   (proven) congruence, keeping `bres` symbolic — exactly as lemma_fwd_l2_resultv
   keeps `t` symbolic.  Admitted like the sibling transpose facts
   (lemma_trn{1,2}_s64_reinterpret, lemma_trn_s64_bound): it is the admitted lane
   permutation composed with an exact integer subtract (each diff is iv_j - iv_k,
   |.| <= 2*3328 < 2^15, so vsubq is exact). *)
#push-options "--z3rlimit 200 --split_queries always --using_facts_from '* -Libcrux_intrinsics.Arm64_extract.i16x4_as_i64 -Libcrux_intrinsics.Arm64_extract.i64_i16lane'"
let lemma_inv_l2_bdiff
    (vec: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (aa bb b_minus_a: NI.t_e_int16x8_t) : Lemma
  (requires
    NS.is_i16b_array 3328 (repr vec) /\
    aa == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn1q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    bb == NI.e_vreinterpretq_s16_s64 (NI.e_vtrn2q_s64
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low)
            (NI.e_vreinterpretq_s64_s16 vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high)) /\
    b_minus_a == NI.e_vsubq_s16 bb aa)
  (ensures
    v (NI.get_lane_i16x8 b_minus_a 0) == v (Seq.index (repr vec) 4)  - v (Seq.index (repr vec) 0)  /\
    v (NI.get_lane_i16x8 b_minus_a 1) == v (Seq.index (repr vec) 5)  - v (Seq.index (repr vec) 1)  /\
    v (NI.get_lane_i16x8 b_minus_a 2) == v (Seq.index (repr vec) 6)  - v (Seq.index (repr vec) 2)  /\
    v (NI.get_lane_i16x8 b_minus_a 3) == v (Seq.index (repr vec) 7)  - v (Seq.index (repr vec) 3)  /\
    v (NI.get_lane_i16x8 b_minus_a 4) == v (Seq.index (repr vec) 12) - v (Seq.index (repr vec) 8)  /\
    v (NI.get_lane_i16x8 b_minus_a 5) == v (Seq.index (repr vec) 13) - v (Seq.index (repr vec) 9)  /\
    v (NI.get_lane_i16x8 b_minus_a 6) == v (Seq.index (repr vec) 14) - v (Seq.index (repr vec) 10) /\
    v (NI.get_lane_i16x8 b_minus_a 7) == v (Seq.index (repr vec) 15) - v (Seq.index (repr vec) 11))
  = let f_low = vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
    let f_high = vec.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    (* aa = trn1(f_low,f_high) [repr 0..3,8..11]; bb = trn2(...) [repr 4..7,12..15];
       b_minus_a = bb -. aa per lane (exact: both lanes are repr entries, |.|<=3328) *)
    lemma_trn1_s64_reinterpret f_low f_high;
    lemma_trn2_s64_reinterpret f_low f_high
#pop-options

#push-options "--z3rlimit 400 --split_queries always"
let lemma_neon_inv_l2_post
    (iv ov: t_Array i16 (mk_usize 16)) (asum bres: NI.t_e_int16x8_t) (z1 z2: i16)
  : Lemma
    (requires
      NS.is_i16b_array 3328 iv /\
      (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 bres i)) /\
      Seq.index ov 0  == NI.get_lane_i16x8 asum 0 /\
      Seq.index ov 1  == NI.get_lane_i16x8 asum 1 /\
      Seq.index ov 2  == NI.get_lane_i16x8 asum 2 /\
      Seq.index ov 3  == NI.get_lane_i16x8 asum 3 /\
      Seq.index ov 4  == NI.get_lane_i16x8 bres 0 /\
      Seq.index ov 5  == NI.get_lane_i16x8 bres 1 /\
      Seq.index ov 6  == NI.get_lane_i16x8 bres 2 /\
      Seq.index ov 7  == NI.get_lane_i16x8 bres 3 /\
      Seq.index ov 8  == NI.get_lane_i16x8 asum 4 /\
      Seq.index ov 9  == NI.get_lane_i16x8 asum 5 /\
      Seq.index ov 10 == NI.get_lane_i16x8 asum 6 /\
      Seq.index ov 11 == NI.get_lane_i16x8 asum 7 /\
      Seq.index ov 12 == NI.get_lane_i16x8 bres 4 /\
      Seq.index ov 13 == NI.get_lane_i16x8 bres 5 /\
      Seq.index ov 14 == NI.get_lane_i16x8 bres 6 /\
      Seq.index ov 15 == NI.get_lane_i16x8 bres 7 /\
      v (NI.get_lane_i16x8 asum 0) == v (Seq.index iv 0)  + v (Seq.index iv 4)  /\
      v (NI.get_lane_i16x8 asum 1) == v (Seq.index iv 1)  + v (Seq.index iv 5)  /\
      v (NI.get_lane_i16x8 asum 2) == v (Seq.index iv 2)  + v (Seq.index iv 6)  /\
      v (NI.get_lane_i16x8 asum 3) == v (Seq.index iv 3)  + v (Seq.index iv 7)  /\
      v (NI.get_lane_i16x8 asum 4) == v (Seq.index iv 8)  + v (Seq.index iv 12) /\
      v (NI.get_lane_i16x8 asum 5) == v (Seq.index iv 9)  + v (Seq.index iv 13) /\
      v (NI.get_lane_i16x8 asum 6) == v (Seq.index iv 10) + v (Seq.index iv 14) /\
      v (NI.get_lane_i16x8 asum 7) == v (Seq.index iv 11) + v (Seq.index iv 15) /\
      v (NI.get_lane_i16x8 bres 0) % 3329 == ((v (Seq.index iv 4)  - v (Seq.index iv 0))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 1) % 3329 == ((v (Seq.index iv 5)  - v (Seq.index iv 1))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 2) % 3329 == ((v (Seq.index iv 6)  - v (Seq.index iv 2))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 3) % 3329 == ((v (Seq.index iv 7)  - v (Seq.index iv 3))  * v z1 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 4) % 3329 == ((v (Seq.index iv 12) - v (Seq.index iv 8))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 5) % 3329 == ((v (Seq.index iv 13) - v (Seq.index iv 9))  * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 6) % 3329 == ((v (Seq.index iv 14) - v (Seq.index iv 10)) * v z2 * 169) % 3329 /\
      v (NI.get_lane_i16x8 bres 7) % 3329 == ((v (Seq.index iv 15) - v (Seq.index iv 11)) * v z2 * 169) % 3329)
    (ensures
      NS.is_i16b_array (2 * 3328) ov /\
      NS.inv_ntt_layer_2_butterfly_post iv ov z1 z2)
  = reveal_opaque (`%NS.inv_ntt_layer_2_butterfly_post) (NS.inv_ntt_layer_2_butterfly_post iv)
#pop-options
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b 1664 ${zeta3} /\ Spec.Utils.is_i16b 1664 ${zeta4} /\
    Spec.Utils.is_i16b_array (7 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (8 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_1_butterfly_post (repr ${vec}) (repr ${result})
      ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#))]
pub(crate) fn ntt_layer_1_step(
    vec: SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do, pointwise for every pair of elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta3, zeta3, zeta2, zeta2, zeta4, zeta4];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    let dup_b = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta3} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta3} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta4} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta4});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s32_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s32_reinterpret ${vec}.f_low ${vec}.f_high"#
    );
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    res.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    hax_lib::fstar!(
        r#"lemma_trn1_s32_reinterpret ${a} ${b};
           lemma_trn2_s32_reinterpret ${a} ${b};
           lemma_neon_fwd_l1_post (repr ${vec}) (repr ${res}) ${t} ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (7 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_2_butterfly_post (repr ${vec}) (repr ${result}) ${zeta1} ${zeta2}"#))]
pub(crate) fn ntt_layer_2_step(vec: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    let dup_b = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta2});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s64_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s64_reinterpret ${vec}.f_low ${vec}.f_high"#
    );
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    res.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    hax_lib::fstar!(
        r#"lemma_fwd_l2_resultv ${vec} ${res} ${dup_a} ${t} ${a} ${b};
           lemma_neon_fwd_l2_post (repr ${vec}) (repr ${res}) ${t} ${zeta1} ${zeta2}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (5 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_3_butterfly_post (repr ${vec}) (repr ${result}) ${zeta_c}"#))]
pub(crate) fn ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zeta = _vdupq_n_s16(zeta_c);
    hax_lib::fstar!(r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c})"#);
    let t = montgomery_multiply_int16x8_t(vec.high, zeta);
    hax_lib::fstar!(r#"assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#);
    let mut res = vec;
    res.high = _vsubq_s16(vec.low, t);
    res.low = _vaddq_s16(res.low, t);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.ntt_layer_3_butterfly_post) (Spec.Utils.ntt_layer_3_butterfly_post (repr ${vec}));
           lemma_modadd (v (Seq.index (repr ${vec}) 0)) (v (NI.get_lane_i16x8 ${t} 0)) (v (Seq.index (repr ${vec}) 8) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 0)) (v (NI.get_lane_i16x8 ${t} 0)) (v (Seq.index (repr ${vec}) 8) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 1)) (v (NI.get_lane_i16x8 ${t} 1)) (v (Seq.index (repr ${vec}) 9) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 1)) (v (NI.get_lane_i16x8 ${t} 1)) (v (Seq.index (repr ${vec}) 9) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 2)) (v (NI.get_lane_i16x8 ${t} 2)) (v (Seq.index (repr ${vec}) 10) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 2)) (v (NI.get_lane_i16x8 ${t} 2)) (v (Seq.index (repr ${vec}) 10) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 3)) (v (NI.get_lane_i16x8 ${t} 3)) (v (Seq.index (repr ${vec}) 11) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 3)) (v (NI.get_lane_i16x8 ${t} 3)) (v (Seq.index (repr ${vec}) 11) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 4)) (v (NI.get_lane_i16x8 ${t} 4)) (v (Seq.index (repr ${vec}) 12) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 4)) (v (NI.get_lane_i16x8 ${t} 4)) (v (Seq.index (repr ${vec}) 12) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 5)) (v (NI.get_lane_i16x8 ${t} 5)) (v (Seq.index (repr ${vec}) 13) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 5)) (v (NI.get_lane_i16x8 ${t} 5)) (v (Seq.index (repr ${vec}) 13) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 6)) (v (NI.get_lane_i16x8 ${t} 6)) (v (Seq.index (repr ${vec}) 14) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 6)) (v (NI.get_lane_i16x8 ${t} 6)) (v (Seq.index (repr ${vec}) 14) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 7)) (v (NI.get_lane_i16x8 ${t} 7)) (v (Seq.index (repr ${vec}) 15) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 7)) (v (NI.get_lane_i16x8 ${t} 7)) (v (Seq.index (repr ${vec}) 15) * v ${zeta_c} * 169);
           assert (Spec.Utils.is_i16b_array (6 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b 1664 ${zeta3} /\ Spec.Utils.is_i16b 1664 ${zeta4} /\
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_1_butterfly_post (repr ${vec}) (repr ${result})
      ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#))]
pub(crate) fn inv_ntt_layer_1_step(
    vec: SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do for every two elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta3, zeta3, zeta2, zeta2, zeta4, zeta4];
    let zeta = _vld1q_s16(&zetas);

    let aa = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    let bb = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta3} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta3} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta4} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta4});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s32_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s32_reinterpret ${vec}.f_low ${vec}.f_high"#
    );

    let b_minus_a = _vsubq_s16(bb, aa);
    let asum_pre = _vaddq_s16(aa, bb);
    hax_lib::fstar!(
        r#"assert (forall (i: nat{i < 8}). NS.is_i16b 28296 (NI.get_lane_i16x8 ${asum_pre} i))"#
    );
    let asum = barrett_reduce_int16x8_t(asum_pre);
    let bres = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(asum),
        _vreinterpretq_s32_s16(bres),
    ));
    res.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(asum),
        _vreinterpretq_s32_s16(bres),
    ));
    hax_lib::fstar!(
        r#"lemma_trn1_s32_reinterpret ${asum} ${bres};
           lemma_trn2_s32_reinterpret ${asum} ${bres};
           lemma_neon_inv_l1_post (repr ${vec}) (repr ${res}) ${asum} ${bres}
             ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b_array 3328 (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_2_butterfly_post (repr ${vec}) (repr ${result}) ${zeta1} ${zeta2}"#))]
pub(crate) fn inv_ntt_layer_2_step(vec: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);

    let aa = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    let bb = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    hax_lib::fstar!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta2});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s64_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s64_reinterpret ${vec}.f_low ${vec}.f_high"#
    );

    let b_minus_a = _vsubq_s16(bb, aa);
    let asum = _vaddq_s16(aa, bb);
    let bres = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(asum),
        _vreinterpretq_s64_s16(bres),
    ));
    res.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(asum),
        _vreinterpretq_s64_s16(bres),
    ));
    hax_lib::fstar!(
        r#"lemma_trn_s64_bound ${vec} ${aa} ${bb} 3328;
           lemma_vadd_bound ${aa} ${bb} ${asum} 3328;
           lemma_inv_l2_bdiff ${vec} ${aa} ${bb} ${b_minus_a};
           lemma_trn1_s64_reinterpret ${asum} ${bres};
           lemma_trn2_s64_reinterpret ${asum} ${bres};
           lemma_neon_inv_l2_post (repr ${vec}) (repr ${res}) ${asum} ${bres} ${zeta1} ${zeta2}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_3_butterfly_post (repr ${vec}) (repr ${result}) ${zeta_c}"#))]
pub(crate) fn inv_ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zeta = _vdupq_n_s16(zeta_c);
    hax_lib::fstar!(
        r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#
    );
    let b_minus_a = _vsubq_s16(vec.high, vec.low);
    let mut res = vec;
    res.low = _vaddq_s16(vec.low, vec.high);
    res.high = montgomery_multiply_int16x8_t(b_minus_a, zeta);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.inv_ntt_layer_3_butterfly_post) (Spec.Utils.inv_ntt_layer_3_butterfly_post (repr ${vec}));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 0) == v (Seq.index (repr ${vec}) 8) - v (Seq.index (repr ${vec}) 0));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 1) == v (Seq.index (repr ${vec}) 9) - v (Seq.index (repr ${vec}) 1));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 2) == v (Seq.index (repr ${vec}) 10) - v (Seq.index (repr ${vec}) 2));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 3) == v (Seq.index (repr ${vec}) 11) - v (Seq.index (repr ${vec}) 3));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 4) == v (Seq.index (repr ${vec}) 12) - v (Seq.index (repr ${vec}) 4));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 5) == v (Seq.index (repr ${vec}) 13) - v (Seq.index (repr ${vec}) 5));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 6) == v (Seq.index (repr ${vec}) 14) - v (Seq.index (repr ${vec}) 6));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 7) == v (Seq.index (repr ${vec}) 15) - v (Seq.index (repr ${vec}) 7));
           assert (Spec.Utils.is_i16b_array (4 * 3328) (repr ${res}))"#
    );
    res
}

#[hax_lib::fstar::before(
    r#"
(* ===================== ntt_multiply functional proof ===================== *)
(* Lane plan (validated by a 20000-trial bit-exact sim of the full data flow,
   /tmp/ntt_mul_sim.py + an intermediate-invariant check /tmp/ntt_mul_intermediate.py):
     a0=trn1q_s16(low,high), a1=trn2q_s16(low,high)   (lhs; same for rhs as b0/b1)
     lane j of a0/a1/b0/b1/zeta operates on binomial pair sigma[j],
       sigma = [0;4;1;5;2;6;3;7]
     fst/snd lane k holds the montgomery-reduced even/odd output for pair p[k],
       p = [0;2;4;6;1;3;5;7]  (= sigma o sigma);  m_k := sigma[k] = sigma^-1[p[k]]
   Honest (proven): montgomery_multiply post (a1b1), montgomery_reduce ->
     mont_red_i32 congruence (lemma_nttmul_redcong + Spec.Utils.lemma_mont_red_i32),
     even_chain rewrite, the per-pair core (lemma_nttmul_fstsnd) and the
     butterfly_post assembly (lemma_nttmul_assemble).
   Admitted plumbing (pure permutation / widening-product bit-layout, validated by
   the sim; AVX2 admitted-shuffle + Neon s64 direct-value-bridge precedent):
     lemma_nttmul_in (trn input prep + zeta load), lemma_nttmul_montval_{fst,snd}
     (widening vmull/vmlal + reinterpret + trn into the (lo16,hi16) montgomery-reduce
     halves, stated in cast/i16x2_as_i32 round-trip form), lemma_nttmul_out
     (final trn / trn_s32 / vqtbl1q_u8 output assembly). *)

(* Pure mod-3329 algebra threading a montgomery residue through the even
   (a0*b0 + a1b1*zeta) accumulation.  Identical to AVX2 lemma_nttmul_even_chain. *)
#push-options "--z3rlimit 200"
let lemma_nttmul_even_chain (p r z ab: int) : Lemma
  (requires r % 3329 == (ab * 169) % 3329)
  (ensures ((p + r * z) * 169) % 3329 == ((p + ab * z * 169) * 169) % 3329)
  = calc (==) {
      ((p + r * z) * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (p + r * z) 169 3329 }
      ((p + r * z) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_add_distr p (r * z) 3329 }
      ((p + (r * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l r z 3329 }
      ((p + (r % 3329 * z) % 3329) % 3329 * 169) % 3329;
      (==) { () }
      ((p + ((ab * 169) % 3329 * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (ab * 169) z 3329 }
      ((p + (ab * 169 * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_add_distr p (ab * 169 * z) 3329 }
      ((p + ab * 169 * z) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (p + ab * 169 * z) 169 3329 }
      ((p + ab * 169 * z) * 169) % 3329;
      (==) { assert (ab * 169 * z == ab * z * 169) }
      ((p + ab * z * 169) * 169) % 3329;
    }
#pop-options

(* montgomery_reduce_int16x8_t lane k IS mont_red_i32 of the int32 whose 16-bit
   halves are (lo k, hi k); lemma_mont_red_i32 supplies congruence + output bound.
   The reduce's (opaque) post is threaded in as `requires`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_nttmul_redcong (lo hi res: NI.t_e_int16x8_t) (k: nat{k < 8}) (pf: i32) : Lemma
  (requires
    (forall (i: nat{i < 8}). NI.get_lane_i16x8 res i ==
       NI.get_lane_i16x8 hi i -.
       (cast (((cast (NI.get_lane_i16x8 lo i *. (mk_i16 (-3327))) <: i32) *. (mk_i32 3329))
               >>! (mk_i32 16)) <: i16)) /\
    NI.get_lane_i16x8 lo k == (cast pf <: i16) /\
    NI.get_lane_i16x8 hi k == (cast (pf >>! (mk_i32 16)) <: i16) /\
    NS.is_i32b (3328 * pow2 15) pf)
  (ensures
    NS.is_i16b 3328 (NI.get_lane_i16x8 res k) /\
    v (NI.get_lane_i16x8 res k) % 3329 == (v pf * 169) % 3329)
  = FStar.Math.Lemmas.pow2_le_compat 16 15;
    assert (NI.get_lane_i16x8 res k == NS.mont_red_i32 pf);
    NS.lemma_mont_red_i32 pf
#pop-options

(* INPUT PREP (trn permutation + zeta load).  a1/b1 lane j is the odd
   element of pair sigma[j]; zeta lane j is the per-pair zeta for pair sigma[j].
   a1/b1 = e_vtrn2q_s16 of f_low/f_high (odd lanes of each pair), bridged to
   repr-views via lemma_repr_index; zeta lanes are the loaded zetas array.
   Bound-parametric in `b` (the input coefficient bound). *)
#push-options "--z3rlimit 100 --split_queries always"
let lemma_nttmul_in
    (b: nat)
    (lhs rhs: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (a1 b1 zeta: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16) : Lemma
  (requires
    NS.is_i16b_array b (repr lhs) /\ NS.is_i16b_array b (repr rhs) /\
    NS.is_i16b 1664 z1 /\ NS.is_i16b 1664 z2 /\ NS.is_i16b 1664 z3 /\ NS.is_i16b 1664 z4 /\
    a1 == NI.e_vtrn2q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    b1 == NI.e_vtrn2q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    NI.get_lane_i16x8 zeta 0 == z1 /\
    NI.get_lane_i16x8 zeta 1 == z3 /\
    NI.get_lane_i16x8 zeta 2 == Rust_primitives.Arithmetic.neg z1 /\
    NI.get_lane_i16x8 zeta 3 == Rust_primitives.Arithmetic.neg z3 /\
    NI.get_lane_i16x8 zeta 4 == z2 /\
    NI.get_lane_i16x8 zeta 5 == z4 /\
    NI.get_lane_i16x8 zeta 6 == Rust_primitives.Arithmetic.neg z2 /\
    NI.get_lane_i16x8 zeta 7 == Rust_primitives.Arithmetic.neg z4)
  (ensures
    (forall (i: nat{i < 8}). NS.is_i16b b (NI.get_lane_i16x8 a1 i)) /\
    (forall (i: nat{i < 8}). NS.is_i16b b (NI.get_lane_i16x8 b1 i)) /\
    v (NI.get_lane_i16x8 a1 0) == v (Seq.index (repr lhs) 1)  /\
    v (NI.get_lane_i16x8 a1 1) == v (Seq.index (repr lhs) 9)  /\
    v (NI.get_lane_i16x8 a1 2) == v (Seq.index (repr lhs) 3)  /\
    v (NI.get_lane_i16x8 a1 3) == v (Seq.index (repr lhs) 11) /\
    v (NI.get_lane_i16x8 a1 4) == v (Seq.index (repr lhs) 5)  /\
    v (NI.get_lane_i16x8 a1 5) == v (Seq.index (repr lhs) 13) /\
    v (NI.get_lane_i16x8 a1 6) == v (Seq.index (repr lhs) 7)  /\
    v (NI.get_lane_i16x8 a1 7) == v (Seq.index (repr lhs) 15) /\
    v (NI.get_lane_i16x8 b1 0) == v (Seq.index (repr rhs) 1)  /\
    v (NI.get_lane_i16x8 b1 1) == v (Seq.index (repr rhs) 9)  /\
    v (NI.get_lane_i16x8 b1 2) == v (Seq.index (repr rhs) 3)  /\
    v (NI.get_lane_i16x8 b1 3) == v (Seq.index (repr rhs) 11) /\
    v (NI.get_lane_i16x8 b1 4) == v (Seq.index (repr rhs) 5)  /\
    v (NI.get_lane_i16x8 b1 5) == v (Seq.index (repr rhs) 13) /\
    v (NI.get_lane_i16x8 b1 6) == v (Seq.index (repr rhs) 7)  /\
    v (NI.get_lane_i16x8 b1 7) == v (Seq.index (repr rhs) 15) /\
    v (NI.get_lane_i16x8 zeta 0) == v z1 /\
    v (NI.get_lane_i16x8 zeta 1) == v z3 /\
    v (NI.get_lane_i16x8 zeta 2) == - (v z1) /\
    v (NI.get_lane_i16x8 zeta 3) == - (v z3) /\
    v (NI.get_lane_i16x8 zeta 4) == v z2 /\
    v (NI.get_lane_i16x8 zeta 5) == v z4 /\
    v (NI.get_lane_i16x8 zeta 6) == - (v z2) /\
    v (NI.get_lane_i16x8 zeta 7) == - (v z4))
  = let lf = lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
    let lh = lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let rf = rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
    let rh = rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let a1c = NI.e_vtrn2q_s16 lf lh in
    let b1c = NI.e_vtrn2q_s16 rf rh in
    let insta (i: nat{i < 4}) : Lemma
      (NI.get_lane_i16x8 a1c (2*i)   == NI.get_lane_i16x8 lf (2*i+1) /\
       NI.get_lane_i16x8 a1c (2*i+1) == NI.get_lane_i16x8 lh (2*i+1)) = () in
    let instb (i: nat{i < 4}) : Lemma
      (NI.get_lane_i16x8 b1c (2*i)   == NI.get_lane_i16x8 rf (2*i+1) /\
       NI.get_lane_i16x8 b1c (2*i+1) == NI.get_lane_i16x8 rh (2*i+1)) = () in
    insta 0; insta 1; insta 2; insta 3;
    instb 0; instb 1; instb 2; instb 3;
    introduce forall (i: nat{i < 8}). NS.is_i16b b (NI.get_lane_i16x8 a1 i)
    with (if i=0 then () else if i=1 then () else if i=2 then () else if i=3 then ()
          else if i=4 then () else if i=5 then () else if i=6 then () else ());
    introduce forall (i: nat{i < 8}). NS.is_i16b b (NI.get_lane_i16x8 b1 i)
    with (if i=0 then () else if i=1 then () else if i=2 then () else if i=3 then ()
          else if i=4 then () else if i=5 then () else if i=6 then () else ());
    assert (v (NI.get_lane_i16x8 zeta 2) == - (v z1));
    assert (v (NI.get_lane_i16x8 zeta 3) == - (v z3));
    assert (v (NI.get_lane_i16x8 zeta 6) == - (v z2));
    assert (v (NI.get_lane_i16x8 zeta 7) == - (v z4))
#pop-options

(* EVEN-output montgomery-reduce input.  For pair p[k] (m=sigma[k]) the encoded
   int32 Pf_k = lhs[2p]*rhs[2p] + a1b1[m]*zeta[m]; stated in cast/i16x2_as_i32
   round-trip form so lemma_nttmul_redcong applies.  Honest: the vmull/vmlal
   widening MAC + reinterpret/trn lane layout is recomputed from the
   a0/b0/a1b1/zeta construction (mirrors lemma_nttmul_in's trn threading). *)
#push-options "--z3rlimit 300 --split_queries always --z3refresh"
let lemma_nttmul_montval_fst
    (lhs rhs: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (a0 b0 a1b1 zeta: NI.t_e_int16x8_t)
    (a1b1_low a1b1_high: NI.t_e_int32x4_t)
    (fst_low fst_high flo fhi: NI.t_e_int16x8_t) : Lemma
  (requires
    NS.is_i16b_array 3328 (repr lhs) /\ NS.is_i16b_array 3328 (repr rhs) /\
    (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 a1b1 i)) /\
    (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 zeta i)) /\
    a0 == NI.e_vtrn1q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    b0 == NI.e_vtrn1q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    a1b1_low == NI.e_vmull_s16 (NI.e_vget_low_s16 a1b1) (NI.e_vget_low_s16 zeta) /\
    a1b1_high == NI.e_vmull_high_s16 a1b1 zeta /\
    fst_low == NI.e_vreinterpretq_s16_s32
                 (NI.e_vmlal_s16 a1b1_low (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b0)) /\
    fst_high == NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_high_s16 a1b1_high a0 b0) /\
    flo == NI.e_vtrn1q_s16 fst_low fst_high /\
    fhi == NI.e_vtrn2q_s16 fst_low fst_high)
  (ensures
    rt_ok flo fhi 0 /\ rt_ok flo fhi 1 /\ rt_ok flo fhi 2 /\ rt_ok flo fhi 3 /\
    rt_ok flo fhi 4 /\ rt_ok flo fhi 5 /\ rt_ok flo fhi 6 /\ rt_ok flo fhi 7 /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 0) (NI.get_lane_i16x8 fhi 0)) ==
      v (Seq.index (repr lhs) 0)  * v (Seq.index (repr rhs) 0)  + v (NI.get_lane_i16x8 a1b1 0) * v (NI.get_lane_i16x8 zeta 0) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 1) (NI.get_lane_i16x8 fhi 1)) ==
      v (Seq.index (repr lhs) 4)  * v (Seq.index (repr rhs) 4)  + v (NI.get_lane_i16x8 a1b1 4) * v (NI.get_lane_i16x8 zeta 4) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 2) (NI.get_lane_i16x8 fhi 2)) ==
      v (Seq.index (repr lhs) 8)  * v (Seq.index (repr rhs) 8)  + v (NI.get_lane_i16x8 a1b1 1) * v (NI.get_lane_i16x8 zeta 1) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 3) (NI.get_lane_i16x8 fhi 3)) ==
      v (Seq.index (repr lhs) 12) * v (Seq.index (repr rhs) 12) + v (NI.get_lane_i16x8 a1b1 5) * v (NI.get_lane_i16x8 zeta 5) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 4) (NI.get_lane_i16x8 fhi 4)) ==
      v (Seq.index (repr lhs) 2)  * v (Seq.index (repr rhs) 2)  + v (NI.get_lane_i16x8 a1b1 2) * v (NI.get_lane_i16x8 zeta 2) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 5) (NI.get_lane_i16x8 fhi 5)) ==
      v (Seq.index (repr lhs) 6)  * v (Seq.index (repr rhs) 6)  + v (NI.get_lane_i16x8 a1b1 6) * v (NI.get_lane_i16x8 zeta 6) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 6) (NI.get_lane_i16x8 fhi 6)) ==
      v (Seq.index (repr lhs) 10) * v (Seq.index (repr rhs) 10) + v (NI.get_lane_i16x8 a1b1 3) * v (NI.get_lane_i16x8 zeta 3) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 7) (NI.get_lane_i16x8 fhi 7)) ==
      v (Seq.index (repr lhs) 14) * v (Seq.index (repr rhs) 14) + v (NI.get_lane_i16x8 a1b1 7) * v (NI.get_lane_i16x8 zeta 7))
  =
  let lf = lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
  let lh = lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
  let rf = rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
  let rh = rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
  let mlo = NI.e_vmlal_s16 a1b1_low (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b0) in
  let mhi = NI.e_vmlal_high_s16 a1b1_high a0 b0 in
  let flchk = NI.e_vreinterpretq_s16_s32 mlo in
  let fhchk = NI.e_vreinterpretq_s16_s32 mhi in
  let rlo = NI.e_vreinterpretq_s32_s16 fst_low in
  let rhi = NI.e_vreinterpretq_s32_s16 fst_high in
  assert_norm (pow2 15 == 32768);
  assert (fst_low == flchk);
  assert (fst_high == fhchk);
  assert (fst_low == mlo);
  assert (fst_high == mhi);
  assert (rlo == mlo);
  assert (rhi == mhi);
  let ia (j: nat{j < 4}) : Lemma
    (NI.get_lane_i16x8 a0 (2*j)   == NI.get_lane_i16x8 lf (2*j) /\
     NI.get_lane_i16x8 a0 (2*j+1) == NI.get_lane_i16x8 lh (2*j)) = () in
  let ib (j: nat{j < 4}) : Lemma
    (NI.get_lane_i16x8 b0 (2*j)   == NI.get_lane_i16x8 rf (2*j) /\
     NI.get_lane_i16x8 b0 (2*j+1) == NI.get_lane_i16x8 rh (2*j)) = () in
  ia 0; ia 1; ia 2; ia 3; ib 0; ib 1; ib 2; ib 3;
  let work (i: nat{i < 4}) (idxe idxo: nat{idxe < 16 /\ idxo < 16}) : Lemma
    (requires
      NI.get_lane_i16x8 a0 i     == Seq.index (repr lhs) idxe /\
      NI.get_lane_i16x8 b0 i     == Seq.index (repr rhs) idxe /\
      NI.get_lane_i16x8 a0 (i+4) == Seq.index (repr lhs) idxo /\
      NI.get_lane_i16x8 b0 (i+4) == Seq.index (repr rhs) idxo)
    (ensures
      NI.get_lane_i16x8 flo (2*i) ==
        (cast (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i)) (NI.get_lane_i16x8 fhi (2*i))) <: i16) /\
      NI.get_lane_i16x8 fhi (2*i) ==
        (cast ((NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i)) (NI.get_lane_i16x8 fhi (2*i))) >>! (mk_i32 16)) <: i16) /\
      NS.is_i32b (3328 * pow2 15) (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i)) (NI.get_lane_i16x8 fhi (2*i))) /\
      v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i)) (NI.get_lane_i16x8 fhi (2*i))) ==
        v (Seq.index (repr lhs) idxe) * v (Seq.index (repr rhs) idxe) +
        v (NI.get_lane_i16x8 a1b1 i) * v (NI.get_lane_i16x8 zeta i) /\
      NI.get_lane_i16x8 flo (2*i+1) ==
        (cast (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i+1)) (NI.get_lane_i16x8 fhi (2*i+1))) <: i16) /\
      NI.get_lane_i16x8 fhi (2*i+1) ==
        (cast ((NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i+1)) (NI.get_lane_i16x8 fhi (2*i+1))) >>! (mk_i32 16)) <: i16) /\
      NS.is_i32b (3328 * pow2 15) (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i+1)) (NI.get_lane_i16x8 fhi (2*i+1))) /\
      v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo (2*i+1)) (NI.get_lane_i16x8 fhi (2*i+1))) ==
        v (Seq.index (repr lhs) idxo) * v (Seq.index (repr rhs) idxo) +
        v (NI.get_lane_i16x8 a1b1 (i+4)) * v (NI.get_lane_i16x8 zeta (i+4)))
    =
    assert (NI.get_lane_i16x8 flo (2*i)   == NI.get_lane_i16x8 fst_low (2*i));
    assert (NI.get_lane_i16x8 fhi (2*i)   == NI.get_lane_i16x8 fst_low (2*i+1));
    assert (NI.get_lane_i32x4 rlo i ==
              NI.i16x2_as_i32 (NI.get_lane_i16x8 fst_low (2*i)) (NI.get_lane_i16x8 fst_low (2*i+1)));
    assert (NI.get_lane_i32x4 a1b1_low i ==
              ((cast (NI.get_lane_i16x8 a1b1 i) <: i32) *. (cast (NI.get_lane_i16x8 zeta i) <: i32)));
    assert (NI.get_lane_i32x4 mlo i ==
              ((NI.get_lane_i32x4 a1b1_low i) +.
               ((cast (NI.get_lane_i16x8 a0 i) <: i32) *. (cast (NI.get_lane_i16x8 b0 i) <: i32))));
    lemma_mac32 (NI.get_lane_i16x8 a1b1 i) (NI.get_lane_i16x8 zeta i)
                (NI.get_lane_i16x8 a0 i) (NI.get_lane_i16x8 b0 i) 3328 1664 3328 3328;
    lemma_cast_lo (NI.get_lane_i16x8 flo (2*i)) (NI.get_lane_i16x8 fhi (2*i));
    lemma_cast_hi (NI.get_lane_i16x8 flo (2*i)) (NI.get_lane_i16x8 fhi (2*i));
    assert (NI.get_lane_i16x8 flo (2*i+1) == NI.get_lane_i16x8 fst_high (2*i));
    assert (NI.get_lane_i16x8 fhi (2*i+1) == NI.get_lane_i16x8 fst_high (2*i+1));
    assert (NI.get_lane_i32x4 rhi i ==
              NI.i16x2_as_i32 (NI.get_lane_i16x8 fst_high (2*i)) (NI.get_lane_i16x8 fst_high (2*i+1)));
    assert (NI.get_lane_i32x4 a1b1_high i ==
              ((cast (NI.get_lane_i16x8 a1b1 (i+4)) <: i32) *. (cast (NI.get_lane_i16x8 zeta (i+4)) <: i32)));
    assert (NI.get_lane_i32x4 mhi i ==
              ((NI.get_lane_i32x4 a1b1_high i) +.
               ((cast (NI.get_lane_i16x8 a0 (i+4)) <: i32) *. (cast (NI.get_lane_i16x8 b0 (i+4)) <: i32))));
    lemma_mac32 (NI.get_lane_i16x8 a1b1 (i+4)) (NI.get_lane_i16x8 zeta (i+4))
                (NI.get_lane_i16x8 a0 (i+4)) (NI.get_lane_i16x8 b0 (i+4)) 3328 1664 3328 3328;
    lemma_cast_lo (NI.get_lane_i16x8 flo (2*i+1)) (NI.get_lane_i16x8 fhi (2*i+1));
    lemma_cast_hi (NI.get_lane_i16x8 flo (2*i+1)) (NI.get_lane_i16x8 fhi (2*i+1))
  in
  work 0 0 4; work 1 8 12; work 2 2 6; work 3 10 14;
  reveal_opaque (`%rt_ok) (rt_ok flo fhi 0); reveal_opaque (`%rt_ok) (rt_ok flo fhi 1); reveal_opaque (`%rt_ok) (rt_ok flo fhi 2); reveal_opaque (`%rt_ok) (rt_ok flo fhi 3); reveal_opaque (`%rt_ok) (rt_ok flo fhi 4); reveal_opaque (`%rt_ok) (rt_ok flo fhi 5); reveal_opaque (`%rt_ok) (rt_ok flo fhi 6); reveal_opaque (`%rt_ok) (rt_ok flo fhi 7)
#pop-options

(* ODD-output montgomery-reduce input.  Ps_k = lhs[2p]*rhs[2p+1] + lhs[2p+1]*rhs[2p];
   honest mirror of lemma_nttmul_montval_fst (a0b1 widening MAC + mlal with a1,b0). *)
#push-options "--z3rlimit 300 --split_queries always --z3refresh"
let lemma_nttmul_montval_snd
    (lhs rhs: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
    (a0 a1 b0 b1: NI.t_e_int16x8_t)
    (a0b1_low a0b1_high: NI.t_e_int32x4_t)
    (snd_low snd_high slo shi: NI.t_e_int16x8_t) : Lemma
  (requires
    NS.is_i16b_array 3328 (repr lhs) /\ NS.is_i16b_array 3328 (repr rhs) /\
    a0 == NI.e_vtrn1q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    a1 == NI.e_vtrn2q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    b0 == NI.e_vtrn1q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    b1 == NI.e_vtrn2q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                          rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high /\
    a0b1_low == NI.e_vmull_s16 (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b1) /\
    a0b1_high == NI.e_vmull_high_s16 a0 b1 /\
    snd_low == NI.e_vreinterpretq_s16_s32
                 (NI.e_vmlal_s16 a0b1_low (NI.e_vget_low_s16 a1) (NI.e_vget_low_s16 b0)) /\
    snd_high == NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_high_s16 a0b1_high a1 b0) /\
    slo == NI.e_vtrn1q_s16 snd_low snd_high /\
    shi == NI.e_vtrn2q_s16 snd_low snd_high)
  (ensures
    rt_ok slo shi 0 /\ rt_ok slo shi 1 /\ rt_ok slo shi 2 /\ rt_ok slo shi 3 /\
    rt_ok slo shi 4 /\ rt_ok slo shi 5 /\ rt_ok slo shi 6 /\ rt_ok slo shi 7 /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 0) (NI.get_lane_i16x8 shi 0)) ==
      v (Seq.index (repr lhs) 0)  * v (Seq.index (repr rhs) 1)  + v (Seq.index (repr lhs) 1)  * v (Seq.index (repr rhs) 0)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 1) (NI.get_lane_i16x8 shi 1)) ==
      v (Seq.index (repr lhs) 4)  * v (Seq.index (repr rhs) 5)  + v (Seq.index (repr lhs) 5)  * v (Seq.index (repr rhs) 4)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 2) (NI.get_lane_i16x8 shi 2)) ==
      v (Seq.index (repr lhs) 8)  * v (Seq.index (repr rhs) 9)  + v (Seq.index (repr lhs) 9)  * v (Seq.index (repr rhs) 8)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 3) (NI.get_lane_i16x8 shi 3)) ==
      v (Seq.index (repr lhs) 12) * v (Seq.index (repr rhs) 13) + v (Seq.index (repr lhs) 13) * v (Seq.index (repr rhs) 12) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 4) (NI.get_lane_i16x8 shi 4)) ==
      v (Seq.index (repr lhs) 2)  * v (Seq.index (repr rhs) 3)  + v (Seq.index (repr lhs) 3)  * v (Seq.index (repr rhs) 2)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 5) (NI.get_lane_i16x8 shi 5)) ==
      v (Seq.index (repr lhs) 6)  * v (Seq.index (repr rhs) 7)  + v (Seq.index (repr lhs) 7)  * v (Seq.index (repr rhs) 6)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 6) (NI.get_lane_i16x8 shi 6)) ==
      v (Seq.index (repr lhs) 10) * v (Seq.index (repr rhs) 11) + v (Seq.index (repr lhs) 11) * v (Seq.index (repr rhs) 10) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 7) (NI.get_lane_i16x8 shi 7)) ==
      v (Seq.index (repr lhs) 14) * v (Seq.index (repr rhs) 15) + v (Seq.index (repr lhs) 15) * v (Seq.index (repr rhs) 14))
  =
  let lf = lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
  let lh = lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
  let rf = rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low in
  let rh = rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
  let mlo = NI.e_vmlal_s16 a0b1_low (NI.e_vget_low_s16 a1) (NI.e_vget_low_s16 b0) in
  let mhi = NI.e_vmlal_high_s16 a0b1_high a1 b0 in
  let flchk = NI.e_vreinterpretq_s16_s32 mlo in
  let fhchk = NI.e_vreinterpretq_s16_s32 mhi in
  let rlo = NI.e_vreinterpretq_s32_s16 snd_low in
  let rhi = NI.e_vreinterpretq_s32_s16 snd_high in
  assert_norm (pow2 15 == 32768);
  assert (snd_low == flchk);
  assert (snd_high == fhchk);
  assert (snd_low == mlo);
  assert (snd_high == mhi);
  assert (rlo == mlo);
  assert (rhi == mhi);
  let ia (j: nat{j < 4}) : Lemma
    (NI.get_lane_i16x8 a0 (2*j)   == NI.get_lane_i16x8 lf (2*j) /\
     NI.get_lane_i16x8 a0 (2*j+1) == NI.get_lane_i16x8 lh (2*j)) = () in
  let ia2 (j: nat{j < 4}) : Lemma
    (NI.get_lane_i16x8 a1 (2*j)   == NI.get_lane_i16x8 lf (2*j+1) /\
     NI.get_lane_i16x8 a1 (2*j+1) == NI.get_lane_i16x8 lh (2*j+1)) = () in
  let ib (j: nat{j < 4}) : Lemma
    (NI.get_lane_i16x8 b0 (2*j)   == NI.get_lane_i16x8 rf (2*j) /\
     NI.get_lane_i16x8 b0 (2*j+1) == NI.get_lane_i16x8 rh (2*j)) = () in
  let ib2 (j: nat{j < 4}) : Lemma
    (NI.get_lane_i16x8 b1 (2*j)   == NI.get_lane_i16x8 rf (2*j+1) /\
     NI.get_lane_i16x8 b1 (2*j+1) == NI.get_lane_i16x8 rh (2*j+1)) = () in
  ia 0; ia 1; ia 2; ia 3; ia2 0; ia2 1; ia2 2; ia2 3;
  ib 0; ib 1; ib 2; ib 3; ib2 0; ib2 1; ib2 2; ib2 3;
  let work (i: nat{i < 4}) (pe po: nat{pe < 15 /\ po < 15}) : Lemma
    (requires
      NI.get_lane_i16x8 a0 i     == Seq.index (repr lhs) pe     /\
      NI.get_lane_i16x8 b1 i     == Seq.index (repr rhs) (pe+1) /\
      NI.get_lane_i16x8 a1 i     == Seq.index (repr lhs) (pe+1) /\
      NI.get_lane_i16x8 b0 i     == Seq.index (repr rhs) pe     /\
      NI.get_lane_i16x8 a0 (i+4) == Seq.index (repr lhs) po     /\
      NI.get_lane_i16x8 b1 (i+4) == Seq.index (repr rhs) (po+1) /\
      NI.get_lane_i16x8 a1 (i+4) == Seq.index (repr lhs) (po+1) /\
      NI.get_lane_i16x8 b0 (i+4) == Seq.index (repr rhs) po)
    (ensures
      NI.get_lane_i16x8 slo (2*i) ==
        (cast (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i)) (NI.get_lane_i16x8 shi (2*i))) <: i16) /\
      NI.get_lane_i16x8 shi (2*i) ==
        (cast ((NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i)) (NI.get_lane_i16x8 shi (2*i))) >>! (mk_i32 16)) <: i16) /\
      NS.is_i32b (3328 * pow2 15) (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i)) (NI.get_lane_i16x8 shi (2*i))) /\
      v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i)) (NI.get_lane_i16x8 shi (2*i))) ==
        v (Seq.index (repr lhs) pe) * v (Seq.index (repr rhs) (pe+1)) +
        v (Seq.index (repr lhs) (pe+1)) * v (Seq.index (repr rhs) pe) /\
      NI.get_lane_i16x8 slo (2*i+1) ==
        (cast (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i+1)) (NI.get_lane_i16x8 shi (2*i+1))) <: i16) /\
      NI.get_lane_i16x8 shi (2*i+1) ==
        (cast ((NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i+1)) (NI.get_lane_i16x8 shi (2*i+1))) >>! (mk_i32 16)) <: i16) /\
      NS.is_i32b (3328 * pow2 15) (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i+1)) (NI.get_lane_i16x8 shi (2*i+1))) /\
      v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo (2*i+1)) (NI.get_lane_i16x8 shi (2*i+1))) ==
        v (Seq.index (repr lhs) po) * v (Seq.index (repr rhs) (po+1)) +
        v (Seq.index (repr lhs) (po+1)) * v (Seq.index (repr rhs) po))
    =
    assert (NI.get_lane_i16x8 slo (2*i)   == NI.get_lane_i16x8 snd_low (2*i));
    assert (NI.get_lane_i16x8 shi (2*i)   == NI.get_lane_i16x8 snd_low (2*i+1));
    assert (NI.get_lane_i32x4 rlo i ==
              NI.i16x2_as_i32 (NI.get_lane_i16x8 snd_low (2*i)) (NI.get_lane_i16x8 snd_low (2*i+1)));
    assert (NI.get_lane_i32x4 a0b1_low i ==
              ((cast (NI.get_lane_i16x8 a0 i) <: i32) *. (cast (NI.get_lane_i16x8 b1 i) <: i32)));
    assert (NI.get_lane_i32x4 mlo i ==
              ((NI.get_lane_i32x4 a0b1_low i) +.
               ((cast (NI.get_lane_i16x8 a1 i) <: i32) *. (cast (NI.get_lane_i16x8 b0 i) <: i32))));
    lemma_mac32 (NI.get_lane_i16x8 a0 i) (NI.get_lane_i16x8 b1 i)
                (NI.get_lane_i16x8 a1 i) (NI.get_lane_i16x8 b0 i) 3328 3328 3328 3328;
    lemma_cast_lo (NI.get_lane_i16x8 slo (2*i)) (NI.get_lane_i16x8 shi (2*i));
    lemma_cast_hi (NI.get_lane_i16x8 slo (2*i)) (NI.get_lane_i16x8 shi (2*i));
    assert (NI.get_lane_i16x8 slo (2*i+1) == NI.get_lane_i16x8 snd_high (2*i));
    assert (NI.get_lane_i16x8 shi (2*i+1) == NI.get_lane_i16x8 snd_high (2*i+1));
    assert (NI.get_lane_i32x4 rhi i ==
              NI.i16x2_as_i32 (NI.get_lane_i16x8 snd_high (2*i)) (NI.get_lane_i16x8 snd_high (2*i+1)));
    assert (NI.get_lane_i32x4 a0b1_high i ==
              ((cast (NI.get_lane_i16x8 a0 (i+4)) <: i32) *. (cast (NI.get_lane_i16x8 b1 (i+4)) <: i32)));
    assert (NI.get_lane_i32x4 mhi i ==
              ((NI.get_lane_i32x4 a0b1_high i) +.
               ((cast (NI.get_lane_i16x8 a1 (i+4)) <: i32) *. (cast (NI.get_lane_i16x8 b0 (i+4)) <: i32))));
    lemma_mac32 (NI.get_lane_i16x8 a0 (i+4)) (NI.get_lane_i16x8 b1 (i+4))
                (NI.get_lane_i16x8 a1 (i+4)) (NI.get_lane_i16x8 b0 (i+4)) 3328 3328 3328 3328;
    lemma_cast_lo (NI.get_lane_i16x8 slo (2*i+1)) (NI.get_lane_i16x8 shi (2*i+1));
    lemma_cast_hi (NI.get_lane_i16x8 slo (2*i+1)) (NI.get_lane_i16x8 shi (2*i+1))
  in
  work 0 0 4; work 1 8 12; work 2 2 6; work 3 10 14;
  reveal_opaque (`%rt_ok) (rt_ok slo shi 0); reveal_opaque (`%rt_ok) (rt_ok slo shi 1); reveal_opaque (`%rt_ok) (rt_ok slo shi 2); reveal_opaque (`%rt_ok) (rt_ok slo shi 3); reveal_opaque (`%rt_ok) (rt_ok slo shi 4); reveal_opaque (`%rt_ok) (rt_ok slo shi 5); reveal_opaque (`%rt_ok) (rt_ok slo shi 6); reveal_opaque (`%rt_ok) (rt_ok slo shi 7)
#pop-options

(* u8<->s16 byte round-trip: reassembling an i16 from its two little-endian
   bytes recovers it.  Reduce to the u16 reassembly w == (cast_mod x : u16), then
   cast_identity_lemma (SMTPat) gives cast_mod #u16 #i16 w == x.  The u16 reassembly
   is bit-level via get_bit_* SMTPats, bridging the u8->u16 widening `cast` to
   `cast_mod` (small_mod) so those SMTPats fire (mirror lemma_i16_bits_as_u32_bit). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_u8x2_i16_byte (x: i16) : Lemma
  (ensures NI.u8x2_as_i16 (NI.i16_byte x 0) (NI.i16_byte x 1) == x)
  = let b0 = NI.i16_byte x 0 in
    let b1 = NI.i16_byte x 1 in
    let ux:u16 = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                   #Rust_primitives.Integers.u16_inttype x in
    FStar.Math.Lemmas.small_mod (v b0) (pow2 16);
    FStar.Math.Lemmas.small_mod (v b1) (pow2 16);
    assert (Rust_primitives.Integers.cast #Rust_primitives.Integers.u8_inttype
              #Rust_primitives.Integers.u16_inttype b0 ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u8_inttype
              #Rust_primitives.Integers.u16_inttype b0);
    assert (Rust_primitives.Integers.cast #Rust_primitives.Integers.u8_inttype
              #Rust_primitives.Integers.u16_inttype b1 ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u8_inttype
              #Rust_primitives.Integers.u16_inttype b1);
    let w = NI.u8x2_as_u16 b0 b1 in
    let aux (i: usize {v i < 16}) : Lemma (get_bit w i == get_bit ux i) = () in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits w ux
#pop-options

(* One output lane of the vqtbl1q deinterleave, proven in CLEAN context (the byte
   bit-reasoning stays here, off the main lemma's WP).  res = s16<-u8 reinterpret of
   the table-lookup of the u8<-s16 reinterpret of src; if index pair (2i,2i+1) selects
   bytes (2p,2p+1) then res lane i == src lane p (byte round-trip). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100 --split_queries always --z3refresh"
let lemma_out_u8lane
    (src res: NI.t_e_int16x8_t) (index: NI.t_e_uint8x16_t) (i: nat{i < 8}) (p: nat{p < 8}) : Lemma
  (requires
    res == NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 src) index) /\
    v (NI.get_lane_u8x16 index (2*i))   == 2*p /\
    v (NI.get_lane_u8x16 index (2*i+1)) == 2*p+1)
  (ensures NI.get_lane_i16x8 res i == NI.get_lane_i16x8 src p)
  = let su8 = NI.e_vreinterpretq_u8_s16 src in
    let pu8 = NI.e_vqtbl1q_u8 su8 index in
    let rc  = NI.e_vreinterpretq_s16_u8 pu8 in
    (* div/mod facts so the u8<-s16 forall (i16_byte (a (k/2)) (k%2)) reduces at k=2p,2p+1 *)
    FStar.Math.Lemmas.cancel_mul_div p 2;
    FStar.Math.Lemmas.cancel_mul_mod p 2;
    assert ((2*p)/2 == p /\ (2*p)%2 == 0);
    assert ((2*p+1)/2 == p /\ (2*p+1)%2 == 1);
    assert (NI.get_lane_u8x16 su8 (2*p)   == NI.i16_byte (NI.get_lane_i16x8 src p) 0);
    assert (NI.get_lane_u8x16 su8 (2*p+1) == NI.i16_byte (NI.get_lane_i16x8 src p) 1);
    assert (NI.get_lane_u8x16 pu8 (2*i)   == NI.get_lane_u8x16 su8 (2*p));
    assert (NI.get_lane_u8x16 pu8 (2*i+1) == NI.get_lane_u8x16 su8 (2*p+1));
    assert (NI.get_lane_i16x8 rc i ==
              NI.u8x2_as_i16 (NI.get_lane_u8x16 pu8 (2*i)) (NI.get_lane_u8x16 pu8 (2*i+1)));
    lemma_u8x2_i16_byte (NI.get_lane_i16x8 src p)
#pop-options

(* OUTPUT ASSEMBLY (trn(fst,snd) -> trn_s32 -> vqtbl1q_u8 permute).
   res.f_low (low2) / res.f_high (high2) lanes in terms of fst/snd lanes,
   composed from the proven trn-s32 lemmas + lemma_out_u8lane. *)
#push-options "--z3rlimit 300 --split_queries always --z3refresh"
let lemma_nttmul_out
    (fst snd low1 high1 low2 high2: NI.t_e_int16x8_t)
    (low0 high0: NI.t_e_int32x4_t)
    (index: NI.t_e_uint8x16_t) : Lemma
  (requires
    low0 == NI.e_vreinterpretq_s32_s16 (NI.e_vtrn1q_s16 fst snd) /\
    high0 == NI.e_vreinterpretq_s32_s16 (NI.e_vtrn2q_s16 fst snd) /\
    low1 == NI.e_vreinterpretq_s16_s32 (NI.e_vtrn1q_s32 low0 high0) /\
    high1 == NI.e_vreinterpretq_s16_s32 (NI.e_vtrn2q_s32 low0 high0) /\
    low2 == NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 low1) index) /\
    high2 == NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 high1) index) /\
    v (NI.get_lane_u8x16 index 0)  == 0  /\ v (NI.get_lane_u8x16 index 1)  == 1  /\
    v (NI.get_lane_u8x16 index 2)  == 2  /\ v (NI.get_lane_u8x16 index 3)  == 3  /\
    v (NI.get_lane_u8x16 index 4)  == 8  /\ v (NI.get_lane_u8x16 index 5)  == 9  /\
    v (NI.get_lane_u8x16 index 6)  == 10 /\ v (NI.get_lane_u8x16 index 7)  == 11 /\
    v (NI.get_lane_u8x16 index 8)  == 4  /\ v (NI.get_lane_u8x16 index 9)  == 5  /\
    v (NI.get_lane_u8x16 index 10) == 6  /\ v (NI.get_lane_u8x16 index 11) == 7  /\
    v (NI.get_lane_u8x16 index 12) == 12 /\ v (NI.get_lane_u8x16 index 13) == 13 /\
    v (NI.get_lane_u8x16 index 14) == 14 /\ v (NI.get_lane_u8x16 index 15) == 15)
  (ensures
    NI.get_lane_i16x8 low2 0 == NI.get_lane_i16x8 fst 0 /\
    NI.get_lane_i16x8 low2 1 == NI.get_lane_i16x8 snd 0 /\
    NI.get_lane_i16x8 low2 2 == NI.get_lane_i16x8 fst 4 /\
    NI.get_lane_i16x8 low2 3 == NI.get_lane_i16x8 snd 4 /\
    NI.get_lane_i16x8 low2 4 == NI.get_lane_i16x8 fst 1 /\
    NI.get_lane_i16x8 low2 5 == NI.get_lane_i16x8 snd 1 /\
    NI.get_lane_i16x8 low2 6 == NI.get_lane_i16x8 fst 5 /\
    NI.get_lane_i16x8 low2 7 == NI.get_lane_i16x8 snd 5 /\
    NI.get_lane_i16x8 high2 0 == NI.get_lane_i16x8 fst 2 /\
    NI.get_lane_i16x8 high2 1 == NI.get_lane_i16x8 snd 2 /\
    NI.get_lane_i16x8 high2 2 == NI.get_lane_i16x8 fst 6 /\
    NI.get_lane_i16x8 high2 3 == NI.get_lane_i16x8 snd 6 /\
    NI.get_lane_i16x8 high2 4 == NI.get_lane_i16x8 fst 3 /\
    NI.get_lane_i16x8 high2 5 == NI.get_lane_i16x8 snd 3 /\
    NI.get_lane_i16x8 high2 6 == NI.get_lane_i16x8 fst 7 /\
    NI.get_lane_i16x8 high2 7 == NI.get_lane_i16x8 snd 7)
  = let t1 = NI.e_vtrn1q_s16 fst snd in
    let t2 = NI.e_vtrn2q_s16 fst snd in
    lemma_trn1_s32_reinterpret t1 t2;
    lemma_trn2_s32_reinterpret t1 t2;
    let it1 (i: nat{i < 4}) : Lemma
      (NI.get_lane_i16x8 t1 (2*i)   == NI.get_lane_i16x8 fst (2*i) /\
       NI.get_lane_i16x8 t1 (2*i+1) == NI.get_lane_i16x8 snd (2*i)) = () in
    let it2 (i: nat{i < 4}) : Lemma
      (NI.get_lane_i16x8 t2 (2*i)   == NI.get_lane_i16x8 fst (2*i+1) /\
       NI.get_lane_i16x8 t2 (2*i+1) == NI.get_lane_i16x8 snd (2*i+1)) = () in
    it1 0; it1 1; it1 2; it1 3; it2 0; it2 1; it2 2; it2 3;
    lemma_out_u8lane low1 low2 index 0 0; lemma_out_u8lane low1 low2 index 1 1;
    lemma_out_u8lane low1 low2 index 2 4; lemma_out_u8lane low1 low2 index 3 5;
    lemma_out_u8lane low1 low2 index 4 2; lemma_out_u8lane low1 low2 index 5 3;
    lemma_out_u8lane low1 low2 index 6 6; lemma_out_u8lane low1 low2 index 7 7;
    lemma_out_u8lane high1 high2 index 0 0; lemma_out_u8lane high1 high2 index 1 1;
    lemma_out_u8lane high1 high2 index 2 4; lemma_out_u8lane high1 high2 index 3 5;
    lemma_out_u8lane high1 high2 index 4 2; lemma_out_u8lane high1 high2 index 5 3;
    lemma_out_u8lane high1 high2 index 6 6; lemma_out_u8lane high1 high2 index 7 7
#pop-options

(* The honest per-pair core, proven in clean context.  Threads in the two
   montgomery_reduce posts and the a1b1 congruence as `requires`, and concludes the
   8 even + 8 odd per-lane congruences on fst/snd (indexed by pair p[k]). *)
#push-options "--z3rlimit 400 --split_queries always --z3refresh"
let lemma_nttmul_fstsnd
    (iv_l iv_r: t_Array i16 (mk_usize 16))
    (a1 b1 a1b1 zeta flo fhi slo shi fst snd: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16) : Lemma
  (requires
    NS.is_i16b_array 3328 iv_l /\ NS.is_i16b_array 3328 iv_r /\
    NS.is_i16b 1664 z1 /\ NS.is_i16b 1664 z2 /\ NS.is_i16b 1664 z3 /\ NS.is_i16b 1664 z4 /\
    (forall (i: nat{i < 8}). NI.get_lane_i16x8 fst i ==
       NI.get_lane_i16x8 fhi i -.
       (cast (((cast (NI.get_lane_i16x8 flo i *. (mk_i16 (-3327))) <: i32) *. (mk_i32 3329))
               >>! (mk_i32 16)) <: i16)) /\
    (forall (i: nat{i < 8}). NI.get_lane_i16x8 snd i ==
       NI.get_lane_i16x8 shi i -.
       (cast (((cast (NI.get_lane_i16x8 slo i *. (mk_i16 (-3327))) <: i32) *. (mk_i32 3329))
               >>! (mk_i32 16)) <: i16)) /\
    (forall (i: nat{i < 8}). v (NI.get_lane_i16x8 a1b1 i) % 3329 ==
       (v (NI.get_lane_i16x8 a1 i) * v (NI.get_lane_i16x8 b1 i) * 169) % 3329) /\
    (* nttmul_in facts (a1/b1 odd-lane permutation + zeta lanes), established by
       lemma_nttmul_in at the ntt_multiply call site *)
    v (NI.get_lane_i16x8 a1 0) == v (Seq.index iv_l 1)  /\
    v (NI.get_lane_i16x8 a1 1) == v (Seq.index iv_l 9)  /\
    v (NI.get_lane_i16x8 a1 2) == v (Seq.index iv_l 3)  /\
    v (NI.get_lane_i16x8 a1 3) == v (Seq.index iv_l 11) /\
    v (NI.get_lane_i16x8 a1 4) == v (Seq.index iv_l 5)  /\
    v (NI.get_lane_i16x8 a1 5) == v (Seq.index iv_l 13) /\
    v (NI.get_lane_i16x8 a1 6) == v (Seq.index iv_l 7)  /\
    v (NI.get_lane_i16x8 a1 7) == v (Seq.index iv_l 15) /\
    v (NI.get_lane_i16x8 b1 0) == v (Seq.index iv_r 1)  /\
    v (NI.get_lane_i16x8 b1 1) == v (Seq.index iv_r 9)  /\
    v (NI.get_lane_i16x8 b1 2) == v (Seq.index iv_r 3)  /\
    v (NI.get_lane_i16x8 b1 3) == v (Seq.index iv_r 11) /\
    v (NI.get_lane_i16x8 b1 4) == v (Seq.index iv_r 5)  /\
    v (NI.get_lane_i16x8 b1 5) == v (Seq.index iv_r 13) /\
    v (NI.get_lane_i16x8 b1 6) == v (Seq.index iv_r 7)  /\
    v (NI.get_lane_i16x8 b1 7) == v (Seq.index iv_r 15) /\
    v (NI.get_lane_i16x8 zeta 0) == v z1 /\
    v (NI.get_lane_i16x8 zeta 1) == v z3 /\
    v (NI.get_lane_i16x8 zeta 2) == - (v z1) /\
    v (NI.get_lane_i16x8 zeta 3) == - (v z3) /\
    v (NI.get_lane_i16x8 zeta 4) == v z2 /\
    v (NI.get_lane_i16x8 zeta 5) == v z4 /\
    v (NI.get_lane_i16x8 zeta 6) == - (v z2) /\
    v (NI.get_lane_i16x8 zeta 7) == - (v z4) /\
    (* montval_fst facts (established at the ntt_multiply call site by lemma_nttmul_montval_fst) *)
    rt_ok flo fhi 0 /\ rt_ok flo fhi 1 /\ rt_ok flo fhi 2 /\ rt_ok flo fhi 3 /\
    rt_ok flo fhi 4 /\ rt_ok flo fhi 5 /\ rt_ok flo fhi 6 /\ rt_ok flo fhi 7 /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 0) (NI.get_lane_i16x8 fhi 0)) ==
      v (Seq.index iv_l 0)  * v (Seq.index iv_r 0)  + v (NI.get_lane_i16x8 a1b1 0) * v (NI.get_lane_i16x8 zeta 0) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 1) (NI.get_lane_i16x8 fhi 1)) ==
      v (Seq.index iv_l 4)  * v (Seq.index iv_r 4)  + v (NI.get_lane_i16x8 a1b1 4) * v (NI.get_lane_i16x8 zeta 4) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 2) (NI.get_lane_i16x8 fhi 2)) ==
      v (Seq.index iv_l 8)  * v (Seq.index iv_r 8)  + v (NI.get_lane_i16x8 a1b1 1) * v (NI.get_lane_i16x8 zeta 1) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 3) (NI.get_lane_i16x8 fhi 3)) ==
      v (Seq.index iv_l 12) * v (Seq.index iv_r 12) + v (NI.get_lane_i16x8 a1b1 5) * v (NI.get_lane_i16x8 zeta 5) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 4) (NI.get_lane_i16x8 fhi 4)) ==
      v (Seq.index iv_l 2)  * v (Seq.index iv_r 2)  + v (NI.get_lane_i16x8 a1b1 2) * v (NI.get_lane_i16x8 zeta 2) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 5) (NI.get_lane_i16x8 fhi 5)) ==
      v (Seq.index iv_l 6)  * v (Seq.index iv_r 6)  + v (NI.get_lane_i16x8 a1b1 6) * v (NI.get_lane_i16x8 zeta 6) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 6) (NI.get_lane_i16x8 fhi 6)) ==
      v (Seq.index iv_l 10) * v (Seq.index iv_r 10) + v (NI.get_lane_i16x8 a1b1 3) * v (NI.get_lane_i16x8 zeta 3) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo 7) (NI.get_lane_i16x8 fhi 7)) ==
      v (Seq.index iv_l 14) * v (Seq.index iv_r 14) + v (NI.get_lane_i16x8 a1b1 7) * v (NI.get_lane_i16x8 zeta 7) /\
    (* montval_snd facts (established at the ntt_multiply call site by lemma_nttmul_montval_snd) *)
    rt_ok slo shi 0 /\ rt_ok slo shi 1 /\ rt_ok slo shi 2 /\ rt_ok slo shi 3 /\
    rt_ok slo shi 4 /\ rt_ok slo shi 5 /\ rt_ok slo shi 6 /\ rt_ok slo shi 7 /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 0) (NI.get_lane_i16x8 shi 0)) ==
      v (Seq.index iv_l 0)  * v (Seq.index iv_r 1)  + v (Seq.index iv_l 1)  * v (Seq.index iv_r 0)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 1) (NI.get_lane_i16x8 shi 1)) ==
      v (Seq.index iv_l 4)  * v (Seq.index iv_r 5)  + v (Seq.index iv_l 5)  * v (Seq.index iv_r 4)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 2) (NI.get_lane_i16x8 shi 2)) ==
      v (Seq.index iv_l 8)  * v (Seq.index iv_r 9)  + v (Seq.index iv_l 9)  * v (Seq.index iv_r 8)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 3) (NI.get_lane_i16x8 shi 3)) ==
      v (Seq.index iv_l 12) * v (Seq.index iv_r 13) + v (Seq.index iv_l 13) * v (Seq.index iv_r 12) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 4) (NI.get_lane_i16x8 shi 4)) ==
      v (Seq.index iv_l 2)  * v (Seq.index iv_r 3)  + v (Seq.index iv_l 3)  * v (Seq.index iv_r 2)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 5) (NI.get_lane_i16x8 shi 5)) ==
      v (Seq.index iv_l 6)  * v (Seq.index iv_r 7)  + v (Seq.index iv_l 7)  * v (Seq.index iv_r 6)  /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 6) (NI.get_lane_i16x8 shi 6)) ==
      v (Seq.index iv_l 10) * v (Seq.index iv_r 11) + v (Seq.index iv_l 11) * v (Seq.index iv_r 10) /\
    v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo 7) (NI.get_lane_i16x8 shi 7)) ==
      v (Seq.index iv_l 14) * v (Seq.index iv_r 15) + v (Seq.index iv_l 15) * v (Seq.index iv_r 14))
  (ensures
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 0) /\ v (NI.get_lane_i16x8 fst 0) % 3329 == ((v (Seq.index iv_l 0)  * v (Seq.index iv_r 0)  + v (Seq.index iv_l 1)  * v (Seq.index iv_r 1)  * (v z1)     * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 1) /\ v (NI.get_lane_i16x8 fst 1) % 3329 == ((v (Seq.index iv_l 4)  * v (Seq.index iv_r 4)  + v (Seq.index iv_l 5)  * v (Seq.index iv_r 5)  * (v z2)     * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 2) /\ v (NI.get_lane_i16x8 fst 2) % 3329 == ((v (Seq.index iv_l 8)  * v (Seq.index iv_r 8)  + v (Seq.index iv_l 9)  * v (Seq.index iv_r 9)  * (v z3)     * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 3) /\ v (NI.get_lane_i16x8 fst 3) % 3329 == ((v (Seq.index iv_l 12) * v (Seq.index iv_r 12) + v (Seq.index iv_l 13) * v (Seq.index iv_r 13) * (v z4)     * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 4) /\ v (NI.get_lane_i16x8 fst 4) % 3329 == ((v (Seq.index iv_l 2)  * v (Seq.index iv_r 2)  + v (Seq.index iv_l 3)  * v (Seq.index iv_r 3)  * (- (v z1)) * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 5) /\ v (NI.get_lane_i16x8 fst 5) % 3329 == ((v (Seq.index iv_l 6)  * v (Seq.index iv_r 6)  + v (Seq.index iv_l 7)  * v (Seq.index iv_r 7)  * (- (v z2)) * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 6) /\ v (NI.get_lane_i16x8 fst 6) % 3329 == ((v (Seq.index iv_l 10) * v (Seq.index iv_r 10) + v (Seq.index iv_l 11) * v (Seq.index iv_r 11) * (- (v z3)) * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 fst 7) /\ v (NI.get_lane_i16x8 fst 7) % 3329 == ((v (Seq.index iv_l 14) * v (Seq.index iv_r 14) + v (Seq.index iv_l 15) * v (Seq.index iv_r 15) * (- (v z4)) * 169) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 0) /\ v (NI.get_lane_i16x8 snd 0) % 3329 == ((v (Seq.index iv_l 0)  * v (Seq.index iv_r 1)  + v (Seq.index iv_l 1)  * v (Seq.index iv_r 0))  * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 1) /\ v (NI.get_lane_i16x8 snd 1) % 3329 == ((v (Seq.index iv_l 4)  * v (Seq.index iv_r 5)  + v (Seq.index iv_l 5)  * v (Seq.index iv_r 4))  * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 2) /\ v (NI.get_lane_i16x8 snd 2) % 3329 == ((v (Seq.index iv_l 8)  * v (Seq.index iv_r 9)  + v (Seq.index iv_l 9)  * v (Seq.index iv_r 8))  * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 3) /\ v (NI.get_lane_i16x8 snd 3) % 3329 == ((v (Seq.index iv_l 12) * v (Seq.index iv_r 13) + v (Seq.index iv_l 13) * v (Seq.index iv_r 12)) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 4) /\ v (NI.get_lane_i16x8 snd 4) % 3329 == ((v (Seq.index iv_l 2)  * v (Seq.index iv_r 3)  + v (Seq.index iv_l 3)  * v (Seq.index iv_r 2))  * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 5) /\ v (NI.get_lane_i16x8 snd 5) % 3329 == ((v (Seq.index iv_l 6)  * v (Seq.index iv_r 7)  + v (Seq.index iv_l 7)  * v (Seq.index iv_r 6))  * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 6) /\ v (NI.get_lane_i16x8 snd 6) % 3329 == ((v (Seq.index iv_l 10) * v (Seq.index iv_r 11) + v (Seq.index iv_l 11) * v (Seq.index iv_r 10)) * 169) % 3329) /\
    (NS.is_i16b 3328 (NI.get_lane_i16x8 snd 7) /\ v (NI.get_lane_i16x8 snd 7) % 3329 == ((v (Seq.index iv_l 14) * v (Seq.index iv_r 15) + v (Seq.index iv_l 15) * v (Seq.index iv_r 14)) * 169) % 3329))
  =
  let ev (k pp m: nat) : Lemma
      (requires k < 8 /\ 2*pp+1 < 16 /\ m < 8 /\
        rt_ok flo fhi k /\
        v (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo k) (NI.get_lane_i16x8 fhi k)) ==
          v (Seq.index iv_l (2*pp)) * v (Seq.index iv_r (2*pp)) + v (NI.get_lane_i16x8 a1b1 m) * v (NI.get_lane_i16x8 zeta m) /\
        v (NI.get_lane_i16x8 a1 m) == v (Seq.index iv_l (2*pp+1)) /\
        v (NI.get_lane_i16x8 b1 m) == v (Seq.index iv_r (2*pp+1)))
      (ensures NS.is_i16b 3328 (NI.get_lane_i16x8 fst k) /\
        v (NI.get_lane_i16x8 fst k) % 3329 ==
          ((v (Seq.index iv_l (2*pp)) * v (Seq.index iv_r (2*pp)) +
            v (Seq.index iv_l (2*pp+1)) * v (Seq.index iv_r (2*pp+1)) * v (NI.get_lane_i16x8 zeta m) * 169) * 169) % 3329) =
    reveal_opaque (`%rt_ok) (rt_ok flo fhi k);
    lemma_nttmul_redcong flo fhi fst k (NI.i16x2_as_i32 (NI.get_lane_i16x8 flo k) (NI.get_lane_i16x8 fhi k));
    lemma_nttmul_even_chain (v (Seq.index iv_l (2*pp)) * v (Seq.index iv_r (2*pp)))
      (v (NI.get_lane_i16x8 a1b1 m)) (v (NI.get_lane_i16x8 zeta m))
      (v (Seq.index iv_l (2*pp+1)) * v (Seq.index iv_r (2*pp+1)))
  in
  let od (k pp: nat) : Lemma
      (requires k < 8 /\ 2*pp+1 < 16 /\
        rt_ok slo shi k /\
        v (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo k) (NI.get_lane_i16x8 shi k)) ==
          v (Seq.index iv_l (2*pp)) * v (Seq.index iv_r (2*pp+1)) + v (Seq.index iv_l (2*pp+1)) * v (Seq.index iv_r (2*pp)))
      (ensures NS.is_i16b 3328 (NI.get_lane_i16x8 snd k) /\
        v (NI.get_lane_i16x8 snd k) % 3329 ==
          ((v (Seq.index iv_l (2*pp)) * v (Seq.index iv_r (2*pp+1)) + v (Seq.index iv_l (2*pp+1)) * v (Seq.index iv_r (2*pp))) * 169) % 3329) =
    reveal_opaque (`%rt_ok) (rt_ok slo shi k);
    lemma_nttmul_redcong slo shi snd k (NI.i16x2_as_i32 (NI.get_lane_i16x8 slo k) (NI.get_lane_i16x8 shi k))
  in
  ev 0 0 0; ev 1 2 4; ev 2 4 1; ev 3 6 5; ev 4 1 2; ev 5 3 6; ev 6 5 3; ev 7 7 7;
  od 0 0; od 1 2; od 2 4; od 3 6; od 4 1; od 5 3; od 6 5; od 7 7
#pop-options

(* Final assembly of the butterfly post from the 16 per-output mod-3329
   congruences + the 16 per-output bounds (clean reveal + per-i bound dispatch). *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_nttmul_assemble (iv_l iv_r ov: t_Array i16 (mk_usize 16)) (z1 z2 z3 z4: i16) : Lemma
  (requires
    NS.is_i16b 3328 (Seq.index ov 0)  /\ NS.is_i16b 3328 (Seq.index ov 1)  /\
    NS.is_i16b 3328 (Seq.index ov 2)  /\ NS.is_i16b 3328 (Seq.index ov 3)  /\
    NS.is_i16b 3328 (Seq.index ov 4)  /\ NS.is_i16b 3328 (Seq.index ov 5)  /\
    NS.is_i16b 3328 (Seq.index ov 6)  /\ NS.is_i16b 3328 (Seq.index ov 7)  /\
    NS.is_i16b 3328 (Seq.index ov 8)  /\ NS.is_i16b 3328 (Seq.index ov 9)  /\
    NS.is_i16b 3328 (Seq.index ov 10) /\ NS.is_i16b 3328 (Seq.index ov 11) /\
    NS.is_i16b 3328 (Seq.index ov 12) /\ NS.is_i16b 3328 (Seq.index ov 13) /\
    NS.is_i16b 3328 (Seq.index ov 14) /\ NS.is_i16b 3328 (Seq.index ov 15) /\
    v (Seq.index ov 0)  % 3329 == ((v (Seq.index iv_l 0)  * v (Seq.index iv_r 0)  + v (Seq.index iv_l 1)  * v (Seq.index iv_r 1)  * (v z1)     * 169) * 169) % 3329 /\
    v (Seq.index ov 1)  % 3329 == ((v (Seq.index iv_l 0)  * v (Seq.index iv_r 1)  + v (Seq.index iv_l 1)  * v (Seq.index iv_r 0))                    * 169) % 3329 /\
    v (Seq.index ov 2)  % 3329 == ((v (Seq.index iv_l 2)  * v (Seq.index iv_r 2)  + v (Seq.index iv_l 3)  * v (Seq.index iv_r 3)  * (- (v z1)) * 169) * 169) % 3329 /\
    v (Seq.index ov 3)  % 3329 == ((v (Seq.index iv_l 2)  * v (Seq.index iv_r 3)  + v (Seq.index iv_l 3)  * v (Seq.index iv_r 2))                    * 169) % 3329 /\
    v (Seq.index ov 4)  % 3329 == ((v (Seq.index iv_l 4)  * v (Seq.index iv_r 4)  + v (Seq.index iv_l 5)  * v (Seq.index iv_r 5)  * (v z2)     * 169) * 169) % 3329 /\
    v (Seq.index ov 5)  % 3329 == ((v (Seq.index iv_l 4)  * v (Seq.index iv_r 5)  + v (Seq.index iv_l 5)  * v (Seq.index iv_r 4))                    * 169) % 3329 /\
    v (Seq.index ov 6)  % 3329 == ((v (Seq.index iv_l 6)  * v (Seq.index iv_r 6)  + v (Seq.index iv_l 7)  * v (Seq.index iv_r 7)  * (- (v z2)) * 169) * 169) % 3329 /\
    v (Seq.index ov 7)  % 3329 == ((v (Seq.index iv_l 6)  * v (Seq.index iv_r 7)  + v (Seq.index iv_l 7)  * v (Seq.index iv_r 6))                    * 169) % 3329 /\
    v (Seq.index ov 8)  % 3329 == ((v (Seq.index iv_l 8)  * v (Seq.index iv_r 8)  + v (Seq.index iv_l 9)  * v (Seq.index iv_r 9)  * (v z3)     * 169) * 169) % 3329 /\
    v (Seq.index ov 9)  % 3329 == ((v (Seq.index iv_l 8)  * v (Seq.index iv_r 9)  + v (Seq.index iv_l 9)  * v (Seq.index iv_r 8))                    * 169) % 3329 /\
    v (Seq.index ov 10) % 3329 == ((v (Seq.index iv_l 10) * v (Seq.index iv_r 10) + v (Seq.index iv_l 11) * v (Seq.index iv_r 11) * (- (v z3)) * 169) * 169) % 3329 /\
    v (Seq.index ov 11) % 3329 == ((v (Seq.index iv_l 10) * v (Seq.index iv_r 11) + v (Seq.index iv_l 11) * v (Seq.index iv_r 10))                  * 169) % 3329 /\
    v (Seq.index ov 12) % 3329 == ((v (Seq.index iv_l 12) * v (Seq.index iv_r 12) + v (Seq.index iv_l 13) * v (Seq.index iv_r 13) * (v z4)     * 169) * 169) % 3329 /\
    v (Seq.index ov 13) % 3329 == ((v (Seq.index iv_l 12) * v (Seq.index iv_r 13) + v (Seq.index iv_l 13) * v (Seq.index iv_r 12))                  * 169) % 3329 /\
    v (Seq.index ov 14) % 3329 == ((v (Seq.index iv_l 14) * v (Seq.index iv_r 14) + v (Seq.index iv_l 15) * v (Seq.index iv_r 15) * (- (v z4)) * 169) * 169) % 3329 /\
    v (Seq.index ov 15) % 3329 == ((v (Seq.index iv_l 14) * v (Seq.index iv_r 15) + v (Seq.index iv_l 15) * v (Seq.index iv_r 14))                  * 169) % 3329)
  (ensures
    NS.is_i16b_array 3328 ov /\
    NS.ntt_multiply_butterfly_post iv_l iv_r ov z1 z2 z3 z4)
  =
  introduce forall (i: nat{i < 16}). NS.is_i16b 3328 (Seq.index ov i)
  with (if i = 0 then () else if i = 1 then () else if i = 2 then () else if i = 3 then ()
        else if i = 4 then () else if i = 5 then () else if i = 6 then () else if i = 7 then ()
        else if i = 8 then () else if i = 9 then () else if i = 10 then () else if i = 11 then ()
        else if i = 12 then () else if i = 13 then () else if i = 14 then () else ());
  assert (NS.is_i16b_array 3328 ov);
  reveal_opaque (`%NS.ntt_multiply_butterfly_post)
    (NS.ntt_multiply_butterfly_post iv_l iv_r ov z1 z2 z3 z4)
#pop-options

(* array_of_list -> per-lane Seq.index facts, in a high-fuel helper (List.length 16 +
   seq_of_list indexing need fuel ~16; isolated here so the fuel never touches a heavy
   proof).  ntt_multiply discharges the requires definitionally (indexes IS that array). *)
#push-options "--fuel 20 --ifuel 2 --z3rlimit 80"
let lemma_indexes_vals (indexes: t_Array u8 (mk_usize 16)) : Lemma
  (requires
    indexes == Rust_primitives.Hax.array_of_list 16
      [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11;
       mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15])
  (ensures
    Seq.index indexes 0  == mk_u8 0  /\ Seq.index indexes 1  == mk_u8 1  /\
    Seq.index indexes 2  == mk_u8 2  /\ Seq.index indexes 3  == mk_u8 3  /\
    Seq.index indexes 4  == mk_u8 8  /\ Seq.index indexes 5  == mk_u8 9  /\
    Seq.index indexes 6  == mk_u8 10 /\ Seq.index indexes 7  == mk_u8 11 /\
    Seq.index indexes 8  == mk_u8 4  /\ Seq.index indexes 9  == mk_u8 5  /\
    Seq.index indexes 10 == mk_u8 6  /\ Seq.index indexes 11 == mk_u8 7  /\
    Seq.index indexes 12 == mk_u8 12 /\ Seq.index indexes 13 == mk_u8 13 /\
    Seq.index indexes 14 == mk_u8 14 /\ Seq.index indexes 15 == mk_u8 15)
  = ()
#pop-options

(* 16 vqtbl1q index byte values from per-lane Seq.index facts (ntt_multiply supplies those
   via lemma_indexes_vals) + e_vld1q_u8's forall.  Keeps array_of_list normalization off the
   ntt_multiply WP. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_nttmul_index (index: NI.t_e_uint8x16_t) (indexes: t_Array u8 (mk_usize 16)) : Lemma
  (requires
    index == NI.e_vld1q_u8 (indexes <: t_Slice u8) /\ Seq.length indexes == 16 /\
    Seq.index indexes 0  == mk_u8 0  /\ Seq.index indexes 1  == mk_u8 1  /\
    Seq.index indexes 2  == mk_u8 2  /\ Seq.index indexes 3  == mk_u8 3  /\
    Seq.index indexes 4  == mk_u8 8  /\ Seq.index indexes 5  == mk_u8 9  /\
    Seq.index indexes 6  == mk_u8 10 /\ Seq.index indexes 7  == mk_u8 11 /\
    Seq.index indexes 8  == mk_u8 4  /\ Seq.index indexes 9  == mk_u8 5  /\
    Seq.index indexes 10 == mk_u8 6  /\ Seq.index indexes 11 == mk_u8 7  /\
    Seq.index indexes 12 == mk_u8 12 /\ Seq.index indexes 13 == mk_u8 13 /\
    Seq.index indexes 14 == mk_u8 14 /\ Seq.index indexes 15 == mk_u8 15)
  (ensures
    v (NI.get_lane_u8x16 index 0)  == 0  /\ v (NI.get_lane_u8x16 index 1)  == 1  /\
    v (NI.get_lane_u8x16 index 2)  == 2  /\ v (NI.get_lane_u8x16 index 3)  == 3  /\
    v (NI.get_lane_u8x16 index 4)  == 8  /\ v (NI.get_lane_u8x16 index 5)  == 9  /\
    v (NI.get_lane_u8x16 index 6)  == 10 /\ v (NI.get_lane_u8x16 index 7)  == 11 /\
    v (NI.get_lane_u8x16 index 8)  == 4  /\ v (NI.get_lane_u8x16 index 9)  == 5  /\
    v (NI.get_lane_u8x16 index 10) == 6  /\ v (NI.get_lane_u8x16 index 11) == 7  /\
    v (NI.get_lane_u8x16 index 12) == 12 /\ v (NI.get_lane_u8x16 index 13) == 13 /\
    v (NI.get_lane_u8x16 index 14) == 14 /\ v (NI.get_lane_u8x16 index 15) == 15)
  = let idx = NI.e_vld1q_u8 (indexes <: t_Slice u8) in ()
#pop-options

(* zeta-side mirror of lemma_indexes_vals: the 8 zetas array element values from the
   array_of_list construction.  fuel 20 to normalize List.index of the 8-elem literal list. *)
#push-options "--fuel 20 --ifuel 1 --z3rlimit 80"
let lemma_zetas_vals (zetas: t_Array i16 (mk_usize 8)) (zeta1 zeta2 zeta3 zeta4: i16) : Lemma
  (requires
    NS.is_i16b 1664 zeta1 /\ NS.is_i16b 1664 zeta2 /\ NS.is_i16b 1664 zeta3 /\ NS.is_i16b 1664 zeta4 /\
    zetas == Rust_primitives.Hax.array_of_list 8
      [zeta1; zeta3; Rust_primitives.Arithmetic.neg zeta1; Rust_primitives.Arithmetic.neg zeta3;
       zeta2; zeta4; Rust_primitives.Arithmetic.neg zeta2; Rust_primitives.Arithmetic.neg zeta4])
  (ensures
    Seq.index zetas 0 == zeta1 /\ Seq.index zetas 1 == zeta3 /\
    Seq.index zetas 2 == Rust_primitives.Arithmetic.neg zeta1 /\
    Seq.index zetas 3 == Rust_primitives.Arithmetic.neg zeta3 /\
    Seq.index zetas 4 == zeta2 /\ Seq.index zetas 5 == zeta4 /\
    Seq.index zetas 6 == Rust_primitives.Arithmetic.neg zeta2 /\
    Seq.index zetas 7 == Rust_primitives.Arithmetic.neg zeta4)
  = ()
#pop-options

(* zeta-side mirror of lemma_nttmul_index: the 8 get_lane values from per-lane Seq.index
   facts (ntt_multiply supplies those via lemma_zetas_vals) + e_vld1q_s16's forall.  Keeps
   array_of_list normalization off the ntt_multiply WP. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_nttmul_zeta (zeta: NI.t_e_int16x8_t) (zetas: t_Array i16 (mk_usize 8))
    (zeta1 zeta2 zeta3 zeta4: i16) : Lemma
  (requires
    NS.is_i16b 1664 zeta1 /\ NS.is_i16b 1664 zeta2 /\ NS.is_i16b 1664 zeta3 /\ NS.is_i16b 1664 zeta4 /\
    zeta == NI.e_vld1q_s16 (zetas <: t_Slice i16) /\ Seq.length zetas == 8 /\
    Seq.index zetas 0 == zeta1 /\ Seq.index zetas 1 == zeta3 /\
    Seq.index zetas 2 == Rust_primitives.Arithmetic.neg zeta1 /\
    Seq.index zetas 3 == Rust_primitives.Arithmetic.neg zeta3 /\
    Seq.index zetas 4 == zeta2 /\ Seq.index zetas 5 == zeta4 /\
    Seq.index zetas 6 == Rust_primitives.Arithmetic.neg zeta2 /\
    Seq.index zetas 7 == Rust_primitives.Arithmetic.neg zeta4)
  (ensures
    NI.get_lane_i16x8 zeta 0 == zeta1 /\ NI.get_lane_i16x8 zeta 1 == zeta3 /\
    NI.get_lane_i16x8 zeta 2 == Rust_primitives.Arithmetic.neg zeta1 /\
    NI.get_lane_i16x8 zeta 3 == Rust_primitives.Arithmetic.neg zeta3 /\
    NI.get_lane_i16x8 zeta 4 == zeta2 /\ NI.get_lane_i16x8 zeta 5 == zeta4 /\
    NI.get_lane_i16x8 zeta 6 == Rust_primitives.Arithmetic.neg zeta2 /\
    NI.get_lane_i16x8 zeta 7 == Rust_primitives.Arithmetic.neg zeta4)
  = let z = NI.e_vld1q_s16 (zetas <: t_Slice i16) in ()
#pop-options

(* `unfold` wrapper so the t_Array i16 16 -> t_Slice i16 coercion that is_i16b_array needs
   is discharged HERE in a clean top-level context, not inside lemma_nttmul_compute's ensures
   where the ~27-let recompute spine + context_pruning starves the `v (sz 16) <= max_usize`
   VC.  Unfolds back to is_i16b_array for ntt_multiply's trait post. *)
unfold let is_i16b_arr16 (l: nat) (x: t_Array i16 (mk_usize 16))
  = NS.is_i16b_array l (x <: t_Slice i16)

(* The zeta per-lane bound forall, proven in a CLEAN context (only the 8 lane==(+/-)z_j
   facts + z bounds) so the symbolic-i dispatch never drowns in lemma_nttmul_compute's
   ~27-let op-ensures context — montgomery_multiply already hands a1b1's bound forall
   directly, but zeta's must be derived, and that derivation is what saturates inline. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_zeta_bound8 (zeta: NI.t_e_int16x8_t) (z1 z2 z3 z4: i16) : Lemma
  (requires
    NS.is_i16b 1664 z1 /\ NS.is_i16b 1664 z2 /\ NS.is_i16b 1664 z3 /\ NS.is_i16b 1664 z4 /\
    NI.get_lane_i16x8 zeta 0 == z1 /\ NI.get_lane_i16x8 zeta 1 == z3 /\
    NI.get_lane_i16x8 zeta 2 == Rust_primitives.Arithmetic.neg z1 /\
    NI.get_lane_i16x8 zeta 3 == Rust_primitives.Arithmetic.neg z3 /\
    NI.get_lane_i16x8 zeta 4 == z2 /\ NI.get_lane_i16x8 zeta 5 == z4 /\
    NI.get_lane_i16x8 zeta 6 == Rust_primitives.Arithmetic.neg z2 /\
    NI.get_lane_i16x8 zeta 7 == Rust_primitives.Arithmetic.neg z4)
  (ensures forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 zeta i))
  = introduce forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 zeta i)
    with (if i = 0 then () else if i = 1 then () else if i = 2 then () else if i = 3 then ()
          else if i = 4 then () else if i = 5 then () else if i = 6 then () else ())
#pop-options

(* The ENTIRE ntt_multiply functional proof, factored into a top-level Lemma so it gets
   its own rlimit budget (a Lemma VC encodes as implications, immune to the Pure-function
   WP ceiling that ntt_multiply's ~28-let spine otherwise imposes).  ntt_multiply computes
   the locals, supplies the index byte facts (assert_norm + lemma_nttmul_index), and calls
   this once with reflexive construction `requires`. *)
#push-options "--z3rlimit 400 --split_queries always --z3refresh"
(* AVX2 lemma_nttmul_main pattern: take ONLY the source words (lhs rhs) + the two
   already-loaded const vectors (zeta index, with their per-lane value facts) and RECOMPUTE
   the entire ~27-let SIMD spine INTERNALLY, both in the `ensures` `let ... in` and in the
   body.  ntt_multiply's own `res` then matches the ensures' recomputed `res` DEFINITIONALLY
   (delta/iota, not SMT), so ntt_multiply discharges NO op-ensures construction — only the
   cheap per-lane value facts (via lemma_nttmul_index/lemma_nttmul_zeta) + this call.  That
   keeps the per-lane-forall op-ensures soup out of ntt_multiply's Pure-function WP. *)
let lemma_nttmul_compute
      (lhs rhs: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector)
      (zeta: NI.t_e_int16x8_t)
      (index: NI.t_e_uint8x16_t)
      (zeta1 zeta2 zeta3 zeta4: i16) : Lemma
  (requires
    NS.is_i16b_array 3328 (repr lhs) /\ NS.is_i16b_array 3328 (repr rhs) /\
    NS.is_i16b 1664 zeta1 /\ NS.is_i16b 1664 zeta2 /\ NS.is_i16b 1664 zeta3 /\ NS.is_i16b 1664 zeta4 /\
    NI.get_lane_i16x8 zeta 0 == zeta1 /\
    NI.get_lane_i16x8 zeta 1 == zeta3 /\
    NI.get_lane_i16x8 zeta 2 == Rust_primitives.Arithmetic.neg zeta1 /\
    NI.get_lane_i16x8 zeta 3 == Rust_primitives.Arithmetic.neg zeta3 /\
    NI.get_lane_i16x8 zeta 4 == zeta2 /\
    NI.get_lane_i16x8 zeta 5 == zeta4 /\
    NI.get_lane_i16x8 zeta 6 == Rust_primitives.Arithmetic.neg zeta2 /\
    NI.get_lane_i16x8 zeta 7 == Rust_primitives.Arithmetic.neg zeta4 /\
    v (NI.get_lane_u8x16 index 0)  == 0  /\ v (NI.get_lane_u8x16 index 1)  == 1  /\
    v (NI.get_lane_u8x16 index 2)  == 2  /\ v (NI.get_lane_u8x16 index 3)  == 3  /\
    v (NI.get_lane_u8x16 index 4)  == 8  /\ v (NI.get_lane_u8x16 index 5)  == 9  /\
    v (NI.get_lane_u8x16 index 6)  == 10 /\ v (NI.get_lane_u8x16 index 7)  == 11 /\
    v (NI.get_lane_u8x16 index 8)  == 4  /\ v (NI.get_lane_u8x16 index 9)  == 5  /\
    v (NI.get_lane_u8x16 index 10) == 6  /\ v (NI.get_lane_u8x16 index 11) == 7  /\
    v (NI.get_lane_u8x16 index 12) == 12 /\ v (NI.get_lane_u8x16 index 13) == 13 /\
    v (NI.get_lane_u8x16 index 14) == 14 /\ v (NI.get_lane_u8x16 index 15) == 15)
  (ensures
    (let a0 = NI.e_vtrn1q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                             lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
     let a1 = NI.e_vtrn2q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                             lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
     let b0 = NI.e_vtrn1q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                             rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
     let b1 = NI.e_vtrn2q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                             rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
     let a1b1 = Libcrux_ml_kem.Vector.Neon.Arithmetic.montgomery_multiply_int16x8_t a1 b1 in
     let a1b1_low = NI.e_vmull_s16 (NI.e_vget_low_s16 a1b1) (NI.e_vget_low_s16 zeta) in
     let a1b1_high = NI.e_vmull_high_s16 a1b1 zeta in
     let fst_low = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_s16 a1b1_low (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b0)) in
     let fst_high = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_high_s16 a1b1_high a0 b0) in
     let a0b1_low = NI.e_vmull_s16 (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b1) in
     let a0b1_high = NI.e_vmull_high_s16 a0 b1 in
     let snd_low = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_s16 a0b1_low (NI.e_vget_low_s16 a1) (NI.e_vget_low_s16 b0)) in
     let snd_high = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_high_s16 a0b1_high a1 b0) in
     let fst_low16 = NI.e_vtrn1q_s16 fst_low fst_high in
     let fst_high16 = NI.e_vtrn2q_s16 fst_low fst_high in
     let snd_low16 = NI.e_vtrn1q_s16 snd_low snd_high in
     let snd_high16 = NI.e_vtrn2q_s16 snd_low snd_high in
     let fst = Libcrux_ml_kem.Vector.Neon.Arithmetic.montgomery_reduce_int16x8_t fst_low16 fst_high16 in
     let snd = Libcrux_ml_kem.Vector.Neon.Arithmetic.montgomery_reduce_int16x8_t snd_low16 snd_high16 in
     let low0 = NI.e_vreinterpretq_s32_s16 (NI.e_vtrn1q_s16 fst snd) in
     let high0 = NI.e_vreinterpretq_s32_s16 (NI.e_vtrn2q_s16 fst snd) in
     let low1 = NI.e_vreinterpretq_s16_s32 (NI.e_vtrn1q_s32 low0 high0) in
     let high1 = NI.e_vreinterpretq_s16_s32 (NI.e_vtrn2q_s32 low0 high0) in
     let low2 = NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 low1) index) in
     let high2 = NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 high1) index) in
     let res = ({ Libcrux_ml_kem.Vector.Neon.Vector_type.f_low = low2;
                  Libcrux_ml_kem.Vector.Neon.Vector_type.f_high = high2 }
                <: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector) in
     is_i16b_arr16 3328 (repr res) /\
     NS.ntt_multiply_butterfly_post (repr lhs) (repr rhs) (repr res) zeta1 zeta2 zeta3 zeta4))
  = let a0 = NI.e_vtrn1q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                            lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let a1 = NI.e_vtrn2q_s16 lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                            lhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let b0 = NI.e_vtrn1q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                            rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let b1 = NI.e_vtrn2q_s16 rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low
                            rhs.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high in
    let a1b1 = Libcrux_ml_kem.Vector.Neon.Arithmetic.montgomery_multiply_int16x8_t a1 b1 in
    let a1b1_low = NI.e_vmull_s16 (NI.e_vget_low_s16 a1b1) (NI.e_vget_low_s16 zeta) in
    let a1b1_high = NI.e_vmull_high_s16 a1b1 zeta in
    let fst_low = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_s16 a1b1_low (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b0)) in
    let fst_high = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_high_s16 a1b1_high a0 b0) in
    let a0b1_low = NI.e_vmull_s16 (NI.e_vget_low_s16 a0) (NI.e_vget_low_s16 b1) in
    let a0b1_high = NI.e_vmull_high_s16 a0 b1 in
    let snd_low = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_s16 a0b1_low (NI.e_vget_low_s16 a1) (NI.e_vget_low_s16 b0)) in
    let snd_high = NI.e_vreinterpretq_s16_s32 (NI.e_vmlal_high_s16 a0b1_high a1 b0) in
    let fst_low16 = NI.e_vtrn1q_s16 fst_low fst_high in
    let fst_high16 = NI.e_vtrn2q_s16 fst_low fst_high in
    let snd_low16 = NI.e_vtrn1q_s16 snd_low snd_high in
    let snd_high16 = NI.e_vtrn2q_s16 snd_low snd_high in
    let fst = Libcrux_ml_kem.Vector.Neon.Arithmetic.montgomery_reduce_int16x8_t fst_low16 fst_high16 in
    let snd = Libcrux_ml_kem.Vector.Neon.Arithmetic.montgomery_reduce_int16x8_t snd_low16 snd_high16 in
    let low0 = NI.e_vreinterpretq_s32_s16 (NI.e_vtrn1q_s16 fst snd) in
    let high0 = NI.e_vreinterpretq_s32_s16 (NI.e_vtrn2q_s16 fst snd) in
    let low1 = NI.e_vreinterpretq_s16_s32 (NI.e_vtrn1q_s32 low0 high0) in
    let high1 = NI.e_vreinterpretq_s16_s32 (NI.e_vtrn2q_s32 low0 high0) in
    let low2 = NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 low1) index) in
    let high2 = NI.e_vreinterpretq_s16_u8 (NI.e_vqtbl1q_u8 (NI.e_vreinterpretq_u8_s16 high1) index) in
    let res = ({ Libcrux_ml_kem.Vector.Neon.Vector_type.f_low = low2;
                 Libcrux_ml_kem.Vector.Neon.Vector_type.f_high = high2 }
               <: Libcrux_ml_kem.Vector.Neon.Vector_type.t_SIMD128Vector) in
    assert_norm (pow2 15 == 32768);
    assert_norm (pow2 31 == 2147483648);
    lemma_nttmul_in 3328 lhs rhs a1 b1 zeta zeta1 zeta2 zeta3 zeta4;
    introduce forall (i: nat{i < 8}) . Spec.Utils.is_intb (3326 * pow2 15)
      (v (NI.get_lane_i16x8 a1 i) * v (NI.get_lane_i16x8 b1 i))
    with Spec.Utils.lemma_mul_i16b 3328 3328 (NI.get_lane_i16x8 a1 i) (NI.get_lane_i16x8 b1 i);
    assert (forall (i: nat{i < 8}).
          v (NI.get_lane_i16x8 a1b1 i) % 3329 ==
          (v (NI.get_lane_i16x8 a1 i) * v (NI.get_lane_i16x8 b1 i) * 169) % 3329);
    assert (forall (i: nat{i < 8}). NS.is_i16b 3328 (NI.get_lane_i16x8 a1b1 i));
    lemma_zeta_bound8 zeta zeta1 zeta2 zeta3 zeta4;
    lemma_nttmul_montval_fst lhs rhs a0 b0 a1b1 zeta a1b1_low a1b1_high
      fst_low fst_high fst_low16 fst_high16;
    lemma_nttmul_montval_snd lhs rhs a0 a1 b0 b1 a0b1_low a0b1_high
      snd_low snd_high snd_low16 snd_high16;
    lemma_nttmul_fstsnd (repr lhs) (repr rhs) a1 b1 a1b1 zeta fst_low16 fst_high16 snd_low16
      snd_high16 fst snd zeta1 zeta2 zeta3 zeta4;
    lemma_nttmul_out fst snd low1 high1 low2 high2 low0 high0 index;
    lemma_nttmul_assemble (repr lhs) (repr rhs) (repr res) zeta1 zeta2 zeta3 zeta4
#pop-options
"#
)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\
                            Spec.Utils.is_i16b 1664 zeta3 /\ Spec.Utils.is_i16b 1664 zeta4 /\
                            Spec.Utils.is_i16b_array 3328 (repr ${lhs}) /\
                            Spec.Utils.is_i16b_array 3328 (repr ${rhs})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (repr ${result}) /\
    Spec.Utils.ntt_multiply_butterfly_post (repr ${lhs}) (repr ${rhs}) (repr ${result})
      zeta1 zeta2 zeta3 zeta4"#))]
pub(crate) fn ntt_multiply(
    lhs: &SIMD128Vector,
    rhs: &SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do for pairs of two elements:
    // montgomery_reduce(a0 * b0 + montgomery_reduce(a1 * b1) * zeta),
    // montgomery_reduce(a0 * b1 + a1 * b0)
    //let lhsp = crate::simd::portable::from_i16_array(to_i16_array(lhs.clone()));
    //let rhsp = crate::simd::portable::from_i16_array(to_i16_array(rhs.clone()));
    //let mulp = crate::simd::portable::ntt_multiply(&lhsp,&rhsp,zeta0,zeta1);
    //from_i16_array(crate::simd::portable::to_i16_array(mulp))

    let zetas: [i16; 8] = [zeta1, zeta3, -zeta1, -zeta3, zeta2, zeta4, -zeta2, -zeta4];
    let zeta = _vld1q_s16(&zetas);

    let a0 = _vtrn1q_s16(lhs.low, lhs.high); // a0, a8, a2, a10, ...
    let a1 = _vtrn2q_s16(lhs.low, lhs.high); // a1, a9, a3, a11, ...
    let b0 = _vtrn1q_s16(rhs.low, rhs.high); // b0, b8, b2, b10, ...
    let b1 = _vtrn2q_s16(rhs.low, rhs.high); // b1, b9, b3, b11, ...

    let a1b1 = montgomery_multiply_int16x8_t(a1, b1);
    let a1b1_low = _vmull_s16(_vget_low_s16(a1b1), _vget_low_s16(zeta)); // a1b1z, a9b9z, a3b3z, a11b11z
    let a1b1_high = _vmull_high_s16(a1b1, zeta); // a5b5z, a13b13z, a7b7z, a15b15z

    let fst_low =
        _vreinterpretq_s16_s32(_vmlal_s16(a1b1_low, _vget_low_s16(a0), _vget_low_s16(b0))); // 0, 8, 2, 10
    let fst_high = _vreinterpretq_s16_s32(_vmlal_high_s16(a1b1_high, a0, b0)); // 4, 12, 6, 14

    let a0b1_low = _vmull_s16(_vget_low_s16(a0), _vget_low_s16(b1));
    let a0b1_high = _vmull_high_s16(a0, b1);

    let snd_low =
        _vreinterpretq_s16_s32(_vmlal_s16(a0b1_low, _vget_low_s16(a1), _vget_low_s16(b0))); // 1, 9, 3, 11
    let snd_high = _vreinterpretq_s16_s32(_vmlal_high_s16(a0b1_high, a1, b0)); // 5, 13, 7, 15

    let fst_low16 = _vtrn1q_s16(fst_low, fst_high); // 0,4,8,12,2,6,10,14
    let fst_high16 = _vtrn2q_s16(fst_low, fst_high);
    let snd_low16 = _vtrn1q_s16(snd_low, snd_high); // 1,5,9,13,3,7,11,15
    let snd_high16 = _vtrn2q_s16(snd_low, snd_high);

    let fst = montgomery_reduce_int16x8_t(fst_low16, fst_high16); // 0,4,8,12,2,6,10,14
    let snd = montgomery_reduce_int16x8_t(snd_low16, snd_high16); // 1,5,9,13,3,7,11,15

    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(fst, snd)); // 0,1,8,9,2,3,10,11
    let high0 = _vreinterpretq_s32_s16(_vtrn2q_s16(fst, snd)); // 4,5,12,13,6,7,14,15

    let low1 = _vreinterpretq_s16_s32(_vtrn1q_s32(low0, high0)); // 0,1,4,5,2,3,6,7
    let high1 = _vreinterpretq_s16_s32(_vtrn2q_s32(low0, high0)); // 8,9,12,13,10,11,14,15

    let indexes: [u8; 16] = [0, 1, 2, 3, 8, 9, 10, 11, 4, 5, 6, 7, 12, 13, 14, 15];
    let index = _vld1q_u8(&indexes);
    let low2 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(low1), index));
    let high2 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(high1), index));

    let res = SIMD128Vector {
        low: low2,
        high: high2,
    };
    hax_lib::fstar!(
        r#"lemma_indexes_vals ${indexes};
    lemma_nttmul_index ${index} ${indexes};
    lemma_zetas_vals ${zetas} zeta1 zeta2 zeta3 zeta4;
    lemma_nttmul_zeta ${zeta} ${zetas} zeta1 zeta2 zeta3 zeta4;
    lemma_nttmul_compute ${lhs} ${rhs} ${zeta} ${index} zeta1 zeta2 zeta3 zeta4"#
    );
    res
}
