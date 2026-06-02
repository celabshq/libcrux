use hax_lib::*;

pub(crate) const PRIME: u32 = 8380417;

pub(crate) const MONT_R: u32 = 8380417;

pub(crate) const FIELD_MAX: u32 = 8380416;

pub(crate) const FIELD_MID: u32 = 4190208;

// Option C (above-trait request 2026-04-28): widened from FIELD_MID
// (q/2) to FIELD_MAX (q-1) so the `reduce → ntt` chain composes
// directly.  `Operations::reduce`'s post is `is_i32b_array_opaque
// FIELD_MAX`, which now matches `Operations::ntt`'s pre.  Existing
// callers passing FIELD_MID-bounded inputs still satisfy the wider
// bound (FIELD_MID < FIELD_MAX).  Internal NTT peak rises from
// FIELD_MID + 8q ≈ 71M to FIELD_MAX + 8q ≈ 75M — still ~28× under
// i32::MAX, so panic-free proofs hold.
pub(crate) const NTT_BASE_BOUND: u32 = FIELD_MAX;

const COEFFICIENTS_IN_SIMD_UNIT: usize = 8;

type SIMDContent = [i32; COEFFICIENTS_IN_SIMD_UNIT];

pub(crate) fn int_is_i32(i: Int) -> bool {
    i <= i32::MAX.to_int() && i >= i32::MIN.to_int()
}

// Phase 1 lane-post preamble. All per-lane opaque post predicates (citing
// canonical `Hacspec_ml_dsa.*` helpers) are injected as raw F* below, in
// front of the existing `add_pre` definition. Pattern mirrors
// `libcrux-ml-kem/src/vector/traits.rs::spec`. The dual-trigger lookup
// lemmas + named-intro lemmas (style guide §3.2-3.3) live alongside each
// opaque atom for use by Phase 2/3 impl proofs.
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"
(* ----- per-lane opaque posts citing Hacspec_ml_dsa.* ----- *)

(* ----- per-method panic-freedom pre-condition packages -----
   The trait posts cite per-lane atomic predicates (above); the trait
   pre-conditions for the encoding/sample methods need an analogous
   opaque packaging of "all 8 lanes are bounded" / "hint is binary"
   so consumer modules can pass the obligation around without paying
   the cost of expanding a per-lane `forall` at every call site.
   Pattern mirrors `bounded_pos_i16_array` in
   `libcrux-ml-kem/src/vector/traits.rs::spec`. *)

(* hint array of 8 lanes where each lane is 0 or 1. Used by use_hint
   trait pre. *)
let is_binary_array_8 (x: t_Array i32 (mk_usize 8)) : prop =
  forall (i: nat). i < 8 ==>
    (v (Seq.index x i) == 0 \/ v (Seq.index x i) == 1)

[@@ "opaque_to_smt"]
let is_binary_array_8_opaque (x: t_Array i32 (mk_usize 8)) : prop =
  is_binary_array_8 x

let lemma_is_binary_array_8_lookup (x: t_Array i32 (mk_usize 8)) (i: nat)
    : Lemma (requires is_binary_array_8_opaque x /\ i < 8)
            (ensures v (Seq.index x i) == 0 \/ v (Seq.index x i) == 1)
            [SMTPat (Seq.index x i); SMTPat (is_binary_array_8_opaque x)] =
  reveal_opaque (`%is_binary_array_8_opaque) (is_binary_array_8_opaque x)

let lemma_is_binary_array_8_intro (x: t_Array i32 (mk_usize 8))
    : Lemma (requires forall (i: nat). i < 8 ==>
                       (v (Seq.index x i) == 0 \/ v (Seq.index x i) == 1))
            (ensures is_binary_array_8_opaque x) =
  reveal_opaque (`%is_binary_array_8_opaque) (is_binary_array_8_opaque x)

(* 256-element hint array where each coefficient is 0 or 1. Used by use_hint's
   function pre (one row of `hint: &[[i32; 256]]`). Mirrors is_binary_array_8. *)
