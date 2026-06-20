pub(crate) use crate::vector::{
    Operations, PolynomialRingElement, FIELD_MODULUS, MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS,
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
    #[cfg_attr(
        hax,
        hax_lib::fstar::after(
            r#"
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
"#
        )
    )]
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
            $result /\
          Libcrux_ml_kem.Vector.Traits.Spec.multiply_by_constant_post
            (Libcrux_ml_kem.Vector.Traits.f_repr $vec) $c
            (Libcrux_ml_kem.Vector.Traits.f_repr $result)"#))]
pub(crate) fn multiply_by_constant_bounded<Vector: Operations>(
    vec: Vector,
    _b: usize,
    c: i16,
) -> Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    // `i16::abs` (Rust_primitives.Arithmetic.abs_i16) is left uninterpreted by the
    // pinned hax-lib, so its spec is the single trusted primitive axiom
    // `Proof_utils.lemma_abs_i16` (a MIN-guarded i16-abs spec, to be upstreamed to
    // hax-lib; once abs_i16 carries it there this call and the axiom can be dropped).
    hax_lib::fstar!(r#"Proof_utils.lemma_abs_i16 c"#);
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
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
    Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
      (Seq.index ${result}.Libcrux_ml_kem.Vector.f_coefficients i)
    == Seq.slice $a (16 * i) (16 * i + 16)"#))]
#[hax_lib::fstar::options("--z3rlimit 200")]
fn from_i16_array<Vector: Operations>(a: &[i16]) -> PolynomialRingElement<Vector> {
    let mut result = ZERO();
    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"forall (j: nat). j < v $i ==>
            Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${result}.Libcrux_ml_kem.Vector.f_coefficients j)
            == Seq.slice $a (16 * j) (16 * j + 16)"#
        ));
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

/// Runtime check that every lane of `vec` lies in
/// `[-(FIELD_MODULUS - 1), FIELD_MODULUS - 1]` = `[-3328, 3328]`.
///
/// Used to validate raw-decoded (16-bit) serialization inputs before they
/// flow into field arithmetic.
#[inline(always)]
#[allow(dead_code)]
#[hax_lib::ensures(|result| hax_lib::implies(result, spec::is_bounded_vector(3328, vec)))]
fn vector_within_field_bound<Vector: Operations>(vec: &Vector) -> bool {
    let arr = Vector::to_i16_array(*vec);
    // Ground per-lane conjunction (no loop) so the per-lane facts are
    // directly available to fold into the opaque bound atom below.
    let ok = arr[0] > -FIELD_MODULUS
        && arr[0] < FIELD_MODULUS
        && arr[1] > -FIELD_MODULUS
        && arr[1] < FIELD_MODULUS
        && arr[2] > -FIELD_MODULUS
        && arr[2] < FIELD_MODULUS
        && arr[3] > -FIELD_MODULUS
        && arr[3] < FIELD_MODULUS
        && arr[4] > -FIELD_MODULUS
        && arr[4] < FIELD_MODULUS
        && arr[5] > -FIELD_MODULUS
        && arr[5] < FIELD_MODULUS
        && arr[6] > -FIELD_MODULUS
        && arr[6] < FIELD_MODULUS
        && arr[7] > -FIELD_MODULUS
        && arr[7] < FIELD_MODULUS
        && arr[8] > -FIELD_MODULUS
        && arr[8] < FIELD_MODULUS
        && arr[9] > -FIELD_MODULUS
        && arr[9] < FIELD_MODULUS
        && arr[10] > -FIELD_MODULUS
        && arr[10] < FIELD_MODULUS
        && arr[11] > -FIELD_MODULUS
        && arr[11] < FIELD_MODULUS
        && arr[12] > -FIELD_MODULUS
        && arr[12] < FIELD_MODULUS
        && arr[13] > -FIELD_MODULUS
        && arr[13] < FIELD_MODULUS
        && arr[14] > -FIELD_MODULUS
        && arr[14] < FIELD_MODULUS
        && arr[15] > -FIELD_MODULUS
        && arr[15] < FIELD_MODULUS;
    // Fold the 16 ground lane bounds into the opaque
    // `is_i16b_array_opaque` atom behind `spec::is_bounded_vector`.
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
            (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    ok
}

/// Runtime check that every coefficient of `re` lies in
/// `[-(FIELD_MODULUS - 1), FIELD_MODULUS - 1]`.
#[inline(always)]
#[allow(dead_code)]
#[hax_lib::ensures(|result| hax_lib::implies(result, spec::is_bounded_poly(3328, re)))]
pub(crate) fn poly_within_field_bound<Vector: Operations>(
    re: &PolynomialRingElement<Vector>,
) -> bool {
    let mut ok = true;
    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| {
            hax_lib::implies(
                ok,
                hax_lib::forall(|ii: usize| {
                    hax_lib::implies(
                        ii < 16 && ii < i,
                        spec::is_bounded_vector(3328, &re.coefficients[ii]),
                    )
                }),
            )
        });
        ok = ok && vector_within_field_bound(&re.coefficients[i]);
    }
    ok
}

/// Runtime check that every coefficient of every ring element in `v` lies
/// in `[-(FIELD_MODULUS - 1), FIELD_MODULUS - 1]`.
#[inline(always)]
#[allow(dead_code)]
#[hax_lib::ensures(|result|
    hax_lib::implies(result, spec::is_bounded_polynomial_vector(3328, v)))]
pub(crate) fn polyvec_within_field_bound<const N: usize, Vector: Operations>(
    v: &[PolynomialRingElement<Vector>; N],
) -> bool {
    let mut ok = true;
    for i in 0..N {
        hax_lib::loop_invariant!(|i: usize| {
            hax_lib::implies(
                ok,
                hax_lib::forall(|ii: usize| {
                    hax_lib::implies(ii < N && ii < i, spec::is_bounded_poly(3328, &v[ii]))
                }),
            )
        });
        ok = ok && poly_within_field_bound(&v[i]);
    }
    // Fold the per-element bounds into the opaque
    // `is_bounded_polynomial_vector` atom.
    #[cfg(hax)]
    if ok {
        spec::lemma_is_bounded_polynomial_vector_intro(v, 3328);
    }
    ok
}

/// Runtime check that every coefficient of every ring element in `m` lies
/// in `[-(FIELD_MODULUS - 1), FIELD_MODULUS - 1]`.
#[inline(always)]
#[allow(dead_code)]
#[hax_lib::ensures(|result|
    hax_lib::implies(result, spec::is_bounded_polynomial_matrix(3328, m)))]
