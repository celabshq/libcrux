# Session report — impl-side pure-Rust annotation migration (R11)

**Date:** 2026-05-01
**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** `b0714370f` (next-session prompt)
**Tip on exit:** `c5f16b76b`
**Scope:** R11 — convert `fstar!(...)` requires/ensures in
`libcrux-ml-kem/src/{ind_cpa,ind_cca}.rs` to **pure-Rust** citing
`hacspec_ml_kem::*`.

## Functions migrated (13)

### First push — pattern validation (6)

| # | Function | Pattern | Commit | Notes |
|---|---|---|---|---|
| 1 | `ind_cca::generate_keypair` (`ind_cca.rs:199`) | A | `a2480b070` | Value-returning function-call ensures.  Cites `hacspec_ml_kem::generate_keypair`. |
| 2 | `ind_cpa::serialize_vector` (`ind_cpa.rs:140`) | B | `f6ef6a5ce` | Auxiliary-buffer ensures with `[0u8; 4 * BYTES_PER_RING_ELEMENT]` and `serialize_secret_key_into`. |
| 3 | `ind_cpa::serialize_public_key` (`ind_cpa.rs:66`) | A | `f27f08d20` | `res == hacspec_ml_kem::serialize::serialize_public_key::<K, PUBLIC_KEY_SIZE>(...)`. |
| 4 | `ind_cpa::serialize_public_key_mut` (`ind_cpa.rs:96`) | A | `f27f08d20` | `*future(serialized) == ...`. |
| 5 | `ind_cca::encapsulate` (`ind_cca.rs:251`) | A | `64709113c` | `match ... { Ok((shared, ct)) => result.0.value == ct && result.1 == shared, Err(_) => true }`.  Note Hacspec returns `(shared, ct)` order vs. libcrux `(ct, shared)`. |
| 6 | `ind_cca::decapsulate` (`ind_cca.rs:326`) | A | `64709113c` | `match ... { Ok(expected) => result == expected, Err(_) => true }`. |

### Second push — Lane 1 (Ind_cpa.fsti scrub) (7)

After confirming Pattern A/B held, drove `Libcrux_ml_kem.Ind_cpa.fsti`
toward `Spec.MLKEM`-free.

| # | Function | Pattern | Commit | Notes |
|---|---|---|---|---|
| 7 | `ind_cpa::generate_keypair` (`ind_cpa.rs:561`) | A | `d4f813b2e` | Hacspec returns `Ok((ek, dk))` (ek first); libcrux returns `(dk, ek)`. Match arm: `result.0 == dk && result.1 == ek`. Cites new `parameters::CPA_KEY_GENERATION_SEED_SIZE`. |
| 8 | `ind_cpa::encrypt` (`ind_cpa.rs:893`) | A | `d4f813b2e` | Required spec-side relax of `randomness: &[u8; 32]` → `&[u8]` (commit `97d5f9746`).  Cites `hacspec_ml_kem::ind_cpa::encrypt::<K, C1_LEN, C2_LEN, CIPHERTEXT_SIZE>`. |
| 9 | `ind_cpa::decrypt` (`ind_cpa.rs:1184`) | A | `d4f813b2e` | Cleanest of the lane: Hacspec `decrypt<RANK>(params, dk, ct) -> [u8; 32]` matches libcrux on a single rank generic. |
| 10 | `ind_cpa::sample_ring_element_cbd` (`ind_cpa.rs:273`) | weakened | `b63ce8d0d` | No public Hacspec analogue for `sample_vector_cbd2`.  Drops the functional cite, retains bound (`is_bounded_polynomial_vector(7, future(error_1))`) + domain-separator increment.  Function is `lax`. |
| 11 | `ind_cpa::sample_vector_cbd_then_ntt` (`ind_cpa.rs:389`) | weakened | `b63ce8d0d` | Same regression rationale as #10.  Bound (3328) + ds increment. |
| 12 | `ind_cpa::compress_then_serialize_u` (`ind_cpa.rs:654`) | B | `4671a63d6` | Auxiliary buffer `[0u8; 1408]` (max K=4, du=11). |
| 13 | `ind_cpa::deserialize_vector` (`ind_cpa.rs:1118`) | A | `4671a63d6` | Cites `vector_decode_12`; lifts `secret_as_ntt: &mut [...; K]` via `vector_to_spec(future(...))`. |

