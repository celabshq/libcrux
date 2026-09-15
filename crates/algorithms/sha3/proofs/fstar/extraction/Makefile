# NOTE: the shared crates/utils/intrinsics extraction dir is intentionally NOT
# on the include path — each algorithm carries its own Libcrux_intrinsics.*
# copy (extracted via --output-dir into THIS dir) to avoid the cross-crate
# flip-flop / stale-.fsti contamination. See fstar-helpers/Makefile.generic.
FSTAR_INCLUDE_DIRS_EXTRA += \
	$(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	../intrinsics \
	../spec \
	../stubs \
	../equivalence

# Verify the hand-written companions in ../spec as well.  The default ROOTS
# globs THIS directory only, and ../spec has no Makefile of its own (unlike
# ml-kem's spec/commute and ml-dsa's spec, which are verified by their own
# invocation), so a companion that no extracted module happens to import would
# otherwise never be checked at all.
ROOTS = $(wildcard *.fst *.fsti) $(wildcard ../spec/*.fst ../spec/*.fsti)
# These are hax-generated modules; they typecheck but carry no proof obligations.

FSTAR_EXT_FLAGS = --ext context_pruning
ENABLE_HINTS = --use_hints --record_hints

# Per-algorithm F* cache + hints, isolated from other algorithms. Each
# algorithm extracts the shared dep crates (core-models/secrets/intrinsics)
# under its OWN config (e.g. ml-kem uses --cfg pre_core_models), so their
# .checked and .hints must NOT share one dir in a single working tree or
# they clobber each other. Relative to this extraction dir => <crate>/proofs/fstar/{cache,hints}.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
