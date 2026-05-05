# Next-session prompt — fully verify `serialize_1`, `deserialize_5`, and (stretch) discharge the storeu/loadu axioms

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry (post-merge of sprint 2026-05-10):** the merge commit
of `agent-mlkem-avx2-serialize-2026-05-10` (or later).  Concretely
HEAD must include commits `9ba739333` (serialize_5 fully proven),
`636b4042f` (`_11`/`-_11` fully proven), and `1170d2df6` (status doc).
Verify by `grep -l mm256_storeu_si256_i16_post_axiom
libcrux-ml-kem/src/vector/avx2/serialize.rs` returning the path.

**Scope (3 sites):**

| Site | Status today | Goal |
|---|---|---|
| `src/vector/avx2/serialize.rs:5` `serialize_1`     | `lax` (transient FOLLOW-UP from prior sprint)                       | `panic_free` minimum, fully proven ideal |
| `src/vector/avx2/serialize.rs:468` `deserialize_5` | `panic_free` (BitVec ensures admitted)                              | fully proven (Math tier with full BitVec ensures) |
| `src/vector/avx2/serialize.rs:683` storeu/loadu axioms (stretch) | local `admit ()` in `before` block on `serialize_11` | upstream into `Libcrux_intrinsics.Avx2_extract.fsti` `val`s |

**Out of scope:** every other lax site (`avx2.rs:65,1140,1153` `to_bytes`/`from_bytes` blocked on hax-lib upstream; NTT admits in `portable.rs`; Generic-side admits — separate sprints).

## Branch hygiene — mandatory

The user works in `/Users/karthik/libcrux-trait-opacify` on parallel
tasks.  **Do NOT operate on the shared worktree.** Per
`feedback_branch_means_worktree`:

```bash
git -C /Users/karthik/libcrux-trait-opacify worktree add \
    /Users/karthik/libcrux-serialize-1-deserialize-5 \
    -b agent-mlkem-serialize-1-deserialize-5-2026-05-11
cd /Users/karthik/libcrux-serialize-1-deserialize-5/libcrux-ml-kem
```

If the worktree directory exists from a prior abandoned session, prompt
the user before reusing — do not blindly `git worktree remove`.

## Read first (non-negotiable)

1. **`~/.claude/skills/fstar-for-libcrux/README.md`** — Rules 1-8.
   Especially Rule 5 (pipe `make` to log + grep, never `Read` full
   make log) and Rule 7 (`fstar-mcp` for symbol lookups, full `make`
   for verification gate).

2. **`MEMORY.md`** entries:
   - `feedback_panic_free_vs_lax` — `panic_free` admits ensures, `lax`
     admits everything.  Default flip target = `panic_free`.
   - `feedback_proof_debug_budget` — 30-60 min hard cap per fn debugging
     F*/hax proofs.  Per-fn budget for this sprint:
     45-60 min for `serialize_1` (stabilise tactic) →
     60-90 min if going for fully proven →
     60-90 min for `deserialize_5` (touches shared
     `BitVec.Intrinsics.fsti`) →
     30 min for stretch storeu/loadu upstream.
   - `feedback_branch_means_worktree`, `feedback_grep_make_output`,
     `feedback_rlimit_cap_800` (≤800 in any annotation, ≤400 with
     `--split_queries always`), `feedback_smtpat_percent_above_trait`.
   - `feedback_serialize_admit_other_fns` — wrap every non-target
     function in `--admit_smt_queries true` push-options when
     iterating; each F* run must stay <5 min wall; UNDO before commit.
     This is what unblocked path 1 in the prior sprint.

3. **Recent context:**
   - `proofs/agent-status/sprint-2026-05-10-status.md` — full record of
     the prior sprint: lessons learned (concat_pairs_n vs madd_epi16,
     shuffle_epi32 has no BitVec spec, rlimit 400 for forall_n 40),
     plus the FOLLOW-UP list this sprint addresses.
   - The three sites' specs/bodies are stable — no changes expected
     to the requires/ensures clauses.

## Site 1: `serialize_1` (line 5) — stabilise + (stretch) fully prove

