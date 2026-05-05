# Sprint 2026-05-09 — Portable+Avx2+impl-layer lax cleanup — live status

Tip on entry: `1c94eae53`. Branch: `libcrux-ml-kem-proofs`.

## Stage tracker

- Stage 1 (Generic/serialize _5/_11): DONE — 4/4 sites flipped, commit 05dd2bd05.
- Stage 2 (Avx2 vector serialize+ntt + Portable ntt step): DEFERRED.
  - serialize_1/5/11 + deserialize_5/11 in vector/avx2/serialize.rs are
    by-design lax per prior commit 107c76641 author's explicit choice
    ("SIMD body treated as a signature-level axiom"). Flipping requires
    BitVec↔i16-array bridge for the PortableVector delegation, ~30-45min
    each — gap with prompt's "small ensures strengthening" estimate.
  - vector/avx2/ntt.rs:200 (inv_ntt_layer_1_step) and :336 (ntt_multiply)
    require new bound-tracking proofs through SIMD pipelines (60-90min
    per fn per the prompt).
  - vector/portable.rs:445/684 (op_ntt_layer_1_step / inv variant)
    explicitly need 4-way branch refactor per the inline comment + memory
    feedback_layer2_branch_post_z3_unlock (60-90min each).
  - Total realistic budget: 6h+ for these 9 sites. Per
    feedback_proof_debug_budget (30-60min cap), all defer to a follow-up
    sprint.
- Stage 3 (Generic ntt/invert_ntt body admits): PARTIAL.
  - invert_ntt_montgomery: lax → panic_free.  Body of sequential layer
    calls + is_bounded_poly_higher widenings verifies under
    `--z3rlimit 200 --ext context_pruning --split_queries always`
    (245s wall).
  - invert_ntt_at_layer_4_plus: DEFERRED.  Inner-loop maintenance VC
    saturates rlimit 400 even with split_queries (queries 188 and 204
    canceled at the cap).  Per `feedback_rlimit_cap_800` and
    `feedback_proof_debug_budget`, exceeded budget — needs structural
    refactor (per-branch helpers or simplified loop invariant).
  - ntt_vector_u: DEFERRED.  panic_free fails on the precondition checks
    of sequential `ntt_at_layer_{2,1}` calls (`6*3328`, `7*3328` bound
    args) — sibling `ntt_binomially_sampled_ring_element` verifies
    identical chain because it does not use `verification_status(panic_free)`.
    Switching to that pattern would require dropping the functional
    Hacspec ensures (cleaner deferral path; not pursued in this sprint).
- Stage 4 (op_* glue + to_bytes/from_bytes): DEFERRED — see above.
- Stage 5 (rollup): pending.

## Live notes

- 2026-05-05: stage entry. Baseline lax counts confirmed match the prompt
  (Generic/serialize=4, Generic/ntt=1, Generic/invert_ntt=2, Generic/sampling=2,
  Portable/vector=4, Avx2/ntt=2, Avx2/serialize=5, Avx2/vector=4 = 24 minus the
  2 sampling left-aside = 22 sites).
- Starting with Stage 1, fn 1: `compress_then_serialize_11` (mirror `_10`).
