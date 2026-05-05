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
    pub(crate) fn from_i32_array(array: &[i32], result: &mut Self) {
        for i in 0..SIMD_UNITS_IN_RING_ELEMENT {
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