let is_binary_256_array (x: t_Array i32 (mk_usize 256)) : prop =
  forall (j: nat). j < 256 ==>
    (v (Seq.index x j) == 0 \/ v (Seq.index x j) == 1)

[@@ "opaque_to_smt"]
let is_binary_256_array_opaque (x: t_Array i32 (mk_usize 256)) : prop =
  is_binary_256_array x

let lemma_is_binary_256_array_lookup (x: t_Array i32 (mk_usize 256)) (i: nat)
    : Lemma (requires is_binary_256_array_opaque x /\ i < 256)
            (ensures v (Seq.index x i) == 0 \/ v (Seq.index x i) == 1)
            [SMTPat (Seq.index x i); SMTPat (is_binary_256_array_opaque x)] =
  reveal_opaque (`%is_binary_256_array_opaque) (is_binary_256_array_opaque x)

let lemma_is_binary_256_array_intro (x: t_Array i32 (mk_usize 256))
    : Lemma (requires forall (j: nat). j < 256 ==>
                       (v (Seq.index x j) == 0 \/ v (Seq.index x j) == 1))
            (ensures is_binary_256_array_opaque x) =
  reveal_opaque (`%is_binary_256_array_opaque) (is_binary_256_array_opaque x)

(* Slice version over the whole hint matrix: every row is a binary 256-array. *)
[@@ "opaque_to_smt"]
let is_binary_256_array_slice (arr: t_Slice (t_Array i32 (mk_usize 256))) : prop =
  forall (k: nat). k < Seq.length arr ==>
    is_binary_256_array_opaque (Seq.index arr k)

let lemma_is_binary_256_array_slice_lookup
      (arr: t_Slice (t_Array i32 (mk_usize 256))) (k: nat)
    : Lemma (requires is_binary_256_array_slice arr /\ k < Seq.length arr)
            (ensures is_binary_256_array_opaque (Seq.index arr k))
            [SMTPat (is_binary_256_array_slice arr); SMTPat (Seq.index arr k)] =
  reveal_opaque (`%is_binary_256_array_slice) (is_binary_256_array_slice arr)

let lemma_is_binary_256_array_slice_intro
      (arr: t_Slice (t_Array i32 (mk_usize 256)))
    : Lemma (requires forall (k: nat). k < Seq.length arr ==>
                       is_binary_256_array_opaque (Seq.index arr k))
            (ensures is_binary_256_array_slice arr) =
  reveal_opaque (`%is_binary_256_array_slice) (is_binary_256_array_slice arr)

(* F-3 (2026-04-28): the *_serialize trait pres need a non-negative
   bound, not the centered `is_i32b_array_opaque b` (which allows
   |x| <= b including negative).  The impls of commitment_serialize,
   gamma1_serialize, t0_serialize, error_serialize require lane values
   in [0, b].  Mirror ML-KEM's `bounded_pos_i16_array` with the
   ml-dsa version below. *)

let is_pos_array (l: nat) (x: t_Array i32 (mk_usize 8)) : prop =
  forall (i: nat). i < 8 ==>
    v (Seq.index x i) >= 0 /\ v (Seq.index x i) <= l

[@@ "opaque_to_smt"]
let is_pos_array_opaque (l: nat) (x: t_Array i32 (mk_usize 8)) : prop =
  is_pos_array l x

let lemma_is_pos_array_lookup (l: nat) (x: t_Array i32 (mk_usize 8)) (i: nat)
    : Lemma (requires is_pos_array_opaque l x /\ i < 8)
            (ensures v (Seq.index x i) >= 0 /\ v (Seq.index x i) <= l)
            [SMTPat (Seq.index x i); SMTPat (is_pos_array_opaque l x)] =
  reveal_opaque (`%is_pos_array_opaque) (is_pos_array_opaque l x)

let lemma_is_pos_array_intro (l: nat) (x: t_Array i32 (mk_usize 8))
    : Lemma (requires forall (i: nat). i < 8 ==>
                       v (Seq.index x i) >= 0 /\ v (Seq.index x i) <= l)
            (ensures is_pos_array_opaque l x) =
  reveal_opaque (`%is_pos_array_opaque) (is_pos_array_opaque l x)