The prior sprint reverted this from `panic_free` → `lax` because the
existing `prove_forall_nat_pointwise` tactic on the body's intermediate
assertion fails Z3 deterministically at `i=1` with "incomplete
quantifiers" (uses ~1.5 of 80 rlimit then gives up).  Earlier verified
runs passed via hint replay; once those hints were invalidated by any
edit to the module, the proof stopped going through.

**Primary goal: re-flip to `panic_free`, hint-independent.**

Approaches (in order of expected difficulty):

(A) **Stabilise the existing tactic.**  Tweak the proof so Z3 doesn't
need stale-hint magic.  Suggested edits:

```rust
hax_lib::fstar!(r#"
let bits_packed' = BitVec.Intrinsics.mm_movemask_epi8_bv msbs in
  // Per-i lemma instead of the forall_nat_pointwise tactic.  Each
  // sub-goal is concrete enough that compute() handles it without
  // quantifier instantiation.
  introduce forall (i: nat{i < 16}). bits_packed' i = $vector ((i / 1) * 16 + i % 1)
  with assert_norm (BitVec.Utils.forall_n 16 (fun i ->
    BitVec.Intrinsics.mm_movemask_epi8_bv msbs i = $vector ((i / 1) * 16 + i % 1)))
"#);
```

The `assert_norm forall_n 16` style is what worked for `serialize_5`'s
SIMD chain in the prior sprint — it unrolls the per-i case and lets
F*'s normaliser handle the bit-level reduction.  The `mm_packs_epi16`
saturation semantics may need a per-lane lemma to get fully concrete;
see Approach (C).

(B) **Add `--z3refresh` and bump rlimit.**  Already tried in the prior
sprint (no help — `--z3refresh` alone didn't fix it; rlimit isn't the
issue since only 1.5 of 80 is used).  Skip.

(C) **Replace the tactic with explicit per-lane lemmas.** For each
i ∈ [0, 16), an explicit lemma about `mm_packs_epi16`'s signed-saturation
behaviour bridging bit `i*16+15` of the original `vector` to bit `i*8+7`
of the packed result, then `mm_movemask_epi8_bv` extracting the MSB
gives the bit at position `i` of `bits_packed'`.  More work but
deterministic.

**Stretch (full ensures):** if the body is panic-free, push to fully
proven by extending the assertion to include the full
`bit_vec_of_int_t_array result 8 i == vector (i * 16)` chain.  The
existing second `hax_lib::fstar!` block (about
`bits_packed >>! 8`) is already in place; the missing piece is the
`as u8` cast bridge from `bits_packed: i32` to bytes.  Likely needs a
`Rust_primitives.cast_u8_get_bit` style lemma + range bound on
`bits_packed`.

**Acceptance:** `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst`
rc=0 from a clean cache (no hints).  Re-run after `rm
.fstar-cache/hints/Libcrux_ml_kem.Vector.Avx2.Serialize.fst.hints` to
prove hint-independence.

## Site 2: `deserialize_5` (line 468) — fully prove

Body chain:
```rust
mm_set_epi8(bytes[9], bytes[8], bytes[8], bytes[7], …)
  → mm256_si256_from_two_si128(c, c)
  → mm256_shuffle_epi8
  → mm256_mullo_epi16(_, mm256_set_epi16(1<<0,1<<5,1<<2,1<<7,1<<4,1<<9,1<<6,1<<11,…))
  → mm256_srli_epi16<11>
```

**Blocker (per prior sprint):** the multiplier
`(1<<0, 1<<5, 1<<2, 1<<7, 1<<4, 1<<9, 1<<6, 1<<11, …)` matches none
of `mm256_mullo_epi16_specialized{1,2,3}` in
`fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti`, so
`mm256_mullo_epi16` resolves to `mm256_mullo_epi16_no_semantics` and
the assert_norm chain breaks.

**Step 1:** add `mm256_mullo_epi16_specialized4` to
`BitVec.Intrinsics.fsti`.  The shape:

