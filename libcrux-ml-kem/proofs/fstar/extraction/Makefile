SLOW_MODULES += Libcrux_ml_kem.Vector.Rej_sample_table.fsti
SLOW_MODULES += Libcrux_ml_kem.Vector.Portable.Serialize.fst

# G3 module-trust mirror (annotation_lint V5 / trust_ledger --check): every module
# verified-on-cadence (SLOW) carries a machine-readable reason here. The .fst/.fsti
# themselves are gitignored/extracted, so the Makefile is the git-tracked authority
# for this trust surface. Reasons use the shared category vocabulary (reason_ok).
# trusted-module: Libcrux_ml_kem.Vector.Rej_sample_table.fsti : slow-proof: large rejection-sample shuffle-table VCs, verified on cadence
# trusted-module: Libcrux_ml_kem.Vector.Portable.Serialize.fst : slow-proof: portable serialize bit-manipulation VCs, verified on cadence

ADMIT_MODULES =
# ===================================================================
# TEMPORARY — commute-layer lane-bridge cliffs (regression, 2026-08-21)
# ===================================================================
# These commute modules carry the per-lane SIMD `lemma_*_lane_bridge`
# proofs (symbolic i/4, i%4 + SIMD-helper opaque-reveal chains).  They
# were VERIFIED before; the intrinsics/core-models migration (PR #1568,
# d98cbff57) cascaded new .checked digests that invalidated their
# recorded hints, and cold they hit a non-terminating Z3 path (hang
# 67+ min without hitting rlimit).  This is a hint-context REGRESSION,
# not a proof defect — no new trust axiom, the obligations were
# discharged previously.  Admitting these bodies lets the ENTIRE rest
# of the crate verify while the lane bridges are made cold-fast
# separately (per-lane `match i` restructure — multi-session).
#
# REMOVE each entry as its lane bridges are restructured to be robustly
# cold-provable.  The firewall plan adds a hand-written `.fsti` to each
# so consumers depend on the cheap, stable X.fsti.checked instead of the
# hanging X.fst.checked (Bridges.fsti landed 2026-08-21; Bridges.fst is
# therefore no longer a dep of the extraction graph, but stays listed
# for the commute-dir build).
# DROPPED 2026-08-25 — Chunk.fsti firewall landed (3bc8d94f3): consumers (Ntt/Invert_ntt/Polynomial/
#   Vector.*) depend on the stable Chunk.fsti, so dropping Chunk from ADMIT re-proves ONLY Chunk.fst
#   itself (cold-green as-is: --split_queries always on the 6 saturating helper blocks, commit
#   00d3743af; 202KB, heavy but non-saturating).  Ntt's layer_7 cold gap is closed by #restart-solver
#   (d664a1110).  Supersedes the 2026-08-24 CONFIRM-TEST that re-admitted Chunk to confirm this.
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Chunk.fst
# DROPPED 2026-08-24 — cold-verifies fast-stable as-is (helpers already carry split); NOT in the extraction graph (consumers use Bridges.fsti).
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Bridges.fst
# DROPPED 2026-08-22 — cold-hardened (family-i per-coeff two-knob, commit b8bdb4f7e); cold-verifies e2e.
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Ntt_bridge.fst
# DROPPED 2026-08-22 — cold-hardened (family-i per-coeff two-knob, commit cb05479f2); cold-verifies e2e.
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Invert_ntt_bridge.fst
# DROPPED 2026-08-24 — cold-verifies green as-is (no lane bridges; batch-admitted defensively @85c712f3e).
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Invert_scale.fst
# DROPPED 2026-08-24 — cold-verifies as-is, non-saturating (SLOW normalizer-heavy shuffle-table; rides committed hint).
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Rej_table.fst
# DROPPED 2026-08-22 — cold-hardened (family-iii, commits 53b7abddf/b69e4e8c8); cold-verify e2e.
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Serialize_bits.fst
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Serialize_compress.fst
# DROPPED 2026-08-24 — cold-verify green as-is (no lane bridges; batch-admitted defensively @85c712f3e).
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Compute_dot_bridge.fst
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Compute_u_bridge.fst
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Matrix_bilin.fst
# ADMIT_MODULES += Hacspec_ml_kem.Commute.Matrix_bridge.fst
# NOTE: ALL incremental modules are now VERIFIED (removed from ADMIT_MODULES):
# Ind_cca.Incremental{,.Types,.Avx2,.Portable,.Multiplexing} and
# Mlkem{512,768,1024}.Incremental.  The Types/Incremental closure (2026-06-03)
# hangs off plain-Rust requires/ensures on the byte-layout helpers
# (write/vec_len_bytes/vec_to_bytes len-preservation), KeyPair/EncapsState
# methods, and an is_rank/ETA/size-relation requires-cascade through
# {generic, platform, multiplexing, mlkem.rs concrete} layers.
# Function-scoped PROOF GAPS remain (admit() in the Rust source, see
# proofs/ml_kem_verification_status.md "Findings"): the 2 From-instance
# bodies + to_bytes_compressed (hax core-trait pres are forced trivial /
# serialize_vector exact-length contract).  encapsulate2_serialized +
# decapsulate_incremental_key are now FULLY VERIFIED via validate-on-decode
# (coefficient bounds checked in EncapsState::try_from_bytes /
# KeyPair::from_bytes, Error::InvalidInput on failure).
#
# The .Alloc and .Incremental.Rand submodules are deleted post-extraction by
# hax.py (they use `Box<dyn Keys>` and `try_fill_bytes` which lack F* models).
#
# Neon sprint 1 (2026-06-04): Vector_type.{fst,fsti} + Arithmetic.fst
# removed from ADMIT_MODULES.  The historical "Error 162"
# (decidable-equality, Phase 6d) no longer reproduces under hax 0.3.7 —
# the un-admitted Vector_type.fsti verifies clean; no masking needed.
# Remaining Neon entries: Serialize (Sprint 2 — vec128 BitVec bridges),
# Ntt + Compress + Vector.Neon.{fst,fsti} (Sprint 3 — branch-post opacity,
# vqdmulh models, impl-Operations contract pass).