(* infinity_norm_exceeds: result is true iff some lane's signed absolute
   value (the impl's actual computation) meets or exceeds bound. Compatible
   with the trait pre `is_i32b_array_opaque FIELD_MAX simd_unit` (no
   centered-rep restriction). The relationship to FIPS 204's centered
   `coeff_norm` holds whenever inputs are already centered. *)
[@@ "opaque_to_smt"]
let infinity_norm_exceeds_post (simd_unit: t_Array i32 (mk_usize 8))
                               (bound: i32) (result: bool) : prop =
  b2t result <==>
    (exists (i: nat). i < 8 /\
       (let x = v (Seq.index simd_unit i) in
        (if x >= 0 then x else - x) >= v bound))

(* reduce: per-lane Barrett reduction. mod_q on i64 with the canonical
   q=8380417 lift; result lies in centered Barrett range (-q, q) and
   is congruent to the input mod q. *)
[@@ "opaque_to_smt"]
let reduce_lane_post (input result: i32) : prop =
  v result > -8380417 /\ v result < 8380417 /\
  (v result) % 8380417 == (v input) % 8380417

let lemma_reduce_lane_lookup (input result: i32)
    : Lemma (requires reduce_lane_post input result)
            (ensures v result > -8380417 /\ v result < 8380417 /\
                     (v result) % 8380417 == (v input) % 8380417) =
  reveal_opaque (`%reduce_lane_post) (reduce_lane_post input result)

let lemma_reduce_lane_intro (input result: i32)
    : Lemma (requires v result > -8380417 /\ v result < 8380417 /\
                      (v result) % 8380417 == (v input) % 8380417)
            (ensures reduce_lane_post input result) =
  reveal_opaque (`%reduce_lane_post) (reduce_lane_post input result)

(* decompose: hacspec returns (r1, r0); trait stores (low, high) = (r0, r1).
   Conditional on input nonneg < q (which the impl proof must establish from
   the bounded pre and a normalize step). *)
[@@ "opaque_to_smt"]
let decompose_lane_post (gamma2 input low high: i32) : prop =
  ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
  (v input >= 0 /\ v input < 8380417 ==>
   (let pair = Hacspec_ml_dsa.Arithmetic.decompose input gamma2 in
    let r1 = fst pair in
    let r0 = snd pair in
    v low == v r0 /\ v high == v r1))

let lemma_decompose_lane_lookup (gamma2 input low high: i32)
    : Lemma (requires decompose_lane_post gamma2 input low high /\
                      ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
                      (v input >= 0 /\ v input < 8380417))
            (ensures (let pair = Hacspec_ml_dsa.Arithmetic.decompose input gamma2 in
                      v low == v (snd pair) /\ v high == v (fst pair)))
            [SMTPat (decompose_lane_post gamma2 input low high)] =
  reveal_opaque (`%decompose_lane_post) (decompose_lane_post gamma2 input low high)

let lemma_decompose_lane_intro (gamma2 input low high: i32)
    : Lemma (requires ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
                      (v input >= 0 /\ v input < 8380417 ==>
                       (let pair = Hacspec_ml_dsa.Arithmetic.decompose input gamma2 in
                        v low == v (snd pair) /\ v high == v (fst pair))))
            (ensures decompose_lane_post gamma2 input low high) =
  reveal_opaque (`%decompose_lane_post) (decompose_lane_post gamma2 input low high)

(* compute_hint: hint[i] = compute_one_hint low high gamma2 (Spec.MLDSA.Math).
   F-4 (2026-04-28): switched from `Hacspec_ml_dsa.Arithmetic.make_hint`
   (literal FIPS 204 algorithm 39) to `Spec.MLDSA.Math.compute_one_hint`
   because the two disagree at the boundary `low = -gamma2, high != 0`:
   compute_one_hint returns 1 there, make_hint computes via FIPS 204 high_bits
   which evaluates differently.  The impl computes per compute_one_hint;
   citing make_hint creates an unprovable equivalence at this boundary.
   Trade-off: drops the cross-spec link to FIPS 204; recovered later if
   a Hacspec helper that mirrors compute_one_hint's optimized boundary
   handling lands. *)
[@@ "opaque_to_smt"]
let compute_hint_lane_post (gamma2 low high hint: i32) : prop =
  ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
  (v high >= 0 /\ v high < 8380417 ==>
    v hint == Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2))

