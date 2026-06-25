use crate::arithmetic::mod_q;
use crate::createi;
use crate::ntt::{intt, ntt};
/// Polynomial and vector operations over R_q — FIPS 204.
///
/// Polynomials are [i32; 256] with coefficients in [0, q-1].
/// Vectors are arrays of polynomials: [Polynomial; K] or [Polynomial; L].
use crate::parameters::{Polynomial, Q};

/// Coefficient-wise addition of two polynomials.
pub(crate) fn poly_add(a: &Polynomial, b: &Polynomial) -> Polynomial {
    createi(|i| mod_q(a[i] as i64 + b[i] as i64))
}

/// Coefficient-wise subtraction of two polynomials.
pub(crate) fn poly_sub(a: &Polynomial, b: &Polynomial) -> Polynomial {
    createi(|i| mod_q(a[i] as i64 - b[i] as i64))
}

/// Coefficient-wise multiplication in NTT domain (Hadamard product).
pub(crate) fn poly_pointwise_mul(a: &Polynomial, b: &Polynomial) -> Polynomial {
    createi(|i| mod_q(a[i] as i64 * b[i] as i64))
}

/// Apply NTT to each polynomial in a vector.
pub(crate) fn vector_ntt<const N: usize>(v: &[Polynomial; N]) -> [Polynomial; N] {
    createi(|i| ntt(v[i]))
}

/// Apply INTT to each polynomial in a vector.
pub(crate) fn vector_intt<const N: usize>(v: &[Polynomial; N]) -> [Polynomial; N] {
    createi(|i| intt(v[i]))
}

/// Coefficient-wise addition of two vectors.
pub(crate) fn vector_add<const N: usize>(
    a: &[Polynomial; N],
    b: &[Polynomial; N],
) -> [Polynomial; N] {
    createi(|i| poly_add(&a[i], &b[i]))
}

/// Coefficient-wise subtraction of two vectors.
pub(crate) fn vector_sub<const N: usize>(
    a: &[Polynomial; N],
    b: &[Polynomial; N],
) -> [Polynomial; N] {
    createi(|i| poly_sub(&a[i], &b[i]))
}

/// Multiply a polynomial (in NTT domain) with each element of a vector (in NTT domain).
pub(crate) fn scalar_vector_ntt<const N: usize>(
    c: &Polynomial,
    v: &[Polynomial; N],
) -> [Polynomial; N] {
    createi(|i| poly_pointwise_mul(c, &v[i]))
}

/// Infinity norm of a polynomial: max |coeff| where coefficients are centered mod q.
pub(crate) fn poly_infinity_norm(p: &Polynomial) -> i32 {
    let mut max = 0i32;
    for i in 0..256 {
        let c = crate::arithmetic::coeff_norm(p[i]);
        if c > max {
            max = c;
        }
    }
    max
}

/// Infinity norm of a vector: max over all polynomials.
pub(crate) fn vector_infinity_norm<const N: usize>(v: &[Polynomial; N]) -> i32 {
    let mut max = 0i32;
    for i in 0..N {
        let n = poly_infinity_norm(&v[i]);
        if n > max {
            max = n;
        }
    }
    max
}

/// Apply Power2Round componentwise to a vector, returning (v1, v0).
#[hax_lib::requires(fstar!(r#"forall i. i < Seq.length ${v} ==> (forall j. j < 256 ==> Rust_primitives.Integers.v (Seq.index (Seq.index ${v} i) j) >= 0 /\ Rust_primitives.Integers.v (Seq.index (Seq.index ${v} i) j) < Rust_primitives.Integers.v ${Q})"#))]
pub(crate) fn vector_power2round<const N: usize>(
    v: &[Polynomial; N],
) -> ([Polynomial; N], [Polynomial; N]) {
    let v1: [Polynomial; N] = createi(|i| {
        createi(|j| {
            let (r1, _) = crate::arithmetic::power2round(v[i][j]);
            r1
        })
    });
    let v0: [Polynomial; N] = createi(|i| {
        createi(|j| {
            let (_, r0) = crate::arithmetic::power2round(v[i][j]);
            r0
        })
    });
    (v1, v0)
}