pub(crate) fn matrix_within_field_bound<const N: usize, Vector: Operations>(
    m: &[[PolynomialRingElement<Vector>; N]; N],
) -> bool {
    let mut ok = true;
    for i in 0..N {
        hax_lib::loop_invariant!(|i: usize| {
            hax_lib::implies(
                ok,
                hax_lib::forall(|ii: usize| {
                    hax_lib::implies(
                        ii < N && ii < i,
                        spec::is_bounded_polynomial_vector(3328, &m[ii]),
                    )
                }),
            )
        });
        ok = ok && polyvec_within_field_bound(&m[i]);
    }
    // Fold the per-row atoms into the opaque
    // `is_bounded_polynomial_matrix` atom.
    #[cfg(hax)]
    if ok {
        spec::lemma_is_bounded_polynomial_matrix_intro(m, 3328);
    }
    ok
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
    hax_lib::fstar!(
        r#"
        Hacspec_ml_kem.Commute.Chunk.lemma_to_spec_poly_mont_unfold #v_Vector $b
      "#
    );

    for i in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant uses an OPAQUE per-vector predicate
        // (subtract_reduce_finalize_chunk) for already-processed chunks,
        // and an array-equality marker for unprocessed chunks (so the
        // body's trait posts of mont_mul talk about _b[i]).
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &b_acc.coefficients[j])
                        & fstar!(
                            r#"
                            Hacspec_ml_kem.Commute.Chunk.subtract_reduce_finalize_chunk
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${myself}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${b_acc}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${_b} (v $j)))
                          "#
                        )
                } else {
                    fstar!(
                        r#"
                        Seq.index ${b_acc}.f_coefficients (v $j) == Seq.index ${_b} (v $j)
                      "#
                    )
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
        hax_lib::fstar!(
            r#"
            Hacspec_ml_kem.Commute.Chunk.lemma_subtract_reduce_iter
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${myself}.f_coefficients (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${_b} (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${coefficient_normal_form})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${diff})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${red})
          "#
        );

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
    hax_lib::fstar!(
        r#"
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
      "#
    );

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
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3328, &myself) & (spec::is_bounded_poly(3328, &message)))]
#[hax_lib::ensures(|output|
    spec::is_bounded_poly(3328, &output)
    & fstar!(r#"
        Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${output}
          == Hacspec_ml_kem.Polynomial.add_message_error_reduce
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${myself})
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${message})
               (Hacspec_ml_kem.Parameters.createi
                  #Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
                  #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                  (fun (j: usize {j <. mk_usize 256}) ->
                    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${result}) (v j))
                      Hacspec_ml_kem.Commute.Chunk.fe_1441))
      "#))]
fn add_message_error_reduce<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    message: &PolynomialRingElement<Vector>,
    result: PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    #[cfg(hax)]
    let _result = result.coefficients;

    // Keep the parameter `result` UNSHADOWED so the post-loop bridge can name
    // the INPUT `result` poly (the mont-scaled operand).  The mutable loop
    // accumulator goes through a freshly-named local `result_acc`.  Mirror of
    // subtract_reduce keeping the immutable `b` nameable via `b_acc`.
    let mut result_acc: PolynomialRingElement<Vector> = result;

    // Seed F1 (scoped to the parameter `result`, snapshotted as `_result`):
    //   to_spec_poly_mont (param result) == to_spec_poly_mont_arr _result.
    // The post-loop bridge then chains via `lemma_add_message_error_reduce_scaled_eq`
    // on the parameter `result` and the constructed `result_input` (sharing
    // `f_coefficients == _result`).
    hax_lib::fstar!(
        r#"
        Hacspec_ml_kem.Commute.Chunk.lemma_to_spec_poly_mont_unfold #v_Vector $result
      "#
    );

    for i in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant uses an OPAQUE per-chunk finalize atom
        // (add_message_error_reduce_finalize_chunk) for already-processed
        // chunks keyed on the PLAIN `myself`/`message` chunks and the
        // ORIGINAL (snapshot) `_result` chunk (the mont-scaled operand);
        // an array-equality marker for unprocessed chunks (so the body's
        // trait posts of mont_mul talk about _result[i]).
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &result_acc.coefficients[j])
                        & fstar!(
                            r#"
                            Hacspec_ml_kem.Commute.Chunk.add_message_error_reduce_finalize_chunk
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${myself}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${message}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${result_acc}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${_result} (v $j)))
                          "#
                        )
                } else {
                    fstar!(
                        r#"
                        Seq.index ${result_acc}.f_coefficients (v $j) == Seq.index ${_result} (v $j)
                      "#
                    )
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
            Vector::montgomery_multiply_by_constant(result_acc.coefficients[i], 1441);

        let sum1 = add_bounded(myself.coefficients[i], 3328, &message.coefficients[i], 3328);
        hax_lib::assert_prop!(spec::is_bounded_vector(6656, &sum1));

        let sum2 = add_bounded(coefficient_normal_form, 3328, &sum1, 6656);

        hax_lib::assert_prop!(spec::is_bounded_vector(9984, &sum2));
        #[cfg(hax)]
        spec::is_bounded_vector_higher(&sum2, 9984, 28296);

        let red = Vector::barrett_reduce(sum2);
        hax_lib::assert_prop!(spec::is_bounded_vector(3328, &red));

        // Encapsulated per-iteration helper: takes the trait posts of
        // mont_mul, add (×2), barrett at chunk i and produces the opaque
        // chunk-level finalize predicate.
        hax_lib::fstar!(
            r#"
            Hacspec_ml_kem.Commute.Chunk.lemma_add_message_error_reduce_iter
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${myself}.f_coefficients (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${message}.f_coefficients (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${_result} (v $i)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${coefficient_normal_form})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${sum1})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${sum2})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${red})
          "#
        );

        result_acc.coefficients[i] = red;
    }

    // Post-loop bridge: lift the 16 per-chunk opaque finalize predicates
    // (from the loop invariant at i = 16) to the polynomial-level equation
    // citing HP.add_message_error_reduce.  The commute lemma produces the
    // helper-form; the eq-helper lemma chains it to HP.add_message_error_reduce.
    // The scaled-eq lemma bridges the lemma's `result_input` (sharing
    // `f_coefficients == _result`) to the parameter `result` referenced by the
    // post.
    hax_lib::fstar!(
        r#"
        let result_input : Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
          { Libcrux_ml_kem.Vector.f_coefficients = ${_result} } in
        Hacspec_ml_kem.Commute.Chunk.lemma_add_message_error_reduce_commute
          #v_Vector ${myself} ${message} result_input result_acc;
        let myself_lift = Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector ${myself} in
        let message_lift = Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector ${message} in
        let scaled_result = Hacspec_ml_kem.Parameters.createi
                         #Hacspec_ml_kem.Parameters.t_FieldElement
                         (mk_usize 256)
                         #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                         (fun (j: usize {j <. mk_usize 256}) ->
                           Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                             (Seq.index
                                (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector result_input)
                                (v j))
                             Hacspec_ml_kem.Commute.Chunk.fe_1441) in
        Hacspec_ml_kem.Commute.Chunk.lemma_add_message_error_reduce_eq_helper
          myself_lift message_lift scaled_result;
        Hacspec_ml_kem.Commute.Chunk.lemma_add_message_error_reduce_scaled_eq #v_Vector $result result_input
      "#
    );

    result_acc
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
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(7, &error))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &future(myself))
    & fstar!(r#"
        Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${myself}_future
          == Hacspec_ml_kem.Polynomial.add_error_reduce
               (Hacspec_ml_kem.Parameters.createi
                  #Hacspec_ml_kem.Parameters.t_FieldElement
                  (mk_usize 256)
                  #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                  (fun (j: usize {j <. mk_usize 256}) ->
                    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Seq.index
                         (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${myself}) (v j))
                      Hacspec_ml_kem.Commute.Chunk.fe_1441))
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${error})
      "#))]
