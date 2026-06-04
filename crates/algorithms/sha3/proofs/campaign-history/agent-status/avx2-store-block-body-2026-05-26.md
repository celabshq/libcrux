# AVX2 `store_block` body — session 2026-05-26

Worktree: `/Users/karthik/libcrux-sha3-proofs`, branch
`sha3-proofs-focused`. Started at HEAD `7692f9939`. Merged
`avx2-store-block-proof` (5 commits: Layers 1-3b + cliff doc) at
merge commit `9a9f15974`.

## Net delta this session

| Step | Status |
|---|---|
| Merge `avx2-store-block-proof` (Layers 1-3b + cliff doc) into `sha3-proofs-focused` | Done — merge commit `9a9f15974`, clean (no conflicts) |
| Re-extract `Libcrux_intrinsics.Avx2_extract.fsti` (gitignored, was stale post-merge) | Done via `cargo hax` in `crates/utils/intrinsics` |
| Write `lemma_lane_chain_to_s` in `StoreBlockHelpers.fst` (Option 2 from cliff doc) | **Verified clean** (build `2b5d5b42`, 15.3 s wall, rlimit 200) |
| Add `store_block_full_avx2` to `src/simd/avx2/store.rs` with admit body | Done; canonical `bash hax.sh extract` ran patches; new fn is at `Avx2.Store.fst:1410` |
| Validate `store_block_full_avx2` skeleton via proxy | **Blocked** — fstar_build on 2394-line `Avx2.Store.fst` times out at MCP 60s; retries spawned orphan make/fstar/z3 racing on the same `.checked` file |

