pub(crate) use crate::vector::{
    Operations, PolynomialRingElement, MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS,
    VECTORS_IN_RING_ELEMENT,
};

#[cfg(hax)]
use hax_lib::{int::ToInt, prop::ToProp};

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
#[hax_lib::ensures(|result| result >= -1664 && result <= 1664)]
pub fn zeta(i: usize) -> i16 {
    ZETAS_TIMES_MONTGOMERY_R[i]
}

#[cfg(hax)]
#[allow(dead_code, unused_variables)]
pub(crate) mod spec {

    use crate::vector::{Operations, PolynomialRingElement};

    pub(crate) fn is_bounded_vector<Vector: Operations>(b: usize, vec: &Vector) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (v b) (Libcrux_ml_kem.Vector.Traits.f_to_i16_array vec)"#
        )
    }

    pub(crate) fn is_bounded_poly<Vector: Operations>(
        b: usize,
        p: &PolynomialRingElement<Vector>,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            forall (i:nat). i < 16 ==> is_bounded_vector b (p.f_coefficients.[ sz i ])"#
        )
    }

    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub(crate) fn is_bounded_polynomial_vector<const RANK: usize, Vector: Operations>(
        b: usize,
        v: &[PolynomialRingElement<Vector>; RANK],
    ) -> hax_lib::Prop {
        hax_lib::forall(|i: usize| hax_lib::implies(i < RANK, is_bounded_poly(b, &v[i])))
    }

    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub(crate) fn is_bounded_polynomial_matrix<const RANK: usize, Vector: Operations>(
        b: usize,
        m: &[[PolynomialRingElement<Vector>; RANK]; RANK],
    ) -> hax_lib::Prop {
        hax_lib::forall(|i: usize| {
            hax_lib::implies(i < RANK, is_bounded_polynomial_vector(b, &m[i]))
        })
    }

    /// INTRO (vector): from a per-element bound forall, fold into the opaque
    /// `is_bounded_polynomial_vector` atom.  No SMTPat — call explicitly at
    /// producer sites.
    #[hax_lib::requires(fstar!(r#"
        forall (i:usize). v i < v $RANK ==>
            is_bounded_poly $b (arr.[ i ] <:
                Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    "#))]
    #[hax_lib::ensures(|_| is_bounded_polynomial_vector(b, arr))]
    pub(crate) fn lemma_is_bounded_polynomial_vector_intro<
        const RANK: usize,
        Vector: Operations,
    >(
        arr: &[PolynomialRingElement<Vector>; RANK],
        b: usize,
    ) {
        hax_lib::fstar!(
            r#"reveal_opaque (`%is_bounded_polynomial_vector)
                              (is_bounded_polynomial_vector v_RANK #v_Vector $b $arr)"#
        );
    }

    /// INTRO (matrix): from a per-element bound forall (nested), fold into the
    /// opaque `is_bounded_polynomial_matrix` atom.  No SMTPat.
    #[hax_lib::requires(fstar!(r#"
        forall (i:usize). v i < v $RANK ==>
            is_bounded_polynomial_vector v_RANK $b (m.[ i ] <:
                t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_RANK)
    "#))]
    #[hax_lib::ensures(|_| is_bounded_polynomial_matrix(b, m))]
    pub(crate) fn lemma_is_bounded_polynomial_matrix_intro<
        const RANK: usize,
        Vector: Operations,
    >(
        m: &[[PolynomialRingElement<Vector>; RANK]; RANK],
        b: usize,
    ) {
        hax_lib::fstar!(
            r#"reveal_opaque (`%is_bounded_polynomial_matrix)
                              (is_bounded_polynomial_matrix v_RANK #v_Vector $b $m)"#
        );
    }

    #[hax_lib::requires(is_bounded_vector(b1, vec) & (b1 <= b2))]
    #[hax_lib::ensures(|_| is_bounded_vector(b2, vec))]
    pub(crate) fn is_bounded_vector_higher<Vector: Operations>(vec: &Vector, b1: usize, b2: usize) {
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
        );
    }

    #[hax_lib::requires(is_bounded_poly(b1, p) & (b1 <= b2))]
    #[hax_lib::ensures(|_| is_bounded_poly(b2, p))]
    pub(crate) fn is_bounded_poly_higher<Vector: Operations>(
        p: &PolynomialRingElement<Vector>,
        b1: usize,
        b2: usize,
    ) {
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
        );
    }


    /// Conversion lemma: from the strengthened decompress trait post
    /// `bounded_i16_array (mk_i16 0) (mk_i16 3328) (f_repr v)` derive
    /// `is_bounded_vector b v` for any b >= 3328 (b given as parameter
    /// to allow callers like `deserialize_then_decompress_4`/`_5` to
    /// expose the loose `is_bounded_poly 4095` bound that
    /// `Matrix.compute_message`'s `subtract_reduce` requires).
    #[hax_lib::requires(fstar!(r#"
        Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array (mk_i16 0) (mk_i16 3328)
            (Libcrux_ml_kem.Vector.Traits.f_repr $vec) /\ v $b >= 3328 /\ v $b < 32768
    "#))]
    #[hax_lib::ensures(|_| is_bounded_vector(b, vec))]
    #[cfg_attr(hax, hax_lib::fstar::after(r#"
(* SMTPat'd elim lemmas (consume direction).  The dual-trigger multi-pattern
   `[SMTPat (Seq.index arr i); SMTPat (is_bounded_polynomial_vector ...)]`
   only fires when Z3 has BOTH the indexed access AND the opaque atom in its
   E-graph — instead of dumping every per-index bound whenever the predicate
   appears.  Same idiom as `Vector.Traits.Spec.lemma_bounded_i16_array_lookup`. *)

(* The elim lemmas take a `usize` index (matching the underlying body's
   `forall (i: usize)`).  We provide BOTH a `usize`-indexed and a `nat`-
   indexed form so consumers using either index type fire automatically.
   The nat form delegates to the usize form via `mk_int` rebox. *)

let lemma_is_bounded_polynomial_vector_elim
      (v_RANK: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (b: usize)
      (arr: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_RANK)
      (i: usize)
    : Lemma
        (requires is_bounded_polynomial_vector v_RANK #v_Vector b arr /\
                  v i < v v_RANK)
        (ensures is_bounded_poly #v_Vector b (Seq.index arr (v i)))
        [SMTPat (Seq.index arr (v i));
         SMTPat (is_bounded_polynomial_vector v_RANK #v_Vector b arr)] =
  reveal_opaque (`%is_bounded_polynomial_vector)
                 (is_bounded_polynomial_vector v_RANK #v_Vector b arr)

let lemma_is_bounded_polynomial_matrix_elim
      (v_RANK: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (b: usize)
      (m: t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_RANK) v_RANK)
      (i: usize)
    : Lemma
        (requires is_bounded_polynomial_matrix v_RANK #v_Vector b m /\
                  v i < v v_RANK)
        (ensures is_bounded_polynomial_vector v_RANK #v_Vector b (Seq.index m (v i)))
        [SMTPat (Seq.index m (v i));
         SMTPat (is_bounded_polynomial_matrix v_RANK #v_Vector b m)] =
  reveal_opaque (`%is_bounded_polynomial_matrix)
                 (is_bounded_polynomial_matrix v_RANK #v_Vector b m)

(* Nat-indexed convenience trigger: fires when consumer code accesses
   `Seq.index arr i` directly (e.g. in lemma_aux contexts where i: nat). *)

let lemma_is_bounded_polynomial_vector_elim_nat
      (v_RANK: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (b: usize)
      (arr: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_RANK)
      (i: nat)
    : Lemma
        (requires is_bounded_polynomial_vector v_RANK #v_Vector b arr /\
                  i < v v_RANK)
        (ensures is_bounded_poly #v_Vector b (Seq.index arr i))
        [SMTPat (Seq.index arr i);
         SMTPat (is_bounded_polynomial_vector v_RANK #v_Vector b arr)] =
  reveal_opaque (`%is_bounded_polynomial_vector)
                 (is_bounded_polynomial_vector v_RANK #v_Vector b arr);
  (* Instantiate the body's `forall (i_u: usize)` at the usize whose v-cast
     equals our nat i. *)
  assert (v (mk_int #usize_inttype i) == i)

let lemma_is_bounded_polynomial_matrix_elim_nat
      (v_RANK: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (b: usize)
      (m: t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_RANK) v_RANK)
      (i: nat)
    : Lemma
        (requires is_bounded_polynomial_matrix v_RANK #v_Vector b m /\
                  i < v v_RANK)
        (ensures is_bounded_polynomial_vector v_RANK #v_Vector b (Seq.index m i))
        [SMTPat (Seq.index m i);
         SMTPat (is_bounded_polynomial_matrix v_RANK #v_Vector b m)] =
  reveal_opaque (`%is_bounded_polynomial_matrix)
                 (is_bounded_polynomial_matrix v_RANK #v_Vector b m);
  assert (v (mk_int #usize_inttype i) == i)

(* HIGHER (vector): widen the per-element bound on an opaque
   `is_bounded_polynomial_vector` atom from b1 to b2 (b1 <= b2).
   Strategy: instantiate the SMTPat'd elim lemma at every i, then call
   the per-poly higher widening, then re-fold via the intro reveal. *)

#push-options "--z3rlimit 400 --ext context_pruning --split_queries always"
let lemma_is_bounded_polynomial_vector_higher
      (v_RANK: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (arr: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_RANK)
      (b1 b2: usize)
    : Lemma (requires is_bounded_polynomial_vector v_RANK #v_Vector b1 arr /\
                      v b1 <= v b2)
            (ensures is_bounded_polynomial_vector v_RANK #v_Vector b2 arr) =
  assert (Seq.length arr == v v_RANK);
  let aux (i: nat{i < Seq.length arr})
      : Lemma (i < v v_RANK ==>
               is_bounded_poly #v_Vector b2 (Seq.index arr i)) =
    if i < v v_RANK then begin
      lemma_is_bounded_polynomial_vector_elim_nat v_RANK #v_Vector b1 arr i;
      is_bounded_poly_higher #v_Vector (Seq.index arr i) b1 b2
    end
  in
  Classical.forall_intro aux;
  lemma_is_bounded_polynomial_vector_intro v_RANK #v_Vector arr b2
#pop-options
"#))]
    pub(crate) fn lemma_decompress_post_to_is_bounded_vector<Vector: Operations>(
        vec: &Vector,
        b: usize,
    ) {
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                              (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque);
               // Trigger the typeclass post for f_to_i16_array on `vec` so
               // Z3 sees `f_to_i16_array vec == f_repr vec`.
               let _ = Libcrux_ml_kem.Vector.Traits.f_to_i16_array $vec in
               ()"#
        );
    }
}

#[inline(always)]
#[hax_lib::requires(spec::is_bounded_vector(_b1, &vec1) & (spec::is_bounded_vector(_b2, vec2) & (_b1 < 32768 && _b2 < 32768 && _b1 + _b2 < 32768)))]
#[hax_lib::ensures(|result| spec::is_bounded_vector(_b1+_b2, &result) & (crate::vector::traits::spec::add_post(&vec1.repr(), &vec2.repr(), &result.repr())))]
pub(crate) fn add_bounded<Vector: Operations>(
    vec1: Vector,
    _b1: usize,
    vec2: &Vector,
    _b2: usize,
) -> Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    Vector::add(vec1, vec2)
}

#[inline(always)]
#[hax_lib::requires(spec::is_bounded_vector(_b1, &vec1) & (spec::is_bounded_vector(_b2, vec2) & (_b1 < 32768 && _b2 < 32768 && _b1 + _b2 < 32768)))]
#[hax_lib::ensures(|result| spec::is_bounded_vector(_b1+_b2, &result) & (crate::vector::traits::spec::sub_post(&vec1.repr(), &vec2.repr(), &result.repr())))]
pub(crate) fn sub_bounded<Vector: Operations>(
    vec1: Vector,
    _b1: usize,
    vec2: &Vector,
    _b2: usize,
) -> Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    Vector::sub(vec1, vec2)
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 100 --split_queries always")]
#[hax_lib::requires(spec::is_bounded_vector(_b, &vec) & (c > -32768 && _b.to_int() * c.abs().to_int() < 32768.to_int()))]
#[hax_lib::ensures(|result| fstar!(r#"let abs_c = Core_models.Num.impl_i16__abs $c in
          b2t (abs_c >=. mk_i16 0 && abs_c <=. mk_i16 32767) /\
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #$:Vector
            (mk_usize (v $_b * v abs_c))
            $result"#))]
pub(crate) fn multiply_by_constant_bounded<Vector: Operations>(
    vec: Vector,
    _b: usize,
    c: i16,
) -> Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    // TODO: the following assume should be removed once hax_lib is updated to latest version
    hax_lib::fstar!(
        r#"assume (if v c >= 0 then Rust_primitives.Arithmetic.abs_i16 c = c else v (Rust_primitives.Arithmetic.abs_i16 c) = 0 - v c)"#
    );
    Vector::multiply_by_constant(vec, c)
}

#[allow(non_snake_case)]
#[hax_lib::ensures(|result| spec::is_bounded_poly(0, &result))]
fn ZERO<Vector: Operations>() -> PolynomialRingElement<Vector> {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 0)"#
    );
    PolynomialRingElement {
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
#[hax_lib::ensures(|_| future(out).len() == out.len())]
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
#[hax_lib::ensures(|_| future(out).len() == out.len())]
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
#[hax_lib::requires(out.len() <= 4 && 512 * out.len() <= bytes.len())]
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
#[hax_lib::ensures(|result| result == K * 512)]
#[allow(dead_code)]
pub(crate) const fn vec_len_bytes<const K: usize, Vector: Operations>() -> usize {
    K * PolynomialRingElement::<Vector>::num_bytes()
}

/// Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
/// sum of their constituent coefficients.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 500 --split_queries always")]
#[hax_lib::requires((_bound <= 4* 3328).to_prop() & (spec::is_bounded_poly(_bound, &myself) & (spec::is_bounded_poly(3328, &rhs))))]
#[hax_lib::ensures(|_| fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #$:Vector
                                  (${_bound} +! mk_usize 3328) ${myself}_future /\
                                Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector
                                  ${myself}_future ==
                                Hacspec_ml_kem.Polynomial.add_to_ring_element
                                  (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain
                                    #$:Vector ${myself})
                                  (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain
                                    #$:Vector ${rhs})"#))]
fn add_to_ring_element<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    rhs: &PolynomialRingElement<Vector>,
    _bound: usize, // Used to state properties about the bound on myself
) {
    #[cfg(hax)]
    let _myself_orig = myself.coefficients;

    for i in 0..16 {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(_bound + 3328, &myself.coefficients[j])
                        & fstar!(
                            r#"Libcrux_ml_kem.Vector.Traits.Spec.add_post
                                 (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                   (Seq.index ${_myself_orig} (v $j)))
                                 (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                   (Seq.index ${rhs}.f_coefficients (v $j)))
                                 (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                   (Seq.index ${myself}.f_coefficients (v $j)))"#
                        )
                } else {
                    spec::is_bounded_vector(_bound, &myself.coefficients[j])
                        & fstar!(
                            r#"Seq.index ${myself}.f_coefficients (v $j) ==
                               Seq.index ${_myself_orig} (v $j)"#
                        )
                }
            } else {
                true.to_prop()
            }
        }));

        myself.coefficients[i] =
            add_bounded(myself.coefficients[i], _bound, &rhs.coefficients[i], 3328);
    }
    // Phase 7a (E2): cite Hacspec_ml_kem.Polynomial.add_to_ring_element.
    // The strengthened loop invariant carries per-vector add_pre + add_post
    // for already-processed chunks; the Tier-1 lemma lifts to poly-level.
    hax_lib::fstar!(
        r#"
          Hacspec_ml_kem.Commute.Chunk.lemma_add_to_ring_element_commute
            #$:Vector
            ({ Libcrux_ml_kem.Vector.f_coefficients = ${_myself_orig} })
            ${rhs}
            ${myself}
        "#
    );
}

#[inline(always)]
#[hax_lib::requires(spec::is_bounded_poly(28296, &myself))]
#[hax_lib::ensures(|_| fstar!(r#"Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #$:Vector
                                  (mk_usize 3328) ${myself}_future /\
                                Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector
                                  ${myself}_future ==
                                Hacspec_ml_kem.Polynomial.poly_barrett_reduce
                                  (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain
                                    #$:Vector ${myself})"#))]
pub(crate) fn poly_barrett_reduce<Vector: Operations>(myself: &mut PolynomialRingElement<Vector>) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &myself.coefficients[j])
                        & fstar!(
                            r#"Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_post
                                 (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                   (Seq.index ${_myself} (v $j)))
                                 (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                   (Seq.index ${myself}.f_coefficients (v $j)))"#
                        )
                } else {
                    spec::is_bounded_vector(28296, &myself.coefficients[j])
                        & fstar!(
                            r#"Seq.index ${myself}.f_coefficients (v $j) ==
                               Seq.index ${_myself} (v $j)"#
                        )
                }
            } else {
                true.to_prop()
            }
        }));

        myself.coefficients[i] = Vector::barrett_reduce(myself.coefficients[i]);
    }
    // Phase 7a (E1): cite Hacspec_ml_kem.Polynomial.poly_barrett_reduce.
    // After the loop, the strengthened invariant gives us, for each chunk,
    // the per-vector `barrett_reduce_post (orig.[k]) (curr.[k])`.  The
    // Tier-1 lemma `lemma_poly_barrett_reduce_commute` lifts this to the
    // poly-level hacspec equation.
    hax_lib::fstar!(
        r#"
          Hacspec_ml_kem.Commute.Chunk.lemma_poly_barrett_reduce_commute
            #$:Vector
            ({ Libcrux_ml_kem.Vector.f_coefficients = ${_myself} })
            ${myself}
        "#
    );
}

/// Compute `myself - InvNTT(b)` lane-wise, with the `· 128⁻¹` finalize of
/// the inverse NTT fused into the per-lane `mont_mul(_, 1441)` here (saving
/// ~80 SIMD ops per call vs running invert_ntt_montgomery's finalize +
/// a separate scale).
///
/// Scaling on entry:
/// - `myself` is **plain** (`v c ≡ α mod q`) — caller deserialized v from
///   the ciphertext via `decompress_then_deserialize_ring_element_v`,
///   which produces plain coefficients.
/// - `b` is in libcrux's **`·R⁻¹` form** (`v c ≡ β · R⁻¹ mod q`) — output
///   of `invert_ntt_montgomery`, which preserves the `·R⁻¹` form set by
///   `ntt_multiply` (each `mont_mul` carries one `·R⁻¹` factor through).
///
/// The fused `mont_mul(b, 1441)` step:
///   `mont_mul(b, 1441) = (β · R⁻¹) · 1441 · R⁻¹ = β · R²/128 · R⁻²
///                       = β · 1/128 (mod q)`
/// where `1441 = R²/128 mod q` per `pq-crystals/kyber/main/ref/ntt.c:106`.
/// This simultaneously discharges the missing `· 128⁻¹` from
/// `invert_ntt_montgomery`'s 7-layer GS chain AND brings the lane back
/// to plain form (`v r ≡ β · 128⁻¹ mod q`), so `myself - mont_mul(b, 1441)`
/// is plain-form arithmetic.  Result is plain post-Barrett.
///
/// See `src/invert_ntt.rs` (above `invert_ntt_montgomery`) for the
/// upstream chain doc.
// CLOSED 2026-04-29 (Phase 7a / lane A3): body discharged via
// hypothesis (b) — array-form `to_spec_poly_mont_arr` + unfold lemma
// + parameter unshadowing (loop uses local `b_acc`, parameter `b`
// reachable from post-loop fragment).  All algebra + commute lemmas
// in `Hacspec_ml_kem.Commute.Chunk` (commits `c698908ba`, `0a8c7289d`,
// + Phase 7a/A3 additions: `to_spec_poly_mont_arr`,
// `lemma_to_spec_poly_mont_unfold`, `lemma_subtract_reduce_scaled_eq`).
// Sibling fns `add_message_error_reduce` / `add_error_reduce` still
// have bounds-only posts; strengthening those (per-fe + per-chunk +
// per-poly commute chain) is open follow-up — see MLKEM_STATUS USER-7.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(4095, &myself))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &result)
    & fstar!(r#"
        Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${result}
          == Hacspec_ml_kem.Polynomial.subtract_reduce
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${myself})
               (Hacspec_ml_kem.Parameters.createi
                  #Hacspec_ml_kem.Parameters.t_FieldElement
                  (mk_usize 256)
                  #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                  (fun (j: usize {j <. mk_usize 256}) ->
                    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Seq.index
                         (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${b}) (v j))
                      Hacspec_ml_kem.Commute.Chunk.fe_1441))
      "#))]