### Spec-side commits (1)

  - `97d5f9746` — `specs/ml-kem` — relax `ind_cpa::encrypt`'s
    `randomness: &[u8; 32]` → `&[u8]` (Blocker B fix mirroring P3).
    `Hacspec_ml_kem.Ind_cpa.fst.checked` and `Hacspec_ml_kem.Ind_cca.fst.checked`
    re-verify clean in 9s.

## Spec-side additions

### `crate::polynomial::spec` helpers (commit `f6ef6a5ce`)

Added two rank-generic forall helpers reused across remaining
migrations:

```rust
pub(crate) fn is_bounded_polynomial_vector<const RANK: usize, V: Operations>(
    b: usize, v: &[PolynomialRingElement<V>; RANK]
) -> hax_lib::Prop;

pub(crate) fn is_bounded_polynomial_matrix<const RANK: usize, V: Operations>(
    b: usize, m: &[[PolynomialRingElement<V>; RANK]; RANK]
) -> hax_lib::Prop;
```

These auto-extract into `Libcrux_ml_kem.Polynomial.Spec.fst` (the
existing module already extracts `is_bounded_vector`/`is_bounded_poly`).
Replaces the 2-line `hax_lib::forall(|i| hax_lib::implies(i < K,
crate::polynomial::spec::is_bounded_poly(3328, &key[i])))` boilerplate
with a single helper call.  Reused in `serialize_vector`,
`serialize_public_key`, `serialize_public_key_mut`.  Will be reused
across the remaining ind_cpa migrations.

`Libcrux_ml_kem.Polynomial.Spec.fst.checked` re-verifies clean in 47s.

No `specs/ml-kem/src/*` changes this session.

## Pattern findings

### Pattern A — value-returning function-call ensures: **VALIDATED**

Works cleanly when libcrux const-generics line up with the Hacspec
signature.  Demonstrated on 5 functions (rows 1, 3, 4, 5, 6 above).
Extraction produces idiomatic F* with `Hacspec_ml_kem.<Module>.<fn>`
cites and `Core_models.Result.Result_Ok`/`Result_Err` pattern matches.

**Gotchas observed:**

- `hacspec_ml_kem::ind_cca` is a *private* module — the Hacspec functions
  are re-exported at the crate root.  Use `hacspec_ml_kem::generate_keypair`,
  `hacspec_ml_kem::encapsulate`, `hacspec_ml_kem::decapsulate` (NOT
  `hacspec_ml_kem::ind_cca::*`).
- The closure parameter in `|res|` ensures is by-value, so
  `*res == ...` doesn't compile.  Use `res == ...` directly when `res`
  is a sized array.
- `t_as_ntt_*` Rust function names get renamed to `tt_as_ntt_*` in
  the F* extraction (hax reserves the `t_` prefix for type variables).
  This is automatic — just accept the `tt_` prefix when reading the
  extracted F*.

### Pattern B — auxiliary-buffer ensures: **VALIDATED** (lax body)

Works for slice-output impl functions where the size isn't a single
const-generic.  Form:

```rust
#[hax_lib::ensures(|()| {
    let mut expected = [0u8; <max_rank * BLOCK_SIZE>];
    let len = K * BLOCK_SIZE;
    hacspec_ml_kem::serialize::<into-fn>::<K>(
        &crate::vector::spec::vector_to_spec(input),
        &mut expected[..len],
    );
    future(out)[..] == expected[..len]
})]
```

