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
# X4-parallel matrix sampling dispatchers (4)
# =====================================================================
# Requires trait-method panic-freedom on the underlying X4 Xof hash
# functions.  Tracked as Session D in `proofs/post-merge-handoff.md`.
ADMIT_MODULES = \
	Libcrux_ml_dsa.Samplex4.fst \
	Libcrux_ml_dsa.Samplex4.Avx2.fst \
	Libcrux_ml_dsa.Samplex4.Neon.fst \
	Libcrux_ml_dsa.Samplex4.Portable.fst

# =====================================================================
# AVX2 rejection sample shuffle table + samplers (3)
# =====================================================================
# Shuffle_table is dead-code "executable spec" for the const
# `SHUFFLE_TABLE`; its `is_bit_set` does `1 << bit_position` which
# requires `v bit_position < bits USIZE` (opaque in the proof-libs
# .fsti) — fixable but not trivial.  Samplers are admit-only
# pending bit-vec body proofs (similar shape to Step 13 Track A AVX2
# closures).  Tracked as Session C in
# `proofs/post-merge-handoff.md`.
ADMIT_MODULES += \
	Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Shuffle_table.fst

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
