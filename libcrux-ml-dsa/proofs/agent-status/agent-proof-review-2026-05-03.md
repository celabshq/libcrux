# Proof Review — Milestones A vs B (2026-05-03)

## TL;DR (5 lines)

* **Milestone A** (trait-and-below vs Hacspec, trait-and-above panic-free to ml-dsa-generic): **~4–6 calendar weeks**. Blockers: 12 SIMD body-admits (Mont Stages 4–7, NTT/invntt lax, compute_hint/use_hint, encoding) + 7 above-trait body admits gated on Mont sprint.
* **Milestone B** (up to ml-dsa vs Hacspec): **~3–6 months**. Blockers: 7 in-spec `assume` clauses in `Hacspec_ml_dsa.Ml_dsa.fst`, 2 admitted commute lemmas, `Spec.MLDSA.Math.lemma_mont_red_mod_q = admit ()`, no correctness posts on 22 of 27 trait methods, and 17 `ADMIT_MODULES` wrappers.
* Surfaces are **mostly disjoint** (A: bodies + bound posts; B: Hacspec equality posts + wrapper extraction). Parallel possible after Mont Sprint 1 closes, since both need the tightened `invert_ntt_montgomery` post.
* **Recommendation: Sprint A first** — closes the Mont sprint payoff, produces "panic-free ML-DSA". B as slow-burn parallel on `encoding/t0`, `encoding/t1`, `polynomial.rs` where the Hacspec side is clean.
* **Biggest user-design call**: shape of per-lane correctness posts on `Operations::{ntt, invert_ntt_montgomery, *_serialize, *_deserialize, rejection_sample_*}`. Without those, B is structurally blocked.

---

## 1. AVX2 admits / lax — exact inventory

### Bodies that are `admit ()` inside the trait impl (post still trusted)

`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/src/simd/avx2.rs`:

| Line | Method | Status |
|---|---|---|
| 500 | `compute_hint` | Body admitted; full 3-part post (bounds + hacspec lane post) declared |
| 522 | `use_hint` | Body admitted; full 3-part post (gamma2-conditional bounds + hacspec lane post) |
| 622 | `rejection_sample_less_than_field_modulus` | Body admitted; bounds-only post (no per-3byte hacspec) |
| 635 | `rejection_sample_less_than_eta_equals_2` | Body admitted; bounds-only post |
| 648 | `rejection_sample_less_than_eta_equals_4` | Body admitted; bounds-only post |
| 819 | `ntt` | Body admitted; bounds-only post `is_i32b_array_opaque FIELD_MAX` |
| 833 | `invert_ntt_montgomery` | Body admitted; bounds-only post `is_i32b_array_opaque 4_211_177` (tight bound is already declared, ready for Stage 4 chain) |

### Whole-fn `verification_status(lax)` (worse — body AND post untrusted)

| File:line | Function |
|---|---|
| `src/simd/avx2/arithmetic.rs:302` | `compute_hint` (free fn) |
| `src/simd/avx2/arithmetic.rs:329` | `use_hint` (free fn) |
| `src/simd/avx2/invntt.rs:8` | `invert_ntt_montgomery` (free fn, outer wrapper) |
| `src/simd/avx2/invntt.rs:12` | `inv_inner` (inner unsafe fn) |

### `assume (...)` shims inside otherwise-panic-free bodies

| File:line | Context |
|---|---|
| `src/simd/avx2/rejection_sample/less_than_field_modulus.rs:64-67` | 4 `assume` clauses on `count_ones` / nibble bounds |
| `src/simd/avx2/rejection_sample/less_than_eta.rs:54-57` | Same shape |

### AVX2 encoding admits (panic_free + body admit)

| File:line | Function |
|---|---|
| `src/simd/avx2/encoding/gamma1.rs:52` | `gamma1::serialize::serialize` |
| `src/simd/avx2/encoding/gamma1.rs:106` | `gamma1::serialize::deserialize` |
| `src/simd/avx2/encoding/gamma1.rs:185` | (variant) |
| `src/simd/avx2/encoding/gamma1.rs:213` | (variant) |
| `src/simd/avx2/encoding/t0.rs:69` | `t0::serialize` |
| `src/simd/avx2/encoding/error.rs:58, 109` | `error::serialize`, `error::deserialize` |

### AVX2 clean (post-Mont-Stage-3)

* `src/simd/avx2.rs:539-580` `montgomery_multiply`: per-lane equality + bound + mod-q via `Classical.forall_intro` bridge. ~112s.
* `src/simd/avx2.rs:608, 850` `power2round`, `reduce`: delegate to verified helpers.
* `src/simd/avx2/invntt.rs:521, 550` layers 3, 4 = `panic_free`.
* `src/simd/avx2/arithmetic.rs::montgomery_multiply{,_aux,_by_constant}`: posts = per-lane equality only; bodies = single `reveal_opaque mont_red`.

### Portable (`src/simd/portable.rs`)