fn subtract_reduce<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    b: PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    let _b = b.coefficients;

    // Phase 7a / lane A3: keep the parameter `b` UNSHADOWED so we can
    // refer to it in the post-loop bridge lemma.  The mutable loop
    // accumulator goes through a freshly-named local `b_acc`.  The
    // post (in the .fsti) references the parameter `b`, so reaching it
    // by name from the body is required for the createi-extensionality
    // lemma at the end.  Without this rename, hax's shadowing the
    // parameter inside the `for` made the body unable to talk about
    // the parameter directly — three prior session attempts hit Q143
    // saturated rlimit 800 specifically because of this.
    let mut b_acc: PolynomialRingElement<Vector> = b;

    // Seed F1 (still scoped to the parameter `b`):
    //   to_spec_poly_mont (param b) == to_spec_poly_mont_arr e_b
    // The post-loop bridge then chains via `lemma_subtract_reduce_scaled_eq`
    // on the parameter `b` and the constructed `b_input` (sharing
    // `f_coefficients == e_b`).
    hax_lib::fstar!(r#"
        Hacspec_ml_kem.Commute.Chunk.lemma_to_spec_poly_mont_unfold #v_Vector $b
      "#);

    for i in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant uses an OPAQUE per-vector predicate
        // (subtract_reduce_finalize_chunk) for already-processed chunks,
        // and an array-equality marker for unprocessed chunks (so the
        // body's trait posts of mont_mul talk about _b[i]).
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &b_acc.coefficients[j])
                        & fstar!(r#"
                            Hacspec_ml_kem.Commute.Chunk.subtract_reduce_finalize_chunk
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${myself}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${b_acc}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${_b} (v $j)))
                          "#)
                } else {
                    fstar!(r#"
                        Seq.index ${b_acc}.f_coefficients (v $j) == Seq.index ${_b} (v $j)
                      "#)
                }
            } else {
                true.to_prop()
            }
        }));

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
            Vector::montgomery_multiply_by_constant(b_acc.coefficients[i], 1441);

        let diff = sub_bounded(myself.coefficients[i], 4095, &coefficient_normal_form, 3328);

        #[cfg(hax)]
        spec::is_bounded_vector_higher(&diff, 7423, 28296);

        hax_lib::assert_prop!(spec::is_bounded_vector(28296, &diff));
        let red = Vector::barrett_reduce(diff);
        hax_lib::assert_prop!(spec::is_bounded_vector(3328, &red));

        // Encapsulated per-iteration helper: takes the trait posts of
        // mont_mul, sub, barrett at chunk i and produces the opaque
        // chunk-level finalize predicate.  Single-call interface keeps
        // the loop body's Z3 context small.
        hax_lib::fstar!(r#"
            Hacspec_ml_kem.Commute.Chunk.lemma_subtract_reduce_iter
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${myself}.f_coefficients (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${_b} (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${coefficient_normal_form})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${diff})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${red})
          "#);

        b_acc.coefficients[i] = red;
    }

    // Post-loop bridge: lift the 16 per-chunk opaque finalize predicates
    // (from the loop invariant at i = 16) to the polynomial-level equation
    // citing HP.subtract_reduce.  The commute lemma produces the helper-
    // form `subtract_reduce_helper`; the eq-helper lemma chains it to
    // HP.subtract_reduce.
    //
    // Phase 7a / lane A3: with the parameter `b` unshadowed (loop uses
    // `b_acc`), invoke `lemma_subtract_reduce_scaled_eq` directly on
    // `(b, b_input)` — both share `f_coefficients == e_b`, so their
    // createi-of-`to_spec_poly_mont` outputs coincide.  This is the
    // bridge from the lemma's conclusion (uses `b_input`) to the post
    // (uses `b`) without resorting to record extensionality or
    // per-index Seq.lemma_eq_intro inside the body.
    hax_lib::fstar!(r#"
        let b_input : Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
          { Libcrux_ml_kem.Vector.f_coefficients = ${_b} } in
        Hacspec_ml_kem.Commute.Chunk.lemma_subtract_reduce_commute
          #v_Vector myself b_input b_acc;
        let myself_lift = Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself in
        let scaled_b = Hacspec_ml_kem.Parameters.createi
                         #Hacspec_ml_kem.Parameters.t_FieldElement
                         (mk_usize 256)
                         #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                         (fun (j: usize {j <. mk_usize 256}) ->
                           Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                             (Seq.index
                                (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector b_input)
                                (v j))
                             Hacspec_ml_kem.Commute.Chunk.fe_1441) in
        Hacspec_ml_kem.Commute.Chunk.lemma_subtract_reduce_eq_helper myself_lift scaled_b;
        Hacspec_ml_kem.Commute.Chunk.lemma_subtract_reduce_scaled_eq #v_Vector $b b_input
      "#);

    b_acc
}

