# ML-KEM F\* hints — stable vs experiment hygiene

The heavy ML-KEM proofs (`Libcrux_ml_kem.*`, `Hacspec_ml_kem.*`) verify **with hints**: the
gate is `--use_hints --record_hints` (the extraction Makefile default). Some functions are
budget-bound right at their `--z3rlimit` (e.g. `Matrix.compute_vector_u` needs ~377–444 cold
against a 400 limit), so their recorded hint is what keeps the build reliably green.

## What is committed

The ML-KEM proof's stable hints under `/.fstar-cache/hints/` are **git-tracked** via a
`.gitignore` exception:
- `Libcrux_ml_kem.*.hints` — the ml-kem extraction (generated modules), and
- `Hacspec_ml_kem.*.hints` — the hand-written specs + commute bridges (e.g. `Chunk`,
  `Bridges`, `Keygen_bridge`, `Sample_matrix_bridge`), plus
- the hand-written proof-lib helpers in ml-kem's dependency closure: `Spec.Utils.*`,
  `Proof_utils.*`, `MkSeq.*`, `BitVec.*`, `Bitvec.*`, `BitVecEq.*`, `Tactics.*`.

Everything else in `.fstar-cache/` (the `checked/` cache, other crates' hints, scratch-module
hints such as `Scratch*`) stays ignored. This makes an accidental hint clobber recoverable:

```sh
git checkout -- .fstar-cache/hints/        # restore the stable hint snapshot
```

## Hygiene: stable hints vs experiment hints

The clobber this guards against: a **bare `--record_hints`** run (without `--use_hints`) does
not load the existing hints, so it **overwrites** the file with only the queries proven in that
run — silently dropping the hints for functions that didn't fully re-prove cold. Rules:

- **Experiments / inner loop** → use the `fstar-proxy` `admit_except` mode (it writes **no**
  `.checked`/`.hints` — zero pollution), or `fstar_typecheck`. Never `--record_hints` for a
  one-off check.
- **Full builds** (verdict / re-record) → always `--use_hints --record_hints` (the Makefile
  default — i.e. plain `make ... VERIFY_SLOW_MODULES=yes`). This **merges**: it replays existing
  hints and records freshly-proven ones, never dropping a function's hint. **Never** pass a bare
  `ENABLE_HINTS=--record_hints` (no `--use_hints`).
- After any experimental full build that touched the tracked hints, `git status` shows the diff;
  `git checkout -- .fstar-cache/hints/<Module>.hints` to restore the stable snapshot before
  committing, unless you intend to update the stable hints.
- Updating stable hints is a deliberate act: run the full `--use_hints --record_hints` build,
  confirm it is green (`all VCs discharged`), then commit the changed `*.hints`.

## Position-shift note

Hints are sensitive to source position. Editing a function early in a large `.fst` shifts the
later functions' lines and invalidates their recorded hints, forcing a cold re-prove (which, for
budget-bound NTT functions, can momentarily miss the rlimit). Prefer appending new declarations
at end-of-file; after an unavoidable shift, do a full `--use_hints --record_hints` build to
re-record, confirm green, and commit the updated hints.
