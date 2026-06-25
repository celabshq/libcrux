# Handoff prompt — convert ml-dsa wiring ensures to Rust-level expressions

You are picking up a libcrux-ml-dsa verification project at the end of
a successful wiring session.  The three top-level public APIs
(`generate_key_pair`, `sign`, `verify`) in
`libcrux-ml-dsa/src/ml_dsa_generic.rs` now have spec-equality
`ensures` clauses wired to `Hacspec_ml_dsa.Ml_dsa.{keygen_internal,
sign, verify}` respectively.  Bodies remain `admit ()` (wiring, not
body proof).

The current ensures use raw F* strings via `hax_lib::ensures(|r|
fstar!(r#"..."#))`.  Your job: refactor all three to **pure Rust**
expressions with `cfg(hax)` access to the spec crate, so the spec
calls and the params struct are type-checked at `cargo check` time
instead of F*-extract time.

## What's already in the tree (commits on `ml-dsa-proofs`)

| SHA | Title |
|---|---|
| `003076098` | wire `generate_key_pair` ensures to `Hacspec_ml_dsa.Ml_dsa.keygen_internal` |
| `d875b281a` | specs: ml-dsa W1_BYTES precondition gamma2-conditional (Gap F) |
| `68f275cee` | wire `verify` ensures to `Hacspec_ml_dsa.Ml_dsa.verify` |
| `ce324fdb7` | wire `sign` ensures to `Hacspec_ml_dsa.Ml_dsa.sign` |

Read `libcrux-ml-dsa/proofs/agent-status/agent-mldsa-generic-status.md`
for a one-page status of the wiring; read this file in full; then start
on the refactor.

## Why refactor

Current state — three `fstar!(r#"..."#)` ensures:

```rust
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${signing_key}_future == Seq.length ${signing_key} /\
    Seq.length ${verification_key}_future == Seq.length ${verification_key} /\
    (let pk_spec, sk_spec =
       Hacspec_ml_dsa.Ml_dsa.keygen_internal
          v_HACSPEC_PARAMS.Hacspec_ml_dsa.Parameters.f_k
          v_HACSPEC_PARAMS.Hacspec_ml_dsa.Parameters.f_l
          v_VERIFICATION_KEY_SIZE v_SIGNING_KEY_SIZE
          ${randomness} v_HACSPEC_PARAMS in
     Seq.equal ${signing_key}_future sk_spec /\
     Seq.equal ${verification_key}_future pk_spec)"#))]
```

Cost of leaving this:
- Typos in `Hacspec_ml_dsa.Ml_dsa.keygen_internal` or `f_k`/`f_l`/etc.
  surface only at `make`/F* time, ~8s per variant.
- The macro in `crates/utils/macros/src/lib.rs` injects a raw F*
  declaration via `hax_lib::fstar::after(...)`, sidestepping Rust's
  type system entirely.
- Cross-error-type mismatches (`Result<(), VerificationError>` vs.
  `Result<(), MlDsaError>`) are papered over with manual
  `Core_models.Result.impl__is_ok` calls instead of Rust's `is_ok()`.

Goal: make the spec call look like a Rust function call that `cargo
check --cfg hax` would catch typos on.

## The plan (option 1 from this session's discussion)

### Step 1 — add the cfg(hax) dep

`libcrux-ml-dsa/Cargo.toml` currently has:

```toml
[target.'cfg(hax)'.dependencies]
core-models = { path = "../crates/utils/core-models", version = "0.0.5" }
```

Add a sibling line for `hacspec_ml_dsa`:

```toml
[target.'cfg(hax)'.dependencies]
core-models = { path = "../crates/utils/core-models", version = "0.0.5" }
hacspec_ml_dsa = { path = "../specs/ml-dsa" }
```

Note: `hacspec_ml_dsa` is already a `[dev-dependencies]` for the
cross-spec tests (line 49); the cfg(hax) entry is the parallel one
for hax-extract builds.  Both should coexist — Cargo treats them as
distinct slots.

