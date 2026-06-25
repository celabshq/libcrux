# Bug-fix demonstrations — fresh session handoff

You are producing **two parallel deliverables** that show libcrux's
F\* verification effort would have caught three real bug-fix PRs:

- https://github.com/cryspen/libcrux/pull/1395 — AVX2 input-reduce 4-of-32
- https://github.com/cryspen/libcrux/pull/1348 — HintBitUnpack cumulative count
- https://github.com/cryspen/libcrux/pull/1347 — γ₁ bound in verify

For every PR, both deliverables must show:

1. **Fixed**: the function (with FIPS-mandated `requires` / `ensures` /
   `loop_invariant`) verifies in F\*.
2. **Buggy**: the same function with the buggy line restored fails F\*
   verification.  Capture the actual F\* error message (not paraphrased).

You may **admit postconditions on inner calls** — the demos are about
the bug shape, not closing surrounding admits.

## Two deliverables

### Deliverable 1 — in-place reproducibility in libcrux

Annotate the production function at the actual callsite, run F\* on the
real codebase, capture the result.  No buggy code committed; the
buggy variant is captured only in the markdown report (with the actual
F\* error from running it transiently).

**Worktrees**:
- `~/libcrux-ml-dsa-above-trait/` (branch `ml-dsa-above-trait`) — for
  PR 1347 and PR 1348.
- `~/libcrux-ml-dsa-proofs/` (branch `ml-dsa-proofs`) — for PR 1395
  (the AVX2 simd impl is below-trait).

Per-PR feasibility (also see § per-PR briefing):
- **PR 1395**: in-place READY today.  `Operations::reduce` already
  verifies; just toggle the loop, capture both outcomes, restore.
- **PR 1347**: in-place HARDER.  `Ml_dsa_generic.verify_internal` is
  body-admitted.  If you can land a focused un-admit (the γ₁ bound
  expression alone) in budget, do it; otherwise note "in-place pending
  verify_internal un-admit" in the report and rely on Deliverable 2 for
  the demo.
- **PR 1348**: in-place HARDER.  `Encoding.Signature.deserialize` body
  is admitted (USER lane per `proofs/outstanding-admits.md`).  Same
  guidance — try a focused un-admit; if not feasible, note + rely on
  Deliverable 2.

Per-PR commit on the relevant branch.  Mirror existing commit-message
style: `git log --oneline -10`.  Don't push.

### Deliverable 2 — standalone reproducible demo folder

A self-contained directory that builds, extracts via hax, and verifies
in F\* with no dependency on libcrux source.  Every reader can clone +
`make verify` and reproduce both fixed-passes and buggy-fails outcomes
end-to-end.

**Path**: `~/libcrux-ml-dsa-bug-demos/` (sibling to other worktrees).
Initialize as a fresh directory; do NOT make it a git worktree of
libcrux.  This folder is its own thing.

**Structure**:

```
libcrux-ml-dsa-bug-demos/
├── README.md                         # overall index + reproduce instructions
├── Cargo.toml                        # depends only on hax-lib
├── rust-toolchain.toml               # pinned (mirror libcrux's)
├── hax.sh                            # extract script (one-line cargo hax invocation)
├── src/
│   ├── lib.rs                        # pub mod pr_1347; pub mod pr_1348; pub mod pr_1395;
│   ├── pr_1347.rs                    # gamma1_bound_fixed + gamma1_bound_buggy
│   ├── pr_1348.rs                    # unpack_one_row_fixed + unpack_one_row_buggy
│   └── pr_1395.rs                    # reduce_loop_fixed + reduce_loop_buggy + stub primitive
├── proofs/fstar/
│   ├── Makefile                      # `make verify` runs F* on all extracted files
│   └── extraction/                   # auto-populated by `hax.sh`
└── docs/
    ├── PR-1347.md
    ├── PR-1348.md
    └── PR-1395.md
```

**Cargo.toml minimal shape** (mirror libcrux's hax-lib pin from
`/Users/karthik/libcrux-ml-dsa-above-trait/libcrux-ml-dsa/Cargo.toml`):

