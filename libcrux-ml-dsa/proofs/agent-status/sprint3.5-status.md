# Sprint 3.5 — opacify is_bounded_poly_range / is_bounded_poly_slice

Started: 2026-05-05
Closed: 2026-05-05

## Outcome — DONE

All success criteria met:
- `make check/Libcrux_ml_dsa.Polynomial.Spec.fst` → exit 0
- `make check/Libcrux_ml_dsa.Matrix.fst`         → exit 0 at z3rlimit
  **400** (down from 800) for `compute_matrix_x_mask`
- `cargo test --release --lib`                    → 20 passed
- `compute_as1_plus_s2`'s admitted-body annotations still verify under
  the new opacity (verified as part of the Matrix.fst run).

## Steps taken

### Step 1 — opacity + lemmas in `src/polynomial.rs::spec`

For both `is_bounded_poly_range` and `is_bounded_poly_slice`, mirrored
the existing `is_bounded_poly` recipe:

- `[@@ "opaque_to_smt"]` via `hax_lib::fstar::before`
- `lemma_*_lookup`: `requires opaque-pred /\ k-in-range`,
  `ensures is_bounded_poly b (Seq.index arr k)` with multi-pattern
  `[SMTPat (opaque-pred); SMTPat (Seq.index arr k)]` (per
  `feedback_dual_smtpat_opaque_atom`, fires bidirectionally).
- `lemma_*_intro`: takes the universal hypothesis as `requires`,
  proves the opaque predicate.

Body of each is a single `reveal_opaque (\`%name) (name args)`.

### Step 2 — `compute_matrix_x_mask` rlimit dropped

`src/matrix.rs:95`: `--z3rlimit 800` → `--z3rlimit 400 --ext context_pruning --split_queries always`.

### Step 3 — verified clean

Full re-extraction + per-module clean rebuild (with stale hints
cleared, since the `Polynomial.Spec.fst` digest changed and old hints
no longer applied):

```
Polynomial.Spec.fst   →  ~1s
Encoding.T0.fst       →  ~14s
Ntt.fst               → ~24s
Encoding.Error.fst    → ~17s
Matrix.fst            → ~183s   (rlimit 400, fresh hint recording)
```

## Notes

1. **Stale hint mismatch is the only "false-failure" mode**:
   when `Polynomial.Spec.fst`'s body changed, all hint files in
   modules that depend on it (transitively) have wrong query
   digests. F* falls back to recording new hints — but the cold
   record is slower, and on rare cases tougher queries time out
   before recording. Always `rm -f` both `.checked` AND
   `.hints` for affected modules per `feedback_per_stage_clean_rebuild`.
   This was Sprint 3.5's only debugging episode.

2. **The dual-SMTPat lookup eliminated all manual lookup calls**
   needed in caller bodies. None of the consumer rewrites required
   adding `lemma_*_lookup` invocations: the multi-pattern fires
   automatically when both the opaque predicate and `Seq.index arr k`
   appear in the goal. (No `lemma_*_intro` calls needed in the body
   either, but they're kept around for future caller use.)

3. **Profiling not re-run** because the success criterion was
   already met and we're within the 60-90 min budget. The
   Sprint 3 profile that motivated this — `equation_*is_bounded_poly_range/slice`
   dominating QI — is moot now that those predicates are opaque.
   If `compute_as1_plus_s2` body proofs (separate sprint) hit
   another cliff, profile then.

4. **rlimit headroom**: `compute_matrix_x_mask` now passes at 400 with
   no margin issues observed. If future Sprint 3 follow-ups (closing
   `compute_as1_plus_s2` body, addressing `compute_w_approx`) need
   to bump rlimit, they have headroom up to the 800 cap (per
   `feedback_rlimit_cap_800`).

## Files touched

- `src/polynomial.rs` — added opacity + 4 lemmas (~70 LOC)
- `src/matrix.rs` — single rlimit edit (800 → 400)
- `proofs/fstar/extraction/Libcrux_ml_dsa.Polynomial.Spec.fst` —
  re-extracted (Sprint 3.5 owns the diff)
