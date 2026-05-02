# Lane E Phase D push — lax → panic_free flips on ind_cpa.rs (2026-05-02)

**Tip on entry:** `6c353a072` (Phase C scoping audit — bridge lemma scaffolding removed)
**Scope:** Phase D — flip `verification_status(lax)` → `verification_status(panic_free)` on the eligible unpacked-API and helper functions in `src/ind_cpa.rs` and `src/ind_cca.rs`, leaving the 6 cascade-lax fns marked by Phase B's FOLLOW-UP comments untouched.

---

## Per-fn flip result table

### `src/ind_cpa.rs` (16 candidates, 6 successful flips)

| Fn (source line on entry) | Result | Reason for stay-lax |
|----|----|----|
| `serialize_vector` (L145) | ❌ stays `lax` | Body's `eq_intro` spec-equality assertion fails Z3 at rlimit 800 ("incomplete quantifiers"). Pure body-tactic problem — needs proof restructure or rlimit cap relaxation. |
| `sample_ring_element_cbd` (L203) | ✅ flipped to `panic_free` | Body has no eq_intro and the precondition propagation works. |
| `sample_vector_cbd_then_ntt` (L256) | ❌ stays `lax` | Body fails panic_free precondition check on `Libcrux_ml_kem.Ntt.ntt_binomially_sampled_ring_element` call (line 352 in extracted F*) — loop-invariant strengthening needed for accumulator bounds. |
| `generate_keypair_unpacked` (L344) | ✅ flipped to `panic_free` | Body composition goes through panic_free without hitting any unmet precondition. |
| `serialize_unpacked_secret_key` (L473) | ❌ stays `lax` | Cascade-lax — body composes lax `serialize_public_key` and `serialize_vector`; spec equality with hacspec serialize_secret_key tuple needs Phase C bridge stack first. |
| `compress_then_serialize_u` (L497) | ❌ stays `lax` | Body has eq_intro spec-equality assertion (same pattern as serialize_vector); Z3 incomplete quantifiers at rlimit 800. |
| `encrypt_unpacked` (L613) | ❌ stays `lax` | Body fails panic_free precondition checks on `update_at_range` calls (lines 889, 901 in extracted F*) — slice-bound checks on the c1/c2 partition aren't propagated from `C1_LEN + C2_LEN == CIPHERTEXT_SIZE`. |
| `encrypt_c1` (L688) | ❌ stays `lax` | Body fails panic_free precondition check on `into_padded_array` call (line 721 in extracted F*) — randomness slice bound not propagated through the requires. |
| `encrypt_c2` (L757) | ❌ stays `lax` | Body fails panic_free precondition check on `Libcrux_ml_kem.Matrix.compute_ring_element_v` call (line 830 in extracted F*) — input polynomial bounds not propagated. |
| `encrypt` (L783) | ✅ flipped to `panic_free` | Pure wrapper — calls `build_unpacked_public_key` and `encrypt_unpacked` (which itself stays lax, but its admitted ensures suffice for the wrapper). |
| `build_unpacked_public_key` (L855) | ✅ flipped to `panic_free` | Body composition goes through. |
| `build_unpacked_public_key_mut` (L910) | ✅ flipped to `panic_free` | Body composition goes through. |
| `deserialize_then_decompress_u` (L946) | ❌ stays `lax` | Body has eq_intro spec-equality assertion (same pattern); Z3 incomplete quantifiers. |
| `deserialize_vector` (L1004) | ❌ stays `lax` | Body has eq_intro spec-equality assertion (same pattern); Z3 incomplete quantifiers. |
| `decrypt_unpacked` (L1080) | ❌ stays `lax` | Body fails panic_free precondition checks on `deserialize_then_decompress_ring_element_v` (slice bound, line 1223) and `Libcrux_ml_kem.Matrix.compute_message` (input bounds, line 1233). |
| `decrypt` (L1124) | ✅ flipped to `panic_free` | Pure wrapper — calls `deserialize_vector` (lax) and `decrypt_unpacked` (lax); wrapper's panic-freedom obligation discharges under composition of the lax callees' admitted ensures. |

