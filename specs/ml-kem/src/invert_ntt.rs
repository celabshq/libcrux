use crate::ntt::ZETAS;
use crate::parameters::*;

const INVERSE_OF_128: FieldElement = FieldElement::new(3303);

/// Use the Gentleman-Sande butterfly to invert, in-place, the NTT representation
/// of a `Polynomial`.
///
/// This function implements <strong>Algorithm 9</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
///
/// ```plaintext
/// Input: array fˆ ∈ ℤ₂₅₆.
/// Output: array f ∈ ℤ₂₅₆.
///
/// f ← fˆ
/// k ← 127
/// for (len ← 2; len ≤ 128; len ← 2·len)
///     for (start ← 0; start < 256; start ← start + 2·len)
///         zeta ← ζ^(BitRev₇(k)) mod q
///         k ← k − 1
///         for (j ← start; j < start + len; j++)
///             t ← f[j]
///             f[j] ← t + f[j + len]
///             f[j + len] ← zeta·(f[j+len] − t)
///         end for
///     end for
/// end for
///
/// f ← f·3303 mod q
/// return f
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
///
/// Gentleman–Sande butterfly: `(a, b, ζ) ↦ (a + b, ζ·(b − a))`.
/// Used in the inverse NTT.
pub fn inv_butterfly(
    zeta: FieldElement,
    a: FieldElement,
    b: FieldElement,
) -> (FieldElement, FieldElement) {
    (a.add(b), zeta.mul(b.sub(a)))
}

/// One layer of the inverse NTT, generic over the array length `N`.
///
/// As in `ntt_layer_n`, `N = 2·groups·len` where `groups = zetas.len()`.
/// Each group spans `2·len` coefficients and uses one zeta for its `len`
/// Gentleman–Sande butterflies.
///
/// This is FIPS 203 Algorithm 9 lines 3-8 applied once at butterfly
/// half-size `len`.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(
    len >= 1 && len < 1024 && zetas.len() < 1024 && zetas.len() * 2 * len == N
)]
pub fn ntt_inverse_layer_n<const N: usize>(
    p: [FieldElement; N],
    len: usize,
    zetas: &[FieldElement],
) -> [FieldElement; N] {
    createi(|i| {
        let group = i / (2 * len);
        let idx = i % (2 * len);
        if idx < len {
            inv_butterfly(zetas[group], p[i], p[i + len]).0
        } else {
            inv_butterfly(zetas[group], p[i - len], p[i]).1
        }
    })
}

/// One layer of the 256-coefficient inverse NTT.
///
/// Follows FIPS 203 Algorithm 9.  Butterfly half-size `len = 2^layer`,
/// groups = `128 / len`, zetas used = `ZETAS[groups .. 2·groups]` reversed
/// (the inverse NTT consumes the zeta table top-down).
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(layer >= 1 && layer <= 7)]
fn ntt_inverse_layer(p: Polynomial, layer: usize) -> Polynomial {
    let len = 1 << layer;
    let groups = 128 / len;
    // ZETAS[groups .. 2·groups] reversed: zetas[round] = ZETAS[2·groups − 1 − round]
    let zetas: [FieldElement; 128] = createi(|round| {
        if round < groups {
            ZETAS[2 * groups - 1 - round]
        } else {
            FieldElement::new(0)
        }
    });
    ntt_inverse_layer_n(p, len, &zetas[0..groups])
}

pub fn reduce_polynomial(p: Polynomial) -> Polynomial {
    createi(|i| p[i].mul(INVERSE_OF_128))
}

/// FIPS 203 Algorithm 9 lines 3-8 only — the seven layers of
/// Gentleman–Sande inverse butterflies, *without* the final
/// `f ← f · 3303 mod q` finalization (the `· 128⁻¹` factor).
///
/// This is the natural intermediate form of the inverse NTT and
/// matches the impl's `invert_ntt_montgomery`, which deliberately
/// omits the `· 3303` finalization because every call site fuses it
/// with the next per-element operation (see `polynomial.rs::subtract_reduce`,
/// `add_error_reduce`, `add_message_error_reduce`'s `mont_mul(b, 1441)`
/// where `1441 ≡ R²/128 mod q`).  Reference:
/// `pq-crystals/kyber/ref/ntt.c` line 106 (the `1441 = mont²/128` comment).
///
/// The fully-finalized FIPS-203 INTT (`ntt_inverse` above) factors as
/// `reduce_polynomial ∘ ntt_inverse_butterflies`.
#[hax_lib::fstar::options("--z3rlimit 150")]
pub fn ntt_inverse_butterflies(p: Polynomial) -> Polynomial {
    let p = ntt_inverse_layer(p, 1);
    let p = ntt_inverse_layer(p, 2);
    let p = ntt_inverse_layer(p, 3);
    let p = ntt_inverse_layer(p, 4);
    let p = ntt_inverse_layer(p, 5);
    let p = ntt_inverse_layer(p, 6);
    let p = ntt_inverse_layer(p, 7);
    p
}

#[hax_lib::fstar::options("--z3rlimit 150")]
pub fn ntt_inverse(p: Polynomial) -> Polynomial {
    reduce_polynomial(ntt_inverse_butterflies(p))
}

/// Inverse NTT applied to each polynomial in a vector.
pub fn vector_inverse_ntt<const RANK: usize>(vector_as_ntt: Vector<RANK>) -> Vector<RANK> {
    createi(|i| ntt_inverse(vector_as_ntt[i]))
}

/// Performs Barrett reduction on all coefficients of a polynomial.
/// This is the spec equivalent of `poly_barrett_reduce` in the implementation.
pub fn poly_barrett_reduce(p: Polynomial) -> Polynomial {
    createi(|i| FieldElement::new(p[i].val % FIELD_MODULUS))
}