5 trait-impl body admits, same shape as AVX2 twins: lines 529, 541, 553, 683, 696. **`ntt` and `invert_ntt_montgomery` portable impls are NOT admitted — they're verified.**

---

## 2. Above-trait admits / lax — classification

| File:line | Function | Class | Notes |
|---|---|---|---|
| `src/arithmetic.rs:84-89` | `power2round_vector` | (i) body admit, panic_free | Class B sprint reverted; needs `power2round_one_ring_element` refactor |
| `src/arithmetic.rs:215-245` | `use_hint` | (i) body admit, panic_free | Same shape |
| `src/matrix.rs:52` | `compute_as1_plus_s2` | (i) body admit | Mont-Sprint-2 target |
| `src/matrix.rs:105` | `compute_matrix_x_mask` | (i) body admit | Mont-Sprint-2 target |
| `src/matrix.rs:296` | `compute_w_approx` | (i) body admit | Mont-Sprint-2 target |
| `src/sample.rs:104` | `sample_up_to_four_ring_elements_flat` | (i) body admit | Has bounds-post; chain1 surfaced ensures |
| `src/sample.rs:318` | `sample_four_error_ring_elements` | (i) body admit | Bounds-post present |
| `src/sample.rs:419, 447, 539` | `sample_mask_ring_element`, `sample_mask_vector`, `sample_challenge_ring_element` | (i) body admit | No surfaced ensures |
| `src/ml_dsa_generic.rs:78, 157, 385, 513, 549, 591, 640, 688, 718, 761` | `generate_key_pair`, `sign_internal`, `verify_internal`, plus the 6 wrapper variants + `derive_message_representative` | (i) body admit | Posts cite `Hacspec_ml_dsa.Ml_dsa.{keygen,sign,verify}` (correctness equality) — admitted bodies |
| `src/encoding/signature.rs:107` | `serialize` | (i) body admit, panic_free | Documented count-ones lemma gap |
| `Makefile ADMIT_MODULES` | `Libcrux_ml_dsa.Ml_dsa_{44,65,87}_*.fst` (12 files) + `Libcrux_ml_dsa.Samplex4*.fst` (4 files) + `Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Shuffle_table.fst` (1) | (ii) lax (whole module) | 17 admitted modules |

No `verification_status(lax)` in any above-trait Rust file. No `hax_lib::opaque` shims. No SMTPat-only patterns.

---

## 3. Hacspec_ml_dsa — coverage state

`/Users/karthik/libcrux-ml-dsa-proofs/specs/ml-dsa/proofs/fstar/extraction/`:

* `Hacspec_ml_dsa.Ml_dsa.fst` — 1135 lines; defines `keygen_internal`, `sign_internal`, `verify_internal`, plus wrapper `keygen`, `sign`, `verify`. **Public API surface IS present.**
* `Hacspec_ml_dsa.{Sampling, Polynomial, Encoding, Ntt, Arithmetic, Matrix, Parameters, Hash_functions, Error}.fst` — full supporting modules, 81 `let` defs total.
* Hash_functions/Error use `assume val ...` for primitives (acceptable opaque interface).

**7 floating `assume (forall i. ...)` clauses inline in `Hacspec_ml_dsa.Ml_dsa.fst`** at lines 115, 224, 326, 354, 403, 598, 800 — mid-function loop-invariant axioms (spec is internally cheating). Plus 1 admitted commute lemma `lemma_decompose_spec_eq_decompose` (`Hacspec_ml_dsa.Commute.Chunk.fst:639`).

**Verdict**: spec scaffold done, but not in "hookable as final equality post" state. The 7 inline `assume`s must be discharged or lifted to caller preconditions for B to be meaningful. Spec sanity (`cargo test` in `specs/ml-dsa`) passes — KAT-correct.

---

## 4. `Spec.MLDSA.Math` admits

`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/proofs/fstar/spec/Spec.MLDSA.Math.fst`:

* Line 258: `lemma_mont_red_mod_q` — `admit ()`. Documented USER-FOLLOWUP. Original calc-chain preserved as 50-line comment (lines 195-252). Discharging is mechanical (port from `Hacspec_ml_dsa.Commute.Chunk.lemma_mont_mul_bound_and_mod_q`, which already proves the same property for `mont_mul`).

`Spec.Intrinsics.fsti:20` — single `admit()` on a low-level intrinsic; out-of-scope (not ml-dsa-specific).

`Spec.MLDSA.Ntt.fst` — clean (no admits/assumes).

`Hacspec_ml_dsa.Commute.Chunk.fst:639` — `lemma_decompose_spec_eq_decompose` is `admit ()`. Documented as "150-200 line bit-trick interval analysis" deferred to a USER-lane sprint. **This blocks AVX2 `decompose` body proof** which currently delegates to `arithmetic::decompose` (verified) but the AVX2 free-fn `compute_hint` / `use_hint` bridge through this lemma. Direct line of dependency to §1's `compute_hint`/`use_hint` lax markers.

---

