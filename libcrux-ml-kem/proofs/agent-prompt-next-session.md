# Agent prompt — libcrux-ml-kem-proofs, fstar! literal sweep + Rust annotations

Paste this into a fresh Claude Code session opened in
`~/libcrux-trait-opacify/libcrux-ml-kem` (auto mode recommended).

You are a single-lane agent for the libcrux-ml-kem F\* verification
effort.  This session continues a partial sweep started in the prior
session: replace stale legacy F\* paths in `fstar!()` literal blocks
across `libcrux-ml-kem/src/` with the Rust-side antiquote
mechanism, so hax automatically resolves the right F\* names.

The end-state goal (a couple sessions from now) is to flip eight
unpacked-API functions in `ind_cpa.rs` / `ind_cca.rs` from
`verification_status(lax)` to `verification_status(panic_free)`.
That requires `.fst`-level body verification, which transitively
requires every imported `.fsti.checked` to typecheck.  Right now
many of those `.fsti`'s fail because of stale literal F\* references
left behind by commit `60af8d332`'s "promote impl→spec lift functions
from F\* injection to Rust" rename.

This session: continue and complete the sweep.  Don't flip
`lax → panic_free` yet — that's its own session.

## Read first (in order)

  1. `proofs/agent-status/session-2026-05-01-impl-pure-rust.md`
     (Lane C section at the bottom) — the prior session's full log.
  2. `git log --oneline c5636496a^..HEAD` — see the three "sweep prep"
     commits already landed:
     ```
     e2cae3b2d agent-mlkem: sampling.rs — antiquote to_spec_poly_t reference
     7d286a401 agent-mlkem: ntt+invert_ntt — antiquote+arity-fix mont_i16_to_spec_array
     c5636496a agent-mlkem: ntt+invert_ntt — antiquote zetas_N references in fstar! blocks
     ```
     Read each diff for the established pattern.

## Branch state at session start

```
$ git log --oneline -5
e2cae3b2d agent-mlkem: sampling.rs — antiquote to_spec_poly_t reference
7d286a401 agent-mlkem: ntt+invert_ntt — antiquote+arity-fix mont_i16_to_spec_array
c5636496a agent-mlkem: ntt+invert_ntt — antiquote zetas_N references in fstar! blocks
19f9a39e9 agent-mlkem: session report — append Lane C push (Pattern C ×5 + Ind_cpa.fsti mop-up)
6bbec798d agent-mlkem: ind_cpa::{build_unpacked_public_key,deserialize_then_decompress_u} — pure-Rust ensures
```

Branch: `libcrux-ml-kem-proofs`.  Tip: `e2cae3b2d`.  23 commits
ahead of `origin/libcrux-ml-kem-proofs`.  Not yet pushed.

## Background: what `60af8d332` changed (and what got missed)

