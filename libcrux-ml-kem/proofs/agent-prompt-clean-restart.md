# Agent prompt — clean-restart session for ML-KEM proofs

Paste this into a fresh Claude Code session opened in
`~/libcrux-trait-opacify/libcrux-ml-kem` (auto mode recommended).

You are a single-lane agent for the libcrux-ml-kem F\* verification effort.

**Read first**: `proofs/agent-status/AUDIT-2026-05-01.md` — the entire
2026-05-01 session was suspect (textual Spec.MLKEM removal via
namespace-squatting wrappers, no real progress).  The audit explains
the failure and the rules below derive from it.

## Your first action — set up a clean branch

The current branch `trait-opacify` carries 18 suspect commits from
2026-05-01.  Create a new branch off the pre-session state:

```bash
cd ~/libcrux-trait-opacify
# Verify the pre-session SHA is reachable
git log --oneline d28a79c26 -1
# Should print: d28a79c26 agent-mlkem: rewrite prompt for next session — Spec.MLKEM removal continuation

# New working branch
git checkout -b libcrux-ml-kem-proofs d28a79c26

# Cherry-pick the ONE legitimate commit from today's session (SD4 cleanup)
git cherry-pick 0be31f1a6
# Verify clean: only src/serialize.rs and src/ind_cpa.rs touched, no commute/ files added
git show --stat HEAD
```