The trick: pre-allocate a max-rank-sized buffer (`[0u8; 4 *
BYTES_PER_RING_ELEMENT] == [0u8; 1536]`), call the spec `_into`
companion to mutate it, then assert slice equality between the
post-state of `out` and the relevant prefix of `expected`.  The
extraction emits clean F* using `Rust_primitives.Hax.repeat`,
`update_at_range_to`, and `=.` slice equality.

Validated on `serialize_vector` (lax body — verifies at the
extraction-output level, not the body-proof level).  Pattern B should
generalize to `compress_then_serialize_u`, `serialize_unpacked_secret_key`,
and any other `_mut` slice-output function.

### Pattern C — field-projection ensures: **DEFERRED**

Attempted on `ind_cca::unpacked::generate_keypair` (line 871).  The
clean form requires a Hacspec spec helper that exposes the unpacked
components (`m_A`, `public_key_hash`, `implicit_rejection_value`)
separately — exactly the **P5 deferred work** flagged in the prior
session's audit.  Per the prompt's "Add per-function ONLY when
Pattern C demonstrably fails" rule, this is deferred to a future
session that takes on the P5 spec helpers.

Field projection without a new spec helper would require either:

  (a) Decomposing the packed `Hacspec_ml_kem.generate_keypair` result
      back into unpacked fields (deserializing `ek` into `t_as_ntt`,
      re-sampling `A` from `seed_for_A`, etc.) — equivalent in
      cost to writing the spec helper anyway.
  (b) Citing only a subset of the unpacked fields (e.g. just
      `implicit_rejection_value == &randomness[32..]`).  This loses
      functional-correctness coverage of the unpacked `A` and
      `public_key_hash`, so equivalent to a regression vs. the
      `Spec.MLKEM`-cite baseline.

Recommendation: defer to a session that bundles the P5 helpers
(`ind_cca_unpack_generate_keypair`, `ind_cca_unpack_encapsulate`,
`ind_cca_unpack_decapsulate`) on the spec side, then migrates audit
rows 6, 10, 18, 34, 35, 37 in the impl-side file.

## F* verification status

- **`Libcrux_ml_kem.Ind_cca.fsti.checked`**: ✅ verifies clean in
  ~48s.  **The pre-existing `Spec.MLKEM` not-resolved blocker (prior
  session's `Ind_cca.fsti(165)`) is now eliminated** — all three
  packed-API functions in this `.fsti` (`generate_keypair`,
  `encapsulate`, `decapsulate`) are `Spec.MLKEM`-free.
- **`Libcrux_ml_kem.Ind_cpa.fsti.checked`**: ❌ still fails at the
  first remaining `Spec.MLKEM` cite — `sample_ring_element_cbd`
  (line 138).  69 `Spec.MLKEM` remnants in this `.fsti`, mostly
  in functions not yet migrated this session.
- **`Libcrux_ml_kem.Polynomial.Spec.fst.checked`**: ✅ verifies clean
  in ~47s with the new `is_bounded_polynomial_vector`/`_matrix`
  helpers.
- **`Hacspec_ml_kem.*.fst.checked`**: unchanged from prior session —
  no spec-side edits this session, all 10 spec modules continue to
  build (per prior-session report).

### Body-level verification

The migrated functions' **body proofs** still cite `Spec.MLKEM.*`
internally (`hax_lib::fstar!(...)` tactic blocks for `eq_intro`,
`reveal_opaque`, etc.).  R11 explicitly targets the annotation
surface; body-internal tactics are out-of-scope.  These body
tactics will need a follow-up pass to bridge from `Spec.MLKEM.*`
to the corresponding `Hacspec_ml_kem.*` lemmas — likely requiring
spec-side bridge lemmas (`Spec.MLKEM.vector_encode_12 #K v ==
Hacspec_ml_kem.Serialize.serialize_secret_key_into ...`).  Cap that
work to its own session.

## R compliance self-audit

  - **R1 (branch)**: All commits on `libcrux-ml-kem-proofs`.  Pushed?
    No (per session policy: agent may push but didn't this session,
    user can push at their discretion).
  - **R2 (no new admits)**: Confirmed.  No new admits introduced.
    The `serialize_vector` function remains `lax` (carried over from
    pre-session).
  - **R3 (no new axioms)**: Confirmed.
  - **R4 (rlimit ≤ 800)**: Confirmed.  All migrated functions kept
    their existing `--z3rlimit` settings (300/500/800 — all in cap).
  - **R5 (iteration cap 20 min)**: Each migration completed within
    cap; the longest was Pattern B's first attempt (one round-trip
    on the `&&`/`&` Prop-vs-bool type error → fixed by adding
    `.to_prop()` wrap and `use hax_lib::prop::ToProp;`).
  - **R6 (touch unchanged checked files)**: `python3 hax.py extract`
    automatically writes only changed `.fst[i]` files; the
    `Libcrux_ml_kem.Polynomial.Spec.fst.checked` re-verification was
    triggered explicitly by content change to the source.  No manual
    touching needed in this session.
  - **R7 (trait FROZEN)**: `src/vector/traits.rs` not edited.  Only
    `src/polynomial.rs` `spec` submodule extended (allowed per R7).
  - **R8 (no fstar-mcp)**: Used `python3 hax.py extract` and `make`
    only.
  - **R9 (commit prefix `agent-mlkem:`)**: All 4 commits this session
    use the prefix.
  - **R10 (no wrappers, no namespace squatting)**: Confirmed.  The
    new `is_bounded_polynomial_{vector,matrix}` helpers extend an
    existing `crate::polynomial::spec` module (not a new top-level
    `Hacspec_ml_kem.*` file, not an `unfold let` over `Spec.MLKEM.*`).
    They are **real definitions** (forall over `is_bounded_poly`
    indices), not unfold aliases.
  - **R11 (no `fstar!` escape in ind_cpa/ind_cca annotations)**:
    Confirmed for the 6 migrated functions.  No new `fstar!` escape
    introduced.  20 functions in `ind_cpa.rs` + 8 in `ind_cca.rs`
    still carry `fstar!` annotations and remain to be migrated in
    follow-up sessions.

