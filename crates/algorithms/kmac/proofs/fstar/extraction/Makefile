# SLOW_MODULES +=
# ADMIT_MODULES =

# kmac's F* proofs build the sha3 dependency modules (portable::incremental is
# extracted transparently — see kmac/hax.py). Those sha3 modules reference the
# hand-written proof-helper modules that live in sha3's `equivalence` dir
# (e.g. Proof_Utils.Lemmas, referenced from Hacspec_sha3.Sponge), plus
# fstar-bitvec and the intrinsics extraction — none of which are auto-discovered.
# Mirror sha3's own Makefile include set (absolute paths, since kmac lives in a
# different directory).
FSTAR_INCLUDE_DIRS_EXTRA += \
	$(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	$(shell git rev-parse --show-toplevel)/libcrux-intrinsics/proofs/fstar/extraction \
	$(shell git rev-parse --show-toplevel)/crates/algorithms/sha3/proofs/fstar/equivalence

# Per-algorithm F* cache + hints, isolated from other algorithms. Each
# algorithm extracts the shared dep crates (core-models/secrets/intrinsics)
# under its OWN config (e.g. ml-kem uses --cfg pre_core_models), so their
# .checked and .hints must NOT share one dir in a single working tree or
# they clobber each other. Relative to this extraction dir => <crate>/proofs/fstar/{cache,hints}.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