```fstar
(* For deserialize_5: shifts each 16-bit lane by `(k%2)*5 + (k/2)*2`
   where k is the lane index (0..7), repeated for the upper 128-bit
   half.  Output bit at position i (in 0..255) is set iff input bit
   at position (i - shift_k) is set, where shift_k is determined by
   k = i / 16. *)
let mm256_mullo_epi16_specialized4 (a: bit_vec 256): bit_vec 256 =
  mk_bv (fun i ->
    let nth_bit = i % 16 in
    let k = (i / 16) % 8 in
    let shift = (k % 2) * 5 + (k / 2) * 2 in
    if nth_bit >= shift then a (i - shift) else 0)
```

(Verify the shift formula against the actual multiplier
constants.  The pattern in the source is:

  Lane 0: `1<<0`,  Lane 1: `1<<5`,  Lane 2: `1<<2`,  Lane 3: `1<<7`,
  Lane 4: `1<<4`,  Lane 5: `1<<9`,  Lane 6: `1<<6`,  Lane 7: `1<<11`,

repeated for lanes 8-15.  Walk through the F* spec
of `mm256_mullo_epi16` to confirm bit positions.)

Then add a 4th `unify_app` arm to the `mm256_mullo_epi16` tactic that
matches this pattern and emits
`Tactics.exact (quote (mm256_mullo_epi16_specialized4 a))`.

**Step 2:** factor an inner `deserialize_5_vec` helper in
`src/vector/avx2/serialize.rs` mirroring `deserialize_10_vec` /
`deserialize_12_vec`.  Helper signature suggestions:

```rust
#[hax_lib::fstar::options("--ext context_pruning --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 10"#))]
#[hax_lib::ensures(|result| fstar!(r#"
  forall (i: nat{i < 256}).
    $result i = (if i % 16 >= 5 then 0
                 else let j = (i / 16) * 5 + i % 16 in
                      bit_vec_of_int_t_array (Seq.upd
                          (Seq.upd … bytes …) … …) 8 j)
"#))]
fn deserialize_5_vec(bytes: &[u8]) -> Vec256 { … }
```

Actual ensures shape will follow from the `assert_norm forall256`
chain — the prior sprint's `serialize_5_vec` factor / `forall_n 80`
pattern is the template.

**Acceptance:** `make check/...` rc=0; `deserialize_5` no longer has
`verification_status(panic_free)`; `verification_status.md` shows
`deserialize_5` in Math tier.

**Difficulty:** medium-hard — touches shared
`BitVec.Intrinsics.fsti`, may need 2-3 iterations on the
`mm256_mullo_epi16_specialized4` shape.  Estimate: 60-120 min.

## Stretch: discharge storeu/loadu axioms

The prior sprint added two local axioms in `serialize.rs`'s `before`
block on `serialize_11` (committed as `636b4042f`):

```fstar
let mm256_storeu_si256_i16_post_axiom output vector
  : Lemma (requires len output == 16)
          (ensures storeu output vector == vec256_as_i16x16 vector)
  = admit ()

let mm256_loadu_si256_i16_post_axiom input
  : Lemma (requires len input == 16)
          (ensures vec256_as_i16x16 (loadu input) == input)
  = admit ()
```

These are honest axioms about the AVX2 intrinsics' semantics — the
canonical place for them is in `Libcrux_intrinsics.Avx2_extract.fsti`'s
`val` ensures clauses for `mm256_storeu_si256_i16` and
`mm256_loadu_si256_i16`.  Currently those vals only state length
preservation.

**Step 1:** Strengthen the val ensures in
`crates/utils/intrinsics/proofs/fstar/extraction/Libcrux_intrinsics.Avx2_extract.fsti`.
Move from:

```fstar
val mm256_storeu_si256_i16 (output: t_Slice i16) (vector: t_Vec256)
    : Prims.Pure (t_Slice i16) Prims.l_True
      (ensures fun output_future ->
         len output_future == len output)
```

to:

```fstar
val mm256_storeu_si256_i16 (output: t_Slice i16) (vector: t_Vec256)
    : Prims.Pure (t_Slice i16) Prims.l_True
      (ensures fun output_future ->
         len output_future == len output /\
         (len output == 16 ==>
            (output_future <: Seq.seq i16) == (vec256_as_i16x16 vector <: Seq.seq i16)))
```

and symmetrically for `mm256_loadu_si256_i16`.

