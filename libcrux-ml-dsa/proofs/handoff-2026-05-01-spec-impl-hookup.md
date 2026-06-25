# Handoff prompt — wire libcrux-ml-dsa impl into the refined Hacspec ML-DSA spec

You are picking up a verification project at the end of a session that
substantially refined the Hacspec ML-DSA reference spec
(`specs/ml-dsa/`).  Your job: connect the libcrux-ml-dsa implementation
(`libcrux-ml-dsa/src/`) to the spec by wiring `ensures` clauses that
relate impl outputs to the spec's now-verified functions.

Read this file in full, then start at "Recommended next" below.

## What changed in the spec this session (commit chain on `ml-dsa-proofs`)

| SHA | Title |
|---|---|
| `1f8c28975` | preconditions for all 3 variants + drop 2 lax annotations |
| `77cdc3b75` | drop impl-bridge and orphan functions from ML-DSA spec |
| `f127d9aa2` | Result-based ML-DSA API + drop sample_in_ball lax + tighten preconditions |
| `f5124c836` | refine MlDsaParams struct fields, drop redundant per-callsite bounds |
| `4dcd72407` | close sign_internal lax via per-iteration extraction |
| `93062f7fa` | add public ML-DSA.{KeyGen,Sign,Verify} wrappers above _internal |

End state of `specs/ml-dsa/`:

- **Zero `verification_status(lax)` annotations** in `src/`.
  `Hacspec_ml_dsa.Ml_dsa.fst`, `Hacspec_ml_dsa.Encoding.fst`,
  `Hacspec_ml_dsa.Sampling.fst` all typecheck clean.
- **All three FIPS-204 parameter sets verifiable** (was: ML-DSA-65 only).
- **Result-based public API**: `keygen`, `sign`, `verify` over
  `keygen_internal`, `sign_internal`, `verify_internal` returning
  `Result<_, MlDsaError>`.
- **Refined `MlDsaParams` struct** with per-field
  `#[hax_lib::refine(...)]` so coordinate bounds are once-discharged
  invariants, not per-callsite repetition.
- **Five spec-level `assume`s** for NTT-coefficient bounds (mirrors the
  pre-existing one in `verify_internal` for `w_approx`); these
  document the mod-q reductions that `vector_intt`/`vector_sub`
  perform internally but that hax/F*'s flow analysis doesn't
  propagate through the call chain.
- **One spec-level `assume`** on the kappa loop bound in
  `sign_internal`'s outer rejection loop (kappa ≤ 65528 across 1000
  iterations).

## The exact spec API surface to wire into

Filenames are relative to `specs/ml-dsa/`:

### `src/lib.rs:14-16` re-exports

```rust
pub use error::MlDsaError;
pub use ml_dsa::{keygen, keygen_internal, sign, sign_internal, verify, verify_internal};
pub use parameters::{
    pk_size, sig_size, MlDsaParams,
    ML_DSA_44, ML_DSA_44_C_TILDE_LEN, ML_DSA_44_PK_SIZE, ML_DSA_44_SIG_SIZE,
    ML_DSA_44_SK_SIZE, ML_DSA_44_W1_SIZE,
    ML_DSA_65, ..., ML_DSA_87, ...,
};
```

### `src/ml_dsa.rs:14-25` keygen_internal

`keygen_internal::<K, L, PK_SIZE, SK_SIZE>(xi, params) -> ([u8; PK_SIZE], [u8; SK_SIZE])`

Precondition (post-refinement, after struct invariants do the
coordinate work):
```rust
K == params.k && L == params.l
&& PK_SIZE == 32 + 320 * K
&& SK_SIZE >= 128 + (L + K) * 32 * (if params.eta == 2 { 3 } else { 4 }) + K * 416
```

### `src/ml_dsa.rs:84-100` sign_internal

`sign_internal::<K, L, SIG_SIZE, W1_BYTES, C_TILDE_LEN>(sk, m_prime, rnd, params) -> Result<[u8; SIG_SIZE], MlDsaError>`