/// Compute `myself + message + InvNTT(result)` lane-wise — same fused
/// `mont_mul(result, 1441)` pattern as `subtract_reduce`.
///
/// Scaling on entry:
/// - `myself` (= `error_2` at the call site `compute_ring_element_v` in
///   matrix.rs): plain (`v ≡ α mod q`).
/// - `message`: plain (output of `deserialize_then_decompress_message`).
/// - `result`: `·R⁻¹` form (output of `invert_ntt_montgomery`).
///
/// `mont_mul(result, 1441)` brings `result` to plain (per the fused-finalize
/// algebra documented above `subtract_reduce`), then plain + plain + plain
/// → plain, Barrett-reduced into `[0, q)`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3328, &myself) & (spec::is_bounded_poly(3328, &message)))]
#[hax_lib::ensures(|output| spec::is_bounded_poly(3328, &output))]
fn add_message_error_reduce<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    message: &PolynomialRingElement<Vector>,
    mut result: PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    let _result = result.coefficients;

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &result.coefficients[j])
                } else {
                    spec::is_bounded_vector(3328, &myself.coefficients[j])
                }
            } else {
                true.to_prop()
            }
        }));

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

        let sum1 = add_bounded(myself.coefficients[i], 3328, &message.coefficients[i], 3328);
        hax_lib::assert_prop!(spec::is_bounded_vector(6656, &sum1));

        let sum2 = add_bounded(coefficient_normal_form, 3328, &sum1, 6656);

        hax_lib::assert_prop!(spec::is_bounded_vector(9984, &sum2));
        #[cfg(hax)]
        spec::is_bounded_vector_higher(&sum2, 9984, 28296);

        let red = Vector::barrett_reduce(sum2);
        hax_lib::assert_prop!(spec::is_bounded_vector(3328, &red));
        result.coefficients[i] = red;
    }
    result
}

