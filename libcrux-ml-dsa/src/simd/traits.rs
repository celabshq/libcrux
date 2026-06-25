use crate::constants::{Eta, Gamma2};

/// Specs for the proofs
#[cfg(hax)]
pub(crate) mod specs;

// Each field element occupies 32 bits and the size of a simd_unit is 256 bits.
pub(crate) const COEFFICIENTS_IN_SIMD_UNIT: usize = 8;

// Note: For proofs, it is better to use concrete constants instead of const expressions
//COEFFICIENTS_IN_RING_ELEMENT / COEFFICIENTS_IN_SIMD_UNIT;
pub(crate) const SIMD_UNITS_IN_RING_ELEMENT: usize = 32;

pub const FIELD_MODULUS: i32 = 8_380_417;

// FIELD_MODULUS^{-1} mod MONTGOMERY_R
pub const INVERSE_OF_MODULUS_MOD_MONTGOMERY_R: u64 = 58_728_449;

/// If 'x' denotes a value of type `fe`, values having this type hold a
/// representative y ≡ x·MONTGOMERY_R (mod FIELD_MODULUS).
/// We use 'fer' as a shorthand for this type.
pub(crate) type FieldElementTimesMontgomeryR = i32;

#[cfg(hax)]
#[hax_lib::attributes]
pub(crate) trait Repr: Copy + Clone {
    #[cfg(hax)]
    #[requires(true)]
    fn repr(&self) -> [i32; COEFFICIENTS_IN_SIMD_UNIT];
}

#[cfg(not(hax))]
pub trait Repr {}

// Functional NTT view + post atom (Track B): surface the free NTT functional
// posts through the SIMD trait.  `ntt_poly_view` flattens the 32 SIMD units to
// the abstract 256-coefficient i32 array (via the `Repr` projector `f_repr`),
// matching the `simd_units_to_array (chunks_of_re_*)` view the free `ntt`/`invert`
// fns prove their functional correctness against.  Emitted between `Repr` and
// `Operations`, so `t_Repr` / `f_repr` are in scope.
//
// `ntt_func_post` is an OPAQUE atom wrapping the functional spec (output ≡
// Hacspec ntt mod q).  Keeping it opaque is what lets the impl `ntt` method
// dispatch (`ntt_with_proof`) propagate the post WITHOUT quantifier
// instantiation: a raw `forall` in the trait post either saturates the
// monolithic `impl_1` record query or gets pruned out of the split f_ntt
// sub-query ("incomplete quantifiers").  `lemma_ntt_func_post_intro` seals the
// raw forall (proven by the backend bridges, in a clean context that defeats the
// view-mismatch trigger problem) into the atom; consumers only ever carry the atom.
#[cfg_attr(hax, hax_lib::fstar::before(r#"
[@@ "opaque_to_smt"]
let ntt_poly_view
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Repr v_SIMDUnit)
      (simd_units: t_Array v_SIMDUnit (mk_usize 32))
    : t_Array i32 (mk_usize 256)
  = Hacspec_ml_dsa.createi #i32 (mk_usize 256)
      #(usize -> i32)
      (fun (j: usize{j <. mk_usize 256}) ->
         Seq.index (f_repr (Seq.index simd_units (v j / 8))) (v j % 8))

[@@ "opaque_to_smt"]
let ntt_func_post
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Repr v_SIMDUnit)
      (pre post: t_Array v_SIMDUnit (mk_usize 32))
    : Type0
  = (let in_flat = ntt_poly_view pre in
     let out_flat = ntt_poly_view post in
     forall (j: nat). j < 256 ==>
       (v (Seq.index out_flat j)) % 8380417 ==
       (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt in_flat) j)) % 8380417)