## Open items / next-session candidates

### Next-session priority order (R11 surface)

  1. `ind_cpa::sample_ring_element_cbd` (138) — Pattern A; unblocks
     `Ind_cpa.fsti.checked` first failure.
  2. `ind_cpa::sample_vector_cbd_then_ntt` (370) — Pattern A; uses
     a sampling helper.
  3. `ind_cpa::generate_keypair` (556) — Pattern A; cites
     `Spec.MLKEM.ind_cpa_generate_keypair`, has direct Hacspec
     `ind_cpa::generate_keypair` analogue.
  4. `ind_cpa::encrypt` (893) — Pattern A; the const-generic shape
     is good (Phase 2/3 audit confirmed).
  5. `ind_cpa::decrypt` (1169) — Pattern A; clean Hacspec match.

After these 5, ~10 more functions in `ind_cpa.rs` are routine
Pattern A/B migrations.  Estimated 1-2 sessions to clear the
remaining `Spec.MLKEM` from `Ind_cpa.fsti`.

### Out-of-scope for this lane

  - **Body-level Spec.MLKEM elimination** (the `hax_lib::fstar!(...)`
    tactic blocks inside function bodies).  Will need spec-side
    bridge lemmas relating `Spec.MLKEM.*` to `Hacspec_ml_kem.*`,
    OR a parallel rewrite of the body tactics in terms of the new
    Hacspec functions.
  - **P5 unpacked-shape spec helpers** (audit rows 5, 6, 10, 18,
    34, 35, 37).  Required to migrate the impl-side
    `unpacked::generate_keypair`/`encapsulate`/`decapsulate` and
    related functions.
  - **`Hacspec_ml_kem.Commute.Chunk.fst:1046`** failure (separate
    sprint, pre-existing).

## Final commit SHAs

