# As in ../extraction/Makefile, sha3's own Libcrux_intrinsics.* copy lives in
# ../intrinsics; the shared crates/utils/intrinsics tree is off the include path.
FSTAR_INCLUDE_DIRS_EXTRA += \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell pwd)/../intrinsics \
	      $(shell pwd)/../spec \
	      $(shell pwd)/../extraction

ROOTS = \
	Proof_Utils.Lemmas.fst \
	Proof_Utils.NatFold.fst \
	Proof_Utils.FoldRange.fst \
	EquivImplSpec.Keccakf.ChiFold.fst \
	EquivImplSpec.Keccakf.Generic.fst \
	EquivImplSpec.Keccakf.Portable.fst \
	EquivImplSpec.Keccakf.Arm64.fst \
	EquivImplSpec.Keccakf.Avx2.fst \
	EquivImplSpec.Sponge.Generic.Core.fst \
	EquivImplSpec.Sponge.Generic.Squeeze.fst \
	EquivImplSpec.Sponge.Portable.fst \
	EquivImplSpec.Sponge.Arm64.fst \
	EquivImplSpec.Sponge.Avx2.fst \
	EquivImplSpec.Sponge.Portable.Steps.fst \
	EquivImplSpec.Sponge.Arm64.Steps.fst \
	EquivImplSpec.Sponge.Arm64.OneStep.fst \
	EquivImplSpec.Sponge.Avx2.Steps.fst \
	EquivImplSpec.Sponge.Avx2.OneStepByte.fst \
	EquivImplSpec.Sponge.Avx2.OneStep.fst \
	Hacspec_sha3.Sponge.Lemmas.fst \
	EquivImplSpec.Sponge.Portable.SqueezeAPI.fst \
	EquivImplSpec.Correctness.Portable.fst \
	EquivImplSpec.Correctness.Neon.fst \
	EquivImplSpec.Correctness.Avx2.fst \
	EquivImplSpec.Keccakf.SpecRounds.fst

# Same per-algorithm cache/hints as ../extraction/Makefile.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

# Disable context_pruning for proof modules — it strips definitions
# the normalizer needs for assert_norm and fold_range reduction.
FSTAR_EXT_FLAGS = --ext context_pruning

ENABLE_HINTS = --use_hints --record_hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
