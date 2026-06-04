# SHA-3 equivalence proof — session handoff

Date: 2026-04-26 (appended to prior 2026-04-25 / 2026-04-24 / 2026-04-23 / 2026-04-21 docs below)
Working dir: `crates/algorithms/sha3/proofs/fstar/equivalence/`
Branch: `sha3-proofs-focused` (focused PR to main)

## 2026-04-26: arm64 `load_block` body admit DISCHARGED

`Libcrux_sha3.Simd.Arm64.fst::load_block` is no longer a body admit.
Per-lane ensures + a lane-bytewise loop invariant on `Simd.Arm64.load_block`
follow the same pattern that already verifies arm64 `load_last`.  The
`hax_lib::fstar!("admit()")` line in
`crates/algorithms/sha3/src/simd/arm64.rs::load_block` is gone; the
extracted `let load_block` carries no `admit ()`.  Net arm64 body-admit
count drops from 2 to 1.  Committed in `abf8b5297` ("load-blocl").

### Real load-bearing admits remaining (5 total)

**Arm64 (2):**

| # | File | Line | Kind | Notes |
|---|------|------|------|-------|
| A1 | `Libcrux_sha3.Simd.Arm64.fst` | 909 | body `admit ()` in `store_block` | bytewise reasoning + loop invariant; mirror of the now-proved `load_block`/`load_last` patterns |
| A2 | `EquivImplSpec.Sponge.Arm64.API.fst` | 88 | `assume val lemma_squeeze2_arm64` | unblocked by per-lane Steps lemma (Option B from the squeeze2 post-mortem) |

**AVX2 (3):**

| # | File | Line | Kind | Notes |
|---|------|------|------|-------|
| V1 | `Libcrux_sha3.Simd.Avx2.fst` | 1433 | body `admit ()` in `store_block` | mirror of A1 at N=4 |
| V2 | `EquivImplSpec.Sponge.Avx2.API.fst` | 87 | `assume val lemma_squeeze4_avx2` | mirror of A2 at N=4; same Option B lemma closes both |
| V3 | `Libcrux_intrinsics.Avx2_extract.fsti` | 21 | `val lemma_get_lane_u64x4_bit` | bit-bridge axiom; the only AVX2-specific intrinsic-level trust point — all ten `lemma_mm256_*_u64x4` SMTPats (six original + four new this session for `unpackhi/unpacklo_epi64`, `set_epi64x`, `permute2x128_si256`) now derive from it |

NEON intrinsic-level `val` declarations in `Libcrux_intrinsics.Arm64_extract.fsti`
are uninterpreted axioms too, but represent the same trust boundary as
intrinsics on AVX2 (not double-counted above).

### Discharged this session

- **Arm64 `load_block`** — body admit removed.  Verified inline.

### Discharged earlier (since the 2026-04-25 evening snapshot)

- 6 × `lemma_mm256_*_u64x4` SMTPats in
  `Libcrux_intrinsics.Avx2_extract.fsti` — converted from `admit ()`
  to real proofs via the bit-bridge `lemma_get_lane_u64x4_bit` +
  `Rust_primitives.Integers.lemma_int_t_eq_via_bits` (commit
  `ef12db0ce`).  Trust shrinks from 6 axioms to 1.
- `Libcrux_sha3.Proof_utils.Lemmas.lemma_shl_xor_shr_is_rotate_left`
  — proved.  (The comment at `EquivImplSpec.Keccakf.Avx2.fst:96-99`
  still calls it "admitted"; that is stale documentation, not a real
  admit.)
- 4 × `Proof_Utils.Lemmas.fst` upstream admits closed via the
  `cryspen/hax integer-lemmas` branch (commit `a078bd14c`).

### AVX2 `load_block` — committed but NOT verifying

`crates/algorithms/sha3/src/simd/avx2.rs` adds `load_lane_u64`,
`load_u64x4x4`, `load_u64x4` helpers + an inline loop invariant on
AVX2 `load_block` (mirror of the arm64 work).  Helpers verify cleanly
(both annotated `[@@ "opaque_to_smt"]` so callers consume their
ensures without unfolding the body).  **`load_block` itself does NOT
verify within F*'s practical limits** in this session.

#### Configurations attempted on `load_block` (all failed)

| Config | Outcome |
|---|---|
| `--z3rlimit 800 --split_queries always` (no opaque helpers) | Z3 4.13.3 segfaulted under heavy split-query load |
| `--z3rlimit 2000` (opaque helpers, no split) | Z3 ran 13.1 min, canceled, F* retry-split path → Z3 segfault |
| `--z3rlimit 800 --split_queries always` (opaque helpers, no z3refresh) | reached split #653, hit a 4-min sub-goal, killed by user |
| Same + `OTHERFLAGS="--z3refresh"` | Z3 RPC parse-error after ~490 splits (F* harness bug at ~800 z3 process spawns) |

#### Why N=4 is harder than N=2

arm64 `load_block` verifies inline at N=2 with `--z3rlimit 800
--split_queries always` in ~300 splits.  AVX2 N=4 doubles the
per-state-element conjuncts in both the postcondition and the loop
invariant, pushing total split count to 600+.  The verification path
also exposes a hard F*/Z3 RPC parse-error bug after ~800 z3 process
spawns (likely an fd / handle leak in F* 2025.03.25-dev).

#### Intrinsic-level work that DID land this session

`crates/utils/intrinsics/src/avx2_extract.rs` and
`fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti`: added bit-level
definitions and four new SMTPat lemmas
`lemma_mm256_{unpackhi,unpacklo}_epi64_u64x4`,
`lemma_mm256_set_epi64x_u64x4`, `lemma_mm256_permute2x128_si256_u64x4`
— all **proved** from the existing bridge axiom
`lemma_get_lane_u64x4_bit` (no new trust axioms).  Trust budget at the
AVX2 intrinsic boundary remains 1 (V3).  These benefit any future
verification that needs u64-lane semantics for these intrinsics (e.g.
ML-DSA NTT/INVNTT/encoding which use them today without u64-level
specs).  Per-call `mm256_loadu_si256_u8` ensures (+27 lines added to
`avx2_extract.rs`) are still ad-hoc and should be moved to the same
lemma-from-bridge pattern when revisited.

#### Next steps for whoever picks this up

1. **Refactor**: hoist the `load_block` per-iteration body into a
   `load_block_chunk` helper with its own `[@@ "opaque_to_smt"]` +
   `--split_queries always` + ensures
   (`state[(4i)..(4i+4)]` = loaded, rest preserved).
   `load_block`'s outer VC then becomes thin loop-glue and should
   drop the split count well under the F* harness threshold.  This
   is one level deeper than what arm64 needed.
2. **Or simplify ensures**: replace the explicit per-lane conjunction
   in both postcondition and invariant with a recursive predicate.
3. **Or hand-prove in F***: write the proof directly via an `assume
   val` interface + a `.fst` body using lemmas, bypassing hax's
   auto-VC.  Heavier but sidesteps the harness bug entirely.

## 2026-04-25 (later): squeeze4_avx2 — partial progress, banked infrastructure

Attempted to mirror the Portable.squeeze inline-ensures pattern at N=4
to discharge `lemma_squeeze4_avx2`.  **Banking the infrastructure
that verified, reverting the rest, deferring the full proof.**

### What landed

- `KeccakState<4, Vec256>::squeeze_last4` — new `pub(crate) fn`
  mirroring `KeccakState<1, u64>::squeeze_last`.  Carries per-lane
  ensures: for each `l ∈ [0,4)`, the post-state lane-`l` extraction
  and the corresponding output buffer match
  `Hacspec_sha3.Sponge.squeeze_last`.  Internal proof composes 4 ×
  `EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2`.
  Verifies clean (824 successful queries, 0 failed).
