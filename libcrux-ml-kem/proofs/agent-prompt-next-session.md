# Agent prompt — libcrux-ml-kem-proofs, R11 mop-up + Pattern C scaling

Paste this into a fresh Claude Code session opened in
`~/libcrux-trait-opacify/libcrux-ml-kem` (auto mode recommended).

You are a single-lane agent for the libcrux-ml-kem F\* verification
effort.  The R11 surface migration (impl-side `fstar!(...)` →
pure-Rust citing `hacspec_ml_kem::*`) is **partially done**.  This
session's goal: scale the validated Pattern C across the remaining
unpacked-API functions, then mop up the last few packed-API
holdouts in `Ind_cpa.fsti`.

## Read first (in order)

  1. `proofs/agent-status/session-2026-05-01-impl-pure-rust.md` —
     the full session log of three pushes (pattern validation,
     Lane 1, Lane B).  Authoritative state at session start.
  2. `proofs/agent-status/spec-audit-2026-05-01.md` lines 56-99
     (the function-by-function audit table).  Note rows 6, 10, 18,
     34, 35, 37 are now **unblocked** by the P5 helpers landed in
     commits `ba0832a30` and `86f880bdc`.

## Branch state at session start

```
$ git log --oneline -5
aacfdc4d8 agent-mlkem: session report — append Lane B push (P5 helpers + Pattern C validation)
c5f16b76b agent-mlkem: ind_cca::unpacked::generate_keypair — pure-Rust ensures (Pattern C)
86f880bdc agent-mlkem: specs/ml-kem — add ind_cca::ind_cca_unpack_{generate_keypair,encapsulate,decapsulate} (P5)
ba0832a30 agent-mlkem: specs/ml-kem — add ind_cpa::{generate_keypair,encrypt,decrypt}_unpacked (P5)
260430b06 agent-mlkem: ind_cpa::sample_{ring_element_cbd,vector_cbd_then_ntt} — restore functional spec
```

Branch: `libcrux-ml-kem-proofs`.  Tip: `aacfdc4d8`.  16 commits
ahead of `origin/libcrux-ml-kem-proofs`.  Not yet pushed (user can
push at their discretion).

## R11 surface state

| File | `Spec.MLKEM` cites |
|---|---|
| `Libcrux_ml_kem.Ind_cca.fsti` | **0** (DONE) |
| `Libcrux_ml_kem.Ind_cca.Unpacked.fsti` | 46 (unpacked API + 1-2 holdouts) |
| `Libcrux_ml_kem.Ind_cpa.fsti` | 34 (unpacked API + 1-2 holdouts) |

13 functions migrated this far.  Three patterns validated:

  - **Pattern A** — value-returning function-call ensures
    (e.g. `match hacspec_ml_kem::encapsulate::<...>(...) { Ok((shared, ct)) => result.0.value == ct && result.1 == shared, Err(_) => true }`).
  - **Pattern B** — auxiliary-buffer ensures for slice-output impl
    functions (allocate `[0u8; max_size]`, call `_into` companion,
    assert slice equality on the prefix).
  - **Pattern C** — field-projection ensures for unpacked-API
    functions, citing the P5 helper tuples in `hacspec_ml_kem`.

## Spec state — what you have to call

### Re-exports at `hacspec_ml_kem` crate root (commit `c5f16b76b`)

```rust
pub use ind_cca::{
    decapsulate, encapsulate, generate_keypair,
    ind_cca_unpack_decapsulate, ind_cca_unpack_encapsulate,
    ind_cca_unpack_generate_keypair, public_key_modulus_check,
    IndCcaUnpackedKeyPair,
};
```

`hacspec_ml_kem::ind_cpa::*` is publicly accessible
(module is `pub mod ind_cpa`); `ind_cca` is private (use the
crate-root re-exports above).

### P5 unpacked-shape helpers (commits `ba0832a30`, `86f880bdc`)

#### `hacspec_ml_kem::ind_cpa::*`

```rust
pub type IndCpaKeypairUnpacked<const RANK: usize> =
    (Vector<RANK>, Vector<RANK>, Matrix<RANK>, [u8; 32]);
//   secret_as_ntt   t_as_ntt    A_as_ntt(raw)  seed_for_A

pub fn generate_keypair_unpacked<const RANK: usize>(
    params: &MlKemParams,
    key_generation_seed: &[u8],
) -> Result<IndCpaKeypairUnpacked<RANK>, BadRejectionSamplingRandomnessError>;

pub fn encrypt_unpacked<RANK, U_SIZE, V_SIZE, CT_SIZE>(
    params: &MlKemParams,
    t_as_ntt: &Vector<RANK>,
    A_as_ntt: &Matrix<RANK>,            // raw form (un-transposed)
    message: &[u8; 32],
    randomness: &[u8],                   // len() == 32
) -> Result<[u8; CT_SIZE], BadRejectionSamplingRandomnessError>;

pub fn decrypt_unpacked<const RANK: usize>(
    params: &MlKemParams,
    secret_as_ntt: &Vector<RANK>,
    ciphertext: &[u8],
) -> [u8; 32];
```