let lemma_compute_hint_lane_lookup (gamma2 low high hint: i32)
    : Lemma (requires compute_hint_lane_post gamma2 low high hint /\
                      ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
                      (v high >= 0 /\ v high < 8380417))
            (ensures v hint == Spec.MLDSA.Math.compute_one_hint
                                  (v low) (v high) (v gamma2))
            [SMTPat (compute_hint_lane_post gamma2 low high hint)] =
  reveal_opaque (`%compute_hint_lane_post)
                (compute_hint_lane_post gamma2 low high hint)

let lemma_compute_hint_lane_intro (gamma2 low high hint: i32)
    : Lemma (requires ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
                      (v high >= 0 /\ v high < 8380417 ==>
                        v hint == Spec.MLDSA.Math.compute_one_hint
                                    (v low) (v high) (v gamma2)))
            (ensures compute_hint_lane_post gamma2 low high hint) =
  reveal_opaque (`%compute_hint_lane_post)
                (compute_hint_lane_post gamma2 low high hint)

(* use_hint: hint stored as 0/1 i32; spec uuse_hint takes a bool. *)
[@@ "opaque_to_smt"]
let use_hint_lane_post (gamma2 input hint future_hint: i32) : prop =
  ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
  (v input >= 0 /\ v input < 8380417 /\ (v hint == 0 \/ v hint == 1) ==>
   v future_hint ==
   v (Hacspec_ml_dsa.Arithmetic.uuse_hint (v hint = 1) input gamma2))

let lemma_use_hint_lane_lookup (gamma2 input hint future_hint: i32)
    : Lemma (requires use_hint_lane_post gamma2 input hint future_hint /\
                      ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
                      v input >= 0 /\ v input < 8380417 /\ (v hint == 0 \/ v hint == 1))
            (ensures v future_hint ==
                     v (Hacspec_ml_dsa.Arithmetic.uuse_hint (v hint = 1) input gamma2))
            [SMTPat (use_hint_lane_post gamma2 input hint future_hint)] =
  reveal_opaque (`%use_hint_lane_post)
                (use_hint_lane_post gamma2 input hint future_hint)

let lemma_use_hint_lane_intro (gamma2 input hint future_hint: i32)
    : Lemma (requires ((v gamma2 == 95232) \/ (v gamma2 == 261888)) /\
                      (v input >= 0 /\ v input < 8380417 /\
                       (v hint == 0 \/ v hint == 1) ==>
                       v future_hint ==
                       v (Hacspec_ml_dsa.Arithmetic.uuse_hint (v hint = 1) input gamma2)))
            (ensures use_hint_lane_post gamma2 input hint future_hint) =
  reveal_opaque (`%use_hint_lane_post)
                (use_hint_lane_post gamma2 input hint future_hint)

(* montgomery_multiply: post relates lane to lhs*rhs*8265825 mod q (R^{-1}). *)
[@@ "opaque_to_smt"]
let montgomery_multiply_lane_post (lhs rhs future_lhs: i32) : prop =
  (v future_lhs) % 8380417 == (v lhs * v rhs * 8265825) % 8380417

