# Track B-2 follow-up — 2026-05-24 (PM)

Worktree: `/Users/karthik/libcrux-sha3-track-b-2`
Branch: `sha3-proofs-track-b-2`
Base HEAD: `c9f2e4f09` (the rollup-AM commit)
Mission: close the remaining 5 byte_eq admits from the AM rollup
(2 relocated standalone load_block byte_eq + 3 in the AVX2 squeeze
regression).

## Outcome

- **Admits closed: 5 of 5.** All structurally-extracted standalone
  byte_eq lemmas and the AVX2 squeeze byte_eq now verify with no
  `admit ()`.
- **`lemma_load_block_byte_eq_arm64`**: closes in 11.8 s, rlimit
  111/400.
- **`lemma_load_block_byte_eq_avx2`**: closes in 20.2 s, rlimit
  176/400.
- **`lemma_sq_lane_avx2_eq_squeeze_state` (byte_eq inside)**: closes
  in 85 s, rlimit 588/800.
- Full `EquivImplSpec.Sponge.{Arm64,Avx2}.API.fst` build verifies
  end-to-end (no new admits, no regressions). Pre-existing
  `Libcrux_sha3.Generic_keccak.Simd128.impl__squeeze_first_*_blocks`
  cancels are unrelated and retry-pass on split.

## Diagnosis (was: "incomplete quantifiers" at rlimit 200/400)

The AM rollup's prior attempts (minimal one-liner, slices_same_len
trigger, precondition asserts, split_queries) all failed at low
rlimit with `incomplete quantifiers`. The structural extraction was
the right architecture, but each attempted body lacked the per-lane
`get_lane_u64` seeds the SIMD context needs to bridge:

```
extract_lane lc state l . [ii]
  --(lemma_extract_lane_index SMTPat)-->  lc.lane state.[ii] l
  --(typeclass resolution)-->             arm64_lane state.[ii] l
  --(lemma_arm64_lane_eq_get_lane_u64)--> get_lane_u64 state.[ii] (mk_usize l)
```

The bridge SMTPat fires on `arm64_lane v l` (post-resolution), but
without seed terms of the form `get_lane_u64 state.[ii] (mk_usize j)`
for *each* lane `j` in scope, Z3 can't unify the per-lane equation
from load_block's ensures forall with the consumed-by extract_lane
chain. The fix: explicitly mention every `get_lane_u64` at the
relevant state index — 2 lanes for Arm64, 4 for AVX2.

The Arm64 squeeze byte_eq already shipped this pattern (in
`lemma_sq_lane_arm64_eq_squeeze_state` lines 508-527) and passes at
800/161 rlimit — that was the *working template* the AM rollup
missed when iterating on load_block byte_eq.

## What changed

### `lemma_load_block_byte_eq_arm64` body (was `admit ()`)
- `let ii = mk_usize i in`
- `let lb_state = load_block rate state blocks offset in`
- `assert (Seq.length blocks.[mk_usize l] == Seq.length blocks.[mk_usize 0])` —
  slices_same_len bridge for the `l != 0` case
- `KA.lemma_arm64_lane_unfold state.[ii] l;`
- `KA.lemma_arm64_lane_unfold lb_state.[ii] l;`
- `assert ((extract_lane ... state l).[ii] == arm64_lane state.[ii] l)` — fires extract_lane SMTPat
- Same `assert` for `lb_state`
- `let _ = get_lane_u64 state.[ii] (mk_usize 0) in ...` × 4 (state lane 0/1, lb_state lane 0/1)
- `reveal_opaque load_lane_u64` — unfold per-lane spec
- `if v ii < v (rate /! 8) then lemma_subslice_bytes_eq blocks.[mk_usize l] offset rate ii`

### `lemma_load_block_byte_eq_avx2` body (was `admit ()`)
Same shape, 4 lanes (`mk_usize 0..3` on both `state` and `lb_state`),
AVX2-flavoured intrinsic module.

### `lemma_sq_lane_avx2_eq_squeeze_state` byte_eq (was: just
`lemma_avx2_lane_unfold state.[j] l`)
Strengthened to the same 4-lane seed pattern (lemma_avx2_lane_unfold
+ extract_lane assert + 4 get_lane_u64 mentions) that the
load_block byte_eq closure used. Inline strengthening sufficed —
no structural extraction needed.

## Empirical findings

1. **The 4-lane variant is *not* structurally harder than the 2-lane
   one** when given the right seeds. The AM rollup's claim ("4-lane
   cascade is structurally larger than 2-lane") was a budget
   symptom, not a structural cliff. 4 `get_lane_u64` mentions
   instead of 2 doubles the seed-set size and Z3 closes in 85 s at
   588/800 rlimit — well below the 800 cap.

2. **`smtprofiling` was *not* needed** to close these admits. The
   diagnosis fell out of comparing the failing load_block byte_eq
   to the working squeeze byte_eq in the *same module*. Lesson for
   future: before reaching for qi.profile, check whether a sibling
   lemma in the same file has solved the same shape.

3. **Inline strengthening beats structural extraction** when an
   in-scope working template exists. The AM rollup's structural
   extraction architecture was correct (and we kept it — the
   standalone lemmas are still cleaner than monolithic `byte_eq`s),
   but Step 3's body proof was 60 lines of mechanical seeding, not
   an SMTPat-redesign.

## Files modified

- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.fst`:
  - `lemma_load_block_byte_eq_arm64` body: `admit ()` → full proof (14 lines)
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.fst`:
  - `lemma_load_block_byte_eq_avx2` body: `admit ()` → full proof (22 lines, 4-lane)
  - `lemma_sq_lane_avx2_eq_squeeze_state` byte_eq inner: minimal reveal → full 4-lane seed pattern

## Build artifacts

- `/tmp/tb2-arm64-full.log` — Arm64 full module verify, 33.8 s
- `/tmp/tb2-avx2-squeeze-attempt1.log` — AVX2 full module verify, 105 s wall
- `/tmp/tb2-final-sponge.log` — full Sponge.{Arm64,Avx2}.API build, both verified
