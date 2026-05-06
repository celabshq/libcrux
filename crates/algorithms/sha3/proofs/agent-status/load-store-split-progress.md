# load-store module split — progress

## T+0 — start

- Worktree: `/Users/karthik/libcrux-sha3-load-store-split` on `load-store-module-split`, base `a2a3f5f71`.
- Read sprint prompt + arm64.rs (959 lines) + avx2.rs (641 lines).
- Confirmed `generic_keccak.rs` + `generic_keccak/` directory pattern is already in use.

## T+30 — first split (Rust mod-split, parent kept impls)

- Created `simd/arm64/{load.rs, store.rs}` with the obvious `load_*` / `store_*` move; parent `arm64.rs` kept math wrappers + trait impls.
- Same for `simd/avx2`.
- `bash hax.sh extract` produces `Libcrux_sha3.Simd.Arm64.{Load,Store}.fst` plus a thin `Libcrux_sha3.Simd.Arm64.fst` — but ALL bodies end up in a `Libcrux_sha3.Simd.Arm64.Bundle.fst` (2314 lines), with each per-module `.fst` being a thin `include … {sym as sym}` view onto the bundle.
- Bundle is generated whenever the parent Rust module mixes file-submodules with its OWN top-level items (impl blocks, `pub use load::…`).
- `make check/Libcrux_sha3.Simd.Arm64.Load.fst` still verifies the **full** Bundle, so the load_block query 301 cliff still triggers in `Bundle.load_block`. **The split alone failed to shrink the SMT context.**

## T+60 — second split (parent module emptied, impls moved into submodules)

Restructure to **completely empty** parent `arm64.rs`/`avx2.rs`:

- New `simd/arm64/wrappers.rs` (`Libcrux_sha3.Simd.Arm64.Wrappers`) — math wrappers + `KeccakItem<2>` impl + `uint64x2_t`.
- `simd/arm64/load.rs` — `load_*` + `Absorb<2>` impl.
- `simd/arm64/store.rs` — `store_*` + `Squeeze2` impl.
- `simd/arm64.rs` — pure `pub(crate) mod …;` shim with one `module_anchor` ghost lemma + a `hax_lib::fstar::after` block that injects `include` directives so the umbrella re-exports everything (needed because (frozen) `EquivImplSpec.{Keccakf,Sponge}.Arm64.*` open `Libcrux_sha3.Simd.Arm64`).
- AVX2: mirror.

After re-extraction: NO Bundle files. Each module is independent.

## Per-module verification (clean cache, no stale hints)

| Module | Lines | First-run time | Status |
|---|---|---|---|
| `Libcrux_sha3.Simd.Arm64.Wrappers.fst` | 194 | 9 s | OK |
| `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` | 162 | 7 s | OK |
| `Libcrux_sha3.Simd.Arm64.Store.fst` | 1386 | 2 m 20 s | OK |
| `Libcrux_sha3.Simd.Arm64.Load.fst` | 755 | **fails** (~3-5 m, query 301 cliff) | FAIL — pre-existing |
| `Libcrux_sha3.Simd.Avx2.Wrappers.fst` | 223 | 28 s | OK |
| `Libcrux_sha3.Simd.Avx2.Store.fst` | 778 | 12 s | OK (entry-`admit()` preserved) |
| `Libcrux_sha3.Simd.Avx2.Load.fst` | 1648 | 11 m 47 s | OK |
| `EquivImplSpec.Keccakf.Avx2.fst` | … | 4 m 40 s | OK |
| `EquivImplSpec.Keccakf.Arm64.fst` | … | n/a | FAILS via Load.fst cliff |
| `EquivImplSpec.Sponge.Avx2.fst` | … | n/a | FAILS via Load.fst cliff |

## load_block cliff status — DID NOT close

The Arm64 `load_block` query 301 cliff persists after the split:

```
(Libcrux_sha3.Simd.Arm64.Load.fst(355,27-505,7))
Query-stats (Libcrux_sha3.Simd.Arm64.Load.load_block, 301)
  failed {reason-unknown=unknown because canceled} in 169600 ms with rlimit 800
```

Same query, same time, same cancellation behavior as before the split. This refines the 2026-05-06 investigation finding — the cliff is **intrinsic to `load_block`'s body shape**, not a side-effect of the surrounding module's open-list / SMT context size.

### Next-attempt path (out of scope for this sprint)

Mirror what fixed `store_block`: factor `load_block` into

- `load_block_full(state, blocks, offset, q)` — the for-loop body alone (`for i in 0..RATE/16`, calls `load_u64x2x2`).
- `load_block_tail(state, blocks, offset, remaining)` — the `if remaining > 0` partial-load case.
- `load_block` — bridges Euclidean equation `RATE = 16*q + remaining` and dispatches.

The query 301 cliff is the loop-body invariant-preservation sub-query; isolating it inside `load_block_full` (with strong byte-level ensures and a smaller body) should suppress the array_from_fn refinement / k!61 instance cascade the same way the store-side decomposition did.

## Cross-cutting cleanups

- `crates/algorithms/sha3/hax.sh`: `_super_i0` patches retargeted from `Simd.Arm64.fst`/`Simd.Avx2.fst` to `Simd.Arm64.Store.fst`/`Simd.Avx2.Store.fst` (where the `Squeeze2` / `Squeeze4` impls now live). Documented inline.
- No Makefile edit needed — default ROOTS picks up new files.
- No `crates/algorithms/sha3/src/lib.rs` change.

## Branch

`load-store-module-split` on `/Users/karthik/libcrux-sha3-load-store-split`.