```toml
[package]
name = "libcrux-ml-dsa-bug-demos"
version = "0.0.1"
edition = "2021"
publish = false

[dependencies]
hax-lib = { workspace = false, git = "https://github.com/hacspec/hax.git", rev = "<copy from libcrux>" }
```

**hax.sh** (one-line):

```sh
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
cargo hax into fstar --output-dir proofs/fstar/extraction
```

**Makefile** (in `proofs/fstar/`):

```make
FSTAR_FILES := $(wildcard extraction/Libcrux_ml_dsa_bug_demos.*.fst)

verify:
	@for f in $(FSTAR_FILES); do \
	  echo "=== $$f ==="; \
	  fstar.exe --cache_dir .fstar-cache --include extraction \
	            --include $(HAX_LIB_FSTAR_PATH) \
	            $$f 2>&1 | tee $$f.log; \
	done

clean:
	rm -rf .fstar-cache extraction
```

(Adjust `HAX_LIB_FSTAR_PATH` to point at hax-lib's F\* proof libs;
copy the include flags from
`libcrux-ml-dsa/proofs/fstar/extraction/Makefile`.)

**README.md** at top of standalone folder must include:
- Goal: 3 demos showing F\* catches PR 1347, 1348, 1395.
- Build prerequisites: rustc + hax + fstar.exe + z3.
- One-command reproduce: `./hax.sh && cd proofs/fstar && make verify`.
- Expected output: 3 fixed-passes (per `_fixed` function) + 3 buggy-fails
  (per `_buggy` function), each with the F\* error excerpt embedded
  in `proofs/fstar/extraction/*.fst.log`.
- Pointers to `docs/PR-NNNN.md` for the full per-PR write-up.

## Per-PR briefing

### PR 1395 — AVX2 input-reduce 4-of-32

- **Production location**: `libcrux-ml-dsa/src/simd/avx2.rs::Operations::reduce`
  (helper `reduce_with_proof` ~line 169-188).  The post-`3faaff641`
  refactor introduced a dedicated `arithmetic::reduce` primitive; the
  production reduce loop now calls `arithmetic::reduce(&mut simd_units[i].value)`
  per lane.  `shift_left_then_reduce::<0>` is no longer used here.
- **Bug**: only 4 of 32 simd_units reduced (indices 0, 8, 16, 24).
- **Trait postcondition** (`src/simd/traits.rs::Operations::reduce`):
  ```
  forall (j:nat). j < 32 ==>
    Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
      (Seq.index $simd_units j) (Seq.index ${simd_units}_future j)
  ```
- **Fixed body** (current production):
  ```rust
  for i in 0..simd_units.len() {
      hax_lib::loop_invariant!(...);  // forall k<i. reduce_lane_post old[k] new[k]
      arithmetic::reduce(&mut simd_units[i].value);
  }
  ```
- **Buggy body** (revert to 4 explicit calls):
  ```rust
  arithmetic::reduce(&mut simd_units[0].value);
  arithmetic::reduce(&mut simd_units[8].value);
  arithmetic::reduce(&mut simd_units[16].value);
  arithmetic::reduce(&mut simd_units[24].value);
  ```
