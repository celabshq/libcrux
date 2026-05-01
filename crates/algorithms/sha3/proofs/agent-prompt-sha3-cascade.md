# Handoff: closing `lemma_squeeze2_arm64` and USER-2 against the byteform spec

Branch: `sha3-byteform-migration` (tip `634f898b5`).

## State at handoff

| Commit | Content |
|---|---|
| `bba52bf48` | Byteform spec migration: `Hacspec_sha3.Sponge.squeeze` is now a `createi`-based byte-map; recursive `squeeze_blocks` / `squeeze_last` and 5 bridge lemmas deleted; `iterate_keccak_f` added.  Net −863/+81. |
| `f11e50419` | `EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_one_step_portable` — per-iteration byteform step lemma, N=1 analogue of the Arm64 step lemma in the migration commit.  Verifies in 38.7 s standalone. |
| `634f898b5` | Cross-branch audit of `~/libcrux-trait-opacify` `rust-spec` branch: SUPERSEDED, no cherry-pick available. |

End-to-end `Libcrux_sha3.fst` verifies (with USER-2 admit retained on
`Libcrux_sha3.Generic_keccak.Portable.squeeze`).  Arm64 + Portable
byteform per-iteration step lemmas are shipped as building blocks.

## What's open

**Two interrelated admits.** Both share the same root obstacle:

  - `Libcrux_sha3.Generic_keccak.Portable.squeeze` (USER-2) — body
    wrapped in `--admit_smt_queries true`.
  - `EquivImplSpec.Sponge.Arm64.Driver.lemma_squeeze2_arm64` —
    `assume val`.

## Root obstacle: createi_lemma SMTPat × loop-invariant forall

Both inline closures need a 4-clause loop invariant of shape:

  - state condition: `s.f_st == iterate_keccak_f (v i - 1) s_init_st`
  - write region:    `output[k] == squeeze[k]` for `k < v i * v rate`
  - tail region:     `output[k] == output_initial[k]` for `v i * v rate <= k`
  - arithmetic bounds.

The `Seq.index (squeeze ...) k` term triggers
`Hacspec_sha3.createi_lemma` SMTPat (in `specs/sha3/proofs/fstar/extraction/Hacspec_sha3.fst`,
which is `assume val createi_lemma ... [SMTPat (Seq.index (createi N f) (v i))]`).

For every k that Z3 considers in any forall instantiation (during
loop-invariant well-formedness check, init check, or step check),
the SMTPat fires, evaluating the createi lambda — which contains
`iterate_keccak_f`, division/modulus, machine-int `*!` overflow
checks, and the lane byte indexing.

Empirical:
  - q170 (some loop iter's invariant preservation) cost 50 s on
    cold cache for the inline Portable proof.
  - Subsequent iterations (q197, q223, q231) had similar or longer
    costs.  Eventually the run was killed; total wall time would
    plausibly be in the 10-30 minute range *if it completed at all*.

Per-iteration step lemma (factored standalone, no surrounding
loop-invariant context) verifies in 26 s (Arm64) / 38 s (Portable).
The cascade is specifically about the loop-invariant verification,
not the per-step work.

## Three candidate fixes (smtprofiling techniques)

**Fix A — strip the SMTPat and use a `bring_` helper.**  In
`specs/sha3/proofs/fstar/extraction/Hacspec_sha3.fst`, change
`createi_lemma` from a global SMTPat to a regular lemma + a
`bring_createi_lemma` Classical.forall_intro_2 wrapper.  Consumers
call `bring_createi_lemma ()` only where needed (e.g., inside the
per-byte aux closures of the squeeze proof).  Loop-invariant well-
formedness no longer triggers per-k cascading.

  Tradeoff: every consumer of `createi` (shared spec module) now
  needs explicit `bring_` calls.  Wider blast radius but principled.
  Affects `Hacspec_sha3.Sponge.fst`'s own internal uses (xor_block_into_state,
  squeeze_state etc.).