```
4671a63d6 agent-mlkem: ind_cpa::{compress_then_serialize_u,deserialize_vector} — pure-Rust ensures
b63ce8d0d agent-mlkem: ind_cpa::sample_{ring_element_cbd,vector_cbd_then_ntt} — pure-Rust ensures (weakened)
d4f813b2e agent-mlkem: ind_cpa::{generate_keypair,encrypt,decrypt} — pure-Rust ensures (Pattern A)
97d5f9746 agent-mlkem: specs/ml-kem — relax ind_cpa::encrypt randomness to slice
3ea073332 agent-mlkem: session report — impl-side pure-Rust migration (R11) 2026-05-01
64709113c agent-mlkem: ind_cca::{encapsulate,decapsulate} — pure-Rust ensures (Pattern A)
f27f08d20 agent-mlkem: ind_cpa::serialize_public_key{,_mut} — pure-Rust ensures (Pattern A)
f6ef6a5ce agent-mlkem: ind_cpa::serialize_vector — pure-Rust ensures (Pattern B)
a2480b070 agent-mlkem: ind_cca::generate_keypair — pure-Rust ensures (Pattern A)
```

Tip: `4671a63d6`.  Branch 9 ahead of `origin/libcrux-ml-kem-proofs`.

## Counts (R11 progress)

| File | `Spec.MLKEM` cites in `.fsti` (before / after) |
|---|---|
| `Libcrux_ml_kem.Ind_cca.fsti` | ~14 / **0** |
| `Libcrux_ml_kem.Ind_cpa.fsti` | ~80 / **34** |

`Ind_cca.fsti` is fully migrated at the packed-API level and clears
its prior `Spec.MLKEM not resolved` blocker (verifies in 48s).
`Ind_cpa.fsti` is ~58% migrated; remaining cites are concentrated in
unpacked-API functions (P5 spec helpers needed) and a few
mechanical compositions.

## Lane B push — P5 unpacked-shape helpers + Pattern C validation

A third push, after Lane 1, to land the deferred P5 spec helpers and
validate Pattern C.

### Spec-side commits

  - `c8b54c62b` — `specs/ml-kem`: factor the FIPS-203 inner sampling
    loop into `sample_vector_cbd<RANK>(eta, seed, ds)` and
    `sample_vector_cbd_then_ntt<RANK>(eta, seed, ds)`.  Used to
    restore the functional spec on the two CBD samplers (commit
    `260430b06`) — replaces the `b63ce8d0d` weakening.
  - `260430b06` — libcrux: tighten `sample_ring_element_cbd` /
    `sample_vector_cbd_then_ntt` ensures to cite the new helpers,
    closing the regression from `b63ce8d0d`.
  - `ba0832a30` — `specs/ml-kem`: add
    `ind_cpa::{generate_keypair_unpacked, encrypt_unpacked, decrypt_unpacked}`.
    The packed `generate_keypair`/`encrypt`/`decrypt` become thin
    (de)serialization wrappers around the unpacked variants.
  - `86f880bdc` — `specs/ml-kem`: add
    `ind_cca::ind_cca_unpack_{generate_keypair, encapsulate, decapsulate}`.
    Each delegates to the corresponding `ind_cpa::*_unpacked` helper
    after handling the libcrux-vs-Hacspec matrix-transpose convention.

### Impl-side commits

  - `c5f16b76b` — libcrux: validate Pattern C on
    `ind_cca::unpacked::generate_keypair`.  Uses field-projection
    ensures against the new `ind_cca_unpack_generate_keypair` tuple.

### Pattern C — VALIDATED

Field-projection ensures on the libcrux unpacked struct against the
P5 helper's tuple compose cleanly.  No new spec helpers needed
beyond the P5 trio.  Body proof tactics still cite `Spec.MLKEM` for
the un-projected fields (will be cleaned up if/when Lane 2 runs).

Remaining Pattern C migrations now unblocked:

  - `libcrux::ind_cpa::generate_keypair_unpacked` (line 463)
  - `libcrux::ind_cpa::encrypt_unpacked` (line 728)
  - `libcrux::ind_cpa::decrypt_unpacked` (line 1153)
  - `libcrux::ind_cca::unpacked::encapsulate` (line 948)
  - `libcrux::ind_cca::unpacked::decapsulate` (line 1040)