Verify with `cargo check --tests` (no cfg(hax)) and a
`RUSTFLAGS="--cfg hax" cargo check` (should compile but with
hax-only deps active).

### Step 2 — replace the macro-level F* injection with a Rust const

`crates/utils/macros/src/lib.rs:86-103` currently injects:

```rust
#[hax_lib::fstar::after(#fstar_params_decl)]
use crate::constants::#modpath::*;
```

where `fstar_params_decl` is a string `"let v_HACSPEC_PARAMS = Hacspec_ml_dsa.Parameters.v_ML_DSA_<id>_"`.

Replace with a Rust `const` (or `pub const`) injected per variant:

```rust
#[cfg(hax)]
pub(crate) const HACSPEC_PARAMS: hacspec_ml_dsa::parameters::MlDsaParams =
    hacspec_ml_dsa::parameters::ML_DSA_<id>;
```

The macro source for the const declaration token-stream lives next to
the existing `pub type #sk_ident = MLDSASigningKey<SIGNING_KEY_SIZE>;`
type-alias block at lines 95-98.  Use `format_ident!` for the per-id
spec-const name (`ML_DSA_44`, `ML_DSA_65`, `ML_DSA_87`).

This replaces the F*-string injection.  When hax extracts the
const, it will produce `let v_HACSPEC_PARAMS : ... = Hacspec_ml_dsa.Parameters.v_ML_DSA_<id>_`
in the corresponding F* module — same end state, but now the
identifier is a real Rust path that `cargo check` validates.

### Step 3 — rewrite the three ensures clauses as Rust

For each of `generate_key_pair`, `verify`, `sign` in
`libcrux-ml-dsa/src/ml_dsa_generic.rs`, replace the
`hax_lib::ensures(|r| fstar!(r#"..."#))` with a Rust closure that:

- Calls `hacspec_ml_dsa::ml_dsa::{keygen_internal, sign, verify}`
  directly with concrete generics.
- Compares the result with `is_ok()` (or `Seq.equal`-equivalent —
  hax should map slice equality to `==` in Rust).
- Uses `cfg(hax)` imports for the spec module.

Open question to resolve while implementing — verify hax handles
each cleanly:

1. Can a `hax_lib::ensures` closure call a `cfg(hax)`-only function?
   (Probably yes — the ensures is itself only used at hax-extract
   time, so the call site is also cfg(hax)-active.)
2. Threading the const-generic args (`K`, `L`, `PK_SIZE`, `SK_SIZE`,
   `C_TILDE_LEN`, `W1_BYTES`) into the spec call.  The impl
   functions don't currently take these as Rust generics — they come
   from `crate::constants::ml_dsa_<id>::*` re-exports.  In the
   macroized scope, `ROWS_IN_A`, `COLUMNS_IN_A`,
   `VERIFICATION_KEY_SIZE`, `SIGNING_KEY_SIZE`,
   `SIGNATURE_SIZE`, `COMMITMENT_VECTOR_SIZE`,
   `COMMITMENT_HASH_SIZE` are all in scope — pass them positionally
   into the spec call.
3. Cross-type comparisons.  Impl returns
   `Result<MLDSASignature<...>, SigningError>` for `sign` and
   `Result<(), VerificationError>` for `verify`.  Spec returns
   `Result<[u8; SIG_SIZE], MlDsaError>` and `Result<(),
   MlDsaError>`.  The `is_ok()` equivalence is what we want — Rust
   `result.is_ok() == spec_result.is_ok()` should compile and
   extract.  For `generate_key_pair` the impl mutates two slices
   while the spec returns a `(pk, sk)` array tuple; the current
   F* form uses `Seq.equal slice array` — in Rust this would want
   `signing_key == &spec_sk[..]` and similar.  Test that hax
   extracts this to `Seq.equal` (it should).

### Step 4 — verify

After each function's refactor (do them one at a time, not all
together):

