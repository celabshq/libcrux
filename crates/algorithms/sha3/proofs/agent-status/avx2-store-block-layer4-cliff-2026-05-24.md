# AVX2 store_block Layer 4 cliff — 2026-05-24 (handed back 2026-05-25)

Worktree: `/Users/karthik/libcrux-avx2-store` (branch `avx2-store-block-proof`).
Tip: `7b9139ea7` (Layer 3b verified clean). Touched fn for Layer 4:
`store_block_full_avx2` (`crates/algorithms/sha3/src/simd/avx2/store.rs`).

## Where we are

Layers 1–3b verify and are committed:

| Commit | Layer | Function | Status |
|---|---|---|---|
| `9906145c8` | 1 | `StoreBlockHelpers` (manual F* module) + strengthened `mm256_storeu_si256_u8` per-byte axiom | rlimit 80, ~1s |
| `b39935f40` | 2 | `store_u64x4x4` (per-iteration 4-output wrapper, 4× permute2x128 + 2× unpack chain) | rlimit 400 / split_queries always, max sub-query rlimit ~220 |
| `a7f963d7c` | 3a | `store_chunk8x4` (inner 8-byte chunk wrapper) | rlimit 400 / split_queries always, max sub-query rlimit ~8 |
| `7b9139ea7` | 3b | `store_tail_ragged_avx2` (rem8<8 partial) | rlimit 400 / split_queries always |

## Where Layer 4 cliffs

`store_block_full_avx2` — the outer-loop half. Tried three structurally distinct shapes; all hit the same wall.

### Shape A: invariant + per-iteration helper in `s_m` form (Arm64 mirror)

`store_u64x4x4(out0..out3, *get_ij(s,i0,j0), .., start, i)`; its ensures uses the 4 raw `s_m` args with a 4-way `(j-start)/8 == 4*i+k'` discriminator. The outer loop's invariant has 4 forall clauses over `s[(j-start)/8]`. Bridge done in the loop body with `lemma_div_mod (4*v i + k) 5` calls. Verifies cleanly for Arm64's 2-output / 2-discriminator analog.

| Options | Outcome |
|---|---|
| `--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '* -array_from_fn -rem_euclid -rem_euclid'` | Sub-query 1 canceled at 140s / rlimit 800.000. Cascading "Subtyping check failed" errors at the signature level, but root is body-proof timeout. |
| Same without filter | Same canceled at 140s. |
| `--z3rlimit 400 --split_queries always --z3refresh ...` | Inner-call subtype check on `store_u64x4x4`'s `start + 32*(i+1) <= out0.len()` precondition canceled at 70s / rlimit 400.000. |
| With explicit `assert (v start + 32 * (v i + 1) <= ...)` before the call | New cliff at `fold_range` elaboration: `Expected b: int_t USIZE { range (v i * v b) USIZE }` (the loop invariant's `i * 32` arithmetic). |

### Shape B: refactor `store_u64x4x4` to take `&[Vec256; 25]` + `i`

`store_u64x4x4` internally does the get_ij linearisation; its ensures is directly in `s[(j-start)/8]` form. The outer loop's invariant uses the same form, so the loop step is trivially the post (no 4-way discriminator bridging needed in the loop body). The chaining moves INSIDE store_u64x4x4: bridge_out_m gives `v_m` form, then SMTPats relate `v_m` to `s_m`, then Euclidean asserts relate `s_m` to `s[4*i+m]`, then arithmetic relates `(j-a_pos)/8 + 4*i` to `(j-start)/8` and `(j-a_pos)%8` to `(j-start)%8`.

| Options | Outcome |
|---|---|
| `--z3rlimit 400 --split_queries always` | store_u64x4x4 query 420 canceled at 72s / rlimit 400.000. Same cliff shape moved one layer deeper. |
| `--z3rlimit 800 --split_queries no --z3refresh` | store_u64x4x4 query 1 canceled at 166s / rlimit 800.000. Cascading signature subtyping errors. |

### QI profile

Ran `--z3cliopt 'smt.qi.profile=true' --z3cliopt 'smt.qi.profile_freq=20000'` with rlimit 800 / split_queries no for >4 hours wall; Z3 never hit the 20000-instantiation threshold to dump a profile. That suggests the saturation is **not** classic quantifier explosion but rather **arithmetic/range subtyping with combinatorial case-split** — Z3 chews through 4 × 4 = 16 lane/discriminator combinations chained through Euclidean div_mod + permute/unpack SMTPats + per-byte `to_le_bytes` indexing. Each individual SMTPat fires modestly, but the cross-product is wide.

## Comparison vs Arm64

Arm64 `store_block_full` verifies at the same `--z3rlimit 800 --split_queries no --z3refresh --using_facts_from ...` options because:
- 2 outputs × 2-way `(j-start)/8 == 2*i + k'` discriminator (4 cross-product cases) vs AVX2's 4 × 4 = 16.
- Per-byte axiom on `_vst1q_bytes_u64` (Arm64 intrinsic) doesn't need the SMTPat-fan-out trick.

The structural difference makes the AVX2 case ~4× heavier on the SMT side at the loop-step preservation VC.

## Three candidate paths forward (not yet tried)

1. **Per-output sub-loops**. Run the outer loop four times (one per `out_m`). Each loop's invariant has 1 forall clause; the step is 4× simpler. 4× more iterations total but each step trivially absorbs the per-iteration `store_u64x4x4` post slice on a single output. Most likely to land; ~3× the F* code volume.
2. **External lane-chain helper lemma**. Add a lemma in `StoreBlockHelpers.fst` that takes `(s: Seq.seq t_Vec256, s0..s3: t_Vec256, i: nat, j: nat{..}, lane_m: nat{<4})` and packages the entire chain `out_m[j] == byte (j-start)%8 of to_le_bytes(get_lane_u64 s[(j-start)/8] lane_m)` from the input facts `v_m == unpack/permute(s0..s3)` + `s[4i+k] == s_k`. The bridge body in `store_u64x4x4` becomes a single call per `m`, with all the heavy chaining isolated in the helper lemma where Z3 sees one chain at a time.
3. **Drop the post's `s[(j-start)/8]` form in store_u64x4x4 and re-emit it once at the OUTER LOOP exit**. Keep Shape A's per-iteration helper (in `s_m` form, verified clean at rlimit 220). Have the loop invariant ALSO use the 4-way discriminator form `if (j-start)/8 == 4*i' + k'` (with witness `i' < i`). After the loop, bridge the discriminator form to `s[k]` form via a single Classical.forall_intro pass over `j`. Risk: the invariant form might still trip up the loop step due to the existential-style ladder.

## Recommended next-session approach

Try (2) first — minimal source churn, isolates the cliff into a single helper lemma whose own proof can be driven with explicit case splits (per-`k` branches that Z3 handles in isolation). If (2) doesn't land within a session, fall back to (1).

## Per-function debug-time budget

~2 sessions on Layer 4 across approaches A and B. Per memory rule `feedback_proof_debug_budget` (30–60 min/fn cap), handing back here.
