# Next-session prompt — ML-KEM proof sprint 2026-05-05

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** (commit pending — see `sprint-2026-05-04-rollup.md`)

## Read first (non-negotiable)

1. **`~/.claude/skills/fstar-for-libcrux/README.md`** — token discipline rules
   (Rules 1–8). Especially:
   - Pipe `make` output to `/tmp/*.log`, grep for errors. Never `Read` a full
     make log.
   - `grep` before `Read`. Use targeted line ranges only.
   - Per-fn budget 30–60 min; if blocked, mark FOLLOW-UP and stop.
   - Never bulk-delete `.checked`; delete only the targeted module's cache
     when an `.fsti` actually changes.
   - **`fstar-mcp` is NOT configured in `~/.claude/settings.json`.** Don't
     waste time looking for it. Use grep + targeted `Read` + outer-loop `make`.

2. **`MEMORY.md`** entries (already loaded as auto-memory). The most relevant
   for this phase:
   - `feedback_panic_free_vs_lax` — `panic_free` is the real middle step;
     prefer it over `lax` whenever the body type-checks for panic-freedom.
   - `feedback_smtpat_percent_above_trait` — drop SMTPats whose bodies expose
     raw `%` above the trait; keep ones whose bodies are bounds or Hacspec
     calls.
   - `feedback_extraction_first` — audit `extraction.toml` filtering before
     spending time on proofs; flipped-out modules are worse than admitted.

## What was done last session (2026-05-04)

The serialize_public_key chain is wired end-to-end:

