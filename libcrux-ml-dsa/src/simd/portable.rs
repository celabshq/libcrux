use crate::{
    constants::{Eta, Gamma2},
    simd::traits::*,
};

mod arithmetic;
mod vector_type;
// Some of the portable implementations are used in lieu of vectorized ones in
// the AVX2 module.
pub(crate) mod encoding;
mod invntt;
mod ntt;
mod sample;

use arithmetic::shift_left_then_reduce;
/// Portable SIMD coefficients
pub(crate) use vector_type::Coefficients as PortableSIMDUnit;
use vector_type::Coefficients;

#[cfg(hax)]
use crate::simd::traits::specs;

#[cfg(hax)]
impl Repr for Coefficients {
    fn repr(&self) -> [i32; COEFFICIENTS_IN_SIMD_UNIT] {
        self.values
    }
}

#[cfg(not(hax))]
impl Repr for Coefficients {}

// ---------------------------------------------------------------------------
// Track B (Step 10): one-line-wrapper refactor for non-trivial impl methods.
// Each `*_with_proof` free function carries the strong trait-side pre/post
// and the full proof body; the impl method below is a one-liner that
// dispatches to it.  This splits `Simd.Portable.fst impl_1`'s function-level
// VC into per-method VCs, dropping the rlimit-saturated single-query
// 9.5s impl_1 (was top-3 in fstar-perf-top20.md) into smaller per-method
// queries.
// ---------------------------------------------------------------------------

#[hax_lib::requires(fstar!(r#"v $bound > 0 /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post
        (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) $bound $result"#))]
pub(crate) fn infinity_norm_exceeds_with_proof(simd_unit: &Coefficients, bound: i32) -> bool {
    let result = arithmetic::infinity_norm_exceeds(simd_unit, bound);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post)
            (Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post);
        reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"#
    );
    result
}

#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND}) (${lhs.repr()}) /\
    Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND}) (${rhs.repr()})"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
        (Seq.index (${lhs.repr()}) i)
        (Seq.index (${rhs.repr()}) i)
        (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) i)) /\
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) i ==
      Spec.MLDSA.Math.mont_mul (Seq.index (${lhs.repr()}) i) (Seq.index (${rhs.repr()}) i))"#))]
pub(crate) fn montgomery_multiply_with_proof(lhs: &mut Coefficients, rhs: &Coefficients) {
    #[cfg(hax)]
    let _orig_lhs = lhs.clone();
    arithmetic::montgomery_multiply(lhs, rhs);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let pf (k: nat{k < 8}) : Lemma
            (ensures Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_lhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k)) =
            Libcrux_ml_dsa.Simd.Traits.Specs.lemma_montgomery_multiply_lane_intro
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_lhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${rhs}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}) k)
        in
        Classical.forall_intro pf"#
    );
}

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
    simd_unit: &mut Coefficients,
) {
    #[cfg(hax)]
    let _orig = simd_unit.clone();
    hax_lib::fstar!(
        r#"assert_norm (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}
            == ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values)"#
    );
    shift_left_then_reduce::<SHIFT_BY>(simd_unit);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
            (Spec.Utils.is_i32b_array_opaque 8380416
                ${simd_unit}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values);
        let pf (k: nat{k < 8}) : Lemma
            (ensures Libcrux_ml_dsa.Simd.Traits.Specs.shift_left_then_reduce_lane_post
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)) =
            Hacspec_ml_dsa.Commute.Chunk.lemma_shift_left_then_reduce_lane_commute_mod_q
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig}) k)
                (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)
        in
        Classical.forall_intro pf"#
    );
}

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
pub(crate) fn power2round_with_proof(t0: &mut Coefficients, t1: &mut Coefficients) {
    #[cfg(hax)]
    let _orig_t0 = t0.clone();
    arithmetic::power2round(t0, t1);
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
        // Math lemma applies; FIPS 204 Algorithm 35 has no special-case
        // adjustment, so the strict-lower bound on t0 holds (cf. F-13 for
        // why the analogous `decompose` strict-lower is unprovable).
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
                ${t0}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values);
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