**Fix B — per-function `--using_facts_from` prune.**  Tried in
this session at `--using_facts_from '* -Hacspec_sha3.createi_lemma'`
on the `squeeze` function alone.  The squeeze body must then call
`createi_lemma` explicitly inside aux closures.  The exact lambda
must match what's in `Hacspec_sha3.Sponge.squeeze`'s body for the
SMT pattern to fire when called manually — fiddly but localized.

  Tradeoff: localized to one function but the explicit invocation
  is awkward (must reconstruct the lambda).

**Fix C — replace createi-based byteform with a sibling primitive
that has a more controlled SMT footprint.**  E.g., expose
`squeeze_byte_at outlen state_init rate k` as a top-level helper
returning the byte directly, with a direct lemma
`Seq.index (squeeze ...) k == squeeze_byte_at ... k` triggered
only when the k is known.  Rewriting `Hacspec_sha3.Sponge.squeeze`
to use this primitive instead of inline createi would re-shape
how the SMTPat fires.

  Tradeoff: another spec rewrite.  Possibly cleanest for the
  long term but biggest churn.

## Diagnostic next step (smtprofiling skill, "Systematic profiling workflow")

```bash
# 1. Add to push-options on the squeeze function (or
#    on lemma_squeeze2_arm64's body once it's a real `let`):
#    "--log_queries --z3refresh --query_stats --split_queries always"
#
# 2. Run make on that one file.  F* dumps queries-*.smt2 files in cwd.
#
# 3. The cascade query (e.g., q170 or q223) gets its own .smt2.  Profile:
z3 smt.qi.profile=true queries-Libcrux_sha3-Generic_keccak-Portable-squeeze-NNN.smt2 2> qi.txt
#
# 4. Parse: top quantifier instantiations
awk '/\[quantifier_instances\]/ { name=$2; count=$3; total[name]+=count }
     END { for (n in total) printf "%8d %s\n", total[n], n }' qi.txt | sort -rn | head -20
#
# 5. Dominant quantifier likely Hacspec_sha3.createi_lemma's underlying
#    SMTPat axiom.  Confirms Fix A/B is the right direction.
```

## Recommended order

1. Diagnostic profile of one cascade query.  Confirms the offending
   quantifier is `createi_lemma`-derived.
2. **Fix A** — strip the SMTPat and provide `bring_createi_lemma`.
   Fix all consumers of `createi` in `Hacspec_sha3.Sponge` to
   explicitly invoke when needed.  This is principled and addresses
   the root cause for all future byteform-style proofs (squeeze4,
   etc.).
3. With Fix A applied, attempt the inline Portable proof again.
   Expected: cascade dies, proof completes in O(seconds) per
   iteration step.
4. Mirror to Arm64 driver via `Simd128.squeeze2` body using
   `lemma_squeeze_one_step_arm64` × 2 lanes per iteration.
5. Avx2 follows the same pattern, gated on the `Libcrux_sha3.Simd.Avx2.fst`
   cold-cache flake (separate AVX2-effort issue).

## Anti-recommendations (tried, didn't work)

  - Just bumping rlimit higher: cascade cost grew per iteration,
    queries q149/q197/q223 each took 24-53 s; later iterations would
    plausibly exceed any reasonable rlimit.
  - `[@@"opaque_to_smt"]` on `iterate_keccak_f` + reveal lemmas:
    helped some queries (q149 went from 24s to ~24s, q170 from
    indefinite stall to 50s) but didn't eliminate the cascade.
    The createi_lemma SMTPat is the deeper trigger.
  - Top-level per-iteration step lemma (the f11e50419 commit)
    plus inline call: the step lemma itself verifies fine in 38 s
    standalone, but using it inline doesn't eliminate the loop
    invariant verification cascade.  The SMTPat fires on every k
    in the invariant's forall, regardless of whether the step's
    work is factored.
