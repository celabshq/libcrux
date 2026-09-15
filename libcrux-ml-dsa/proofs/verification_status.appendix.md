# Appendix (hand-written; appended by generate_verification_status.py)

## Coverage boundaries

ML-DSA is a work in progress. The arithmetic core (NTT/inverse-NTT, Montgomery/Barrett,
decompose/`make_hint`/`use_hint`) and the (de)serialization bounds are functionally correct against
`Hacspec_ml_dsa`, and essentially the whole API is panic-free. Not yet covered by functional
correctness (per-function tiers are in the tables above):

- **Top-level API — FC admitted.** The public `sign`/`verify`/`generate_key_pair`
  (`ml_dsa_generic`, per parameter set) are proven panic-free, but their functional-correctness
  `ensures` are **admitted** (`trusted(inline-admit)`) — the end-to-end FIPS-204 equivalence theorem
  is not yet closed. These sites appear in the `Lax` column and the *Body-admit sites (audit)*
  section above.
- **`trusted(replace)` sites.** A few functions carry a hand-written F\* body via
  `#[hax_lib::fstar::replace]` (F\*-checked, but a distinct trust class from the extracted code). They
  are not reflected in the `Lax`/`Panic-safe` tally and are tracked separately (reclassification
  pending).
- **Lax and Unverified:** the `sample` rejection-sampling `lax` markers (unbounded loops,
  probabilistic termination — trusted by design) and `src/simd/tests.rs` test functions (not
  extracted), both listed in the body above.

## Proof times

Verification runs with the committed hints (`--use_hints`); a full cold build is on the order of
one to two hours, and a warm build with a seeded `.checked` cache is minutes. Per-module timings are
not pinned here — they drift with F\*/Z3 versions and hint state — so obtain current figures by
aggregating the `Query-stats` lines from a cold `make` run. The heaviest modules are the AVX2 and
portable NTT / inverse-NTT layer proofs, the `sign_internal`/`verify_internal` composition, and the
encoding serializers (the declarations carrying `--split_queries always` / `#restart-solver`).
