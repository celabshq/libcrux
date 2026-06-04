# loop-invariant-opacify — progress log

Branch: `loop-invariant-opacify` (worktree: `/Users/karthik/libcrux-loop-inv-opacify`)
Base: `eeeb8b136` on `sha3-proofs-focused`
Cherry-picked: `4bbe9b667` (AVX2 store_block structural split + 3 wrappers + helpers)

## Diagnosis recap (from `avx2-cliff-profile-progress.md`)

q213 of `store_block_full` cliffs because of a **quantifier cascade**:
- `k!61` (anonymous goal forall, 124 vars) at **1.19M instantiations**
- Total cascade 10.7M instantiations
- The 124-var goal forall comes from the 4-buffer × byte-level forall
  shape of the loop_invariant. Same mechanism dominates Arm64
  `load_block` q301 (1.97M instantiations).

## Plan (AlgoStar Technique 4 — opaque bundles)

For each of the 4 output buffers in `store_block_full` (lane k = 0..3),
introduce an opaque predicate:

```
[@@"opaque_to_smt"]
val byte_inv_lane_k :
  (lane: nat{lane < 4}) ->
  (out: t_Slice u8) -> (out_old: t_Slice u8) ->
  (s: t_Array Vec256 25) ->
  (start: usize) -> (i_bound: usize) ->
  Type0
```

Body: the existing per-lane `forall (j: usize). if j < len out then ...`.

Plus `_init` / `_step` / `_after_loop` lemmas. Loop invariant becomes
the conjunction of 4 opaque calls; goal forall shrinks from 124 vars
to ~30, breaking the cascade at source.

For **Arm64 `load_block`** the analog: 1 mutable state array (25 slots),
the inner forall over `i in 0..rate/16` is fed by `get_lane_u64` reads
on each lane. Will sketch in detail after AVX2 first cliff is closed.

## T+0 (kickoff)

- Cherry-picked `4bbe9b667` to bring structural split into
  loop-invariant-opacify branch. State: `store_block_full` and
  `store_block_tail` carry `--admit_smt_queries true`. Wrappers,
  helpers, `store_block` composer all verify clean.
- Read AlgoStar Technique 4 in skill. Sketch on paper:
  1. `byte_inv_lane k out_lane out_old_lane s start i_bound` (one
     definition parameterized by lane k)
  2. The body is the same per-lane forall over j currently inlined
     in 4 places (lines 1602-1701 of Store.fst).
  3. `_init: byte_inv_lane k out_lane out_old_lane s start 0`
     (trivial — `i*32 == 0` so the `j < start + i*32` branch is empty)
  4. `_step: i+1 <= q -> byte_inv_lane k out_lane' out_old_lane s start (i+1)
     given byte_inv_lane k out_lane out_old_lane s start i and the
     storeu wrapper post on the i-th iteration window.`
  5. `_after_loop: byte_inv_lane k out_lane out_old_lane s start q
     -> the original ensures-shape forall.` (Just `reveal_opaque`.)

- Status target: in 30 min, opaque pred + 3 lemmas drafted in a new
  `Libcrux_sha3.Simd.Avx2.LoopInv.fst` co-located helper module,
  with one clean lemma. Then plumb into `store_block_full`.

## ETA

- Phase 1: AVX2 byte_inv predicates + 3 lemmas (60 min)
- Phase 2: Plug into `store_block_full`, drop `--admit_smt_queries` (60 min)
- Phase 3: Same for `store_block_tail` (60 min)
- Phase 4: Arm64 `load_block` analog (90 min)