let lemma_ntt_func_post_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Repr v_SIMDUnit)
      (pre post: t_Array v_SIMDUnit (mk_usize 32))
      (xpre xpost: t_Array i32 (mk_usize 256))
    : Lemma
      (requires
        ntt_poly_view pre == xpre /\ ntt_poly_view post == xpost /\
        (forall (j: nat). j < 256 ==>
          (v (Seq.index xpost j)) % 8380417 ==
          (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt xpre) j)) % 8380417))
      (ensures ntt_func_post pre post)
  = reveal_opaque (`%ntt_func_post) (ntt_func_post pre post)

[@@ "opaque_to_smt"]
let invert_func_post
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Repr v_SIMDUnit)
      (pre post: t_Array v_SIMDUnit (mk_usize 32))
    : Type0
  = (let in_flat = ntt_poly_view pre in
     let out_flat = ntt_poly_view post in
     forall (j: nat). j < 256 ==>
       (v (Seq.index out_flat j)) % 8380417 ==
       (v (Seq.index (Spec.MLDSA.Math.to_mont (Hacspec_ml_dsa.Ntt.intt in_flat)) j)) % 8380417)

let lemma_invert_func_post_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i1: t_Repr v_SIMDUnit)
      (pre post: t_Array v_SIMDUnit (mk_usize 32))
      (xpre xpost: t_Array i32 (mk_usize 256))
    : Lemma
      (requires
        ntt_poly_view pre == xpre /\ ntt_poly_view post == xpost /\
        (forall (j: nat). j < 256 ==>
          (v (Seq.index xpost j)) % 8380417 ==
          (v (Seq.index (Spec.MLDSA.Math.to_mont (Hacspec_ml_dsa.Ntt.intt xpre)) j)) % 8380417))
      (ensures invert_func_post pre post)
  = reveal_opaque (`%invert_func_post) (invert_func_post pre post)
"#))]
#[hax_lib::attributes]
pub(crate) trait Operations: Copy + Clone + Repr {
    #[hax_lib::requires(true)]
    #[hax_lib::ensures(|result| result.repr() == [0i32; COEFFICIENTS_IN_SIMD_UNIT])]
    fn zero() -> Self;

    #[hax_lib::requires(array.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[hax_lib::ensures(|result| future(out).repr() == array)]
    fn from_coefficient_array(array: &[i32], out: &mut Self);

    #[hax_lib::requires(out.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[hax_lib::ensures(|result| future(out) == value.repr())]
    fn to_coefficient_array(value: &Self, out: &mut [i32]);

    // Arithmetic
    #[hax_lib::requires(specs::add_pre(&lhs.repr(), &rhs.repr()))]
    #[hax_lib::ensures(|_| specs::add_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn add(lhs: &mut Self, rhs: &Self);

    #[hax_lib::requires(specs::sub_pre(&lhs.repr(), &rhs.repr()))]
    #[hax_lib::ensures(|_| specs::sub_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn subtract(lhs: &mut Self, rhs: &Self);

    #[hax_lib::requires(fstar!(r#"v $bound > 0 /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${simd_unit})"#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post
            (f_repr ${simd_unit}) $bound $result"#))]
    fn infinity_norm_exceeds(simd_unit: &Self, bound: i32) -> bool;

