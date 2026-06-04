# AVX2 `store_block` body-admit closure plan — 2026-05-24

Worktree: `/Users/karthik/libcrux-sha3-proofs`, branch `sha3-proofs-focused`, base `4fc269a51`.
Target: `crates/algorithms/sha3/src/simd/avx2/store.rs:74` — `hax_lib::fstar!("admit()")`.
Reference: Arm64 mirror at `crates/algorithms/sha3/src/simd/arm64/store.rs` (lines 1–691, fully verified).

## 1. Anatomy of the AVX2 `store_block` body — 3 phases

### Phase A — outer loop `for i in 0..chunks` where `chunks = len/32`
`store.rs:75-101`. Per iteration: 4× `mm256_permute2x128_si256<{0x20|0x31}>` + 4× `mm256_unpacklo/hi_epi64` deinterleave the 4 lanes of 4 state words (`s[i0..i3, j0..j3]` indexed by linearisation `4*i + {0,1,2,3}`) into 4 output streams `v0..v3`, then 4× `mm256_storeu_si256_u8` write 32 bytes per buffer.

Required per-byte facts on `out{0..3}_future[j]` for `j ∈ [start+32i, start+32(i+1))`:
- `out0_future[j] == to_le_bytes(get_lane_u64 s[(j-start)/8] 0)[(j-start) % 8]`, etc.

Composing lemmas (all SMTPat, already proven at `avx2_extract.rs:603-891`):
- `lemma_mm256_permute2x128_si256_u64x4` (lines 858-887) — pins `get_lane_u64x4 (permute2x128 IMM a b) k`.
- `lemma_mm256_unpacklo_epi64_u64x4` (lines 834-852) — `get_lane_u64x4 (unpacklo a b) k`.
- `lemma_mm256_unpackhi_epi64_u64x4` (lines 605-622).

Walking the four permute+unpack outputs through the pinned identities yields:
- `get_lane_u64x4 v_m k == get_lane_u64x4 s[i_{...}, j_{...}] 0` for the lane `m ∈ {0..3}`, `k ∈ {0..3}` mapping back to the 4 consecutive state words.

### Phase B — inner loop `for k in 0..chunks8` where `chunks8 = rem/8`
`store.rs:108-116`. Per iteration: 1× `mm256_storeu_si256_u8` into local `[0u8; 32]` `u8s`, then 4× `copy_from_slice` of 8-byte windows of `u8s` into 4 output buffers.

Required per-byte facts for `j ∈ [start+8k, start+8(k+1))` for each `out_m` (where `start` is re-bound to `start+32*chunks`):
- `out0_future[j] == to_le_bytes(get_lane_u64 s[lin/5][lin%5] 0)[(j-start)%8]` with `lin = 4*chunks + k`. The `out1`/`out2`/`out3` cases pull from lanes 1/2/3 of the same state word.

Composing facts:
- `mm256_storeu_si256_u8`'s ensures (avx2_extract.rs:91-99): `from_le_bytes(u8s[m*8..m*8+8]) == get_lane_u64 vec m` for `m < 4`. This is **u64-level**, not per-byte — see §4.
- `copy_from_slice` (in `Core_models.Slice.impl__copy_from_slice`) supplies per-byte equality between the destination range and the source slice.

