# AVX2 load_block third cascade — Phase 2 status (2026-05-05)

## Headline

Third cascade source identified, partial fix landed. Filtering
`Rust_primitives.Slice.array_from_fn` cuts the load_block sub-query
cost from ~100s/query (cliff) down to ~70ms/query for the bulk of
load_block — but **4 per-iteration assertion sub-queries remain
failing** (canceled at 400/400 rlimit, ~100s each). The remaining
cliff source is the F* SMT-prelude axiom `;;fuel irrelevance`,
which lives upstream of hax-lib and is per-brief out of scope for
this worktree.

## Prereq commits intact

Verified both inherited fixes are still active:
- `7bb581f8b` `[@@ "opaque_to_smt"]` on `createi` — confirmed in
  `specs/sha3/proofs/fstar/extraction/Hacspec_sha3.fst`.
- `8203c9ace` `get_lane_u64_post` SMTPat lemma — confirmed in
  `crates/utils/intrinsics/proofs/fstar/extraction/Libcrux_intrinsics.Avx2_extract.fsti`
  with body in `.fst`, `[SMTPat (get_lane_u64 vec lane)]`.
  qi.profile shows `lemma_..._get_lane_u64_post` at 195K
  instantiations vs the prior `199K`-shape, so the lemma IS firing.

## Third cascade source

**`k!61`** — the F* SMT-prelude `;;fuel irrelevance` axiom:

```smt
(assert (forall ((f Fuel) (x Term) (t Term))
 (! (= (HasTypeFuel (SFuel f) x t)
       (HasTypeZ x t))
  :pattern ((HasTypeFuel (SFuel f) x t)))))
```

Verified by Z3 trace on the failing query (q-692.smt2):
```
[mk-app] #71 HasTypeFuel ZFuel #68 #69
[mk-app] #75 HasTypeFuel #73 #69 #68      ; #73 = MaxIFuel
[mk-app] #77 SFuel #76                     ; #76 = var 2
[mk-app] #78 HasTypeFuel #77 #68 #69
[mk-app] #79 = #78 #71
[mk-app] #80 pattern #78
[mk-quant] #81 k!61 3 #80 #79
```

This axiom has **no `:qid`** and a single very-broad pattern that
fires on every `HasTypeFuel (SFuel _) _ _` term in the goal —
including all transitive type-refinements from
`Rust_primitives.Slice.array_from_fn`,
`Core_models.Num.impl_u64__rem_euclid`,
`Rust_primitives.Integers.range_t`,
`Rust_primitives.Arrays.t_Array`, and any user-defined
refinement that materialises HasType.

In the load_block forall-25 goal, this fires **~1.1M times**
(qi.profile measurement, max-generation 11), accounting for ~12% of
total instantiations and the dominant Z3 wallclock cost.

The fix is upstream of hax-lib (it lives in F*'s SMT prelude,
emitted by `FStar.SMTEncoding.Encode`); modifying it is per the
brief's "out of scope; surface to user" clause.

## Fix landed

**Option (c) from the brief: per-fn filter.**

```rust
#[hax_lib::fstar::options(
  "--z3rlimit 400 --split_queries always \
   --using_facts_from '* -Rust_primitives.Slice.array_from_fn \
                         -Core_models.Num.impl_u64__rem_euclid \
                         -Core_models.Num.impl_u32__rem_euclid'")]
```

on `crates/algorithms/sha3/src/simd/avx2.rs::load_block`. (Filtering
out `Rust_primitives.Slice.array_from_fn` does the bulk of the work;
adding `Core_models.Num.impl_u64__rem_euclid` is a marginal further
defense.)

Effect:
- Pre-fix: 4 per-iteration assertion sub-queries cancel at 400/400
  rlimit, ~100s each. (qs 693-695 line 1091; q 796 line 1164.)
  Total cliff cost: ~6-8 min for the load_block prove.
- Post-fix: bulk of load_block subqueries now succeed in ~70ms
  (used_rlimit 0.43-0.56 vs 400.0 before).
- **Remaining**: 4 sub-queries at line 1091 (the first per-iteration
  `hax_lib::assert!` for index `4*i` lanes 0-3) still cancel at
  400/400 in ~100s.  Lines 1164, 1249, 1351 (the other three
  per-iteration asserts) appear to also fail; make timed out at 480s
  before reaching them. Same shape, same root cause.

## Fix attempts that didn't close

### (b) Opacify `load_lane_u64` via `[@@ "opaque_to_smt"]`

Reasoning: load_lane_u64's body is XOR(get_lane_u64, from_le_bytes).
Opacifying makes load_lane_u64 an uninterpreted symbol everywhere,
which would prevent its body from contributing HasTypeFuel terms.

