# Appendix (hand-written; appended by generate_verification_status.py)

## Coverage boundaries

All three backends (Portable, Neon, AVX2) are fully verified: `ADMIT_MODULES` is empty, and the
`EquivImplSpec.*` functional-correctness equivalence against `Hacspec_sha3` holds for the one-shot
hashing APIs on every backend. Two deliberate coverage boundaries remain, plus a `replace` trust
note (per-function tier counts are in the auto-generated body above and not repeated here):

- **Unverified — the `Digest`-trait glue.** `src/impl_digest_trait.rs`
  (`new`/`reset`/`update`/`finish`/`hash`) is the RustCrypto `Digest`/`Hasher` wrapper layer. It is
  filtered out of hax extraction (`-i -…::**`), so it has no F\* proof at any tier — it is thin
  dispatch over the verified one-shot/incremental core.
- **Incremental APIs — panic-free, not spec-equivalent.** The streaming absorb/squeeze paths
  (all backends) are proven panic-free with state-machine invariants, but not proven equal to
  `Hacspec_sha3`. An incremental-sponge ≡ spec refinement would extend functional correctness to
  these paths.
- **`trusted(replace)` sites.** A handful of proof-helper lemmas and stubs carry a hand-written F\*
  body via `#[hax_lib::fstar::replace]` (F\*-checked, used for proof scaffolding — not to replace
  algorithm logic). They are a distinct trust class, tracked separately and not reflected in the
  `Lax`/`Panic-safe` tally.

## Proof times

Verification runs with the committed hints (`--use_hints`); a full cold build takes a few minutes
on a laptop. Per-module timings are not pinned here — they drift with F\*/Z3 versions and hint
state — so obtain current figures by aggregating the `Query-stats` lines from a cold `make` run.
The heaviest modules are the Portable absorb/squeeze functional-correctness bodies, the generic
keccak-f equivalence, and the XOF bundle.