# =====================================================================
# Trait-boundary admits — TEMPORARY, scope = trait_post-strengthening work
# =====================================================================
# These three modules sit ABOVE `vector/traits.rs` in the dep graph.  Their
# proof failures all share the same shape: subtyping checks on a loop
# accumulator's forall-in-refinement get re-proven from scratch each
# iteration, exhausting Z3's rlimit.  This is downstream fallout from the
# trait post strengthening — call sites here haven't been retuned yet.
# Confirmed failing on macOS reference run too, so admitting hides no
# regression.
#
# **REMOVE ALL THREE as soon as the trait-boundary work is done.**  They are
# tracked in `MLKEM_STATUS.md` "Trait-boundary admits" section.  Drop the
# admit in the same commit as the fix so reviewers can re-verify the bound.

# `compress_then_serialize_message` Q2 cancels at rlimit 80 (full 80 in
#   ~18 s on Mac, ~26 s here).  Assert at line 57 about Vector.Traits.fst:587.
# Phase 5: dual-trigger SMTPat `lemma_bounded_i16_array_lookup` in traits::spec
# auto-unfolds the predicate per-index (consume direction).  Intro direction
# (proving `bounded_i16_array` from per-lane bounds) handled by explicit
# `reveal_opaque` at the few call sites that need it.
# ADMIT_MODULES += Libcrux_ml_kem.Serialize.fst

# `Polynomial.ntt_multiply` Q54 cancels at rlimit 300 (full 300 in ~90 s on
#   Mac).  Subtyping on the loop accumulator refinement at line 845.
# REMOVED 2026-04-27: with Phase 2 opacity (per-branch opaque
# ntt_multiply_branch_post), the loop-accumulator forall-in-refinement
# no longer floods the SMT context with FE equations.
# ADMIT_MODULES += Libcrux_ml_kem.Polynomial.fst

# `invert_ntt_at_layer_2_` Q2 "unknown" at rlimit 400 (full 400 in ~86 s).
#   Subtyping on the loop accumulator refinement at line 149 — same forall-
#   in-refinement pattern as Polynomial above.
# REMOVED 2026-04-27: Phase 2 opacity (per-branch opaque
# inv_ntt_layer_2_step_branch_post) eliminates the SMT context flooding.
# ADMIT_MODULES += Libcrux_ml_kem.Invert_ntt.fst

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
