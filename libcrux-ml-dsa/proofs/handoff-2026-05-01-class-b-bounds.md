# Handoff — close panic-free on `generate_key_pair` (Class B, parallel-agent sprint)

You are picking up libcrux-ml-dsa at the end of a wiring + Class A
helper sprint.  Three top-level functions in
`libcrux-ml-dsa/src/ml_dsa_generic.rs` —
`generate_key_pair`, `sign`, `verify` — are wired with Rust-level
spec-equality `ensures` against `Hacspec_ml_dsa.Ml_dsa.*`, and bodies
are admitted via `hax_lib::fstar!("admit ()")`.  The next sprint
target is to flip `generate_key_pair` from `admit ()` to
`#[hax_lib::fstar::verification_status(panic_free)]` — i.e. prove the
body is panic-free while keeping the spec-ensures admitted.

A scope-out pass already separated blockers into:

- **Class A — opacity-only / length-preservation** — already landed
  (`requires(true)` on hash_functions traits, length-preserving
  ensures on `samplex4::sample_s1_and_s2` and `X4Sampler::matrix_flat`,
  `requires(true)` on the trait method).  After Class A, the next
  iteration of F* on `generate_key_pair` (with `panic_free` flipped
  on) hits the **NTT-bound chain** at the first `ntt(&mut s1_ntt[i])`
  call in `generate_key_pair`.
- **Class B — real coefficient-bound preconditions** — this sprint.

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

The error line moves down the body as each chain closes.  Pre-Class-B
the first failure is at the `ntt(&mut s1_ntt[i])` inside the inner
`for i in 0..s1_ntt.len()` loop — that's where Chain 1 starts.

**DO NOT commit the panic_free flip itself** until all three chains
close.  Each agent leaves `generate_key_pair` with the body
`hax_lib::fstar!("admit ()")` intact in `main`; the panic_free flip
is a local test scaffolding only.

## Three parallel chains

The body of `generate_key_pair` (read top-to-bottom in
`ml_dsa_generic.rs:65–139`) breaks into three roughly-independent
proof-obligation chains.  Each can be attacked by a separate
subagent.

### Chain 1 — NTT-bound chain (Sample → Ntt)

**Owner:** agent-ntt-bound

**Surface:** `src/sample.rs`, `src/ntt.rs`, `src/samplex4.rs`,
`src/encoding/error.rs::deserialize_to_vector_then_ntt` (consumed by
`sign_internal`, but the `keygen` chain only goes through
`sample_s1_and_s2 → ntt`).

**Goal:** establish that every coefficient of every polynomial
produced by `samplex4::sample_s1_and_s2` (and by extension
`sample_four_error_ring_elements` in `src/sample.rs`) lies in
`[-eta, eta]`, which is sufficient to discharge `ntt`'s precondition
(`Spec.Utils.is_i32b_array_opaque (v NTT_BASE_BOUND) ...` — see
`src/ntt.rs:7-10`).

**Steps:**
1. Read `Hacspec_ml_dsa.Ml_dsa.keygen_internal` and `expand_s` —
   the spec-side bound is `[-eta, eta]` on each output coefficient.
2. Add a coefficient-bound ensures to
   `samplex4::sample_s1_and_s2`:
   `forall i j. abs(future(s1_s2)[i].coefs[j]) <= eta`
   in whatever idiom matches existing `Spec.Utils.is_i32b_array_opaque`.
3. Same on `sample_four_error_ring_elements`.  Likely already has
   internal bounds — surface them as ensures.
4. Discharge `ntt`'s pre at the call site in `generate_key_pair`
   either via the chained ensures, or via a one-line
   `hax_lib::fstar!("assert ...")` hint.

**Existing assists:** `Spec.Utils.is_i32b_array_opaque` already
exists in `Spec.Utils` and is the canonical predicate; `arithmetic.rs`
has many bound-tracking patterns to mirror.

**Estimate:** 1.5–2 sessions.  This is the dominant chain.

### Chain 2 — encoding chain (verification_key + signing_key)

**Owner:** agent-encoding-bound

**Surface:** `src/encoding/verification_key.rs`,
`src/encoding/signing_key.rs`, `src/encoding/t0.rs`,
`src/encoding/error.rs`.

**Goal:** establish that the two `generate_serialized` calls at the
end of `generate_key_pair` are panic-free given:

- `t1: [Polynomial; ROWS_IN_A]` with coefficients in
  `[0, (Q-1)/2^D]` (set by `power2round_vector`).
- `s1_s2: [Polynomial; ROW_COLUMN]` with coefficients in
  `[-eta, eta]` (from Chain 1).
