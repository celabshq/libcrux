# Handoff — close panic-free on `generate_key_pair` (Class B, parallel-agent sprint)

You are picking up libcrux-ml-dsa at the end of a wiring + audit
sprint.  Three top-level functions in
`libcrux-ml-dsa/src/ml_dsa_generic.rs` —
`generate_key_pair`, `sign`, `verify` — are wired with Rust-level
spec-equality `ensures` against `Hacspec_ml_dsa.Ml_dsa.*`, and bodies
are admitted via `hax_lib::fstar!("admit ()")`.  The next sprint
target is **`generate_key_pair`** specifically — flip from `admit ()`
to `#[hax_lib::fstar::verification_status(panic_free)]`.

A read-only audit has classified every function in the keygen cone
under strict polarity (POSITIVE = `panic_free` with no body admit OR
full verification with NO admit/lax/opaque of any kind).  Read it
before starting work:
**`proofs/audit-pre-post-chain.md`**.

## Strict-polarity baseline (from the audit)

The keygen cone has **~52 NEGATIVE-marker functions** out of ~115
total (positivity rate ~60%).  To panic-free *just*
`generate_key_pair`, the cone of NEGATIVES that must close is
small — most of the 52 are off the hot path (sign/verify helpers,
SIMD impl admits the keygen body doesn't touch, hash impls behind
opacity).  The keygen-only closure is:

| Work item | File | Action |
|---|---|---|
| **Body proof** | `arithmetic.rs::power2round_vector` | Remove the `admit ()` at line 89; the `panic_free` flag is already on; one inner-loop call to a fully-spec'd helper (`power2round_one_ring_element`).  Single session. |
| **Body proof** | `matrix.rs::compute_as1_plus_s2` | Remove `admit ()` at line 52.  Has a documented post-condition mismatch — see Chain 3 below. |
| **Ensures-only surface** | `samplex4.rs::sample_s1_and_s2` | Add coefficient-bound `ensures` (matching shape `is_pos_array_opaque η`).  Body stays admitted. |
| **Ensures-only surface** | `samplex4.rs::X4Sampler::matrix_flat` (trait) | Add `is_i32b_array_opaque FIELD_MAX` ensures on `matrix`. |
| **Ensures-only surface** | `sample.rs::sample_four_error_ring_elements` | Add per-coeff η bound.  Body stays admitted. |
| **Ensures-only surface** | `sample.rs::sample_up_to_four_ring_elements_flat` | Add per-coeff `is_i32b FIELD_MAX` bound (consumed via `samplex4::matrix_flat`).  Body stays admitted. |
| **The flip** | `ml_dsa_generic.rs::generate_key_pair` | Add `#[hax_lib::fstar::verification_status(panic_free)]` + `requires(signing_key.len() == SIGNING_KEY_SIZE && verification_key.len() == VERIFICATION_KEY_SIZE)`; remove `admit ()` body line.  Single commit lands the flip atomically. |

That's **2 body proofs + 4 ensures-surfacings + 1 flip = 7 work items**,
1–3 sessions parallelized.

The *signing_key* path's downstream consumer
`encoding::signing_key::generate_serialized` and
`encoding::verification_key::generate_serialized` are already
panic_free and their preconditions match what the surfaced ensures
deliver — Chain 2 in the original handoff has effectively folded
into Chains 1 + 3.

## What's already in the tree (`ml-dsa-proofs` branch)

Read these recent commits before starting:

| SHA | Title |
|---|---|
| `008109557` | ml-dsa: Class A — length-preservation ensures on samplex4 helpers |
| `b97f7f18c` | ml-dsa: add `requires(true)` to hash_functions trait methods |
| `f5f99ec11` | ml-dsa: switch wiring ensures to Rust-level spec calls |
| `ce324fdb7` | ml-dsa: wire `sign` ensures to `Hacspec_ml_dsa.Ml_dsa.sign` |
| `68f275cee` | ml-dsa: wire `verify` ensures to `Hacspec_ml_dsa.Ml_dsa.verify` |
| `003076098` | ml-dsa: wire `generate_key_pair` ensures to keygen_internal |

## The target (test mechanism for each agent)

Each agent works against the same end-state test: re-applying
`#[hax_lib::fstar::verification_status(panic_free)]` and
`#[hax_lib::requires(signing_key.len() == SIGNING_KEY_SIZE && verification_key.len() == VERIFICATION_KEY_SIZE)]`
to `generate_key_pair`, re-extracting, and running

```bash
cd libcrux-ml-dsa/proofs/fstar/extraction
make check/Libcrux_ml_dsa.Ml_dsa_generic.Ml_dsa_44_.fst
```