Uncommitted in working tree:
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` (+76 lines — helper lemma)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.Store.fst` (+228 lines — hax-regenerated from updated Rust source with new fn)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Simd128.fst` (~40 lines cosmetic hax regen — no semantic change)
- `crates/algorithms/sha3/src/simd/avx2/store.rs` (+85 lines — `store_block_full_avx2` signature + admit body)
- `crates/algorithms/sha3/proofs/agent-status/avx2-store-block-body-2026-05-26.md` (this file)

## `lemma_lane_chain_to_s` — the verified cliff-breaker

`StoreBlockHelpers.fst:213-275`. Signature takes `(s, s0..s3, start, i, lane_m, j, out_byte)` and from
`(start + 32*i <= j < start + 32*(i+1))` + `s_k == Seq.index s (4*i + k)` + a 4-branch hypothesis
in `s_k` form, concludes the unified `out_byte == byte (j-start)%8 of to_le_bytes (get_lane_u64 s[(j-start)/8] lane_m)`.
Body uses Euclidean to derive `q = (j-start)/8 ∈ {4i, 4i+1, 4i+2, 4i+3}`, then case-splits with one
linearisation step per branch. ~22-line body, verifies at rlimit 200.

This is the Option-2 helper from `avx2-store-block-layer4-cliff-2026-05-24.md` — designed to isolate
the 4×4 cross-product case-split that saturated the prior Layer 4 attempts at rlimit 400/800.

## `store_block_full_avx2` — landed, unvalidated

`src/simd/avx2/store.rs:550-619`. Signature mirrors Arm64's `store_block_full` at width 4:
- requires: 4-way length-eq, `q <= 6`, `start + 32*q <= out0.len()`
- ensures: 4 per-lane forall on `out_m[j] == get_lane_u64(s[(j-start)/8], m).to_le_bytes()[(j-start)%8]` for j in `[start, start+32q)`
- hax opts: `--z3rlimit 400 --split_queries always --z3refresh --using_facts_from '* -array_from_fn -rem_euclid -rem_euclid'`
- body: simple `for i in 0..q { let (i0,j0,..)=...; let s0..s3=*get_ij(s,_,_); store_u64x4x4(out0..3, s0..s3, start, i); }` with `hax_lib::fstar!("admit()")` first line

**Body proof still TODO** — the structural addition is in place; the actual loop_invariant + bridge
via `lemma_lane_chain_to_s` is the next session's work.

## Proxy validation blocker (operational, not proof-theoretic)

`fstar_build` on `Libcrux_sha3.Simd.Avx2.Store.fst` (2394 lines after hax regen) consistently exceeds
the proxy MCP tool's 60s response timeout. Worse: on MCP-side timeout, the proxy does not tear down
the spawned `make`/`fstar.exe`/`z3` subprocess tree, so a retry call spawns a **second** parallel set
racing on the same `.checked` file output. Observed orphan pairs killed this session:
PIDs 83129, 87218, 85169, 85170, 89259, 89260, 34900, 34902, 37682, 37683.

The `fstar_typecheck` lax-to-position pattern (skill §2 Rule 5b) is the right escape but requires
passing the full 95KB buffer as the `code` param, which is awkward across multi-page Read.

Operational mitigations the user may want to apply:
1. Restart the proxy with a longer MCP-side timeout (env or config).
2. Have the proxy tear down spawned subprocesses on MCP disconnect to prevent orphans.
3. Or: accept that heavy builds need a different validation transport (e.g. curl POST direct to
   `localhost:3002` with longer client timeout — same proxy, different transport).

## Next session — clean entry point

1. **Phase 0** (2 min): verify proxy + baseline. Now there are 3 live admits PLUS the new admit in
   `store_block_full_avx2` PLUS the existing one in `store_block` ⇒ 5 live admits total. The end-state
   (Layer 4 body done + composer body updated) drops back to 3 (or 2 if `lemma_squeeze4_avx2` also
   closes as a bonus).
2. **Validate the skeleton** via a quick lax build of `Avx2.Store.fst` to confirm the signature
   typechecks structurally. Run via whatever path doesn't time out.
3. **Fill in `store_block_full_avx2` body**: add `loop_invariant!` (4 per-lane forall in
   `s[(j-start)/8]` form) + the per-iteration bridge using `lemma_lane_chain_to_s` via
   `Classical.forall_intro`. See sketch below.
4. **Add `store_block_tail_avx2`** (composer over `store_chunk8x4` + `store_tail_ragged_avx2`).
5. **Replace `store_block`'s `admit()`** with composer body calling full + tail.
6. **Bonus**: close `lemma_squeeze4_avx2` (`assume val` at `EquivImplSpec.Sponge.Avx2.API.fst:87`)
   via the same one-liner pattern that closed `lemma_squeeze2_arm64` in commit `7692f9939`.

### Body sketch for `store_block_full_avx2`

```rust
fn store_block_full_avx2(...) {
    #[cfg(hax)] let old_out0 = out0.to_vec().as_slice();
    #[cfg(hax)] let old_out1 = out1.to_vec().as_slice();
    #[cfg(hax)] let old_out2 = out2.to_vec().as_slice();
    #[cfg(hax)] let old_out3 = out3.to_vec().as_slice();
    hax_lib::fstar!(r#"assert_norm (... == out0); ... "#);  // bridge old_out_m == out_m

    for i in 0..q {
        hax_lib::loop_invariant!(|i: usize|
            (out0.len() == old_out0.len()).to_prop()
            & /* same for out1..3 */
            & hax_lib::forall(|j: usize| if j < out0.len() {
                if j < start { out0[j] == old_out0[j] }
                else if j < start + i * 32 {
                    out0[j] == get_lane_u64(s[(j-start)/8], 0).to_le_bytes()[(j-start)%8]
                } else { out0[j] == old_out0[j] }
            } else { true })
            & /* same forall for out1 (lane 1), out2 (lane 2), out3 (lane 3) */);
        let i0 = (4 * i) / 5;
        let j0 = (4 * i) % 5;
        let i1 = (4 * i + 1) / 5;
        let j1 = (4 * i + 1) % 5;
        let i2 = (4 * i + 2) / 5;
        let j2 = (4 * i + 2) % 5;
        let i3 = (4 * i + 3) / 5;
        let j3 = (4 * i + 3) % 5;
        let s0 = *get_ij(s, i0, j0);
        let s1 = *get_ij(s, i1, j1);
        let s2 = *get_ij(s, i2, j2);
        let s3 = *get_ij(s, i3, j3);
        store_u64x4x4(out0, out1, out2, out3, s0, s1, s2, s3, start, i);
        // Bridge: store_u64x4x4's post is in 4-branch s_k form.
        // lemma_lane_chain_to_s lifts to s[(j-start)/8] form per (m, j).
        hax_lib::fstar!(r#"
            FStar.Math.Lemmas.lemma_div_mod (4 * v i) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 1) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 2) 5;
            FStar.Math.Lemmas.lemma_div_mod (4 * v i + 3) 5;
            assert (Seq.index s (4 * v i + 0) == s0);
            assert (Seq.index s (4 * v i + 1) == s1);
            assert (Seq.index s (4 * v i + 2) == s2);
            assert (Seq.index s (4 * v i + 3) == s3);
            // Per output, lift the 4-branch fact at each j in the new window
            // to s[(j-start)/8] form.
            let bridge_chain (m_lane: nat{m_lane < 4})
                             (out_m old_out_m: Seq.seq u8)
                             (j_n: nat{j_n < Seq.length out_m})
              : Lemma (... s[(j-start)/8] form ...)
              = if j_n >= v start + 32 * v i && j_n < v start + 32 * (v i + 1) then
                  Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.lemma_lane_chain_to_s
                    s s0 s1 s2 s3 (v start) (v i) m_lane j_n (Seq.index out_m j_n)
                else ()
            in
            Classical.forall_intro (bridge_chain 0 out0 old_out0);
            Classical.forall_intro (bridge_chain 1 out1 old_out1);
            Classical.forall_intro (bridge_chain 2 out2 old_out2);
            Classical.forall_intro (bridge_chain 3 out3 old_out3)
        "#);
    }
}
```

## Process / cap notes for next session

- Multiple "task tool reminder" interrupts this session — task list is current.
- Many orphan fstar/z3 processes killed across the session — audit
  `pgrep -lf 'fstar.exe|z3 -smt2'` regularly after `fstar_build` calls, especially after MCP
  timeouts.
- Avoid invoking `fstar_build` on `Avx2.Store.fst` until the proxy's timeout behavior is fixed,
  or use `fstar_typecheck` with full buffer + `kind="lax-to-position"`.
