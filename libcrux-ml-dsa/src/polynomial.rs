use crate::simd::traits::{Operations, COEFFICIENTS_IN_SIMD_UNIT, SIMD_UNITS_IN_RING_ELEMENT};

#[cfg(hax)]
use crate::simd::traits::specs::*;

#[derive(Clone, Copy)]
#[hax_lib::fstar::after("open Libcrux_ml_dsa.Simd.Traits.Specs")]
pub(crate) struct PolynomialRingElement<SIMDUnit: Operations> {
    pub(crate) simd_units: [SIMDUnit; SIMD_UNITS_IN_RING_ELEMENT],
}

/// Spec helpers for stating bounds at the polynomial level (mirrors
/// `Libcrux_ml_kem.Polynomial.spec`).  Use these in matrix/vector wrappers
/// to avoid nested-forall trigger explosion in SMT search.
#[cfg(hax)]
pub(crate) mod spec {
    use crate::polynomial::PolynomialRingElement;
    use crate::simd::traits::Operations;

    pub(crate) fn is_bounded_simd_unit<SIMDUnit: Operations>(b: usize, vec: &SIMDUnit) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"Spec.Utils.is_i32b_array_opaque (v b) (i0._super_i2.f_repr vec)"#
        )
    }

    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_bounded_poly_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
      (j: nat{j < 32})
    : Lemma
      (requires is_bounded_poly b p)
      (ensures Spec.Utils.is_i32b_array_opaque (v b)
                 (i0._super_i2.f_repr (Seq.index p.f_simd_units j)))
      [SMTPat (Spec.Utils.is_i32b_array_opaque (v b)
                 (i0._super_i2.f_repr (Seq.index p.f_simd_units j)));
       SMTPat (is_bounded_poly b p)]
  = reveal_opaque (`%is_bounded_poly) (is_bounded_poly b p)

let lemma_is_bounded_poly_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
    : Lemma
      (requires forall (j: nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque (v b)
          (i0._super_i2.f_repr (Seq.index p.f_simd_units j)))
      (ensures is_bounded_poly b p)
  = reveal_opaque (`%is_bounded_poly) (is_bounded_poly b p)

(* Monotonicity: tighter bound implies looser bound. *)
let lemma_is_bounded_poly_higher
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b1 b2: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
    : Lemma
      (requires is_bounded_poly b1 p /\ v b1 <= v b2)
      (ensures is_bounded_poly b2 p)
  = reveal_opaque (`%is_bounded_poly) (is_bounded_poly b1 p);
    reveal_opaque (`%is_bounded_poly) (is_bounded_poly b2 p);
    let lemma_lane (j: nat{j < 32}) :
      Lemma (Spec.Utils.is_i32b_array_opaque (v b2)
               (i0._super_i2.f_repr (Seq.index p.f_simd_units j))) =
      reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) Spec.Utils.is_i32b_array_opaque in
    Classical.forall_intro lemma_lane
"#
        )
    )]
    pub(crate) fn is_bounded_poly<SIMDUnit: Operations>(
        b: usize,
        p: &PolynomialRingElement<SIMDUnit>,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque (v b)
                  (i0._super_i2.f_repr (p.f_simd_units.[ sz i ]))"#
        )
    }

    /// All entries of `arr` in the half-open index range `[lo, hi)` are
    /// bounded by `b` at the polynomial level. Made `opaque_to_smt` to
    /// stop quantifier cascades when this predicate appears in many
    /// hypotheses (e.g. nested loop invariants in `compute_matrix_x_mask`,
    /// `compute_as1_plus_s2`). The body conjoins `k < Seq.length arr`
    /// with `k < v hi` so `Seq.index arr k` is well-typed without a
    /// `requires` (avoiding `Pure` overhead at every use site).
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_bounded_poly_range_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b lo hi: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
      (k: nat)
    : Lemma
      (requires is_bounded_poly_range b lo hi arr /\
                v lo <= k /\ k < v hi /\ k < Seq.length arr)
      (ensures Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr k))
      [SMTPat (is_bounded_poly_range b lo hi arr); SMTPat (Seq.index arr k)]
  = reveal_opaque (`%is_bounded_poly_range) (is_bounded_poly_range b lo hi arr)

let lemma_is_bounded_poly_range_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b lo hi: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires forall (k: nat). v lo <= k /\ k < v hi /\ k < Seq.length arr ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr k))
      (ensures is_bounded_poly_range b lo hi arr)
  = reveal_opaque (`%is_bounded_poly_range) (is_bounded_poly_range b lo hi arr)