/// Apply HighBits componentwise to a vector.
#[hax_lib::requires(fstar!(r#"${gamma2} >. mk_i32 0 /\ ${gamma2} <. (${Q} /! mk_i32 2) /\ (forall i. i < Seq.length ${v} ==> (forall j. j < 256 ==> Rust_primitives.Integers.v (Seq.index (Seq.index ${v} i) j) >= 0 /\ Rust_primitives.Integers.v (Seq.index (Seq.index ${v} i) j) < Rust_primitives.Integers.v ${Q}))"#))]
pub(crate) fn vector_high_bits<const N: usize>(
    v: &[Polynomial; N],
    gamma2: i32,
) -> [Polynomial; N] {
    createi(|i| createi(|j| crate::arithmetic::high_bits(v[i][j], gamma2)))
}

/// Apply LowBits componentwise to a vector.
#[hax_lib::requires(fstar!(r#"${gamma2} >. mk_i32 0 /\ ${gamma2} <. (${Q} /! mk_i32 2) /\ (forall i. i < Seq.length ${v} ==> (forall j. j < 256 ==> Rust_primitives.Integers.v (Seq.index (Seq.index ${v} i) j) >= 0 /\ Rust_primitives.Integers.v (Seq.index (Seq.index ${v} i) j) < Rust_primitives.Integers.v ${Q}))"#))]
pub(crate) fn vector_low_bits<const N: usize>(v: &[Polynomial; N], gamma2: i32) -> [Polynomial; N] {
    createi(|i| createi(|j| crate::arithmetic::low_bits(v[i][j], gamma2)))
}

/// Count the number of `true` entries in a hint vector.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v v_N <= 8"#))]
pub(crate) fn count_hints<const N: usize>(h: &[[bool; 256]; N]) -> usize {
    let mut total = 0usize;
    for i in 0usize..N {
        hax_lib::loop_invariant!(|i: usize| total <= i * 256);
        for j in 0usize..256usize {
            hax_lib::loop_invariant!(|j: usize| total <= i * 256 + j);
            if h[i][j] {
                total += 1;
            }
        }
    }
    total
}

/// Apply MakeHint componentwise to two vectors, returning hint vector and count of ones.
#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v v_N <= 8 /\ ${gamma2} >. mk_i32 0 /\ ${gamma2} <. (${Q} /! mk_i32 2) /\ (forall i. i < Seq.length ${r} ==> (forall j. j < 256 ==> Rust_primitives.Integers.v (Seq.index (Seq.index ${r} i) j) >= 0 /\ Rust_primitives.Integers.v (Seq.index (Seq.index ${r} i) j) < Rust_primitives.Integers.v ${Q}))"#))]
pub(crate) fn vector_make_hint<const N: usize>(
    z: &[Polynomial; N],
    r: &[Polynomial; N],
    gamma2: i32,
) -> ([[bool; 256]; N], usize) {
    let h: [[bool; 256]; N] =
        createi(|i| createi(|j| crate::arithmetic::make_hint(z[i][j], r[i][j], gamma2)));
    let count = count_hints(&h);
    (h, count)
}

/// Apply UseHint componentwise to a hint vector and a polynomial vector.
#[hax_lib::requires(fstar!(r#"${gamma2} >. mk_i32 0 /\ ${gamma2} <. (${Q} /! mk_i32 2) /\ (forall i. i < Seq.length ${r} ==> (forall j. j < 256 ==> Rust_primitives.Integers.v (Seq.index (Seq.index ${r} i) j) >= 0 /\ Rust_primitives.Integers.v (Seq.index (Seq.index ${r} i) j) < Rust_primitives.Integers.v ${Q}))"#))]
pub(crate) fn vector_use_hint<const N: usize>(
    h: &[[bool; 256]; N],
    r: &[Polynomial; N],
    gamma2: i32,
) -> [Polynomial; N] {
    createi(|i| createi(|j| crate::arithmetic::use_hint(h[i][j], r[i][j], gamma2)))
}

/// Reduce each coefficient to centered representative in [-(q-1)/2, (q-1)/2].
#[hax_lib::fstar::options("--z3rlimit 150")]
pub(crate) fn poly_mod_pm(p: &Polynomial) -> Polynomial {
    createi(|i| {
        let r = ((p[i] as i64 % Q as i64) + Q as i64) % Q as i64;
        let r = r as i32;
        if r > Q / 2 {
            r - Q
        } else {
            r
        }
    })
}