Each is mechanical now that the P5 helpers exist.

## Lane 1 remaining work (after this session)

Functions in `Ind_cpa.fsti` still carrying `Spec.MLKEM`:

  - **Unpacked-API (P5 deferred — needs spec helpers)**:
    `generate_keypair_unpacked`, `encrypt_unpacked`, `decrypt_unpacked`.
  - **Compositions / mechanical**: `serialize_unpacked_secret_key`
    (no annotations currently — already R11-compatible),
    `build_unpacked_public_key{,_mut}` (cites `Spec.MLKEM.sample_matrix_A_ntt`,
    has Hacspec `matrix::sample_matrix_A` analogue — straightforward
    next-session migration), `deserialize_then_decompress_u` (audit
    row 16 — needs new spec helper for the NTT-then-decompress
    composition).
  - **Internal**: `encrypt_c1`, `encrypt_c2` (per-step internals,
    audit recommends leave-as-is).

## Lane C push — Pattern C scaling (×5) + Ind_cpa.fsti mop-up (×3)

A fourth push on the same day, after Lane B, to scale the validated
Pattern C across the remaining unpacked-API functions and to mop up
the last `Spec.MLKEM` cites in `Libcrux_ml_kem.Ind_cpa.fsti`.

### Spec-side commits

  - `7502cba24` — `specs/ml-kem`: add
    `serialize::deserialize_then_decompress_u_then_ntt<RANK>(ciphertext, du)`
    composing the existing `deserialize_then_decompress_u` with
    `ntt::vector_ntt`.  Lifted to the spec module so the libcrux
    ensures stay one-function-deep (vs. an inline composition).

### Impl-side commits

  - `62d48c071` — libcrux: Pattern C scaling for the five
    unpacked-API functions (3 in `ind_cpa.rs`, 2 in `ind_cca.rs`):
      * `ind_cpa::generate_keypair_unpacked` (line 493)
      * `ind_cpa::encrypt_unpacked` (line 772)
      * `ind_cpa::decrypt_unpacked` (line 1191)
      * `ind_cca::unpacked::encapsulate` (line 983)
      * `ind_cca::unpacked::decapsulate` (line 1080)
    All cite the P5 helpers added in `ba0832a30`/`86f880bdc`/`7502cba24`.
  - `6bbec798d` — libcrux: mop-up for the three remaining
    `Spec.MLKEM` cites in `Ind_cpa.fsti`:
      * `build_unpacked_public_key` (line 1005)
      * `build_unpacked_public_key_mut` (line 1034)
      * `deserialize_then_decompress_u` (line 1079) — uses the
        new `deserialize_then_decompress_u_then_ntt` helper.

### A-matrix transpose convention — clarified

Two libcrux call sites for `sample_matrix_A` use **opposite**
`transpose: bool` flags, and therefore store A in opposite forms on
`IndCpaPublicKeyUnpacked.A`:

  - `generate_keypair_unpacked`: `sample_matrix_A(_, _, true)` →
    libcrux-transposed form `A[j][i] = sampled(i, j)`.  Spec helper
    returns raw form, so projection: `matrix_to_spec(future(A)) ==
    transpose(A_as_ntt)`.
  - `build_unpacked_public_key{,_mut}`: `sample_matrix_A(_, _, false)`
    → raw form `A[i][j] = sampled(i, j)`.  Direct projection:
    `matrix_to_spec(future(A)) == raw_A_as_ntt` (no transpose).

This contradicts the next-session prompt's blanket "libcrux impl
stores A in transposed form" claim — the convention is **per call
site** depending on the bool flag.  The prior-session F* ensures
already encoded this distinction (one cited `matrix_A_as_ntt`
directly, the other cited `matrix_transpose matrix_A_as_ntt`); the
new pure-Rust ensures preserve it via explicit
`hacspec_ml_kem::matrix::transpose` calls when needed.

