use crate::vector::{Operations, MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS};

pub(crate) const ZETAS_TIMES_MONTGOMERY_R: [i16; 128] = {
    hax_lib::fstar!(r#"assert_norm (pow2 16 == 65536)"#);
    [
        -1044, -758, -359, -1517, 1493, 1422, 287, 202, -171, 622, 1577, 182, 962, -1202, -1474,
        1468, 573, -1325, 264, 383, -829, 1458, -1602, -130, -681, 1017, 732, 608, -1542, 411,
        -205, -1571, 1223, 652, -552, 1015, -1293, 1491, -282, -1544, 516, -8, -320, -666, -1618,
        -1162, 126, 1469, -853, -90, -271, 830, 107, -1421, -247, -951, -398, 961, -1508, -725,
        448, -1065, 677, -1275, -1103, 430, 555, 843, -1251, 871, 1550, 105, 422, 587, 177, -235,
        -291, -460, 1574, 1653, -246, 778, 1159, -147, -777, 1483, -602, 1119, -1590, 644, -872,
        349, 418, 329, -156, -75, 817, 1097, 603, 610, 1322, -1285, -1465, 384, -1215, -136, 1218,
        -1335, -874, 220, -1187, -1659, -1185, -1530, -1278, 794, -1510, -854, -870, 478, -108,
        -308, 996, 991, 958, -1460, 1522, 1628,
    ]
};

// A function to retrieve zetas so that we can add a post-condition
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(i < 128)]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b 1664 result"#))]
pub fn zeta(i: usize) -> i16 {
    ZETAS_TIMES_MONTGOMERY_R[i]
}

pub(crate) const VECTORS_IN_RING_ELEMENT: usize = 16;