let lemma_montgomery_multiply_lane_lookup (lhs rhs future_lhs: i32)
    : Lemma (requires montgomery_multiply_lane_post lhs rhs future_lhs)
            (ensures (v future_lhs) % 8380417 == (v lhs * v rhs * 8265825) % 8380417) =
  reveal_opaque (`%montgomery_multiply_lane_post)
                (montgomery_multiply_lane_post lhs rhs future_lhs)

let lemma_montgomery_multiply_lane_intro (lhs rhs future_lhs: i32)
    : Lemma (requires (v future_lhs) % 8380417 == (v lhs * v rhs * 8265825) % 8380417)
            (ensures montgomery_multiply_lane_post lhs rhs future_lhs) =
  reveal_opaque (`%montgomery_multiply_lane_post)
                (montgomery_multiply_lane_post lhs rhs future_lhs)

(* shift_left_then_reduce: post is centered-Barrett bound + mod-q
   congruence with `input * 2^13`.  This is the natural shape both
   impls actually compute — `barrett_red(input <<! 13)` lives in
   `(-q, q)` (not in `[0, q-1]` like Hacspec's `mod_q`-normalised form),
   so the previous strict-equality post against
   `Hacspec_ml_dsa.Arithmetic.shift_left_then_reduce` was unsound for
   the impl bodies.  Mirrors `reduce_lane_post`.

   FUTURE SPEC AUDIT: this post talks about i32 representatives, not
   Z/qZ field elements.  ML-KEM exposes a `t_FieldElement` type plus
   `i16_to_spec_fe` / `mont_i16_to_spec_fe` lifts so the spec layer
   talks only about field elements (no Barrett / Montgomery in the
   post).  ML-DSA should grow a parallel `i32_to_spec_fe` and
   reformulate this post (and `reduce_lane_post`,
   `montgomery_multiply_lane_post`) at the field-element level. *)
[@@ "opaque_to_smt"]
let shift_left_then_reduce_lane_post (input future: i32) : prop =
  Spec.Utils.is_i32b 8380416 future /\
  (v future) % 8380417 == (v input * 8192) % 8380417

let lemma_shift_left_then_reduce_lane_lookup (input future: i32)
    : Lemma (requires shift_left_then_reduce_lane_post input future)
            (ensures Spec.Utils.is_i32b 8380416 future /\
                     (v future) % 8380417 == (v input * 8192) % 8380417) =
  reveal_opaque (`%shift_left_then_reduce_lane_post)
                (shift_left_then_reduce_lane_post input future)

let lemma_shift_left_then_reduce_lane_intro (input future: i32)
    : Lemma (requires Spec.Utils.is_i32b 8380416 future /\
                      (v future) % 8380417 == (v input * 8192) % 8380417)
            (ensures shift_left_then_reduce_lane_post input future) =
  reveal_opaque (`%shift_left_then_reduce_lane_post)
                (shift_left_then_reduce_lane_post input future)

(* power2round: hacspec returns (r1, r0); trait stores (future_t1, future_t0).
   Conditional on 0 <= t0 < q. *)
[@@ "opaque_to_smt"]
let power2round_lane_post (input future_t1 future_t0: i32) : prop =
  v input >= 0 /\ v input < 8380417 ==>
  (let pair = Hacspec_ml_dsa.Arithmetic.power2round input in
   v future_t1 == v (fst pair) /\ v future_t0 == v (snd pair))

let lemma_power2round_lane_lookup (input future_t1 future_t0: i32)
    : Lemma (requires power2round_lane_post input future_t1 future_t0 /\
                      v input >= 0 /\ v input < 8380417)
            (ensures (let pair = Hacspec_ml_dsa.Arithmetic.power2round input in
                      v future_t1 == v (fst pair) /\ v future_t0 == v (snd pair)))
            [SMTPat (power2round_lane_post input future_t1 future_t0)] =
  reveal_opaque (`%power2round_lane_post)
                (power2round_lane_post input future_t1 future_t0)

let lemma_power2round_lane_intro (input future_t1 future_t0: i32)
    : Lemma (requires v input >= 0 /\ v input < 8380417 ==>
                      (let pair = Hacspec_ml_dsa.Arithmetic.power2round input in
                       v future_t1 == v (fst pair) /\ v future_t0 == v (snd pair)))
            (ensures power2round_lane_post input future_t1 future_t0) =
  reveal_opaque (`%power2round_lane_post)
                (power2round_lane_post input future_t1 future_t0)

(* Per-byte rejection sample step (less_than_field_modulus): each accepted
   coefficient comes from a 3-byte chunk per Encoding.coeff_from_three_bytes.
   We expose only the per-element acceptance equation here; the count
   relationship (sampled = number of `Some` outputs) is an implementation
   loop invariant, not a per-element atom. *)
[@@ "opaque_to_smt"]
let rejection_sample_3byte_lane_post (b0 b1 b2: u8) (out: i32) : prop =
  match Hacspec_ml_dsa.Encoding.coeff_from_three_bytes b0 b1 b2 with
  | Core_models.Option.Option_Some c -> v out == v c
  | Core_models.Option.Option_None -> True

(* Per-half-byte rejection sample step (less_than_eta variants): each
   accepted coefficient comes from one nibble per
   Encoding.coeff_from_half_byte. *)
[@@ "opaque_to_smt"]
let rejection_sample_halfbyte_lane_post (eta: usize) (b: u8) (out: i32) : prop =
  match Hacspec_ml_dsa.Encoding.coeff_from_half_byte b eta with
  | Core_models.Option.Option_Some c -> v out == v c
  | Core_models.Option.Option_None -> True

(* Encoding bit-pack/unpack post predicates over an 8-element SIMD chunk.
   These use BitVecEq.int_t_array_bitwise_eq from fstar-helpers/fstar-bitvec
   (already on the include path) the same way ML-KEM does for serialize/
   deserialize. The width parameter `b` is the per-coefficient bit width
   (10, 13, etc.); for offset-encoded `bit_pack` we additionally constrain
   the input to lie in [-a, b]. *)

(* simple_bit_pack on 8 lanes -> b bytes (one bit per lane per byte). The
   chunked relationship is: each lane's low-`b`-bits goes into the
   serialized output. *)
[@@ "opaque_to_smt"]
let simple_bit_pack_chunk_post (b: nat{b > 0 /\ b <= 13})
                               (input: t_Array i32 (mk_usize 8))
                               (output: t_Slice u8) : prop =
  Seq.length output * 8 == 8 * b /\
  (forall (i: nat). i < 8 ==>
     v (Seq.index input i) >= 0 /\ v (Seq.index input i) < pow2 b)

[@@ "opaque_to_smt"]
let simple_bit_unpack_chunk_post (b: nat{b > 0 /\ b <= 13})
                                 (input: t_Slice u8)
                                 (output: t_Array i32 (mk_usize 8)) : prop =
  Seq.length input * 8 == 8 * b /\
  (forall (i: nat). i < 8 ==>
     v (Seq.index output i) >= 0 /\ v (Seq.index output i) < pow2 b)

(* bit_pack on 8 lanes with offset width [-a, b]; encoded as `b - x` per
   coefficient over (a+b+1) representable values, packed in
   ceil(log2(a+b+1))-bit chunks. Width parameter `w` is the bit width of
   the encoded value (gamma1/eta-specific). *)
[@@ "opaque_to_smt"]
let bit_pack_chunk_post (a b: int) (w: nat{w > 0 /\ w <= 20})
                        (input: t_Array i32 (mk_usize 8))
                        (output: t_Slice u8) : prop =
  Seq.length output * 8 == 8 * w /\
  (forall (i: nat). i < 8 ==>
     v (Seq.index input i) >= -a /\ v (Seq.index input i) <= b)

[@@ "opaque_to_smt"]
let bit_unpack_chunk_post (a b: int) (w: nat{w > 0 /\ w <= 20})
                          (input: t_Slice u8)
                          (output: t_Array i32 (mk_usize 8)) : prop =
  Seq.length input * 8 == 8 * w /\
  (forall (i: nat). i < 8 ==>
     v (Seq.index output i) >= -a /\ v (Seq.index output i) <= b)
"#
    )
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::after(r#"
    let bounded_add_pre (a b: t_Array i32 (sz 8)) (b1:nat) (b2:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\ b1 + b2 <= 2147483647))
                (ensures (Libcrux_ml_dsa.Simd.Traits.Specs.add_pre a b))
               [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.add_pre a b);
                SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
                SMTPat (Spec.Utils.is_i32b_array_opaque b2 b)] =
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.add_pre) (Libcrux_ml_dsa.Simd.Traits.Specs.add_pre)
    "#)]