(* Extend an opaque is_bounded_poly_range carryover by one more index, given
   the bound on the new entry and a frame from the prior arr.  Verified in
   its own clean context so callers (e.g. `compute_w_approx`'s post-body
   inv re-establishment) avoid the cascade pollution that would otherwise
   trip Z3 on trivial assertions like `k = v i` under heavy ambient context. *)
let lemma_is_bounded_poly_range_extend_after_update
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (i: usize)
      (arr_old arr_new:
          t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires
        Seq.length arr_new == Seq.length arr_old /\
        v i < Seq.length arr_new /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range b (mk_usize 0) i arr_old /\
        (forall (k:nat). k < Seq.length arr_new /\ k <> v i ==>
          Seq.index arr_new k == Seq.index arr_old k) /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr_new (v i)))
      (ensures
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
          b (mk_usize 0) (i +! mk_usize 1) arr_new)
  = let aux (k: nat{k < v i + 1 /\ k < Seq.length arr_new}) :
      Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr_new k)) =
      if k < v i then begin
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
          b (mk_usize 0) i arr_old k;
        assert (Seq.index arr_new k == Seq.index arr_old k)
      end
      else
        assert (k == v i)
    in
    Classical.forall_intro aux;
    Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
      b (mk_usize 0) (i +! mk_usize 1) arr_new
"#
        )
    )]
    pub(crate) fn is_bounded_poly_range<SIMDUnit: Operations>(
        b: usize,
        lo: usize,
        hi: usize,
        arr: &[PolynomialRingElement<SIMDUnit>],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (k:nat). v lo <= k /\ k < v hi /\ k < Seq.length arr ==>
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr k)"#
        )
    }

    /// All entries of `arr` are bounded by `b` at the polynomial level.
    /// Made `opaque_to_smt` to stop quantifier cascades when this
    /// predicate appears in many hypotheses (e.g. function preconditions
    /// like `compute_as1_plus_s2`).
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_bounded_poly_slice_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
      (k: nat)
    : Lemma
      (requires is_bounded_poly_slice b arr /\ k < Seq.length arr)
      (ensures Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr k))
      [SMTPat (is_bounded_poly_slice b arr); SMTPat (Seq.index arr k)]
  = reveal_opaque (`%is_bounded_poly_slice) (is_bounded_poly_slice b arr)

let lemma_is_bounded_poly_slice_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires forall (k: nat). k < Seq.length arr ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr k))
      (ensures is_bounded_poly_slice b arr)
  = reveal_opaque (`%is_bounded_poly_slice) (is_bounded_poly_slice b arr)
"#
        )
    )]
    pub(crate) fn is_bounded_poly_slice<SIMDUnit: Operations>(
        b: usize,
        arr: &[PolynomialRingElement<SIMDUnit>],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (k:nat). k < Seq.length arr ==>
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b (Seq.index arr k)"#
        )
    }

    /// Per-poly predicate: every lane coefficient is strictly bounded above
    /// by `b` in the half-open sense (`is_i32b_strict_lower_array_opaque`,
    /// the `(-b, b]` shape used by t0 serialize/deserialize).  Mirror of
    /// `is_bounded_poly` but wrapping the strict-lower atom.  Made
    /// `opaque_to_smt` so it appears as a single atom in pre/inv, dropping
    /// the 2-deep `forall k j.` from quantifier search context.
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_strict_lower_poly_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
      (j: nat{j < 32})
    : Lemma
      (requires is_strict_lower_poly b p)
      (ensures Spec.Utils.is_i32b_strict_lower_array_opaque (v b)
                 (i0._super_i2.f_repr (Seq.index p.f_simd_units j)))
      [SMTPat (Spec.Utils.is_i32b_strict_lower_array_opaque (v b)
                 (i0._super_i2.f_repr (Seq.index p.f_simd_units j)));
       SMTPat (is_strict_lower_poly b p)]
  = reveal_opaque (`%is_strict_lower_poly) (is_strict_lower_poly b p)

let lemma_is_strict_lower_poly_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
    : Lemma
      (requires forall (j: nat). j < 32 ==>
        Spec.Utils.is_i32b_strict_lower_array_opaque (v b)
          (i0._super_i2.f_repr (Seq.index p.f_simd_units j)))
      (ensures is_strict_lower_poly b p)
  = reveal_opaque (`%is_strict_lower_poly) (is_strict_lower_poly b p)