#[cfg_attr(
    hax,
    hax_lib::fstar::after(
        interface,
        r#"let to_spec_poly_t (#v_Vector: Type0)
    {| i2: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
    (p: t_PolynomialRingElement v_Vector) : Spec.MLKEM.polynomial =
    createi (sz 256) (fun i -> Spec.MLKEM.Math.to_spec_fe 
                                (Seq.index (i2._super_i2.f_repr 
                                    (Seq.index p.f_coefficients (v i / 16))) (v i % 16)))
let to_spec_vector_t (#r:Spec.MLKEM.rank) (#v_Vector: Type0)
    {| i2: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
    (m:t_Array (t_PolynomialRingElement v_Vector) r) : Spec.MLKEM.vector r =
    createi r (fun i -> to_spec_poly_t #v_Vector (m.[i]))
let to_spec_matrix_t (#r:Spec.MLKEM.rank) (#v_Vector: Type0)
    {| i2: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
    (m:t_Array (t_Array (t_PolynomialRingElement v_Vector) r) r) : Spec.MLKEM.matrix r =
    createi r (fun i -> to_spec_vector_t #r #v_Vector (m.[i]))

let is_bounded_vector (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (bound: nat) (x: v_Vector) = Spec.Utils.is_i16b_array bound (i0.f_to_i16_array x)

let is_bounded_poly (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (bound: nat)
      (re: t_PolynomialRingElement v_Vector) =
    forall (i:nat). i < 16 ==> is_bounded_vector bound (Seq.index re.f_coefficients i)"#
    )
)]
// XXX: We don't want to copy this. But for eurydice we have to have this.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct PolynomialRingElement<Vector: Operations> {
    pub(crate) coefficients: [Vector; VECTORS_IN_RING_ELEMENT],
}

#[allow(non_snake_case)]
fn ZERO<Vector: Operations>() -> PolynomialRingElement<Vector> {
    PolynomialRingElement {
        // https://github.com/hacspec/hax/issues/27
        // FIXME:  The THIR body of item DefId(0:415 ~ libcrux_ml_kem[9000]::polynomial::{impl#0}::ZERO::{constant#0}) was stolen.
        coefficients: [Vector::ZERO(); 16],
    }
}

#[inline(always)]
#[hax_lib::requires(VECTORS_IN_RING_ELEMENT * 16 <= a.len())]
fn from_i16_array<Vector: Operations>(a: &[i16]) -> PolynomialRingElement<Vector> {
    let mut result = ZERO();
    for i in 0..VECTORS_IN_RING_ELEMENT {
        result.coefficients[i] = Vector::from_i16_array(&a[i * 16..(i + 1) * 16]);
    }
    result
}

#[allow(dead_code)]
#[inline(always)]
#[hax_lib::requires(out.len() >= VECTORS_IN_RING_ELEMENT * 16)]
fn to_i16_array<Vector: Operations>(re: PolynomialRingElement<Vector>, out: &mut [i16]) {
    #[cfg(hax)]
    let _out_len = out.len();

    for i in 0..re.coefficients.len() {
        hax_lib::loop_invariant!(|_i: usize| out.len() == _out_len);
        out[i * 16..(i + 1) * 16].copy_from_slice(&Vector::to_i16_array(re.coefficients[i]));
    }
}

#[inline(always)]
#[hax_lib::requires(VECTORS_IN_RING_ELEMENT * 16 *2 <= bytes.len())]
fn from_bytes<Vector: Operations>(bytes: &[u8]) -> PolynomialRingElement<Vector> {
    let mut result = ZERO();
    for i in 0..VECTORS_IN_RING_ELEMENT {
        result.coefficients[i] = Vector::from_bytes(&bytes[i * 32..(i + 1) * 32]);
    }
    result
}

#[inline(always)]
#[hax_lib::requires(VECTORS_IN_RING_ELEMENT * 32 <= out.len())]
fn to_bytes<Vector: Operations>(re: PolynomialRingElement<Vector>, out: &mut [u8]) {
    #[cfg(hax)]
    let _out_len = out.len();

    for i in 0..re.coefficients.len() {
        hax_lib::loop_invariant!(|_i: usize| out.len() == _out_len);
        Vector::to_bytes(re.coefficients[i], &mut out[i * 32..(i + 1) * 32]);
    }
}

/// Get the bytes of the vector of ring elements in `re` and write them to `out`.
#[inline(always)]
#[allow(dead_code)]
#[hax_lib::fstar::options("--z3rlimit 500 --split_queries always")]
#[hax_lib::requires(re.len() <= 4 && 512 * re.len() <= out.len())]
pub(crate) fn vec_to_bytes<Vector: Operations>(
    re: &[PolynomialRingElement<Vector>],
    out: &mut [u8],
) {
    #[cfg(hax)]
    let _out_len = out.len();
    let re_bytes = PolynomialRingElement::<Vector>::num_bytes();
    for i in 0..re.len() {
        hax_lib::loop_invariant!(|_i: usize| out.len() == _out_len);
        PolynomialRingElement::<Vector>::to_bytes(re[i], &mut out[i * re_bytes..]);
    }
}

/// Build a vector of ring elements from `bytes`.
#[inline(always)]
#[allow(dead_code)]
#[hax_lib::fstar::options("--z3rlimit 500 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Seq.length out <= 4 /\ 
    Seq.length bytes >= 512 * Seq.length out"#))]
#[hax_lib::ensures(|_| future(out).len() == out.len())]
pub(crate) fn vec_from_bytes<Vector: Operations>(
    bytes: &[u8],
    out: &mut [PolynomialRingElement<Vector>],
) {
    #[cfg(hax)]
    let _out_len = out.len();

    let re_bytes = PolynomialRingElement::<Vector>::num_bytes();
    for i in 0..out.len() {
        hax_lib::loop_invariant!(|_i: usize| out.len() == _out_len);
        out[i] = PolynomialRingElement::<Vector>::from_bytes(&bytes[i * re_bytes..]);
    }
}

/// The length of a vector of ring elements in bytes
#[hax_lib::requires(K <= 4)]
#[allow(dead_code)]
pub(crate) const fn vec_len_bytes<const K: usize, Vector: Operations>() -> usize {
    K * PolynomialRingElement::<Vector>::num_bytes()
}

/// Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
/// sum of their constituent coefficients.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 500 --split_queries always")]
#[hax_lib::requires(fstar!(r#"
        forall (i:nat). i < v ${VECTORS_IN_RING_ELEMENT} ==>
         (let lhs_i = i0.f_to_i16_array (${myself}.f_coefficients.[ sz i ]) in
          let rhs_i = i0.f_to_i16_array (${rhs}.f_coefficients.[ sz i ]) in
          Libcrux_ml_kem.Vector.Traits.Spec.add_pre lhs_i rhs_i)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
        forall (i:nat). i < v ${VECTORS_IN_RING_ELEMENT} ==>
         (let lhs_i = i0.f_to_i16_array (${myself}.f_coefficients.[ sz i ]) in
          let rhs_i = i0.f_to_i16_array (${rhs}.f_coefficients.[ sz i ]) in
          let result_i = i0.f_to_i16_array (${myself}_future.f_coefficients.[ sz i ]) in
          Libcrux_ml_kem.Vector.Traits.Spec.add_post lhs_i rhs_i result_i)"#))]
fn add_to_ring_element<Vector: Operations, const K: usize>(
    myself: &mut PolynomialRingElement<Vector>,
    rhs: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;
    hax_lib::fstar!(
        r#"assert(forall (v: v_Vector).
        i0.f_to_i16_array v == i0._super_i2.f_repr v)"#
    );

    for i in 0..myself.coefficients.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
                v $i <= v ${VECTORS_IN_RING_ELEMENT} /\
                (forall (j:nat). (j >= v $i /\ j < v ${VECTORS_IN_RING_ELEMENT}) ==>
                   (let _myself_j = i0.f_to_i16_array (${_myself}.[ sz j ]) in
                    let myself_j = i0.f_to_i16_array (${myself}.f_coefficients.[ sz j ]) in
                    let rhs_j = i0.f_to_i16_array (${rhs}.f_coefficients.[ sz j ]) in
                    myself_j ==  _myself_j /\
                    Libcrux_ml_kem.Vector.Traits.Spec.add_pre myself_j rhs_j)) /\
                (forall (j:nat). j < v $i ==>
                   (let _myself_j = i0.f_to_i16_array (${_myself}.[ sz j ]) in
                    let myself_j = i0.f_to_i16_array (${myself}.f_coefficients.[ sz j ]) in
                    let rhs_j = i0.f_to_i16_array (${rhs}.f_coefficients.[ sz j ]) in
                    Libcrux_ml_kem.Vector.Traits.Spec.add_post _myself_j rhs_j myself_j))"#
            )
        });

        myself.coefficients[i] = Vector::add(myself.coefficients[i], &rhs.coefficients[i]);
    }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"is_bounded_poly 28296 ${myself}"#))]
#[hax_lib::ensures(|_| fstar!(r#"is_bounded_poly 3328 ${myself}_future"#))]
fn poly_barrett_reduce<Vector: Operations>(myself: &mut PolynomialRingElement<Vector>) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
        (forall j. j < v i ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz j ]) /\
        (forall j. (j >= v i /\ j < 16) ==> ${myself}.f_coefficients.[ sz j ] == ${_myself}.[ sz j ])
        "#
        ));

        myself.coefficients[i] = Vector::barrett_reduce(myself.coefficients[i]);
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"is_bounded_poly (pow2 12 - 1) ${myself}"#))]
#[hax_lib::ensures(|result| fstar!(r#"is_bounded_poly 3328 ${result}"#))]
fn subtract_reduce<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    mut b: PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    let _b = b.coefficients;

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
        (forall j. j < v i ==> is_bounded_vector 3328 ${b}.f_coefficients.[ sz j ]) /\
        (forall j. (j >= v i /\ j < 16) ==> ${b}.f_coefficients.[ sz j ] == ${_b}.[ sz j ])
        "#
        ));
        hax_lib::fstar!(
            r#"
          assert (v $i < 16);
          assert_norm (1441 < pow2 15);
          assert_norm (1664 < pow2 15);
          assert_norm (mk_i16 1441 <. mk_i16 1664);
          assert(Spec.Utils.is_i16b 1664 (mk_i16 1441))
        "#
        );

        let coefficient_normal_form =
            Vector::montgomery_multiply_by_constant(b.coefficients[i], 1441);

        hax_lib::fstar!(
            r#"
            assert (is_bounded_vector 3328 ${coefficient_normal_form});
            assert (is_bounded_vector (pow2 12 - 1) (${myself}.f_coefficients.[ i ]));
            assert_norm (pow2 12 - 1 == 4095);
            Spec.Utils.lemma_sub_intb_forall 4095 3328;
            assert (forall j. Spec.Utils.is_intb 7423
                (v (Seq.index (i0.f_to_i16_array ${myself}.f_coefficients.[ i ]) j) -
                 v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) j)));
            assert_norm (7423 <= pow2 15 - 1);
            Spec.Utils.lemma_intb_le 7423 (pow2 15 - 1);
            Spec.Utils.lemma_intb_le 7423 28296;
            assert (forall j. Spec.Utils.is_intb (pow2 15 - 1) 
                (v (Seq.index (i0.f_to_i16_array ${myself}.f_coefficients.[ i ]) j) -
                 v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) j)));
            assert (forall j. Spec.Utils.is_intb 28296 
                (v (Seq.index (i0.f_to_i16_array ${myself}.f_coefficients.[ i ]) j) -
                 v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) j)))
        "#
        );

        let diff = Vector::sub(myself.coefficients[i], &coefficient_normal_form);
        hax_lib::fstar!("assert (is_bounded_vector 28296 diff)");
        let red = Vector::barrett_reduce(diff);
        hax_lib::fstar!("assert (is_bounded_vector 3328 red)");
        b.coefficients[i] = red;

        hax_lib::fstar!(
            r#"
            assert (forall j. (j > v $i /\ j < 16) ==> ${b}.f_coefficients.[ sz j ] == ${_b}.[ sz j]);
            assert (forall j. j < v $i ==> is_bounded_vector 3328 ${b}.f_coefficients.[ sz j ]);
            assert (${b}.f_coefficients.[ $i ] == ${red});
            assert (forall j. j <= v $i ==> is_bounded_vector 3328 ${b}.f_coefficients.[ sz j ])
        "#
        );
    }
    b
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    is_bounded_poly 3328 ${myself} /\ 
    is_bounded_poly 3328 ${message}"#))]