**Important convention**: `ind_cpa::*_unpacked` consume A in **raw**
form (`A_as_ntt[i][j] = sampled(i, j)`).  The libcrux impl stores A
in **transposed** form on `IndCpaPublicKeyUnpacked.A`
(`A_transposed[j][i] = sampled(i, j)`) — apply
`hacspec_ml_kem::matrix::transpose` before/after as needed.

#### `hacspec_ml_kem::ind_cca::*`

These wrap the `ind_cpa::*_unpacked` helpers with the libcrux-side
transpose convention already applied:

```rust
pub type IndCcaUnpackedKeyPair<const RANK: usize> = (
    Vector<RANK>,    // secret_as_ntt
    Vector<RANK>,    // t_as_ntt
    Matrix<RANK>,    // m_A (LIBCRUX-TRANSPOSED form)
    [u8; 32],        // seed_for_A
    [u8; 32],        // public_key_hash
    [u8; 32],        // implicit_rejection_value
);

pub fn ind_cca_unpack_generate_keypair<RANK, EK_SIZE>(
    params: &MlKemParams, randomness: &[u8; 64],
) -> Result<IndCcaUnpackedKeyPair<RANK>, _>;

pub fn ind_cca_unpack_encapsulate<RANK, U_SIZE, V_SIZE, CT_SIZE>(
    params: &MlKemParams,
    public_key_hash: &[u8; 32],
    t_as_ntt: &Vector<RANK>,
    m_A: &Matrix<RANK>,                  // LIBCRUX-TRANSPOSED form
    randomness: &[u8; 32],
) -> Result<([u8; 32], [u8; CT_SIZE]), _>;

pub fn ind_cca_unpack_decapsulate<RANK, U_SIZE, V_SIZE, CT_SIZE, J_INPUT_SIZE>(
    params: &MlKemParams,
    public_key_hash: &[u8; 32],
    implicit_rejection_value: &[u8; 32],
    ciphertext: &[u8; CT_SIZE],
    secret_as_ntt: &Vector<RANK>,
    t_as_ntt: &Vector<RANK>,
    m_A: &Matrix<RANK>,                  // LIBCRUX-TRANSPOSED form
) -> Result<[u8; 32], _>;
```

### CBD sampling helpers (commit `c8b54c62b`)

```rust
pub fn sample_vector_cbd<RANK>(eta: usize, seed: &[u8], domain_separator: u8) -> Vector<RANK>;
pub fn sample_vector_cbd_then_ntt<RANK>(eta: usize, seed: &[u8], domain_separator: u8) -> Vector<RANK>;
```

### Existing helpers (prior sessions — still in use)

  - `parameters::{is_rank, rank_to_params, cpa_*_size, c1_size, c2_size,
    eta1, eta2, eta1_randomness_size, eta2_randomness_size,
    vector_u_compression_factor, vector_v_compression_factor,
    c1_block_size, t_as_ntt_encoded_size, ranked_bytes_per_ring_element,
    cca_private_key_size, implicit_rejection_hash_input_size,
    SHARED_SECRET_SIZE, CPA_KEY_GENERATION_SEED_SIZE}`
  - `serialize::{serialize_secret_key_into, compress_then_serialize_u_into,
    serialize_public_key, vector_decode_12, byte_encode, byte_decode}`
  - `ind_cpa::{generate_keypair, encrypt, decrypt}` (packed)
  - `matrix::{sample_matrix_A, transpose, compute_*}`
  - `crate::polynomial::spec::{is_bounded_poly, is_bounded_polynomial_vector,
    is_bounded_polynomial_matrix}` (libcrux-side, not Hacspec)
  - `crate::vector::spec::{poly_to_spec, vector_to_spec, matrix_to_spec}`
    (libcrux-side, not Hacspec)

## Open work — priority order

### 1. Scale Pattern C across the unpacked API (5 functions)

Each is mechanical now that the P5 helpers exist.  Smallest first.

