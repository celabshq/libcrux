# Brief: per-lane Steps lemma for `lemma_squeeze2_arm64`

**Goal**: write **one** new F* lemma in `EquivImplSpec.Sponge.Arm64.Steps.fst` that
captures "one iteration of the squeeze loop maintains the per-lane invariant
at lane `l`". Once it lands, two load-bearing admits get closed:

1. `lemma_squeeze2_arm64` (currently `assume val` at
   `EquivImplSpec.Sponge.Arm64.API.fst:88`) — closed in one follow-up session
   by Claude (mechanical: replace `assume val` with one inline-ensures pass on
   `Simd128.squeeze2` that calls the new lemma 2× per loop iteration).
2. `lemma_squeeze4_avx2` (currently `assume val` at
   `EquivImplSpec.Sponge.Avx2.API.fst:87`) — closed in a second follow-up
   session by porting the lemma to N=4 (rename, lift `l < 2` to `l < 4`, mirror
   in `EquivImplSpec.Sponge.Avx2.Steps.fst`) and the inline-ensures uses 4×
   calls per iteration.

So this **one lemma unblocks both remaining squeeze admits**, which together
are the last load-bearing items in the equivalence chain (modulo the
`load_block`/`store_block` body admits — see `BRIEF_load_store_block.md`).

## Why this is the right unit of work

Two prior attempts (`lemma_squeeze2_arm64` on 2026-04-25 morning, and
`lemma_squeeze4_avx2` on 2026-04-25 later) tried the *monolithic* approach:
write the inline-ensures + loop invariant directly on `squeeze2`/`squeeze4`,
let Z3 grind the per-iteration step. Both attempts were reverted. The N=2 case
hit a 400k-instantiation BoxBool/BoxInt cascade on **one** query inside the
loop body. The N=4 case was even worse (8 forall conjuncts in the invariant
vs 4 at N=2; 12 live arrays in the VC vs 6).

The fundamental issue is not the math — the math is straightforward — it's
that Z3 sees all N lanes' state at once and explores combinatorially. A
dedicated per-lane Steps lemma sidesteps this by **operating on one lane at a
time**, so each verification context only contains that lane's facts.

The "happy path" reference is `Libcrux_sha3.Generic_keccak.Portable.squeeze`
(in `crates/algorithms/sha3/src/generic_keccak/portable.rs:243-439`), which
verifies clean at N=1.

## Exact lemma signature

Add to `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst`:

```fstar
(* ================================================================
   Step 5: one iteration of the squeeze loop preserves the per-lane
   invariant.

   Given:
     - state matching squeeze_blocks at iteration i
     - the impl's outX agreeing with spec_out_l at indices [0, i*rate)
     - the impl's outX preserving outX_initial at indices [i*rate, outlen)
   After one (keccakf1600; squeeze2(start = i*rate, len = rate)):
     - state matches squeeze_blocks at iteration i+1
     - outX agrees with spec_out_l at indices [0, (i+1)*rate)
     - outX preserves outX_initial at indices [(i+1)*rate, outlen)
   ================================================================ *)

let lemma_squeeze_one_step_arm64
      (rate: usize)
      (s_init_st: t_Array u64 (mk_usize 25)) (* the state BEFORE the first squeeze *)
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
        (* the impl state ENTERING this iteration (post-block-i, pre-keccakf-i+1) *)
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 2))
        (* impl outputs ENTERING this iteration *)
      (outputs_initial: t_Array (t_Slice u8) (mk_usize 2))
        (* the original buffers at function entry *)
      (i: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v i >= 1 /\
        v (i +! mk_usize 1) * v rate <=
            Seq.length #u8 (outputs_pre.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs_pre /\
        Seq.length #u8 (outputs_initial.[ mk_usize l ]) ==
            Seq.length #u8 (outputs_pre.[ mk_usize l ]) /\
        (* lane-l state agrees with squeeze_blocks(s_init_st, ..., i) *)
        (let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
         let zeros = Rust_primitives.Hax.repeat (mk_u8 0) outlen in
         let lane_st_init =
            G.extract_lane (mk_usize 2) KA.lc_arm64 s_init_st l in
         let outX_block0 =
            Hacspec_sha3.Sponge.squeeze_state outlen lane_st_init
              zeros (mk_usize 0) rate in
         let (spec_st_pre, spec_out_pre) =
            Hacspec_sha3.Sponge.squeeze_blocks outlen lane_st_init
              rate (mk_usize 1) i outX_block0 in
         G.extract_lane (mk_usize 2) KA.lc_arm64
           ks_pre.Libcrux_sha3.Generic_keccak.f_st l == spec_st_pre /\
         (* lane-l write range matches spec_out *)
         (forall (k: nat). k < v i * v rate /\ k < v outlen ==>
            Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
            Seq.index (spec_out_pre <: Seq.seq u8) k) /\
         (* lane-l tail preserves initial *)
         (forall (k: nat). v i * v rate <= k /\ k < v outlen ==>
            Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
            Seq.index (outputs_initial.[ mk_usize l ] <: Seq.seq u8) k)))
      (ensures (
        let ks_post =
            Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
              (mk_usize 2) #I.t_e_uint64x2_t ks_pre in
        let (out0', out1') =
            Libcrux_sha3.Generic_keccak.Simd128.squeeze2 rate ks_post
              (outputs_pre.[ mk_usize 0 ]) (outputs_pre.[ mk_usize 1 ]) in
            (* Note: in practice squeeze2 takes start, len too; check the
               actual extracted signature.  Likely s.squeeze2 trait method
               with (start = i*rate, len = rate). *)
        let outX' = if l = 0 then out0' else out1' in
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        let zeros = Rust_primitives.Hax.repeat (mk_u8 0) outlen in
        let lane_st_init =
            G.extract_lane (mk_usize 2) KA.lc_arm64 s_init_st l in
        let outX_block0 =
            Hacspec_sha3.Sponge.squeeze_state outlen lane_st_init
              zeros (mk_usize 0) rate in
        let (spec_st_post, spec_out_post) =
            Hacspec_sha3.Sponge.squeeze_blocks outlen lane_st_init
              rate (mk_usize 1) (i +! mk_usize 1) outX_block0 in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          ks_post.Libcrux_sha3.Generic_keccak.f_st l == spec_st_post /\
        (forall (k: nat). k < v (i +! mk_usize 1) * v rate /\ k < v outlen ==>
           Seq.index (outX' <: Seq.seq u8) k ==
           Seq.index (spec_out_post <: Seq.seq u8) k) /\
        (forall (k: nat). v (i +! mk_usize 1) * v rate <= k /\ k < v outlen ==>
           Seq.index (outX' <: Seq.seq u8) k ==
           Seq.index (outputs_initial.[ mk_usize l ] <: Seq.seq u8) k)))
```

(Iterate on the exact signature; the shape above is the target. Some
type-correctness fix-ups will be needed during write-up — e.g. the
`squeeze2` trait method's exact form, whether to thread `s_init_st` as
explicit ghost or recover from a different invariant shape.)

## Proof sketch (~50-80 lines)

The proof composes three existing pieces:

1. **`lemma_squeeze_block_arm64`** (already in the same file at
   line 137) — handles "after one keccakf1600, the lane-l squeeze writes
   bytes matching `squeeze_state state'_l outputs[l] start rate`."
2. **`Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_tail`** (already in
   `Hacspec_sha3.Sponge.Lemmas.fst:83`) — extends the spec
   `squeeze_blocks` recursion by one step on the right.
3. **A per-byte `aux_write_step` + `aux_tail_step`** for the new write
   range and tail (mirrors Portable's `aux_write_step`/`aux_tail_step` at
   `portable.rs:368-405`).

Concretely:

```fstar
= let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
  let zeros = Rust_primitives.Hax.repeat (mk_u8 0) outlen in
  let lane_st_init = G.extract_lane (mk_usize 2) KA.lc_arm64 s_init_st l in
  let outX_block0 =
      Hacspec_sha3.Sponge.squeeze_state outlen lane_st_init
        zeros (mk_usize 0) rate in
  (* Step 1: extract impl-side per-lane equality from the existing block lemma *)
  lemma_squeeze_block_arm64 rate ks_pre outputs_pre (i *! rate) l;
  (* Step 2: extend spec recursion by one step *)
  Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_tail
    outlen lane_st_init rate (mk_usize 1) i (i +! mk_usize 1) outX_block0;
  (* Step 3: per-byte reconciliation for the new write range and tail.
     Mirror Portable.squeeze's aux_write_step / aux_tail_step at
     portable.rs:368-405 -- two small helpers, a few asserts each, then
     FStar.Classical.forall_intro. *)
  let aux_write_step (k: nat{k < v outlen}) : Lemma (...) = ... in
  let aux_tail_step  (k: nat{k < v outlen}) : Lemma (...) = ... in
  FStar.Classical.forall_intro aux_write_step;
  FStar.Classical.forall_intro aux_tail_step
```

## Reference proofs to mirror