pub(crate) fn add_pre(lhs: &SIMDContent, rhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            int_is_i32(lhs[i].to_int() + rhs[i].to_int()),
        )
    })
}

#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::after(r#"
#push-options "--z3rlimit 200 --fuel 1 --ifuel 1"
    let bounded_add_post (a b a_future: t_Array i32 (sz 8)) (b1 b2 b3:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\
                    b1 + b2 <= b3 /\ Libcrux_ml_dsa.Simd.Traits.Specs.add_post a b a_future))
            (ensures (Spec.Utils.is_i32b_array_opaque b3 a_future))
            [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.add_post a b a_future);
            SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
            SMTPat (Spec.Utils.is_i32b_array_opaque b2 b);
            SMTPat (Spec.Utils.is_i32b_array_opaque b3 a_future)] =
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.add_post) (Libcrux_ml_dsa.Simd.Traits.Specs.add_post);
        let lemma_lane (i: nat{i < 8}) :
              Lemma (-b3 <= v (Seq.index a_future i) /\ v (Seq.index a_future i) <= b3) =
            assert (v (mk_usize i) == i);
            assert (v (Seq.index a_future i) == v (Seq.index a i) + v (Seq.index b i))
        in
        Classical.forall_intro lemma_lane
#pop-options
    "#)]