"#
        )
    )]
    pub(crate) fn is_strict_lower_poly<SIMDUnit: Operations>(
        b: usize,
        p: &PolynomialRingElement<SIMDUnit>,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_strict_lower_array_opaque (v b)
                  (i0._super_i2.f_repr (p.f_simd_units.[ sz i ]))"#
        )
    }

    /// All entries of `arr` are strict-lower bounded by `b` at the
    /// polynomial level.  Made `opaque_to_smt` to stop quantifier
    /// cascades when this predicate appears in many hypotheses (e.g.
    /// `generate_serialized`'s t0 precondition / loop invariant).
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_strict_lower_poly_slice_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
      (k: nat)
    : Lemma
      (requires is_strict_lower_poly_slice b arr /\ k < Seq.length arr)
      (ensures Libcrux_ml_dsa.Polynomial.Spec.is_strict_lower_poly b (Seq.index arr k))
      [SMTPat (is_strict_lower_poly_slice b arr); SMTPat (Seq.index arr k)]
  = reveal_opaque (`%is_strict_lower_poly_slice) (is_strict_lower_poly_slice b arr)

let lemma_is_strict_lower_poly_slice_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires forall (k: nat). k < Seq.length arr ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_strict_lower_poly b (Seq.index arr k))
      (ensures is_strict_lower_poly_slice b arr)
  = reveal_opaque (`%is_strict_lower_poly_slice) (is_strict_lower_poly_slice b arr)
"#
        )
    )]
    pub(crate) fn is_strict_lower_poly_slice<SIMDUnit: Operations>(
        b: usize,
        arr: &[PolynomialRingElement<SIMDUnit>],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (k:nat). k < Seq.length arr ==>
                  Libcrux_ml_dsa.Polynomial.Spec.is_strict_lower_poly b (Seq.index arr k)"#
        )
    }

    /// Per-poly predicate: every lane coefficient is in the inclusive
    /// range `[lo, hi]`.  Asymmetric counterpart of `is_bounded_poly`
    /// (which is `|x| < b`, symmetric around 0).  Used for things like
    /// the T1-decoded polynomial entries (`lo = 0`, `hi = 261631`)
    /// that `shift_left_then_reduce` requires on its input.  Made
    /// `opaque_to_smt` so it appears as a single atom in pre/inv,
    /// dropping the 2-deep `forall j m.` from quantifier search context.
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_lane_range_poly_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (lo hi: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
      (j: nat{j < 32}) (m: nat{m < 8})
    : Lemma
      (requires is_lane_range_poly lo hi p)
      (ensures
        v (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m) >= v lo /\
        v (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m) <= v hi)
      [SMTPat (is_lane_range_poly lo hi p);
       SMTPat (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m)]
  = reveal_opaque (`%is_lane_range_poly) (is_lane_range_poly lo hi p)

let lemma_is_lane_range_poly_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (lo hi: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
    : Lemma
      (requires forall (j:nat). j < 32 ==>
        (forall (m:nat). m < 8 ==>
          v (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m) >= v lo /\
          v (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m) <= v hi))
      (ensures is_lane_range_poly lo hi p)
  = reveal_opaque (`%is_lane_range_poly) (is_lane_range_poly lo hi p)
"#
        )
    )]
    pub(crate) fn is_lane_range_poly<SIMDUnit: Operations>(
        lo: usize,
        hi: usize,
        p: &PolynomialRingElement<SIMDUnit>,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (j:nat). j < 32 ==>
                (forall (m:nat). m < 8 ==>
                  v (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m) >= v lo /\
                  v (Seq.index (i0._super_i2.f_repr (Seq.index p.f_simd_units j)) m) <= v hi)"#
        )
    }

    /// Slice version of `is_lane_range_poly`: every entry of `arr` has all
    /// lane coefficients in `[lo, hi]`.  Made `opaque_to_smt`.
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
let lemma_is_lane_range_poly_slice_lookup
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (lo hi: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
      (k: nat)
    : Lemma
      (requires is_lane_range_poly_slice lo hi arr /\ k < Seq.length arr)
      (ensures Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly lo hi (Seq.index arr k))
      [SMTPat (is_lane_range_poly_slice lo hi arr); SMTPat (Seq.index arr k)]
  = reveal_opaque (`%is_lane_range_poly_slice) (is_lane_range_poly_slice lo hi arr)

let lemma_is_lane_range_poly_slice_intro
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (lo hi: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires forall (k: nat). k < Seq.length arr ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly lo hi (Seq.index arr k))
      (ensures is_lane_range_poly_slice lo hi arr)
  = reveal_opaque (`%is_lane_range_poly_slice) (is_lane_range_poly_slice lo hi arr)

(* Bridge + widen: lanes in [0, b1] (asymmetric, b1 <= b2) imply
   |lane| <= b2 (symmetric, possibly looser).  Collapses the
   lane-range -> bounded-poly conversion plus an upper-bound widening
   into one call so callers don't expand three nested
   Classical.forall_intro chains.  Used in keygen to bridge from
   sample_s1_and_s2's `is_lane_range_poly_slice 0 eta_val` post to
   downstream consumers wanting `is_bounded_poly_slice 4` (compute_as1)
   or `is_bounded_poly_slice 8380416` (ntt).  No SMTPats — caller
   invokes once per target bound. *)
let lemma_lane_range_pos_to_bounded_poly
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b1 b2: usize)
      (p: Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
    : Lemma
      (requires Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly
                  (mk_usize 0) b1 p /\ v b1 <= v b2)
      (ensures Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b2 p)
  = let lemma_lane (j: nat{j < 32})
        : Lemma (Spec.Utils.is_i32b_array_opaque (v b2)
                   (i0._super_i2.f_repr (Seq.index p.f_simd_units j))) =
      let lane_arr = i0._super_i2.f_repr (Seq.index p.f_simd_units j) in
      let aux_m (m: nat{m < 8})
          : Lemma (Spec.Utils.is_i32b (v b2) (Seq.index lane_arr m)) =
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_lookup
          (mk_usize 0) b1 p j m
      in
      Classical.forall_intro aux_m;
      reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                    (Spec.Utils.is_i32b_array_opaque (v b2) lane_arr)
    in
    Classical.forall_intro lemma_lane;
    Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro b2 p

let lemma_lane_range_pos_to_bounded_poly_slice
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b1 b2: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
                  (mk_usize 0) b1 arr /\ v b1 <= v b2)
      (ensures Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice b2 arr)
  = let aux (k: nat{k < Seq.length arr})
        : Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b2 (Seq.index arr k)) =
      Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_lookup
        (mk_usize 0) b1 arr k;
      lemma_lane_range_pos_to_bounded_poly b1 b2 (Seq.index arr k)
    in
    Classical.forall_intro aux;
    Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro b2 arr