- `t0: [Polynomial; ROWS_IN_A]` with coefficients in
  `[-2^(D-1), 2^(D-1)]` (set by `power2round_vector`).
- `verification_key`, `signing_key`: fixed-size byte slices.

**Steps:**
1. Audit `encoding::verification_key::generate_serialized` — already
   has hax_lib::requires on `t1` bounds; verify they discharge from
   the upstream chain.
2. Same for `encoding::signing_key::generate_serialized`.
3. Add length-preserving ensures on the output byte slices
   (`signing_key`, `verification_key` mutated via `&mut`).
4. Internal helpers (`encoding::t1::serialize`, `encoding::error::serialize`)
   may need their own opacity-only `requires(true)` and length
   ensures — Class A pattern.

**Existing assists:** the encoding modules are mostly out of
ADMIT_MODULES already; they verify their own bodies.  The work is
threading ensures, not closing new proofs.

**Estimate:** 1 session.

### Chain 3 — arithmetic / matrix chain (compute_as1_plus_s2 → power2round)

**Owner:** agent-arith-bound

**Surface:** `src/matrix.rs::compute_as1_plus_s2`,
`src/arithmetic.rs::power2round_vector`,
`src/ntt.rs::ntt_multiply_montgomery` (used inside compute_as1_plus_s2).

**Goal:** establish that:

- `compute_as1_plus_s2` produces a `t0: [Polynomial; ROWS_IN_A]`
  where coefficients are in `[0, q)` (or whatever bound
  `power2round_vector` requires).
- `power2round_vector(&mut t0, &mut t1)` is panic-free given that
  bound on `t0`.

**Steps:**
1. `compute_as1_plus_s2`'s requires already constrain `a_as_ntt`,
   `s1_ntt`, `s1_s2` bounds.  Confirm they're discharged from
   Chain 1 (sampled bounds + post-NTT bounds).
2. Add ensures on `compute_as1_plus_s2` for the output `t0`'s bound
   (`< q`).
3. `power2round_vector` already has internal bounds; surface as
   ensures on the outputs `(t0, t1)`.
4. Discharge in `generate_key_pair` — the Chain 1 NTT post-bound
   feeds into compute_as1_plus_s2 as `s1_ntt` precondition.

**Existing assists:** `Spec.MLDSA.Math` and `Spec.Utils` already have
the bound predicates needed.

**Estimate:** 1 session.

## Execution order

Chain 1 must close first (NTT-bound is the input invariant for
Chain 3).  Chains 2 and 3 are roughly parallel after Chain 1, but
Chain 2 mostly runs on its own data path (writing the keys at the end
of `generate_key_pair`) and only needs the bounds Chain 3 produces
for `t0`/`t1`.

Recommended scheduling: **kick off all three agents in parallel**;
they communicate through the shared `src/` tree.  The Chain 1 agent
should publish its `sample_s1_and_s2` ensures shape EARLY (within the
first hour) so Chain 3 can consume it; Chain 2 can run independently
the whole time on the encoding side.

If you only have one agent at a time: do Chain 1, then Chain 3, then
Chain 2.

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
  flip atomically once all three chains close.
- **Status reports every 15 minutes** to the parent (sub-task,
  blocker, ETA) so the parent can detect stalls without disruptive
  SendMessage pings.

## Operational notes

- Branch: `ml-dsa-proofs`.
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

## Success criterion

After all three chains close:

1. `generate_key_pair` carries
   `#[hax_lib::fstar::verification_status(panic_free)]` and
   `#[hax_lib::requires(...)]` (the length precondition).
2. The body's `hax_lib::fstar!("admit ()")` is **removed** (replaced
   by the implicit `_hax_panic_freedom_admit_` injected by panic_free).
3. F* verifies all three variants (44/65/87) clean.
4. `cargo test --release --lib`: 20/20.
5. Single commit: "ml-dsa: panic-free `generate_key_pair`".

## Don't pursue (out of scope)

- Lifting `Ml_dsa_44/65/87` user-API wrappers out of `ADMIT_MODULES`.
- Body-side functional correctness (proving the spec-equality
  ensures); the wiring keeps it admitted.
- Touching `specs/ml-dsa/` unless a new spec gap surfaces (then stop
  and surface it to the parent).

## Decision points where you should stop and ask

- If a chain requires changing a function's signature (not just
  annotations), surface to parent.
- If a chain needs a `hax_lib::fstar!("assume ...")` more than once
  per file, consolidate into a named lemma in the
  consumer-file-local `hax_lib::fstar!` block and surface the
  lemma's signature to parent.
- If `--z3rlimit` would have to exceed 800 to discharge a single
  query, stop — that's a structural problem that needs decomposition.