// 2026-05-08: `--z3rlimit 200` (state (d)) — see Avx2 impl block comment.
#[hax_lib::fstar::options("--z3rlimit 200")]
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
pub(crate) fn reduce_with_proof(simd_units: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let _orig = simd_units.clone();

    for i in 0..simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(r#"
            v $i <= 32 /\
            (forall (j:nat{j < 32}). j < v $i ==>
                Spec.Utils.is_i32b_array_opaque 8380416
                    (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values /\
                (forall (k:nat{k < 8}).
                    Spec.MLDSA.Math.mod_q
                        (v (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)) ==
                    Spec.MLDSA.Math.mod_q
                        (v (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)))) /\
            (forall (j:nat{j < 32}). j >= v $i ==>
                Seq.index ${simd_units} j == Seq.index ${_orig} j)"#));

        arithmetic::reduce(&mut simd_units[i]);
    }

    hax_lib::fstar!(r#"
        reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let pf (j: nat{j < 32}) : Lemma
            (ensures Spec.Utils.forall8 (fun (k: nat{k < 8}) ->
                Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
                    (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
                    (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k))) =
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque 8380416
                    (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values);
            let pfk (k: nat{k < 8}) : Lemma
                (ensures Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
                    (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
                    (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_reduce_lane_commute
                    (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
                    (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
            in
            Classical.forall_intro pfk
        in
        Classical.forall_intro pf
    "#);
}

// Functional NTT surfacing (Track B).  `ntt_with_proof` carries the strong
// trait `ntt` post — the bound clause (NTT_OUTPUT_BOUND) AND the functional
// clause (output ≡ Hacspec ntt mod q, in the `ntt_poly_view` flat view) — and
// the impl `ntt` method below dispatches to it.  Extracting the discharge into
// this free fn gives it its OWN un-split VC, so the free `ntt::ntt`'s functional
// `forall` stays in scope and `lemma_ntt_func_transport` can rewrite it onto
// `ntt_poly_view` (in the combined `impl_1` query the auto-split prunes that
// hypothesis out of the f_ntt sub-query → "incomplete quantifiers").
//
// `lemma_ntt_view_portable` bridges the two flat views: `ntt_poly_view re`
// (createi over `f_repr`) equals `simd_units_to_array (chunks_of_re re)` (the
// view the free fn proves against).  For Portable `f_repr c == c.f_values`
// (defeq), so both reduce element-wise to `(re[j/8]).f_values[j%8]`.
#[cfg_attr(hax, hax_lib::fstar::before(r#"
let lemma_ntt_view_portable
      (re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (mk_usize 32))
    : Lemma (Libcrux_ml_dsa.Simd.Traits.ntt_poly_view re ==
             Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
               (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re))
  = reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.ntt_poly_view)
      (Libcrux_ml_dsa.Simd.Traits.ntt_poly_view re);
    let lhs = Libcrux_ml_dsa.Simd.Traits.ntt_poly_view re in
    let rhs = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
                (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) in
    let aux (j: nat{j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
      let b: nat = j / 8 in
      let l: nat = j % 8 in
      FStar.Math.Lemmas.euclidean_division_definition j 8;
      Hacspec_ml_dsa.Commute.Chunk.lemma_simd_units_to_array_reveal
        (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) b l;
      Hacspec_ml_dsa.createi_lemma #(t_Array i32 (mk_usize 8)) (mk_usize 32)
        #(usize -> t_Array i32 (mk_usize 8))
        (fun (b': usize{b' <. mk_usize 32}) ->
           (Seq.index re (v b')).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values)
        (mk_usize b);
      Hacspec_ml_dsa.createi_lemma #i32 (mk_usize 256)
        #(usize -> i32)
        (fun (j': usize{j' <. mk_usize 256}) ->
           Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index re (v j' / 8))) (v j' % 8))
        (mk_usize j)
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs
"#))]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
        Spec.Utils.is_i32b_array_opaque
        (v ${specs::NTT_BASE_BOUND})
        (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
        Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND})
        (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future i))) /\
    Libcrux_ml_dsa.Simd.Traits.ntt_func_post ${simd_units} ${simd_units}_future
"#))]
pub(crate) fn ntt_with_proof(simd_units: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let _orig = simd_units.clone();
    ntt::ntt(simd_units);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial)
             (Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial
                (v ${specs::NTT_BASE_BOUND} + 8 * v ${specs::FIELD_MAX}) ${simd_units});
           assert_norm (v ${specs::NTT_OUTPUT_BOUND} == v ${specs::NTT_BASE_BOUND} + 8 * v ${specs::FIELD_MAX});
           lemma_ntt_view_portable ${_orig};
           lemma_ntt_view_portable ${simd_units};
           Libcrux_ml_dsa.Simd.Traits.lemma_ntt_func_post_intro ${_orig} ${simd_units}
             (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
                (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${_orig}))
             (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
                (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${simd_units}))"#
    );
}

// Functional inverse-NTT surfacing (Track B), mirror of `ntt_with_proof`.
// `invert_ntt_with_proof` carries the tight output bound (4_211_177) AND the
// functional clause `output ≡ to_mont (Hacspec_ml_dsa.Ntt.intt input) mod q`
// (in the ntt_poly_view flat view, wrapped in the opaque `invert_func_post`
// atom).  Reuses the forward view bridge `lemma_ntt_view_portable` (same
// 32×8 flatten) and `Spec.MLDSA.Math.to_mont` (byte-identical to — hence defeq
// with — `Portable.Invntt.to_mont`, which the free fn's post uses;
// `lemma_to_mont_eq` makes that bridge explicit for the intro).
#[cfg_attr(hax, hax_lib::fstar::before(r#"
let lemma_to_mont_eq (y: t_Array i32 (mk_usize 256))
    : Lemma (Libcrux_ml_dsa.Simd.Portable.Invntt.to_mont y == Spec.MLDSA.Math.to_mont y)
  = ()
"#))]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
        (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
        Spec.Utils.is_i32b_array_opaque 4211177
        (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future i))) /\
    Libcrux_ml_dsa.Simd.Traits.invert_func_post ${simd_units} ${simd_units}_future
"#))]
pub(crate) fn invert_ntt_with_proof(simd_units: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let _orig = simd_units.clone();
    invntt::invert_ntt_montgomery(simd_units);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial)
             (Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial 4211177 ${simd_units});
           lemma_ntt_view_portable ${_orig};
           lemma_ntt_view_portable ${simd_units};
           lemma_to_mont_eq (Hacspec_ml_dsa.Ntt.intt
             (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
                (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${_orig})));
           Libcrux_ml_dsa.Simd.Traits.lemma_invert_func_post_intro ${_orig} ${simd_units}
             (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
                (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${_orig}))
             (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array
                (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${simd_units}))"#
    );
}

// 2026-06-15: `--z3rlimit 200 --split_queries always`.  TWO fixes are needed
// together for the functional `ntt` post: (1) `--split_queries always` — the
// monolithic impl_1 record query doesn't fit COLD (after re-extraction shifts
// lines, its hint goes stale and it re-proves cold → saturates at rlimit 200);
// splitting makes each field its own small sub-query.  (2) the opaque
// `ntt_func_post` atom — under split, a raw `forall` in the f_ntt post is pruned
// from its sub-query ("incomplete quantifiers"); wrapping it in the opaque atom
// makes the f_ntt dispatch propagate one atomic term.  Neither alone suffices.
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 200 --split_queries always"))]
#[hax_lib::attributes]
impl Operations for Coefficients {
    #[ensures(|result| result.repr() == [0i32; COEFFICIENTS_IN_SIMD_UNIT])]
    fn zero() -> Coefficients {
        vector_type::zero()
    }

    #[requires(array.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[ensures(|_| future(out).repr() == array)]
    fn from_coefficient_array(array: &[i32], out: &mut Coefficients) {
        vector_type::from_coefficient_array(array, out)
    }

    #[requires(out.len() == COEFFICIENTS_IN_SIMD_UNIT)]
    #[ensures(|_| future(out) == value.repr())]
    fn to_coefficient_array(value: &Coefficients, out: &mut [i32]) {
        vector_type::to_coefficient_array(value, out)
    }

    #[requires(specs::add_pre(&lhs.repr(), &rhs.repr()))]
    #[ensures(|_| specs::add_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn add(lhs: &mut Coefficients, rhs: &Coefficients) {
        arithmetic::add(lhs, rhs)
    }

    #[requires(specs::sub_pre(&lhs.repr(), &rhs.repr()))]
    #[ensures(|_| specs::sub_post(&lhs.repr(), &rhs.repr(), &future(lhs).repr()))]
    fn subtract(lhs: &mut Coefficients, rhs: &Coefficients) {
        arithmetic::subtract(lhs, rhs)
    }

    #[requires(fstar!(r#"v $bound > 0 /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|result| fstar!(r#"
        Libcrux_ml_dsa.Simd.Traits.Specs.infinity_norm_exceeds_post
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) $bound $result"#))]
    fn infinity_norm_exceeds(simd_unit: &Coefficients, bound: i32) -> bool {
        infinity_norm_exceeds_with_proof(simd_unit, bound)
    }

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
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}))"#
        );
        arithmetic::decompose(gamma2, simd_unit, low, high);
        hax_lib::fstar!(
            r#"
            // F-1 verdict (above-trait commit 7a4dc28df, option d):
            // discharge per-lane decompose_lane_post via paired commute lemma.
            // The arithmetic post gives `v low[k] == r0_s /\ v high[k] == r1_s`
            // (per-lane Spec.MLDSA.Math.decompose match) which is exactly the
            // commute lemma's precondition.  We help Z3 by extracting the
            // per-lane post via the forall-i instantiation explicitly.
            let pf_eq (k: nat{k < 8}) : Lemma
                (ensures Libcrux_ml_dsa.Simd.Traits.Specs.decompose_lane_post
                    $gamma2
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)) =
                let r = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k in
                let r0 = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k in
                let r1 = Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k in
                let (r0_s, r1_s, _) = Spec.MLDSA.Math.decompose (v $gamma2) (v r) in
                assert (v r0 == r0_s /\ v r1 == r1_s);
                Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_lane_commute_conditional
                    $gamma2 r r0 r1
            in
            Classical.forall_intro pf_eq;
            // Bound conjuncts: per-lane bounds from arithmetic post → array-level
            // is_i32b_array opaque on both gamma2 branches.
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
    fn compute_hint(
        low: &Coefficients,
        high: &Coefficients,
        gamma2: i32,
        hint: &mut Coefficients,
    ) -> usize {
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}))"#
        );
        let result = arithmetic::compute_hint(low, high, gamma2, hint);
        hax_lib::fstar!(
            r#"
            // F-1 verdict (above-trait commit 7a4dc28df, option d):
            // discharge bound + conditional equation via paired commute lemmas.
            let pf_eq (k: nat{k < 8}) : Lemma
                (ensures Libcrux_ml_dsa.Simd.Traits.Specs.compute_hint_lane_post
                    $gamma2
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_compute_hint_lane_commute_conditional
                    $gamma2
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k)
            in
            Classical.forall_intro pf_eq;
            // Bound conjunct: per-lane hint ∈ {0, 1} → array-level binary.
            let pf_bound (k: nat{k < 8}) : Lemma
                (ensures
                    v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k) == 0 \/
                    v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k) == 1) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_compute_one_hint_bound
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${low}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${high}) k)
                    $gamma2
            in
            Classical.forall_intro pf_bound;
            reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque)
                (Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}));
            // Discharge `v result <= 8` from the spec popcount under binary
            // hint (each lane ∈ {0, 1}, so total ≤ 8).
            Hacspec_ml_dsa.Commute.Chunk.lemma_compute_hint_bound
                (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint})"#
        );
        result
    }

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
    fn use_hint(gamma2: Gamma2, simd_unit: &Coefficients, hint: &mut Coefficients) {
        #[cfg(hax)]
        let _orig_hint = hint.clone();
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque)
                (Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}));
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}))"#
        );
        arithmetic::use_hint(gamma2, simd_unit, hint);
        hax_lib::fstar!(
            r#"
            // F-1 verdict (above-trait commit 7a4dc28df, option d):
            // discharge bound + conditional equation via paired commute lemmas.
            let pf_eq (k: nat{k < 8}) : Lemma
                (ensures Libcrux_ml_dsa.Simd.Traits.Specs.use_hint_lane_post
                    $gamma2
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_hint}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_use_hint_lane_commute_conditional
                    $gamma2
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_hint}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k)
            in
            Classical.forall_intro pf_eq;
            // Bound conjunct: per-lane [0, 44) or [0, 16) → array-level opaque.
            let pf_bound (k: nat{k < 8}) : Lemma
                (ensures
                    (v $gamma2 == 95232 ==>
                        0 <= v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k) /\
                        v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k) < 44) /\
                    (v $gamma2 == 261888 ==>
                        0 <= v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k) /\
                        v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}) k) < 16)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_use_one_hint_bound
                    $gamma2
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) k)
                    (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${_orig_hint}) k)
            in
            Classical.forall_intro pf_bound;
            // Reveal both array-level bound predicates so F* can fold the
            // per-lane bounds into the array-level form on either branch.
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque 44
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}));
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque 16
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${hint}))"#
        );
    }

    // Both operands NTT_OUTPUT_BOUND (matches widened trait pre); output FIELD_MAX.
    #[requires(fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND}) (${lhs.repr()}) /\
        Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND}) (${rhs.repr()})"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX}) (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.montgomery_multiply_lane_post
            (Seq.index (${lhs.repr()}) i)
            (Seq.index (${rhs.repr()}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${lhs}_future) i))"#))]
    fn montgomery_multiply(lhs: &mut Coefficients, rhs: &Coefficients) {
        montgomery_multiply_with_proof(lhs, rhs)
    }

    #[requires(fstar!(r#"v $SHIFT_BY == 13 /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
            v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) >= 0 /\
            v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) <= 261631)"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          Libcrux_ml_dsa.Simd.Traits.Specs.shift_left_then_reduce_lane_post
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i)
            (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}_future) i))"#))]
    fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Coefficients) {
        shift_left_then_reduce_with_proof::<SHIFT_BY>(simd_unit)
    }

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
    fn power2round(t0: &mut Coefficients, t1: &mut Coefficients) {
        power2round_with_proof(t0, t1)
    }

    #[requires(fstar!(r#"
        Seq.length $randomness == 24 /\
        Seq.length $randomness / 3 <= 4294967295 /\
        Seq.length $randomness / 3 <= Seq.length $out"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= 0 /\
          v (Seq.index ${out}_future i) < 8380417)"#))]
    fn rejection_sample_less_than_field_modulus(randomness: &[u8], out: &mut [i32]) -> usize {
        sample::rejection_sample_less_than_field_modulus(randomness, out)
    }

    #[requires(fstar!(r#"
        Seq.length $randomness == 4 /\
        Seq.length $randomness * 2 <= 4294967295 /\
        Seq.length $randomness * 2 <= Seq.length $out"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= -2 /\ v (Seq.index ${out}_future i) <= 2)"#))]
    fn rejection_sample_less_than_eta_equals_2(randomness: &[u8], out: &mut [i32]) -> usize {
        sample::rejection_sample_less_than_eta_equals_2(randomness, out)
    }

    #[requires(fstar!(r#"
        Seq.length $randomness == 4 /\
        Seq.length $randomness * 2 <= 4294967295 /\
        Seq.length $randomness * 2 <= Seq.length $out"#))]
    #[ensures(|result| fstar!(r#"v $result <= 8 /\
        Seq.length ${out}_future == Seq.length $out /\
        (forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==>
          v (Seq.index ${out}_future i) >= -4 /\ v (Seq.index ${out}_future i) <= 4)"#))]
    fn rejection_sample_less_than_eta_equals_4(randomness: &[u8], out: &mut [i32]) -> usize {
        sample::rejection_sample_less_than_eta_equals_4(randomness, out)
    }

    #[requires(fstar!(r#"
        (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19) /\
        Seq.length $serialized == 1 + v $gamma1_exponent /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (pow2 (v $gamma1_exponent) - 1)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn gamma1_serialize(simd_unit: &Coefficients, serialized: &mut [u8], gamma1_exponent: usize) {
        encoding::gamma1::serialize(simd_unit, serialized, gamma1_exponent)
    }

    #[requires(fstar!(r#"
        (v $gamma1_exponent == 17 \/ v $gamma1_exponent == 19) /\
        Seq.length $serialized == 1 + v $gamma1_exponent"#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_array_opaque (pow2 (v $gamma1_exponent))
          (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future)"#))]
    fn gamma1_deserialize(serialized: &[u8], out: &mut Coefficients, gamma1_exponent: usize) {
        encoding::gamma1::deserialize(serialized, out, gamma1_exponent);
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque (pow2 (v $gamma1_exponent))
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}))"#
        );
    }

    #[requires(fstar!(r#"
        (Seq.length $serialized == 4 \/ Seq.length $serialized == 6) /\
        Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (pow2 (Seq.length $serialized) - 1)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
    fn commitment_serialize(simd_unit: &Coefficients, serialized: &mut [u8]) {
        encoding::commitment::serialize(simd_unit, serialized)
    }

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
    fn error_serialize(eta: Eta, simd_unit: &Coefficients, serialized: &mut [u8]) {
        encoding::error::serialize(eta, simd_unit, serialized)
    }

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
    fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Coefficients) {
        encoding::error::deserialize(eta, serialized, out);
    }

    #[requires(fstar!(r#"
        Seq.length $out == 13 /\
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
            (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit})"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${out}_future == Seq.length ${out}"#))]
    fn t0_serialize(simd_unit: &Coefficients, out: &mut [u8]) {
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_strict_lower_array_opaque)
                (Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}))"#
        );
        encoding::t0::serialize(simd_unit, out)
    }

    #[requires(serialized.len() == 13)]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
          (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future)"#))]
    fn t0_deserialize(serialized: &[u8], out: &mut Coefficients) {
        encoding::t0::deserialize(serialized, out);
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i32b_strict_lower_array_opaque)
                (Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
                    (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}))"#
        );
    }

    #[requires(fstar!(r#"
        Seq.length $out == 10 /\
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) >= 0 /\
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${simd_unit}) i) < pow2 10)"#))]
    #[ensures(|_| fstar!(r#"
        Seq.length ${out}_future == Seq.length ${out}"#))]
    fn t1_serialize(simd_unit: &Self, out: &mut [u8]) {
        encoding::t1::serialize(simd_unit, out);
    }

    #[requires(serialized.len() == 10)]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) >= 0 /\
          v (Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr ${out}_future) i) < pow2 10)"#))]
    fn t1_deserialize(serialized: &[u8], out: &mut Self) {
        encoding::t1::deserialize(serialized, out);
    }

    #[requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque
            (v ${specs::NTT_BASE_BOUND})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque (v ${specs::NTT_OUTPUT_BOUND})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future i))) /\
        Libcrux_ml_dsa.Simd.Traits.ntt_func_post ${simd_units} ${simd_units}_future
    "#))]
    fn ntt(simd_units: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
        ntt_with_proof(simd_units)
    }

    #[requires(fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units} i)))
    "#))]
    #[ensures(|_| fstar!(r#"
        Spec.Utils.forall32 (fun (i: nat{i < 32}) ->
            Spec.Utils.is_i32b_array_opaque 4211177
            (Libcrux_ml_dsa.Simd.Traits.f_repr (Seq.index ${simd_units}_future i))) /\
        Libcrux_ml_dsa.Simd.Traits.invert_func_post ${simd_units} ${simd_units}_future
    "#))]
    fn invert_ntt_montgomery(simd_units: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
        invert_ntt_with_proof(simd_units)
    }

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