#[hax_lib::ensures(|output| fstar!("is_bounded_poly 3328 ${output}"))]
fn add_message_error_reduce<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    message: &PolynomialRingElement<Vector>,
    mut result: PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    let _result = result.coefficients;

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            (forall j. j < v i ==> is_bounded_vector 3328 ${result}.f_coefficients.[ sz j ]) /\
            (forall j. (j >= v i /\ j < 16) ==> ${result}.f_coefficients.[ sz j ] == ${_result}.[ sz j ])
            "#
        ));
        hax_lib::fstar!(
            r#"
          assert (v $i < 16);
          Spec.Utils.pow2_more_values 15;
          assert_norm (1441 < pow2 15);
          assert_norm (1664 < pow2 15);
          assert_norm (mk_i16 1441 <. mk_i16 1664);
          assert(Spec.Utils.is_i16b 1664 (mk_i16 1441))
        "#
        );

        let coefficient_normal_form =
            Vector::montgomery_multiply_by_constant(result.coefficients[i], 1441);

        hax_lib::fstar!(
            r#"
                Spec.Utils.lemma_add_intb_forall 3328 3328;
                assert (6656 <= (pow2 15 - 1));
                Spec.Utils.lemma_intb_le 6656 (pow2 15 - 1);
                assert (forall j. Spec.Utils.is_intb 6656 
                (v (Seq.index (i0.f_to_i16_array ${myself}.f_coefficients.[ i ]) j) +
                 v (Seq.index (i0.f_to_i16_array ${message}.f_coefficients.[ i ]) j)));
                assert (forall j. Spec.Utils.is_intb (pow2 15 - 1) 
                (v (Seq.index (i0.f_to_i16_array ${myself}.f_coefficients.[ i ]) j) +
                 v (Seq.index (i0.f_to_i16_array ${message}.f_coefficients.[ i ]) j)))
            "#
        );

        let sum1 = Vector::add(myself.coefficients[i], &message.coefficients[i]);
        hax_lib::fstar!("assert(is_bounded_vector 6656 sum1)");

        hax_lib::fstar!(
            r#"                
                Spec.Utils.lemma_add_intb_forall 3328 6656;
                Spec.Utils.lemma_intb_le 9984 (pow2 15 - 1);
                Spec.Utils.lemma_intb_le 9984 28296;
                assert (forall j. Spec.Utils.is_intb 9984 
                    (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) j) +
                    v (Seq.index (i0.f_to_i16_array ${sum1}) j)));
                assert (forall j. Spec.Utils.is_intb 28296 
                    (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) j) +
                    v (Seq.index (i0.f_to_i16_array ${sum1}) j)));
                assert (forall j. Spec.Utils.is_intb (pow2 15 - 1)
                    (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) j) +
                    v (Seq.index (i0.f_to_i16_array ${sum1}) j)))
            "#
        );

        let sum2 = Vector::add(coefficient_normal_form, &sum1);
        hax_lib::fstar!("assert(is_bounded_vector 9984 sum2)");
        let red = Vector::barrett_reduce(sum2);
        hax_lib::fstar!("assert(is_bounded_vector 3328 red)");
        result.coefficients[i] = red;

        hax_lib::fstar!(
            r#"
            assert (forall j. (j > v $i /\ j < 16) ==> ${result}.f_coefficients.[ sz j ] == ${_result}.[ sz j]);
            assert (forall j.  j < v $i ==> is_bounded_vector 3328 ${result}.f_coefficients.[ sz j ]);
            assert (${result}.f_coefficients.[ $i ] == ${red});
            assert (forall j. j <= v $i ==> is_bounded_vector 3328 ${result}.f_coefficients.[ sz j ])
        "#
        );
    }
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!("is_bounded_poly 7 ${error}"))]
#[hax_lib::ensures(|result| fstar!(r#"is_bounded_poly 3328 ${myself}_future"#))]
fn add_error_reduce<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    error: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for j in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            (forall j. j < v i ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz j ]) /\
            (forall j. (j >= v i /\ j < 16) ==> ${myself}.f_coefficients.[ sz j ] == ${_myself}.[ sz j ])
            "#
        ));

        let coefficient_normal_form =
            Vector::montgomery_multiply_by_constant(myself.coefficients[j], 1441);

        hax_lib::fstar!(
            r#"
              assert (is_bounded_vector 3328 ${coefficient_normal_form});
              assert (is_bounded_vector 7 (error.f_coefficients.[ j ]));
              Spec.Utils.lemma_add_intb_forall 3328 7;
              assert (forall i. Spec.Utils.is_intb 3335 
                (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) i) +
                 v (Seq.index (i0.f_to_i16_array ${error}.f_coefficients.[ j ]) i)));
              assert_norm (3335 <= pow2 15 - 1);
              Spec.Utils.lemma_intb_le 3335 (pow2 15 - 1);
              Spec.Utils.lemma_intb_le 3335 28296;
              assert (forall i. Spec.Utils.is_intb (pow2 15 - 1) 
                (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) i) +
                v (Seq.index (i0.f_to_i16_array ${error}.f_coefficients.[ j ]) i)));
              assert (forall i. Spec.Utils.is_intb 28296 
                (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) i) +
                v (Seq.index (i0.f_to_i16_array ${error}.f_coefficients.[ j ]) i)))
            "#
        );

        let sum = Vector::add(coefficient_normal_form, &error.coefficients[j]);
        hax_lib::fstar!("assert(is_bounded_vector 3335 sum)");
        let red = Vector::barrett_reduce(sum);
        hax_lib::fstar!("assert(is_bounded_vector 3328 red)");
        myself.coefficients[j] = red;

        hax_lib::fstar!(
            r#"
            assert (forall i. (i > v $j /\ i < 16) ==> ${myself}.f_coefficients.[ sz i ] == ${_myself}.[ sz i]);
            assert (forall i. i < v $j ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz i ]);
            assert (${myself}.f_coefficients.[ $j ] == ${red});
            assert (forall i. i <= v $j ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz i ])
        "#
        );
    }
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (i0.f_to_i16_array ${result}) /\
                (forall i. i < 16 ==> ((v (Seq.index (i0.f_to_i16_array ${result}) i) % 3329)==
                                       (v (Seq.index (i0.f_to_i16_array ${vector}) i) * 1353 * 169) % 3329))"#))]
