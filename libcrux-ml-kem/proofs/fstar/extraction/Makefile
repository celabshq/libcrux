SLOW_MODULES += Libcrux_ml_kem.Vector.Rej_sample_table.fsti
SLOW_MODULES += Libcrux_ml_kem.Vector.Portable.Serialize.fst

# G3 module-trust mirror (annotation_lint V5 / trust_ledger --check): every module
# verified-on-cadence (SLOW) carries a machine-readable reason here. The .fst/.fsti
# themselves are gitignored/extracted, so the Makefile is the git-tracked authority
# for this trust surface. Reasons use the shared category vocabulary (reason_ok).
# trusted-module: Libcrux_ml_kem.Vector.Rej_sample_table.fsti : slow-proof: large rejection-sample shuffle-table VCs, verified on cadence
# trusted-module: Libcrux_ml_kem.Vector.Portable.Serialize.fst : slow-proof: portable serialize bit-manipulation VCs, verified on cadence

ADMIT_MODULES =
# ml-kem verifies with an EMPTY ADMIT_MODULES: every commute, trait-boundary
# and incremental module that was once admitted is now verified. Two modules
# remain SLOW (verified on cadence, declared at the top of this file).
#
# Two build facts outlive that history:
#  - hax.py deletes the .Alloc and .Incremental.Rand submodules post-extraction;
#    they use `Box<dyn Keys>` and `try_fill_bytes`, which have no F* models.
#  - Function-scoped proof gaps remain as `admit()` in the RUST source (not as
#    admitted modules): the 2 From-instance bodies and to_bytes_compressed,
#    where hax forces core-trait preconditions trivial and serialize_vector
#    lacks an exact-length contract. See proofs/ml_kem_verification_status.md
#    "Findings".

# NOTE: the shared crates/utils/intrinsics extraction dir is intentionally NOT
# on the include path — ml-kem carries its own Libcrux_intrinsics.* copy in the
# sibling `../intrinsics` dir (extracted via --output-dir under ml-kem's
# pre_core_models config) to avoid the cross-crate flip-flop / stale-.fsti
# contamination. `../intrinsics` is a dedicated dir (NOT `../extraction`) so
# FINDLIBS never auto-includes it on sibling crates' paths. See
# fstar-helpers/Makefile.generic.
FSTAR_INCLUDE_DIRS_EXTRA += ../spec \
	      ../intrinsics \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      ../commute \
	      $(shell git rev-parse --show-toplevel)/crates/algorithms/sha3/proofs/fstar/equivalence \
	      $(shell git rev-parse --show-toplevel)/crates/algorithms/sha3/proofs/fstar/spec

# Hash wiring: with `hash_functions::portable` extracted transparently, ml-kem's
# Hash_functions.Portable depends on libcrux_sha3's transparent portable Keccak
# `.fst` bodies (Libcrux_sha3.Portable + Generic_keccak), reached via the
# auto-discovered sha3 `extraction` dir, plus the hand-written equivalence
# helpers (EquivImplSpec) under sha3's `equivalence` dir which is not
# auto-discovered.  (sha3 formerly carried a stub `Spec.Utils` in a `stubs` dir
# that had to be kept off this include path, since it would shadow ml-kem's real
# `../spec/Spec.Utils` and break the `Spec.Utils.v_*` references.  That stub fell
# out of sha3's dependency closure and has been deleted, so the hazard is gone.)

ENABLE_HINTS = --use_hints --record_hints

# Per-algorithm F* cache + hints, isolated from other algorithms. Each
# algorithm extracts the shared dep crates (core-models/secrets/intrinsics)
# under its OWN config (e.g. ml-kem uses --cfg pre_core_models), so their
# .checked and .hints must NOT share one dir in a single working tree or
# they clobber each other. Relative to this extraction dir => <crate>/proofs/fstar/{cache,hints}.
CACHE_DIR := ../cache
HINT_DIR  := ../hints

# Verify the hand-written companions as first-class roots, not merely as
# incidental dependencies of the extracted modules: `../spec` holds the Hacspec
# specs and proof companions, `../commute` the impl<->spec commute theory. Under
# Makefile.generic's default (extraction dir only) a companion that no extracted
# module happens to cite is never verified at all, and one carrying a .fsti has
# only its interface checked while the .fst proof body goes unchecked. Mirrors
# the sha3 extraction Makefile. Set before the include — Makefile.generic uses
# `ROOTS ?=`, so this assignment wins.
# NOTE: after changing this list, `rm .depend` before building; make will not
# re-derive the dependency graph on its own when roots are ADDED, because the
# existing .depend is already newer than the newly-rooted files.
ROOTS = $(wildcard *.fst *.fsti) \
        $(wildcard ../spec/*.fst ../spec/*.fsti) \
        $(wildcard ../commute/*.fst ../commute/*.fsti)

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
