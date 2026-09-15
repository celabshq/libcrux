# Appendix (hand-written; appended by generate_verification_status.py)

## Coverage boundaries

All three backends are functionally correct for the one-shot KEM API and the arithmetic core, and
essentially the whole API is at least panic-free (per-function tiers in the tables above).
Boundaries not covered by functional correctness:

- **Incremental API — panic-free, not spec-equivalent.** The `ind_cca::incremental` key/encaps
  paths are proven panic-free and precondition-respecting, but carry no `Hacspec_ml_kem`
  equivalence (unlike the one-shot API). An incremental ≡ one-shot refinement would extend
  functional correctness to these paths.
- **Lax and Unverified:** listed in the body above — `sampling::sample_from_xof` and two
  incremental-API `From`-instance bodies (lax; hax-limited), plus `src/lib.rs` top-level glue
  (unverified, not extracted).

## Proof times

Verification runs with the committed hints (`--use_hints`); a warm full-crate `make all` re-checks
the whole crate in a few minutes, and a cold build is longer. Per-module timings are not pinned
here — they drift with F\*/Z3 versions and hint state — so obtain current figures by aggregating the
`Query-stats` lines from a cold `make` run. The heaviest queries are the AVX2 serialization
routines, the portable NTT / inverse-NTT layer-step wrappers, and the `Hacspec_ml_kem.Commute.*`
NTT-bridge lemmas.
