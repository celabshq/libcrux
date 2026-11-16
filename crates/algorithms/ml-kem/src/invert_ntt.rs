use crate::{
    hax_utils::hax_debug_assert,
    polynomial::{zeta, PolynomialRingElement},
    vector::{Operations, FIELD_ELEMENTS_IN_VECTOR},
};

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning"))]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        interface,
        "[@@ \"opaque_to_smt\"]
   let invert_ntt_re_range_2 (#v_Vector: Type0)
           {| i1: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
           (re: Libcrux_ml_kem.Polynomial.t_PolynomialRingElement v_Vector) =
       forall (i:nat). i < 16 ==> Spec.Utils.is_i16b_array_opaque 3328
               (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ]))"
    )
)]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        interface,
        "[@@ \"opaque_to_smt\"]
   let invert_ntt_re_range_1 (#v_Vector: Type0)
         {| i1: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
         (re: Libcrux_ml_kem.Polynomial.t_PolynomialRingElement v_Vector) =
       forall (i:nat). i < 16 ==> Spec.Utils.is_i16b_array_opaque (4 * 3328)
            (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ]))"
    )
)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v ${*zeta_i} == 128 /\
    invert_ntt_re_range_1 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"invert_ntt_re_range_2 ${re}_future /\
    v ${*zeta_i}_future == 64"#)))]
pub(crate) fn invert_ntt_at_layer_1<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    hax_lib::fstar!(r#"reveal_opaque (`%invert_ntt_re_range_1) (invert_ntt_re_range_1 #$:Vector)"#);
    #[cfg(hax)]
    hax_lib::fstar!(r#"reveal_opaque (`%invert_ntt_re_range_2) (invert_ntt_re_range_2 #$:Vector)"#);
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;

    for round in 0..16 {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|round: usize| {
            fstar!(
                r#"v zeta_i == v $_zeta_i_init - v $round * 4 /\
          (v round < 16 ==> (forall (i:nat). (i >= v round /\ i < 16) ==>
            Spec.Utils.is_i16b_array_opaque (4 * 3328)
              (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ])))) /\
          (forall (i:nat). i < v $round ==> Spec.Utils.is_i16b_array_opaque 3328
              (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ])))"#
            )
        });
        *zeta_i -= 1;
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i16b_array_opaque) 
                        (Spec.Utils.is_i16b_array_opaque (4*3328) 
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        re.coefficients[round] = Vector::inv_ntt_layer_1_step(
            re.coefficients[round],
            zeta(*zeta_i),
            zeta(*zeta_i - 1),
            zeta(*zeta_i - 2),
            zeta(*zeta_i - 3),
        );
        *zeta_i -= 3;
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i16b_array_opaque) 
                        (Spec.Utils.is_i16b_array_opaque 3328 
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        #[cfg(hax)]
        hax_lib::fstar!(
            "assert (Spec.Utils.is_i16b_array_opaque 3328
            (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ $round ])))"
        );
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v ${*zeta_i} == 64 /\
    invert_ntt_re_range_2 $re "#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"invert_ntt_re_range_2 ${re}_future /\
    v ${*zeta_i}_future == 32"#)))]
pub(crate) fn invert_ntt_at_layer_2<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    hax_lib::fstar!(r#"reveal_opaque (`%invert_ntt_re_range_2) (invert_ntt_re_range_2 #$:Vector)"#);
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;

    for round in 0..16 {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|round: usize| {
            fstar!(
                r#"v zeta_i == v $_zeta_i_init - v $round * 2 /\
          (v round < 16 ==> (forall (i:nat). (i >= v round /\ i < 16) ==>
            Spec.Utils.is_i16b_array_opaque 3328
              (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ])))) /\
          (forall (i:nat). i < v $round ==> Spec.Utils.is_i16b_array_opaque 3328
              (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ])))"#
            )
        });
        *zeta_i -= 1;
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i16b_array_opaque) 
                        (Spec.Utils.is_i16b_array_opaque 3328 
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        re.coefficients[round] =
            Vector::inv_ntt_layer_2_step(re.coefficients[round], zeta(*zeta_i), zeta(*zeta_i - 1));
        *zeta_i -= 1;
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i16b_array_opaque) 
                        (Spec.Utils.is_i16b_array_opaque 3328 
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        #[cfg(hax)]
        hax_lib::fstar!(
            "assert (Spec.Utils.is_i16b_array_opaque 3328
            (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ $round ])))"
        );
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v ${*zeta_i} == 32 /\
    invert_ntt_re_range_2 $re"#)))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"invert_ntt_re_range_2 ${re}_future /\
    v ${*zeta_i}_future == 16"#)))]