(* Bridge: lanes in [0, b] (asymmetric opaque atom) imply per-simd-unit
   `is_pos_array_opaque b` for every (k, j) — the bare-forall shape that
   `signing_key::generate_serialized`'s `s1_2` pre wants.  Caller invokes
   once just before the call site that needs the bare form. *)
let lemma_lane_range_pos_to_pos_array_slice
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (b: usize)
      (arr: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
    : Lemma
      (requires Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
                  (mk_usize 0) b arr)
      (ensures forall (k: nat). k < Seq.length arr ==>
                 (forall (j: nat). j < 32 ==>
                    Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (v b)
                      (i0._super_i2.f_repr (Seq.index (Seq.index arr k).f_simd_units j))))
  = let aux (k: nat{k < Seq.length arr}) (j: nat{j < 32})
        : Lemma (Libcrux_ml_dsa.Simd.Traits.Specs.is_pos_array_opaque (v b)
                   (i0._super_i2.f_repr (Seq.index (Seq.index arr k).f_simd_units j))) =
      Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_lookup
        (mk_usize 0) b arr k;
      let p = Seq.index arr k in
      let lane_arr = i0._super_i2.f_repr (Seq.index p.f_simd_units j) in
      let aux_m (m: nat{m < 8})
          : Lemma (v (Seq.index lane_arr m) >= 0 /\ v (Seq.index lane_arr m) <= v b) =
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_lookup
          (mk_usize 0) b p j m
      in
      Classical.forall_intro aux_m;
      Libcrux_ml_dsa.Simd.Traits.Specs.lemma_is_pos_array_intro (v b) lane_arr
    in
    Classical.forall_intro_2 aux
"#
        )
    )]
    pub(crate) fn is_lane_range_poly_slice<SIMDUnit: Operations>(
        lo: usize,
        hi: usize,
        arr: &[PolynomialRingElement<SIMDUnit>],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall (k:nat). k < Seq.length arr ==>
                  Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly lo hi (Seq.index arr k)"#
        )
    }
}