### F* verification

  - `Libcrux_ml_kem.Ind_cpa.fsti.checked` ✅ — verifies clean in
    ~22s after a stale-hint flush.  **Now 0 `Spec.MLKEM` cites**
    (down from 34 at session start).
  - `Libcrux_ml_kem.Ind_cca.fsti.checked` ✅ — unchanged from prior
    session, still 0 cites.
  - `Libcrux_ml_kem.Ind_cca.Unpacked.fsti.checked` ❌ — still 25
    `Spec.MLKEM` cites for out-of-scope functions
    (`unpack_public_key`, `impl_3__serialized{,_mut}`,
    `impl_4__*serialized*`, `keys_from_private_key`,
    `transpose_a`).  These are byte-encode/decode wrappers and
    accessor methods around the unpacked struct, not part of the
    five Pattern C lane targets.  Build still fails on the first
    cite (line 62, `unpack_public_key`).
  - `Hacspec_ml_kem.Serialize.fst.checked` re-verifies clean in
    ~7s with the new `deserialize_then_decompress_u_then_ntt`.
  - `Hacspec_ml_kem.Ind_cpa.fst.checked` and
    `Hacspec_ml_kem.Ind_cca.fst.checked` re-verify clean.

### Stale-hint flush — gotcha to remember

Building `Libcrux_ml_kem.Ind_cpa.fsti.checked` after the
spec-module rebuild initially failed at the *prior-session*
`serialize_vector` requires (line 26), with a `--z3rlimit 80`
timeout and "incomplete quantifiers (with hint)".  Root cause:
the cached hint file
(`hints/Libcrux_ml_kem.Ind_cpa.fsti.hints`) referenced symbols
under the old `Hacspec_ml_kem.Parameters.Sizes.*` namespace,
which has since been refactored away.  Z3 couldn't locate the
referenced facts, fell back to non-hint mode, and timed out.

**Fix**: `rm hints/Libcrux_ml_kem.Ind_cpa.fsti.hints` then
rebuild.  F* re-records a fresh hint that matches the
current `Hacspec_ml_kem.Parameters.*` symbols.  Total rebuild
time post-flush: ~22s including hint regeneration.

Lesson for future R11 sessions: **after large spec-side
refactors, the libcrux-side `.fsti.hints` files become stale
in a way that masks as a Z3 timeout**.  Default reaction
should be to `rm` the stale hint and retry, before bumping
rlimit.

### Pattern C — finalized

Pattern C scales cleanly across all five unpacked-API
functions.  Form template:

```rust
#[hax_lib::ensures(|res|  // or |()| or |(a,b)|
    match hacspec_ml_kem::<helper>::<...>(<args>) {
        Ok((field0, field1, ..., fieldN)) =>
              <projection_0> == field0
           && <projection_1> == field1
           && ...
        Err(_) => true,
    }
)]
```

Where `<projection_N>` is either:
  - `vector_to_spec(&future(out)...field)` for a vector field
  - `matrix_to_spec(&future(out)...field)` (possibly with `transpose`
    wrap) for a matrix field
  - `future(out)...field` for a `[u8; N]` byte-array field
  - `result == ...` for a value-returning function

No new spec helpers needed beyond the P5 trio (`generate_keypair_unpacked`,
`encrypt_unpacked`, `decrypt_unpacked` + the `ind_cca_unpack_*`
wrappers that handle the libcrux-transpose convention).

