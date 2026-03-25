use super::arithmetic::*;
use super::vector_type::*;
use libcrux_secrets::*;

#[inline(always)]
#[hax_lib::fstar::before("[@@ \"opaque_to_smt\"]")]
#[hax_lib::requires(fstar!(r#"v i < 16 /\ v j < 16 /\ v i <> v j /\ 
                            Spec.Utils.is_i16b 1664 $zeta  /\
                            Spec.Utils.is_i16b_array (11207 + 6 * 3328) vec.f_elements /\
                            Spec.Utils.is_i16b (11207 + 5*3328) vec.f_elements.[i] /\
                            Spec.Utils.is_i16b (11207 + 5*3328) vec.f_elements.[j]"#))]
#[hax_lib::ensures(|result| fstar!(r#"(forall k. (k <> v i /\ k <> v j) ==>
                                         Seq.index ${vec}_future.f_elements k == Seq.index ${vec}.f_elements k) /\
                                    (forall b. (Spec.Utils.is_i16b b ${vec}.f_elements.[i] /\
                                               Spec.Utils.is_i16b b ${vec}.f_elements.[j]) ==>
                                              (Spec.Utils.is_i16b (b+3328) ${vec}_future.f_elements.[i] /\
                                               Spec.Utils.is_i16b (b+3328) ${vec}_future.f_elements.[j])) /\
                                    Spec.Utils.ntt_spec ${vec}.f_elements (v $zeta) (v $i) (v $j) ${vec}_future.f_elements"#))]