fn add_error_reduce<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    error: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;
    // Snapshot the ORIGINAL `myself` ring element under a name that survives
    // the loop's rebinding of `myself` (hax desugars the `&mut` mutation into
    // `let myself = fold_range ... myself ...`, shadowing the input).  The
    // post references the INPUT `myself`; the post-loop bridge needs to name
    // it, so we keep `myself_orig` (= input value).  Mirror of subtract_reduce
    // keeping the immutable `b` nameable via a fresh `b_acc` accumulator.
    #[cfg(hax)]
    let myself_orig = *myself;

    // Seed F1 (scoped to the ORIGINAL `myself`, snapshotted as `_myself`):
    //   to_spec_poly_mont (orig myself) == to_spec_poly_mont_arr _myself.
    // The post-loop bridge chains via `lemma_add_error_reduce_scaled_eq` on
    // the original `myself_orig` and the constructed `myself_input` (both share
    // `f_coefficients == _myself`).  Mirror of subtract_reduce's seed.
    hax_lib::fstar!(
        r#"
        Hacspec_ml_kem.Commute.Chunk.lemma_to_spec_poly_mont_unfold #v_Vector ${myself}
      "#
    );

    for j in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant: for already-processed chunks, the OPAQUE per-chunk
        // finalize atom keyed on the ORIGINAL (snapshot) `_myself` chunk (the
        // mont-scaled operand) and the PLAIN `error` chunk; for unprocessed
        // chunks, the chunk is still equal to the original.  Mirror of
        // subtract_reduce's invariant (SUB → ADD, swapped operand roles).
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &myself.coefficients[j])
                        & fstar!(
                            r#"
                            Hacspec_ml_kem.Commute.Chunk.add_error_reduce_finalize_chunk
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${_myself} (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${myself}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${error}.f_coefficients (v $j)))
                          "#
                        )
                } else {
                    fstar!(
                        r#"
                        Seq.index ${myself}.f_coefficients (v $j) == Seq.index ${_myself} (v $j)
                      "#
                    )
                }
            } else {
                true.to_prop()
            }
        }));

        hax_lib::fstar!(
            r#"
          assert (v $j < 16);
          assert_norm (1441 < pow2 15);
          assert_norm (1664 < pow2 15);
          assert_norm (mk_i16 1441 <. mk_i16 1664);
          assert(Spec.Utils.is_i16b 1664 (mk_i16 1441))
        "#
        );

        let coefficient_normal_form =
            Vector::montgomery_multiply_by_constant(myself.coefficients[j], 1441);

        let sum = add_bounded(coefficient_normal_form, 3328, &error.coefficients[j], 7);

        hax_lib::assert_prop!(spec::is_bounded_vector(3335, &sum));
        #[cfg(hax)]
        spec::is_bounded_vector_higher(&sum, 3335, 28296);

        let red = Vector::barrett_reduce(sum);
        hax_lib::assert_prop!(spec::is_bounded_vector(3328, &red));

        // Encapsulated per-iteration helper: takes the trait posts of
        // mont_mul, add, barrett at chunk j and produces the opaque
        // chunk-level finalize predicate (mirror of lemma_subtract_reduce_iter).
        hax_lib::fstar!(
            r#"
            Hacspec_ml_kem.Commute.Chunk.lemma_add_error_reduce_iter
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${_myself} (v $j)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index ${error}.f_coefficients (v $j)))
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${coefficient_normal_form})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${sum})
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector ${red})
          "#
        );

        myself.coefficients[j] = red;
    }

    // Post-loop bridge: lift the 16 per-chunk opaque finalize predicates
    // (from the loop invariant at j = 16) to the polynomial-level equation
    // citing HP.add_error_reduce.  The commute lemma produces the helper-
    // form `add_error_reduce_helper`; the eq-helper lemma chains it to
    // HP.add_error_reduce.  The scaled-eq lemma bridges the lemma's
    // `myself_input` (sharing `f_coefficients == _myself`) to the original
    // `myself` referenced by the post.  Mirror of subtract_reduce's bridge.
    hax_lib::fstar!(
        r#"
        let myself_input : Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
          { Libcrux_ml_kem.Vector.f_coefficients = ${_myself} } in
        Hacspec_ml_kem.Commute.Chunk.lemma_add_error_reduce_commute
          #v_Vector myself_input ${error} ${myself};
        let error_lift = Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector ${error} in
        let scaled_myself = Hacspec_ml_kem.Parameters.createi
                         #Hacspec_ml_kem.Parameters.t_FieldElement
                         (mk_usize 256)
                         #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                         (fun (j: usize {j <. mk_usize 256}) ->
                           Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                             (Seq.index
                                (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector myself_input)
                                (v j))
                             Hacspec_ml_kem.Commute.Chunk.fe_1441) in
        Hacspec_ml_kem.Commute.Chunk.lemma_add_error_reduce_eq_helper scaled_myself error_lift;
        Hacspec_ml_kem.Commute.Chunk.lemma_add_error_reduce_scaled_eq #v_Vector ${myself_orig} myself_input
      "#
    );
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
// Standard-domain lifts emitted into the INTERFACE (.fsti) so the strengthened
// `ensures` below can reference them, WITHOUT touching the hand-maintained
// `Hacspec_ml_kem.Commute.Chunk` module (editing Chunk invalidates its heavy
// NTT lane-bridge replay hints and forces a cold pass that saturates).
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        interface,
        r#"
(* Standard-domain per-lane lift: interprets an i16 NTT-domain coefficient
   `m` (in `·R⁻¹` form) as the plain abstract value `α` it represents,
   `α ≡ v m * 2285 ≡ v m * R (mod q)` — exactly the value
   `to_standard_domain` (= `mont_mul(_, R²)`) recovers.  Mirrors
   `mont_i16_to_spec_fe` (which scales by `169 = R⁻¹`) with `2285 = R`. *)
let std_i16_to_spec_fe (x: i16)
    : Prims.Pure Hacspec_ml_kem.Parameters.t_FieldElement
      Prims.l_True
      (ensures
        fun r ->
          let r:Hacspec_ml_kem.Parameters.t_FieldElement = r in
          v r.Hacspec_ml_kem.Parameters.f_val == (v x * 2285) % 3329) =
  let (q: i32):i32 = mk_i32 3329 in
  let r:u16 =
    cast (((((cast (x <: i16) <: i32) *! mk_i32 2285 <: i32) %! q <: i32) +! q <: i32) %! q <: i32)
    <:
    u16
  in
  Hacspec_ml_kem.Parameters.impl_FieldElement__new r

(* Poly-level standard-domain lift: `to_standard_domain` applied lane-wise
   and read as the plain spec polynomial. *)
let to_spec_poly_standard
    (#v_Vector: Type0)
    {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
    (p: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
  = Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
      #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
      (fun (j: usize { j <. mk_usize 256 }) ->
        (std_i16_to_spec_fe
          (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                       (Seq.index p.Libcrux_ml_kem.Vector.f_coefficients (v j / 16)))
                     (v j % 16))
         <: Hacspec_ml_kem.Parameters.t_FieldElement))
"#
    )
)]
#[hax_lib::fstar::before(
    r#"
(* Opaque per-chunk "done" atom: the lane-wise plain FE-add equation for a
   processed chunk, with `red_chunk` the result lane, `myself_chunk` the
   ORIGINAL (pre-to_standard_domain) lane, `error_chunk` the error lane.
   Kept opaque so the loop invariant carries it as ONE atomic term per
   chunk instead of an unfolded `forall l`, preventing per-iteration Z3
   re-instantiation (mirrors subtract_reduce's `subtract_reduce_finalize_chunk`). *)
[@@ "opaque_to_smt"]
let add_std_chunk_done
    (myself_chunk error_chunk red_chunk: t_Array i16 (mk_usize 16)) : prop =
  forall (l: nat). l < 16 ==>
    Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index red_chunk l)
      == Hacspec_ml_kem.Parameters.impl_FieldElement__add
           (std_i16_to_spec_fe (Seq.index myself_chunk l))
           (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index error_chunk l))