    // F-11 (2026-04-29): originally tightened `low_future` post to
    // `is_i32b_strict_lower_array_opaque γ2`.  REVERTED 2026-04-29
    // (F-13): FIPS 204 Algorithm 36's special-case adjustment
    // `r0 ← r0 - 1` (when `r_q - r_g == q - 1`) drives `r0` to exactly
    // `-γ2` at the boundary, breaking the strict-lower bound.  Concrete
    // counter-example: γ2=95232, r=8285185 → r0=-95232.  Closed
    // `is_i32b_array_opaque γ2` is the correct bound for `decompose`'s
    // `low_future`.  F-8 / F-9 / F-10 retain strict-lower (those are
    // pure `mod^±` outputs without the special-case adjustment).
    #[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${simd_unit})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
            Spec.Utils.is_i32b_array_opaque 95232 (f_repr ${low}_future) /\
            Spec.Utils.is_i32b_array_opaque 44 (f_repr ${high}_future)) /\
         (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
            Spec.Utils.is_i32b_array_opaque 261888 (f_repr ${low}_future) /\
            Spec.Utils.is_i32b_array_opaque 16 (f_repr ${high}_future))) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post
            $gamma2
            (Seq.index (f_repr ${simd_unit}) i)
            (Seq.index (f_repr ${low}_future) i)
            (Seq.index (f_repr ${high}_future) i))"#))]
    fn decompose(gamma2: Gamma2, simd_unit: &Self, low: &mut Self, high: &mut Self);

    #[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${low}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${high})"#))]
    #[hax_lib::ensures(|result| fstar!(r#"v $result <= 8 /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
          (f_repr ${hint}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.compute_hint_lane_post
            $gamma2
            (Seq.index (f_repr ${low}) i)
            (Seq.index (f_repr ${high}) i)
            (Seq.index (f_repr ${hint}_future) i))"#))]
    fn compute_hint(low: &Self, high: &Self, gamma2: i32, hint: &mut Self) -> usize;

    #[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${simd_unit}) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque (f_repr ${hint})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
            Spec.Utils.is_i32b_array_opaque 44 (f_repr ${hint}_future)) /\
         (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
            Spec.Utils.is_i32b_array_opaque 16 (f_repr ${hint}_future))) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.use_hint_lane_post
            $gamma2
            (Seq.index (f_repr ${simd_unit}) i)
            (Seq.index (f_repr ${hint}) i)
            (Seq.index (f_repr ${hint}_future) i))"#))]
    fn use_hint(gamma2: Gamma2, simd_unit: &Self, hint: &mut Self);

    // Modular operations
    // Both operands may be NTT-domain values bounded by NTT_OUTPUT_BOUND
    // (= 9*FIELD_MAX), the lazily-accumulated forward-NTT output bound. The
    // product (9*FIELD_MAX)^2 < FIELD_MAX*2^31, so montgomery_reduce_element
    // still reduces the output back to FIELD_MAX (the output post below).
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND}) (${lhs.repr()}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND}) (${rhs.repr()})"#))]
    // 2026-05-08 (audit Phase A item 10): dropped third clause
    //   `forall (i:nat). i < 8 ==> Seq.index ... == Spec.MLDSA.Math.mont_mul ...`
    // because `mont_mul` is a non-opaque (transparent) wrapper over `mont_red`/`i32_mul`,
    // so the bare `forall i<8` Skolem leaked raw arithmetic above the trait at every
    // call site (k!61 cascade candidate per
    // proofs/agent-status/abstraction-boundary-audit-2026-05-07.md).  Audit confirmed
    // zero above-trait consumers reference `Spec.MLDSA.Math.mont_mul`, so this is a
    // free clause-drop.  The free-fn posts (avx2/arithmetic.rs, portable/arithmetic.rs,
    // portable.rs::montgomery_multiply_with_proof) retain the mont_mul clause; that
    // lives below the trait boundary and is fine.
    #[hax_lib::ensures(|result| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${lhs}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
            (Seq.index (${lhs.repr()}) i)
            (Seq.index (${rhs.repr()}) i)
            (Seq.index (f_repr ${lhs}_future) i))"#))]
    fn montgomery_multiply(lhs: &mut Self, rhs: &Self);

    // 261631 is the largest x such that x * pow2 13 <= 2143289343 (the barrett reduce input bound)
    // 2026-05-08: bare `forall i. i < 8 ==> ...` → `Spec.Utils.forall8` (transparent macro
    // that unfolds to a finite 8-way conjunction; no Z3 quantifier instantiation).
    #[hax_lib::requires(fstar!(r#"v $SHIFT_BY == 13 /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
            v (Seq.index (f_repr ${simd_unit}) i) >= 0 /\
            v (Seq.index (f_repr ${simd_unit}) i) <= 261631)"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.shift_left_then_reduce_lane_post
            (Seq.index (f_repr ${simd_unit}) i)
            (Seq.index (f_repr ${simd_unit}_future) i))"#))]
    fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Self);

    // Decomposition operations
    // F-9 (2026-04-29): `t0_future` post tightened from closed
    // `is_i32b_array_opaque (pow2 12)` to half-open
    // `is_i32b_strict_lower_array_opaque (pow2 12)` matching FIPS 204
    // Algorithm 35 (Power2Round), where t0 ∈ (-2^12, 2^12].  Chain-critical
    // with F-8: `power2round → t0_serialize` flow now lines up.
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (f_repr ${t0})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12) (f_repr ${t0}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (f_repr ${t1}_future) i) >= 0 /\
          v (Seq.index (f_repr ${t1}_future) i) < pow2 10) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.power2round_lane_post
            (Seq.index (f_repr ${t0}) i)
            (Seq.index (f_repr ${t1}_future) i)
            (Seq.index (f_repr ${t0}_future) i))"#))]
    fn power2round(t0: &mut Self, t1: &mut Self);

    // Sampling
    //
    // In the sampling functions, since each SIMD unit can hold 8 coefficients,
    // we expect that `out` has the capacity for up to 8 coefficients.

    // Since each coefficient could potentially be sampled with 3 bytes, we expect
    // `randomness` to hold 24 bytes.
    #[hax_lib::requires(fstar!(r#"
        Seq.length $randomness == 24 /\
        Seq.length $randomness / 3 <= 4294967295 /\
        Seq.length $randomness / 3 <= Seq.length $out"#))]
    #[hax_lib::ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= 0 /\
          v (Seq.index ${out}_future i) < 8380417)"#))]
    fn rejection_sample_less_than_field_modulus(randomness: &[u8], out: &mut [i32]) -> usize;

    // Since each coefficient could potentially be sampled with half a byte,
    // we expect `randomness` to hold 4 bytes.
    #[hax_lib::requires(fstar!(r#"
        Seq.length $randomness == 4 /\
        Seq.length $randomness * 2 <= 4294967295 /\
        Seq.length $randomness * 2 <= Seq.length $out"#))]
    #[hax_lib::ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= -2 /\ v (Seq.index ${out}_future i) <= 2)"#))]
    fn rejection_sample_less_than_eta_equals_2(randomness: &[u8], out: &mut [i32]) -> usize;

    #[hax_lib::requires(fstar!(r#"
        Seq.length $randomness == 4 /\
        Seq.length $randomness * 2 <= 4294967295 /\
        Seq.length $randomness * 2 <= Seq.length $out"#))]
    #[hax_lib::ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= -4 /\ v (Seq.index ${out}_future i) <= 4)"#))]
    fn rejection_sample_less_than_eta_equals_4(randomness: &[u8], out: &mut [i32]) -> usize;

    // Encoding operations

    // Gamma1: serialized length is 8 * (gamma1_exponent + 1) / 8 = gamma1_exponent + 1 bytes
    // per 8-coefficient SIMD unit, but with each coefficient using w = gamma1_exponent + 1 bits
    // (w in {18, 20}). For 8 lanes that's 18 or 20 bytes.
    // F-3 (2026-04-28): pre uses non-negative-bounded `is_pos_array_opaque`
    // since the impl operates on the shifted (non-negative) representation.
    // F-7 (2026-04-29): tighten upper bound to `pow2 d - 1` (strict `< pow2 d`)
    // so the trait pre matches the free fns' `bounded x d` (= `< pow2 d`) exactly.
    #[hax_lib::requires(fstar!(r#"
        (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19) /\
        Seq.length $serialized == 1 + v $gamma1_exponent /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (pow2 (v $gamma1_exponent) - 1)
            (f_repr ${simd_unit})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn gamma1_serialize(simd_unit: &Self, serialized: &mut [u8], gamma1_exponent: usize);
    #[hax_lib::requires(fstar!(r#"
        (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19) /\
        Seq.length $serialized == 1 + v $gamma1_exponent"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (pow2 (v $gamma1_exponent))
          (f_repr ${out}_future)"#))]
    fn gamma1_deserialize(serialized: &[u8], out: &mut Self, gamma1_exponent: usize);

    // Commitment: 4 bytes for gamma2 = 261888 (4-bit packing) or 6 for gamma2 = 95232 (6-bit).
    // F-3 (2026-04-28): pre uses non-negative-bounded `is_pos_array_opaque`
    // since commitment values are the high half of decompose, in [0, 16) or [0, 44).
    // F-7 (2026-04-29): tighten upper bound to `pow2 d - 1` (strict `< pow2 d`)
    // so the trait pre matches the free fns' `bounded x d` (= `< pow2 d`) exactly.
    #[hax_lib::requires(fstar!(r#"
        (Seq.length $serialized == 4 \/ Seq.length $serialized == 6) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (pow2 (Seq.length $serialized) - 1)
            (f_repr ${simd_unit})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn commitment_serialize(simd_unit: &Self, serialized: &mut [u8]);

    // Error: 3 bytes for eta = 2 (3-bit), 4 bytes for eta = 4 (4-bit).
    // F-3 (2026-04-28): pre uses non-negative-bounded `is_pos_array_opaque`
    // since the impl operates on the shifted (non-negative) representation
    // of error values (eta - x).
    #[hax_lib::requires(fstar!(r#"
        Seq.length $serialized == (match $eta with
                                   | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                   | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque
            (match $eta with
             | Libcrux_ml_dsa.Constants.Eta_Two -> 2
             | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
            (f_repr ${simd_unit})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn error_serialize(eta: Eta, simd_unit: &Self, serialized: &mut [u8]);
    #[hax_lib::requires(fstar!(r#"
        Seq.length $serialized == (match $eta with
                                   | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                   | Libcrux_ml_dsa.Constants.Eta_Four -> 4)"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          ($eta == Libcrux_ml_dsa.Constants.Eta_Two ==>
              v (Seq.index (f_repr ${out}_future) i) >= -5 /\
              v (Seq.index (f_repr ${out}_future) i) <= 2) /\
          ($eta == Libcrux_ml_dsa.Constants.Eta_Four ==>
              v (Seq.index (f_repr ${out}_future) i) >= -11 /\
              v (Seq.index (f_repr ${out}_future) i) <= 4))"#))]
    fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Self);

    // t0: bit_pack with width 13.
    // F-3 (2026-04-28): pre uses non-negative-bounded `is_pos_array_opaque`
    // since the impl operates on the shifted (non-negative) t0 representation.
    // F-6 (2026-04-29): switch t0_serialize back to centered `is_i32b_array_opaque (pow2 12)`.
    // The AVX2 free fn requires `(POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE - lane) in (0, pow2 13)`,
    // i.e. lane in (-4095, 4096], which is the centered semantic of t0 inputs (lower 13 signed
    // bits of t centered around 0). The non-negative `is_pos_array_opaque (pow2 13)` allowed
    // boundary `lane == 8192`, which made the AVX2 pre fail (`4096 - 8192 = -4096 < 0`).
    // F-8 (2026-04-29): tighten further to half-open `is_i32b_strict_lower_array_opaque (pow2 12)`
    // = (-pow2 12, pow2 12], because the AVX2 free fn requires `(POW_2_..._MINUS_ONE - lane) < pow2 13`
    // which solves to `lane > -pow2 12` (strict).  At the closed-form boundary `lane == -4096`,
    // the AVX2 free fn pre fails (4096 - (-4096) = 8192 = pow2 13, not strictly less).
    #[hax_lib::requires(fstar!(r#"
        Seq.length $out == 13 /\
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
            (f_repr ${simd_unit})"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${out}_future == Seq.length ${out}"#))]
    fn t0_serialize(simd_unit: &Self, out: &mut [u8]); // out len 13
    // F-10 (2026-04-29): post tightened to half-open `is_i32b_strict_lower_array_opaque (pow2 12)`
    // for round-trip symmetry with `t0_serialize`.
    #[hax_lib::requires(serialized.len() == 13)]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12) (f_repr ${out}_future)"#))]
    fn t0_deserialize(serialized: &[u8], out: &mut Self);

    // t1: simple_bit_pack with width 10.
    // 2026-05-08: bare `forall (i: nat). i < 8 ==> ...` → `Spec.Utils.forall8`.
    #[hax_lib::requires(fstar!(r#"
        Seq.length $out == 10 /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (f_repr ${simd_unit}) i) >= 0 /\
          v (Seq.index (f_repr ${simd_unit}) i) < pow2 10)"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Seq.length ${out}_future == Seq.length ${out}"#))]
    fn t1_serialize(simd_unit: &Self, out: &mut [u8]); // out len 10
    #[hax_lib::requires(serialized.len() == 10)]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (f_repr ${out}_future) i) >= 0 /\
          v (Seq.index (f_repr ${out}_future) i) < pow2 10)"#))]
    fn t1_deserialize(serialized: &[u8], out: &mut Self);

    // NTT
    // 2026-05-08: bare `forall (i:nat). i < 32 ==> ...` → `Spec.Utils.forall32` here
    // and on `invert_ntt_montgomery` / `reduce` below.  Same rationale as forall8:
    // transparent macro unfolds to a 32-way conjunction so Z3 doesn't instantiate
    // a quantifier at the call site.
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque
            (v ${specs::NTT_BASE_BOUND})
            (f_repr (Seq.index ${simd_units} i)))
    "#))]
    // Output bound is NTT_OUTPUT_BOUND (= 9*FIELD_MAX), not FIELD_MAX: the
    // forward NTT lazily accumulates to NTT_BASE_BOUND + 8*FIELD_MAX and is
    // deliberately NOT reduced (the downstream montgomery multiply absorbs it).
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND})
            (f_repr (Seq.index ${simd_units}_future i))) /\
        Libcrux_ml_dsa.Simd.Traits.ntt_func_post ${simd_units} ${simd_units}_future
    "#))]
    fn ntt(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);

    // invert NTT and convert to standard domain
    //
    // Tight output bound 4_211_177 = q/2 + ⌈256·FIELD_MAX·41978 / 2³²⌉.
    // Justification: after the inverse-NTT layer functions each lane holds a
    // value bounded by 256·FIELD_MAX; the final per-vector Montgomery-multiply
    // by 41978 reduces it via `mont_red`, and
    // `Spec.MLDSA.Math.lemma_mont_red_bound_256_field_max_times_41978`
    // proves `is_i32b 4_211_177` on the per-lane result.  Surfacing the
    // tight bound here unblocks `compute_as1_plus_s2`'s body proof (the
    // headline Sprint 2 payoff: 4_211_177 + 4 (eta) ≪ FIELD_MAX so the
    // subsequent `+ s2` step stays comfortably bounded).
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
            (f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque 4211177
            (f_repr (Seq.index ${simd_units}_future i))) /\
        Libcrux_ml_dsa.Simd.Traits.invert_func_post ${simd_units} ${simd_units}_future
    "#))]
    fn invert_ntt_montgomery(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);

    // Barrett reduce all coefficients
    #[hax_lib::requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque 2143289343
                (f_repr (Seq.index ${simd_units} i)))"#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (j: nat{j < 32}) ->
          Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
            (f_repr (Seq.index ${simd_units}_future j)) /\
          Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
            Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
              (Seq.index (f_repr (Seq.index ${simd_units} j)) i)
              (Seq.index (f_repr (Seq.index ${simd_units}_future j)) i)))"#))]
    fn reduce(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);
}
