# Inline Portable.squeeze cascade profile (2026-05-01)

Baseline: `--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always --log_queries`.
Run captured at `/tmp/profile_inline_baseline.log`.

## Top-15 squeeze sub-queries by wall time

| q   | wall (ms) | rlimit used | status     | Cluster |
|----:|----------:|------------:|------------|---------|
| 231 | 95247     | 400.000     | **FAIL** (canceled) | 4 — cascade |
| 224 | 94388     | 400.000     | **FAIL** (canceled) | 4 — cascade |
| 280 | 87816     | 400.000     | **FAIL** (canceled) | 4 — cascade |
| 263 | 83002     | 400.000     | **FAIL** (canceled) | 4 — cascade |
| 170 | 58249     | 326.037     | ok         | 3 |
| 169 | 42480     | 225.077     | ok         | 3 |
| 101 | 24805     | 123.549     | ok         | 2 |
| 052 | 13748     |  91.242     | ok         | 1 |
| 044 |  6796     |  52.524     | ok         | 1 |
| 049 |  5803     |  37.176     | ok         | 1 |
| 042 |  4991     |  39.922     | ok         | 1 |
| 048 |  2942     |  ?          | ok         | 1 |
| 098 |  1763     |  14.748     | ok         | 2 |
| 097 |   917     |   7.030     | ok         | 2 |
| 241 |   457     |   3.279     | ok         | (warm) |

All other ~290 sub-queries succeeded in <500 ms.

## Cluster diagnosis (mapped against Portable.fst:505-707)

- **Cluster 1 (q42-q52)** — `output_blocks == 0` branch, lines 514-547:
  the inline `aux` closure + `Seq.lemma_eq_intro`.  The closure computes
  `b = k / RATE` (with `k < RATE`) so b == 0 — but Z3 verifies that for
  each k via `createi_lemma`, hitting the lambda body with arithmetic.
- **Cluster 2 (q97-q101)** — pre-loop init in the else branch, lines
  548-592: `aux_write` and `aux_tail` closures plus the loop invariant
  initialisation check.
- **Cluster 3 (q169-q170)** — loop body well-formedness or invariant
  preservation, lines 594-649.  Used 326 of 400 rlimit on q170 — close
  to the 400 ceiling but completed.
- **Cluster 4 (q224, q231, q263, q280)** — **the actual cascade**.  All
  four hit `rlimit 400/400` and cancel.  Likely the post-loop branch
  (output_rem != 0 vs == 0 paths, lines 651-705) plus the per-byte
  `aux_partial` closure, where the closure body re-evaluates the
  createi-form spec at every k and the bound forall extends across the
  full output range.

## Key observation

The four canceled queries are **specific compositions**, not a per-iteration
explosion.  All other loop-invariant queries finish in <100 ms.  This means
opacifying `iterate_keccak_f` (so the createi closure body is a black box)
should make the createi_lemma SMTPat trivially harmless on these four
without touching the 290 already-fast queries.