fn to_standard_domain<T: Operations>(vector: T) -> T {
    T::montgomery_multiply_by_constant(vector, MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS as i16)
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"is_bounded_poly #$:Vector 3328 ${error}"#))]
#[hax_lib::ensures(|result| fstar!(r#"is_bounded_poly 3328 ${myself}_future"#))]
fn add_standard_error_reduce<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    error: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for j in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            (forall j. j < v i ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz j ]) /\
            (forall j. (j >= v i /\ j < 16) ==> ${myself}.f_coefficients.[ sz j ] == ${_myself}.[ sz j ])
            "#
        ));

        // The coefficients are of the form aR^{-1} mod q, which means
        // calling to_montgomery_domain() on them should return a mod q.
        let coefficient_normal_form = to_standard_domain::<Vector>(myself.coefficients[j]);

        hax_lib::fstar!(
            r#"
          Spec.Utils.pow2_more_values 15;
          assert (is_bounded_vector 3328 ${coefficient_normal_form});
          assert (is_bounded_vector 3328 (error.f_coefficients.[ j ]));
          Spec.Utils.lemma_add_intb_forall 3328 3328;
          assert (forall i. Spec.Utils.is_intb 6656 
            (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) i) +
             v (Seq.index (i0.f_to_i16_array ${error}.f_coefficients.[ j ]) i)));
          assert_norm (6656 <= pow2 15 - 1);
          Spec.Utils.lemma_intb_le 6656 (pow2 15 - 1);
          Spec.Utils.lemma_intb_le 6656 28296;
          assert (forall i. Spec.Utils.is_intb (pow2 15 - 1) 
            (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) i) +
            v (Seq.index (i0.f_to_i16_array ${error}.f_coefficients.[ j ]) i)));
          assert (forall i. Spec.Utils.is_intb 28296 
            (v (Seq.index (i0.f_to_i16_array ${coefficient_normal_form}) i) +
            v (Seq.index (i0.f_to_i16_array ${error}.f_coefficients.[ j ]) i)))
        "#
        );

        let sum = Vector::add(coefficient_normal_form, &error.coefficients[j]);
        hax_lib::fstar!("assert(is_bounded_vector 6656 sum)");
        let red = Vector::barrett_reduce(sum);
        hax_lib::fstar!("assert(is_bounded_vector 3328 red)");
        myself.coefficients[j] = red;

        hax_lib::fstar!(
            r#"
            assert (forall i. (i > v $j /\ i < 16) ==> ${myself}.f_coefficients.[ sz i ] == ${_myself}.[ sz i]);
            assert (forall i. i < v $j ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz i ]);
            assert (${myself}.f_coefficients.[ $j ] == ${red});
            assert (forall i. i <= v $j ==> is_bounded_vector 3328 ${myself}.f_coefficients.[ sz i ])
        "#
        );
    }
}

