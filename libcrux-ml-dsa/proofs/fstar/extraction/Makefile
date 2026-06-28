SLOW_MODULES +=

# ML-DSA verification scope (post-merge of `ml-dsa-above-trait` into
# `ml-dsa-proofs`).  This Makefile lists modules that are ADMITTED
# (default-allowlist), mirroring `libcrux-ml-kem`.  Every other .fst
# in this directory is verified by full SMT.  Every newly-extracted
# file therefore enters the verified set automatically — drop a line
# below to admit it explicitly when you cannot prove it yet, and
# delete the line as soon as it verifies.
#
# Goal: drive this list to zero.

# =====================================================================
# User-facing per-parameter-set API wrappers — ALL un-laxed (2026-06-01)
# =====================================================================
# Public entry points (`generate_key_pair`, `sign`, `verify`) for
# ml_dsa_44/65/87.  All 12 modules now verify (panic-free tier): the
# pre-hashed path required a length-preservation postcondition threaded
# through multiplexing -> instantiations -> generic (admitted core).

# =====================================================================
# Trust boundary: as much as possible is now declared IN SOURCE.
# =====================================================================
# MOVED TO SOURCE (no longer admitted here):
# - Samplex4.fst: the free fns `matrix_flat` / `sample_s1_and_s2` in
#   `src/samplex4.rs` carry `#[hax_lib::fstar::verification_status(lax)]`
#   (bodies need X4 Xof trait-method panic-freedom, not yet proven).
# - Simd.Avx2.Rejection_sample.Shuffle_table: the dead-code executable-
#   spec fns are `#[hax_lib::exclude]`d in `shuffle_table.rs`, leaving
#   only the verified `SHUFFLE_TABLE` constant — admit ELIMINATED.
#
# CANNOT move to source (hax 0.3.7 limitation): the per-platform
# `X4Sampler` impls below fail the trait-instance subtyping check
# (`Samplex4.{Avx2,Portable,Neon}.fst:43 f_matrix_flat`, Error 19), not
# a body VC.  `verification_status(lax)` is illegal on impl methods
# (expands to an unnamed `const _`, rustc rejects) and `panic_free`
# only admits the ensures, not this instance check.  Until hax gains a
# module-/impl-level admit attribute they stay admitted here.
ADMIT_MODULES = \
	Libcrux_ml_dsa.Samplex4.Avx2.fst \
	Libcrux_ml_dsa.Samplex4.Neon.fst \
	Libcrux_ml_dsa.Samplex4.Portable.fst

FSTAR_INCLUDE_DIRS_EXTRA += ../spec \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell git rev-parse --show-toplevel)/libcrux-ml-kem/proofs/fstar/spec \
	      $(shell git rev-parse --show-toplevel)/libcrux-intrinsics/proofs/fstar/extraction \
	      $(shell git rev-parse --show-toplevel)/specs/ml-dsa/proofs/fstar/commute

ENABLE_HINTS = --use_hints --record_hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base

# =====================================================================
# Hand-written / shared modules that carry a .fsti, so consumers depend
# only on their .fsti.checked — `make verify` would NOT verify the .fst
# proof bodies. Force-gate the .fst modules here so CI re-checks them.
#
#  - Avx2NttTheory / Avx2NttCompose: committed AVX2 forward-NTT commute
#    theory under specs/ml-dsa/proofs/fstar/commute/ (on the include path
#    above). (Avx2NttCompose has no .fsti; listing it is explicit/harmless.)
#  - Hacspec_ml_dsa.Commute.Chunk (commute/) and Spec.MLDSA.Math (../spec):
#    shared per-lane commute + integer-spec libraries. Each gained a hand-
#    written .fsti so closing an admit in their .fst body no longer reproves
#    the heavy NTT consumers; their .fst proofs must be force-gated here.
# =====================================================================
verify: $(CACHE_DIR)/Avx2NttTheory.fst.checked $(CACHE_DIR)/Avx2NttCompose.fst.checked \
        $(CACHE_DIR)/Hacspec_ml_dsa.Commute.Chunk.fst.checked \
        $(CACHE_DIR)/Spec.MLDSA.Math.fst.checked