/// Compute `InvNTT(myself) + error` lane-wise — fused `mont_mul(myself, 1441)`
/// pattern as `subtract_reduce`.
///
/// Scaling on entry:
/// - `myself`: `·R⁻¹` form (output of `invert_ntt_montgomery` at the call
///   site `compute_vector_u` in matrix.rs).
/// - `error` (= `error_1[i]`): plain, small (`is_bounded_poly(7)` — direct
///   from CBD sampling).
///
/// `mont_mul(myself, 1441)` brings `myself` to plain (fused finalize), then
/// `plain + plain → plain`, Barrett-reduced.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(7, &error))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, &future(myself)))]
fn add_error_reduce<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    error: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for j in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &myself.coefficients[j])
                } else {
                    true.to_prop()
                }
            } else {
                true.to_prop()
            }
        }));

        let coefficient_normal_form =
            Vector::montgomery_multiply_by_constant(myself.coefficients[j], 1441);

        let sum = add_bounded(coefficient_normal_form, 3328, &error.coefficients[j], 7);

        hax_lib::assert_prop!(spec::is_bounded_vector(3335, &sum));
        #[cfg(hax)]
        spec::is_bounded_vector_higher(&sum, 3335, 28296);

        let red = Vector::barrett_reduce(sum);
        hax_lib::assert_prop!(spec::is_bounded_vector(3328, &red));
        myself.coefficients[j] = red;
    }
}

