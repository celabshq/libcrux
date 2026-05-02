# Lane E Phase E continuation — finish decrypt_unpacked → panic_free

Working directory: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem`
Branch: `libcrux-ml-kem-proofs` (47 commits ahead of `origin/libcrux-ml-kem-proofs`, NOT pushed)
Tip: `01d9adbc4` (Phase E — decrypt_unpacked partial progress)

## Read first
- `proofs/agent-status/session-2026-05-02-phaseD.md` (Phase D outcome + per-fn flip table)
- The HEAD commit message of `01d9adbc4` and `dfc1eeeb0` (Phase E so far — encrypt_unpacked solved, decrypt_unpacked blocked on bounds)
- The FOLLOW-UP comment at `src/ind_cpa.rs:~1112` (current decrypt_unpacked status)

## State on entry

Phase D+E flipped 7 of 16 candidates in `ind_cpa.rs` from `lax` → `panic_free`:

- `sample_ring_element_cbd`
- `generate_keypair_unpacked`
- `encrypt`
- `build_unpacked_public_key{,_mut}`
- `decrypt`
- `encrypt_unpacked` (Phase E — keystone for the encapsulate chain)

The `encrypt_unpacked` flip required:
- Adding length-preserving `#[hax_lib::ensures]` to its lax callees `encrypt_c1` and `encrypt_c2` (admitted under lax, but exposed in the val signature so callers see the slice-length info).
- An `assert_norm` body block establishing `c1_size(K) + c2_size(K) == cpa_ciphertext_size(K)` for K ∈ {2, 3, 4}.

All `.checked` targets green: `Ind_cpa.fst` (16s self-time), `Ind_cca.fst`, `Mlkem512/1024.fst`.

## Concrete task

Finish `decrypt_unpacked` lax→panic_free.  The slice-length precondition for `deserialize_then_decompress_ring_element_v` is already solved (assert_norm body block in source establishes `cpa_ciphertext_size(K) - c1_size(K) == 32 * vector_v_compression_factor(K)` for K ∈ {2, 3, 4}).

Remaining blocker: `Libcrux_ml_kem.Matrix.compute_message` requires:

- `is_bounded_poly 4095 v` — where `v` is the result of `deserialize_then_decompress_ring_element_v`
- `is_bounded_poly 3328 (secret_as_ntt.[i])` for `i < K`
- `is_bounded_poly 3328 (u_as_ntt.[i])` for `i < K` — where `u_as_ntt` is from `deserialize_then_decompress_u`

None of these bounds are propagated by the current ensures.  Real spec property: decompress output is in `[0, q-1]` where q=3329.  The bound `4095` on `v` is suspicious (looks like the 12-bit decode range pre-compression?) — verify what's actually true and what `compute_message` *needs* (maybe its requires can weaken to 3328 if 4095 is unused).

## Approach

1. Look at how `compute_message` uses its `v` argument internally — does it really need 4095?  If only 3328 suffices, weaken the requires.
2. Add `is_bounded_poly 3328` (or 4095) ensures to `deserialize_then_decompress_ring_element_v` in `specs/ml-kem/src/serialize.rs` (or the F* val if Hacspec-side).  This is a real spec property — should be honestly verified, not lax-admitted.
3. Add `is_bounded_poly 3328` invariant to `IndCpaPrivateKeyUnpacked.secret_as_ntt` field — likely as a struct invariant or as a `decrypt_unpacked` precondition.
4. Add `is_bounded_poly 3328` ensures to `deserialize_then_decompress_u` (currently lax) — since `u_as_ntt` flows from there.
5. Re-verify decrypt_unpacked.

## Rules (carry from Lane E Phase D)

- **R1** No force-push, no PR, no remote push without explicit user authorization.
- **R2** No new admits/axioms — only real verification or document-as-FOLLOW-UP-and-stay-lax.
- **R3** Per-fn 60-min debug budget; flag and move on if exceeded.
- **R4** `--z3rlimit ≤ 800` (≤400 with `--split_queries`).
- **R5** Trait FROZEN — no edits to `src/vector/traits.rs`.
- **R6** No new `Spec.MLKEM` cites — Hacspec-form only.
- **R7** Source-only edits in `src/*.rs` (and `specs/ml-kem/src/*.rs` is OK for spec work).  Never manually edit `proofs/fstar/extraction/*.fst[i]` or `specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*`.
- **R8** Targeted `cargo hax` extract (use the exact flags from `python3 hax.py extract`'s output for libcrux-ml-kem to skip cross-crate work; finalize with `python3 hax.py extract` once before commit to apply post-extract patch).
- **R9** Prefer real verification over admit shuffling — do NOT remove modules from `ADMIT_MODULES` only to add `lax` markers.  (User mandate from this session.)
- **R10** `fstar-mcp` may be useful for iterative work on a single file; be aware it can choke on large proof files (`Ind_cpa.fst` is borderline).
- **R11** Commits prefixed `agent-mlkem:`.

## Stretch goal if decrypt_unpacked closes

Once `decrypt_unpacked` is `panic_free`, attempt `unpacked::encapsulate` and `unpacked::decapsulate` (currently default-verify and module-admitted in `Ind_cca.Unpacked.fst`) → flip to `panic_free`.  Then **honestly** drop `Ind_cca.Unpacked.fst` from `ADMIT_MODULES` in `proofs/fstar/extraction/Makefile` (the previous attempt was reverted at `685191caf` because it was admit-shuffling).

## End-of-session deliverable

Append "Phase E push 2" section to a fresh `proofs/agent-status/session-2026-05-XX-phaseE-cont.md` with per-fn flip table, R-rule audit, final commit SHA chain.  Do not push to remote.