- `_keccak_state_impl4_opts` dummy fn before `impl KeccakState<4, Vec256>`
  pushes `--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always`
  for methods inside the impl block (workaround for hax#1698 — same as
  Portable's `_keccak_state_impl_opts`).
- `out0.len() < usize::MAX - 200` precondition propagated up to
  `keccak4` and `avx2::shake256` — needed by the eventual
  `Hacspec_sha3.Sponge.squeeze` post-conditions.

### What was reverted

- The full inline-ensures attempt on `Simd256.squeeze4`.  Verified the
  `blocks == 0` branch (985 succeeded, 0 failed), then hit the Z3
  cascade on the pre-loop scaffolding — one query timed out at
  rlimit 800 (~5 min) before the verify gave up.  The per-iteration
  block (the next placeholder) would have been strictly harder.

### Why the monolithic VC blows up at N=4

This mirrors the 2026-04-25 (earlier) squeeze2 post-mortem.  At N=2
the Arm64 attempt hit a 400k-instantiation BoxBool/BoxInt cascade on
ONE loop-invariant re-establishment query, with 4 forall conjuncts
in the invariant.  At N=4:

- 4 lane-state equality conjuncts in the loop invariant (vs 2 at N=2)
- 8 forall conjuncts (write/tail × 4 lanes vs write/tail × 2)
- 4 spec arrays + 4 impl arrays + 4 initial arrays = 12 arrays live in
  the VC context (vs 6 at N=2)

Z3's pattern-driven instantiation explores combinatorially across
those arrays.  Empirically: pre-loop scaffolding (which is
*easier* than the per-iteration step) already times out at rlimit 800
on at least one query in ~5 min.

### Path forward — Option B (per-lane Steps lemmas)

The fix is to factor the per-iteration proof into a dedicated Steps
lemma that operates on ONE lane at a time, so each VC sees only
one lane's worth of state:

```
val lemma_squeeze_one_step_avx2
      (rate: usize)
      (ks_pre: t_KeccakState 4 Vec256)
      (outputs: t_Array (t_Slice u8) 4)
      (i: usize)
      (l: nat{l < 4})
  : Lemma
      (requires
         valid_rate rate /\
         v i >= 1 /\
         v (i +! mk_usize 1) * v rate <= Seq.length (outputs.[mk_usize l]) /\
         (* lane-l invariant entering iteration: state, write range, tail *)
         ...)
      (ensures
         (* lane-l invariant exiting iteration *)
         ...)
```

With this lemma, the Rust per-iteration scaffolding becomes 4 calls
(one per lane), each with a small VC.  Pre-loop and post-loop
similarly factored.

### Estimated effort to close lemma_squeeze4_avx2

- Build the per-lane Steps lemma: 1-2 days
- Wire the inline scaffolding: 0.5-1 day
- Same for `lemma_squeeze2_arm64` once squeeze4 lands (strictly easier
  port at N=2)

### Squeeze step lemmas already proved (re-usable for Option B)

- `EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_block_avx2`
- `EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2`
- `Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_base/unfold/tail`
- `Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_unfold/seq_trans/last_extensional`
- `EquivImplSpec.Sponge.Avx2.avx2_sc_store_block` (no-keccakf bridge)

These are the building blocks; they're all in place and verified.

## 2026-04-25 (night, late): lemma_absorb4_avx2 PROVED

Mirrored the arm64 `lemma_absorb2_arm64` proof at N=4: `lemma_absorb4_avx2`
is now a real `let` discharged from the Rust-side ensures on
`Libcrux_sha3.Generic_keccak.Simd256.absorb4`.  Net AVX2 admit count drops
from 9 to 8.

### Changes

1. `crates/algorithms/sha3/src/generic_keccak/simd256.rs::absorb4`
   - Added per-lane `#[hax_lib::ensures]` (4 conjuncts, one per lane).
   - Added `--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always`.
   - Added inline scaffolding mirroring arm64 absorb2 at N=4:
     - Pre-loop: `lemma_extract_lane_zero_avx2` × 4 + `lemma_absorb_blocks_base` × 4.
     - Loop invariant: 4-lane `absorb_blocks` equality + `i <= data_blocks`.
     - Per-iteration: `lemma_absorb_block_avx2` × 4 + `lemma_absorb_blocks_tail` × 4,
       gated by `assert (slices_same_len 4 data)`.
     - Post-loop: `lemma_absorb_last_avx2` × 4 + `lemma_absorb_rec_via_blocks` × 4,
       same `slices_same_len` assert.
   - Body `hax_lib::fstar!("admit()")` removed.
2. `crates/algorithms/sha3/src/generic_keccak/simd256.rs::squeeze4`
   - Bumped `#[hax_lib::fstar::options]` from `--z3rlimit 300` to
     `--z3rlimit 600 --split_queries always` (pre-existing flake unmasked
     once absorb4 stopped admitting; loop-body `f_squeeze4` precondition
     was timing out at rlimit 300).
3. `proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.API.fst`
   - `assume val lemma_absorb4_avx2` → `let lemma_absorb4_avx2 ... =
     let _ = Libcrux_sha3.Generic_keccak.Simd256.absorb4 rate delim data
     in ()`.

### Z3 strategy notes

The N=4 fan-out (4× lemma calls per lane bunched in one `fstar!` block)
initially tripped Z3 on the lane-1 and lane-2 `lemma_absorb_block_avx2`
preconditions at queries 286/290 (timeout at rlimit 800).  Adding an
explicit `assert (slices_same_len 4 data)` at the head of each bunched
block resolved both — Z3 needed the bridge fact made explicit rather
than rederiving it via SMT-pattern dispatch on the original
function-precondition pair.

### Verification

- `Libcrux_sha3.Generic_keccak.Simd256.fst.checked` builds clean (no
  failed queries on `absorb4`; the F* split-retry mechanism handles
  one slow `squeeze4` query).
- `EquivImplSpec.Sponge.Avx2.API.fst.checked` builds in ~4 s.
- `make verify` under `proofs/fstar/equivalence/` is GREEN.

### Remaining AVX2 admits (8)

| File | Kind |
|---|---|
| `Libcrux_intrinsics.Avx2_extract.fst` | 6 × per-u64-lane SMTPat lemma admits (intrinsic-level trust boundary, same as arm64) |
| `EquivImplSpec.Keccakf.Avx2.fst` | 1 × `lemma_shl_xor_shr_is_rotate_left` (Core_models.Num opacity) |
| `EquivImplSpec.Sponge.Avx2.API.fst` | 1 × driver-level `assume val lemma_squeeze4_avx2` |

Plus 3 body-level `admit()`s in `Libcrux_sha3.Simd.Avx2.fst` for
`load_block` / `load_last` / `store_block` (heavy bit-twiddling that
mirrors arm64's body admits).

### Next steps

1. Close `lemma_squeeze4_avx2` — same mirror-the-arm64-pattern strategy,
   harder (analogous to `lemma_squeeze2_arm64` which is *also* still
   admitted on arm64).  See the 2026-04-25 squeeze2-pivot post-mortem
   below for the BoxBool/BoxInt cascade you'll likely hit at N=4.
2. Close `lemma_shl_xor_shr_is_rotate_left` by adding
   `Core_models.Num.lemma_impl_u64__rotate_left_via_shifts` upstream.

## 2026-04-25 (night): All 3 avx2_sc_* bridge admits closed

Mirrored the arm64 sc_* bridge proof pattern at N=4.  All three
`avx2_sc_load_block`, `avx2_sc_load_last`, `avx2_sc_store_block`
are now real `let`s (no `admit ()`s).  Net AVX2 admit count drops
from 17 to 9.

### Changes

1. `crates/utils/intrinsics/src/avx2_extract.rs`
   - Added `get_lane_u64(vec: Vec256, lane: usize) -> u64` (Rust
     stub) with ensures `result == get_lane_u64x4 vec (v lane)`,
     mirroring arm64's `get_lane_u64` over `t_e_uint64x2_t`.
   - Added `vec256_as_u64x4` opaque val + `get_lane_u64x4` helper to
     the `Vec256` interface (via fstar::replace).
   - Added per-u64-lane ensures to `mm256_loadu_si256_u8` and
     `mm256_storeu_si256_u8`.
2. `crates/utils/intrinsics/src/avx2.rs` — added matching
   `get_lane_u64` Rust impl (used by Rust callers that need lane
   extraction).
3. `crates/algorithms/sha3/src/simd/avx2.rs`
   - Added per-u64-lane `#[hax_lib::ensures(...)]` to `load_block`
     and `store_block` mirroring the arm64 form (4 lanes vs 2).
   - Unrolled `load_last`'s buffer loop into `buffer0..buffer3`
     (same compiled code as the previous `for i in 0..4 {
     buffers[i]... }`; needed so the F* bridge can reconstruct
     each buffer in scope).
4. `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.fst`
   - Added bridge lemmas:
     `lemma_load_block_eq_xor_block_into_state_avx2`,
     `lemma_load_last_eq_xor_block_into_state_avx2`,
     `lemma_sq_lane_avx2_eq_squeeze_state` — all PROVED, mirroring
     the arm64 versions at N=4.
   - `avx2_sc_load_block`, `avx2_sc_load_last`, `avx2_sc_store_block`
     now reduce to one-line calls of the bridge lemmas.
   - Reuses `EquivImplSpec.Sponge.Arm64.lemma_load_last_buffer_eq_padded_arm64`
     (the per-buffer padded equality is generic — not arm64-specific).

### Verification

`make verify` GREEN under `proofs/fstar/equivalence/` (exit 0).
`lemma_load_last_eq_xor_block_into_state_avx2` is the costly one
at `--z3rlimit 600`, ~10 minutes wall-clock.

### Remaining AVX2 admits (9)

| File | Kind |
|---|---|
| `Libcrux_intrinsics.Avx2_extract.fst` | 6 × per-u64-lane SMTPat lemma admits (intrinsic-level trust boundary, same as arm64) |
| `EquivImplSpec.Keccakf.Avx2.fst` | 1 × `lemma_shl_xor_shr_is_rotate_left` (Core_models.Num opacity) |
| `EquivImplSpec.Sponge.Avx2.API.fst` | 2 × driver-level `assume val`s: `lemma_absorb4_avx2`, `lemma_squeeze4_avx2` |

Plus 4 body-level `admit()`s in `Libcrux_sha3.Simd.Avx2.fst` and
`Libcrux_sha3.Generic_keccak.Simd256.fst` for the heavy bit-twiddling
(load_block, load_last, store_block bodies; absorb4 body) — these
mirror arm64's body-level admits.

### Next steps

1. Close `lemma_absorb4_avx2` via inline loop-invariant proof on
   `Generic_keccak.Simd256.absorb4` (mirrors `Simd128.absorb2`,
   already done on arm64).
2. Close `lemma_squeeze4_avx2` — same pattern, harder (analogous
   to `lemma_squeeze2_arm64` which is *also* still admitted on arm64).
3. Close `lemma_shl_xor_shr_is_rotate_left` by adding
   `Core_models.Num.lemma_impl_u64__rotate_left_via_shifts` upstream.

## 2026-04-25 (evening, late): AVX2 verification + equivalence chain

Two layers landed (commit `d9dd345ef`):

### Layer 1 — AVX2 modules verify without `--admit_smt_queries`

Added `#[hax_lib::requires]`/`#[hax_lib::ensures]` to:

- `crates/algorithms/sha3/src/simd/avx2.rs` — `load_block`, `load_last`,
  `store_block` (with body-level `hax_lib::fstar!("admit()")` mirroring
  arm64); trait impls `Absorb<4>`, `Squeeze4`, `KeccakItem<4>`.
- `crates/algorithms/sha3/src/generic_keccak/simd256.rs` — refactored
  to expose `absorb4` and `squeeze4` as separate functions (mirrors
  `Simd128.absorb2` / `Simd128.squeeze2`); `keccak4` now composes
  them.  Body of `absorb4` admitted; `squeeze4` body verifies clean.
- `crates/algorithms/sha3/src/avx2.rs` — `x4::shake256` and seven
  `x4::incremental::*` functions get the analogous Portable-style
  preconditions.
- `crates/algorithms/sha3/src/traits.rs` — `Squeeze4` trait now has
  `#[hax_lib::attributes]` with `requires`/`ensures` on `squeeze4`
  (mirrors `Squeeze2`).
- `crates/algorithms/sha3/hax.sh::patch_fstar_extractions` — adds
  `noeq` to `Avx2.X4.Incremental.t_KeccakState`.

`make verify` GREEN under `proofs/fstar/extraction/`.

### Layer 2 — equivalence chain replicated through `lemma_squeeze4_avx2`

Created four new files mirroring the arm64 chain:

1. `EquivImplSpec.Keccakf.Avx2.fst` — defines `avx2_lane =
   get_lane_u64x4`, `lc_avx2`, derives `lemma_keccakf1600_avx2`.
   **All 7 lane-correctness `avx2_lc_*` lemmas are PROVED** (real
   `let`s, not admits) by composing per-u64-lane SMTPats on AVX2
   intrinsics.  One small admit:
   `lemma_shl_xor_shr_is_rotate_left` (Core_models.Num
   `impl_u64__rotate_left` is opaque).

2. `EquivImplSpec.Sponge.Avx2.fst` — `sq_lane_avx2`,
   `sponge_correctness` record `sc_avx2`.  Three `avx2_sc_*` bridges
   are admitted (analogous to where arm64 SC bridges would be admitted
   if `mm256_loadu_si256_u8` / `mm256_storeu_si256_u8` had no
   bytewise lane ensures).

3. `EquivImplSpec.Sponge.Avx2.Steps.fst` — four step lemmas
   (`absorb_block`, `absorb_last`, `squeeze_block`, `squeeze_last`)
   all proved by composition.

4. `EquivImplSpec.Sponge.Avx2.API.fst` — `assume val
   lemma_absorb4_avx2`, `assume val lemma_squeeze4_avx2`,
   `lemma_keccak4_avx2` proved by composition,
   `lemma_shake256_x4_avx2` (top-level for the AVX2 X4 hasher)
   proved from `lemma_keccak4_avx2`.

### Strategy lessons

- Initial bit_vec-extensionality approach (using
  `bit_vec_to_int_t` directly) was wrong — closing it requires a
  bit-extensionality lemma on `int_t` that doesn't exist in
  `Rust_primitives.BitVectors`.  Pivoted to **mirror the arm64
  strategy exactly**: opaque `vec256_as_u64x4 : bit_vec 256 → t_Array
  u64 (sz 4)` + `get_lane_u64x4` + per-u64-lane `lemma_mm256_*_u64x4`
  SMTPats on the AVX2 intrinsics.  The per-lane lemmas are admitted
  in `Libcrux_intrinsics.Avx2_extract.fst` (appended via
  `hax.sh::patch_fstar_extractions`), exactly the same trust boundary
  arm64 uses where the `e_v*q_u64` intrinsic ensures clauses are
  trusted.
- Re-bracketed `_veor5q_u64` left-associatively in the Rust source
  (((a^b)^c)^d)^e to match the spec shape — avoids needing
  associativity/commutativity of `^.` on u64 in `avx2_lc_xor5`.
- Added bit_vec definitions to `BitVec.Intrinsics.fsti` for the 5
  newly-imported AVX2 ops (`mm256_xor_si256`, `mm256_or_si256`,
  `mm256_andnot_si256`, `mm256_slli_epi64`, `mm256_set1_epi64x`).
  These don't directly serve the lane-correctness proofs (those go
  through the per-u64-lane SMTPats) but make the operations
  semantically meaningful.
- Added `vec256_as_u64x4` + `get_lane_u64x4` to `avx2_extract.rs`'s
  `Vec256` interface via `hax::fstar::replace`.

### Next steps

1. Close the 6 `lemma_mm256_*_u64x4` admits in
   `Libcrux_intrinsics.Avx2_extract.fst`.  The bit_vec
   definitions + the `vec256_as_u64x4` opaque val give enough
   structure; just need a bit-extensionality lemma on `t_Array u64
   (sz 4)`.  Same task that arm64 has already done via its NEON
   intrinsic ensures clauses (which are themselves trusted).
2. Close `lemma_shl_xor_shr_is_rotate_left` by adding a
   spec lemma to `Core_models.Num` saying
   `impl_u64__rotate_left x n == (x <<! cast n) ^. (x >>! cast (64-n))`
   for `0 < n < 64`.
3. Close the three `avx2_sc_*` admits in `EquivImplSpec.Sponge.Avx2`
   by adding bytewise lane ensures to `mm256_loadu_si256_u8` and
   `mm256_storeu_si256_u8` (mirror the arm64 stubs' ensures).
4. Close `lemma_absorb4_avx2` via inline loop-invariant proof on
   `Generic_keccak.Simd256.absorb4` (mirrors `Simd128.absorb2`,
   already done).
5. Close `lemma_squeeze4_avx2` — same pattern, harder (analogous to
   `lemma_squeeze2_arm64` which is *also* still admitted on arm64).

## 2026-04-25 (afternoon): AVX2 extraction enabled (mirrors arm64)

Extraction of the SHA-3 AVX2 backend (`crates/algorithms/sha3/src/simd/avx2.rs`,
`avx2.rs::x4`, and `generic_keccak/simd256.rs`) is now wired into `hax.sh`,
mirroring the arm64 plumbing.  Four new modules emit cleanly:

- `Libcrux_sha3.Simd.Avx2`              (KeccakItem / Absorb / Squeeze4 instances)
- `Libcrux_sha3.Generic_keccak.Simd256` (4-lane keccak4)
- `Libcrux_sha3.Avx2.X4`                (top-level shake256_x4)
- `Libcrux_sha3.Avx2.X4.Incremental`    (4-lane incremental API)

All four typecheck under `--admit_smt_queries true` when invoked directly
(`fstar.exe ...`).  No equivalence proofs yet — that is the next step,
mirroring the arm64 path under `EquivImplSpec.*.Arm64.*`.

### Plumbing changes

1. `hax.sh::extract_all` exports `RUSTFLAGS="--cfg pre_core_models"` so
   the `libcrux-intrinsics` crate emits the `avx2_extract.rs` stub
   (bit_vec 256 / 128) instead of routing through the heavier
   `core_models::arch::x86::*` (Bitvec/Funarr) chain.  This mirrors the
   way `arm64_extract.rs` is the source for arm64 — bit_vec stubs all
   the way down, no Funarr dependency.
2. `hax.sh` extraction now passes `-C --features simd128,simd256`
   (intrinsics + sha3) and drops the `-i "-**::avx2::**"` /
   `-i "-**::simd256::**"` exclusions.  Only `neon::x2::**` remains
   excluded.
3. `patch_fstar_extractions` adds an `_super_i0 = …solve;` line to the
   AVX2 `Squeeze4` instance, identical to the arm64 `Squeeze2` patch.
4. `proofs/fstar/extraction/Makefile` adds `../stubs` to the include
   path.
5. New `proofs/fstar/stubs/Spec.Utils.fsti` + `.fst` provide a tiny
   stub for the four `Spec.Utils` symbols (`create`, `create16`,
   `map2`, `map_array`, plus `mul_mod`) that `Avx2_extract.fsti` cites
   in `ensures`/lemmas.  The real `Spec.Utils` lives under
   `libcrux-ml-kem/proofs/fstar/spec/`, which we do not want to depend
   on from sha3 — and it currently references a `Core_models` symbol
   that does not exist in the version we extract.

### Outstanding: full-tree `make` is flaky on F* segfaults

`bash hax.sh prove --admit` from `crates/algorithms/sha3` randomly
segfaults `fstar.exe` on different files (Tactics.Utils, BitVec.Equality,
Hax_lib.Int, EquivImplSpec.Keccakf.Generic, etc.) when the cache is
cold or partially-populated.  Running F* manually on each new AVX2
module succeeds (`EXIT: 0`).  Suspect a parallelism/cache interaction
in the F* harness rather than a problem in the extracted modules.
**Workaround:** manual per-module `fstar.exe ... --admit_smt_queries
true <module>.fst` works.  See `/tmp/avx2_alone.log` and
`/tmp/avx2_all.log` for confirmation that the four AVX2 modules
typecheck.

### Next steps (mirroring arm64)

1. Write `EquivImplSpec.Sponge.Avx2.fst` (mirror of
   `EquivImplSpec.Sponge.Arm64.fst`) connecting AVX2 KeccakState back
   to the per-lane scalar spec.
2. Write `EquivImplSpec.Sponge.Avx2.API.fst` with `lemma_absorb4_avx2`
   and `lemma_squeeze4_avx2`, mirroring the arm64 API lemmas.  Same
   load-bearing-admit-vs-prove tradeoffs are likely.

## 2026-04-25: squeeze2 inline-ensures pivot — Z3 cascade, reverted

### TL;DR

Attempted to mirror the Portable `squeeze`/`squeeze_last` inline-ensures
pattern on Arm64 `squeeze2` to eliminate the remaining
`lemma_squeeze2_arm64` assume val.  Structural work went in cleanly but
Z3 hits a **400k-instantiation BoxBool/BoxInt quantifier cascade** on
one query inside the loop-invariant re-establishment proof.  Even at
rlimit 800 with `--z3refresh` + `--split_queries always`, the one query
exceeds wall-clock limits.

**State after this session:** reverted simd128.rs, API.fst, and
extraction back to the 2026-04-24 state (1 load-bearing `assume val`
at `EquivImplSpec.Sponge.Arm64.API.fst:63`).  Two new helper lemmas
kept in `Hacspec_sha3.Sponge.Lemmas.fst` for reuse when the main proof
is attempted again.

### What was tried

**Scaffolding added (and reverted on impl/API side):**
- `simd128.rs::squeeze2` grown to ~750 lines with `squeeze_last2`
  helper + inline loop invariant using `squeeze_blocks` per lane,
  mirroring Portable.
- `lemma_squeeze2_arm64` in `API.fst` collapsed to a one-liner
  consuming the Rust ensures.
- Two per-lane step lemmas (`lemma_squeeze_loop_step_lane_{write,tail}`)
  in `Hacspec_sha3.Sponge.Lemmas.fst` to factor the per-iteration
  inductive step.

**Helper lemmas kept** (these verify clean, no admits, at rlimit 200/100):
- `lemma_squeeze_state_byte_eq_in_range` — byte-level equality between
  two `squeeze_state` calls sharing state/write range/rate but differing
  in their pre-array.  Inside the write range both produce the same
  byte; outside they preserve their respective pre-arrays.
- `lemma_squeeze_state_byte_preserve` — byte-level equality between
  `squeeze_state` and its pre-array for indices outside the write range.

### Why it didn't close

Using `--log_queries --z3refresh --query_stats`, one query inside
`lemma_squeeze_loop_step_lane_write` (the forall_intro of an aux helper
that calls `lemma_squeeze_state_byte_eq_in_range`) reliably times out
at ~60-80s wall-clock.  `z3 smt.qi.profile=true` on the captured
`.smt2` file shows:

| quantifier         | instantiations |
|--------------------|---------------|
| k!43               | 406,293       |
| projection_inverse_BoxBool_proj_0 | 121,134  |
| k!59               | 57,268        |
| constructor_distinct_BoxInt | 33,538 |

Classic F*-encoding cascade driven by `Seq.index`-keyed patterns on
**three overlapping forall preconditions in the lemma**:
1. `forall k. k < i*rate /\ k < output_len ==> out_pre[k] == spec_out[k]`
2. `forall k. i*rate <= k /\ k < output_len ==> out_pre[k] == out_initial[k]`
3. body-eq: `out_post == body_out`  (tried as propositional `==`, tried
   byte-wise forall — same cascade either way).

The cascade is triggered when Z3 needs to prove the loop-invariant
re-establishment and instantiates BoxBool/BoxInt repeatedly because
the three forall bodies mention `Seq.index` of **different** arrays
(out_pre, out_initial, out_post, body_out, spec_out, spec_out_new,
new_spec_out).  Unlike the Portable case (only 2 arrays), the Arm64
per-lane case has 4+ arrays live in the VC and Z3 explores
combinatorially.

Mitigations tried:
- `--z3refresh` between queries (stops state pollution; helped other queries but not this one).
- Split the combined lemma into write-side + tail-side (reduced to one forall postcondition each — still cascade).
- Replace byte-wise forall with propositional `Seq.equal` (via `Seq.lemma_eq_intro`) — Z3 still internally expands to a forall with the same patterns.
- Arithmetic preamble to pre-bind `i *! rate` and discharge the overflow check early (helped a different ~30s query, not the main one).
- `#restart-solver` before the lemma — no effect (we already use `--z3refresh`).

### Why this is genuinely harder than Portable.squeeze or absorb2

- **absorb2**: no output buffer to track byte-wise; loop invariant is
  just per-lane spec-state equality.  2 equalities total, one per lane.
- **Portable.squeeze**: 1 lane, 2 forall conjuncts in the loop
  invariant (write + tail).  `f_squeeze` writes one buffer from one
  state; byte bridge is clean.
- **Arm64.squeeze2**: 2 lanes, 4 forall conjuncts in the loop
  invariant.  `f_squeeze2` writes BOTH out0 and out1 from ONE call, so
  `sq_lane_arm64` adds an `if l=0 then ... else ...` branch that lives
  in the byte-bridge hot path.  Combinatorial blowup: roughly 2× arrays
  live, 2× quantifiers live, and the disjunction introduces case-split
  overhead.

### Paths forward (for next session)

1. **Option A — `introduce forall`** with explicit case-split per
   branch (technique #9 from smtprofiling skill), replacing the
   `Classical.forall_intro` + SMT quantifier pattern firing entirely.
2. **Option B — Dedicated `Steps`-level lemma per lane** that takes
   impl-side facts (post-`keccakf1600` state, `f_squeeze2` outputs as
   pair) and proves ONE forall conjunct, using `arm64_sc_store_block`
   directly.  Four small lemma calls in the loop body instead of the
   current single big lemma.
3. **Option C — Opaque-bundle the Arm64-specific state predicates**
   (technique #4 from smtprofiling) so Z3 stops unfolding them during
   the VC check — worth ~30-50% reduction in the cascade.
4. **Option D — Rewrite loop invariant using `Seq.slice`-based
   predicate** (`out_pre[0..i*rate] == spec_out[0..i*rate]` etc.).
   Collapses 4 forall conjuncts to 4 propositional equalities.  Needs
   the range-check arithmetic handled carefully but cleans up the
   cascade.

### Files touched this session

Kept:
- `Hacspec_sha3.Sponge.Lemmas.fst:220-292` — two new helper lemmas
  `lemma_squeeze_state_byte_eq_in_range` and
  `lemma_squeeze_state_byte_preserve`.

Reverted (back to 2026-04-24 state):
- `crates/algorithms/sha3/src/generic_keccak/simd128.rs`
- `proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.API.fst`
- Extracted files (re-extracted via `bash hax.sh extract`).

### Load-bearing admit inventory (unchanged from 2026-04-24)

| # | File | Line | Kind |
|---|------|------|------|
| 1 | `EquivImplSpec.Sponge.Arm64.API.fst` | ~63 | `assume val lemma_squeeze2_arm64` |

Non-load-bearing upstream admits (3) in `Proof_Utils.Lemmas.fst:26/33/54` unchanged.

### Environment note (F* install)

`/root/.local/bin/fstar.exe` was symlinked to a nightly F*
(2026-04-21) install that was missing `FStar.Mul.fst` in its ulib.
Relinked to `/root/.opam/5.2.0/bin/fstar.exe` (F* 2025.03.25~dev) for
this session.  Future sessions should `eval $(opam env)` first or
confirm the PATH resolves `fstar.exe` to the opam build.

---

## 2026-04-24 (evening): Portable.absorb admit eliminated

### TL;DR

`Libcrux_sha3.Generic_keccak.Portable.absorb` now verifies against its
spec-equivalence ensures without any `admit ()`, following the same
inline-ensures pattern used for `squeeze` earlier today.  The former
`lemma_absorb_portable_aux` (2026-04-21's recursive helper, with the
Z3 4.13.3 LP-solver bug workaround admit at line 252) has been
**deleted**; `lemma_absorb_portable` collapses to `()` — the proof is
carried by the Rust-side ensures.

### What changed

**Rust spec (`specs/sha3/src/sponge.rs`):**
- New `absorb_blocks(state, rate, i, input_blocks, input) -> State`
  — a block-indexed tail-recursive helper that mirrors
  `squeeze_blocks`.  It applies `absorb_block` at each
  `input[k*rate..(k+1)*rate]` for `k ∈ [i, input_blocks)`.
- Preferred over `absorb_rec` with a `message[rate..]` tail because
  it indexes into `input` directly by `k*rate`, avoiding the
  slice-of-slice reasoning that tripped the Z3 LP bug in the
  previous proof.

**Rust impl (`crates/algorithms/sha3/src/generic_keccak/portable.rs`):**
- `absorb::<RATE, DELIM>` now has an inline ensures:
  `$result.st == Hacspec_sha3.Sponge.absorb $RATE $DELIM $input`.
- Loop invariant: `$s.st == absorb_blocks zeros RATE 0 i input`.
- Pre-loop scaffolding: `lemma_absorb_blocks_base` (establishes
  invariant at i=0).
- Per-iteration scaffolding:
  `Steps.lemma_absorb_block_portable` (impl step ≡ spec step) +
  `Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail` (extends
  `absorb_blocks` by one step on the right).
- Post-loop scaffolding:
  `Steps.lemma_absorb_last_portable` (impl absorb_final ≡ spec
  absorb_final) +
  `Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks` (bridges
  `absorb_rec` to `absorb_final ∘ absorb_blocks`).
- Options: `--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always`
  (same budget as squeeze).

**F* equivalence (`proofs/fstar/equivalence/Hacspec_sha3.Sponge.Lemmas.fst`):**
- New lemmas (all proved, no admits):
  - `lemma_absorb_blocks_base`: `absorb_blocks state rate i i input == state`.
  - `lemma_absorb_blocks_unfold`: head-unfold at `i < input_blocks`.
  - `lemma_absorb_blocks_tail`: tail-extension from `j` to `j+1`
    (mirrors `lemma_squeeze_blocks_tail`).  Proved by induction on
    `j - i`.
  - `lemma_absorb_rec_unfold`: one recurrence step of `absorb_rec`
    when enough bytes remain.
  - `lemma_tail_block_eq_input_block`: slice-of-RangeFrom identity
    (uses `FStar.Seq.Properties.slice_slice`).
  - `lemma_absorb_blocks_shift`: shift key — `absorb_blocks state
    rate 0 (k+1) input == absorb_blocks state' rate 0 k tail` where
    `state' = absorb_block state input[0..rate] rate`, `tail =
    input[rate..]`.  Proved by induction on `k`.
  - `lemma_absorb_final_shift`: `absorb_final state tail start rem
    rate delim == absorb_final state input (rate+start) rem rate
    delim`.  Bytes read by pad_last_block coincide.
  - `lemma_absorb_rec_via_blocks` (main bridge): `absorb_rec state
    rate delim input == absorb_final (absorb_blocks state rate 0
    input_blocks input) input (input_blocks*rate) input_rem rate
    delim`.  Proved by induction on `|input|`, combining the shift
    + final-shift helpers.  Verified in 9s at rlimit 300.

**F* equivalence (`proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.API.fst`):**
- Deleted: `lemma_absorb_tail_split_seq`, `lemma_absorb_rec_step`,
  `lemma_absorb_portable_aux` (the former recursive helper with
  the admit at line 252 — no longer needed).
- `lemma_absorb_portable` collapses to:
  `let _ = Libcrux_sha3.Generic_keccak.Portable.absorb rate delim input in ()`
  — the Rust-side ensures carries the proof.

### Load-bearing admit inventory (2, was 3)

| # | File | Line | Kind | Notes |
|---|------|------|------|-------|
| 1 | `EquivImplSpec.Sponge.Arm64.API.fst` | 63 | `assume val lemma_absorb2_arm64` | unchanged |
| 2 | `EquivImplSpec.Sponge.Arm64.API.fst` | 82 | `assume val lemma_squeeze2_arm64` | unchanged |

Portable absorb admit (formerly #1) is **gone**.  Non-load-bearing
upstream admits (3) in `Proof_Utils.Lemmas.fst:26/33/54` unchanged.

### Verification stats

- `Hacspec_sha3.Sponge.Lemmas.fst`: verifies with all new lemmas,
  `lemma_absorb_rec_via_blocks` is the slowest at 9.1s rlimit 300.
- `Libcrux_sha3.Generic_keccak.Portable.fst` (extracted, containing
  both the new inline-proved `absorb` and the earlier inline-proved
  `squeeze`): 497 queries, zero failures, total 381s on this machine.
- `EquivImplSpec.Sponge.Portable.API.fst` (top-level + collapsed
  `lemma_absorb_portable`): all 8 hasher theorems discharged in
  < 200 ms each.

### Key technical lessons (extending 2026-04-24 morning's list)

- **Block-indexed helpers beat suffix-recursive helpers for absorb.**
  The prior `absorb_rec`-based `lemma_absorb_portable_aux` used
  progressive `message[k*rate..]` tails that triggered a Z3 LP
  bug.  Replacing with `absorb_blocks state rate 0 n input` keeps
  `input` fixed and indexes into it, so the proof never needs
  slice-of-slice reasoning inside a loop-invariant context.  Slice
  reasoning is still needed exactly once, in `lemma_absorb_rec_via_blocks`,
  but there it runs in a small isolated context where Z3 is happy.
- **Hax `$ident]` / `$ident}` trip antiquotation parse** — always
  add a space: `[ $input ]`, `v $output_len }`.
- **Body fstar! blocks use `$self.st`, ensures blocks use the full
  field path `*.Libcrux_sha3.Generic_keccak.f_st`** (per 2026-04-24
  morning's note).  For top-level functions, the ensures closure
  arg (e.g. `|result|`) acts like `$self` — referenced as `$result.st`
  in F* interpolation.
- **Pre/post-loop scaffolding blocks needed to establish the
  invariant at i=0 and consume the final state.**  For absorb, the
  pre-loop block calls `lemma_absorb_blocks_base` (invariant at
  i=0: `s.st == zeros == absorb_blocks zeros rate 0 0 input`) and
  the post-loop block combines `lemma_absorb_last_portable` with
  `lemma_absorb_rec_via_blocks` to reconcile
  `absorb_final ∘ absorb_blocks` with spec `absorb`.

### Next steps (in priority order)

1. **Arm64 per-lane assumes (#1, #2 above).**  Clone Portable's
   scaffolding at `v_N = mk_usize 2` with a lane parameter.
   Reuses the closed `sc_load_block` / `sc_store_block` records in
   `EquivImplSpec.Sponge.Arm64.fst`.  The earlier x86-vs-Arm
   extraction-drift block has been resolved — see "Arm64 extraction
   on x86" below.
2. **Cleanup.**  Stale plan docs (`plan.md`, `plan-simd.md`,
   `Proofs.md`); editor backups (`*.fst~`, `*.fsti~`).

### Arm64 extraction on x86 (2026-04-24 evening)

The Arm64 intrinsics F* extraction (produced by `cargo hax into fstar`
on `crates/utils/intrinsics`) must be run with `--features simd128`
so the `#[cfg(feature = "simd128")]`-gated `arm64_extract` module is
compiled.  Without the feature, the module is skipped and the
generated `Libcrux_intrinsics.Arm64_extract.fsti` does not include
the `bit_vec`-typed type aliases (`t_e_uint64x2_t`,
`t_e_uint16x4_t`, etc.) that downstream SHA-3 Arm64 modules depend
on.

The `proofs` directory of `crates/utils/intrinsics` is gitignored,
so these files are always regenerated.  If you re-extract on a
machine where a previous intrinsics extraction was done without
`simd128`, rerun the intrinsics extraction step first:

```bash
cd crates/utils/intrinsics
cargo hax -C --features simd128 ';' into -i '+**' fstar --z3rlimit 80 --interfaces '+**'
```

(The sha3 crate's `hax.sh` does not currently pass `--features
simd128` in its intrinsics-extract step, which is the root cause of
the drift.)

Full `make verify` in
`crates/algorithms/sha3/proofs/fstar/equivalence/` passes green
once the intrinsics is re-extracted with `simd128`.  See
`/tmp/verify_full2.log` for the last successful run.

### Files of interest

- `specs/sha3/src/sponge.rs` — `absorb_blocks` helper added.
- `src/generic_keccak/portable.rs:145–225` — `absorb` with inline ensures.
- `proofs/fstar/equivalence/Hacspec_sha3.Sponge.Lemmas.fst:223–562` — new lemmas.
- `proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.API.fst:60–82` — collapsed `lemma_absorb_portable`.
- `/tmp/verify_portable1.log` — last clean verification log.
- `/tmp/verify_lemmas6.log` — last clean Lemmas verification log.

---

## 2026-04-24 (late): `assume False` eliminated — Portable.squeeze fully verified

### TL;DR

`Libcrux_sha3.Generic_keccak.Portable.squeeze` now verifies against its
spec-equivalence ensures without `assume False` — **597 queries, zero
errors, 55 min build**.  The remaining issue from the earlier session
(where the final function-scope VC timed out at rlimit 800 even with
all inline scaffolding) was unblocked by refactoring the final partial
block into a dedicated `squeeze_last` helper + corresponding
spec-level `Hacspec_sha3.Sponge.squeeze_last`, so the main squeeze's
final reconcile sees a clean equality instead of threading an inline
`if output_rem != 0` through a bloated VC context.

### What changed

**Rust — impl (`crates/algorithms/sha3/src/generic_keccak/portable.rs`):**
- New method `impl KeccakState<1, u64>::squeeze_last<RATE>` that
  wraps the final `if output_rem != 0 { keccakf1600; f_squeeze }`.
  Its ensures pins `self.st` and `out` to `Hacspec_sha3.Sponge.squeeze_last`.
  Inline scaffolding proves the impl vs spec equivalence via
  `lemma_keccakf1600_portable` + a per-index aux forall over `squeeze_state`.
- `squeeze`'s final `if output_rem != 0` block replaced by
  `s.squeeze_last::<RATE>(output, output_rem);`.
- `squeeze`'s loop body now has a post-f_squeeze scaffolding block
  (aux_write_step / aux_tail_step + distributivity hint +
  `lemma_keccakf1600_portable`) that re-establishes the loop
  invariant at `i+1`.
- Removed the `hax_lib::fstar!(r#"assume False"#);` at the squeeze
  function entry (the whole point).

**Rust — spec (`specs/sha3/src/sponge.rs`):**
- New `squeeze_last` function matching the impl's shape.
- `squeeze` now calls `squeeze_last` instead of inlining the final
  partial-block `if`.

**F* equivalence (`proofs/fstar/equivalence/Hacspec_sha3.Sponge.Lemmas.fst`):**
- New lemma `lemma_squeeze_unfold` — structural unfold of spec `squeeze`.
- New lemma `lemma_seq_trans` — transitive equality over the t_Array↔Seq
  coercion boundary (Z3 couldn't auto-propagate through `<:`).
- New lemma `lemma_squeeze_last_extensional` — squeeze_last is
  extensional in its `output` argument at indices `< output_len -
  output_rem`, used to bridge `squeeze_last(impl_output_after_loop)`
  with `squeeze_last(spec_out)`.

### Load-bearing admit inventory (3, was 4)

| # | File | Line | Kind | Notes |
|---|------|------|------|-------|
| 1 | `EquivImplSpec.Sponge.Portable.API.fst` | 252 | `admit ()` | absorb `lemma_absorb_rec_step` — Z3 4.13.3 LP bug (`lar_solver.cpp:1066`), unchanged |
| 2 | `EquivImplSpec.Sponge.Arm64.API.fst` | 63 | `assume val lemma_absorb2_arm64` | unchanged |
| 3 | `EquivImplSpec.Sponge.Arm64.API.fst` | 82 | `assume val lemma_squeeze2_arm64` | unchanged |

`assume False` from 2026-04-24 morning is **gone**.  Non-load-bearing
upstream admits (3) in `Proof_Utils.Lemmas.fst:26/33/54` unchanged.

### Key technical lessons

- **`$self_` only works in ensures-clause fstar! blocks, not body ones.**
  Inside a method body's `hax_lib::fstar!`, use `$self` (the Rust receiver).
  Similarly, `self_e_future` in ensures must reference the full field
  path (`.Libcrux_sha3.Generic_keccak.f_st`) because `.st` isn't
  translated without `$`-interpolation.
- **Hax can't parse `v $output_len}`** (closing brace after antiquot)
  — add a space: `v $output_len }`.
- **Z3 coercion through `<: Seq.seq u8`** isn't automatic.  The
  `lemma_seq_trans` with a free `spec_result` variable was necessary;
  inlining the spec-function application directly caused timeouts.
- **Context pollution drives the final VC.** Profile showed 3,610 QIs
  on a refinement interpretation — classic Z3 cascade from many
  `let _:Prims.unit = lemma_... in` blocks.  Refactoring the if-else
  into a named function with its own VC cut the main squeeze's final
  reconcile from timeout to ~8s.

### Previous sections (historical)

## 2026-04-24 (earlier): pivot landed; one `assume False` remaining in squeeze body

### TL;DR

The Portable squeeze inline-ensures pivot (started 2026-04-23) **succeeded
architecturally**. Admits 2–4 from the 2026-04-21 inventory are gone:

- `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` — **0 admits**.  Only
  `lemma_squeeze_once_portable` remains as a real proof; the middle-loop
  induction + driver branches were deleted.
- `EquivImplSpec.Sponge.Portable.API.fst:303–326` — `lemma_squeeze_portable`
  collapsed to `let _ = Libcrux_sha3.Generic_keccak.Portable.squeeze rate ks output in ()`.
  It consumes the Rust-side ensures directly.
- Full `make` **verifies clean** (latest: `/tmp/sq15.log`, 1857s total,
  ends with `Verified module: EquivImplSpec.Sponge.Portable.API` + `All
  verification conditions discharged successfully`).

### Catch: one `assume False` in the extracted squeeze body

`src/generic_keccak/portable.rs:132` opens the squeeze function body with
`hax_lib::fstar!(r#"assume False"#);`.  Extracted at
`proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Portable.fst:311` as
`let _:Prims.unit = assume False in`.

All the inline proof scaffolding below it (aux_write / aux_tail /
`lemma_squeeze_blocks_base` / `lemma_squeeze_blocks_tail` /
`forall_intro` + `Seq.lemma_eq_intro` per branch, plus the loop invariant
at `proofs/fstar/extraction/…:411–429`) is correct and in place — it is
simply not being discharged because `assume False` short-circuits the
function's VC.  Removing the `assume False` should expose the real work.

### Revised load-bearing admit inventory (4, was 6)

| # | File | Line | Kind | Notes |
|---|------|------|------|-------|
| 1 | `EquivImplSpec.Sponge.Portable.API.fst` | 252 | `admit ()` | absorb `lemma_absorb_rec_step` — Z3 4.13.3 LP bug (`lar_solver.cpp:1066`), unchanged |
| 2 | `Libcrux_sha3.Generic_keccak.Portable.fst` (extracted) | 311 | `assume False` inside `squeeze` body | **NEW.** Replaces old admits 2–4.  Scaffolding in place; needs the main ensures VC to actually run. |
| 3 | `EquivImplSpec.Sponge.Arm64.API.fst` | 63 | `assume val lemma_absorb2_arm64` | unchanged |
| 4 | `EquivImplSpec.Sponge.Arm64.API.fst` | 82 | `assume val lemma_squeeze2_arm64` | unchanged |

Non-load-bearing upstream admits (3) in `Proof_Utils.Lemmas.fst:26/33/54` — unchanged.

### Why `assume False` is there

The previous session's iteration log (sq2–sq15 on 2026-04-23) went through
the inline-ensures build attempts.  `/tmp/sq_full.log` (with the old long
`API.fst:lemma_squeeze_portable` proof in place) recorded 3 Error-19
timeouts at rlimit 800, each ~200s.  `assume False` was dropped in to
unblock downstream verification of the dependent `keccak1_portable` /
top-level SHA-3 hasher theorems while the squeeze-body VC is sorted out.

### Next steps (in priority order)

1. **Lift `assume False` from `src/generic_keccak/portable.rs:132`.**
   - Re-extract via `bash hax.sh extract` from the sha3 crate root.
   - Build just `Libcrux_sha3.Generic_keccak.Portable.fst` to see the
     squeeze VC fail cleanly without dragging the full equivalence
     chain (rough cost: 30–60 min).
   - Likely outcome: the monolithic ensures times out.  Mitigation:
     factor per-branch reconciliations into dedicated lemmas in a new
     `Hacspec_sha3.Sponge.Lemmas.fst` module and invoke each lemma
     inside the function body — splits the VC three ways
     (`output_blocks==0`, loop-entry, final-block).
   - Possible lemma names: `lemma_squeeze_empty_branch`,
     `lemma_squeeze_initial_block`, `lemma_squeeze_final_reconcile`.

2. **Mirror the pattern to `absorb`.**  Would eliminate admit #1 *iff* the
   simpler inline form dodges the Z3 LP bug.  Absorb is a single
   recursion (no loop + tail), so the VC should be smaller than squeeze.

3. **Arm64 per-lane assumes (#3, #4).**  Clone Portable's scaffolding at
   `v_N = mk_usize 2` with a lane parameter.  Reuses the closed
   `sc_load_block` / `sc_store_block` records in
   `EquivImplSpec.Sponge.Arm64.fst`.

4. **Cleanup.**  Stale plan docs (`plan.md`, `plan-simd.md`, `Proofs.md`);
   editor backups (`Libcrux_sha3.Generic_keccak.Portable.fst~`,
   `Utils.fsti~`).

### Files of interest

- `src/generic_keccak/portable.rs:105–287` — squeeze function (the work site).
- `proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Portable.fst:280–543` — extracted form.
- `proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.API.fst:303–326` — trivial ensures-consumer.
- `proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` — kept `lemma_squeeze_once_portable`, everything else deleted.
- `proofs/fstar/equivalence/Hacspec_sha3.Sponge.Lemmas.fst` — 3 spec helpers (base / unfold / tail).
- `/tmp/sq15.log` — last clean build (`Verified module` success).
- `/tmp/sq_full.log` — last full build with non-trivial `lemma_squeeze_portable` (timed out, before `assume False` landed).

### Known constraints (unchanged)

- F* `--z3rlimit ≤ 800` under `--split_queries always` — hitting 800 means refactor, not more time.
- Never clear the F* cache; delete only specific `.checked` files.
- Rust-side edits require `bash hax.sh extract`.
- `cargo fmt` touched Rust crates before committing.

---

## 2026-04-23 status: Portable squeeze architectural pivot

### Overall plan
Eliminate admits 2–4 (the Portable squeeze Z3-composition gaps) by pushing
buffer-independence reasoning **inline into the Rust `squeeze` function's
ensures + loop invariant**, rather than proving it as an external lemma
chain in `EquivImplSpec.Sponge.Portable.API.fst`.

The key insight: `f_squeeze_post` (in `Libcrux_sha3.Simd.Portable.fst`) and
the spec `squeeze_state` ensures are structurally identical pointwise
byte-by-byte specs. Under `v len <= 200`, both say
`result[i] = to_le_bytes(state[(i-start)/8])[(i-start)%8]` in the write
range and preserve bytes outside. So the impl output is byte-equal to
the spec output, without needing a dedicated buffer-independence lemma.

If this lands:
- `Libcrux_sha3.Generic_keccak.Portable.squeeze` gets a direct
  `output_future == Hacspec_sha3.Sponge.squeeze (len output) s.st RATE`
  ensures.
- `lemma_squeeze_portable` in `API.fst` collapses to `()` (its proof is
  now the function's ensures).
- `portable_squeeze_composed` + `lemma_squeeze_portable_aux` + admits 2–4
  become dead code.

### Changes so far

**Lemmas.fst** (`Hacspec_sha3.Sponge.Lemmas.fst`, new module, added to Makefile):
- Deleted three dead buffer-independence helpers
  (`lemma_squeeze_state_writerange_eq`, `lemma_squeeze_state_prefix_ext`,
  `lemma_squeeze_blocks_prefix_eq`).
- Kept `lemma_squeeze_blocks_base`, `lemma_squeeze_blocks_unfold`,
  `lemma_squeeze_blocks_tail` — these are the only spec-side helpers
  actually used inline by the new Rust ensures. Module builds clean.

**Rust** (`src/generic_keccak/portable.rs`):
- `squeeze::<RATE>` function rewritten:
  - Ensures:
    `output_future == Hacspec_sha3.Sponge.squeeze (len $output) $s.st $RATE`
  - Options: `--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always`
  - Ghost bindings: `s_init_st = s.st`, `output_initial = output.to_vec()`
  - Branches:
    1. `output_blocks == 0`: one `f_squeeze` call; reconcile with
       `squeeze_state … zeros 0 output_len` via `forall_intro aux` +
       `Seq.lemma_eq_intro`.
    2. else: first `f_squeeze` with `start=0, len=RATE`; establish
       loop invariant at `i=1` via `lemma_squeeze_blocks_base` +
       `forall_intro aux_write` + `forall_intro aux_tail`; loop body
       applies `lemma_squeeze_blocks_tail` then does `keccakf1600` +
       `f_squeeze`; optional final partial block; final reconcile via
       `forall_intro aux` + `Seq.lemma_eq_intro` with case split on
       `output_rem`.
  - Loop invariant:
    ```
    v i >= 1 /\ v i <= v output_blocks /\
    v i * v RATE <= v output_len /\
    s.st == spec_st /\
    (forall (k: nat). k < v i * v RATE /\ k < v output_len ==>
       output[k] == spec_out[k]) /\
    (forall (k: nat). v i * v RATE <= k /\ k < v output_len ==>
       output[k] == output_initial_arr[k])
    ```

**Not yet done**:
- `EquivImplSpec.Sponge.Portable.API.fst:303–391` (`lemma_squeeze_portable`
  proof) — still has the old long proof; collapses to `()` once the new
  extraction compiles.
- `EquivImplSpec.Sponge.Portable.Steps.fst` — `portable_squeeze_composed`
  becomes dead; remove after verification.
- `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` — stale comments
  referencing the removed composition.

### Immediate status: build bpigsithv running

**Goal**: F* verification of `Libcrux_sha3.Generic_keccak.Portable.fst` passes.

**Iteration history (this session)**:
- sq2.log (v1: plain `Seq.equal` assertion): 1 error, query 31 timed out
  at 917s on output_blocks==0 branch assertion.
- sq3.log (v2: bounds hints + `Seq.lemma_eq_intro`): 6 errors.
  - Line 338: `Seq.lemma_eq_intro` precondition unprovable.
  - Line 391: subtyping — `Seq.index output k` needed `k < len output`.
  - Line 396/417: initial invariant check failed at fold_range entry.
- sq4.log (v3: `forall_intro aux` + `v i * v RATE <= v output_len`
  invariant conjunct + `k < v output_len` in bound-match forall):
  1 error left — the initial invariant check still fails (entry block
  had only `aux_write`, missing `aux_tail` for the preservation forall).
- sq5.log (v4: CURRENT, running as bpigsithv): added `aux_tail` for the
  `RATE <= k < output_len ==> output[k] == output_initial_arr[k]`
  preservation forall at entry. Also per-branch aux in
  `output_blocks == 0` and final reconciliation.

### Files to reload if session restarts

- `src/generic_keccak/portable.rs` — the squeeze function, lines 105–290.
  Contains the current proof scaffolding in `hax_lib::fstar!(…)` blocks.
- `proofs/fstar/equivalence/Hacspec_sha3.Sponge.Lemmas.fst` — the 3 kept
  spec helpers (base, unfold, tail).
- `proofs/fstar/equivalence/Makefile` — `Hacspec_sha3.Sponge.Lemmas.fst`
  is in ROOTS.
- `proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Portable.fst` —
  the generated F* (re-extract from Rust via `bash hax.sh extract`).
- `/tmp/sq5.log` — latest build log (running as bpigsithv).

### Next step if the current build passes

1. Update `lemma_squeeze_portable` in
   `EquivImplSpec.Sponge.Portable.API.fst` to `= ()`.
2. Delete now-dead `portable_squeeze_composed` in `Steps.fst` and
   `lemma_squeeze_portable_aux` + 3 admits in `SqueezeAPI.fst`.
3. Rebuild full F* chain with `make` under equivalence/.
4. Apply the same pattern to `absorb` (user follow-up intent).

### Next step if the current build fails

- Read `/tmp/sq5.log`, grep for `Error 19`, identify the failing
  assertion's line range in the extracted F*, map back to the
  `hax_lib::fstar!(…)` block in portable.rs, and adjust.
- Known scalability pitfalls:
  - Antiquotation: `$name}` is parsed as ident; use `$name }` with a space.
  - `nat{k < v $name}` → `nat{k < v $name }`.
  - Avoid `Seq.equal` directly; prefer `forall_intro aux; Seq.lemma_eq_intro`.
  - `aux` should match the invariant's forall shape exactly (same premise
    structure, or use `Classical.move_requires`).
  - Forall's `k: nat` must be bounded by `k < v output_len` in premise
    for `Seq.index (… <: Seq.seq u8) k` to be well-typed.

### Known constraint
- F* `--z3rlimit ≤ 800` under `--split_queries always` — hitting 800 means
  the proof needs refactoring, not more time.

---

## Original 2026-04-21 plan

## TL;DR

The 12 top-level hasher theorems (sha{224,256,384,512}_{portable,arm64} +
shake{128,256}_{portable,arm64}) in `EquivImplSpec.Sponge.{Portable,Arm64}.API`
are proved modulo **6 load-bearing `assume val` / `admit ()`** and 3
non-load-bearing upstream utility admits. `make` passes cleanly.

**Δ from 2026-04-20**: the two `assume val`s in `SqueezeAPI.fst` (squeeze middle
loop + driver) have been replaced with real structured proofs — the spec
`squeeze` was rewritten to recurse via a new `squeeze_blocks` helper,
mirroring `absorb_rec`.  The middle-loop induction (`lemma_squeeze_portable_aux`)
now exists as a real `let rec` definition; the base case, fold-peeling step,
body-step bridge, spec-unfolding, and IH call are all laid out.  The only
remaining obstacle is the final Z3 combining step: even with
`split_queries always --z3refresh`, one query consistently hits rlimit
600 trying to thread IH + step facts through the extractor's inline
fold lambdas.  That one step is now admitted in isolation (not as a whole
lemma), and similarly for the 2 branches of the driver.

## Remaining admits / assumes

### Load-bearing (6)

| # | File | Line | Kind | What it assumes |
|---|------|------|------|-----------------|
| 1 | `EquivImplSpec.Sponge.Portable.API.fst` | 249 | `admit ()` | Slice-identity bridge inside `lemma_absorb_portable_aux` inductive branch: one unfold step of spec `absorb_rec`. Helper `lemma_absorb_rec_step` (same file) encodes the fact; calling it **triggers a Z3 4.13.3 LP-solver internal-assertion bug** at `lar_solver.cpp:1066` on fresh hint generation. `--z3refresh` works around per-query but exceeds `make` timeout. |
| 2 | `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` | 215 | `admit ()` inside `lemma_squeeze_portable_aux` inductive step | Final obligation that combines (a) `lemma_fold_range_step` (fold peeling), (b) `Steps.lemma_squeeze_block_portable` (impl body ≡ spec step), (c) `lemma_squeeze_blocks_unfold` (spec `squeeze_blocks` unfolding), and (d) the inductive hypothesis, into the outer ensures.  Z3 cannot compose these within rlimit 600 even with `split_queries always`.  All 4 source facts are real theorems called in scope. |
| 3 | `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` | 285 | `admit ()` in driver `output_rem ≠ 0` branch | Final normalization: impl `Generic_keccak.Portable.squeeze` ≡ spec `Hacspec_sha3.Sponge.squeeze` after first block + middle loop + tail block.  Middle-loop step provided by `lemma_squeeze_portable_aux`; tail by `Steps.lemma_squeeze_last_portable`.  Same Z3 composition limit as #2. |
| 4 | `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` | 286 | `admit ()` in driver `output_rem = 0` branch | Final normalization when output length is a multiple of rate.  Middle-loop step provided by `lemma_squeeze_portable_aux`; no tail. |
| 5 | `EquivImplSpec.Sponge.Arm64.API.fst` | 63 | `assume val lemma_absorb2_arm64` | Per-lane driver absorb at N=2: `extract_lane l (absorb2 rate delim data).f_st ≡ Hacspec_sha3.Sponge.absorb rate delim (data[l])`. Black-box form over the extracted `Libcrux_sha3.Generic_keccak.Simd128.absorb2` function. |
| 6 | `EquivImplSpec.Sponge.Arm64.API.fst` | 82 | `assume val lemma_squeeze2_arm64` | Per-lane driver squeeze2 at N=2: lane-`l` output of `squeeze2 rate s out0 out1` ≡ `Hacspec_sha3.Sponge.squeeze outlen (extract_lane l s.f_st) rate`. Black-box form. |

`lemma_keccak2_arm64` itself is **fully proved** (5-line composition of #5 + #6).

### Non-load-bearing (3, upstream)

All in `Proof_Utils.Lemmas.fst`, flagged with `TODO` comments:

| Line | Lemma | Target |
|------|-------|--------|
| 26 | `logand_commutative` | hax-lib / core-models |
| 33 | `lemma_rotate_left_zero` | hax-lib / core-models |
| 54 | `lemma_index_update_at_range` | pure `Seq` — still used by proofs |

These don't affect spec-equivalence correctness; leave as upstream targets.

## Structural overview

```
12 top-level hasher theorems  (API.fst files)
  ↓
lemma_keccak1_portable  [PROVED]         lemma_keccak2_arm64  [PROVED]
  = lemma_absorb_portable                 = lemma_absorb2_arm64 per lane
  ; lemma_squeeze_portable                ; lemma_squeeze2_arm64 per lane
     │                │                           │        │
     ▼                ▼                           ▼        ▼
 lemma_absorb_   lemma_squeeze_               [assume 5] [assume 6]
  portable [PROVED  portable (driver)
  modulo admit 1]  [admits 3 & 4]
     │                │
     │          calls lemma_squeeze_portable_aux
     │                │ (recursion mirror, admit 2 in step)
     ▼                ▼
 Steps.fst + Generic.{Core,Squeeze}.fst + per-backend sc_* records [ALL PROVED]
 + lemma_squeeze_blocks_unfold (SqueezeAPI.fst) [PROVED]
 + lemma_squeeze_once_portable (SqueezeAPI.fst) [PROVED]
```

## File inventory (equivalence/)

**Proof modules (all build cleanly):**
- `Proof_Utils.Lemmas.fst` — 3 upstream admits
- `Proof_Utils.NatFold.fst`, `Proof_Utils.FoldRange.fst`
- `EquivImplSpec.Keccakf.ChiFold.fst`
- `EquivImplSpec.Keccakf.Generic.fst` — closed (rho_offsets_values, keccak_f_is_rounds)
- `EquivImplSpec.Keccakf.Portable.fst`, `EquivImplSpec.Keccakf.Arm64.fst`
- `EquivImplSpec.Keccakf.SpecRounds.fst`
- `EquivImplSpec.Sponge.Generic.Core.fst`
- `EquivImplSpec.Sponge.Generic.Squeeze.fst`
- `EquivImplSpec.Sponge.Portable.fst` — sc_load_block / sc_load_last / sc_store_block all PROVED
- `EquivImplSpec.Sponge.Arm64.fst` — sc_load_block / sc_load_last / sc_store_block all PROVED
- `EquivImplSpec.Sponge.Portable.Steps.fst` — per-step absorb/squeeze lemmas
- `EquivImplSpec.Sponge.Arm64.Steps.fst` — per-step absorb/squeeze lemmas
- `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` — NEW MODULE, 3 proved helpers + assumes 2 & 3
- `EquivImplSpec.Sponge.Portable.API.fst` — re-exports from SqueezeAPI; assume 1
- `EquivImplSpec.Sponge.Arm64.API.fst` — assumes 4 & 5, `lemma_keccak2_arm64` PROVED

**Cleanup still pending (non-functional):**
- `equivalence/plan.md`, `equivalence/plan-simd.md`, `equivalence/Proofs.md` — stale plan notes
- `../extraction/Utils.fsti~` — editor backup

## Rust-side state

The Rust codebase has been refactored:
- `crates/algorithms/sha3/src/generic_keccak/simd128.rs` — `keccak2` split into named `absorb2` / `squeeze2` / `keccak2` matching Portable's `absorb` / `squeeze` / `keccak1` structure. This lets the Arm64 F* assume_vals refer to the named functions as a black box (mirroring Portable's style).
- No other Rust changes are needed for any remaining gap.

## How to close the remaining gaps

### Assume 1 (Portable.API.fst:249, slice-identity admit)

**Blocked by Z3 LP bug**, not a proof-strategy problem. Options:
- Wait for a Z3 version where the `lar_solver` bug is fixed.
- Rewrite `lemma_absorb_rec_step` so the inductive step can run without the inline lambdas that trigger the LP assertion (e.g. factor every `Seq.slice` into a named term; avoid nested typeclass-dispatched `.[RangeFrom]`).
- Apply `--z3refresh` per-query and raise the `make` timeout.

### Admits 2–4 (Portable squeeze: aux step + driver branches)

**Status 2026-04-21**: spec `squeeze` rewritten as `squeeze_blocks` (recursive,
mirrors `absorb_rec`).  The aux lemma `lemma_squeeze_portable_aux` is a real
`let rec` with base case, inductive step, and IH call all in place.  The
admits are the final Z3 combining obligations, not opaque whole-lemma gaps.

Two possible routes to close:

1. **Prove a dedicated "step bridge" lemma** that packs the combined facts
   (`body(output,ks) k ≡ spec step at k`) into a single named postcondition
   over the extractor's inline lambda shape.  The aux lemma then just
   chains: `lemma_fold_range_step ; step_bridge ; lemma_squeeze_blocks_unfold ; IH`,
   each as a separate sub-query.  Prior attempt produced 258 cascading
   Error 19 subtyping failures because the lambda types were
   `Prims.int` vs `range_t USIZE` — need to match the extractor output
   byte-for-byte and add `typ_of_tc` annotations.
2. **Wait for the F* fold_range closure support** — the
   `feedback_fold_range_closure_equality` memory notes this is a
   fundamental SMT limitation.  Proved pattern used in `absorb_rec`
   is the current workaround.

### Assumes 5–6 (Arm64 per-lane absorb2 / squeeze2)

### Assumes 5–6 (Arm64 per-lane absorb2 / squeeze2)

Symmetric with Portable's `lemma_absorb_portable` / `lemma_squeeze_portable`. Strategy:
1. Mirror `lemma_absorb_portable_aux` at N=2, parameterised over lane `l ∈ {0,1}`. Uses closed Arm64 `sc_load_block` / `sc_load_last` records + the `lemma_arm64_lane_eq_get_lane_u64` SMTPat (already committed) for extract_lane indexing.
2. Mirror the squeeze induction (same as admits 2–4) at N=2 per lane, using the closed Arm64 `sc_store_block` record.

Expected: reuse the Portable proof scaffolding once admit 2 is solved — the only difference is `v_N = mk_usize 2` and the lane parameter.

## Verification commands

```bash
# Clean rebuild (remove only specific .checked files — DO NOT wipe the cache):
rm -f /Users/karthik/libcrux-proofs-cleanup/.fstar-cache/checked/EquivImplSpec.Sponge.*.fst.checked
cd /Users/karthik/libcrux-proofs-cleanup/crates/algorithms/sha3/proofs/fstar/equivalence
make

# Status check (should list exactly the 5 load-bearing + 3 upstream):
rg -n "^(assume val|.* admit \(\))" *.fst Proof_Utils.Lemmas.fst

# Re-extract F* from Rust (only if the Rust side changes):
bash /Users/karthik/libcrux-proofs-cleanup/crates/algorithms/sha3/hax.sh extract
```

## Key context / pitfalls

- **Z3 `z3rlimit` ≤ 300** — do not raise higher.
- **Never clear the F* cache** — delete only specific `.checked` files.
- **Rust changes require re-extraction** via `hax.sh extract`.
- **`cargo fmt` the touched Rust crates** before committing.
- **Heavy F* proofs** — factor into separate modules (done for `SqueezeAPI`) to control Z3 context.
- **`lemma_fold_range_step`** requires syntactic match with the extractor's inline lambdas — that's what killed prior attempts at assumes 2–3.
- **The fold_range closure-equality axioms are gone** — earlier versions of the proof had 6 ineliminable closure-equality assumes; these were all removed by the `absorb_rec` rewrite. Don't re-introduce them.

## Related project memories

See `~/.claude/projects/-Users-karthik-libcrux-proofs-cleanup/memory/`:
- `project_sha3_proof_status.md`
- `feedback_fstar_proof_patterns.md`
- `feedback_fstar_factor_modules.md`
- `feedback_fold_range_closure_equality.md`
- `feedback_no_cache_clear.md`
- `feedback_ask_before_killing.md`