```bash
cd libcrux-ml-dsa
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged   # may need a fresh snapshot first
cd proofs/fstar/extraction
make check/Libcrux_ml_dsa.Ml_dsa_generic.Ml_dsa_65_.fst   # tightest loop
# then 44 and 87
```

Each variant should still verify in ~8s.  If F* errors, the F*
output will tell you what the extracted ensures looks like — usually
the Rust-level form extracts to nearly-identical F* as the manual
form, so the error is most likely a missing `is_ok` import or a
type-coercion difference.

### Step 5 — clean up

Once all three ensures are Rust-level and verifying:

- Remove the `fstar_params_decl` string and the `hax_lib::fstar::after`
  attribute from `crates/utils/macros/src/lib.rs`.  The Rust const
  replaces it.
- Smoke-test: `cargo build --tests` clean; `cargo test --release --lib`
  20/20.
- Single commit (or three — one per function — your call): "ml-dsa:
  switch wiring ensures to Rust-level spec calls".

## Hard rules (from `~/.claude/projects/-Users-karthik-libcrux/memory/MEMORY.md`)

- **rlimit cap**: NEVER `--z3rlimit > 800` (or > 400 with `--split_queries always`).
- **Use fstar-mcp** for tight iteration when available; recreate session after each `make`.
- **NEVER bulk-delete `.checked` files**.  `hax.sh prove` / `make` handle stale incrementally.
- **Touch unchanged `.checked`** after `cargo hax extract` (`proofs/agent-status/touch-unchanged-checked.sh skip-unchanged`).
- **Develop locally, upstream specs once**.  This sprint stays in
  libcrux-ml-dsa + the macros crate; do NOT touch `specs/ml-dsa/`
  unless you find a new spec gap (and if you do, stop and surface it).
- **Avoid `Spec.MLKEM` references** in ml-dsa code; cite `Hacspec_ml_dsa.*` only.
- **No matrix array refactor** (per existing memory).
- **Don't add an ensures admit** — these functions already have
  body-level `hax_lib::fstar!("admit ()")`.  That stays.

## Operational notes

- Branch: `ml-dsa-proofs`, tip after this handoff: `ce324fdb7`.
- Don't push to origin.  User merges to main manually.
- Spec sanity check (run BEFORE touching the impl, to confirm tip
  `ce324fdb7` is clean):

  ```bash
  cd specs/ml-dsa && cargo build --tests
  cd specs/ml-dsa/proofs/fstar/extraction && make
  cd specs/ml-dsa && cargo test --test nistkats sign_verify --release
  ```

  Should produce: 3/3 NIST KATs pass, all F* modules verified clean,
  no errors.

## Don't pursue (out of scope for this sprint)

- Removing the body `admit ()` and proving panic-freedom or
  functional correctness.  That's a separate sprint per the
  ml_dsa_generic-status notes.
- Lifting `Ml_dsa_44/65/87` user-API wrappers out of `ADMIT_MODULES`
  (option 4 from `handoff-2026-05-01-spec-impl-hookup.md`).  Possible
  follow-up after this refactor lands.
- Tightening the spec's NTT-bound `assume`s (stretch goal in the
  earlier handoff).

## Decision points where you should stop and ask

- If hax extracts the Rust spec call to something materially
  different from the existing manual F* form (e.g. inlines the
  spec body, or fails to recognize the const-generic args), stop and
  describe the divergence rather than working around it.
- If `cfg(hax)` access to `hacspec_ml_dsa::ml_dsa::*` requires
  promoting items from `pub(crate)` to `pub` in `specs/ml-dsa/src/`,
  flag it — that's a spec-API change, not a wiring change.
- If the macro change to inject a Rust const fails because of
  re-export visibility issues
  (`hacspec_ml_dsa::parameters::ML_DSA_44` not reachable from the
  macroized module), flag it — the workaround would be to take the
  spec const path as a macro argument, which is more invasive.