| Function | State |
|---|---|
| `serialize_vector` | **panic_free** + real ensures using `Hacspec_ml_kem.Serialize.serialize_secret_key` (createi form) |
| `serialize_public_key_mut` | **panic_free** + real ensures using `Hacspec_ml_kem.Serialize.serialize_public_key` |
| `serialize_public_key` | **fully verified** (wrapper closes via `_mut`'s ensures) |

Bridge lemmas in `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Serialize.fst`:
- `serialize_secret_key_chunk_eq` / `serialize_secret_key_all_chunks_eq`
- `serialize_public_key_vector_eq` / `serialize_public_key_seed_eq`

Verified clean (exit 0): `Ind_cpa.fst`, `Ind_cca.fst`, `Ind_cca.Unpacked.fst`,
`Hacspec_ml_kem.Commute.Serialize.fst`, `Mlkem512.Portable.fst`,
`Mlkem768.Portable.fst`.

## Key learnings to apply this phase

### L1 — `panic_free` over `lax` for any body that type-checks
Last session's win: switching from `lax` to `panic_free` gave callers a real
ensures axiom for free, with no SMTPat tuning. The May 3 attempt at the same
wiring failed under `lax`-via-`--admit_smt_queries true` because the createi
axioms polluted later quantifier-instantiation contexts. Under `panic_free`,
the body verifies normally; only the ensures gets an `admit ()` stub.

**Heuristic:** if a function is `lax` and (a) the body has no obvious
panic-freedom violations, (b) the requires already mentions all the bounds
the spec asks for — try `panic_free` first. It's a one-line change with high
leverage.

### L2 — `update_at_range` ensures already give slice-of-result
Don't add helper lemmas like `update_at_range_result_lemma` — the canonical
`Rust_primitives.Hax.Monomorphized_update_at.fsti` already exposes:
```fstar
update_at_range:       Seq.slice res (v i.f_start) (v i.f_end)         == x
update_at_range_from:  Seq.slice res (v i.f_start) (Seq.length res)    == x
update_at_range_to:    Seq.slice res 0 (v i.f_end)                     == x
update_at_range_full:  res == x
```
Plus the "outside the range, slice unchanged" clauses. Read the fsti before
inventing new axioms.

### L3 — `is_rank` boolean must be `.to_prop() &` not `&&`
F* doesn't propagate the left operand of boolean `&&` as a hypothesis when
discharging the right; switching to `.to_prop() &` (logical conjunction)
fixes "incomplete quantifiers" on the right operand. This is now the
default form across `ind_cpa.rs` after `3c73d1a96`. **If you add a new
`#[hax_lib::requires]` with `is_rank(K) && ...`, use `.to_prop() &`.**

### L4 — Inner loop is grep + targeted Read; outer loop is `make`
This session used **3** make invocations total. Most signature lookups
were grep-only. Read only the bytes you need. The cache-warm prompt
discipline matters even more without fstar-mcp.

### L5 — Re-extract resets pre-session "M" files
`./hax.py extract` overwrites all extracted F* files to canonical hax
output. Pre-session manual edits to extracted F* files (anti-pattern per
`feedback_no_manual_edits_extracted`) get wiped. Be aware before you
extract: if you see a working tree with M-state extracted files at session
start, they may be intentional in-progress work — check with the user
before overwriting them. (This session went ahead because the changes
matched the source; flag it next time.)

## Work for this session

### Stream A — Cascade-flip `serialize_unpacked_secret_key` (≤ 30 min)

`libcrux-ml-kem/src/ind_cpa.rs` around line 484. Current state: `lax`,
"FOLLOW-UP (Phase D): cascade-lax — body composes lax serialize_public_key
and serialize_vector". Both dependencies are now panic_free + ensures.

Path:
1. Drop the `verification_status(lax)` annotation.
2. Run `cargo check`, `./hax.py extract`, `make check/Libcrux_ml_kem.Ind_cpa.fst`.
3. If panic_free passes, decide whether to keep the existing ensures
   (functional spec to `Hacspec` `serialize_unpacked_secret_key`) or stay
   structural-only.

Likely outcome: clean panic_free, no further work. If the body fails
panic-freedom, look at the `serialize_vector`/`serialize_public_key`
preconditions — most likely `is_rank(K)` propagation.

### Stream B — Audit debt: 3 `--admit_smt_queries true` in `Ind_cca.Unpacked.fst{i}`

From the May 3 rollup. Two known triggers:
1. `keys_from_private_key` body: assert that `deserialize_vector`'s ensures
   flows through record updates to `f_secret_as_ntt`.
2. `decapsulate` requires: stabilize `is_bounded_polynomial_vector` forall
   in the Z3 context — try an explicit `assert (is_bounded_polynomial_vector ...)`
   hint at the call site, or (cheaper) move the predicate into a `noeq`
   wrapper so Z3 sees it as a single fact rather than a forall.

Per-fn budget: 30 min each. If blocked at 30 min, document blocker and
move on.

### Stream C — Ladder serialize body proofs from panic_free → fully verified
*(Optional hardening; do only if Streams A and B both close fast.)*

Drop the `admit ()` (* Panic freedom *) at the end of `serialize_vector` /
`serialize_public_key_mut` bodies and prove the ensures inline. The
prompt sketched in `next-session-prompt-2026-05-04.md` Step 1 (per-byte aux
+ `serialize_secret_key_all_chunks_eq` + `lemma_eq_intro`) and Step 3
(case-split per-byte aux + both `serialize_public_key_*_eq` bridges +
`update_at_range` ensures) is the recipe.

This is *not* required for downstream callers — they see identical axioms
either way. Treat as polish.

### Stream D — `deserialize_then_decompress_u` loop invariant (U-7)
Long-standing FOLLOW-UP. The `ntt_vector_u` ensures is already stated
(body admitted, commit `56f3eea01`). The loop invariant must track
`vector_to_spec(u[0..i]) == map ntt (vector_to_spec(decompress u))`
across each iteration.

Estimate: 60+ min. Skip unless dedicated time. Mark FOLLOW-UP if blocked.

### Stream E — Update `fstar-perf-top20.md`
After all streams close, run a full `make` from
`libcrux-ml-kem/proofs/fstar/extraction` and refresh
`agent-status/fstar-perf-top20.md` per `feedback_track_fstar_perf`.

## Make commands

```bash
# Single-module verify (panic_free flip checks)
cd libcrux-ml-kem/proofs/fstar/extraction
make check/Libcrux_ml_kem.Ind_cpa.fst > /tmp/ind_cpa.log 2>&1; echo exit=$?
grep -nE '^\* Error|^Error|^make\[|incomplete' /tmp/ind_cpa.log | head -30

# Full rebuild for perf top-20
make > /tmp/full.log 2>&1; echo exit=$?

# Bridge lemma sanity (no need to delete .checked unless touched)
cd specs/ml-kem/proofs/fstar/commute
make check/Hacspec_ml_kem.Commute.Serialize.fst > /tmp/commute.log 2>&1; echo exit=$?
```

## Key file paths

- Implementation: `libcrux-ml-kem/src/ind_cpa.rs` (Rust source — edit here, not extracted F*)
- Hacspec spec: `specs/ml-kem/src/serialize.rs`
- Bridge lemmas: `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Serialize.fst`
- Extracted F* (gitignored, regenerated by hax): `libcrux-ml-kem/proofs/fstar/extraction/Libcrux_ml_kem.Ind_cpa.fst{,i}`
- Cascade target: `libcrux-ml-kem/proofs/fstar/extraction/Libcrux_ml_kem.Ind_cca.Unpacked.fst{,i}`
- Hax extraction entry: `libcrux-ml-kem/hax.py extract`
- Update_at_range axioms (don't reinvent): `<hax-lib>/proof-libs/fstar/primitives/Rust_primitives.Hax.Monomorphized_update_at.fsti`

## Pre-session checklist

- [ ] Working tree clean? If not, ask the user before `./hax.py extract` —
      pre-session "M" extracted files may be intentional.
- [ ] `fstar-mcp` server in `~/.claude/settings.json`? If yes, use
      `typecheck_buffer` for the inner loop. If no, plan for grep + make only.
- [ ] Read the fstar-for-libcrux skill if you haven't this calendar day.
- [ ] Update task list with the streams you intend to work this session;
      claim only A + B unless time-rich.
