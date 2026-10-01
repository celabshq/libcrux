# sha3's intrinsics modules are extracted into ../intrinsics rather than the
# shared crates/utils/intrinsics extraction directory.
FSTAR_INCLUDE_DIRS_EXTRA += \
	$(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	../intrinsics \
	../spec \
	../equivalence

# Also verify the hand-written companions in ../spec: the default ROOTS globs
# this directory only, and ../spec has no Makefile of its own.
ROOTS = $(wildcard *.fst *.fsti) $(wildcard ../spec/*.fst ../spec/*.fsti)

FSTAR_EXT_FLAGS = --ext context_pruning
ENABLE_HINTS = --use_hints --record_hints

# Per-crate F* cache and hints: shared dependency crates are checked under
# each algorithm's own configuration.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