let lemma_add_std_chunk_done_elim
    (myself_chunk error_chunk red_chunk: t_Array i16 (mk_usize 16)) (l: nat) :
    Lemma (requires add_std_chunk_done myself_chunk error_chunk red_chunk /\ l < 16)
          (ensures
            Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index red_chunk l)
              == Hacspec_ml_kem.Parameters.impl_FieldElement__add
                   (std_i16_to_spec_fe (Seq.index myself_chunk l))
                   (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index error_chunk l)))
  = reveal_opaque (`%add_std_chunk_done)
      (add_std_chunk_done myself_chunk error_chunk red_chunk)

(* Per-iteration bridge for `add_standard_error_reduce`: from the three
   trait chunk-posts (to_standard_domain = mont_mul(_, 1353), add, barrett)
   at lane `l` of chunk `i`, conclude the lane-wise plain spec FE-add
   equation against the standard-domain lift of the ORIGINAL `myself`
   chunk.  A clean-context `forall_intro` over the proven per-lane bridge
   `lemma_add_standard_error_reduce_lane`; the `plain` value is fixed to
   `std_i16_to_spec_fe (myself_chunk.[l])` so it matches
   `to_spec_poly_standard` lane-for-lane.  Produces the OPAQUE chunk atom. *)
#push-options "--z3rlimit 300 --split_queries always --fuel 0 --ifuel 1"
let lemma_add_std_err_iter
    (#vV: Type0) {| iop: Libcrux_ml_kem.Vector.Traits.t_Operations vV |}
    (myself_chunk normal_chunk error_chunk sum_chunk red_chunk: t_Array i16 (mk_usize 16)) :
    Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_by_constant_post
          myself_chunk (mk_i16 1353) normal_chunk /\
        Libcrux_ml_kem.Vector.Traits.Spec.add_post normal_chunk error_chunk sum_chunk /\
        Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_post sum_chunk red_chunk)
      (ensures add_std_chunk_done myself_chunk error_chunk red_chunk)
  = let open Libcrux_ml_kem.Vector.Traits.Spec in
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post)
      (Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post);
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post)
      (Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post);
    let aux (l: nat) : Lemma (l < 16 ==>
        i16_to_spec_fe (Seq.index red_chunk l)
          == Hacspec_ml_kem.Parameters.impl_FieldElement__add
               (std_i16_to_spec_fe (Seq.index myself_chunk l))
               (i16_to_spec_fe (Seq.index error_chunk l)))
      = if l < 16 then begin
          let plain = std_i16_to_spec_fe (Seq.index myself_chunk l) in
          (* mont_form_lane (myself.[l]) plain : (v myself * 2285) % q == v plain.f_val % q. *)
          reveal_opaque (`%Hacspec_ml_kem.Commute.Chunk.mont_form_lane)
            (Hacspec_ml_kem.Commute.Chunk.mont_form_lane (Seq.index myself_chunk l) plain);
          assert (v plain.Hacspec_ml_kem.Parameters.f_val
                  == (v (Seq.index myself_chunk l) * 2285) % 3329);
          (* unfold the mod_q_eq trait posts into raw `% 3329`. *)
          Hacspec_ml_kem.ModQ.lemma_mod_q_eq_unfold
            (v (Seq.index normal_chunk l))
            (v (Seq.index myself_chunk l) * v (mk_i16 1353) * 169);
          Hacspec_ml_kem.ModQ.lemma_mod_q_eq_unfold
            (v (Seq.index red_chunk l))
            (v (Seq.index sum_chunk l));
          Hacspec_ml_kem.Commute.Chunk.lemma_add_standard_error_reduce_lane
            (Seq.index myself_chunk l)
            (Seq.index normal_chunk l)
            (Seq.index error_chunk l)
            (Seq.index sum_chunk l)
            (Seq.index red_chunk l)
            plain
        end
    in
    Classical.forall_intro aux;
    reveal_opaque (`%add_std_chunk_done)
      (add_std_chunk_done myself_chunk error_chunk red_chunk)

(* Standalone clean-context bridge: the `ntt_product` slice at chunk `k`,
   lane `l`, equals the standard-domain lift of the original `myself`
   coefficient lane.  Isolates the createi unfold from the loop body. *)
let lemma_ntt_product_slice
    (#vV: Type0) {| iop: Libcrux_ml_kem.Vector.Traits.t_Operations vV |}
    (p: Libcrux_ml_kem.Vector.t_PolynomialRingElement vV) (k l: nat) :
    Lemma (requires k < 16 /\ l < 16)
          (ensures
            Seq.index (Seq.slice (to_spec_poly_standard #vV p) (k * 16) (k * 16 + 16)) l
              == std_i16_to_spec_fe
                   (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                                (Seq.index p.Libcrux_ml_kem.Vector.f_coefficients k)) l))
  = let np = to_spec_poly_standard #vV p in
    Seq.lemma_index_slice np (k * 16) (k * 16 + 16) l;
    Hacspec_ml_kem.Parameters.createi_lemma #Hacspec_ml_kem.Parameters.t_FieldElement
      (mk_usize 256)
      #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
      (fun (jj: usize { jj <. mk_usize 256 }) ->
        (std_i16_to_spec_fe
          (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                       (Seq.index p.Libcrux_ml_kem.Vector.f_coefficients (v jj / 16)))
                     (v jj % 16))
         <: Hacspec_ml_kem.Parameters.t_FieldElement))
      (sz (k * 16 + l))
#pop-options
"#
)]
#[hax_lib::requires(spec::is_bounded_poly(3328, &error))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &future(myself))
    & fstar!(r#"
        Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${myself}_future
          == Hacspec_ml_kem.Polynomial.add_standard_error_reduce
               (to_spec_poly_standard #$:Vector ${myself})
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${error})
      "#))]
fn add_standard_error_reduce<Vector: Operations>(
    myself: &mut PolynomialRingElement<Vector>,
    error: &PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _myself = myself.coefficients;

    for j in 0..VECTORS_IN_RING_ELEMENT {
        // Loop invariant: for already-processed chunks, the OPAQUE per-chunk
        // FE-add atom keyed on the ORIGINAL (snapshot) `_myself` chunk; for
        // unprocessed chunks, the chunk is still equal to the original.
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < 16 {
                if j < i {
                    spec::is_bounded_vector(3328, &myself.coefficients[j])
                        & fstar!(
                            r#"
                            add_std_chunk_done
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${_myself} (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${error}.f_coefficients (v $j)))
                              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                                (Seq.index ${myself}.f_coefficients (v $j)))
                          "#
                        )
                } else {
                    fstar!(
                        r#"
                        Seq.index ${myself}.f_coefficients (v $j) == Seq.index ${_myself} (v $j)
                      "#
                    )
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

        // Establish the per-lane plain FE-add equation for this chunk as the
        // OPAQUE atom keyed on the ORIGINAL `_myself` chunk.
        hax_lib::fstar!(
            r#"
            lemma_add_std_err_iter #v_Vector
              (Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index ${_myself} (v $j)))
              (Libcrux_ml_kem.Vector.Traits.f_repr ${coefficient_normal_form})
              (Libcrux_ml_kem.Vector.Traits.f_repr
                (${error}.f_coefficients.[ $j ] <: v_Vector))
              (Libcrux_ml_kem.Vector.Traits.f_repr ${sum})
              (Libcrux_ml_kem.Vector.Traits.f_repr ${red})
          "#
        );

        myself.coefficients[j] = red;
    }

    // Post-loop bridge: from the 16 per-chunk opaque atoms (loop invariant at
    // i = 16) assemble the `_commute` precondition for every (k,l), then call
    // the proven Chunk commute lemma against `myself_orig` (= original).
    hax_lib::fstar!(
        r#"
        let myself_orig : Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
          { Libcrux_ml_kem.Vector.f_coefficients = ${_myself} } in
        let ntt_product : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
          to_spec_poly_standard #v_Vector myself_orig in
        let aux (k: nat) : Lemma (k < 16 ==>
            (forall (l: nat). l < 16 ==>
              Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                             (Seq.index ${myself}.f_coefficients k)) l)
                == Hacspec_ml_kem.Parameters.impl_FieldElement__add
                     (Seq.index (Seq.slice ntt_product (k * 16) (k * 16 + 16)) l)
                     (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                       (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                                    (Seq.index ${error}.f_coefficients k)) l)))) =
          if k < 16 then begin
            let aux2 (l: nat) : Lemma (l < 16 ==>
                Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                  (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                               (Seq.index ${myself}.f_coefficients k)) l)
                  == Hacspec_ml_kem.Parameters.impl_FieldElement__add
                       (Seq.index (Seq.slice ntt_product (k * 16) (k * 16 + 16)) l)
                       (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                         (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                                      (Seq.index ${error}.f_coefficients k)) l))) =
              if l < 16 then begin
                lemma_add_std_chunk_done_elim
                  (Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index ${_myself} k))
                  (Libcrux_ml_kem.Vector.Traits.f_repr
                    (Seq.index ${error}.f_coefficients k))
                  (Libcrux_ml_kem.Vector.Traits.f_repr
                    (Seq.index ${myself}.f_coefficients k))
                  l;
                lemma_ntt_product_slice #v_Vector myself_orig k l
              end
            in
            Classical.forall_intro aux2
          end
        in
        Classical.forall_intro aux;
        Hacspec_ml_kem.Commute.Chunk.lemma_add_standard_error_reduce_commute #v_Vector
          myself_orig ${error} ${myself} ntt_product
      "#
    );
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
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::fstar::before(r#"(* ════════════════════════════════════════════════════════════════════
   Phase B — `ntt_multiply` poly-level Montgomery commute (re-homed from
   Hacspec_ml_kem.Commute.Bridges).  See subtract_reduce for the wiring
   precedent.  The zeta correspondence is the one approved local assume. *)

module N    = Hacspec_ml_kem.Ntt
module P    = Hacspec_ml_kem.Parameters
module T    = Libcrux_ml_kem.Vector.Traits
module TS   = Libcrux_ml_kem.Vector.Traits.Spec
module V    = Libcrux_ml_kem.Vector
module Poly = Hacspec_ml_kem.Polynomial
module CH   = Hacspec_ml_kem.Commute.Chunk

(* APPROVED LOCAL ASSUME (user 2026-06-06): duplicate of the
   runtime-validated axiom in Hacspec_ml_kem.Commute.Bridges; needed here
   because zeta lives in this module so the Bridges copy can't be imported
   (module cycle).  Bumps this module's assume count 1 -> 2. *)
assume val lemma_zeta_eq_vzetas (k: usize)
  : Lemma (requires v k < 128)
          (ensures TS.mont_i16_to_spec_fe (zeta k) == N.v_ZETAS.[ k ])

(* Bridge: is_bounded_vector b vec -> is_i16b_array_opaque (v b) (f_repr vec).
   is_bounded_vector is over f_to_i16_array; the trait law
   f_to_i16_array x == f_repr x (fired by calling f_to_i16_array) bridges. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_is_i16b_repr_of_bounded
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (vec: vV) (b: usize)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #vV b vec)
    (ensures TS.is_i16b_array_opaque (v b) (T.f_repr vec))
  = reveal_opaque (`%TS.is_i16b_array_opaque)
      (TS.is_i16b_array_opaque (v b) (T.f_to_i16_array vec));
    let _ = T.f_to_i16_array vec in
    reveal_opaque (`%TS.is_i16b_array_opaque)
      (TS.is_i16b_array_opaque (v b) (T.f_repr vec))
#pop-options

#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_ntt_multiply_n_256_lane
    (p1 p2: t_Array P.t_FieldElement (mk_usize 256))
    (zs: t_Slice P.t_FieldElement)
    (i: nat {i < 256})
  : Lemma
    (requires
      (Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) <. mk_usize 1024 &&
      ((Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) *! mk_usize 4 <: usize) =.
        mk_usize 256)
    (ensures
      (let result = N.ntt_multiply_n (mk_usize 256) p1 p2 zs in
       let group : nat = i / 4 in
       let zeta = (if i % 4 < 2 then Seq.index zs group
                   else P.impl_FieldElement__neg (Seq.index zs group)) in
       (i % 2 = 0 ==>
         i + 1 < 256 /\
         Seq.index result i ==
           N.base_case_multiply_even (Seq.index p1 i) (Seq.index p1 (i + 1))
                                     (Seq.index p2 i) (Seq.index p2 (i + 1))
                                     zeta) /\
       (i % 2 = 1 ==>
         i >= 1 /\
         Seq.index result i ==
           N.base_case_multiply_odd (Seq.index p1 (i - 1)) (Seq.index p1 i)
                                    (Seq.index p2 (i - 1)) (Seq.index p2 i))))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 256 }) ->
        let group:usize = j /! mk_usize 4 in
        let zeta:P.t_FieldElement =
          if (j %! mk_usize 4 <: usize) <. mk_usize 2
          then Seq.index zs (v group)
          else P.impl_FieldElement__neg (Seq.index zs (v group))
        in
        (if (j %! mk_usize 2 <: usize) =. mk_usize 0
         then
           N.base_case_multiply_even (Seq.index p1 (v j)) (Seq.index p1 (v j + 1))
                                     (Seq.index p2 (v j)) (Seq.index p2 (v j + 1))
                                     zeta
         else
           N.base_case_multiply_odd (Seq.index p1 (v j - 1)) (Seq.index p1 (v j))
                                    (Seq.index p2 (v j - 1)) (Seq.index p2 (v j)))
        <: P.t_FieldElement)
      (sz i)
#pop-options

let zetas_mul_slice : t_Slice P.t_FieldElement =
  N.v_ZETAS.[ { Core_models.Ops.Range.f_start = mk_usize 64;
                Core_models.Ops.Range.f_end   = mk_usize 128 } ]

#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_zetas_mul_slice_len (_:unit)
  : Lemma ((Core_models.Slice.impl__len #P.t_FieldElement zetas_mul_slice <: usize)
             == mk_usize 64 /\
           (Core_models.Slice.impl__len #P.t_FieldElement zetas_mul_slice <: usize) <. mk_usize 1024 /\
           ((Core_models.Slice.impl__len #P.t_FieldElement zetas_mul_slice <: usize) *! mk_usize 4
              <: usize) =. mk_usize 256)
  = assert_norm (Seq.length N.v_ZETAS == 128)

let lemma_zetas_mul_slice_index (round: nat {round < 64})
  : Lemma (Seq.index zetas_mul_slice round == Seq.index N.v_ZETAS (64 + round))
  = assert_norm (Seq.length N.v_ZETAS == 128);
    FStar.Seq.Base.lemma_index_slice N.v_ZETAS 64 128 round
#pop-options

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_chunk_zeta_eq_slice (m: nat {m < 16}) (g: nat {g < 4})
  : Lemma
    (let zs = TS.zetas_4_ (zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 67 +! mk_usize 4 *! sz m)) in
     64 + (4 * m + g) < 128 /\
     Seq.index zs g == Seq.index N.v_ZETAS (64 + (4 * m + g)))
  = let z0 = zeta (mk_usize 64 +! mk_usize 4 *! sz m) in
    let z1 = zeta (mk_usize 65 +! mk_usize 4 *! sz m) in
    let z2 = zeta (mk_usize 66 +! mk_usize 4 *! sz m) in
    let z3 = zeta (mk_usize 67 +! mk_usize 4 *! sz m) in
    assert (4 * m + g < 64);
    CH.zetas_4_lane z0 z1 z2 z3 (sz g);
    assert (v (mk_usize 64 +! mk_usize 4 *! sz m) == 64 + 4 * m);
    assert (v (mk_usize 65 +! mk_usize 4 *! sz m) == 65 + 4 * m);
    assert (v (mk_usize 66 +! mk_usize 4 *! sz m) == 66 + 4 * m);
    assert (v (mk_usize 67 +! mk_usize 4 *! sz m) == 67 + 4 * m);
    assert (v (mk_usize (64 + 4 * m + g)) == 64 + 4 * m + g);
    lemma_zeta_eq_vzetas (mk_usize (64 + 4 * m + g))
#pop-options

(* Clean-context lift helper: isolate the createi-based unfold so its
   createi_lemma SMTPat doesn't cross-pollinate the per-lane bridge. *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_mont_lift_lane
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (x: V.t_PolynomialRingElement vV) (i: nat {i < 256})
  : Lemma
    (Seq.index (CH.to_spec_poly_mont #vV x) i
       == TS.mont_i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index x.V.f_coefficients (i / 16))) (i % 16)))
  = let m = i / 16 in
    let l = i % 16 in
    FStar.Math.Lemmas.lemma_div_mod i 16;
    CH.poly_lane_mont #vV x i;
    CH.mont_array_lane (T.f_repr (Seq.index x.V.f_coefficients m)) (sz l)
#pop-options

(* RETROFIT 2026-06-06: single symbolic-lane worker replacing the 16-way
   per-literal `lemma_poly_lane_l0..l15` + z3refresh split.  Bridges the
   256-level out-lift lane `16*m+l` to the chunk-level `ntt_multiply_n 16`
   equation (the `requires`).  Createi-free: the createi_lemma SMTPat is
   excluded; both lane unfolds are supplied via the proven lane lemmas and
   the modular index facts are handed to Z3 explicitly, so the even/odd and
   group/zeta ITEs collapse without fixing `l` to a literal.  One `l % 2`
   branch picks the even (partner l+1) vs odd (partner l-1) neighbour. *)
#push-options "--z3rlimit 400 --fuel 0 --ifuel 2 --using_facts_from '* -Hacspec_ml_kem.Parameters.createi_lemma'"
let lemma_poly_lane_any
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (myself rhs out: V.t_PolynomialRingElement vV)
    (m: nat {m < 16}) (l: nat {l < 16})
  : Lemma
    (requires
      (let cm_l = T.f_repr (Seq.index myself.V.f_coefficients m) in
       let cm_r = T.f_repr (Seq.index rhs.V.f_coefficients m) in
       let cm_out = T.f_repr (Seq.index out.V.f_coefficients m) in
       TS.mont_i16_to_spec_array (sz 16) cm_out ==
         N.ntt_multiply_n (mk_usize 16)
           (TS.mont_i16_to_spec_array (sz 16) cm_l)
           (TS.mont_i16_to_spec_array (sz 16) cm_r)
           (Rust_primitives.unsize
             (TS.zetas_4_ (zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 67 +! mk_usize 4 *! sz m))))))
    (ensures
      (let p1 = CH.to_spec_poly_mont #vV myself in
       let p2 = CH.to_spec_poly_mont #vV rhs in
       Seq.index (CH.to_spec_poly_mont #vV out) (16 * m + l)
         == Seq.index (N.ntt_multiply_n (mk_usize 256) p1 p2 zetas_mul_slice) (16 * m + l)))
  = let i : nat = 16 * m + l in
    let cm_l = T.f_repr (Seq.index myself.V.f_coefficients m) in
    let cm_r = T.f_repr (Seq.index rhs.V.f_coefficients m) in
    let cm_out = T.f_repr (Seq.index out.V.f_coefficients m) in
    let zsm = TS.zetas_4_ (zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 67 +! mk_usize 4 *! sz m)) in
    let lm = TS.mont_i16_to_spec_array (sz 16) cm_l in
    let rm = TS.mont_i16_to_spec_array (sz 16) cm_r in
    let p1 = CH.to_spec_poly_mont #vV myself in
    let p2 = CH.to_spec_poly_mont #vV rhs in
    (* Explicit modular index facts: i = 16*m + l, 0 <= l,m < 16. *)
    FStar.Math.Lemmas.lemma_div_plus l m 16;           (* i / 16 == l/16 + m == m *)
    FStar.Math.Lemmas.modulo_addition_lemma l 16 m;    (* i % 16 == l % 16 == l   *)
    FStar.Math.Lemmas.lemma_div_plus l (4 * m) 4;      (* i / 4  == l/4 + 4*m     *)
    FStar.Math.Lemmas.modulo_addition_lemma l 4 (4 * m); (* i % 4 == l % 4        *)
    FStar.Math.Lemmas.modulo_addition_lemma l 2 (8 * m); (* i % 2 == l % 2        *)
    assert (i / 16 == m /\ i % 16 == l);
    assert (i / 4 == 4 * m + l / 4 /\ i % 4 == l % 4 /\ i % 2 == l % 2);
    lemma_mont_lift_lane #vV out i;
    CH.lemma_ntt_multiply_n_16_lane lm rm zsm l;
    lemma_zetas_mul_slice_len ();
    lemma_ntt_multiply_n_256_lane p1 p2 zetas_mul_slice i;
    lemma_mont_lift_lane #vV myself i;
    lemma_mont_lift_lane #vV rhs i;
    CH.mont_array_lane cm_out (sz l);
    CH.mont_array_lane cm_l (sz l);
    CH.mont_array_lane cm_r (sz l);
    lemma_chunk_zeta_eq_slice m (l / 4);
    lemma_zetas_mul_slice_index (i / 4);
    (if l % 2 = 0 then begin
       assert (l <= 14);
       FStar.Math.Lemmas.lemma_div_plus (l + 1) m 16;
       FStar.Math.Lemmas.modulo_addition_lemma (l + 1) 16 m;
       assert ((i + 1) / 16 == m /\ (i + 1) % 16 == l + 1);
       lemma_mont_lift_lane #vV myself (i + 1);
       lemma_mont_lift_lane #vV rhs (i + 1);
       CH.mont_array_lane cm_l (sz (l + 1));
       CH.mont_array_lane cm_r (sz (l + 1))
     end
     else begin
       assert (l >= 1);
       FStar.Math.Lemmas.lemma_div_plus (l - 1) m 16;
       FStar.Math.Lemmas.modulo_addition_lemma (l - 1) 16 m;
       assert ((i - 1) / 16 == m /\ (i - 1) % 16 == l - 1);
       lemma_mont_lift_lane #vV myself (i - 1);
       lemma_mont_lift_lane #vV rhs (i - 1);
       CH.mont_array_lane cm_l (sz (l - 1));
       CH.mont_array_lane cm_r (sz (l - 1))
     end)
#pop-options

(* Trivial dispatcher: lane i -> per-l lemma at l = i % 16, m = i / 16. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_ntt_multiply_poly_lane
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (myself rhs out: V.t_PolynomialRingElement vV)
    (i: nat {i < 256})
  : Lemma
    (requires
      (let m : nat = i / 16 in
       let cm_l = T.f_repr (Seq.index myself.V.f_coefficients m) in
       let cm_r = T.f_repr (Seq.index rhs.V.f_coefficients m) in
       let cm_out = T.f_repr (Seq.index out.V.f_coefficients m) in
       TS.mont_i16_to_spec_array (sz 16) cm_out ==
         N.ntt_multiply_n (mk_usize 16)
           (TS.mont_i16_to_spec_array (sz 16) cm_l)
           (TS.mont_i16_to_spec_array (sz 16) cm_r)
           (Rust_primitives.unsize
             (TS.zetas_4_ (zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 67 +! mk_usize 4 *! sz m))))))
    (ensures
      (let p1 = CH.to_spec_poly_mont #vV myself in
       let p2 = CH.to_spec_poly_mont #vV rhs in
       Seq.index (CH.to_spec_poly_mont #vV out) (i)
         == Seq.index (N.ntt_multiply_n (mk_usize 256) p1 p2 zetas_mul_slice) (i)))
  = let m : nat = i / 16 in
    let l : nat = i % 16 in
    FStar.Math.Lemmas.lemma_div_mod i 16;
    assert (i == 16 * m + l /\ l < 16 /\ m < 16);
    lemma_poly_lane_any #vV myself rhs out m l
#pop-options

(* The poly-level inputs to ntt_multiply are 3328-bounded, but the (weakened)
   low-level f_ntt_multiply pre is now is_i16b_array_opaque 4096.  The chunk_done
   predicate is a bare prop (no proof steps) so it can't reveal/bump; expose the
   3328->4096 opaque bump via SMTPat so the predicate's well-formedness (and the
   impl's f_ntt_multiply call) auto-bridge. *)
let lemma_nttmul_opaque_3328_to_4096 (x: t_Slice i16) : Lemma
  (requires TS.is_i16b_array_opaque 3328 x)
  (ensures TS.is_i16b_array_opaque 4096 x)
  [SMTPat (TS.is_i16b_array_opaque 4096 x)]
  = reveal_opaque (`%TS.is_i16b_array_opaque) (TS.is_i16b_array_opaque)