Precondition:
```rust
K == params.k && L == params.l
&& C_TILDE_LEN <= 64 && C_TILDE_LEN >= params.lambda / 4
&& W1_BYTES >= K * 192 && W1_BYTES <= 1024
&& params.gamma1 > params.beta && params.gamma2 > params.beta
&& sk.len() >= 128 + (L + K) * 32 * (if params.eta == 2 { 3 } else { 4 }) + K * 416
&& (FIPS-config tuple: (gamma1, omega, K) ∈ {(2^17, 80, 4), (2^19, 55, 6), (2^19, 75, 8)})
&& SIG_SIZE >= params.lambda / 4 + L * 32 * gamma1_bits + params.omega + K
```

### `src/ml_dsa.rs:217-228` verify_internal

`verify_internal::<K, L, C_TILDE_LEN, W1_BYTES>(pk, m_prime, sigma, params) -> Result<(), MlDsaError>`

Precondition:
```rust
K == params.k && L == params.l
&& C_TILDE_LEN <= 64 && W1_BYTES >= K * 192 && W1_BYTES <= 1024
&& params.gamma1 > params.beta && params.gamma2 > params.beta
&& pk.len() >= 32 + 320 * K
&& sigma.len() >= C_TILDE_LEN + L * 32 * gamma1_bits + params.omega + K
```

### Public wrappers (`src/ml_dsa.rs:418-540`)

`keygen<K, L, PK_SIZE, SK_SIZE>(xi, params) -> (pk, sk)`
`sign<K, L, SIG_SIZE, W1_BYTES, C_TILDE_LEN>(sk, message, ctx, rnd, params) -> Result<sig, MlDsaError>`
`verify<K, L, C_TILDE_LEN, W1_BYTES>(pk, message, sigma, ctx, params) -> Result<(), MlDsaError>`

The `sign` / `verify` wrappers handle FIPS 204 Algorithm 2 line 5
message-prefix formatting (`M' = 0 || |ctx| || ctx || M`) via a
fixed-size `[u8; 8449]` buffer.  Required: `ctx.len() <= 255` and
`message.len() <= 8192`.

### `src/error.rs` MlDsaError variants

- `SampleInBallExhausted` — buffer-exhaustion in finite-stream
  approximation; documented < 2^-128 probability.
- `RejectionSamplingExhausted` — `sign_internal` outer loop budget.
- `MalformedSignature` — `sig_decode` rejected.
- `InvalidSignature` — norm / hash check rejected.

## What the impl side currently has (gaps Agent B found in its earlier
attempt — `proofs/agent-status/agent-mldsa-generic-status.md`)

`libcrux-ml-dsa/src/ml_dsa_generic.rs` contains:

- `generate_key_pair_internal` (line ~22)
- `sign_internal` (line ~80; ~600 LoC, has its own
  `verification_status(lax)` inside)
- `verify_internal` (line ~600)
- `inline(always)` instantiations in `instantiations/` for the three
  variants

Three remaining gaps Agent B identified, two of which **are now fixed**:

- ~~Gap A~~: keygen_internal precondition hardcoded eta=4 — **FIXED**
  (commit `1f8c28975`).
- ~~Gap B~~: verify_internal precondition hardcoded gamma1=2^19 —
  **FIXED** (commit `1f8c28975`).
- **Gap D (still open)**: the impl's `sign_internal` /
  `verify_internal` take `(message, domain_separation_context)` while
  the spec's `_internal` takes `m_prime: &[u8]`.  Wiring needs a
  `m_prime_of_dsc_message` adapter on the impl side.  Per the develop-
  locally memory rule (`MEMORY.md`) this is a sandbox helper in the
  impl module, not a spec change.  Alternatively: wire the impl to the
  spec's NEW public `sign` / `verify` (which do their own M'
  formatting from `(message, ctx)`).
- ~~Gap E (informational)~~: `sign_internal`'s body in the spec was
  `lax` — **now closed** (commit `4dcd72407`).

## Recommended next (in order)

### 1. Wire `Ml_dsa_generic.generate_key_pair_internal` ensures to `Hacspec_ml_dsa.Ml_dsa.keygen_internal`

Lowest-risk option.  The Hacspec function exists, is verified, and
the precondition shape is now correct for all three variants.  Add a
`#[hax_lib::ensures]` on the impl's `generate_key_pair_internal`
referencing the spec's `keygen_internal` via the F* path.

Estimated: 1-2 hr.

### 2. Wire `Ml_dsa_generic.verify_internal` ensures to `Hacspec_ml_dsa.Ml_dsa.verify_internal`