NOTE: this file is auto-generated from `crates/utils/intrinsics/src/avx2_extract.rs`
by hax.  The strengthening must happen in the Rust source's `#[hax_lib::ensures]`,
not by editing the generated `.fsti` directly.  Alternatively, an after-block
axiom in the consumer files works too.

**Step 2:** Remove the local axioms from `serialize.rs`'s `before`
block on `serialize_11`; the body proofs should still go through using
the strengthened intrinsic spec.

**Acceptance:** `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst`
rc=0 with the two `admit ()` lines removed.  Touches multiple
crates — confirm with the user before flipping the intrinsics file.

**Difficulty:** low if the intrinsics' source-of-truth is the Rust
file (just add `#[hax_lib::ensures(...)]`); medium if hax is
unfriendly about quantified ensures over slice contents.  Estimate:
30-60 min.

## Stage acceptance + commit hygiene

Per site:
1. `python3 hax.py extract` (re-extract).
2. `cd proofs/fstar/extraction && make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst > /tmp/avx2-ser.log 2>&1; echo rc=$?`.
3. Once rc=0, commit per site.

Suggested commit messages (one per site):
- `agent-mlkem: stabilise serialize_1 tactic (lax → panic_free, hint-independent)`
- `agent-mlkem: discharge serialize_1 BitVec ensures (panic_free → fully proven)`  *(if stretch is reached)*
- `agent-mlkem: discharge deserialize_5 SIMD body (panic_free → fully proven via mm256_mullo_epi16_specialized4)`
- `agent-mlkem: upstream storeu/loadu strengthening into Libcrux_intrinsics.Avx2_extract` *(if stretch is reached)*

Final rollup if multiple sites land:
- `agent-mlkem: sprint 2026-05-11 — serialize_1 stabilised + deserialize_5 fully proven`

If only some sites land within budget, document deferred sites in
`proofs/agent-status/sprint-2026-05-11-rollup.md`.

## Pre-session checklist

- [ ] Worktree created (per Branch hygiene above).  Confirm `pwd`.
- [ ] Tip: `git log -1 --oneline` includes `mm256_storeu_si256_i16_post_axiom`
      in `serialize.rs` (commit `636b4042f` is in ancestry).
- [ ] Read `fstar-for-libcrux` skill.
- [ ] Read MEMORY.md entries above + `sprint-2026-05-10-status.md`.
- [ ] Run baseline `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst`
      from `proofs/fstar/extraction/`; should show rc=0 with serialize_1
      lax (and deserialize_5 panic_free).  IF NOT — copy hints from
      `/Users/karthik/libcrux-trait-opacify/.fstar-cache/hints/` BEFORE
      starting; otherwise serialize_1's tactic will block until you
      stabilise it.
- [ ] Pick site to start with — recommended order:
      `serialize_1` stabilisation → `deserialize_5` fully proven →
      stretch storeu/loadu upstream.  `deserialize_5` first if you
      want the harder problem out of the way early.

## Status reports (live)

Per `feedback_agent_status_reports`: every 15 min, append a 3-line
status to `proofs/agent-status/sprint-2026-05-11-status.md`:
- Current site (which fn).
- Blocker if any (specific Z3 query / lemma).
- ETA for current site.

## Key file paths quick reference

- `src/vector/avx2/serialize.rs` — sprint surface.
- `src/vector/portable/serialize.rs` — portable counterparts (specs
  used by deserialize_5_vec; `serialize_11_lemma` /
  `deserialize_11_lemma` already used by the prior sprint).
- `fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti` — for
  `mm256_mullo_epi16_specialized4` addition.
- `crates/utils/intrinsics/src/avx2_extract.rs` — for storeu/loadu val
  strengthening (Rust source, hax-extracted).
- `crates/utils/intrinsics/proofs/fstar/extraction/Libcrux_intrinsics.Avx2_extract.fsti` — generated from above.
- Existing AVX2 templates (mirror for deserialize_5):
  `deserialize_10_vec` line 626, `deserialize_12_vec` line 773 in
  `src/vector/avx2/serialize.rs`.
- Sprint 2026-05-10 retrospective:
  `proofs/agent-status/sprint-2026-05-10-status.md`.