/// Given two `KyberPolynomialRingElement`s in their NTT representations,
/// compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
/// the `iᵗʰ` coefficient of the product `k̂` is determined by the calculation:
///
/// ```plaintext
/// ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X² - ζ^(2·BitRev₇(i) + 1))
/// ```
///
/// This function almost implements <strong>Algorithm 10</strong> of the
/// NIST FIPS 203 standard, which is reproduced below:
///
/// ```plaintext
/// Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
/// Output: An array ĥ ∈ ℤq.
///
/// for(i ← 0; i < 128; i++)
///     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1], ζ^(2·BitRev₇(i) + 1))
/// end for
/// return ĥ
/// ```
/// We say "almost" because the coefficients of the ring element output by
/// this function are in the Montgomery domain.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
// TODO: Remove or replace with something that works and is useful for the proof.
// #[cfg_attr(hax, hax_lib::requires(
//     hax_lib::forall(|i:usize|
//         hax_lib::implies(i < COEFFICIENTS_IN_RING_ELEMENT, ||
//             (lhs.coefficients[i] >= 0 && lhs.coefficients[i] < 4096) &&
//             (rhs.coefficients[i].abs() <= FIELD_MODULUS)

// ))))]
// #[cfg_attr(hax, hax_lib::ensures(|result|
//     hax_lib::forall(|i:usize|
//         hax_lib::implies(i < result.coefficients.len(), ||
//                 result.coefficients[i].abs() <= FIELD_MODULUS
// ))))]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"is_bounded_poly 3328 ${myself} /\
                              is_bounded_poly 3328 ${rhs}"#))]
