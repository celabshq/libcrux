//! Polynomial operations matching the implementation's PolynomialRingElement interface.
//!
//! In the spec, a Polynomial is just `[FieldElement; 256]`. These functions provide
//! named operations that correspond to methods on the implementation's
//! `PolynomialRingElement<Vector>` type, enabling function-by-function verification.

use crate::ntt::multiply_ntts;
use crate::parameters::*;

/// The zero polynomial. Corresponds to `PolynomialRingElement::ZERO()` in the implementation.
pub const fn poly_zero() -> Polynomial {
    [FieldElement::new(0); 256]
}

/// Add `rhs` into `self` in-place. Corresponds to `PolynomialRingElement::add_to_ring_element()`.
///
/// Note: In the spec we return a new polynomial; in the implementation this is `&mut self`.
/// The mathematical operation is identical.
pub fn add_to_ring_element(lhs: &Polynomial, rhs: &Polynomial) -> Polynomial {
    createi(|j| {
        FieldElement::new(((lhs[j].val as u32 + rhs[j].val as u32) % FIELD_MODULUS as u32) as u16)
    })
}

/// Barrett reduction of all coefficients. Corresponds to `PolynomialRingElement::poly_barrett_reduce()`.
///
/// In the spec, this is a no-op modular reduction since we always work with exact arithmetic.
/// In the implementation, this is needed because intermediate values may exceed the field modulus.
pub fn poly_barrett_reduce(p: &Polynomial) -> Polynomial {
    createi(|i| FieldElement::new(p[i].val % FIELD_MODULUS))
}

/// Subtract `b` from `a` and reduce. Corresponds to `PolynomialRingElement::subtract_reduce()`.
pub fn subtract_reduce(a: &Polynomial, b: &Polynomial) -> Polynomial {
    createi(|j| {
        FieldElement::new(
            ((a[j].val as u32 + FIELD_MODULUS as u32 - b[j].val as u32) % FIELD_MODULUS as u32)
                as u16,
        )
    })
}

/// NTT-domain polynomial multiplication. Corresponds to `PolynomialRingElement::ntt_multiply()`.
///
/// Given two polynomials in NTT form, returns their product in NTT form.
pub fn ntt_multiply(a: &Polynomial, b: &Polynomial) -> Polynomial {
    multiply_ntts(a, b)
}

/// Fused add(error, message, result). Corresponds to `PolynomialRingElement::add_message_error_reduce()`.
///
/// Computes: self + message + result, where:
/// - `self` is error_2
/// - `message` is the decompressed message
/// - `result` is NTT⁻¹(tᵀ ◦ r̂)
///
/// In the implementation, this fuses the addition to avoid extra temporaries.
pub fn add_message_error_reduce(
    error_2: &Polynomial,
    message: &Polynomial,
    ntt_product: &Polynomial,
) -> Polynomial {
    createi(|j| {
        FieldElement::new(
            ((error_2[j].val as u32 + message[j].val as u32 + ntt_product[j].val as u32)
                % FIELD_MODULUS as u32) as u16,
        )
    })
}

/// Fused add(self, error) with reduction. Corresponds to `PolynomialRingElement::add_error_reduce()`.
///
/// Used to compute u = NTT⁻¹(Aᵀ ◦ r̂) + e₁
pub fn add_error_reduce(ntt_product: &Polynomial, error: &Polynomial) -> Polynomial {
    createi(|j| {
        FieldElement::new(
            ((ntt_product[j].val as u32 + error[j].val as u32) % FIELD_MODULUS as u32) as u16,
        )
    })
}

/// Fused add(self, error) for NTT-domain values. Corresponds to `PolynomialRingElement::add_standard_error_reduce()`.
///
/// Used to compute t̂ = Â◦ŝ + ê (both operands are in NTT domain).
pub fn add_standard_error_reduce(ntt_product: &Polynomial, error_ntt: &Polynomial) -> Polynomial {
    createi(|j| {
        FieldElement::new(
            ((ntt_product[j].val as u32 + error_ntt[j].val as u32) % FIELD_MODULUS as u32) as u16,
        )
    })
}