### Phase C — `if rem8 > 0` partial
`store.rs:117-126`. Same structure as Phase B but with `rem8 < 8` bytes per buffer, all drawn from a single state word (lane discriminator already covered by Phase B's `m`).

Required per-byte facts for `j ∈ [start+rem-rem8, start+rem)`. Same lane mapping as Phase B with `k = chunks8`.

## 2. Proposed refactor — mirror Arm64's split

Add two helpers to `simd/avx2/store.rs`, then a composer:

```
store_block_full_avx2  (s, out0..out3, start, q)   — Phase A only, q = len/32
store_block_tail_avx2  (s, out0..out3, start, q, rem)  — Phases B+C, rem = len%32, rem < 32
store_block            (composer; computes q, rem, calls the two)
```

Justification (load-bearing structural fix from Arm64 commit `83d1a04c2`): keeping the full and tail in separate functions means F* doesn't have to bridge `len = 32q + rem` Euclidean facts inside the loop's VC. Each helper has its own ensures over a disjoint absolute byte range; the composer only does the additive composition.

**Divergence from Arm64**: AVX2 has a *two-level* tail (inner `for k in 0..chunks8` then `if rem8 > 0`) where Arm64 has a single `if-else` on `remaining`. We may need a third level of helper structure (see §3).

## 3. Required primitive helpers

Three layers, listed bottom-up.

### Layer 1 — byte/lane bridge (new module)
Create `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` (manual `.fsti`-style, like the Arm64 helper at `proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst`). Two lemmas:

```fstar
val storeu_byte_of_lane
    (u8s: Seq.seq u8) (vec: t_Vec256) (k: nat{k < 32})
  : Lemma
    (requires
       Seq.length u8s == 32 /\
       (forall (m:nat{m<4}). u64_from_le_bytes (Seq.slice u8s (m*8) (m*8+8))
                              == get_lane_u64 vec (mk_usize m)))
    (ensures
       Seq.index u8s k ==
       Seq.index (impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize (k/8)))) (k%8))
```

Proof: `from_le_bytes` ∘ `to_le_bytes = id` (round-trip), then `to_le_bytes` is index-stable on the relevant slice. The repo lacks a packaged `from_le_bytes_to_le_bytes` lemma (grep across all `.fst`/`.fsti` shows none); the proof will hand-roll one via bit-extensionality on the 64-bit word, paralleling `lemma_int_t_eq_via_bits` in `Rust_primitives.Integers`. Budget: half a session.

```fstar
val store_block_window_byte_of_storeu
    (out out_new u8s: Seq.seq u8) (vec: t_Vec256) (a: nat) (j: nat)
  : Lemma
    (requires
       a + 32 <= Seq.length out /\ Seq.length u8s == 32 /\
       Seq.length out_new == Seq.length out /\ j < Seq.length out_new /\
       Seq.slice out_new 0 a == Seq.slice out 0 a /\
       Seq.slice out_new a (a+32) == u8s /\
       Seq.slice out_new (a+32) (Seq.length out_new) == Seq.slice out (a+32) (Seq.length out) /\
       (forall (m:nat{m<4}). u64_from_le_bytes (Seq.slice u8s (m*8) (m*8+8))
                              == get_lane_u64 vec (mk_usize m)))
    (ensures
       (if j < a then Seq.index out_new j == Seq.index out j
        else if j < a + 32 then
          Seq.index out_new j ==
          Seq.index (impl_u64__to_le_bytes (get_lane_u64 vec (mk_usize ((j-a)/8)))) ((j-a)%8)
        else Seq.index out_new j == Seq.index out j))
```

Direct analog of `store_block_window_byte_of_vst` (Arm64 helper at `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst:72-104`), generalised from 16-byte window / lane∈{0,1} to 32-byte window / lane∈{0..3}. Body uses `storeu_byte_of_lane` then Arm64-style `Seq.slice/Seq.index` rewriting. **No SMTPat** — explicit lemma calls per iteration.

### Layer 2 — per-iteration wrapper `store_u64x4x4`
Mirrors Arm64's `store_u64x2x2` (`simd/arm64/store.rs:58-131`), but takes the four "input" state vectors `s_4i`/`s_4i+1`/`s_4i+2`/`s_4i+3` (= `s[i_m][j_m]` lookups) instead of two, and writes 32-byte windows to four output buffers. Inside, four `bridge_outM` closures call `store_block_window_byte_of_storeu` once each, then `Classical.forall_intro` lifts to the per-`j` `forall`. Per-byte ensures keyed on `(j-start)/8 == 4*i + m` for `m ∈ {0..3}`.

### Layer 3 — tail
**Open design question**: do we factor the `for k in 0..chunks8` body into a `store_chunk8x4` per-iteration wrapper, the `if rem8 > 0` block into `store_tail_ragged_avx2`, or both?

Recommendation: **two wrappers**, mirroring Arm64's `store_tail_{high,low}`:
- `store_chunk8x4(out0..out3, u8s_scratch, vec, start, k)` — one inner-loop iteration. Writes 8 bytes per buffer from `vec`'s lanes 0/1/2/3. Per-byte ensures over `[start+8k, start+8(k+1))` for each buffer.
- `store_tail_ragged_avx2(out0..out3, vec, start, q_inner, rem8)` — the final `rem8 > 0` block. Same pattern but with `rem8 < 8`. Mirrors `store_tail_low` (the `remaining ≤ 8` Arm64 case) but with 4 lanes instead of 2.

The `store_block_tail_avx2` body then becomes: bridge `rem = 8*chunks8 + rem8`, run `for k in 0..chunks8 { store_chunk8x4(...) }` with a 4-lane invariant, then `if rem8 > 0 { store_tail_ragged_avx2(...) }`. The structure of `store_block_tail`'s body matches `store_block_full`'s body modulo the inner if-else dispatching.

## 4. Missing intrinsic lemmas

`mm256_storeu_si256_u8`'s current ensures at `crates/utils/intrinsics/src/avx2_extract.rs:91-99` is **u64-level** (`from_le_bytes(future[m*8..m*8+8]) == get_lane_u64 vec m`), whereas the Arm64 sibling `_vst1q_bytes_u64` (`arm64_extract.rs:266-274`) is **per-byte** (`future[k] == to_le_bytes(get_lane_u64x2 v (k/8))[k%8]`).

Two options:

**Option (a) — bridge at the SHA-3 layer** (recommended; matches the prompt's instruction at `agent-prompt-store-block-avx2.md:43`): keep the intrinsic as-is; the new `storeu_byte_of_lane` lemma (Layer 1) bridges u64-level to per-byte.

**Option (b) — strengthen the intrinsic** to match Arm64's per-byte form. Pros: composes more directly with the helpers; the round-trip lemma effectively lives at the intrinsic boundary rather than at SHA-3. Cons: cross-cutting change touching `avx2_extract.rs`; needs re-extraction across consumers (Kyber/Dilithium use this intrinsic too — verify nothing else asserts the *exact* current form).

Recommendation: start with (a). If `storeu_byte_of_lane` cliffs (hand-rolled `from_le_bytes ∘ to_le_bytes` round-trip), fall back to (b).

## 5. Loop invariant sketches

### Phase A — outer loop in `store_block_full_avx2`

```fstar
hax_lib::loop_invariant!(|i: usize|
  (out0.len() == old_out0.len()).to_prop() & /* same for out1..3 */ &
  hax_lib::forall(|j: usize| if j < out0.len() {
    if j < start { out0[j] == old_out0[j] }
    else if j < start + i * 32 {
      out0[j] == get_lane_u64(s[(j-start)/8], 0).to_le_bytes()[(j-start)%8]
    } else { out0[j] == old_out0[j] }
  } else { true })
  & /* same forall for out1 (lane 1), out2 (lane 2), out3 (lane 3) */
);
```

Direct lift of `store_block_full` invariant (`arm64/store.rs:471-494`) generalised from 2 to 4 lanes. The key bookkeeping: window grows by 32 per iteration (not 16), and there are four lane indices (not two).

### Phase B — inner loop in `store_block_tail_avx2`

```fstar
hax_lib::loop_invariant!(|k: usize|
  (out0.len() == old_out0.len()).to_prop() & /* ... */ &
  hax_lib::forall(|j: usize| if j < out0.len() {
    if j < start_inner { out0[j] == old_out0[j] }
    else if j < start_inner + k * 8 {
      out0[j] == get_lane_u64(s[(j-start)/8], 0).to_le_bytes()[(j-start)%8]
    } else { out0[j] == old_out0[j] }
  } else { true })
  & /* lanes 1/2/3 analog */
);
```

Where `start_inner = start + 32*chunks`. **Critical**: the `(j-start)/8` term uses the *outer* `start`, not `start_inner`, because the lane mapping for `j ∈ [start_inner+8k, start_inner+8(k+1))` is governed by `(j-start)/8 = 4*chunks + k_to_lane(j)` — the global byte index continues to map state words consecutively.

The Phase A → Phase B transition (entering the `if rem > 0`) needs a one-line bridge asserting `start_inner + 0 == start + 32*chunks` so the `else if j < start_inner + 0*8` branch trivially reduces to "no new bytes yet" matching Phase A's exit.

## 6. Effort estimate

Per the `agent-prompt-store-block-avx2.md` prompt and the §7 SKILL.md "per-lane seeds" lesson (which closed `lemma_load_block_byte_eq_avx2` in 20 s / rlimit 176/400):

- Layer 1 (helper lemmas): 1 session if the round-trip `from_le_bytes ∘ to_le_bytes` lemma proves at low rlimit. Add half a session if option (b) (intrinsic strengthening) is needed.
- Layer 2 (`store_u64x4x4`): 1 session (direct port of `store_u64x2x2`, lines 58–131 of Arm64, with 4 lanes instead of 2).
- Layer 3 (tail wrappers): 1–2 sessions. The two-level tail makes this the highest-variance line item.
- `store_block_full_avx2` / `store_block_tail_avx2` / composer: 1 session combined if the wrappers' posts compose cleanly (they should, per the Arm64 precedent — `store_block` composer at `arm64/store.rs:641-671` is ~30 lines and trivial).

Total: **3–5 focused sessions** for the structural work, plus one fudge factor session for the inevitable SMT-options tuning (`--using_facts_from`, `--split_queries`). The Arm64 mirror took 4 commits (`8c0202a4b`, `c14f94d2c`, `29424f593`, `83d1a04c2`); AVX2 should land in a comparable number with the recipe already known.

Closing this admit unblocks `lemma_squeeze4_avx2` (item #3 in `sha3-admits-summary-2026-05-24.md`) — the per-lane Steps lemma at N=4 consumes `avx2_sc_store_block`'s ensures, which bottoms out here. Item #2 (`lemma_squeeze2_arm64`) is independent.

## 7. Risks

### Known cliffs
- **Z3 4.13.3 LP-solver IPC crash at ask_count≈410** (per `lemma_squeeze2_arm64-attempt-2026-05-24.md:62-72`). Will trigger if any single query exceeds ~410 unique e-matching invocations. The Arm64 store_block composer uses `--z3refresh` to mitigate (see `arm64/store.rs:415`, `arm64/store.rs:515`). **Plan: apply `--z3refresh` to every per-iteration wrapper and to `store_block_full_avx2`/`store_block_tail_avx2`**, matching the Arm64 precedent. The risk is concentrated in `store_u64x4x4`: 4× the SMTPat fan-out from the per-lane unpack/permute lemmas could push past 410 if not split.

### Structural cliffs
- The inner permute+unpack chain (Phase A) is a 4-way deinterleave. Each `v_m` derives from 2 permutes ∘ 1 unpack, so the lemma fan-out is roughly the **product** of `lemma_mm256_unpack{lo,hi}_epi64_u64x4` (4 lane equations each) and `lemma_mm256_permute2x128_si256_u64x4` (4 lane equations × 2 IMM cases). Z3's e-matching could blow up in a single query. **Plan: use `--split_queries always` on `store_u64x4x4`** so each lane's `bridge_outM` discharges independently; that's the same pattern that worked for the Arm64 `store_u64x2x2` (rlimit 400/split, `arm64/store.rs:31`).
- The lane-mapping arithmetic `(j-start)/8 == 4*i + m` is more arithmetic-heavy than Arm64's `(j-start)/8 == 2*i`. The Arm64 store_block uses `--using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'` (lines 415, 515, 614). **Plan: same filter on every AVX2 wrapper**.
- `from_le_bytes ∘ to_le_bytes = id` is not packaged as a lemma in the tree (grep across the repo confirms). The Layer 1 round-trip will need a hand-rolled bit-extensionality proof. If that lemma alone cliffs, fall back to option (b) — strengthen `mm256_storeu_si256_u8` to per-byte form at the intrinsic boundary (where the round-trip can be discharged once for all consumers).

### Squeeze2 cascade comparison
The squeeze2 aux cliff (`lemma_squeeze2_arm64-attempt-2026-05-24.md`) is *driver-level* (composing `arm64_sc_store_block`'s post with `squeeze_state`-vs-`squeeze` bridge); it's structurally distinct from store_block's *body-level* per-byte ensures. We don't expect the same cliff shape here. The risk is rather the LP-solver crash bound itself, which is environmental.

### Critical Files for Implementation
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/src/simd/avx2/store.rs`
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/src/simd/arm64/store.rs` (reference)
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` (helper template)
- `/Users/karthik/libcrux-sha3-proofs/crates/utils/intrinsics/src/avx2_extract.rs` (lines 91-99 for the intrinsic ensures; option (b) site if needed)
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.Store.fst` (the extracted file currently carrying the admit at line 165)