fn ntt_multiply<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    rhs: &PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    let mut out = ZERO();

    for i in 0..VECTORS_IN_RING_ELEMENT {
        out.coefficients[i] = Vector::ntt_multiply(
            &myself.coefficients[i],
            &rhs.coefficients[i],
            zeta(64 + 4 * i),
            zeta(64 + 4 * i + 1),
            zeta(64 + 4 * i + 2),
            zeta(64 + 4 * i + 3),
        );
    }

    out
}

// FIXME: We pulled out all the items because of https://github.com/hacspec/hax/issues/1183
// Revisit when that issue is fixed.
#[hax_lib::attributes]
impl<Vector: Operations> PolynomialRingElement<Vector> {
    #[allow(non_snake_case)]
    pub(crate) fn ZERO() -> Self {
        Self {
            coefficients: [Vector::ZERO(); 16],
        }
    }

    /// Size of a ring element in bytes.
    #[inline(always)]
    #[allow(dead_code)]
    #[ensures(|result| result == 512 )]
    pub(crate) const fn num_bytes() -> usize {
        VECTORS_IN_RING_ELEMENT * 32
    }

    #[inline(always)]
    #[requires(VECTORS_IN_RING_ELEMENT * 16 <= a.len())]
    pub(crate) fn from_i16_array(a: &[i16]) -> Self {
        from_i16_array(a)
    }

    #[allow(dead_code)]
    #[inline(always)]
    #[requires(VECTORS_IN_RING_ELEMENT * 16 <= out.len())]
    pub(crate) fn to_i16_array(self, out: &mut [i16]) {
        to_i16_array(self, out)
    }

