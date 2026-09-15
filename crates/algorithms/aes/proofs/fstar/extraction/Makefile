# aes F* verification.
#
# aes rests on the differentially-tested core-models intrinsics for BOTH
# backends (off `--cfg pre_core_models`): x86 -> `Libcrux_intrinsics.Avx2`,
# ARM -> `Libcrux_intrinsics.Arm64`.  aes carries ZERO hax_lib::ensures/requires
# (extraction / panic-free only), so there are no equivalence proofs here — the
# generated `Libcrux_aes.*` modules just need to typecheck against the intrinsic
# op signatures + their `ensures`.
#
# Each algorithm carries its own Libcrux_intrinsics.* copy (extracted via
# --output-dir into ../intrinsics) to avoid the cross-crate flip-flop; the
# shared crates/utils/intrinsics tree is excluded from the include path in
# Makefile.generic, so add ../intrinsics here.  core-models / secrets / platform
# extraction dirs are auto-discovered by FINDLIBS.
FSTAR_INCLUDE_DIRS_EXTRA += \
	$(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	../intrinsics

# No explicit ROOTS — default picks up all .fst/.fsti in this directory.

FSTAR_EXT_FLAGS = --ext context_pruning
ENABLE_HINTS = --use_hints --record_hints

# Per-algorithm F* cache + hints, isolated from other algorithms. Each
# algorithm extracts the shared dep crates (core-models/secrets/intrinsics)
# under its OWN config, so their .checked and .hints must NOT share one dir in a
# single working tree or they clobber each other. Relative to this extraction
# dir => aes/proofs/fstar/{cache,hints}.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