Result: extraction succeeds, but **load_block's per-iteration
assertions then fail with `incomplete quantifiers` in ~24s** (not
cliff — completeness). Reason:
- Post of `load_u64x4x4` mentions `load_lane_u64 blocks offset
  (4*i+k) inK lane`.
- Assertion mentions `load_lane_u64 blocks offset (4*i+k)
  old_state[4*i+k] lane`.
- inK is `*get_ij(state, iK, jK)` (state at call time = pre-iteration
  state). For these to syntactically unify, Z3 needs
  `state_at_call[4*i+k] == old_state[4*i+k]` (true by loop_invariant
  per-lane equality, BUT not by Vec256 equality — only `get_lane_u64`
  values agree).
- Without unfolding load_lane_u64's body to its
  `get_lane_u64(statei, lane) ^ ...` shape, Z3 can't bridge the
  per-lane equality from the loop_invariant to the
  load_lane_u64-level equality.

Fix would require an SMTPat lemma:
```fstar
val load_lane_u64_lane_extensionality
  (blocks: ...) (offset i: usize) (s1 s2: Vec256) (lane: usize)
  : Lemma (requires get_lane_u64 s1 lane == get_lane_u64 s2 lane)
          (ensures load_lane_u64 blocks offset i s1 lane ==
                   load_lane_u64 blocks offset i s2 lane)
          [SMTPat (load_lane_u64 blocks offset i s1 lane);
           SMTPat (get_lane_u64 s2 lane)]
```

This is a NEW spec-side lemma (not a load_block-proof modification),
but it requires careful pattern discipline to avoid re-introducing
a cascade. **Did not attempt** within the 60-min debug budget; flag
as next-attempt path.

### (a) SMTPat replacement

Mirror of `get_lane_u64` fix: replace `load_lane_u64`'s definition
with `unimplemented!()` plus an SMTPat lemma giving the equation.

Trust: the body is pure spec (XOR + get_lane_u64), so the lemma's
admit() would change the trust footprint (currently the body is
verified against itself). Acceptable trust shift IF the lemma's
SMTPat is tight enough not to cascade. Did not attempt.

## Other observations

### qi.profile dominant quantifiers post-filter

Top 5 by total-instantiations (one failing sub-query, q-692 with
filter on):
```
1,105,123  k!61                                          <-- prelude
  286,277  k!77
  201,049  refinement_interpretation_Tm_refine_8d5d6f...   <-- u64__rem_euclid
  195,008  typing_..._get_lane_u64                          (typing)
  195,008  lemma_..._get_lane_u64_post                      (inherited fix)
  164,792  typing_FStar.Seq.Base.index
  164,119  refinement_interpretation_Tm_refine_a6d4ec...    <-- range_t
  150,182  k!346
  143,265  refinement_interpretation_Tm_refine_21e027...    <-- t_Array
```

`k!61` and `k!77`/`k!346` are anonymous (no `:qid`) F*-prelude
HasType axioms. The named refinements are downstream producers
that funnel terms into k!61's pattern.

### Why lines 1091 / 1164 / 1249 / 1351 specifically

These four are the four per-iteration `hax_lib::assert!` blocks
(avx2.rs:275-290) for the four state indices `4*i`, `4*i+1`,
`4*i+2`, `4*i+3` written by `load_u64x4x4` in this iteration. Each
asserts the 4 lane equalities for that single state index. The
loop_invariant must be re-established for indices `4*(i+1)` after
all four asserts pass — they're the structural bridge between the
opaque `load_u64x4x4` post and the next-iteration loop_invariant.

The bulk of the file's other sub-queries (queries 0-690 and
700-900+) verify quickly with the filter; only these four
structural asserts remain on the cliff.

## Next-attempt path

1. **First option**: add a local `load_lane_u64_lane_extensionality`
   SMTPat lemma in `simd/avx2.rs` (option (b) augmented). Make
   load_lane_u64 opaque, give Z3 the projection-only dependence. The
   lemma is a one-line proof
   (`reveal_opaque (`%load_lane_u64) load_lane_u64`).
2. If the SMTPat triggers a new cascade, narrow it to a
   predicate-style trigger (mirror of the layered SMTPat patterns
   from `feedback_dual_smtpat_opaque_atom`).
3. If still failing, add a per-iteration `hax_lib::fstar!` lemma
   call in load_block's body (technically modifies the proof, but
   stays at the impl-side / load_block source — not in the F*
   extracted file). This is the proof-side analogue of
   factor-into-helper.
4. Last resort: split the 4 per-iteration assertions into 4 separate
   helper lemmas, each with `--split_queries always --z3rlimit 800`
   in its options block.

## Files committed on `avx2-cascade`

- `crates/algorithms/sha3/src/simd/avx2.rs` — filter added to
  load_block options.
- This status doc.
- Progress log
  `crates/algorithms/sha3/proofs/agent-status/avx2-cascade3-progress.md`
  (kept for the parent's stall-detection cadence).