Optionally cherry-pick (after content review):
  - `b3d3d7e5d` — `Spec.MLKEM.v_PRFxN` → `Spec.Utils.v_PRFxN` cosmetic move (the canonical location; doesn't depend on wrappers).
  - `b20b09862` + `daeffd891` + `7f549b318` — retrospective methodology docs; review whether the analysis is still valid given the audit.

DO NOT cherry-pick anything else from the 2026-05-01 session.  All
other commits add or extend hand-written `Hacspec_ml_kem.*` wrapper
modules under `specs/ml-kem/proofs/fstar/commute/`.

## Your second action — move Spec.MLKEM out of the include path

Spec.MLKEM cannot remain available as a fallback or it becomes too
tempting to wrapper-around-it.  Move the source files and update the
Makefile so any remaining `Spec.MLKEM.*` reference becomes a hard
error.

```bash
cd ~/libcrux-trait-opacify
mkdir -p libcrux-ml-kem/proofs/_DEPRECATED_spec_mlkem
git mv libcrux-ml-kem/proofs/fstar/spec/Spec.MLKEM.fst libcrux-ml-kem/proofs/_DEPRECATED_spec_mlkem/
git mv libcrux-ml-kem/proofs/fstar/spec/Spec.MLKEM.Math.fst libcrux-ml-kem/proofs/_DEPRECATED_spec_mlkem/
git mv libcrux-ml-kem/proofs/fstar/spec/Spec.MLKEM.Instances.fst libcrux-ml-kem/proofs/_DEPRECATED_spec_mlkem/
# Spec.Utils.fst stays — it has v_PRFxN, v_H, v_G which are in scope per cross-spec tests
```

Then run `make` from `libcrux-ml-kem/proofs/fstar/extraction/` to
catch all the now-broken references.  Every error is a place that
needs migration to the real Hacspec extracted modules.  EXPECT this
to fail loudly — that's the point.

## Anti-patterns and rules (BLOCKING)

### What's ALLOWED in `specs/ml-kem/proofs/fstar/commute/`

Files in this directory may contain:

  1. **Bridge lemmas** that connect the implementation (impl-side
     traits, lifted to spec-side via `to_spec_*_t`) to extracted
     `Hacspec_ml_kem.*` spec functions.  Example:
     `Hacspec_ml_kem.Commute.Bridges.lemma_inv_ntt_layer_2_step_lane_bridge`.
  2. **Intermediate definitions** that are scoping/opacity helpers,
     NOT new specifications.  Example:
     `Hacspec_ml_kem.ModQ.mod_q_eq` — opaque wrapper around `% 3329`
     so the trait surface doesn't leak raw modular arithmetic.
  3. Files MUST live in either:
     - the `Hacspec_ml_kem.Commute.*` sub-namespace (preferred for
       bridge lemmas), OR
     - a non-extracted-namespace name that does NOT shadow a real
       `Hacspec_ml_kem.<X>` extracted module.  See the list of
       reserved names below.

### What's FORBIDDEN

  1. **No `unfold let X = Spec.MLKEM.X`** wrappers under any
     namespace.  This is the namespace-squatting anti-pattern that
     created today's mess.  If your migration goal would be served
     by a wrapper, you are NOT making real progress.
  2. **No new F\* specifications under `Hacspec_ml_kem.*`.**  That
     namespace is reserved for code extracted by `hax` from
     `/specs/ml-kem/src/*.rs`.  If a function or constant is needed
     in the spec but doesn't exist, ADD IT TO THE RUST SPEC at
     `/specs/ml-kem/src/*.rs` and re-extract.
  3. **No new top-level `Hacspec_ml_kem.<X>.fst` files** in
     `commute/` for any X that already exists (or could exist) as
     an extracted module.  Reserved names — DO NOT CREATE these as
     hand-written files:
     ```
     Hacspec_ml_kem.Compress
     Hacspec_ml_kem.Ind_cca
     Hacspec_ml_kem.Ind_cpa
     Hacspec_ml_kem.Invert_ntt
     Hacspec_ml_kem.Matrix
     Hacspec_ml_kem.Ntt
     Hacspec_ml_kem.Parameters       (sub-namespaces also reserved: .Sizes, .Hash_functions, .X)
     Hacspec_ml_kem.Polynomial
     Hacspec_ml_kem.Sampling
     Hacspec_ml_kem.Serialize
     ```
  4. **No `Spec.MLKEM.*` references** in `src/*.rs` (Rust-side
     hax_lib::ensures/requires) or in `proofs/fstar/extraction/*`
     (extracted F*).  If `make` fails because Spec.MLKEM is moved,
     fix the reference by citing the real `Hacspec_ml_kem.<extracted>`
     symbol — do not work around it by adding a wrapper.
  5. **No new SMTPat-triggered equality lemmas** of the form
     `lemma_X_eq : Lemma (Hacspec.X == Spec.MLKEM.X) [SMTPat (Hacspec.X)]`.
     These are bridges to a spec that is being deleted; they create
     wrapper-by-equation, equivalent semantically to `unfold let`
     wrappers.

### Specifically about Hacspec_ml_kem.Parameters.Sizes

This file (`a65ab3e43`, prior session) lives in
`Hacspec_ml_kem.Parameters.Sizes` — it squats the `Parameters.*`
sub-namespace.  Its bodies define `is_rank`, `v_ETA1`,
`v_RANKED_BYTES_PER_RING_ELEMENT`, etc. — all of which already exist
in extracted `Hacspec_ml_kem.Parameters` as either constants or
`t_MlKemParams` methods.

**Decide first**: do we keep this module (and the prior session's
work that depends on it), or remove it?  If keeping, document the
exception and do not extend it.  If removing, plan the migration to
extracted `Hacspec_ml_kem.Parameters.{t_MlKemParams, impl_MlKemParams__*,
v_ML_KEM_*}` symbols before deleting.

### What's allowed re: opacity

`[@@ "opaque_to_smt"]` is an SMT-visibility attribute, NOT an admit.
Use it freely to scope what Z3 unfolds.  An `opaque_to_smt` function
is still fully verified by F\*; only its body is hidden from Z3
callers.  This is a perf tool, not a proof shortcut.

### What's allowed re: verification status

Three real states (per `feedback_panic_free_vs_lax`):
  - `verification_status(lax)` — admits everything (body + ensures).  Blanket admit.
  - `verification_status(panic_free)` — body verified (callee preconditions met, no panic), ensures admitted.  **Real intermediate step.**
  - (no marker) — full SMT.

For the user's 3-stage proof process:
  1. Top-down strengthen ensures everywhere needed.
  2. Mark all newly-strengthened fns `panic_free` — bodies still safety-check, ensures admitted, callers see strong contract.
  3. Top-down un-`panic_free` working from callers down to the leaves where real proof work lives.

## Migration mapping — Spec.MLKEM → real Hacspec_ml_kem

The cross-spec tests at `libcrux-ml-kem/tests/cross_spec.rs` are the
authoritative roadmap.  They exercise the impl-vs-spec correspondence
at byte level for keygen/encapsulate/decapsulate.

Selected mappings (reverse-engineered from cross_spec.rs and the
extracted `Hacspec_ml_kem.*` surface):

| Spec.MLKEM symbol | Real Hacspec_ml_kem symbol |
|---|---|
| `Spec.MLKEM.is_rank` | implicit in `Hacspec_ml_kem.Parameters.t_MlKemParams.f_rank` refinement |
| `Spec.MLKEM.v_ETA1 r` | `(p:t_MlKemParams).f_eta1` for the matching params |
| `Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE r` | `Hacspec_ml_kem.Parameters.impl_MlKemParams__ek_size p` |
| `Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE r` | `Hacspec_ml_kem.Parameters.impl_MlKemParams__dk_size p` |
| `Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE r` | `Hacspec_ml_kem.Parameters.impl_MlKemParams__ciphertext_size p` |
| `Spec.MLKEM.v_C1_SIZE r` | `Hacspec_ml_kem.Parameters.impl_MlKemParams__u_encoded_size p` |
| `Spec.MLKEM.v_C2_SIZE r` | `Hacspec_ml_kem.Parameters.impl_MlKemParams__vv_encoded_size p` |
| `Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE r` | `Hacspec_ml_kem.Parameters.impl_MlKemParams__tt_as_ntt_encoded_size p` |
| `Spec.MLKEM.byte_encode d p` | `Hacspec_ml_kem.Serialize.byte_encode <N> <D> p d` (with size constants) |
| `Spec.MLKEM.byte_decode d a` | `Hacspec_ml_kem.Serialize.byte_decode <N> <D> a d` |
| `Spec.MLKEM.compress_then_encode_message p` | `Hacspec_ml_kem.Serialize.compress_then_serialize_message p` (or equivalent) |
| `Spec.MLKEM.decode_then_decompress_message b` | `Hacspec_ml_kem.Serialize.deserialize_then_decompress_message b` |
| `Spec.MLKEM.vector_encode_12 v` | composition: `flatten (map_array (Hacspec_ml_kem.Serialize.byte_encode 12) v)` — NO direct equivalent; consumer should use `byte_encode` per-poly + flatten |
| `Spec.MLKEM.poly_ntt p` | `Hacspec_ml_kem.Ntt.???` — find the actual extracted symbol, may be `vector_ntt [p].[0]` |
| `Spec.MLKEM.poly_inv_ntt p` | `Hacspec_ml_kem.Invert_ntt.ntt_inverse p` |
| `Spec.MLKEM.vector_ntt v` | `Hacspec_ml_kem.Ntt.vector_ntt v` |
| `Spec.MLKEM.vector_inv_ntt v` | `Hacspec_ml_kem.Invert_ntt.vector_inverse_ntt v` |
| `Spec.MLKEM.compute_message v s u` | `Hacspec_ml_kem.Matrix.compute_message v s u` (real extracted!) |
| `Spec.MLKEM.sample_matrix_A_ntt seed` | `Hacspec_ml_kem.Matrix.sample_matrix_A v_RANK seed transpose` |
| `Spec.MLKEM.sample_poly_cbd eta input` | `Hacspec_ml_kem.Sampling.sample_poly_cbd v_ETA64 v_ETA512 eta input` |
| `Spec.MLKEM.ind_cpa_generate_keypair r seed` | `Hacspec_ml_kem.Ind_cpa.generate_keypair p seed` |
| `Spec.MLKEM.ind_cpa_encrypt r ek msg rand` | `Hacspec_ml_kem.Ind_cpa.encrypt p ek msg rand` |
| `Spec.MLKEM.ind_cpa_decrypt r dk ct` | `Hacspec_ml_kem.Ind_cpa.decrypt p dk ct` |
| `Spec.MLKEM.ind_cca_generate_keypair r rand` | `Hacspec_ml_kem.Ind_cca.generate_keypair p rand` |
| `Spec.MLKEM.ind_cca_encapsulate r ek rand` | `Hacspec_ml_kem.Ind_cca.encapsulate p ek rand` |
| `Spec.MLKEM.ind_cca_decapsulate r dk ct` | `Hacspec_ml_kem.Ind_cca.decapsulate p dk ct` |
| `Spec.MLKEM.Instances.mlkem768_generate_keypair r` | `Hacspec_ml_kem.Ind_cca.generate_keypair v_ML_KEM_768_ r` |
| `Spec.MLKEM.compress_then_encode_u r v` | composition over `Hacspec_ml_kem.Compress.compress_d` + `Hacspec_ml_kem.Serialize.byte_encode` |

The shapes don't match 1:1.  In particular, the real Hacspec uses a
`t_MlKemParams` struct passed by value instead of rank-parameterized
top-level functions.  Consumers in `src/*.rs` will need to either:

  - thread the `t_MlKemParams` value through their requires/ensures, OR
  - cite the pre-computed instance constants `v_ML_KEM_768_EK_SIZE` etc. for fixed-rank functions.

## Hard rules (R1-R10)

  R1 New branch: `libcrux-ml-kem-proofs`.  Do NOT push to origin.
  R2 No new admits beyond the existing `lax`/`ADMIT_MODULES` carry-overs.
  R3 No new axioms.  If absolutely necessary, file as SIDEWAYS.
  R4 `--z3rlimit ≤ 800` HARD CAP; `≤ 400/query` under `--split_queries always`.  Default tier `--z3rlimit 200`.
  R5 Inner edit-check: `make check/<Mod>.fst` from
     `proofs/fstar/extraction/`.  Cap iteration at 20 min/attempt.
  R6 After `python3 hax.py extract`: snapshot SHAs and touch unchanged
     `.checked` files (per `feedback_touch_unchanged_checked`).
  R7 Trait FROZEN — `src/vector/traits.rs` not edited.
  R8 No `fstar-mcp` (per `feedback_use_fstar_mcp` and
     `feedback_fstar_mcp_session_dies_after_make`).
  R9 After each milestone: regenerate `proofs/ml_kem_verification_status.md`,
     update `proof_milestones.md`, commit prefix `agent-mlkem:`.
  R10 **No wrappers.  No namespace-squatting.  No new F* specs in
      Hacspec_ml_kem.*** (per the FORBIDDEN section above).  If your
      migration would benefit from one, you are not solving the right
      problem.

## Workflow

  1. Set up the branch + cherry-pick + Spec.MLKEM relocation (above).
  2. Run `make` from `libcrux-ml-kem/proofs/fstar/extraction/` — see
     all the broken Spec.MLKEM references.
  3. Pick ONE function in `src/*.rs` that has a Spec.MLKEM reference.
     Migrate its requires/ensures to cite the real `Hacspec_ml_kem.*`
     extracted symbol from the mapping table.  Re-extract, verify.
     Commit.  Move to next.
  4. Where a real Hacspec symbol is missing, ADD it to
     `/specs/ml-kem/src/*.rs` (Rust spec), re-extract the spec, then
     re-extract Libcrux.  Commit the Rust-spec change separately.
  5. Cap: 4-5 functions or 4 hours.

## End-of-session deliverable

Report at `proofs/agent-status/session-<date>.md`:
  - Functions migrated (file:line, before/after pre/post snippet).
  - Any new content added to `/specs/ml-kem/src/` (Rust spec) — list
    the public-API additions with rationale.
  - F\* perf delta on affected modules.
  - Final commit SHA.
  - Spec.MLKEM hit count (must be 0 in `src/`; commute/ count
    monotonically decreasing).
  - **Self-audit**: did you create any wrapper, any unfold-let alias
    over Spec.MLKEM, any new Hacspec_ml_kem.<top-level>.fst file?
    If yes, you violated R10; revert.

DO NOT push to origin.  DO NOT touch `~/libcrux-ml-dsa-proofs` or
`~/libcrux-sha3-focused`.