| # | Function | Helper to cite |
|---|---|---|
| 1.1 | `ind_cpa::generate_keypair_unpacked` (`ind_cpa.rs:463`) | `hacspec_ml_kem::ind_cpa::generate_keypair_unpacked` |
| 1.2 | `ind_cpa::encrypt_unpacked` (`ind_cpa.rs:728`) | `hacspec_ml_kem::ind_cpa::encrypt_unpacked` |
| 1.3 | `ind_cpa::decrypt_unpacked` (`ind_cpa.rs:1153`) | `hacspec_ml_kem::ind_cpa::decrypt_unpacked` |
| 1.4 | `ind_cca::unpacked::encapsulate` (`ind_cca.rs:948`) | `hacspec_ml_kem::ind_cca_unpack_encapsulate` |
| 1.5 | `ind_cca::unpacked::decapsulate` (`ind_cca.rs:1040`) | `hacspec_ml_kem::ind_cca_unpack_decapsulate` |

**Pattern template** (from `c5f16b76b`):

```rust
#[hax_lib::ensures(|()|  // or |result| / |(...)| depending on signature
    match hacspec_ml_kem::<helper>::<...>(<args>) {
        Ok((field0, field1, ..., fieldN)) => {
            // For each observable field on the libcrux post-state:
            <projection> == <hacspec_field>
            && <next>
            && ...
        }
        Err(_) => true,
    }
)]
```

**Note on `m_A` convention**: the libcrux impl's `IndCpaPublicKeyUnpacked.A`
is in transposed form, and the `ind_cca_unpack_*` helpers expect
m_A in transposed form too — so the projection is direct
`matrix_to_spec(future(out)...A) == m_A`, no extra transpose
in the ensures.

### 2. Mop up the last few `Spec.MLKEM` cites in `Ind_cpa.fsti`

After Lane B's pattern C scaling, what remains:

  - `build_unpacked_public_key{,_mut}` (audit row 14, 15) — has
    Hacspec `matrix::sample_matrix_A` analogue.  Cites
    `Spec.MLKEM.sample_matrix_A_ntt` and `Spec.MLKEM.vector_decode_12`;
    both have direct Hacspec equivalents (`matrix::sample_matrix_A`,
    `serialize::vector_decode_12`).  Mechanical.

  - `deserialize_then_decompress_u` (audit row 16) — needs a small
    spec helper `deserialize_then_decompress_u_then_ntt<RANK>(ciphertext, du)`
    composing the existing
    `serialize::deserialize_then_decompress_u` + `ntt::vector_ntt`.
    Add one helper, then mechanical migration.

  - `encrypt_c1`, `encrypt_c2` — internal per-step (audit recommends
    leave-as-is; they have no FIPS algorithm number).  Decide
    per-function whether to migrate or leave with `lax`+no-fn-spec.

### 3. Optional — push to origin

If satisfied with the state, `git push origin libcrux-ml-kem-proofs`
(no force push, no PR).  The branch already exists upstream from
prior sessions; this is a fast-forward push of 16-21 commits.

## Hard rules (R1-R11)

  R1  Branch `libcrux-ml-kem-proofs`.  May `git push` (fast-forward
      only).  DO NOT force-push, DO NOT push to `main`, DO NOT open
      a PR without explicit user authorization.
  R2  No new admits beyond existing `lax` / `ADMIT_MODULES` carry-overs.
  R3  No new axioms.  If absolutely necessary, file as SIDEWAYS.
  R4  `--z3rlimit ≤ 800` HARD CAP; `≤ 400/query` under
      `--split_queries always`.  Default tier `--z3rlimit 200`.
  R5  Inner edit-check: `make check/<Mod>.fst` from
      `proofs/fstar/extraction/`.  Cap iteration at 20 min/attempt.
  R6  After `python3 hax.py extract`: snapshot SHAs and touch unchanged
      `.checked` files (per `feedback_touch_unchanged_checked`).
  R7  Trait FROZEN — `src/vector/traits.rs`'s `Operations` /
      `Repr` definitions not edited.  The `spec` submodule below
      it MAY be edited.
  R8  No `fstar-mcp` (per `feedback_use_fstar_mcp` and
      `feedback_fstar_mcp_session_dies_after_make`).
  R9  Commit prefix `agent-mlkem:`.  Commit Rust-spec changes
      separately from libcrux-side changes.
  R10 No wrappers.  No namespace-squatting.  No new F\* specs in
      `Hacspec_ml_kem.*` (per the FORBIDDEN section of the
      clean-restart prompt).
  R11 **No `fstar!` escape in `src/ind_cpa.rs` / `src/ind_cca.rs`
      annotations.**  This is the goal of the R11 lane.  The two
      documented exceptions (`i16_to_spec_fe`, `mont_i16_to_spec_fe`
      ensures) live in `src/vector/traits.rs` — they are NOT touched.
      If you find yourself wanting `fstar!` in an ensures, capture the
      reason and ask the user before shipping it.