Commit `60af8d332` ("agent-mlkem: cleanup — promote impl→spec lift
functions from F\* injection to Rust") moved several lift functions
from inline F\* string injections in `vector/traits.rs` and
`vector.rs` to real Rust functions under `#[cfg(hax)]`.  This
**simultaneously** changed three things for each function:

  1. **Renamed** the F\* symbol (e.g. `to_spec_poly_t` → `poly_to_spec`,
     `to_spec_vector_t` → `vector_to_spec`, `to_spec_matrix_t` →
     `matrix_to_spec`).
  2. **Moved the module path** (e.g. `Libcrux_ml_kem.Vector.to_spec_poly_t`
     → `Libcrux_ml_kem.Vector.Spec.poly_to_spec`).
  3. **Added Rust const generics** (`<const N: usize>` on
     `mont_i16_to_spec_array` / `i16_to_spec_array`; `<const RANK:
     usize>` on `vector_to_spec` / `matrix_to_spec`), which hax
     extracts as new explicit F\* arguments.

The commit message claimed "Existing F\* names preserved so commute
lemmas continue to resolve at the same module paths" — but the
literal `fstar!()` references throughout `libcrux-ml-kem/src/`
weren't updated.  These were masked by `lax` body verification
elsewhere; surfacing them blocks `panic_free`.

## What's already fixed (this lane)

| Symbol | File(s) | Fix | Commit |
|---|---|---|---|
| `zetas_{1,2,4}` | `ntt.rs`, `invert_ntt.rs` | `${zetas_N}` antiquote (12 sites) | `c5636496a` |
| `mont_i16_to_spec_array` | `ntt.rs`, `invert_ntt.rs` | `${mont_i16_to_spec_array::<16>} (mk_usize 16)` (24 sites) | `7d286a401` |
| `to_spec_poly_t` | `sampling.rs` | `${poly_to_spec::<Vector>}` (1 site) | `e2cae3b2d` |

After these, `Libcrux_ml_kem.Ntt.fsti.checked`,
`Libcrux_ml_kem.Invert_ntt.fsti.checked`, and
`Libcrux_ml_kem.Sampling.fsti.checked` all rebuild with name
resolution succeeding (Sampling still fails on a separate `Spec.MLKEM`
issue — see "Out of scope" below).

## What's left — `to_spec_*_t` sweep

49 stale literal references remain in `fstar!()` blocks across:

| File | `to_spec_poly_t` | `to_spec_vector_t` | `to_spec_matrix_t` | Total |
|---|---|---|---|---|
| `src/ind_cca.rs` | 0 | 4 | 4 | 8 |
| `src/ind_cpa.rs` | 14 | 14 | 4 | 32 |
| `src/serialize.rs` | 8 | 1 | 0 | 9 |
| `src/polynomial.rs` | 1 | 0 | 0 | 1 |

Confirm the exact spread with:

```
grep -rE "[A-Za-z_]+\.[A-Za-z_]+\.to_spec_(poly|vector|matrix)_t" libcrux-ml-kem/src \
  | grep -v "ntt.rs~\|//"
```

(The `ntt.rs~` backup file and commented-out lines in `ntt.rs` are
ignored — only live `fstar!()` blocks count.)

## The substitution recipe (per file)

### Step 1 — add `#[cfg(hax)]` imports at the top of the file

For files that don't already have these imports, add (alongside
the existing `use crate::polynomial::spec;` etc.):

```rust
#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::spec::{matrix_to_spec, poly_to_spec, vector_to_spec};
```