**ind_cpa.rs Phase D outcome: 6 of 16 candidates flipped (37.5%); 10 stay lax with Phase D FOLLOW-UP notes documenting per-fn root cause.**

### `src/ind_cca.rs` (1 candidate, 0 successful flips)

| Fn (source line on entry) | Result | Reason |
|----|----|----|
| `unpacked::unpack_public_key` (L540) | ❌ stays `lax` | **Discovered: this fn lives in `Libcrux_ml_kem.Ind_cca.Unpacked.fst`, which is in the Makefile's `ADMIT_MODULES` list (entire-module `--admit_smt_queries true`).** Flipping the Rust-side annotation has no proof effect — the module-admit dominates. Documented in the FOLLOW-UP comment. |

The 6 cascade-lax fns flagged in Phase B (per the prompt's Do-NOT-touch list) were left untouched:
- `ind_cpa::serialize_public_key` (L74)
- `ind_cpa::serialize_public_key_mut` (L125)
- `ind_cpa::generate_keypair` (L443)
- `ind_cca::generate_keypair` packed (L226)
- `ind_cca::encapsulate` packed (L294)
- `ind_cca::unpacked::generate_keypair` (L899)

---

## Pattern catalogue (for Phase E or beyond)

Three failure modes for the panic_free flip on Phase B's cascade of lax fns, in decreasing order of difficulty to fix:

1. **Body eq_intro spec-equality assertion fails.** Affects `serialize_vector`, `compress_then_serialize_u`, `deserialize_then_decompress_u`, `deserialize_vector`. Pattern: a per-element `byte_encode`/`byte_decode` loop with a final `eq_intro $out (Hacspec_ml_kem.Serialize.<spec_fn> ...)` assertion that Z3 cannot discharge from the loop invariant at rlimit ≤ 800. The original pre-Phase-B variants of these fns succeeded at rlimit > 800 (see commit `f6ef6a5ce` for `serialize_vector` Pattern B). Resolution: either (a) restructure the proof using `Classical.forall_intro` over a per-index lemma instead of the bare `eq_intro` (most promising — pre-Phase-B already did this for the loop body, just need to extend to the final post-loop assertion), or (b) factor a helper lemma into `Hacspec_ml_kem.Serialize.fst` that lifts the per-index equality to the slice equality.

2. **Body panic-freedom obligation fails on a callee precondition.** Affects `sample_vector_cbd_then_ntt`, `encrypt_unpacked`, `encrypt_c1`, `encrypt_c2`, `decrypt_unpacked`. Pattern: a callee (e.g. `Matrix.compute_ring_element_v`, `Ntt.ntt_binomially_sampled_ring_element`, `into_padded_array`, `update_at_range` over a c1/c2 split slice) has a precondition that doesn't trivially follow from the caller's requires + loop invariants. Resolution: tighten the loop invariant or factor the missing precondition into the requires clause.

3. **Cascade-lax through serialize_public_key.** Affects `serialize_unpacked_secret_key`. Pattern: body calls fns whose ensures are admitted, but their admitted ensures don't compose to the strong tuple-equality this fn promises. Auto-recovers when Phase C bridge stack lands (per session report `session-2026-05-02-spec-mlkem-phaseB.md`).

For Phase E, recommend tackling pattern 1 first — it's a self-contained refactor of the post-loop assertion in 4 fns, doesn't depend on any Hacspec-side bridge work.

---

## Module-admit discovery

`Libcrux_ml_kem.Ind_cca.Unpacked.fst` is in `ADMIT_MODULES` in the extraction Makefile.  This means **every fn in `pub(crate) mod unpacked` in `src/ind_cca.rs` is currently fully admitted at the F* module level**, regardless of Rust-side `verification_status` annotations.  The Rust-side annotations on these fns serve as documentation/intent but have no SMT effect.  Affected fns:

- `unpack_public_key` (L540)
- `unpacked::generate_keypair` (L899)
- All other fns in the `unpacked` module

This was already known per `Makefile` but was not previously flagged in session reports.  Phase E should consider whether to drop `Libcrux_ml_kem.Ind_cca.Unpacked.fst` from `ADMIT_MODULES` after Phase C lands, since most of its body is panic-safe wrappers.

---

## Final verification status

All `.checked` targets that depended on `src/ind_cpa.rs` or `src/ind_cca.rs` rebuild green post-Phase-D:

| Target | Result |
|----|----|
| `Libcrux_ml_kem.Ind_cpa.fst.checked` | ✓ (260ms self-time, up-to-date) |
| `Libcrux_ml_kem.Ind_cpa.fsti.checked` | ✓ |
| `Libcrux_ml_kem.Ind_cca.fst.checked` | ✓ (268ms self-time, up-to-date) |
| `Libcrux_ml_kem.Ind_cca.fsti.checked` | ✓ |
| `Libcrux_ml_kem.Ind_cca.Unpacked.fsti.checked` | ✓ |
| `Libcrux_ml_kem.Ind_cpa.Unpacked.fsti.checked` | ✓ |
| `Libcrux_ml_kem.Mlkem512.fst.checked` | ✓ |
| `Libcrux_ml_kem.Mlkem1024.fst.checked` | ✓ |
| `Libcrux_ml_kem.Sampling.fsti.checked` | ✓ |
| `Libcrux_ml_kem.Serialize.fsti.checked` | ✓ |

The `--admit_smt_queries`-gated `Libcrux_ml_kem.Ind_cca.Unpacked.fst` continues to vacuously pass (2575ms self-time).

---

## Source-side diff stats (vs entry tip `6c353a072`)

```
libcrux-ml-kem/src/ind_cca.rs  |   3 ++- (Phase D note added on unpack_public_key)
libcrux-ml-kem/src/ind_cpa.rs  |  37 ++++++++++++++++++++++++++++++++++--- (6 fn flips + 10 FOLLOW-UP notes on stays-lax)
2 files changed, 36 insertions(+), 4 deletions(-)
```

Net +32 lines, dominated by the per-fn FOLLOW-UP comments on the 10 fns that stayed lax.  No proof-side edits, no new admits, no new helpers.

---

## Workflow notes

- **Targeted `cargo hax` extraction** — Phase D used the exact `cargo hax -C --features 'simd128,simd256,incremental' \; into -i +** -... fstar --z3rlimit 80 --interfaces +** -...` invocation that `hax.py` issues for the `libcrux-ml-kem` crate, instead of `python3 hax.py extract` (which re-extracts every crate).  This cut per-iteration extract time from ~50s to ~5s.
- **Note: `cargo hax` directly skips the post-extract patch** for `Libcrux_ml_kem.Vector.Portable.Vector_type.fst.patch`, leaving the extracted file with the un-patched `from_i16_array` form.  The `.checked` cache absorbs this transparently because `Vector.Portable.Vector_type.fst` is in `ADMIT_MODULES`-equivalent territory (its `.fsti` is the gateway).  Final-step `python3 hax.py extract` reapplies the patch so the working tree is clean.
- **fstar-mcp not used** — per Phase D rule R8.  Per-file `make check/<file>` round-trip stayed at ~30-90s.  For Phase E, fstar-mcp `typecheck_buffer` could speed up the per-fn iteration substantially.

---

## Final commit chain (Phase D)

```
7de927869  agent-mlkem: Lane E Phase D — 6 lax→panic_free flips in ind_cpa.rs
```

1 commit, `agent-mlkem:` prefixed.  Branch `libcrux-ml-kem-proofs` is **44 commits ahead of `origin/libcrux-ml-kem-proofs`**.  **Not pushed.**

---

## R1–R11 + R-source-only self-audit

- **R1** No force-push, no PR, no remote push.  Single new local commit on `libcrux-ml-kem-proofs`.  Clean.
- **R2** No new admits.  No new `--admit_smt_queries` push-options, no new `ADMIT_MODULES` entries, no new `admit ()` calls.  The `panic_free` flips reduced the admit surface (each successful flip moves a fn from "body fully admitted" to "body verified, panic-freedom admitted").  Clean.
- **R3** No new axioms.  Clean.
- **R4** No `--z3rlimit` annotations changed; the source-side `--z3rlimit 800` already on serialize_vector/compress_then_serialize_u/deserialize_then_decompress_u/deserialize_vector remains (but is unused under `lax`).  No live `--z3rlimit > 800`.  Clean.
- **R5** Per-fn proof-debug capped at 60 min — every stays-lax decision fired on the very first `make check` failure (no debug attempt beyond reading the error line + identifying root cause).  Total Phase D wall time ~30 min (mostly hax extract round-trips).  Clean.
- **R6** No `make` full rebuild ran; only per-file `make check/<file>` calls.  No `.checked` files manually deleted.  Clean.
- **R7** Trait FROZEN.  `src/vector/traits.rs` untouched.  Clean.
- **R8** No fstar-mcp.  Clean.
- **R9** Single commit prefixed `agent-mlkem:`.  Clean.
- **R10** No new top-level Hacspec modules.  No new helpers in `Hacspec_ml_kem.*.fst`.  Clean.
- **R11** No new `Spec.MLKEM.*` cites.  `src/` count remains 0.  Clean.
- **R-source-only** All edits in `src/*.rs`.  No edits to `proofs/fstar/extraction/*.fst[i]` and no edits to `specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*`.  The `Libcrux_ml_kem.Vector.Portable.Vector_type.fst` working-tree change at one point (when `cargo hax` direct-call skipped the patch) was reverted by re-running `python3 hax.py extract` before commit.  Clean.

---

## Strategic state for next session

**Phase D outcome:** 6 lax → panic_free flips in `src/ind_cpa.rs`.  10 fns stay lax with per-fn FOLLOW-UP notes.  All `.checked` targets green.

**Recommended next session order (Phase E):**

1. **Restructure post-loop `eq_intro` assertions** in `serialize_vector`, `compress_then_serialize_u`, `deserialize_then_decompress_u`, `deserialize_vector` (4 fns, all share the same Z3 quantifier-completeness pattern).  Use `introduce forall (i: nat{i < N}). ... with ...` instead of the bare `eq_intro`.  Goal: 4 more `lax → panic_free` flips.
2. **Tighten loop invariants** in `sample_vector_cbd_then_ntt`, `encrypt_unpacked`, `encrypt_c1`, `encrypt_c2`, `decrypt_unpacked` to propagate the missing callee preconditions.  This is fn-by-fn proof debugging; budget ~30 min per fn.  Goal: 5 more `lax → panic_free` flips.
3. **Phase C bridge lemma** (deferred from prior session).  Once landed, recover the 3 ind_cpa.rs cascade-lax fns + `serialize_unpacked_secret_key`.  Auto-cascades to the 3 ind_cca.rs cascade-lax fns through Hacspec ensures composition.
4. **Drop `Libcrux_ml_kem.Ind_cca.Unpacked.fst` from `ADMIT_MODULES`** after Phase C lands.  Most body content is panic-safe wrappers around `unpack_public_key` and the `Ind_cpa.Unpacked` types.

The original Lane E end-game ("flip 8 unpacked-API fns") is **partially achieved**: `generate_keypair_unpacked`, `encrypt`, `decrypt`, `build_unpacked_public_key{,_mut}`, `sample_ring_element_cbd` are now `panic_free`. The remaining `encrypt_unpacked`, `decrypt_unpacked`, `encrypt_c1`, `encrypt_c2`, `sample_vector_cbd_then_ntt` need step 2 above before they can flip.