unfold
let ntt_multiply_chunk_done
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (myself rhs out: V.t_PolynomialRingElement vV) (m: nat {m < 16}) : prop =
  TS.is_i16b_array_opaque 3328 (T.f_repr (Seq.index myself.V.f_coefficients m)) /\
  TS.is_i16b_array_opaque 3328 (T.f_repr (Seq.index rhs.V.f_coefficients m)) /\
  Seq.index out.V.f_coefficients m ==
    T.f_ntt_multiply #vV (Seq.index myself.V.f_coefficients m)
                         (Seq.index rhs.V.f_coefficients m)
                         (zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                         (zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                         (zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                         (zeta (mk_usize 67 +! mk_usize 4 *! sz m))

#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_ntt_multiply_chunk_eq
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (myself rhs out: V.t_PolynomialRingElement vV) (m: nat {m < 16})
  : Lemma
    (requires ntt_multiply_chunk_done #vV myself rhs out m)
    (ensures
      (let cm_l = T.f_repr (Seq.index myself.V.f_coefficients m) in
       let cm_r = T.f_repr (Seq.index rhs.V.f_coefficients m) in
       let cm_out = T.f_repr (Seq.index out.V.f_coefficients m) in
       TS.mont_i16_to_spec_array (sz 16) cm_out ==
         N.ntt_multiply_n (mk_usize 16)
           (TS.mont_i16_to_spec_array (sz 16) cm_l)
           (TS.mont_i16_to_spec_array (sz 16) cm_r)
           (Rust_primitives.unsize
             (TS.zetas_4_ (zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                          (zeta (mk_usize 67 +! mk_usize 4 *! sz m))))))
  = let z0 = zeta (mk_usize 64 +! mk_usize 4 *! sz m) in
    let z1 = zeta (mk_usize 65 +! mk_usize 4 *! sz m) in
    let z2 = zeta (mk_usize 66 +! mk_usize 4 *! sz m) in
    let z3 = zeta (mk_usize 67 +! mk_usize 4 *! sz m) in
    let lhs = Seq.index myself.V.f_coefficients m in
    let rhs_v = Seq.index rhs.V.f_coefficients m in
    assert (Spec.Utils.is_i16b 1664 z0 /\ Spec.Utils.is_i16b 1664 z1 /\
            Spec.Utils.is_i16b 1664 z2 /\ Spec.Utils.is_i16b 1664 z3);
    reveal_opaque (`%TS.is_i16b_array_opaque) (TS.is_i16b_array_opaque 3328 (T.f_repr lhs));
    reveal_opaque (`%TS.is_i16b_array_opaque) (TS.is_i16b_array_opaque 3328 (T.f_repr rhs_v));
    reveal_opaque (`%TS.is_i16b_array_opaque) (TS.is_i16b_array_opaque 4096 (T.f_repr lhs));
    reveal_opaque (`%TS.is_i16b_array_opaque) (TS.is_i16b_array_opaque 4096 (T.f_repr rhs_v));
    assert (TS.ntt_multiply_pre (T.f_repr lhs) (T.f_repr rhs_v) z0 z1 z2 z3);
    CH.lemma_ntt_multiply_chunk_commutes #vV lhs rhs_v z0 z1 z2 z3
#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always"
let lemma_ntt_multiply_to_hacspec
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (myself rhs out: V.t_PolynomialRingElement vV)
  : Lemma
    (requires (forall (m: nat). m < 16 ==> ntt_multiply_chunk_done #vV myself rhs out m))
    (ensures
      CH.to_spec_poly_mont #vV out ==
        Poly.ntt_multiply (CH.to_spec_poly_mont #vV myself)
                          (CH.to_spec_poly_mont #vV rhs))
  = let p1 = CH.to_spec_poly_mont #vV myself in
    let p2 = CH.to_spec_poly_mont #vV rhs in
    let target = N.ntt_multiply_n (mk_usize 256) p1 p2 zetas_mul_slice in
    lemma_zetas_mul_slice_len ();
    assert (Poly.ntt_multiply p1 p2 == target)
      by (FStar.Tactics.norm
            [delta_only [`%Poly.ntt_multiply; `%N.multiply_ntts]; iota;
             FStar.NormSteps.zeta; primops];
          FStar.Tactics.trefl ());
    let out_lift = CH.to_spec_poly_mont #vV out in
    let aux (i: nat) : Lemma (i < 256 ==> Seq.index out_lift i == Seq.index target i)
      = if i < 256 then begin
          let m : nat = i / 16 in
          FStar.Math.Lemmas.lemma_div_le i 255 16;
          assert (m < 16);
          lemma_ntt_multiply_chunk_eq #vV myself rhs out m;
          lemma_ntt_multiply_poly_lane #vV myself rhs out i
        end
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro out_lift target
#pop-options
"#)]
#[hax_lib::requires(spec::is_bounded_poly(3328, &myself) & (spec::is_bounded_poly(3328, &rhs)))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, &result)
    & fstar!(r#"
        Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${result}
          == Hacspec_ml_kem.Polynomial.ntt_multiply
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${myself})
               (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${rhs})
      "#))]
fn ntt_multiply<Vector: Operations>(
    myself: &PolynomialRingElement<Vector>,
    rhs: &PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    let mut out = ZERO();

    for i in 0..VECTORS_IN_RING_ELEMENT {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            hax_lib::implies(
                j < i,
                spec::is_bounded_vector(3328, &out.coefficients[j])
                    & fstar!(
                        r#"
                  ntt_multiply_chunk_done #$:Vector
                    ${myself} ${rhs} ${out} (v $j)
                "#
                    ),
            )
        }));

        out.coefficients[i] = Vector::ntt_multiply(
            &myself.coefficients[i],
            &rhs.coefficients[i],
            zeta(64 + 4 * i),
            zeta(64 + 4 * i + 1),
            zeta(64 + 4 * i + 2),
            zeta(64 + 4 * i + 3),
        );

        // Establish ntt_multiply_chunk_done for the just-written chunk i.
        hax_lib::fstar!(
            r#"
            lemma_is_i16b_repr_of_bounded #v_Vector
              (Seq.index ${myself}.f_coefficients (v $i)) (mk_usize 3328);
            lemma_is_i16b_repr_of_bounded #v_Vector
              (Seq.index ${rhs}.f_coefficients (v $i)) (mk_usize 3328)
          "#
        );
    }

    hax_lib::fstar!(
        r#"
        lemma_ntt_multiply_to_hacspec #v_Vector $myself $rhs out
      "#
    );

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
    #[ensures(|result| fstar!(r#"forall (i: nat). i < 16 ==>
        Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
          (Seq.index ${result}.Libcrux_ml_kem.Vector.f_coefficients i)
        == Seq.slice $a (16 * i) (16 * i + 16)"#))]
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

// Placement anchor + functional bridges for the `impl__` method wrappers.
//
// The `impl__`-prefixed method wrappers (`impl__ntt_multiply`, `impl__add_to_ring_element`,
// `impl__add_standard_error_reduce`) extract to one-line dispatchers whose `.fsti` vals carry
// only the panic-freedom bound; the functional `to_spec_poly_*` posts live on the FREE
// functions.  The bridge lemmas in the `fstar::after` blocks below expose the functional post
// at the wrapper level so consumers (e.g. `Matrix.fst`'s `compute_As_plus_e`) can compose
// them; they are proven here because the dispatcher bodies are visible only inside
// `Libcrux_ml_kem.Polynomial`.
//
// This `#[cfg(hax)]`-only function exists purely to ANCHOR those `fstar::after` blocks: by
// referencing `impl__ntt_multiply` (the last `impl__` wrapper in source order), hax is forced
// (def-before-use) to emit this item — and hence the appended lemmas — after all three wrapper
// definitions.  It is fully verified (panic-free): the only obligation is `ntt_multiply`'s
// precondition, supplied by the `requires`.
#[cfg(hax)]
#[hax_lib::requires(spec::is_bounded_poly(3328, &p))]
#[cfg_attr(hax, hax_lib::fstar::after(interface, r#"
val lemma_impl_ntt_multiply_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector (impl__ntt_multiply #v_Vector self rhs)
      == Hacspec_ml_kem.Polynomial.ntt_multiply
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector self)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector rhs))

val lemma_impl_add_to_ring_element_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) (e_b: usize)
  : Lemma
    (requires (e_b <=. (mk_usize 4 *! mk_usize 3328 <: usize)) /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector e_b self /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_to_ring_element #v_Vector self rhs e_b)
      == Hacspec_ml_kem.Polynomial.add_to_ring_element
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector self)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector rhs))

val lemma_impl_add_standard_error_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) error)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_standard_error_reduce #v_Vector self error)
      == Hacspec_ml_kem.Polynomial.add_standard_error_reduce
           (to_spec_poly_standard #v_Vector self)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector error))

val lemma_impl_add_error_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 7) error)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_error_reduce #v_Vector self error)
      == Hacspec_ml_kem.Polynomial.add_error_reduce
           (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256)
              #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
              (fun (j: usize{j <. mk_usize 256}) ->
                Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                  (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector self) (v j))
                  Hacspec_ml_kem.Commute.Chunk.fe_1441))
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector error))