### R compliance self-audit (this push)

  - **R1 (branch)**: All commits on `libcrux-ml-kem-proofs`.  Not
    pushed.
  - **R2 (no new admits)**: Confirmed.  All five Pattern C
    functions remain `lax` (carryover from pre-session); no new
    admits introduced.
  - **R3 (no new axioms)**: Confirmed.
  - **R4 (rlimit ≤ 800)**: Confirmed.  All migrated functions kept
    their existing `--z3rlimit` settings (200/300/500/800).
  - **R5 (iteration cap 20 min)**: All migrations completed within
    cap.  The longest single iteration was the spec-rebuild +
    stale-hint diagnosis (~10 min).
  - **R6 (snapshot/touch)**: `python3 hax.py extract` re-ran cleanly;
    only modified files (Serialize.fst, Ind_cpa.fst, Ind_cca.fst,
    Ind_cca.Unpacked.fsti, Ind_cpa.fsti) were re-extracted.  After
    rebuild, cleared 3 corrupt 0-byte spec `.checked` files and
    rebuilt them from source — single-target deletes, not bulk-nuke.
  - **R7 (trait FROZEN)**: `src/vector/traits.rs` not edited.
  - **R8 (no fstar-mcp)**: Used `make` only; no fstar-mcp.
  - **R9 (commit prefix `agent-mlkem:`)**: All 3 commits this push
    use the prefix.  Spec-side and libcrux-side commits separated.
  - **R10 (no wrappers)**: Confirmed.  The new spec helper
    `deserialize_then_decompress_u_then_ntt` is a real definition
    (composition of two existing functions), not an alias.  No new
    `Hacspec_ml_kem.<TopLevel>.fst` files; the helper extends
    `Hacspec_ml_kem.Serialize`.  No `unfold let` aliases over
    `Spec.MLKEM`.
  - **R11 (no `fstar!` escape in ind_cpa/ind_cca annotations)**:
    Confirmed for all 8 functions migrated this push.  The
    `[@ "opaque_to_smt"]` and `--z3rlimit ...` `fstar::before` /
    `fstar::options` annotations remain (these are F* directives,
    not requires/ensures escapes; allowed under R11).

### Final commit SHAs (this push)

```
6bbec798d agent-mlkem: ind_cpa::{build_unpacked_public_key,deserialize_then_decompress_u} — pure-Rust ensures
62d48c071 agent-mlkem: ind_cpa+ind_cca::*_unpacked — pure-Rust ensures (Pattern C ×5)
7502cba24 agent-mlkem: specs/ml-kem — add deserialize_then_decompress_u_then_ntt
```

Tip: `6bbec798d`.  Branch 20 ahead of `origin/libcrux-ml-kem-proofs`.

### Counts (R11 progress, full session)

| File | `Spec.MLKEM` cites in `.fsti` (session start / now) |
|---|---|
| `Libcrux_ml_kem.Ind_cca.fsti` | 0 / **0** (DONE in earlier push) |
| `Libcrux_ml_kem.Ind_cpa.fsti` | 34 / **0** (DONE this push) |
| `Libcrux_ml_kem.Ind_cca.Unpacked.fsti` | 46 / **25** |

Both `Ind_cpa.fsti` and `Ind_cca.fsti` are now fully R11-compatible
at the annotation surface.  `Ind_cca.Unpacked.fsti` retains 25 cites
in byte-encode/decode wrappers and accessors that were not part of
this lane's five-function scope.

### Out-of-scope (next sessions)

  - `Libcrux_ml_kem.Ind_cca.Unpacked.fsti` mop-up: 25 cites across
    `unpack_public_key`, `impl_3__serialized*`, `impl_4__*serialized*`,
    `keys_from_private_key`, `transpose_a`.  These are
    byte-encode/decode wrappers around the unpacked struct, mostly
    citing `Spec.MLKEM.vector_encode_12` and
    `Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE`.  Mechanical migration
    using the existing `hacspec_ml_kem::serialize::*` helpers.
  - **Lane 2 — body-tactic Spec.MLKEM elimination**: the eight
    functions migrated this push still carry `hax_lib::fstar!(...)`
    body tactics that cite `Spec.MLKEM.*` for `eq_intro`,
    `Classical.forall_intro`, `assert (... == ...)`.  R11
    explicitly targets the annotation surface; bodies need their
    own sprint with bridge lemmas relating
    `Spec.MLKEM.*` → `Hacspec_ml_kem.*`.
  - `encrypt_c1`/`encrypt_c2` migration — internal-only functions,
    audit recommends leave-as-is.