    #[inline(always)]
    #[allow(dead_code)]
    #[requires(VECTORS_IN_RING_ELEMENT * 16 * 2 <= bytes.len())]
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        from_bytes(bytes)
    }

    #[inline(always)]
    #[allow(dead_code)]
    #[requires(VECTORS_IN_RING_ELEMENT * 16 * 2 <= out.len())]
    pub(crate) fn to_bytes(self, out: &mut [u8]) {
        to_bytes(self, out)
    }

    /// Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
    /// sum of their constituent coefficients.
    #[inline(always)]
    #[requires(fstar!(r#"
        forall (i:nat). i < v ${VECTORS_IN_RING_ELEMENT} ==>
            (let lhs_i = i0.f_to_i16_array (self.f_coefficients.[ sz i ]) in
            let rhs_i = i0.f_to_i16_array (${rhs}.f_coefficients.[ sz i ]) in
            Libcrux_ml_kem.Vector.Traits.Spec.add_pre lhs_i rhs_i)"#))]
    #[ensures(|_| fstar!(r#"
        forall (i:nat). i < v ${VECTORS_IN_RING_ELEMENT} ==>
            (let lhs_i = i0.f_to_i16_array (self.f_coefficients.[ sz i ]) in
            let rhs_i = i0.f_to_i16_array (${rhs}.f_coefficients.[ sz i ]) in
            let result_i = i0.f_to_i16_array (self_e_future.f_coefficients.[ sz i ]) in
            Libcrux_ml_kem.Vector.Traits.Spec.add_post lhs_i rhs_i result_i)"#))]
    pub(crate) fn add_to_ring_element<const K: usize>(&mut self, rhs: &Self) {
        add_to_ring_element::<Vector, K>(self, rhs);
    }

    #[inline(always)]
    #[requires(fstar!(r#"is_bounded_poly 28296 self"#))]
    #[ensures(|_| fstar!(r#"is_bounded_poly 3328 self_e_future"#))]
    pub(crate) fn poly_barrett_reduce(&mut self) {
        poly_barrett_reduce(self);
    }

    #[inline(always)]
    #[requires(fstar!(r#"is_bounded_poly (pow2 12 - 1) self"#))]
    #[ensures(|result| fstar!(r#"is_bounded_poly 3328 ${result}"#))]
    pub(crate) fn subtract_reduce(&self, b: Self) -> Self {
        subtract_reduce(self, b)
    }

    #[inline(always)]
    #[requires(fstar!(r#"is_bounded_poly 3328 self /\ 
                         is_bounded_poly 3328 ${message}"#))]
    #[ensures(|output| fstar!(r#"is_bounded_poly 3328 ${output}"#))]
    pub(crate) fn add_message_error_reduce(&self, message: &Self, result: Self) -> Self {
        add_message_error_reduce(self, message, result)
    }

    #[inline(always)]
    #[requires(fstar!("is_bounded_poly 7 ${error}"))]
    pub(crate) fn add_error_reduce(&mut self, error: &Self) {
        add_error_reduce(self, error);
    }

    #[inline(always)]
    #[requires(fstar!(r#"is_bounded_poly #$:Vector 3328 ${error}"#))]
    pub(crate) fn add_standard_error_reduce(&mut self, error: &Self) {
        add_standard_error_reduce(self, error);
    }

    #[inline(always)]
    #[requires(fstar!(r#"is_bounded_poly 3328 self /\
                         is_bounded_poly 3328 ${rhs}"#))]
    pub(crate) fn ntt_multiply(&self, rhs: &Self) -> Self {
        ntt_multiply(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::portable::PortableVector;

    use super::PolynomialRingElement;
    use libcrux_secrets::*;

    #[test]
    fn encoding_portable() {
        type RingElement = PolynomialRingElement<PortableVector>;
        let mut re = RingElement::ZERO();
        re.coefficients[0].elements = [0xAB.classify(); 16];
        re.coefficients[15].elements = [0xCD.classify(); 16];

        let mut bytes = [0u8; RingElement::num_bytes()];
        re.to_bytes(&mut bytes);

        let re_decoded = RingElement::from_bytes(&bytes);

        // Compare
        let mut i16s = [0; RingElement::num_bytes() / 2];
        re.to_i16_array(&mut i16s);

        let mut i16s2 = [0; RingElement::num_bytes() / 2];
        re_decoded.to_i16_array(&mut i16s2);

        assert_eq!(i16s, i16s2);
    }

    #[cfg(feature = "simd128")]
    #[test]
    fn encoding_neon() {
        use crate::vector::{Operations, SIMD128Vector};

        type RingElement = PolynomialRingElement<SIMD128Vector>;
        let mut re = RingElement::ZERO();
        re.coefficients[0] = SIMD128Vector::from_i16_array(&[0xAB; 32]);
        re.coefficients[15] = SIMD128Vector::from_i16_array(&[0xCD; 32]);

        let mut bytes = [0u8; RingElement::num_bytes()];
        re.to_bytes(&mut bytes);

        let re_decoded = RingElement::from_bytes(&bytes);

        // Compare
        let mut i16s = [0; RingElement::num_bytes() / 2];
        re.to_i16_array(&mut i16s);

        let mut i16s2 = [0; RingElement::num_bytes() / 2];
        re_decoded.to_i16_array(&mut i16s2);

        assert_eq!(i16s, i16s2);
    }

    #[cfg(feature = "simd128")]
    #[test]
    fn encoding_interop_portable_neon() {
        use crate::vector::{Operations, SIMD128Vector};

        type RePortable = PolynomialRingElement<PortableVector>;
        let mut re_portable = RePortable::ZERO();
        re_portable.coefficients[0] = PortableVector::from_i16_array(&[0xAB; 16]);
        re_portable.coefficients[15] = PortableVector::from_i16_array(&[0xCD; 16]);

        let mut portable_bytes = [0u8; RePortable::num_bytes()];
        re_portable.to_bytes(&mut portable_bytes);

        type ReNeon = PolynomialRingElement<SIMD128Vector>;
        let mut re_neon = ReNeon::ZERO();
        re_neon.coefficients[0] = SIMD128Vector::from_i16_array(&[0xAB; 16]);
        re_neon.coefficients[15] = SIMD128Vector::from_i16_array(&[0xCD; 16]);

        let mut neon_bytes = [0u8; ReNeon::num_bytes()];
        re_neon.to_bytes(&mut neon_bytes);

        assert_eq!(portable_bytes, neon_bytes);

        let re_portable_decoded = RePortable::from_bytes(&neon_bytes);
        let re_neon_decoded = ReNeon::from_bytes(&portable_bytes);

        // Compare
        let mut i16s_re_portable = [0; RePortable::num_bytes() / 2];
        re_portable.to_i16_array(&mut i16s_re_portable);

        let mut i16s_re_portable_decoded = [0; RePortable::num_bytes() / 2];
        re_portable_decoded.to_i16_array(&mut i16s_re_portable_decoded);

        assert_eq!(i16s_re_portable, i16s_re_portable_decoded);

        let mut i16s_re_neon = [0; ReNeon::num_bytes() / 2];
        re_neon.to_i16_array(&mut i16s_re_neon);

        let mut i16s_re_neon_decoded = [0; ReNeon::num_bytes() / 2];
        re_neon_decoded.to_i16_array(&mut i16s_re_neon_decoded);

        assert_eq!(i16s_re_neon, i16s_re_neon_decoded);
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn encoding_avx2() {
        use crate::vector::{Operations, SIMD256Vector};

        type RingElement = PolynomialRingElement<SIMD256Vector>;
        let mut re = RingElement::ZERO();
        re.coefficients[0] = SIMD256Vector::from_i16_array(&[0xAB; 16]);
        re.coefficients[15] = SIMD256Vector::from_i16_array(&[0xCD; 16]);

        let mut bytes = [0u8; RingElement::num_bytes()];
        re.to_bytes(&mut bytes);

        let re_decoded = RingElement::from_bytes(&bytes);

        // Compare
        let mut i16s = [0; RingElement::num_bytes() / 2];
        re.to_i16_array(&mut i16s);

        let mut i16s2 = [0; RingElement::num_bytes() / 2];
        re_decoded.to_i16_array(&mut i16s2);

        assert_eq!(i16s, i16s2);
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn encoding_interop_portable_avx2() {
        use crate::vector::{Operations, SIMD256Vector};

        type RePortable = PolynomialRingElement<PortableVector>;
        let mut re_portable = RePortable::ZERO();
        re_portable.coefficients[0] = PortableVector::from_i16_array(&[0xAB; 16]);
        re_portable.coefficients[15] = PortableVector::from_i16_array(&[0xCD; 16]);

        let mut portable_bytes = [0u8; RePortable::num_bytes()];
        re_portable.to_bytes(&mut portable_bytes);

        type ReAvx2 = PolynomialRingElement<SIMD256Vector>;
        let mut re_avx2 = ReAvx2::ZERO();
        re_avx2.coefficients[0] = SIMD256Vector::from_i16_array(&[0xAB; 16]);
        re_avx2.coefficients[15] = SIMD256Vector::from_i16_array(&[0xCD; 16]);

        let mut avx2_bytes = [0u8; ReAvx2::num_bytes()];
        re_avx2.to_bytes(&mut avx2_bytes);

        assert_eq!(portable_bytes, avx2_bytes);

        let re_portable_decoded = RePortable::from_bytes(&avx2_bytes);
        let re_avx2_decoded = ReAvx2::from_bytes(&portable_bytes);

        // Compare
        let mut i16s_re_portable = [0; RePortable::num_bytes() / 2];
        re_portable.to_i16_array(&mut i16s_re_portable);

        let mut i16s_re_portable_decoded = [0; RePortable::num_bytes() / 2];
        re_portable_decoded.to_i16_array(&mut i16s_re_portable_decoded);

        assert_eq!(i16s_re_portable, i16s_re_portable_decoded);

        let mut i16s_re_avx2 = [0; ReAvx2::num_bytes() / 2];
        re_avx2.to_i16_array(&mut i16s_re_avx2);

        let mut i16s_re_avx2_decoded = [0; ReAvx2::num_bytes() / 2];
        re_avx2_decoded.to_i16_array(&mut i16s_re_avx2_decoded);

        assert_eq!(i16s_re_avx2, i16s_re_avx2_decoded);
    }
}