#[hax_lib::attributes]
impl<SIMDUnit: Operations> PolynomialRingElement<SIMDUnit> {
    #[hax_lib::ensures(|result| fstar!(r#"forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque 0
            (i0._super_i2.f_repr (Seq.index ${result}.f_simd_units j))"#))]
    pub(crate) fn zero() -> Self {
        let s = Self {
            simd_units: [SIMDUnit::zero(); SIMD_UNITS_IN_RING_ELEMENT],
        };
        hax_lib::fstar!(r#"
          let lemma_lane (j:nat{j < 32}) :
            Lemma (Spec.Utils.is_i32b_array_opaque 0
                     (i0._super_i2.f_repr (Seq.index ${s}.f_simd_units j))) =
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) Spec.Utils.is_i32b_array_opaque
          in
          Classical.forall_intro lemma_lane
        "#);
        s
    }

    // This is used in `make_hint` and for tests
    pub(crate) fn to_i32_array(&self) -> [i32; 256] {
        let mut result = [0i32; 256];

        for i in 0..self.simd_units.len() {
            SIMDUnit::to_coefficient_array(
                &self.simd_units[i],
                &mut result[i * COEFFICIENTS_IN_SIMD_UNIT..(i + 1) * COEFFICIENTS_IN_SIMD_UNIT],
            );
        }

        result
    }

    #[hax_lib::requires(array.len() == 256)]
    #[hax_lib::ensures(|_| fstar!(r#"
        forall (kk:nat). kk < 32 ==>
           i0._super_i2.f_repr (Seq.index ${result}_future.f_simd_units kk) ==
             Seq.slice $array (kk * 8) ((kk + 1) * 8)"#))]
    pub(crate) fn from_i32_array(array: &[i32], result: &mut Self) {
        for i in 0..SIMD_UNITS_IN_RING_ELEMENT {
            hax_lib::loop_invariant!(|i: usize| fstar!(r#"
                forall (kk:nat). kk < v ${i} ==>
                   i0._super_i2.f_repr (Seq.index result.f_simd_units kk) ==
                     Seq.slice $array (kk * 8) ((kk + 1) * 8)"#));
            SIMDUnit::from_coefficient_array(
                &array[i * COEFFICIENTS_IN_SIMD_UNIT..(i + 1) * COEFFICIENTS_IN_SIMD_UNIT],
                &mut result.simd_units[i],
            );
        }
    }

    #[cfg(test)]
    pub(crate) fn from_i32_array_test(array: &[i32]) -> Self {
        let mut result = PolynomialRingElement::zero();
        Self::from_i32_array(array, &mut result);
        result
    }

    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"v $bound > 0 /\ 
        (forall i. Spec.Utils.is_i32b_array_opaque 
            (v ${FIELD_MAX}) 
            (i0._super_i2.f_repr (Seq.index self.f_simd_units i)))"#))]
    pub(crate) fn infinity_norm_exceeds(&self, bound: i32) -> bool {
        let mut result = false;
        for i in 0..self.simd_units.len() {
            result = result || SIMDUnit::infinity_norm_exceeds(&self.simd_units[i], bound);
        }

        result
    }

    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"forall i.
        add_pre (i0._super_i2.f_repr (Seq.index self.f_simd_units i))
                (i0._super_i2.f_repr (Seq.index rhs.f_simd_units i))"#))]
    #[hax_lib::ensures(|_| fstar!(r#"forall (j:nat). j < 32 ==>
        add_post (i0._super_i2.f_repr (Seq.index self.f_simd_units j))
                 (i0._super_i2.f_repr (Seq.index rhs.f_simd_units j))
                 (i0._super_i2.f_repr (Seq.index self_e_future.f_simd_units j))"#))]
    pub(crate) fn add(&mut self, rhs: &Self) {
        #[cfg(hax)]
        let old_self = self.clone();

        for i in 0..self.simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"v i <= 32 /\
                  (forall (j:nat). j >= v i /\ j < 32 ==>
                            Seq.index self.f_simd_units j ==
                            Seq.index old_self.f_simd_units j) /\
                  (forall (j:nat). j < v i ==>
                            add_post (i0._super_i2.f_repr (Seq.index old_self.f_simd_units j))
                                     (i0._super_i2.f_repr (Seq.index rhs.f_simd_units j))
                                     (i0._super_i2.f_repr (Seq.index self.f_simd_units j)))"#
            ));

