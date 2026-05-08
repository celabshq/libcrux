use crate::{
    constants::{Eta, Gamma2},
    simd::traits::*,
};

mod arithmetic;
mod encoding;
mod invntt;
mod ntt;
mod rejection_sample;
mod vector_type;

use arithmetic::shift_left_then_reduce;
pub(crate) use vector_type::{AVX2RingElement, Vec256 as AVX2SIMDUnit};

#[cfg(hax)]
use crate::simd::traits::specs;

#[cfg(hax)]
impl Repr for AVX2SIMDUnit {
    fn repr(&self) -> [i32; COEFFICIENTS_IN_SIMD_UNIT] {
        let mut result = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
        vector_type::to_coefficient_array(self, &mut result);
        result
    }
}

#[cfg(not(hax))]
impl Repr for AVX2SIMDUnit {}

// ---------------------------------------------------------------------------
// Track B (Step 10): one-line-wrapper refactor for non-trivial impl methods.
// See `src/simd/portable.rs` for the Portable counterparts and rationale.
// ---------------------------------------------------------------------------

#[inline(always)]
#[hax_lib::requires(fstar!(r#"v $bound > 0 /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post
        (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) $bound $result"#))]
pub(crate) fn infinity_norm_exceeds_with_proof(simd_unit: &AVX2SIMDUnit, bound: i32) -> bool {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}));
        let _r = Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit} in
        assert (forall (i: u64). v i < 8 ==>
            Spec.Utils.is_i32b 8380416
                (Spec.Intrinsics.to_i32x8
                    ${simd_unit}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value i))"#
    );
    let result = arithmetic::infinity_norm_exceeds(&simd_unit.value, bound);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post)
            (Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) $bound $result)"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"v $SHIFT_BY == 13 /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
        v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) >= 0 /\
        v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) <= 261631)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.shift_left_then_reduce_lane_post
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}_future) i))"#))]
pub(crate) fn shift_left_then_reduce_with_proof<const SHIFT_BY: i32>(
    simd_unit: &mut AVX2SIMDUnit,
) {
    #[cfg(hax)]
    let _orig = *simd_unit;
    shift_left_then_reduce::<SHIFT_BY>(&mut simd_unit.value);
    hax_lib::fstar!(
        r#"let pf (k: nat{k < 8}) : Lemma
            (ensures Libcrux_ml_dsa.Simd.Traits.Specs.shift_left_then_reduce_lane_post
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)) =
            Hacspec_ml_dsa.Commute.Chunk.lemma_shift_left_then_reduce_lane_commute
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)
        in
        Classical.forall_intro pf"#
    );
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0})"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12) (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}_future) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}_future) i) >= 0 /\
      v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}_future) i) < pow2 10) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.power2round_lane_post
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}_future) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}_future) i))"#))]
pub(crate) fn power2round_with_proof(t0: &mut AVX2SIMDUnit, t1: &mut AVX2SIMDUnit) {
    #[cfg(hax)]
    let _orig_t0 = *t0;
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}));
        let _r = Libcrux_ml_dsa.Simd.Traits.f_repr ${t0} in
        assert (forall (i: u64). v i < 8 ==>
            Spec.Utils.is_i32b 8380416
                (Spec.Intrinsics.to_i32x8
                    ${t0}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value i))"#
    );
    arithmetic::power2round(&mut t0.value, &mut t1.value);
    hax_lib::fstar!(
        r#"
        let pf (k: nat{k < 8}) : Lemma
            (ensures Libcrux_ml_dsa.Simd.Traits.Specs.power2round_lane_post
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_t0}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}) k)) =
            Hacspec_ml_dsa.Commute.Chunk.lemma_power2round_lane_commute
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_t0}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}) k)
        in
        Classical.forall_intro pf;
        // Track 0 (c6c68bbca propagation): half-open (-pow2 12, pow2 12] post.
        // The AVX2 free fn post on `arithmetic::power2round` only states the
        // closed `is_i32b (pow2 12)` bound; the math lemma supplies the
        // strict-lower side.  cf. F-13 for why `decompose` cannot do the same.
        let pf_t0 (k: nat{k < 8}) : Lemma
            (ensures
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}) k) > -(pow2 12) /\
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}) k) <= pow2 12) =
            Hacspec_ml_dsa.Commute.Chunk.lemma_power2round_t0_strict_lower_bound
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_t0}) k)
        in
        Classical.forall_intro pf_t0;
        reveal_opaque (`%Spec.Utils.is_i32b_strict_lower_array_opaque)
            (Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}));
        let pf_t1 (k: nat{k < 8}) : Lemma
            (ensures
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}) k) >= 0 /\
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}) k) < pow2 10) =
            Hacspec_ml_dsa.Commute.Chunk.lemma_power2round_t1_bound
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_t0}) k)
        in
        Classical.forall_intro pf_t1"#
    );
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_dsa.Simd.Traits.Specs.add_pre
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs})"#))]
#[hax_lib::ensures(|_| fstar!(r#"Libcrux_ml_dsa.Simd.Traits.Specs.add_post
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs})
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs})
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future)"#))]
pub(crate) fn add_with_proof(lhs: &mut AVX2SIMDUnit, rhs: &AVX2SIMDUnit) {
    #[cfg(hax)]
    let _orig = *lhs;
    arithmetic::add(&mut lhs.value, &rhs.value);
    hax_lib::fstar!(r#"
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.add_pre)
            (Libcrux_ml_dsa.Simd.Traits.Specs.add_pre
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}));
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.add_post)
            (Libcrux_ml_dsa.Simd.Traits.Specs.add_post
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}));
        let pfk (k: nat{k < 8}) : Lemma
            (ensures
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k) ==
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k) +
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)) =
            assert (mk_usize k <. Libcrux_ml_dsa.Simd.Traits.Specs.v_COEFFICIENTS_IN_SIMD_UNIT);
            assert (Libcrux_ml_dsa.Simd.Traits.Specs.int_is_i32
                (v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k) +
                 v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)));
            Hacspec_ml_dsa.Commute.Chunk.lemma_add_lane_commute
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k)
        in
        Classical.forall_intro pfk
    "#);
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs})"#))]
#[hax_lib::ensures(|_| fstar!(r#"Libcrux_ml_dsa.Simd.Traits.Specs.sub_post
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs})
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs})
    (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future)"#))]
pub(crate) fn subtract_with_proof(lhs: &mut AVX2SIMDUnit, rhs: &AVX2SIMDUnit) {
    #[cfg(hax)]
    let _orig = *lhs;
    arithmetic::subtract(&mut lhs.value, &rhs.value);
    hax_lib::fstar!(r#"
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre)
            (Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}));
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.sub_post)
            (Libcrux_ml_dsa.Simd.Traits.Specs.sub_post
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}));
        let pfk (k: nat{k < 8}) : Lemma
            (ensures
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k) ==
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k) -
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)) =
            assert (mk_usize k <. Libcrux_ml_dsa.Simd.Traits.Specs.v_COEFFICIENTS_IN_SIMD_UNIT);
            assert (Libcrux_ml_dsa.Simd.Traits.Specs.int_is_i32
                (v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k) -
                 v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)));
            Hacspec_ml_dsa.Commute.Chunk.lemma_sub_lane_commute
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k)
        in
        Classical.forall_intro pfk
    "#);
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (${rhs.repr()})"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
        (Seq.index (${lhs.repr()}) i)
        (Seq.index (${rhs.repr()}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) i))"#))]
pub(crate) fn montgomery_multiply_with_proof(lhs: &mut AVX2SIMDUnit, rhs: &AVX2SIMDUnit) {
    #[cfg(hax)]
    let _orig_lhs = *lhs;
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}))"#
    );
    arithmetic::montgomery_multiply(&mut lhs.value, &rhs.value);
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let pf (k: nat{k < 8}) : Lemma
            (ensures
                Spec.Utils.is_i32b 8380416
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k) /\
                Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_lhs}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k)) =
            Hacspec_ml_dsa.Commute.Chunk.lemma_mont_mul_bound_and_mod_q
                (Spec.Intrinsics.to_i32x8
                    ${_orig_lhs}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                    (mk_u64 k))
                (Spec.Intrinsics.to_i32x8
                    ${rhs}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                    (mk_u64 k));
            Libcrux_ml_dsa.Simd.Traits.Specs.lemma_montgomery_multiply_lane_intro
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_lhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k)
        in
        Classical.forall_intro pf;
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}))"#
    );
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
     v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
        Spec.Utils.is_i32b_array_opaque 95232 (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}_future) /\
        Spec.Utils.is_i32b_array_opaque 44 (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}_future)) /\
     (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
        Spec.Utils.is_i32b_array_opaque 261888 (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}_future) /\
        Spec.Utils.is_i32b_array_opaque 16 (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}_future))) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post
        $gamma2
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}_future) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}_future) i))"#))]
pub(crate) fn decompose_with_proof(
    gamma2: Gamma2,
    simd_unit: &AVX2SIMDUnit,
    low: &mut AVX2SIMDUnit,
    high: &mut AVX2SIMDUnit,
) {
    #[cfg(hax)]
    let _orig = *simd_unit;
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}))"#
    );
    arithmetic::decompose(gamma2, &simd_unit.value, &mut low.value, &mut high.value);
    hax_lib::fstar!(
        r#"
        // Per-lane bridge: AVX2 free fn post (decompose_spec shape) →
        // trait post (combined gamma2-conditional bound + lane post).
        let pf_eq (k: nat{k < 8}) : Lemma
            (ensures Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post
                $gamma2
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)) =
            let r = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k in
            let r0 = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k in
            let r1 = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k in
            if v r >= 0 && v r < 8380417 then begin
                Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_spec_eq_decompose
                    $gamma2 r;
                let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v $gamma2) (v r) in
                assert (v r0 == r0_s /\ v r1 == r1_s);
                Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_lane_commute_conditional
                    $gamma2 r r0 r1
            end else
                reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post)
                              (Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post
                                  $gamma2 r r0 r1)
        in
        Classical.forall_intro pf_eq;
        let pf_bound (k: nat{k < 8}) : Lemma
            (ensures
                (v $gamma2 == 95232 ==>
                    Spec.Utils.is_i32b 95232
                        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k) /\
                    Spec.Utils.is_i32b 44
                        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)) /\
                (v $gamma2 == 261888 ==>
                    Spec.Utils.is_i32b 261888
                        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k) /\
                    Spec.Utils.is_i32b 16
                        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)) ) =
            let r = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k in
            Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_spec_eq_decompose
                $gamma2 r;
            Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_bound $gamma2 r
        in
        Classical.forall_intro pf_bound;
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque 95232
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}));
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque 44
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}));
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque 261888
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}));
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque 16
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}))"#
    );
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
     v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${high})"#))]
#[hax_lib::ensures(|result| fstar!(r#"v $result <= 8 /\
    Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
      (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.compute_hint_lane_post
        $gamma2
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future) i))"#))]
pub(crate) fn compute_hint_with_proof(
    low: &AVX2SIMDUnit,
    high: &AVX2SIMDUnit,
    gamma2: i32,
    hint: &mut AVX2SIMDUnit,
) -> usize {
    hax_lib::fstar!("admit ()");
    arithmetic::compute_hint(&low.value, &high.value, gamma2, &mut hint.value)
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
     v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) /\
    Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint})"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
        Spec.Utils.is_i32b_array_opaque 44 (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future)) /\
     (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
        Spec.Utils.is_i32b_array_opaque 16 (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future))) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.use_hint_lane_post
        $gamma2
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future) i))"#))]
pub(crate) fn use_hint_with_proof(
    gamma2: Gamma2,
    simd_unit: &AVX2SIMDUnit,
    hint: &mut AVX2SIMDUnit,
) {
    hax_lib::fstar!("admit ()");
    arithmetic::use_hint(gamma2, &simd_unit.value, &mut hint.value);
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
        Spec.Utils.is_i32b_array_opaque 2143289343
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall32 (fun (j: nat{j < 32}) ->
      Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
        (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future j)) /\
      Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
        Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
          (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) i)
          (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future j)) i)))"#))]
pub(crate) fn reduce_with_proof(simd_units: &mut [AVX2SIMDUnit; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let _orig = simd_units.clone();

    for i in 0..simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(r#"
            v $i <= 32 /\
            (forall (j:nat{j < 32}). j < v $i ==>
                (forall (k:nat{k < 8}).
                    Spec.Intrinsics.to_i32x8
                        (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                        (mk_u64 k) ==
                    Spec.MLDSA.Math.barrett_red
                        (Spec.Intrinsics.to_i32x8
                            (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                            (mk_u64 k)))) /\
            (forall (j:nat{j < 32}). j >= v $i ==>
                Seq.index ${simd_units} j == Seq.index ${_orig} j)"#));

        arithmetic::reduce(&mut simd_units[i].value);
    }

    hax_lib::fstar!(r#"
        let pf (j: nat{j < 32}) : Lemma
            (ensures
                Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                    (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) /\
                Spec.Utils.forall8 (fun (k: nat{k < 8}) ->
                    Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
                        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${_orig} j)) k)
                        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) k))) =
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque 2143289343
                    (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${_orig} j)));
            let pfk (k: nat{k < 8}) : Lemma
                (ensures Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${_orig} j)) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) k)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_barrett_red_bound_and_mod_q
                    (Spec.Intrinsics.to_i32x8
                        (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                        (mk_u64 k));
                Hacspec_ml_dsa.Commute.Chunk.lemma_reduce_lane_commute
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${_orig} j)) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) k)
            in
            Classical.forall_intro pfk;
            let pfk_bound (k: nat{k < 8}) : Lemma
                (ensures Spec.Utils.is_i32b 8380416
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) k)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_barrett_red_bound_and_mod_q
                    (Spec.Intrinsics.to_i32x8
                        (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                        (mk_u64 k))
            in
            Classical.forall_intro pfk_bound;
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                    (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)))
        in
        Classical.forall_intro pf
    "#);
}

/// Implementing the [`Operations`] for AVX2.
// 2026-05-08: `--z3rlimit 400` (state (d)) — keeps the monolithic VC
// (faster than --split_queries always's per-conjunct fragmentation under
// forallN macros) but raises the budget to fit the bigger forall8/32 unfold.
// rlimit 200 saturates at 32.9s; bumped to 400 for headroom.  Portable's
// equivalent impl block fits at rlimit 200, but Avx2's has more inline
// body code (Track B less deeply factored than Portable's).
// State (a) bare-forall + split_queries was 11.8s/40q.
// State (b) forallN + split_queries was 38s/245q.
// State (c) forallN + no-split + rlimit 80 fails at 15.5s rlimit-sat.
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 400"))]
#[hax_lib::attributes]
impl Operations for AVX2SIMDUnit {
    #[inline(always)]
    #[ensures(|result| result.repr() == [0i32; COEFFICIENTS_IN_SIMD_UNIT])]
    fn zero() -> Self {
        let result = vector_type::zero();
        hax_lib::fstar!(
            r#"
            // f_repr result == [0;8]: the SIMD setzero intrinsic gives all-zero
            // lanes per `Spec.Intrinsics.mm256_setzero_si256_lemma`, and
            // f_repr extracts via to_coefficient_array.
            assert (forall (i: nat). i < 8 ==>
                Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr result) i == mk_i32 0);
            assert (forall (i: nat). i < 8 ==>
                Seq.index (Rust_primitives.Hax.repeat (mk_i32 0) (mk_usize 8)) i == mk_i32 0);
            assert (Seq.equal (Libcrux_ml_dsa.Simd.Traits.f_repr result)
                              (Rust_primitives.Hax.repeat (mk_i32 0) (mk_usize 8)))"#
        );
        result
    }

    #[inline(always)]
    #[requires(coefficient_array.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[ensures(|_| future(out).repr() == coefficient_array)]
    fn from_coefficient_array(coefficient_array: &[i32], out: &mut Self) {
        vector_type::from_coefficient_array(coefficient_array, out);
        hax_lib::fstar!(
            r#"
            // f_repr out_future == coefficient_array: the SIMD loadu intrinsic
            // preserves per-lane content per `Spec.Intrinsics.mm256_loadu_si256_i32_lemma`,
            // and f_repr extracts via to_coefficient_array.
            assert (forall (i: nat). i < 8 ==>
                Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}) i ==
                Seq.index ${coefficient_array} i);
            assert (Seq.equal (Libcrux_ml_dsa.Simd.Traits.f_repr ${out})
                              ${coefficient_array})"#
        );
    }

    #[inline(always)]
    #[requires(out.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[ensures(|_| future(out) == value.repr())]
    fn to_coefficient_array(value: &Self, out: &mut [i32]) {
        vector_type::to_coefficient_array(value, out);
        hax_lib::fstar!(
            r#"
            // out_future == f_repr value: per-lane content from
            // `Spec.Intrinsics.mm256_storeu_si256_i32_lemma` matches the
            // f_repr definition (which itself goes through to_coefficient_array).
            assert (forall (i: nat). i < 8 ==>
                Seq.index ${out} i ==
                Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${value}) i);
            assert (Seq.equal ${out} (Libcrux_ml_dsa.Simd.Traits.f_repr ${value}))"#
        );
    }

    #[inline(always)]
    #[requires(specs::add_pre(&lhs.repr(), &rhs.repr()))]
    #[ensures(|_| specs::add_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn add(lhs: &mut Self, rhs: &Self) {
        add_with_proof(lhs, rhs)
    }

    #[inline(always)]
    #[requires(specs::sub_pre(&lhs.repr(), &rhs.repr()))]
    #[ensures(|_| specs::sub_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn subtract(lhs: &mut Self, rhs: &Self) {
        subtract_with_proof(lhs, rhs)
    }

    #[inline(always)]
    #[requires(fstar!(r#"v $bound > 0 /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|result| fstar!(r#"
        Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) $bound $result"#))]
    fn infinity_norm_exceeds(simd_unit: &Self, bound: i32) -> bool {
        infinity_norm_exceeds_with_proof(simd_unit, bound)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
            Spec.Utils.is_i32b_array_opaque 95232 (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}_future) /\
            Spec.Utils.is_i32b_array_opaque 44 (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}_future)) /\
         (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
            Spec.Utils.is_i32b_array_opaque 261888 (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}_future) /\
            Spec.Utils.is_i32b_array_opaque 16 (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}_future))) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post
            $gamma2
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}_future) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}_future) i))"#))]
    fn decompose(gamma2: Gamma2, simd_unit: &Self, low: &mut Self, high: &mut Self) {
        decompose_with_proof(gamma2, simd_unit, low, high)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${high})"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
          (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.compute_hint_lane_post
            $gamma2
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future) i))"#))]
    fn compute_hint(low: &Self, high: &Self, gamma2: i32, hint: &mut Self) -> usize {
        compute_hint_with_proof(low, high, gamma2, hint)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint})"#))]
    #[ensures(|_| fstar!(r#"
        ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
            Spec.Utils.is_i32b_array_opaque 44 (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future)) /\
         (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
            Spec.Utils.is_i32b_array_opaque 16 (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future))) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.use_hint_lane_post
            $gamma2
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}_future) i))"#))]
    fn use_hint(gamma2: Gamma2, simd_unit: &Self, hint: &mut Self) {
        use_hint_with_proof(gamma2, simd_unit, hint)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (${rhs.repr()})"#))]
    // 2026-05-08: matches the trait declaration drop (Phase A item 10).
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
            (Seq.index (${lhs.repr()}) i)
            (Seq.index (${rhs.repr()}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) i))"#))]
    fn montgomery_multiply(lhs: &mut Self, rhs: &Self) {
        montgomery_multiply_with_proof(lhs, rhs)
    }

    #[inline(always)]
    #[requires(fstar!(r#"v $SHIFT_BY == 13 /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
            v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) >= 0 /\
            v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) <= 261631)"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.shift_left_then_reduce_lane_post
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}_future) i))"#))]
    fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Self) {
        shift_left_then_reduce_with_proof::<SHIFT_BY>(simd_unit)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0})"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12) (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}_future) i) >= 0 /\
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}_future) i) < pow2 10) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.power2round_lane_post
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t1}_future) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${t0}_future) i))"#))]
    fn power2round(t0: &mut Self, t1: &mut Self) {
        power2round_with_proof(t0, t1)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $randomness / 3 <= 4294967295 /\
        Seq.length $randomness / 3 <= Seq.length $out"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= 0 /\
          v (Seq.index ${out}_future i) < 8380417)"#))]
    fn rejection_sample_less_than_field_modulus(randomness: &[u8], out: &mut [i32]) -> usize {
        hax_lib::fstar!("admit ()");
        rejection_sample::less_than_field_modulus::sample(randomness, out)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $randomness * 2 <= 4294967295 /\
        Seq.length $randomness * 2 <= Seq.length $out"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= -2 /\ v (Seq.index ${out}_future i) <= 2)"#))]
    fn rejection_sample_less_than_eta_equals_2(randomness: &[u8], out: &mut [i32]) -> usize {
        hax_lib::fstar!("admit ()");
        rejection_sample::less_than_eta::sample::<2>(randomness, out)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $randomness * 2 <= 4294967295 /\
        Seq.length $randomness * 2 <= Seq.length $out"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= -4 /\ v (Seq.index ${out}_future i) <= 4)"#))]
    fn rejection_sample_less_than_eta_equals_4(randomness: &[u8], out: &mut [i32]) -> usize {
        hax_lib::fstar!("admit ()");
        rejection_sample::less_than_eta::sample::<4>(randomness, out)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19) /\
        Seq.length $serialized == 1 + v $gamma1_exponent /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (pow2 (v $gamma1_exponent) - 1)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn gamma1_serialize(simd_unit: &Self, serialized: &mut [u8], gamma1_exponent: usize) {
        encoding::gamma1::serialize(&simd_unit.value, serialized, gamma1_exponent)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19) /\
        Seq.length $serialized == 1 + v $gamma1_exponent"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (pow2 (v $gamma1_exponent))
          (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future)"#))]
    fn gamma1_deserialize(serialized: &[u8], out: &mut Self, gamma1_exponent: usize) {
        encoding::gamma1::deserialize(serialized, &mut out.value, gamma1_exponent);
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque (pow2 (v $gamma1_exponent))
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}));
            assert (forall (i: nat). i < 8 ==>
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}) i) ==
                v (Spec.Intrinsics.to_i32x8 ${out}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value (mk_u64 i)))"#
        );
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        (Seq.length $serialized == 4 \/ Seq.length $serialized == 6) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (pow2 (Seq.length $serialized) - 1)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn commitment_serialize(simd_unit: &Self, serialized: &mut [u8]) {
        encoding::commitment::serialize(&simd_unit.value, serialized)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $serialized == (match $eta with
                                   | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                   | Libcrux_ml_dsa.Constants.Eta_Four -> 4) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque
            (match $eta with
             | Libcrux_ml_dsa.Constants.Eta_Two -> 2
             | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn error_serialize(eta: Eta, simd_unit: &Self, serialized: &mut [u8]) {
        encoding::error::serialize(eta, &simd_unit.value, serialized)
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $serialized == (match $eta with
                                   | Libcrux_ml_dsa.Constants.Eta_Two -> 3
                                   | Libcrux_ml_dsa.Constants.Eta_Four -> 4)"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          ($eta == Libcrux_ml_dsa.Constants.Eta_Two ==>
              v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) >= -5 /\
              v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) <= 2) /\
          ($eta == Libcrux_ml_dsa.Constants.Eta_Four ==>
              v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) >= -11 /\
              v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) <= 4))"#))]
    fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Self) {
        encoding::error::deserialize(eta, serialized, &mut out.value);
        hax_lib::fstar!(
            r#"assert (forall (i: nat). i < 8 ==>
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}) i) ==
                v (Spec.Intrinsics.to_i32x8 ${out}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value (mk_u64 i)));
            assert ($eta == Libcrux_ml_dsa.Constants.Eta_Two \/ $eta == Libcrux_ml_dsa.Constants.Eta_Four)"#
        );
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $out == 13 /\
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${out}_future == Seq.length ${out}"#))]
    fn t0_serialize(simd_unit: &Self, out: &mut [u8]) {
        // Track 0 (c6c68bbca propagation): expose the half-open
        // (-pow2 12, pow2 12] bound in `to_i32x8`-shape so the AVX2
        // free fn pre `forall i. POW_2_BITS_IN_LOWER_PART_OF_T_MINUS_ONE
        // - to_i32x8 simd_unit i ∈ [0, pow2 13)` discharges.
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_strict_lower_array_opaque)
                (Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}));
            let _r = Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit} in
            assert (forall (i: u64). v i < 8 ==>
                v (Spec.Intrinsics.to_i32x8
                    ${simd_unit}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value i)
                  > -(pow2 12) /\
                v (Spec.Intrinsics.to_i32x8
                    ${simd_unit}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value i)
                  <= pow2 12)"#
        );
        encoding::t0::serialize(&simd_unit.value, out);
    }

    #[inline(always)]
    #[requires(serialized.len() == 13)]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
          (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future)"#))]
    fn t0_deserialize(serialized: &[u8], out: &mut Self) {
        encoding::t0::deserialize(serialized, &mut out.value);
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_strict_lower_array_opaque)
                (Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}));
            assert (forall (i: nat). i < 8 ==>
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}) i) ==
                v (Spec.Intrinsics.to_i32x8 ${out}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value (mk_u64 i)))"#
        );
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Seq.length $out == 10 /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) >= 0 /\
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) < pow2 10)"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${out}_future == Seq.length ${out}"#))]
    fn t1_serialize(simd_unit: &Self, out: &mut [u8]) {
        encoding::t1::serialize(&simd_unit.value, out);
    }

    #[inline(always)]
    #[requires(serialized.len() == 10)]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) >= 0 /\
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) < pow2 10)"#))]
    fn t1_deserialize(serialized: &[u8], out: &mut Self) {
        encoding::t1::deserialize(serialized, &mut out.value);
        hax_lib::fstar!(
            r#"Spec.Intrinsics.i32_bit_zero_lemma_to_lt_pow2_n_weak 10
                ${out}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value;
            assert (forall (i: nat). i < 8 ==>
                v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}) i) ==
                v (Spec.Intrinsics.to_i32x8 ${out}.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value (mk_u64 i)))"#
        );
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque
            (v ${specs::NTT_BASE_BOUND})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future i)))
    "#))]
    fn ntt(simd_units: &mut AVX2RingElement) {
        hax_lib::fstar!("admit ()");
        ntt::ntt(simd_units);
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque 4211177
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future i)))
    "#))]
    fn invert_ntt_montgomery(simd_units: &mut AVX2RingElement) {
        hax_lib::fstar!("admit ()");
        invntt::invert_ntt_montgomery(simd_units);
    }

    #[inline(always)]
    #[requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque 2143289343
                (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (j: nat{j < 32}) ->
          Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future j)) /\
          Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
            Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
              (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} j)) i)
              (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future j)) i)))"#))]
    fn reduce(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]) {
        reduce_with_proof(simd_units)
    }
}
