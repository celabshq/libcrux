module Hacspec_ml_kem.Polynomial
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// The zero polynomial. Corresponds to `PolynomialRingElement::ZERO()` in the implementation.
let poly_zero (_: Prims.unit) : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 0)
      <:
      Hacspec_ml_kem.Parameters.t_FieldElement)
    (mk_usize 256)

/// Add `rhs` into `self` in-place. Corresponds to `PolynomialRingElement::add_to_ring_element()`.
/// Note: In the spec we return a new polynomial; in the implementation this is `&mut self`.
/// The mathematical operation is identical.
let add_to_ring_element (lhs rhs: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (((cast ((lhs.[ j ]
                          <:
                          Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32) +!
                  (cast ((rhs.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

/// Barrett reduction of all coefficients. Corresponds to `PolynomialRingElement::poly_barrett_reduce()`.
/// In the spec, this is a no-op modular reduction since we always work with exact arithmetic.
/// In the implementation, this is needed because intermediate values may exceed the field modulus.
let poly_barrett_reduce (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new ((p.[ i ]
              <:
              Hacspec_ml_kem.Parameters.t_FieldElement)
              .Hacspec_ml_kem.Parameters.f_val %!
            Hacspec_ml_kem.Parameters.v_FIELD_MODULUS
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

/// Subtract `b` from `a` and reduce. Corresponds to `PolynomialRingElement::subtract_reduce()`.
let subtract_reduce (a b: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast ((((cast ((a.[ j ]
                            <:
                            Hacspec_ml_kem.Parameters.t_FieldElement)
                            .Hacspec_ml_kem.Parameters.f_val
                          <:
                          u16)
                      <:
                      u32) +!
                    (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                    <:
                    u32) -!
                  (cast ((b.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

/// NTT-domain polynomial multiplication. Corresponds to `PolynomialRingElement::ntt_multiply()`.
/// Given two polynomials in NTT form, returns their product in NTT form.
let ntt_multiply (a b: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Ntt.multiply_ntts a b

/// Fused add(error, message, result). Corresponds to `PolynomialRingElement::add_message_error_reduce()`.
/// Computes: self + message + result, where:
/// - `self` is error_2
/// - `message` is the decompressed message
/// - `result` is NTT⁻¹(tᵀ ◦ r̂)
/// In the implementation, this fuses the addition to avoid extra temporaries.
let add_message_error_reduce
      (error_2_ message ntt_product:
          t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast ((((cast ((error_2_.[ j ]
                            <:
                            Hacspec_ml_kem.Parameters.t_FieldElement)
                            .Hacspec_ml_kem.Parameters.f_val
                          <:
                          u16)
                      <:
                      u32) +!
                    (cast ((message.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                            .Hacspec_ml_kem.Parameters.f_val
                          <:
                          u16)
                      <:
                      u32)
                    <:
                    u32) +!
                  (cast ((ntt_product.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

/// Fused add(self, error) with reduction. Corresponds to `PolynomialRingElement::add_error_reduce()`.
/// Used to compute u = NTT⁻¹(Aᵀ ◦ r̂) + e₁
let add_error_reduce
      (ntt_product error: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (((cast ((ntt_product.[ j ]
                          <:
                          Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32) +!
                  (cast ((error.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

/// Fused add(self, error) for NTT-domain values. Corresponds to `PolynomialRingElement::add_standard_error_reduce()`.
/// Used to compute t̂ = Â◦ŝ + ê (both operands are in NTT domain).
let add_standard_error_reduce
      (ntt_product error_ntt: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (((cast ((ntt_product.[ j ]
                          <:
                          Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32) +!
                  (cast ((error_ntt.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)