            SIMDUnit::add(&mut self.simd_units[i], &rhs.simd_units[i]);
        }
    }

    /// `add` with explicit bound parameters. Lifts `bounded_add_post` (the
    /// per-simd-unit SMTPat in `Specs.fst`) to the polynomial level so callers
    /// can chain bounds across nested loops without paying the cost of the
    /// per-lane forall expansion at every call site. Mirrors ML-KEM's
    /// `add_to_ring_element`/`add_bounded` recipe.
    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"
        v $_b1 + v $_b2 <= 2147483647 /\
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v $_b1)
                (i0._super_i2.f_repr (Seq.index self.f_simd_units j))) /\
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v $_b2)
                (i0._super_i2.f_repr (Seq.index rhs.f_simd_units j)))"#))]
    #[hax_lib::ensures(|_| fstar!(r#"forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque (v $_b1 + v $_b2)
            (i0._super_i2.f_repr (Seq.index self_e_future.f_simd_units j))"#))]
    pub(crate) fn add_bounded(&mut self, _b1: usize, rhs: &Self, _b2: usize) {
        #[cfg(hax)]
        let old_self = self.clone();

        for i in 0..self.simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"v i <= 32 /\
                  (forall (j:nat). j >= v i /\ j < 32 ==>
                            Seq.index self.f_simd_units j ==
                            Seq.index old_self.f_simd_units j) /\
                  (forall (j:nat). j < v i ==>
                    Spec.Utils.is_i32b_array_opaque (v $_b1 + v $_b2)
                      (i0._super_i2.f_repr (Seq.index self.f_simd_units j)))"#
            ));

            SIMDUnit::add(&mut self.simd_units[i], &rhs.simd_units[i]);
        }
    }

    /// `subtract` with explicit bound parameters; mirrors `add_bounded`.
    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"
        v $_b1 + v $_b2 <= 2147483647 /\
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v $_b1)
                (i0._super_i2.f_repr (Seq.index self.f_simd_units j))) /\
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v $_b2)
                (i0._super_i2.f_repr (Seq.index rhs.f_simd_units j)))"#))]
    #[hax_lib::ensures(|_| fstar!(r#"forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque (v $_b1 + v $_b2)
            (i0._super_i2.f_repr (Seq.index self_e_future.f_simd_units j))"#))]
    pub(crate) fn subtract_bounded(&mut self, _b1: usize, rhs: &Self, _b2: usize) {
        #[cfg(hax)]
        let old_self = self.clone();

        for i in 0..self.simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"v i <= 32 /\
                  (forall (j:nat). j >= v i /\ j < 32 ==>
                            Seq.index self.f_simd_units j ==
                            Seq.index old_self.f_simd_units j) /\
                  (forall (j:nat). j < v i ==>
                    Spec.Utils.is_i32b_array_opaque (v $_b1 + v $_b2)
                      (i0._super_i2.f_repr (Seq.index self.f_simd_units j)))"#
            ));

            SIMDUnit::subtract(&mut self.simd_units[i], &rhs.simd_units[i]);
        }
    }

    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"forall i.
        sub_pre (i0._super_i2.f_repr (Seq.index self.f_simd_units i))
                (i0._super_i2.f_repr (Seq.index rhs.f_simd_units i))"#))]
    #[hax_lib::ensures(|_| fstar!(r#"forall (j:nat). j < 32 ==>
        sub_post (i0._super_i2.f_repr (Seq.index self.f_simd_units j))
                 (i0._super_i2.f_repr (Seq.index rhs.f_simd_units j))
                 (i0._super_i2.f_repr (Seq.index self_e_future.f_simd_units j))"#))]
    pub(crate) fn subtract(&mut self, rhs: &Self) {
        #[cfg(hax)]
        let old_self = self.clone();

        for i in 0..self.simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"v i <= 32 /\
                  (forall (j:nat). j >= v i /\ j < 32 ==>
                        Seq.index self.f_simd_units j ==
                        Seq.index old_self.f_simd_units j) /\
                  (forall (j:nat). j < v i ==>
                        sub_post (i0._super_i2.f_repr (Seq.index old_self.f_simd_units j))
                                 (i0._super_i2.f_repr (Seq.index rhs.f_simd_units j))
                                 (i0._super_i2.f_repr (Seq.index self.f_simd_units j)))"#
            ));

            SIMDUnit::subtract(&mut self.simd_units[i], &rhs.simd_units[i]);
        }
    }
}