The error line moves down the body as each chain closes.  The
**first** failure post-Class-A is at the `ntt(&mut s1_ntt[i])` call
(NTT precondition unmet, line ~109 in `ml_dsa_generic.rs`).

**DO NOT commit the panic_free flip itself** until all three chains
close.  Each agent leaves `generate_key_pair` with the body
`hax_lib::fstar!("admit ()")` intact in `main`; the panic_free flip
is a local test scaffolding only.  Final commit lands the flip
atomically when every chain has closed.

## Three parallel chains

Each chain owns one cone of `generate_key_pair`'s body
(`ml_dsa_generic.rs:65–139`).  Run all three in parallel.

### Chain 1 — NTT-bound chain (`Sample` → `Ntt`)

**Owner:** agent-ntt-bound

**Surface:** `src/sample.rs`, `src/samplex4.rs`, `src/ntt.rs`.

**Goal:** establish that `samplex4::sample_s1_and_s2` produces
coefficients with `is_pos_array_opaque η` (matching what
`encoding::signing_key::generate_serialized` consumes downstream)
**and** that `samplex4::X4Sampler::matrix_flat` produces
coefficients with `is_i32b_array_opaque FIELD_MAX` (matching
`compute_as1_plus_s2`'s `a_as_ntt` precondition).

**Steps (in priority order):**

1. **Resolve the s1_2 shape mismatch first.**
   `signing_key::generate_serialized:29` requires
   `is_pos_array_opaque η` (non-negative shifted form).  But the
   natural sample output is the centered `is_i32b η` form.
   Pick: either (a) Chain 1's ensures uses `is_pos_array_opaque`
   directly, or (b) introduce a bridge lemma in
   `Spec.Utils` (`is_i32b → is_pos` shifted by `+η`).  If (b),
   that's a cross-crate move — flag to parent, do not proceed
   without alignment.
2. Surface the η bound on `sample_four_error_ring_elements`.  This
   is **ensures-only** — body stays `admit ()`.
3. Surface the same bound (or its η-bridged form) on
   `sample_s1_and_s2` (extending the existing length-only ensures
   landed in `008109557`).
4. Surface a `is_i32b_array_opaque FIELD_MAX` ensures on
   `samplex4::matrix_flat` (free fn) and the
   `X4Sampler::matrix_flat` trait method (extending the
   length-only ensures landed in `008109557`).  The matrix `A` is
   rejection-sampled from `[0, Q)`, so the FIELD_MAX bound holds —
   the work is just declaring it.
5. Surface the FIELD_MAX bound on
   `sample_up_to_four_ring_elements_flat` (consumed by
   `matrix_flat`).
6. (Optional, if energy permits) the inner private fns
   (`rejection_sample_less_than_eta_*`,
   `rejection_sample_less_than_field_modulus`) currently expose
   only counter ensures.  If your surface-level bound on the public
   fns can't be discharged from those alone, surface coeff bounds
   on them too.

**Existing assists:** `Spec.Utils.is_i32b_array_opaque`,
`is_pos_array_opaque`, `is_i32b_strict_lower_array_opaque` already
exist; `arithmetic.rs` and `encoding/error.rs` have many
bound-tracking patterns to mirror.

**Audit-flag:** the original "1.5–2 sessions" estimate assumed a
single body to enrich.  The audit found **5 sample-side body-admit
fns**, plus the `samplex4` trait/free-fn pair, plus the s1_2 shape
choice.  Realistic estimate: **2–3 sessions** if the agent picks
"declare ensures, keep bodies admitted" (the recommended path).

### Chain 2 — folded into Chains 1 + 3

The original Chain 2 (encoding helpers) was discovered by audit to
be **already closed**: `encoding::verification_key::generate_serialized`
and `encoding::signing_key::generate_serialized` are both
`verification_status(panic_free)` with no body admit, and their
preconditions exactly match what Chains 1 + 3 deliver.  No
dedicated agent needed.  The Chain 1 / Chain 3 owners just thread
the upstream ensures into the call site.

### Chain 3 — arithmetic / matrix chain (`compute_as1_plus_s2` → `power2round_vector`)

**Owner:** agent-arith-bound

**Surface:** `src/matrix.rs::compute_as1_plus_s2`,
`src/arithmetic.rs::power2round_vector`.

**Goal:** body proofs for the two functions.  Their `ensures` are
already declared; the work is removing the `admit ()` and writing
the loop invariants needed to discharge the body.

**Steps:**

1. **Resolve the `compute_as1_plus_s2` post-mismatch first.**
   Audit-flag: declared post is `is_i32b 16760832 ≈ 2·FIELD_MAX − 2`,
   but `power2round_vector`'s pre is `FIELD_MAX = 8380416`.
   `2·FIELD_MAX > FIELD_MAX`, so the post does **not** chain
   directly.  Inspect the body: the final pass calls a Barrett
   `reduce` then `+ s1_s2[i]` (each ≤ η) — the actual output is
   FIELD_MAX-bounded plus small slack.  Pick: (a) tighten the post
   to FIELD_MAX (preferred — matches body reality), (b) add a
   `reduce` step in `generate_key_pair` between the two calls
   (changes Rust source — needs sign-off), or (c) widen
   `power2round_vector`'s pre to `2·FIELD_MAX` and propagate to
   `power2round`'s SIMD-trait pre.  If (a) is tractable from the
   body, prefer it.
2. Body-proof `compute_as1_plus_s2`.  Pattern in
   `matrix::add_vectors` / `subtract_vectors` (rlimit 800 +
   `--split_queries always` + `Polynomial::add_bounded`-style
   accumulator).  Has nested loops.
3. Body-proof `power2round_vector`.  Single inner-loop call to a
   fully-spec'd helper.  Trivial loop_invariant chain.

**Existing assists:** `Spec.MLDSA.Math` and `Spec.Utils` already
have bound predicates needed.  `polynomial::PolynomialRingElement::add_bounded`
provides the per-iteration ghost.

**Estimate:** 1–1.5 sessions (Chain 3 was 1 in the original
estimate — the post-mismatch resolution is new audit-surfaced work).

## Parallelism plan

- **Spawn Chain 1 and Chain 3 simultaneously** — they touch
  disjoint files (`sample.rs`/`samplex4.rs` vs
  `arithmetic.rs`/`matrix.rs`).
- Chain 1 publishes its ensures shape (especially the s1_2 choice)
  early — within the first 30 minutes.  Chain 3 doesn't strictly
  block on Chain 1, but the final flip in `generate_key_pair`
  needs both.
- After both close, a **third agent (or the parent)** lands the
  panic_free flip on `generate_key_pair`, runs the F* check, and
  commits.

If serial: Chain 1 → Chain 3 → Flip.

## Hard rules (carried forward)

- **rlimit cap**: NEVER `--z3rlimit > 800` (or > 400 with `--split_queries always`).
- **Use fstar-mcp** for tight iteration when available; recreate session after each `make`.
- **NEVER bulk-delete `.checked` files**.  `make` handles stale incrementally.
- **Touch unchanged `.checked`** after `cargo hax extract`
  (`proofs/agent-status/touch-unchanged-checked.sh skip-unchanged`).
- **Develop locally, upstream specs once**.  New bound lemmas go in
  the consumer file (`Commute.Chunk` / sandbox / impl-side
  `hax_lib::fstar!` block); only move into `Specs.fst` /
  `Spec.MLDSA.Math` after the shape is final.
- **Avoid `Spec.MLKEM` references** in ml-dsa code; cite
  `Hacspec_ml_dsa.*` and `Spec.MLDSA.Math.*` only.
- **No matrix array refactor** — work within the slice API.
- **Don't remove the body `admit ()` until your chain closes** — the
  panic_free flip is local test scaffolding.  Final commit lands the
  flip atomically once all chains close.
- **Body admit at the start of a body is NEGATIVE / lax** under the
  strict polarity rule.  An admit at the *end* of a body would be
  equivalent to `panic_free` (admits only the post) but is fishy
  and labeled NEGATIVE too — bypass the canonical attribute and you
  get marked.  In this codebase no admits are at the end currently.
- **Don't disturb the SIMD impl body-admits** in
  `simd/{portable,avx2}.rs` (12 of them).  They're below the trait
  layer the keygen body uses (`Operations::ntt`,
  `Operations::power2round`, etc.) — `simd/portable.rs` exposes
  *some* admits but the F* error chain for the keygen flip never
  reaches them, since those calls dispatch through trait pres that
  the trait-level types already declare.  **Audit-flag: confirm
  during sprint** by checking which simd primitives the body
  actually invokes vs. which are admitted.
- **Status reports every 15 minutes** to the parent (sub-task,
  blocker, ETA) so the parent can detect stalls without disruptive
  SendMessage pings.

## Operational notes

- Branch: `ml-dsa-proofs`.  Tip is `51ecfbbeb` (Class B handoff
  v1, this doc supersedes it).
- Don't push to origin.  User merges to main manually.
- Spec sanity check (run BEFORE touching impl, to confirm the tip
  is clean):

  ```bash
  cd specs/ml-dsa && cargo build --tests
  cd specs/ml-dsa/proofs/fstar/extraction && make
  cd specs/ml-dsa && cargo test --test nistkats sign_verify --release
  ```

- Each agent's wall-event log:
  `libcrux-ml-dsa/proofs/agent-status/agent-<chain>-status.md`.
- The audit's spec-mapping table
  (`proofs/audit-pre-post-chain.md`, "Spec-mapping table") shows
  which `Hacspec_ml_dsa.*` function each impl maps to.  Keep this
  in mind when shaping ensures so a future functional-correctness
  sprint can re-use them.

## Success criterion

After all chains close:

1. `generate_key_pair` carries
   `#[hax_lib::fstar::verification_status(panic_free)]` and the
   length precondition `requires(...)`.
2. Body `hax_lib::fstar!("admit ()")` is **removed** (replaced by
   the implicit `_hax_panic_freedom_admit_` panic_free injects).
3. F* verifies all three variants (44/65/87) clean — same ~8s
   per variant as the wiring sprint.
4. `cargo test --release --lib`: 20/20.
5. Full F* prove pass: 0 errors, 0 make-level failures.
6. **Net body-admit count drops by 3 in the keygen cone** (from 32
   to 29):
   - `ml_dsa_generic::generate_key_pair` (the target, removed)
   - `arithmetic::power2round_vector` (Chain 3)
   - `matrix::compute_as1_plus_s2` (Chain 3)
7. Single commit: "ml-dsa: panic-free `generate_key_pair`".

## Spec-mapping (forward-looking, for the next sprint)

The audit's spec-mapping table flags which impl functions correspond
to which `Hacspec_ml_dsa.*` spec functions.  Highlights for keygen:

- `generate_key_pair` ↔ `ml_dsa::keygen_internal` (already wired)
- `samplex4::matrix_flat` ↔ `sampling::expand_a`
- `samplex4::sample_s1_and_s2` ↔ `sampling::expand_s`
- `compute_as1_plus_s2` ↔ a fragment of `keygen_internal`
  (`A·s₁ + s₂`), composed of `matrix_vector_ntt` +
  `vector_intt` + `vector_add`
- `power2round_vector` ↔ `polynomial::vector_power2round`
- `verification_key::generate_serialized` ↔ `encoding::pk_encode`
- `signing_key::generate_serialized` ↔ `encoding::sk_encode`

When shaping the bound ensures, prefer phrasings that align with
spec function postconditions — that lets a future sprint replace
"function is panic-free" with "function returns the spec value"
without re-shaping the contract.

## Don't pursue (out of scope)

- Lifting `Ml_dsa_44/65/87` user-API wrappers out of `ADMIT_MODULES`.
- Body-side functional correctness (proving the spec-equality
  ensures); the wiring keeps it admitted.
- Closing the `simd/{portable,avx2}.rs` body-admits (12 of them) —
  separate sprint focused on the SIMD layer.
- Closing `sign` / `verify`'s panic_free flip — same sprint shape
  as keygen, but ~2× the helper surface (audit lists
  `compute_matrix_x_mask`, `compute_w_approx`,
  `encoding::signature::serialize`, plus 4 more sample helpers).
- Touching `specs/ml-dsa/` unless a new spec gap surfaces (then
  stop and surface it to the parent).

## Decision points where you should stop and ask

- If the s1_2 shape mismatch resolution requires an
  `is_i32b → is_pos_array_opaque` bridge lemma in `Spec.Utils`,
  surface to parent — that's a cross-crate move.
- If the `compute_as1_plus_s2` post tightening forces a Rust-source
  change in `generate_key_pair` (option (b) in Chain 3 step 1),
  surface to parent.
- If a chain needs a `hax_lib::fstar!("assume ...")` more than once
  per file, consolidate into a named lemma in the
  consumer-file-local `hax_lib::fstar!` block and surface the
  lemma's signature to parent.
- If `--z3rlimit` would have to exceed 800 to discharge a single
  query, stop — that's a structural problem that needs decomposition.

---

## Outcome (postmortem, 2026-05-02)

This sprint partially closed.  Status of the planned work:

| Chain | Outcome | Detail |
|---|---|---|
| Chain 1 (NTT-bound, sample.rs / samplex4.rs ensures-only) | ✅ **DONE** | Three commits merged: `ddc7b2dcb` (Cluster A sample.rs), `773e4af84` (Cluster B samplex4.rs), `140702ee2` (status log). cargo test 20/20.  F* check on Sample, Samplex4*, downstream consumers all clean. |
| Chain 2 (encoding) | ✅ N/A | Audit confirmed already closed at `ad0632490`; no work needed. |
| Chain 3 (arith / matrix body proofs) | ❌ **BLOCKED** | Both `power2round_vector` and `compute_as1_plus_s2` body proofs hit hax tactic limitations.  Reverted all source changes; only the diagnostic status log committed (`2054c9f15`). |
| Final flip on `generate_key_pair` | ⏸ **DEFERRED** | Cannot land until at least `compute_as1_plus_s2`'s post tightens to satisfy `power2round_vector`'s pre. |

### Chain 3 diagnostic summary (full detail in `agent-status/agent-arith-bound-status.md`)

* **`power2round_vector`**: dual `&mut [T]` slice access pattern
  (`&mut t0[i], &mut t1[i]` in a single call to
  `power2round_one_ring_element`) hits the hax slice-bounds tactic
  in a way that `add_vectors`-style invariants cannot unblock.  Seven
  invariant variants tried.  Recommended fix: refactor
  `power2round_one_ring_element` to take t1 by value and return a
  tuple (eliminates the dual-mutable pattern).  Independent of
  Montgomery work.
* **`compute_as1_plus_s2`**: same hax tactic limitation in inner
  loop_invariant + post-mismatch resolution.  Recommended fix
  (cleaner): tighten `invert_ntt_montgomery`'s trait post via the
  shared-spec piecewise pattern (Montgomery sprint) — once landed,
  the body's natural post becomes ~q/2 + ε < FIELD_MAX, no
  in-function reduce needed, body proof simplifies considerably.
* **Independent review** confirmed BENIGN-TIGHTENING verdict on the
  proposed in-function reduce: matches PQClean's `polyveck_caddq`
  step structurally, mod-q-equivalent, KAT-safe.  Empirical
  saturation probe (1200+ adversarial trials of
  `montgomery_multiply_by_constant(_, 41_978)`): max output
  `~q/2 + 21k`, confirming the analytic formula
  `|result| ≤ q/2 + ⌈|value|/2³²⌉ + 1` from ML-KEM's documented
  bound at `libcrux-ml-kem/src/vector/portable/arithmetic.rs:343-348`.

### What this sprint produced beyond Chain 1's commits

* Independent review of the proposed source change (verdict + evidence).
* Empirical saturation probe with `montgomery_mul_const_probe.rs` /
  `power2round_boundary_probe.rs` (in worktree branches).
* Comprehensive diagnostic in `agent-status/agent-arith-bound-status.md`.
* Three USER-followup tasks (now sprint plan items in
  `handoff-2026-05-02-mont-bound-foundation.md`):
  1. `power2round_vector` body proof (refactor the helper).
  2. `compute_as1_plus_s2` body proof (post-Montgomery sprint).
  3. Montgomery-bound piecewise-tightening sprint.
* New rule in agent memory: 30–60 min hard cap per function during
  proof debug; mark and move on.

### What blocked the original plan

The handoff treated body proofs for `compute_as1_plus_s2` and
`power2round_vector` as "1–1.5 sessions" of work.  Both hit
**hax tooling edge cases**, not proof-shape edge cases:

1. The hax slice-bounds tactic checks `i < Seq.length <slice>`
   syntactically against the loop-bound expression.  It does not
   combine `Seq.length t0 == Seq.length t1` (length-equality fact)
   with `i < Seq.length t0` (loop bound) to derive
   `i < Seq.length t1`.  This is fine for single-mutable-slice
   patterns (`add_vectors`) but breaks dual-mutable-slice patterns
   (`power2round_vector`).
2. The same tactic also has trouble with multi-step bound chains
   in nested loop_invariants (`compute_as1_plus_s2` inner loop's
   `result[i]` access requires combining outer-loop bound,
   inner-loop bound, and function-pre length-comparison).
3. F* extraction side-effects from changes in one function can
   cascade into Z3 timeouts on previously-verified functions
   (observed: admitting compute_as1_plus_s2's body with WIP changes
   caused `add_vectors` query 56 to time out at rlimit 800 even
   though `add_vectors` was unchanged).

These are **hax-side limitations**, not proof-design problems.
The Montgomery sprint approach side-steps them by tightening the
trait-level post upstream, after which the consumer body proofs
become simpler and dodge the tactic edge cases.

### Where to look next

* `proofs/handoff-2026-05-02-mont-bound-foundation.md` — next sprint plan.
* `agent-status/agent-arith-bound-status.md` — full diagnostic.
* `agent-status/agent-ntt-bound-status.md` — Chain 1 detail.