| What you're proving | Working reference |
|---|---|
| Per-lane bridge from impl `squeeze` to spec `squeeze_state` | `lemma_squeeze_block_arm64` (this file, line 137) |
| Spec `squeeze_blocks` extension by one step | `Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_tail` (line 83) |
| Per-byte reconciliation across iteration | `Libcrux_sha3.Generic_keccak.Portable.squeeze` per-iteration block — `crates/algorithms/sha3/src/generic_keccak/portable.rs:360-405` (the `aux_write_step` / `aux_tail_step` pattern) |
| End-to-end working analogue at N=1 | `Libcrux_sha3.Generic_keccak.Portable.squeeze` whole function (`portable.rs:243-439`) |

## Concrete tip: extracting one lane's output from `squeeze2`

The trait method `Squeeze2.squeeze2` writes BOTH `out0` and `out1` from
one call (its post-state is a pair). The per-lane Steps lemma needs the
`l`-th component. The shape:

```fstar
let (out0', out1') =
    Libcrux_sha3.Generic_keccak.Simd128.squeeze2 rate ks_post
      (outputs_pre.[ mk_usize 0 ]) (outputs_pre.[ mk_usize 1 ]) in
let outX' = if l = 0 then out0' else out1' in
...
```

This `if l = 0` is what triggered the BoxBool cascade in the monolithic
attempt. **Inside the per-lane Steps lemma it's harmless** because Z3 only
ever sees one specific `l` at a time (the lemma's argument), so the
disjunction collapses.

## Acceptance criteria

- [x] Lemma added to `EquivImplSpec.Sponge.Arm64.Steps.fst` (2026-04-30).
- [x] Lemma verifies clean at `--z3rlimit 400 --split_queries always` — 84s total, 239 sub-queries, all <500 ms each.
- [x] No new admits introduced.
- [ ] Full equivalence `make verify` is GREEN — gated on the pre-existing
      `Libcrux_sha3.Generic_keccak.Simd128.absorb2` cold-cache flake, unrelated
      to this lemma.
- [x] Lemma is exported (no `private` keyword).

### Precondition strengthening note (2026-04-30)

The brief's signature stated `s_init_st: t_Array u64 (mk_usize 25)` but the
correct type is `t_Array I.t_e_uint64x2_t (mk_usize 25)` (the SIMD state at
function entry, not a scalar lane).  Two extra preconditions were also needed
for Z3 to discharge the `squeeze_blocks` precondition without timing out:

  - `v (i +! mk_usize 1) <= v outlen / v rate`
  - `v i <= v outlen / v rate`

Both follow from `v i * v rate + v rate <= v outlen` plus `valid_rate rate`,
but stating them explicitly cuts the verification from ~10 min (with timeouts)
to 84 s clean.

## What Claude does after this lands

1. **One session**: convert `lemma_squeeze2_arm64` from `assume val` to
   real `let` by adding inline ensures + loop invariant to
   `Simd128.squeeze2` (mirroring the absorb2 inline-ensures pattern but
   using `lemma_squeeze_one_step_arm64` × 2 lanes per iteration).
2. **One session**: port the new Steps lemma to N=4
   (`lemma_squeeze_one_step_avx2 (l: nat{l<4})` in
   `EquivImplSpec.Sponge.Avx2.Steps.fst` — same proof body, just lift
   `2 → 4` and `arm64 → avx2`), then close `lemma_squeeze4_avx2` with the
   matching inline ensures on `Simd256.squeeze4` (4 calls per iteration).

Net admit reduction once both follow-ups land: **2 load-bearing admits → 0**.

## Notes / pitfalls

- The `s_init_st` parameter is needed because the loop invariant in the
  original (working) `Portable.squeeze` reasons in terms of
  `squeeze_blocks` *anchored on the function entry state* (the initial
  state before the first squeeze block). The Rust side captures this via
  `let s_init_st = s.st;` at function entry. The Steps lemma needs to
  know what `s_init_st` is to make the `squeeze_blocks` term match.
- `--fuel 1 --ifuel 1` is fine; `--fuel 0` may cause `squeeze_blocks`'s
  recursion to not unfold (since the spec function is `let rec`).
- If you need an extra arithmetic helper, the existing
  `Libcrux_sha3.Proof_utils.Lemmas.lemma_div_mul_mod` and
  `lemma_mul_succ_le` are in scope.
- Try with a `#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"` first;
  raise `--z3rlimit` only if needed. The hypothesis is that **one lane's
  facts** is small enough that Z3 should not cascade.

## Status / log

- Created: 2026-04-25 by Claude session (sha3-proofs-focused branch)
- Branch / HEAD at brief creation: `9d780a03e`
- Ready to hand off to user.