pub(crate) fn invert_ntt_at_layer_3<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    hax_lib::fstar!(r#"reveal_opaque (`%invert_ntt_re_range_2) (invert_ntt_re_range_2 #$:Vector)"#);
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;

    for round in 0..16 {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|round: usize| {
            fstar!(
                r#"v zeta_i == v $_zeta_i_init - v $round /\
          (v round < 16 ==> (forall (i:nat). (i >= v round /\ i < 16) ==>
            Spec.Utils.is_i16b_array_opaque 3328
              (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ])))) /\
          (forall (i:nat). i < v $round ==> Spec.Utils.is_i16b_array_opaque 3328
              (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ sz i ])))"#
            )
        });
        *zeta_i -= 1;
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"reveal_opaque (`%Spec.Utils.is_i16b_array_opaque) 
                        (Spec.Utils.is_i16b_array_opaque 3328 
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        re.coefficients[round] =
            Vector::inv_ntt_layer_3_step(re.coefficients[round], zeta(*zeta_i));
        #[cfg(hax)]
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i16b_array_opaque) 
            (Spec.Utils.is_i16b_array_opaque 3328 
            (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"
        );
        #[cfg(hax)]
        hax_lib::fstar!(
            "assert (Spec.Utils.is_i16b_array_opaque 3328
            (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ $round ])))"
        );
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 $zeta_r /\
    (forall i. i < 16 ==>
        Spec.Utils.is_intb (pow2 15 - 1)
        (v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $b) i) -
        v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $a) i))) /\
    (forall i. i < 16 ==>
        Spec.Utils.is_intb (pow2 15 - 1)
        (v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $a) i) +
        v (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array $b) i))) /\
    Spec.Utils.is_i16b_array 28296 (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
        (Libcrux_ml_kem.Vector.Traits.f_add $a $b))"#)))]
pub(crate) fn inv_ntt_layer_int_vec_step_reduce<Vector: Operations>(
    mut a: Vector,
    mut b: Vector,
    zeta_r: i16,
) -> (Vector, Vector) {
    let a_minus_b = Vector::sub(b, &a);
    a = Vector::barrett_reduce(Vector::add(a, &b));
    b = Vector::montgomery_multiply_by_constant(a_minus_b, zeta_r);
    (a, b)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v $layer >= 4 /\ v $layer <= 7"#)))]
pub(crate) fn invert_ntt_at_layer_4_plus<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    layer: usize,
) {
    let step = 1 << layer;

    for round in 0..(128 >> layer) {
        *zeta_i -= 1;

        let offset = round * step * 2;
        let offset_vec = offset / FIELD_ELEMENTS_IN_VECTOR;
        let step_vec = step / FIELD_ELEMENTS_IN_VECTOR;

        for j in offset_vec..offset_vec + step_vec {
            let (x, y) = inv_ntt_layer_int_vec_step_reduce(
                re.coefficients[j],
                re.coefficients[j + step_vec],
                zeta(*zeta_i),
            );
            re.coefficients[j] = x;
            re.coefficients[j + step_vec] = y;
        }
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"invert_ntt_re_range_1 $re"#)))]
pub(crate) fn invert_ntt_montgomery<const K: usize, Vector: Operations>(
    re: &mut PolynomialRingElement<Vector>,
) {
    // We only ever call this function after matrix/vector multiplication
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .all(|coefficient| coefficient.abs() < (K as i16) * FIELD_MODULUS));

    let mut zeta_i = super::constants::COEFFICIENTS_IN_RING_ELEMENT / 2;

    invert_ntt_at_layer_1(&mut zeta_i, re);
    invert_ntt_at_layer_2(&mut zeta_i, re);
    invert_ntt_at_layer_3(&mut zeta_i, re);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 4);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 5);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 6);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 7);

    hax_debug_assert!(
        to_i16_array(re)[0].abs() < 128 * (K as i16) * FIELD_MODULUS
            && to_i16_array(re)[1].abs() < 128 * (K as i16) * FIELD_MODULUS
    );
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .enumerate()
        .skip(2)
        .all(|(i, coefficient)| coefficient.abs() < (128 / (1 << i.ilog2())) * FIELD_MODULUS));

    re.poly_barrett_reduce()
}
