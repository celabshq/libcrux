# The sha3 modules kmac depends on reference hand-written proof helpers in
# sha3's `equivalence` directory, which is not auto-discovered.
FSTAR_INCLUDE_DIRS_EXTRA += \
	$(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	$(shell git rev-parse --show-toplevel)/crates/algorithms/sha3/proofs/fstar/equivalence

# Per-crate F* cache and hints: shared dependency crates are checked under
# each algorithm's own configuration.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
