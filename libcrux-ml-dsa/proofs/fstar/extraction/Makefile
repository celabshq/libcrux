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
# Trust boundary is declared IN SOURCE — no Makefile admits.
# =====================================================================
# - Samplex4.fst: the free fns `matrix_flat` / `sample_s1_and_s2` in
#   `src/samplex4.rs` carry `#[hax_lib::fstar::verification_status(lax)]`
#   (bodies need X4 Xof trait-method panic-freedom, not yet proven) —
#   the single isolated trust axiom for the X4 sampler.
# - Samplex4.{Avx2,Neon,Portable}: the per-platform `X4Sampler` impls
#   restate the trait `requires`/`ensures` (under `#[hax_lib::attributes]`)
#   and forward to the lax free `matrix_flat`, so their bodies VERIFY
#   (the instance post is the strong post, discharged from the exported
#   lax post — no admit needed).
# - Simd.Avx2.Rejection_sample.Shuffle_table: dead-code executable-spec
#   fns are `#[hax_lib::exclude]`d, leaving only the verified
#   `SHUFFLE_TABLE` constant — admit ELIMINATED.
ADMIT_MODULES =

# NOTE: the shared crates/utils/intrinsics extraction dir is intentionally NOT
# on the include path — ml-dsa carries its own Libcrux_intrinsics.* copy
# (extracted via --output-dir into THIS dir) to avoid the cross-crate flip-flop
# / stale-.fsti contamination. See fstar-helpers/Makefile.generic.
FSTAR_INCLUDE_DIRS_EXTRA += ../spec \
	      ../intrinsics \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell git rev-parse --show-toplevel)/libcrux-ml-kem/proofs/fstar/spec \
	      $(shell git rev-parse --show-toplevel)/specs/ml-dsa/proofs/fstar/commute

ENABLE_HINTS = --use_hints --record_hints

# Per-algorithm F* cache + hints, isolated from other algorithms. Each
# algorithm extracts the shared dep crates (core-models/secrets/intrinsics)
# under its OWN config (e.g. ml-kem uses --cfg pre_core_models), so their
# .checked and .hints must NOT share one dir in a single working tree or
# they clobber each other. Relative to this extraction dir => <crate>/proofs/fstar/{cache,hints}.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

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
        $(CACHE_DIR)/Spec.MLDSA.Math.fst.checked \
        $(CACHE_DIR)/Spec.Intrinsics.fst.checked
