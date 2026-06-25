# F-14 — `error_deserialize` trait post relax (above-trait lane)

You are the **above-trait** ML-DSA proof agent.  Below-trait
(branch `ml-dsa-proofs` in
`/Users/karthik/libcrux-ml-dsa-proofs/`) has filed an audit
finding **F-14** in
`libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md`
(below-trait commit `8b108e6d2`) — the `error_deserialize` trait
post is **wrong** w.r.t. FIPS 204 Algorithm 19 (BitUnpack) and
the Hacspec `bit_unpack` spec.  Fix lives entirely in
`src/simd/traits.rs`; below-trait will then close the trait body
admits.

**Numbering note:** F-14 below-trait is **not the same** as your
lane's F-14 (`above-trait: F-14 — Signing_key.generate_serialized
admit closed`, commit `8fa040756`).  Read the below-trait
`lane-split-protocol.md` entry as the authoritative spec.

## Pre-flight

```bash
cd <your-above-trait-worktree>
git rev-parse HEAD                                 # confirm clean
git log --oneline | head -10                       # confirm tip
# Read the below-trait F-14 audit:
cat /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md \
  | sed -n '/^#### F-14/,/^#### F-/{ /^#### F-13/q; p }'
# Run a baseline prove to confirm 0 errors:
JOBS=2 ./hax.sh prove 2>&1 > /tmp/f14-baseline.log
grep -E "F\* errors|Make-level" /tmp/f14-baseline.log
```

If the prove is dirty, fix first.  Only proceed once 0 errors.

## The change

The current `error_deserialize` trait post in
`libcrux-ml-dsa/src/simd/traits.rs` is (search for
`fn error_deserialize`):

```rust
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      ($eta == Libcrux_ml_dsa.Constants.Eta_Two ==>
          v (Seq.index (f_repr ${out}_future) i) >= -2 /\
          v (Seq.index (f_repr ${out}_future) i) <= 2) /\
      ($eta == Libcrux_ml_dsa.Constants.Eta_Four ==>
          v (Seq.index (f_repr ${out}_future) i) >= -4 /\
          v (Seq.index (f_repr ${out}_future) i) <= 4))"#))]
fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Self);
```

**Replace** the per-eta lower bounds `>= -2` / `>= -4` with the
FIPS-correct asymmetric range:

```rust
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      ($eta == Libcrux_ml_dsa.Constants.Eta_Two ==>
          v (Seq.index (f_repr ${out}_future) i) >= -5 /\
          v (Seq.index (f_repr ${out}_future) i) <= 2) /\
      ($eta == Libcrux_ml_dsa.Constants.Eta_Four ==>
          v (Seq.index (f_repr ${out}_future) i) >= -11 /\
          v (Seq.index (f_repr ${out}_future) i) <= 4))"#))]
fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Self);
```

That is, change two literals: `-2 → -5`, `-4 → -11`.  Upper
bounds (`<= 2`, `<= 4`) stay the same.

## Why this is correct

FIPS 204 Algorithm 19 (BitUnpack) defines the output as
`w[i] ∈ [b - 2^c + 1, b]` where `c = bitlen(a + b)`:

| eta | a | b | a+b | c=bitlen | 2^c | range            |
|-----|---|---|-----|----------|-----|------------------|
| 2   | 2 | 2 | 4   | 3        | 8   | `[2-7, 2] = [-5, 2]`  |
| 4   | 4 | 4 | 8   | 4        | 16  | `[4-15, 4] = [-11, 4]` |

The canonical `[-eta, eta]` shape only holds when `a + b + 1` is
a power of 2 — for ML-DSA values 5 and 9, neither is.  The
Hacspec doc comment at `specs/ml-dsa/src/encoding.rs:82-83`
calls this out explicitly.

## Audit of callers

Below-trait already audited every caller of `error_deserialize`
above the trait — no caller observes the `[-eta, eta]` form:

- `src/encoding/error.rs::deserialize` wrapper has no `#[ensures]`.
- `src/encoding/error.rs::deserialize_to_vector_then_ntt` is
  `verification_status(panic_free)` + body `admit ()`.
- `src/ml_dsa_generic.rs::sign_internal` is body-admitted upstream
  of the calls.
- AVX2 `rejection_sample::less_than_eta::sample` does NOT use the
  trait method (uses `error::deserialize_to_unsigned` separately,
  which has its own bit-vec post and explicit interval validation).

So this change is non-breaking by audit.

## Recommended workflow

This is a focused one-method change.  You may either do it
directly or **spawn a sub-agent** to handle it in isolation.  If
spawning, brief like:

> "Edit `libcrux-ml-dsa/src/simd/traits.rs` `error_deserialize`
> ensures: change the eta=Two lower bound from `-2` to `-5` and
> the eta=Four lower bound from `-4` to `-11`.  Upper bounds
> unchanged.  Then run `./hax.sh extract && ./hax.sh prove` and
> confirm 0 F\* errors.  Commit with message format
> `above-trait: F-14 (below-trait) — error_deserialize post relax
> to FIPS BitUnpack range`.  Do **not** edit any other file."

## After the trait edit

1. **Re-extract**: `./hax.sh extract` (only `Simd.Traits.fst`
   changes).
2. **Verify above-trait**: `JOBS=2 ./hax.sh prove`.  Expected
   result: 0 F\* errors.  No above-trait module currently
   consumes the `[-eta, eta]` post (per the caller audit), so
   nothing should break.  If something *does* break, the caller
   was depending on a (false) tightening — flag it.
3. **Commit** in the above-trait lane.
4. **Handoff to below-trait** (this branch).  Below-trait will
   cherry-pick the `traits.rs` change, then close the
   `error_deserialize` trait body admits using the same template
   as the Track D-2 t1/t0/gamma1 deserialize closes:
   - Add per-eta ensures to
     `src/simd/portable/encoding/error.rs::deserialize` matching
     the new FIPS range.
   - Add per-eta ensures to
     `src/simd/avx2/encoding/error.rs::deserialize` (similarly).
   - Drop the `hax_lib::fstar!("admit ()")` in
     `src/simd/{portable,avx2}.rs::error_deserialize` and
     reveal the relevant opaque (if any).
   - Expected: 2 trait body admits removed.

## Hard rules

1. **Edit only `src/simd/traits.rs`**.  Do **not** touch
   `src/simd/traits/specs.rs` predicate definitions — none are
   needed (the bound is direct `>=` / `<=` on `v out[i]`, no
   opaque predicate).
2. **Do not adjust upper bounds.**  `<= 2` (eta=2) and `<= 4`
   (eta=4) match FIPS exactly.
3. **One commit, focused.**  Match the style of prior above-trait
   F-N commits (e.g., `8fa040756`, `577a112cf`).

## Files to read first

1. The below-trait F-14 entry in
   `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md`
   (search for `#### F-14 (2026-04-29) — error_deserialize trait
   post over-tight`).
2. FIPS 204 Algorithm 19 (BitUnpack) — confirms the
   `[b - 2^c + 1, b]` output range.
3. `specs/ml-dsa/src/encoding.rs:82-83` — Hacspec doc comment.
4. `libcrux-ml-dsa/src/simd/traits.rs` — the file you'll edit.

## Style

User-facing text terse.  No trailing summaries — diff + prove
output speak for themselves.
