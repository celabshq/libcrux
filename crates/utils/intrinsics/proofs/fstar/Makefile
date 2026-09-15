# Dedicated F* verification for the hand-written ARM64 NEON SHA3-extension
# fallback correctness proofs.
#
# These proof modules live under fstar-helpers/fstar-bitvec/ and prove that the
# four REAL fallback bodies in crates/utils/intrinsics/src/arm64.rs
#   _veor3q_u64 / _vbcaxq_u64 / _vrax1q_u64 / _vxarq_u64
# satisfy their software-fallback specs, using only the basic NEON op per-lane
# view facts in Bitvec.Arm64Views (PROVEN from the core-models
# Libcrux_core_models.Neon_views op-lemma set over the REAL core-models-backed
# Libcrux_intrinsics.Arm64), plus the one auditable u64 rotate identity in
# Bitvec.U64Rotate. There is NO dependency on the retired Arm64_extract bit_vec
# stub.
#
# This does NOT root Libcrux_intrinsics.Arm64.fst / Avx2.fst directly; they are
# picked up as dependencies of the Bitvec.Arm64Views view module via the
# intrinsics extraction dir added below to FSTAR_INCLUDE_DIRS_EXTRA (FINDLIBS
# excludes the shared crates/utils/intrinsics tree).
#
# Prerequisite: the intrinsics + core-models crates must be hax-extracted first
# (the intrinsics-hax CI workflow does this, WITHOUT --cfg pre_core_models so the
# real Arm64 is produced) so their proofs/fstar/extraction dirs exist and
# FINDLIBS can discover them.

FSTAR_INCLUDE_DIRS_EXTRA += \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell pwd)/extraction

# The proof modules are hand-written and live in fstar-helpers/fstar-bitvec/,
# not in this directory; teach make where to find the ROOTS source files.
vpath %.fst $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec

ROOTS = \
	Bitvec.U64Rotate.fst \
	Bitvec.Arm64Views.fst \
	Bitvec.VxarqProof.fst \
	Bitvec.Sha3FallbackProof.fst

FSTAR_EXT_FLAGS = --ext context_pruning

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