## 5. Panic-freedom first, or Hacspec correctness first?

The Hacspec scaffold is in place but with 7 in-spec `assume`s and 2 admitted commute lemmas. Above-trait code already cites `Hacspec_ml_dsa.Ml_dsa.{keygen,sign,verify}` in ensures (commits `f5f99ec11`, `ce324fdb7`, `68f275cee`, `003076098`) — wiring done, body proofs missing.

The trait layer is the bottleneck: per the audit, **only 5 of 27 Operations methods carry a correctness post** (`decompose`, `compute_hint`, `use_hint`, `power2round`, `reduce`). The other 22 are bounds-only or have no correctness post (`ntt`, `invert_ntt_montgomery`, all 7 serialize/deserialize, 3 rejection_sample, `gamma1_*`, `commitment_*`, `error_*`, `t0_*`, `t1_*`).

**Without those, Milestone B is structurally blocked**: a caller of `Operations::ntt` can't say "result == `Hacspec_ml_dsa.Ntt.ntt input`" because the fact isn't in the trait contract.

**Conclusion**: Milestone A first. Milestone B requires (a) Mont Sprint completion, (b) trait pattern follow-up sprint (10 methods ≥30 min each = 1–2 weeks), (c) discharge of 7 in-spec `assume`s in `Hacspec_ml_dsa.Ml_dsa.fst`, (d) `lemma_mont_red_mod_q` + `lemma_decompose_spec_eq_decompose` discharge.

Cleaner B parallel target: `encoding/t0.rs`, `encoding/t1.rs`, `polynomial.rs::add/subtract` — clean Hacspec mapping (`Hacspec_ml_dsa.Encoding.bit_pack_*`, `Hacspec_ml_dsa.Polynomial.{add,subtract}`), no body admits. Adding correctness posts here is the right "first piece of B".

---

## 6. Human-vs-agent split

**Human (design, 5–10 hr cumulative):**

* Shape of correctness posts on `Operations::{ntt, invert_ntt_montgomery, *_serialize, *_deserialize, rejection_sample_*}`. 27-method API call.
* Whether to discharge the 7 in-spec `assume (forall i. ...)` in `Hacspec_ml_dsa.Ml_dsa.fst` directly, OR rewrite spec to lift them to caller obligations. Affects every above-trait equality proof.
* Whether `lemma_mont_red_mod_q` is first priority (mechanical, ~1 day) or deferred.
* Whether to refactor `power2round_one_ring_element` to take `t1` by value (Class B recommendation).
* Mont Stage 4 (a) vs (b) decision — in resume prompt; awaiting user.

**Agents (mechanical, parallel):**

* Mont Stages 4–7 (prompted in `agent-mont-stage3-resume-prompt.md`).
* Mont-Sprint-2 body proofs: `compute_as1_plus_s2`, `compute_w_approx`, `compute_matrix_x_mask` once Stage 7 lands.
* 8 trait-pattern drive-by surfacings (audit table).
* 12 SIMD body-admit cleanups in portable + AVX2, ~1–2 hr each.
* 7 AVX2 encoding admits (gamma1, t0, error), ~1 hr each.
* Discharge `lemma_mont_red_mod_q` from preserved calc-chain.
* Lift the 17 `ADMIT_MODULES` once deps close.

---

## Recommendation

**Sprint A first, sequentially:**

1. **Week 1** — Finish Mont Sprint 1 Stages 4–7. Tight `invert_ntt_montgomery` post lands.
2. **Week 1–2** — Mont Sprint 2: `compute_as1_plus_s2` body, refactor + close `power2round_vector`.
3. **Week 2** — `generate_key_pair` panic-free flip.
4. **Week 3** — `sign_internal` flip (closes `compute_w_approx`, `compute_matrix_x_mask`, sample_mask_*, sample_challenge_*).
5. **Week 3–4** — `verify_internal` flip + 7 wrappers in `ml_dsa_generic.rs`.
6. **Week 4–5** — 12 SIMD body-admit cleanup, discharge `lemma_decompose_spec_eq_decompose`, close `compute_hint`/`use_hint` bodies.
7. **Week 5–6** — 7 AVX2 encoding admits, `encoding/signature.rs::serialize` (count-ones), 8 trait drive-bys, `lemma_mont_red_mod_q`.

**End of A: zero body admits in keygen/sign/verify cone, zero `verification_status(lax)`, only 17 `ADMIT_MODULES` (wrapper-API + samplex4 dispatchers) left.**

**Sprint B parallel from Week 4** as slow-burn: `encoding/t0`, `encoding/t1`, `polynomial.rs` first. Trait correctness-post sprint (10 methods) around Week 6. The 7 in-spec `assume`s in `Hacspec_ml_dsa.Ml_dsa.fst` are the gate for the final wrapper-equality flip — defer until trait posts in place.

Pure-B-first is **not recommended**: trait correctness gap means agents stuck on body proofs that can't see their spec. Run B as side-channel to A.
