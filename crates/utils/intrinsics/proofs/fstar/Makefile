# Dedicated F* verification for the hand-written ARM64 NEON SHA3-extension
# fallback correctness proofs.
#
# These proof modules live under fstar-helpers/fstar-bitvec/ and prove that the
# four REAL fallback bodies in crates/utils/intrinsics/src/arm64_extract.rs
#   _veor3q_u64 / _vbcaxq_u64 / _vrax1q_u64 / _vxarq_u64
# satisfy their #[hax_lib::ensures] specs, using only the basic NEON op specs
# from the extracted Libcrux_intrinsics.Arm64_extract.fsti (plus the one
# auditable u64 rotate identity in Bitvec.U64Rotate).
#
# This deliberately does NOT root Libcrux_intrinsics.Arm64_extract.fst /
# Avx2_extract.fst: those mix the proven fallbacks with unimplemented!() model
# ops whose `.fst` bodies cannot discharge (they remain trusted as `.fsti`
# axioms, as in every other consumer crate). Isolating the fallback proofs here
# lets CI actually prove them without dragging in the model-op axioms.
#
# Prerequisite: the intrinsics + core-models crates must be hax-extracted first
# (the intrinsics-hax CI workflow does this) so their proofs/fstar/extraction
# dirs exist and FINDLIBS can discover them.

FSTAR_INCLUDE_DIRS_EXTRA += \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell pwd)/extraction

# The proof modules are hand-written and live in fstar-helpers/fstar-bitvec/,
# not in this directory; teach make where to find the ROOTS source files.
vpath %.fst $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec

ROOTS = \
	Bitvec.U64Rotate.fst \
	Bitvec.VxarqProof.fst \
	Bitvec.Sha3FallbackProof.fst

FSTAR_EXT_FLAGS = --ext context_pruning

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