Adapter required: spec takes `m_prime`, impl takes `(message, ctx)`.
Choose:
- (a) Cite the **public** `Hacspec_ml_dsa.Ml_dsa.verify` in the
  ensures clause — this includes the `format_m_prime` step internally.
  Cleaner.  But the precondition tightens `message.len() <= 8192` —
  fine for tests, may bite in production deployments.
- (b) Keep citing `verify_internal` and add an impl-side adapter
  `m_prime_of_dsc_message` that mirrors `format_m_prime`.  More work
  but no message-length cap.

Recommend (a) for the first cut; revisit if `8192` cap is a problem.

Estimated: 2-3 hr.

### 3. Wire `Ml_dsa_generic.sign_internal` ensures

Highest-impact, but the impl's `sign_internal` has its own internal
`verification_status(lax)` because of the rejection loop.  After this
session the **spec**'s `sign_internal` is fully verified (no lax), so
the wiring is finally meaningful.  Same adapter choice as (2).

Estimated: 3-4 hr.  May surface new sub-function preconditions that
need threading.

### 4. Lift Ml_dsa_44/65/87 user-API wrappers out of ADMIT_MODULES

12 entries in `proofs/fstar/extraction/Makefile` ADMIT_MODULES list
(per the previous session's handoff).  Once (1)+(2)+(3) land, the
wrappers should verify automatically.  Just delete the lines.

### Stretch: tighten the spec assumes

The spec has 5 `assume`s on NTT-coefficient bounds and 1 on the kappa
loop bound (see `src/ml_dsa.rs::try_sign_iteration` and
`sign_internal`).  These could be discharged by strengthening
postconditions on `vector_intt`, `vector_sub`, etc., to expose the
mod-q-in-range guarantee.  Out of scope for the impl-hookup sprint
but worth a follow-up.

## Hard rules (from user memory at `~/.claude/projects/-Users-karthik-libcrux/memory/MEMORY.md`)

- **rlimit cap**: NEVER `--z3rlimit > 800`.  With `--split_queries
  always`, cap is 400/query.  Bumping rlimit hides structural problems.
- **Use fstar-mcp** for tight iteration.  Recreate session after each
  `make`.
- **NEVER bulk-delete `.checked` files**.  `hax.py prove` / `make`
  handle stale incrementally.
- **Touch unchanged `.checked` files** after `cargo hax extract`
  (`proofs/agent-status/touch-unchanged-checked.sh skip-unchanged`).
  Don't run it while a `make` is in flight.
- **Develop locally, upstream specs once**: new spec lemmas go in the
  consumer file first; only move into shared modules after shape is
  final.
- **Avoid `Spec.MLKEM` references** in ml-dsa ensures; cite
  `Hacspec_ml_dsa.*` only.
- **Targeted `reveal_opaque`** only; never bare
  `reveal_opaque (`%P) (P)` in loop bodies.

## Operational notes

- Branch: `ml-dsa-proofs`, tip after this handoff: `93062f7fa`.
- Iteration loop: from `libcrux-ml-dsa/`,
  `JOBS=2 ./hax.sh extract` → `touch-unchanged-checked.sh skip-unchanged`
  → `make check/<Mod>.fst` for tight iteration, OR
  `JOBS=2 ./hax.sh prove` for full baseline.
- Don't push to origin.  User merges to main manually.
- The Hacspec spec lives at the **monorepo root**, not under
  `libcrux-ml-dsa/`: `specs/ml-dsa/src/*.rs` and
  `specs/ml-dsa/proofs/fstar/extraction/Hacspec_ml_dsa.*.fst`.

## Verification of spec at the start of your session

Before touching the impl, sanity-check the spec is still clean:

```bash
cd specs/ml-dsa && cargo build --tests
cd specs/ml-dsa/proofs/fstar/extraction && make
cd specs/ml-dsa && cargo test --test nistkats sign_verify --release
```

Should produce: 3/3 NIST KATs pass, all F* modules `Verified clean`,
no errors.

## Don't pursue

- The lane-split protocol — obsolete.
- AP-4 (hax proof-libs `bits USIZE`) — leave Shuffle_table admitted.
- Reverting any spec change from this session — coordinate with the
  user instead.