### Lessons carried forward from prior sessions

  - **Loop invariant for slice outputs**: `_into`-style functions
    writing to `&mut [u8]` need an explicit
    `hax_lib::loop_invariant!(|_i| out.len() == EXPECTED_SIZE)` to
    discharge per-iteration sub-slice index bounds.
  - **Bit-OR vs addition**: in bit-assembly loops, `coef |= 1u16 << j`
    resists Z3 bound proofs — rewrite to `coef += 1u16 << j` if the
    loop invariant guarantees the new bit isn't already set.
  - **`hacspec_ml_kem::ind_cca` is private** — use crate-root
    re-exports.  `hacspec_ml_kem::ind_cpa` is `pub mod`, so direct
    paths work for ind_cpa.
  - **Closure parameter is by-value**: in `|res|` ensures, use
    `res == ...` (not `*res == ...`) when `res` is a sized array.
  - **`t_as_ntt_*` → `tt_as_ntt_*`**: hax renames `t_*` Rust names
    to `tt_*` in F\* extraction (the `t_` prefix is reserved for
    type variables).  Just accept the renamed form when reading the
    extracted F\*.
  - **`.to_prop()` and `&` for Prop combinators**: when mixing a
    bool requires/ensures with a Prop helper like
    `is_bounded_polynomial_vector`, wrap the bool half in a single
    parenthesized expression `(...).to_prop()` and combine with `&`,
    not `&&`.  Requires `use hax_lib::prop::ToProp;` at module top.
  - **Match arm field names matter**: in `match Ok(tuple) => ...`,
    the projection sites must use the actual tuple field names from
    the Hacspec helper.  Watch for `_underscored` ignored fields
    (the F\* extraction renders these as plain `_`, but Rust is
    happier with `_descriptive_name` for readability).

## Workflow per migration

  1. Edit `src/<file>.rs` annotations to pure Rust.
  2. `python3 hax.py extract` from `libcrux-ml-kem/`.
  3. Snapshot SHAs and touch unchanged `.checked` files (R6).
  4. Verify the target function's `.fst[i]` rebuilds:
     `make /Users/karthik/libcrux-trait-opacify/.fstar-cache/checked/Libcrux_ml_kem.Ind_*.fsti.checked`
     from `proofs/fstar/extraction/`.
  5. Verify spec modules still build:
     `make /Users/karthik/libcrux-trait-opacify/.fstar-cache/checked/Hacspec_ml_kem.*.fst.checked`.
  6. Commit per function with `agent-mlkem:` prefix.

**Cap**: 5 functions or 4 hours, whichever first.

## End-of-session deliverable

Append a new section to
`proofs/agent-status/session-2026-05-01-impl-pure-rust.md` (or
create `session-<date>-<suffix>.md` if the prior log gets too long).
Include:

  - Functions migrated (file:line, Pattern, commit SHA).
  - New content in `/specs/ml-kem/src/` (if any).
  - F\* perf delta (cold vs warm, max rlimit used).
  - Final commit SHA + cite counts in `Ind_cpa.fsti`,
    `Ind_cca.fsti`, `Ind_cca.Unpacked.fsti`.
  - Pattern findings: any new gotchas?
  - **Self-audit (R10 + R11)**: any wrapper, any `unfold let` alias
    over `Spec.MLKEM`, any new `Hacspec_ml_kem.<top-level>.fst` file,
    any new `fstar!` escape in ind_cpa/ind_cca annotations?  If yes:
    revert.

## Out-of-scope for R11 lane (separate sprints)

  - **Lane 2 — body-tactic Spec.MLKEM elimination**: the migrated
    functions' body proofs still cite `Spec.MLKEM.*` inside
    `hax_lib::fstar!(...)` tactic blocks for `eq_intro` /
    `reveal_opaque` / `Classical.forall_intro`.  R11 explicitly
    targets the annotation surface; bodies need their own sprint.
  - **Hacspec_ml_kem.Commute.Chunk.fst:1046** — pre-existing
    failure (separate sprint).
  - **`encrypt_c1`/`encrypt_c2` migration** — internal-only
    functions, audit recommends leave-as-is.

DO NOT touch `~/libcrux-ml-dsa-proofs` or `~/libcrux-sha3-focused`.