- **Standalone version (Deliverable 2)**: stub the primitive — `fn
  reduce_lane(b: &mut i32)` with admit'd `ensures (-FIELD_MAX <= v
  ${*b}_future <= FIELD_MAX)`.  Prove the wrapper post `forall i<32.
  -FIELD_MAX <= v arr_future[i] <= FIELD_MAX` for the 32-lane loop.
  The buggy variant fails the wrapper post on the 28 unreduced indices
  (which retain their pre-bound, e.g. `2 * FIELD_MAX`).

### PR 1348 — HintBitUnpack cumulative count (panic-freedom)

- **Production location**: `libcrux-ml-dsa/src/encoding/signature.rs::deserialize`
  (the hint_bit_unpack portion).  Body-admitted today.
- **Bug**: check is `previous_true_hints_seen < max_ones_in_hint`; fix
  is `current_true_hints_seen <= max_ones_in_hint`.  Per FIPS 204
  Algorithm 22, cumulative hint counters must be non-decreasing and
  bounded by ω.  The buggy check fails to validate the final
  cumulative counter, allowing OOB read on the inner
  `serialized[indices_offset + idx]` when `idx >= max_ones_in_hint`.
- **Demo shape (Shape B — panic-freedom)**, identical for in-place and
  standalone:

  ```rust
  // FIXED — both bounds enforced; the loop's slice index is in-range.
  #[hax_lib::requires(fstar!(r#"
      Seq.length $serialized >= v $indices_offset + v $max"#))]
  pub fn unpack_one_row_fixed(
      serialized: &[u8],
      prev: u8,
      curr: u8,
      max: u8,
      indices_offset: usize,
  ) -> Result<(), ()> {
      if curr < prev || curr > max {
          return Err(());
      }
      let mut idx = prev;
      while idx < curr {
          hax_lib::loop_invariant!(|idx: u8| fstar!(r#"
              v $idx >= v $prev /\ v $idx <= v $curr /\ v $curr <= v $max"#));
          let _ = serialized[indices_offset + idx as usize];
          idx += 1;
      }
      Ok(())
  }

  // BUGGY — F* error: slice-index pre on `serialized[indices_offset + idx as usize]`
  // fails because `v prev < v max` does not bound `v idx + v indices_offset`.
  #[hax_lib::requires(fstar!(r#"
      Seq.length $serialized >= v $indices_offset + v $max"#))]
  pub fn unpack_one_row_buggy(
      serialized: &[u8],
      prev: u8,
      curr: u8,
      max: u8,
      indices_offset: usize,
  ) -> Result<(), ()> {
      if curr < prev || prev >= max {
          return Err(());
      }
      let mut idx = prev;
      while idx < curr {
          hax_lib::loop_invariant!(|idx: u8| fstar!(r#"
              v $idx >= v $prev /\ v $idx <= v $curr /\ v $prev < v $max"#));
          let _ = serialized[indices_offset + idx as usize];
          idx += 1;
      }
      Ok(())
  }
  ```

### PR 1347 — γ₁ bound in verify_internal

- **Production location**: `libcrux-ml-dsa/src/ml_dsa_generic.rs::verify_internal`,
  in the γ₁ bound calculation feeding `infinity_norm_exceeds(gamma1 - beta)`.
  Body-admitted today.
- **Bug**: `gamma1 = 2 << GAMMA1_EXPONENT` (doubles the bound); fix is
  `gamma1 = 1 << GAMMA1_EXPONENT`, i.e. 2^(e_γ₁) per FIPS 204 §6.3.
- **Demo shape**, identical for in-place and standalone:

  ```rust
  /// FIXED — discharges the FIPS bound `r == 2^exp`.
  #[hax_lib::requires(fstar!(r#"v $exp <= 19"#))]
  #[hax_lib::ensures(|r| fstar!(r#"v $r == pow2 (v $exp)"#))]
  pub fn gamma1_bound_fixed(exp: usize) -> i32 {
      1 << exp
  }

  /// BUGGY — F* error: ensures becomes `2 * pow2 exp == pow2 exp` (false for exp > 0).
  #[hax_lib::requires(fstar!(r#"v $exp <= 19"#))]
  #[hax_lib::ensures(|r| fstar!(r#"v $r == pow2 (v $exp)"#))]
  pub fn gamma1_bound_buggy(exp: usize) -> i32 {
      2 << exp
  }
  ```

## Markdown report (per PR, both deliverables)

For Deliverable 1: `libcrux-ml-dsa/proofs/bug-demos/PR-<NNNN>.md` in the
relevant worktree.

For Deliverable 2: `~/libcrux-ml-dsa-bug-demos/docs/PR-<NNNN>.md`.

Sections:

```markdown
# PR <NNNN> — <one-line bug title>

**Production PR**: https://github.com/cryspen/libcrux/pull/<NNNN>
**FIPS reference**: <FIPS 204 §...>
**Bug class**: <functional spec | panic-freedom | trait-post>

## Original (buggy) code

`<file>:<line>` (production):
```rust
<the buggy line(s) as quoted from the PR diff>
```
Why it's wrong: <one paragraph>.

## Final (fixed) code

`<file>:<line>` (production):
```rust
<the fixed line(s)>
```
Why it works: <one paragraph>.

## F\* annotation

```
requires: <pre>
ensures:  <post>
loop_invariant (if any): <inv>
```

## F\* result on FIXED

```
<copy-pasted output: Query-stats line, "Verified module: ...",
 "All verification conditions discharged successfully">
```
Time: <ms>.  rlimit: <n>.

## F\* result on BUGGY

```
<copy-pasted error: file:line, failing query, the assertion that did
 not discharge, Z3 counterexample if reported>
```

## Demo location

- Deliverable 1 (in-place): `<file>:<line>` (or "pending un-admit of <module>")
- Deliverable 2 (standalone): `~/libcrux-ml-dsa-bug-demos/src/pr_<NNNN>.rs`
```

## Index README

- Deliverable 1: `libcrux-ml-dsa/proofs/bug-demos/README.md` —
  table | PR | bug class | in-place verifies | buggy fails | report |
- Deliverable 2: `~/libcrux-ml-dsa-bug-demos/README.md` — overall
  intro + reproduce-from-scratch instructions, plus the same table
  pointing at `docs/PR-<NNNN>.md`.

## Hard rules

1. **No buggy production code committed.**  Buggy variants live in
   the standalone folder's `_buggy` named functions and in the
   markdown reports.  Production code stays clean.
2. **Capture actual F\* output**, not paraphrased.  The reports are
   demonstrations — copy-paste the stdout/stderr.
3. **Status reports every 15 min** to `<worktree>/AGENT_STATUS.md` if
   you spawn sub-agents (per user memory `feedback_agent_status_reports.md`).
4. **20-min wall-clock per demo per deliverable.**  If a demo doesn't
   close, document the blocker in the markdown report and move on.
5. **Don't bulk-delete `.checked` files** (per user memory
   `feedback_no_cache_nuke.md`).
6. **PR 1395 in-place changes belong on `ml-dsa-proofs` only**;
   PR 1347 and 1348 in-place go on `ml-dsa-above-trait`.  Don't
   cross-pollinate.  The standalone folder is independent of both
   branches.
7. Read user memory before starting:
   `~/.claude/projects/-Users-karthik-libcrux/memory/{feedback_lane_split,
   feedback_no_cache_nuke,feedback_use_fstar_mcp,feedback_agent_status_reports,
   feedback_develop_locally_upstream_once}.md`.

## Verification reproducibility checklist

A reader should be able to:

**Deliverable 1**:
```sh
cd ~/libcrux-ml-dsa-above-trait && git checkout ml-dsa-above-trait
# read libcrux-ml-dsa/proofs/bug-demos/PR-1347.md
# follow its "F* result on FIXED" reproduce steps
cd libcrux-ml-dsa && ./hax.sh extract
cd libcrux-ml-dsa/proofs/fstar/extraction
make .../Libcrux_ml_dsa.<Module>.fst.checked
# observe the same Query-stats output as the report
```

**Deliverable 2**:
```sh
cd ~/libcrux-ml-dsa-bug-demos
./hax.sh
cd proofs/fstar
make verify
# observe 3 _fixed verifies + 3 _buggy fails, each logged
```

The reports' "F\* result" sections should match what the reader sees
verbatim.

## Final report (end of session)

Print:

- Branch + final tip SHA per worktree (above-trait, below-trait).
- Standalone folder path + git-init / commit state.
- Per-PR per-deliverable outcome (✅ in-place / ✅ standalone / 📝 documented but pending un-admit).
- Commits made (SHA + subject).
- Confirmation that no buggy production code was left committed.
- Path to all four READMEs (1 per worktree + 1 standalone) and 6 PR-NNNN.md files (3 per deliverable).

## Useful pointers

- `libcrux-ml-dsa/proofs/outstanding-admits.md` — full admit inventory
  (notes on which modules are body-admitted today and why).
- `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` —
  above-trait / below-trait sync protocol.
- `libcrux-ml-dsa/MLDSA_STATUS.md` — module-level CHECK/ADMIT status.
- For copying hax-lib pin / cargo workspace settings, look at
  `~/libcrux-ml-dsa-above-trait/libcrux-ml-dsa/Cargo.toml`.
- For copying F\* makefile include flags, look at
  `~/libcrux-ml-dsa-above-trait/libcrux-ml-dsa/proofs/fstar/extraction/Makefile`.
