module Proof_utils

open FStar.Mul
open Core_models

/// Specs for Rust core primitives, kept in one place.
///
/// These used to be trusted axioms over `Rust_primitives.Arithmetic` primitives
/// that hax-lib left uninterpreted. hax now models `i16::abs` directly, so this
/// is a proof rather than an assumption; `Core_models.Num.Abs_spec` states the
/// same contract against `Rust_primitives.Integers.abs_int`.

/// Spec of `i16::abs`, guarded against `i16::MIN` (where `.abs()` overflows / is
/// out of range).  Consumer: ml-kem `Polynomial.multiply_by_constant_bounded`.
let lemma_abs_i16 (c: i16)
    : Lemma (requires v c > -32768)
            (ensures (if v c >= 0
                      then Core_models.Num.impl_i16__abs c == c
                      else v (Core_models.Num.impl_i16__abs c) == - (v c)))
  = ()