pub(crate) fn ntt_step(vec: &mut PortableVector, zeta: i16, i: usize, j: usize) {
    let t = montgomery_multiply_fe_by_fer(vec.elements[j], zeta.classify());
    hax_lib::fstar!(
        "assert (v t % 3329 == ((v (Seq.index vec.f_elements (v j)) * v zeta * 169) % 3329))"
    );
    let a_minus_t = vec.elements[i] - t;
    hax_lib::fstar!(
        r#"
    calc (==) {
        v $a_minus_t % 3329;
        (==) {}
        (v (Seq.index vec.f_elements (v i)) - v ${t}) % 3329;
        (==) {Math.Lemmas.lemma_mod_sub_distr (v (Seq.index vec.f_elements (v $i))) (v $t) 3329}
        (v (Seq.index vec.f_elements (v $i)) - (v $t % 3329)) % 3329;
        (==) {}
        (v (Seq.index vec.f_elements (v i)) - ((v (Seq.index vec.f_elements (v $j)) * v $zeta * 169) % 3329)) % 3329;
        (==) {Math.Lemmas.lemma_mod_sub_distr (v (Seq.index vec.f_elements (v $i))) (v (Seq.index vec.f_elements (v $j)) * v zeta * 169) 3329}
        (v (Seq.index vec.f_elements (v $i)) - (v (Seq.index vec.f_elements (v $j)) * v $zeta * 169)) % 3329;
        }"#
    );
    let a_plus_t = vec.elements[i] + t;
    hax_lib::fstar!(
        r#"
    calc (==) {
        v a_plus_t % 3329;
        (==) {}
        (v (Seq.index vec.f_elements (v $i)) + v $t) % 3329;
        (==) {Math.Lemmas.lemma_mod_add_distr (v (Seq.index vec.f_elements (v $i))) (v $t) 3329}
        (v (Seq.index vec.f_elements (v $i)) + (v $t % 3329)) % 3329;
        (==) {}
        (v (Seq.index vec.f_elements (v $i)) + ((v (Seq.index vec.f_elements (v $j)) * v $zeta * 169) % 3329)) % 3329;
        (==) {Math.Lemmas.lemma_mod_add_distr (v (Seq.index vec.f_elements (v $i))) (v (Seq.index vec.f_elements (v $j)) * v zeta * 169) 3329}
        (v (Seq.index vec.f_elements (v $i)) + (v (Seq.index vec.f_elements (v $j)) * v $zeta * 169)) % 3329;
    }"#
    );
    vec.elements[j] = a_minus_t;
    vec.elements[i] = a_plus_t;
    hax_lib::fstar!(
        "assert (Seq.index vec.f_elements (v i) == a_plus_t);
                     assert (Seq.index vec.f_elements (v j) == a_minus_t)"
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (11207+5*3328) ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array (11207+6*3328) ${result}.f_elements"#))]
pub(crate) fn ntt_layer_1_step(
    mut vec: PortableVector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> PortableVector {
    ntt_step(&mut vec, zeta0, 0, 2);
    ntt_step(&mut vec, zeta0, 1, 3);
    ntt_step(&mut vec, zeta1, 4, 6);
    ntt_step(&mut vec, zeta1, 5, 7);
    ntt_step(&mut vec, zeta2, 8, 10);
    ntt_step(&mut vec, zeta2, 9, 11);
    ntt_step(&mut vec, zeta3, 12, 14);
    ntt_step(&mut vec, zeta3, 13, 15);
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b_array (11207+4*3328) ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array (11207+5*3328) ${result}.f_elements"#))]
pub(crate) fn ntt_layer_2_step(mut vec: PortableVector, zeta0: i16, zeta1: i16) -> PortableVector {
    ntt_step(&mut vec, zeta0, 0, 4);
    ntt_step(&mut vec, zeta0, 1, 5);
    ntt_step(&mut vec, zeta0, 2, 6);
    ntt_step(&mut vec, zeta0, 3, 7);
    ntt_step(&mut vec, zeta1, 8, 12);
    ntt_step(&mut vec, zeta1, 9, 13);
    ntt_step(&mut vec, zeta1, 10, 14);
    ntt_step(&mut vec, zeta1, 11, 15);
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
                            Spec.Utils.is_i16b_array (11207+3*3328) ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array (11207+4*3328) ${result}.f_elements"#))]
pub(crate) fn ntt_layer_3_step(mut vec: PortableVector, zeta: i16) -> PortableVector {
    ntt_step(&mut vec, zeta, 0, 8);
    ntt_step(&mut vec, zeta, 1, 9);
    ntt_step(&mut vec, zeta, 2, 10);
    ntt_step(&mut vec, zeta, 3, 11);
    ntt_step(&mut vec, zeta, 4, 12);
    ntt_step(&mut vec, zeta, 5, 13);
    ntt_step(&mut vec, zeta, 6, 14);
    ntt_step(&mut vec, zeta, 7, 15);
    vec
}

#[inline(always)]
#[hax_lib::fstar::before("[@@ \"opaque_to_smt\"]")]
#[hax_lib::requires(fstar!(r#"v i < 16 /\ v j < 16 /\  v i <> v j /\ 
                        Spec.Utils.is_i16b 1664 $zeta /\
                        Spec.Utils.is_i16b_array (4*3328) ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array (4*3328) ${vec}_future.f_elements /\
                                    (forall k. (k <> v i /\ k <> v j) ==>
                                         Seq.index ${vec}_future.f_elements k == Seq.index ${vec}.f_elements k) /\
                                    Spec.Utils.is_i16b 3328 (Seq.index ${vec}_future.f_elements (v i)) /\
                                    Spec.Utils.is_i16b 3328 (Seq.index ${vec}_future.f_elements (v j)) /\
                                    Spec.Utils.inv_ntt_spec ${vec}.f_elements (v $zeta) (v $i) (v $j) ${vec}_future.f_elements"#))]
pub(crate) fn inv_ntt_step(vec: &mut PortableVector, zeta: i16, i: usize, j: usize) {
    let a_minus_b = vec.elements[j] - vec.elements[i];
    let a_plus_b = vec.elements[j] + vec.elements[i];
    hax_lib::fstar!(
        r#"assert (v a_minus_b = v (Seq.index vec.f_elements (v j)) - v (Seq.index vec.f_elements (v i)));
                     assert (v a_plus_b = v (Seq.index vec.f_elements (v j)) + v (Seq.index vec.f_elements (v i)))"#
    );
    let o0 = barrett_reduce_element(a_plus_b);
    let o1 = montgomery_multiply_fe_by_fer(a_minus_b, zeta.classify());
    hax_lib::fstar!(
        r#"
    calc (==) {
        v o0 % 3329;
        (==) { }
        v a_plus_b % 3329;
        (==) { }
        (v (Seq.index vec.f_elements (v j)) + v (Seq.index vec.f_elements (v i))) % 3329;
    };
    calc (==) {
        v o1 % 3329;
        (==) { }
        (v a_minus_b * v zeta * 169) % 3329;
        (==) { }
        ((v (Seq.index vec.f_elements (v j)) - v (Seq.index vec.f_elements (v i))) * v zeta * 169) % 3329;
    }"#
    );
    vec.elements[i] = o0;
    vec.elements[j] = o1;
    hax_lib::fstar!(
        r#"assert (Seq.index vec.f_elements (v i) == o0);
                     assert (Seq.index vec.f_elements (v j) == o1)"#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (4*3328) ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 ${result}.f_elements"#))]
pub(crate) fn inv_ntt_layer_1_step(
    mut vec: PortableVector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> PortableVector {
    inv_ntt_step(&mut vec, zeta0, 0, 2);
    inv_ntt_step(&mut vec, zeta0, 1, 3);
    inv_ntt_step(&mut vec, zeta1, 4, 6);
    inv_ntt_step(&mut vec, zeta1, 5, 7);
    inv_ntt_step(&mut vec, zeta2, 8, 10);
    inv_ntt_step(&mut vec, zeta2, 9, 11);
    inv_ntt_step(&mut vec, zeta3, 12, 14);
    inv_ntt_step(&mut vec, zeta3, 13, 15);
    hax_lib::fstar!(
        r#"assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 13));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 15));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 12));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 14));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 9));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 11));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 8));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 10));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 5));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 7));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 4));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 6));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 1));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 3));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 0));
        assert (Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements 2));
        assert (forall (i:nat). i < 16 ==> Spec.Utils.is_i16b 3328 (Seq.index ${vec}.f_elements i))"#
    );
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b_array 3328 ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 ${result}.f_elements"#))]
pub(crate) fn inv_ntt_layer_2_step(
    mut vec: PortableVector,
    zeta0: i16,
    zeta1: i16,
) -> PortableVector {
    inv_ntt_step(&mut vec, zeta0, 0, 4);
    inv_ntt_step(&mut vec, zeta0, 1, 5);
    inv_ntt_step(&mut vec, zeta0, 2, 6);
    inv_ntt_step(&mut vec, zeta0, 3, 7);
    inv_ntt_step(&mut vec, zeta1, 8, 12);
    inv_ntt_step(&mut vec, zeta1, 9, 13);
    inv_ntt_step(&mut vec, zeta1, 10, 14);
    inv_ntt_step(&mut vec, zeta1, 11, 15);
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
                            Spec.Utils.is_i16b_array 3328 ${vec}.f_elements"#))]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 ${result}.f_elements"#))]
pub(crate) fn inv_ntt_layer_3_step(mut vec: PortableVector, zeta: i16) -> PortableVector {
    inv_ntt_step(&mut vec, zeta, 0, 8);
    inv_ntt_step(&mut vec, zeta, 1, 9);
    inv_ntt_step(&mut vec, zeta, 2, 10);
    inv_ntt_step(&mut vec, zeta, 3, 11);
    inv_ntt_step(&mut vec, zeta, 4, 12);
    inv_ntt_step(&mut vec, zeta, 5, 13);
    inv_ntt_step(&mut vec, zeta, 6, 14);
    inv_ntt_step(&mut vec, zeta, 7, 15);
    vec
}

/// Compute the product of two Kyber binomials with respect to the
/// modulus `X² - zeta`.
///
/// This function almost implements <strong>Algorithm 11</strong> of the
/// NIST FIPS 203 standard, which is reproduced below:
///
/// ```plaintext
/// Input:  a₀, a₁, b₀, b₁ ∈ ℤq.
/// Input: γ ∈ ℤq.
/// Output: c₀, c₁ ∈ ℤq.
///
/// c₀ ← a₀·b₀ + a₁·b₁·γ
/// c₁ ← a₀·b₁ + a₁·b₀
/// return c₀, c₁
/// ```
/// We say "almost" because the coefficients output by this function are in
/// the Montgomery domain (unlike in the specification).
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[inline(always)]
#[hax_lib::fstar::options(
    "--z3rlimit 250 --split_queries always --query_stats --ext context_prune"
)]
#[hax_lib::fstar::before(
    interface,
    r#"
    let ntt_multiply_binomials_post
      (a b: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta: i16)
      (i: usize{v i < 8})
      (out_future: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
            let ai = Seq.index a.f_elements (2 * v i) in
            let aj = Seq.index a.f_elements (2 * v i + 1) in
            let bi = Seq.index b.f_elements (2 * v i) in
            let bj = Seq.index b.f_elements (2 * v i + 1) in
            let oi = Seq.index out_future.f_elements (2 * v i) in
            let oj = Seq.index out_future.f_elements (2 * v i + 1) in
            ((v oi % 3329) == (((v ai * v bi + (v aj * v bj * v zeta * 169)) * 169) % 3329)) /\
            ((v oj % 3329) == (((v ai * v bj + v aj * v bi) * 169) % 3329))
"#
)]
#[hax_lib::fstar::before("[@@ \"opaque_to_smt\"]")]
#[hax_lib::requires(fstar!(r#"v i < 8 /\ Spec.Utils.is_i16b 1664 $zeta /\
        Spec.Utils.is_i16b_array 3328 ${a}.f_elements /\
        Spec.Utils.is_i16b_array 3328 ${b}.f_elements /\
        Spec.Utils.is_i16b_array 3328 ${out}.f_elements "#))]
#[hax_lib::ensures(|()| fstar!(r#"
        Spec.Utils.is_i16b_array 3328 ${out}_future.f_elements /\
        Spec.Utils.modifies2_16 ${out}.f_elements ${out}_future.f_elements (sz 2 *! $i) ((sz 2 *! $i) +! sz 1) /\
        ntt_multiply_binomials_post ${a} ${b} ${zeta} ${i} ${out}_future"#))]
pub(crate) fn ntt_multiply_binomials(
    a: &PortableVector,
    b: &PortableVector,
    zeta: FieldElementTimesMontgomeryR,
    i: usize,
    out: &mut PortableVector,
) {
    let ai = a.elements[2 * i];
    let bi = b.elements[2 * i];
    let aj = a.elements[2 * i + 1];
    let bj = b.elements[2 * i + 1];

    hax_lib::fstar!(
        "assert(Spec.Utils.is_i16b 3328 $ai);
                     assert(Spec.Utils.is_i16b 3328 $bi);
                     assert(Spec.Utils.is_i16b 3328 $aj);
                     assert(Spec.Utils.is_i16b 3328 $bj);
                     assert_norm (3328 * 3328 < pow2 31)"
    );

    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i16b 3328 3328 $ai $bi"#);

    let ai_bi = (ai.as_i32()) * (bi.as_i32());

    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i16b 3328 3328 $aj $bj"#);

    let aj_bj_ = (aj.as_i32()) * (bj.as_i32());

    hax_lib::fstar!(r#"assert_norm (3328 * 3328 <= 3328 * pow2 15)"#);

    let aj_bj = montgomery_reduce_element(aj_bj_);

    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i16b 3328 1664 $aj_bj $zeta"#);

    let aj_bj_zeta = (aj_bj.as_i32()) * (zeta.as_i32());
    let ai_bi_aj_bj = ai_bi + aj_bj_zeta;

    hax_lib::fstar!(r#"assert(Spec.Utils.is_i32b (3328*3328 + 3328*1664) $ai_bi_aj_bj)"#);
    hax_lib::fstar!(r#"assert_norm (3328 * 3328 + 3328 * 1664 <= 3328 * pow2 15)"#);

    let o0 = montgomery_reduce_element(ai_bi_aj_bj);

    hax_lib::fstar!(
        r#"calc  ( == ) {
        v $o0 % 3329;
        ( == ) { () }
        (v $ai_bi_aj_bj * 169) % 3329;
        ( == ) { assert(v $ai_bi_aj_bj == v $ai_bi + v $aj_bj_zeta) }
        ((v $ai_bi + v $aj_bj_zeta) * 169) % 3329;
        ( == ) { assert (v $ai_bi == v $ai * v $bi) }
        (((v $ai * v $bi) + v $aj_bj_zeta) * 169) % 3329;
        ( == ) { assert (v $aj_bj_zeta == v $aj_bj * v $zeta) }
        (((v $ai * v $bi) + (v $aj_bj * v $zeta)) * 169) % 3329;
        ( == ) { Math.Lemmas.lemma_mod_mul_distr_l ((v ai * v bi) + (v aj_bj * v zeta)) 169 3329 }
        ((((v $ai * v $bi) + (v $aj_bj * v $zeta)) % 3329) * 169) % 3329;
        ( == ) { Math.Lemmas.lemma_mod_add_distr (v ai * v bi) (v aj_bj * v zeta) 3329 }
        (((v $ai * v $bi) + ((v $aj_bj * v $zeta) % 3329)) % 3329 * 169) % 3329;
        ( == ) { Math.Lemmas.lemma_mod_mul_distr_l (v aj_bj) (v zeta) 3329 }
        (((v $ai * v $bi) + ((v $aj_bj % 3329 * v $zeta) % 3329)) % 3329 * 169) % 3329;
        ( == ) { assert(v aj_bj % 3329 == (v $aj_bj_ * 169) % 3329) }
        (((v $ai * v $bi) + (((v $aj_bj_ * 169) % 3329 * v $zeta) % 3329)) % 3329 * 169) % 3329;
        ( == ) { assert(v $aj_bj_ == v $aj * v $bj) }
        (((v $ai * v $bi) + (((v $aj * v $bj * 169) % 3329 * v $zeta) % 3329)) % 3329 * 169) % 3329;
        ( == ) { Math.Lemmas.lemma_mod_mul_distr_l (v $aj * v $bj * 169) (v $zeta) 3329 }
        (((v $ai * v $bi) + (((v $aj * v $bj * 169 * v $zeta) % 3329))) % 3329 * 169) % 3329;
        ( == ) { Math.Lemmas.lemma_mod_add_distr (v $ai * v $bi) (v $aj * v $bj * 169 * v $zeta) 3329 }
        (((v $ai * v $bi) + ((v $aj * v $bj * 169 * v $zeta))) % 3329 * 169) % 3329;
        ( == ) { Math.Lemmas.lemma_mod_mul_distr_l ((v ai * v bi) + ((v aj * v bj * 169 * v zeta))) 169 3329 }
        (((v $ai * v $bi) + ((v $aj * v $bj * 169 * v $zeta))) * 169) % 3329;
        }"#
    );
    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i16b 3328 3328 $ai $bj"#);

    let ai_bj = (ai.as_i32()) * (bj.as_i32());

    hax_lib::fstar!(r#"Spec.Utils.lemma_mul_i16b 3328 3328 $aj $bi"#);

    let aj_bi = (aj.as_i32()) * (bi.as_i32());
    let ai_bj_aj_bi = ai_bj + aj_bi;

    hax_lib::fstar!(r#"assert(Spec.Utils.is_i32b (3328*3328 + 3328*3328) ai_bj_aj_bi) "#);
    hax_lib::fstar!(r#"assert_norm (3328 * 3328 + 3328 * 3328 <= 3328 * pow2 15)"#);

    let o1 = montgomery_reduce_element(ai_bj_aj_bi);

    hax_lib::fstar!(
        "calc  ( == ) {
        v $o1 % 3329;
        ( == ) { () }
        (v $ai_bj_aj_bi * 169) % 3329;
        ( == ) { assert(v $ai_bj_aj_bi == v $ai_bj + v $aj_bi) }
        ((v $ai_bj + v $aj_bi) * 169) % 3329;
        ( == ) { assert (v ai_bj == v ai * v bj) }
        ((v ai * v bj + v aj_bi) * 169) % 3329;
        ( == ) { assert (v aj_bi == v aj * v bi) }
        ((v ai * v bj + v aj * v bi) * 169) % 3329;
    }"
    );

    #[cfg(hax)]
    let _out0 = out.elements;

    out.elements[2 * i] = o0;
    out.elements[2 * i + 1] = o1;

    hax_lib::fstar!(
        r#"assert (Seq.index out.f_elements (2 * v i) == o0);
                     assert (Seq.index out.f_elements (2 * v i + 1) == o1);
                     assert (Spec.Utils.is_i16b_array 3328 out.f_elements);
                     assert (forall k. (k <> 2 * v i /\ k <> 2 * v i + 1) ==>
                                        Seq.index out.f_elements k ==
                                        Seq.index ${_out0} k)"#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 $zeta0 /\
        Spec.Utils.is_i16b 1664 $zeta1 /\
        Spec.Utils.is_i16b 1664 $zeta2 /\
        Spec.Utils.is_i16b 1664 $zeta3 /\
        Spec.Utils.is_i16b_array 3328 ${lhs}.f_elements /\
        Spec.Utils.is_i16b_array 3328 ${rhs}.f_elements "#))]
#[hax_lib::ensures(|result| fstar!(r#"
        Spec.Utils.is_i16b_array 3328 ${result}.f_elements /\
        (let nzeta0:i16 = Core_models.Ops.Arith.f_neg zeta0 in
         let nzeta1:i16 = Core_models.Ops.Arith.f_neg zeta1 in
         let nzeta2:i16 = Core_models.Ops.Arith.f_neg zeta2 in
         let nzeta3:i16 = Core_models.Ops.Arith.f_neg zeta3 in
         let zetas =
            Seq.seq_of_list [
                zeta0;
                nzeta0;
                zeta1;
                nzeta1;
                zeta2;
                nzeta2;
                zeta3;
                nzeta3
            ]
         in
         Spec.Utils.forall8 (fun i -> ntt_multiply_binomials_post ${lhs} ${rhs} (Seq.index zetas i) (sz i) $result))
"#))]
pub(crate) fn ntt_multiply(
    lhs: &PortableVector,
    rhs: &PortableVector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> PortableVector {
    let nzeta0 = -zeta0;
    let nzeta1 = -zeta1;
    let nzeta2 = -zeta2;
    let nzeta3 = -zeta3;
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b 1664 nzeta0)"#);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b 1664 nzeta1)"#);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b 1664 nzeta2)"#);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b 1664 nzeta3)"#);
    let mut out = zero();
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    ntt_multiply_binomials(lhs, rhs, zeta0.classify(), 0, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    ntt_multiply_binomials(lhs, rhs, nzeta0.classify(), 1, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    ntt_multiply_binomials(lhs, rhs, zeta1.classify(), 2, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta0 (mk_usize 1) out)"#);
    ntt_multiply_binomials(lhs, rhs, nzeta1.classify(), 3, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta0 (mk_usize 1) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta1 (mk_usize 2) out)"#);
    ntt_multiply_binomials(lhs, rhs, zeta2.classify(), 4, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta0 (mk_usize 1) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta1 (mk_usize 2) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta1 (mk_usize 3) out)"#);
    ntt_multiply_binomials(lhs, rhs, nzeta2.classify(), 5, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta0 (mk_usize 1) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta1 (mk_usize 2) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta1 (mk_usize 3) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta2 (mk_usize 4) out)"#);
    ntt_multiply_binomials(lhs, rhs, zeta3.classify(), 6, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta0 (mk_usize 1) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta1 (mk_usize 2) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta1 (mk_usize 3) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta2 (mk_usize 4) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta2 (mk_usize 5) out)"#);
    ntt_multiply_binomials(lhs, rhs, nzeta3.classify(), 7, &mut out);
    hax_lib::fstar!(r#"assert (Spec.Utils.is_i16b_array 3328 out.f_elements)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta0 (mk_usize 0) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta0 (mk_usize 1) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta1 (mk_usize 2) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta1 (mk_usize 3) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta2 (mk_usize 4) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta2 (mk_usize 5) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs zeta3 (mk_usize 6) out)"#);
    hax_lib::fstar!(r#"assert (ntt_multiply_binomials_post lhs rhs nzeta3 (mk_usize 7) out)"#);
    out
}
