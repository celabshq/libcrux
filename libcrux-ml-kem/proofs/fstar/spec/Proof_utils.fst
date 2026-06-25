module Proof_utils

open FStar.Mul
open Core_models

/// Specs for Rust core primitives that the pinned hax-lib currently leaves
/// uninterpreted.  These are trusted primitive axioms (the modeled-primitive
/// trust base), kept in one place and INTENDED TO BE UPSTREAMED to hax-lib;
/// once the corresponding primitive carries the spec there, the axiom and its
/// call sites here can be deleted.

/// Spec of `i16::abs` (`Rust_primitives.Arithmetic.abs_i16`), guarded against
/// `i16::MIN` (where `.abs()` overflows / is out of range).  Upstream target:
/// `Rust_primitives.Arithmetic`.  Consumer: ml-kem
/// `Polynomial.multiply_by_constant_bounded`.
assume
val lemma_abs_i16 (c: i16)
    : Lemma (requires v c > -32768)
            (ensures (if v c >= 0
                      then Rust_primitives.Arithmetic.abs_i16 c == c
                      else v (Rust_primitives.Arithmetic.abs_i16 c) == - (v c)))