#[inline(always)]
#[hax_lib::ensures(|result| spec::is_bounded_vector(3328, &result) & fstar!(r#"
                    Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                      Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post
                        (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr #$:T ${vector}) i)
                        Libcrux_ml_kem.Vector.Traits.v_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS
                        (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr #$:T ${result}) i))
                "#))]
fn to_standard_domain<T: Operations>(vector: T) -> T {
    T::montgomery_multiply_by_constant(vector, MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS as i16)
}

/// Compute `to_standard_domain(myself) + error` lane-wise — different
/// fused-finalize pattern from the three INTT-track reduce fns above.
///
/// Scaling on entry:
/// - `myself` (= `t_as_ntt[i]` at the call site `compute_As_plus_e` in
///   matrix.rs): `·R⁻¹` form (output of accumulated `ntt_multiply` chain).
///   This is the **post-matrix-multiply** path — there is NO inverse NTT
///   upstream here; `myself` is in the NTT domain still.
/// - `error` (= `error_as_ntt[i]`): plain (sampled CBD then NTT'd; NTT
///   preserves plain input scaling — see `src/ntt.rs` cross-spec test
///   `ntt_matches_spec`).
///
/// `to_standard_domain(myself) = mont_mul(myself, R²) = mont_mul(myself, 1353)`
/// applies a single `· R²` factor: `(α · R⁻¹) · 1353 · R⁻¹ = α · R² · R⁻²
/// = α (mod q)`, bringing `myself` to plain form.  Result is plain
/// post-Barrett.  Note `1353 = R² mod q` ≠ `1441 = R²/128 mod q` — the
/// distinction is the missing `· 128⁻¹` factor that ONLY applies to the
/// INTT track (where `invert_ntt_montgomery` skips its FIPS-203 finalize).
// Phase 7a Step 7 (agent-trackD F* infra + agent-trackA Option B
// attempt, 2026-04-28).  F* per-lane and poly-level commute lemmas
// landed in `Hacspec_ml_kem.Commute.Chunk` and verified:
//   - `mont_form_lane`, `mont_form_chunk`: opaque per-lane/chunk
//      standard-domain (`· R⁻¹`) form predicate.
//   - `lemma_to_standard_domain_finalize_fe`: per-lane consumer mirror of
//      `lemma_intt_mont_finalize_fe`.
//   - `lemma_add_standard_error_reduce_lane{,_closed}`: lane bridge
//      (mont_mul + add + barrett ⟹ FE-add equation).  The `_closed`
//      variant takes a single composed mod-q identity instead of three
//      trait posts (designed for Option B's loop invariant).
//   - `lemma_add_standard_error_reduce_commute`: poly-level Tier-1 commute
//      assembling 256 lane equations into the hacspec function identity,
//      parameterized by a ghost `ntt_product : array t_FieldElement 256`.
//
// Step 7.2 (Rust ensures + body) STILL HELD.  Two attempts:
//   - trackD's nested-forall invariant: Z3 timeout on outer
//     `forall ntt_lane. mont_form_lane ==> FE-add` (~85 s/query).
//   - trackA's Option B closed-form invariant
//     (`forall l. v myself % q == (v _myself * 1353 * 169 + v error) % q`):
//     Z3 timeout on the loop body subtyping check (~230 s and ~380 s
//     per failed query at rlimit 800/800 saturated; Q79, Q108, Q109 of
//     the body fail "canceled").  The closed form is structurally
//     simpler than the nested-forall, but the per-iteration accumulator
//     refinement check is still too heavy for Z3.
//
// Likely paths forward (try one of these in a future session):
//   1. Add an explicit ghost `ntt_product` Rust parameter (specialize
//      the post — drop the universal forall over ntt_product) and a
//      precondition citing `mont_form_chunk` per chunk.  Loop invariant
//      then carries specialized per-lane FE-add eq parameterized by
//      `ntt_product[k*16+l]` directly (no inner forall, no closed-form
//      mod arithmetic).  Post becomes a direct citation, no
//      Classical.forall_intro at the boundary.
//   2. Refactor to factor the body proof into an external lemma that
//      reasons about the impl trace abstractly, keeping the loop body
//      simple.  Requires F*-side scaffolding.
//   3. Hand-decompose the body proof: replace `--split_queries always`
//      with explicit per-iteration assert/`#push-options`, profiling
//      each failed sub-query to find the actual hot spot.
//
// Tracking: see `proofs/agent-status/agent-trackD.md` for the original
// hold context, and `agent-trackA.md` for the Option B failure.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3328, &error))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, &future(myself)))]
fn add_standard_error_reduce<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    error: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for j in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &myself.coefficients[j])
                } else {
                    true.to_prop()
                }
            } else {
                true.to_prop()
            }
        }));

        // The coefficients are of the form aR^{-1} mod q, which means
        // calling to_montgomery_domain() on them should return a mod q.
        let coefficient_normal_form = to_standard_domain::<Vector>(myself.coefficients[j]);

        let sum = add_bounded(coefficient_normal_form, 3328, &error.coefficients[j], 3328);

        hax_lib::assert_prop!(spec::is_bounded_vector(6656, &sum));
        #[cfg(hax)]
        spec::is_bounded_vector_higher(&sum, 6656, 28296);

        let red = Vector::barrett_reduce(sum);
        hax_lib::assert_prop!(spec::is_bounded_vector(3328, &red));
        myself.coefficients[j] = red;
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
#[hax_lib::requires(spec::is_bounded_poly(3328, &myself) & (spec::is_bounded_poly(3328, &rhs)))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, &result))]
fn ntt_multiply<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    rhs: &PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    let mut out = ZERO();

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            hax_lib::implies(j < i, spec::is_bounded_vector(3328, &out.coefficients[j]))
        }));

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
    #[ensures(|result| spec::is_bounded_poly(0, &result))]
    pub(crate) fn ZERO() -> Self {
        ZERO()
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
    #[ensures(|_| future(out).len() == out.len())]
    pub(crate) fn to_bytes(self, out: &mut [u8]) {
        to_bytes(self, out)
    }

    /// Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
    /// sum of their constituent coefficients.
    #[inline(always)]
    #[hax_lib::requires((_b <= 4* 3328).to_prop() & (spec::is_bounded_poly(_b, &self) & (spec::is_bounded_poly(3328, &rhs))))]
    #[hax_lib::ensures(|_| spec::is_bounded_poly(_b + 3328, &future(self)))]
    pub(crate) fn add_to_ring_element(&mut self, rhs: &Self, _b: usize) {
        add_to_ring_element::<Vector>(self, rhs, _b);
    }

    #[inline(always)]
    #[requires(spec::is_bounded_poly(28296, &self))]
    #[ensures(|_| spec::is_bounded_poly(3328, &future(self)))]
    pub(crate) fn poly_barrett_reduce(&mut self) {
        poly_barrett_reduce(self);
    }

    #[inline(always)]
    #[requires(spec::is_bounded_poly(4095, &self))]
    #[ensures(|result| spec::is_bounded_poly(3328, &result))]
    pub(crate) fn subtract_reduce(&self, b: Self) -> Self {
        subtract_reduce(self, b)
    }

    #[inline(always)]
    #[requires(spec::is_bounded_poly(3328, &self) & (spec::is_bounded_poly(3328, &message)))]
    #[ensures(|output| spec::is_bounded_poly(3328, &output))]
    pub(crate) fn add_message_error_reduce(&self, message: &Self, result: Self) -> Self {
        add_message_error_reduce(self, message, result)
    }

    #[inline(always)]
    #[requires(spec::is_bounded_poly(7, &error))]
    #[ensures(|result| spec::is_bounded_poly(3328, &future(self)))]
    pub(crate) fn add_error_reduce(&mut self, error: &Self) {
        add_error_reduce(self, error);
    }

    #[inline(always)]
    #[requires(spec::is_bounded_poly(3328, &error))]
    #[ensures(|result| spec::is_bounded_poly(3328, &future(self)))]
    pub(crate) fn add_standard_error_reduce(&mut self, error: &Self) {
        add_standard_error_reduce(self, error);
    }

    #[inline(always)]
    #[requires(spec::is_bounded_poly(3328, &self) & (spec::is_bounded_poly(3328, &rhs)))]
    #[ensures(|result| spec::is_bounded_poly(3328, &result))]
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

/// Lifting functions and cross-spec tests for polynomial operations.
#[cfg(test)]
pub(crate) mod cross_spec_tests {
    use super::*;
    use crate::vector::portable::PortableVector;
    use crate::vector::Operations;
    use hacspec_ml_kem::parameters::{self as spec, FieldElement, Polynomial};

    /// Lift an impl PolynomialRingElement to a spec Polynomial.
    ///
    /// Each i16 coefficient c is converted to FieldElement via c.rem_euclid(3329).
    /// This mirrors the F* function `to_spec_poly_t` / `to_spec_fe`.
    ///
    /// Valid for time-domain polynomials or any polynomial whose coefficients
    /// have been Barrett-reduced (i.e., are in [-3328, 3328]).
    pub(crate) fn lift_poly<Vector: Operations>(p: &PolynomialRingElement<Vector>) -> Polynomial {
        core::array::from_fn(|i| {
            let coeffs = Vector::to_i16_array(p.coefficients[i / 16]);
            let c = coeffs[i % 16] as i32;
            FieldElement::new(c.rem_euclid(3329) as u16)
        })
    }

    /// Lift an impl PolynomialRingElement from Montgomery domain to spec Polynomial.
    ///
    /// For NTT-domain polynomials stored as a*R mod q (where R = 2^16 mod q),
    /// this divides by R to recover the plain value.
    pub(crate) fn lift_poly_montgomery<Vector: Operations>(
        p: &PolynomialRingElement<Vector>,
    ) -> Polynomial {
        const MONT_R_INV: u32 = 169; // 2^{-16} mod 3329
        core::array::from_fn(|i| {
            let coeffs = Vector::to_i16_array(p.coefficients[i / 16]);
            let c = (coeffs[i % 16] as i32).rem_euclid(3329) as u32;
            FieldElement::new((c * MONT_R_INV % 3329) as u16)
        })
    }

    /// Create an impl PolynomialRingElement from a spec Polynomial.
    pub(crate) fn unlift_poly(p: &Polynomial) -> PolynomialRingElement<PortableVector> {
        let mut result = PolynomialRingElement::<PortableVector>::ZERO();
        for i in 0..16 {
            let mut coeffs = [0i16; 16];
            for j in 0..16 {
                coeffs[j] = p[i * 16 + j].val as i16;
            }
            result.coefficients[i] = PortableVector::from_i16_array(&coeffs);
        }
        result
    }

    #[test]
    fn lift_unlift_roundtrip() {
        let spec_poly: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 13 + 7) % spec::FIELD_MODULUS));
        let impl_poly = unlift_poly(&spec_poly);
        let recovered = lift_poly(&impl_poly);
        assert_eq!(spec_poly, recovered);
    }

    #[test]
    fn lift_zero_is_zero() {
        let zero = PolynomialRingElement::<PortableVector>::ZERO();
        let lifted = lift_poly(&zero);
        for c in lifted.iter() {
            assert_eq!(c.val, 0);
        }
    }

    #[test]
    fn add_to_ring_element_matches_spec() {
        let spec_a: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 7) % spec::FIELD_MODULUS));
        let spec_b: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 13 + 100) % spec::FIELD_MODULUS));

        // Spec addition
        let spec_sum = hacspec_ml_kem::polynomial::add_to_ring_element(&spec_a, &spec_b);

        // Impl addition
        let mut impl_a = unlift_poly(&spec_a);
        let impl_b = unlift_poly(&spec_b);
        impl_a.add_to_ring_element(&impl_b, 0);

        assert_eq!(lift_poly(&impl_a), spec_sum);
    }

    #[test]
    fn barrett_reduce_matches_spec() {
        let spec_p: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 17 + 500) % spec::FIELD_MODULUS));

        let spec_reduced = hacspec_ml_kem::polynomial::poly_barrett_reduce(&spec_p);

        let mut impl_p = unlift_poly(&spec_p);
        impl_p.poly_barrett_reduce();

        assert_eq!(lift_poly(&impl_p), spec_reduced);
    }
}