val lemma_impl_subtract_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (myself b: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 4095) myself)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__subtract_reduce #v_Vector myself b)
      == Hacspec_ml_kem.Polynomial.subtract_reduce
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself)
           (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256)
              #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
              (fun (j: usize{j <. mk_usize 256}) ->
                Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                  (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector b) (v j))
                  Hacspec_ml_kem.Commute.Chunk.fe_1441)))

val lemma_impl_add_message_error_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (myself message result: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) message)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_message_error_reduce #v_Vector myself message result)
      == Hacspec_ml_kem.Polynomial.add_message_error_reduce
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector message)
           (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256)
              #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
              (fun (j: usize{j <. mk_usize 256}) ->
                Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                  (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector result) (v j))
                  Hacspec_ml_kem.Commute.Chunk.fe_1441)))

val lemma_impl__poly_barrett_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 28296) self)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__poly_barrett_reduce #v_Vector self)
      == Hacspec_ml_kem.Polynomial.poly_barrett_reduce
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector self))
"#))]
#[cfg_attr(hax, hax_lib::fstar::after(r#"
let lemma_impl_ntt_multiply_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector (impl__ntt_multiply #v_Vector self rhs)
      == Hacspec_ml_kem.Polynomial.ntt_multiply
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector self)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector rhs))
= let _ = ntt_multiply #v_Vector self rhs in ()