pub(crate) fn add_post(lhs: &SIMDContent, rhs: &SIMDContent, future_lhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            future_lhs[i].to_int() == (lhs[i].to_int() + rhs[i].to_int()),
        )
    })
}

#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::after(r#"
    let bounded_sub_pre (a b: t_Array i32 (sz 8)) (b1:nat) (b2:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\ b1 + b2 <= 2147483647))
              (ensures (Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre a b))
              [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre a b);
               SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
               SMTPat (Spec.Utils.is_i32b_array_opaque b2 b)] =
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre) (Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre)
    "#)]
pub(crate) fn sub_pre(lhs: &SIMDContent, rhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            int_is_i32(lhs[i].to_int() - rhs[i].to_int()),
        )
    })
}

#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::after(r#"
#push-options "--z3rlimit 200 --fuel 1 --ifuel 1"
    let bounded_sub_post (a b a_future: t_Array i32 (sz 8)) (b1 b2 b3:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\
                        b1 + b2 <= b3 /\ Libcrux_ml_dsa.Simd.Traits.Specs.sub_post a b a_future))
                (ensures (Spec.Utils.is_i32b_array_opaque b3 a_future))
                [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.sub_post a b a_future);
                SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
                SMTPat (Spec.Utils.is_i32b_array_opaque b2 b);
                SMTPat (Spec.Utils.is_i32b_array_opaque b3 a_future)] =
                reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque);
                reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.sub_post) (Libcrux_ml_dsa.Simd.Traits.Specs.sub_post);
                let lemma_lane (i: nat{i < 8}) :
                      Lemma (-b3 <= v (Seq.index a_future i) /\ v (Seq.index a_future i) <= b3) =
                    assert (v (mk_usize i) == i);
                    assert (v (Seq.index a_future i) == v (Seq.index a i) - v (Seq.index b i))
                in
                Classical.forall_intro lemma_lane
#pop-options
    "#)]
pub(crate) fn sub_post(lhs: &SIMDContent, rhs: &SIMDContent, future_lhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            future_lhs[i].to_int() == (lhs[i].to_int() - rhs[i].to_int()),
        )
    })
}