Note the path: `crate::vector::spec` is the new home (in
`libcrux-ml-kem/src/vector.rs`'s `pub(crate) mod spec`).  These
extract to `Libcrux_ml_kem.Vector.Spec.{poly_to_spec, vector_to_spec,
matrix_to_spec}`.

### Step 2 — three substitutions per file

```
Libcrux_ml_kem.Vector.to_spec_poly_t #$:Vector
  →  ${poly_to_spec::<Vector>}

Libcrux_ml_kem.Vector.to_spec_vector_t #$K #$:Vector
  →  ${vector_to_spec::<K, Vector>} (mk_usize $K)

Libcrux_ml_kem.Vector.to_spec_matrix_t #$K #$:Vector
  →  ${matrix_to_spec::<K, Vector>} (mk_usize $K)
```

The `(mk_usize $K)` suffix on vector/matrix forms is the new explicit
const-generic-as-F\*-arg, mirroring the
`${mont_i16_to_spec_array::<16>} (mk_usize 16)` pattern from
commit `7d286a401`.

### Step 2b — variants you'll see

  - **`#v_K #v_Vector` form** (ind_cpa.rs body lemmas).  These
    already pass the F\*-mangled names directly (in body `fstar!`
    blocks where `#$:Vector` doesn't expand correctly).  Substitute
    the same way; the antiquote will produce the `_to_spec` name and
    the const-generic arg will be `(sz v_K)` since these are inside
    F\*-expression contexts.

    Concretely:
    ```
    Libcrux_ml_kem.Vector.to_spec_vector_t #v_K #v_Vector
      →  ${vector_to_spec::<K, Vector>} v_K
    ```
    (drop the `mk_usize` cast; `v_K` is already a `usize` in F\*
    inside body lemmas).

  - **No-implicit form** (`Libcrux_ml_kem.Vector.to_spec_matrix_t
    public_key.f_A` — no `#$K`/`#$:Vector` at all).  One occurrence
    in `src/ind_cpa.rs:517` inside an `assert`.  Substitute the
    function name antiquote and add the const-generic arg derived
    from the field's type (here `K`).

### Step 3 — re-extract and verify

```
python3 hax.py extract
make /Users/karthik/libcrux-trait-opacify/.fstar-cache/checked/Libcrux_ml_kem.Ind_cpa.fst.checked
```

If the build fails at the **next** stale ref (e.g. a `Spec.MLKEM`
cite — see Out-of-scope below), that's a different sprint; record
the failing line and move on to the next file.

### Step 4 — commit per file

Use `agent-mlkem:` prefix.  One commit per file:

```
agent-mlkem: <file> — antiquote to_spec_*_t references
```

Body should mention:
  - Number of sites fixed (per kind).
  - Whether the file's own `.fsti.checked` rebuilds clean.
  - Whether a downstream `Spec.MLKEM` blocker remains.

## Workflow

  1. Pick the smallest file first: **`polynomial.rs`** (1 site).
  2. Then **`ind_cca.rs`** (8 sites).
  3. Then **`serialize.rs`** (9 sites).
  4. Then **`ind_cpa.rs`** (32 sites — biggest, save for last
     since you'll have the pattern down by then).

For each: edit, extract, build the file's `.fst`/`.fsti`,
commit.  Cap each per-file iteration at 20 min (R5).

After all four are done, attempt a full `Libcrux_ml_kem.Ind_cpa.fst.checked`
rebuild to surface the next layer of issues.

## Hard rules (R1-R11)

  R1  Branch `libcrux-ml-kem-proofs`.  May `git push` (fast-forward
      only).  DO NOT force-push, DO NOT push to `main`, DO NOT open
      a PR without explicit user authorization.
  R2  No new admits beyond existing `lax` / `ADMIT_MODULES` carry-overs.
      The 8 functions that are currently `lax` stay `lax` this session
      (the panic_free flip is for a future session).
  R3  No new axioms.
  R4  `--z3rlimit ≤ 800` HARD CAP; ≤ 400/query under
      `--split_queries always`.  Default tier 200.
  R5  Inner edit-check: `make check/<Mod>.fst[i]` from
      `proofs/fstar/extraction/`.  Cap iteration at 20 min/attempt.
  R6  After `python3 hax.py extract`: snapshot SHAs and touch unchanged
      `.checked` files (per `feedback_touch_unchanged_checked`).
  R7  Trait FROZEN — `src/vector/traits.rs`'s `Operations` /
      `Repr` definitions not edited.  The `spec` submodule below
      it MAY be edited (but should not need to be edited this session).
  R8  No `fstar-mcp`.
  R9  Commit prefix `agent-mlkem:`.  Commit per-file is fine; spec-side
      changes (none expected this session) commit separately from
      libcrux-side.
  R10 No wrappers, no namespace-squatting, no new top-level
      `Hacspec_ml_kem.*` modules.  This sweep adds no new symbols —
      just re-routes existing references.
  R11 No `fstar!` escape in `src/ind_cpa.rs` / `src/ind_cca.rs`
      requires/ensures (R11 lane already done for the unpacked-API
      surface).  But this sweep MAY edit `fstar!` body tactics in
      those files — that's a body-internal Spec.MLKEM cleanup
      (different lane), so be careful: only substitute the
      `to_spec_*_t` references, don't accidentally introduce new
      `Spec.MLKEM` cites or alter the body proof shape.

## Lessons carried forward

  - **Antiquote with turbofish for const generics**: hax
    Rust-typechecks the antiquote expression itself.  If a function
    has `<const N: usize>`, you must turbofish:
    `${func::<16>}`, otherwise Rust errors with "cannot infer the
    value of const parameter `N`".  For type generics, use
    `${func::<Vector>}` etc.  This was the lesson from commit
    `7d286a401`.

  - **Re-extracted `Hacspec_ml_kem.*.fst.checked` files can become
    stale 0-byte files** if F\* fails to write because of a
    transitive corrupt dep.  If you see a "checked file ... was
    not written" warning along with "All verification conditions
    discharged successfully", check the `.checked` file size — if
    0 bytes, delete the specific corrupt file (NOT bulk-delete) and
    re-build.  Memory note `feedback_no_cache_nuke` only prohibits
    bulk-delete.

  - **Stale `.hints` files** (referencing renamed-away symbols
    like `Hacspec_ml_kem.Parameters.Sizes.*`) cause hidden
    `--z3rlimit 80` timeouts that look like proof failures.  When a
    .fsti suddenly fails to verify after spec-side rebuilds, check
    if the `.hints` file mentions deprecated symbol paths; if so,
    `rm` the stale hint and retry.  Recorded in the prior session's
    Lane C "Stale-hint flush" subsection.

  - **`.fst` deps are different from `.fsti` deps**.  The current
    `Ind_cpa.fsti.checked` verifies clean (per Lane C).  But its
    `.fst.checked` will require `Ntt.fsti.checked`,
    `Sampling.fsti.checked`, `Serialize.fsti.checked`, etc., to all
    typecheck — which is what this sweep is about.

## Out-of-scope for this session (different lanes)

  - **`Spec.MLKEM` legacy module references in non-CPA/CCA `.fsti`s.**
    `Libcrux_ml_kem.Sampling.fsti` (line ~186), `Polynomial.fsti`,
    `Serialize.fsti`, `Vector.fsti` etc. still cite `Spec.MLKEM.*`
    (the legacy F\*-only spec module that was scrubbed from
    `Ind_cpa.fsti`/`Ind_cca.fsti` in the prior R11 lane).
    ~395 references remain crate-wide.  This is "R11 extension to
    the rest of libcrux-ml-kem" — its own multi-session sprint.

  - **`lax → panic_free` flip for the 8 functions** in
    `ind_cpa.rs`/`ind_cca.rs`.  Once both this `to_spec_*_t` sweep
    AND the `Spec.MLKEM` sweep are done, `panic_free` will become
    feasible.  Until then, leave `verification_status(lax)` on those
    8 functions as-is.

  - **Other `_<digit>_` mangled names** beyond `zetas_N` and
    `mont_i16_to_spec_array`.  The prior session's broader audit
    found these only in trait-impl modules (Avx2, Neon, Portable
    Vector serialize/deserialize), where the mangling is
    self-consistent (definitions and call sites both use the `_`
    suffix in extracted form).  No fixup needed there.

## End-of-session deliverable

Append a "Lane D push" section to
`proofs/agent-status/session-2026-05-01-impl-pure-rust.md`.  Include:

  - Number of `to_spec_*_t` sites fixed per file.
  - Per-file `.fsti.checked` build status (clean / blocked-on-Spec.MLKEM /
    blocked-on-something-else).
  - Whether `Libcrux_ml_kem.Ind_cpa.fst.checked` builds end-to-end
    after the sweep (probably no — likely still blocked on the
    `Spec.MLKEM` cleanup in dependent .fsti's, which is the next
    lane).
  - Final commit SHA chain and total commit count this session.
  - **Self-audit (R1-R11)**: any new admits? any wrappers? any
    new top-level Hacspec modules?  any `fstar!` body tactics
    altered beyond the targeted substitution?  If yes: revert.

DO NOT touch `~/libcrux-ml-dsa-proofs` or `~/libcrux-sha3-focused`.