let lemma_impl_add_to_ring_element_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) (e_b: usize)
  : Lemma
    (requires (e_b <=. (mk_usize 4 *! mk_usize 3328 <: usize)) /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector e_b self /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_to_ring_element #v_Vector self rhs e_b)
      == Hacspec_ml_kem.Polynomial.add_to_ring_element
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector self)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector rhs))
= let _ = add_to_ring_element #v_Vector self rhs e_b in ()

let lemma_impl_add_standard_error_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) error)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_standard_error_reduce #v_Vector self error)
      == Hacspec_ml_kem.Polynomial.add_standard_error_reduce
           (to_spec_poly_standard #v_Vector self)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector error))
= let _ = add_standard_error_reduce #v_Vector self error in ()

let lemma_impl_add_error_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 7) error)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_error_reduce #v_Vector self error)
      == Hacspec_ml_kem.Polynomial.add_error_reduce
           (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256)
              #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
              (fun (j: usize{j <. mk_usize 256}) ->
                Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                  (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector self) (v j))
                  Hacspec_ml_kem.Commute.Chunk.fe_1441))
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector error))
= let _ = add_error_reduce #v_Vector self error in ()

let lemma_impl_subtract_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (myself b: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 4095) myself)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__subtract_reduce #v_Vector myself b)
      == Hacspec_ml_kem.Polynomial.subtract_reduce
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself)
           (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256)
              #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
              (fun (j: usize{j <. mk_usize 256}) ->
                Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                  (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector b) (v j))
                  Hacspec_ml_kem.Commute.Chunk.fe_1441)))
= let _ = subtract_reduce #v_Vector myself b in ()

let lemma_impl_add_message_error_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (myself message result: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself /\
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) message)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__add_message_error_reduce #v_Vector myself message result)
      == Hacspec_ml_kem.Polynomial.add_message_error_reduce
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself)
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector message)
           (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256)
              #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
              (fun (j: usize{j <. mk_usize 256}) ->
                Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                  (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #v_Vector result) (v j))
                  Hacspec_ml_kem.Commute.Chunk.fe_1441)))
= let _ = add_message_error_reduce #v_Vector myself message result in ()

let lemma_impl__poly_barrett_reduce_spec
    (#v_Vector: Type0)
    (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    (self: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 28296) self)
    (ensures
      Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector (impl__poly_barrett_reduce #v_Vector self)
      == Hacspec_ml_kem.Polynomial.poly_barrett_reduce
           (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector self))
= let _ = poly_barrett_reduce #v_Vector self in ()
"#))]
fn _impl_functional_bridges_anchor<Vector: Operations>(
    p: PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    p.ntt_multiply(&p)
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
