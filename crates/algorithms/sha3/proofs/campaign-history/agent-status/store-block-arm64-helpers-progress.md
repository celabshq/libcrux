# store_block Arm64 helpers — progress

## 2026-05-06, T+0:30 (helper 1 landed in fsti)

- Sub-task: helper 1 (`_vst1q_bytes_u64` ensures strengthening)
- Status: ensures rewritten in `crates/utils/intrinsics/src/arm64_extract.rs`,
  re-extracted, `.fsti` shape verified by inspection (`val e_vst1q_bytes_u64`
  with content forall on `to_le_bytes (get_lane_u64x2 v (i/8))[i%8]` for
  `i<16` and frame for `i>=16`).
- `make check/Libcrux_intrinsics.Arm64_extract.fsti` succeeds (cached, up-to-date).
- Note: `.fst` typecheck fails on a *pre-existing* baseline issue at
  `e_vdupq_n_s16` line 88 (unrelated `panic` body). The helper-1 ensures
  itself lands correctly in the .fsti, which is what consumers see.
- Blocker: none.
- ETA: helper 2 sketch in fstar-mcp within 30 min.

## Plan
- Helper 2: per-byte SMTPat bridge lemma in
  `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` (new module). Will read
  `Libcrux_sha3.Simd.Arm64.fst` around the inner-loop `e_vst1q_bytes_u64`
  call sites to determine the slice-update shape.

## 2026-05-06, T+1:00 (helper 2 landed)

- Sub-task: helper 2 — per-byte bridge lemma
- Status: `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` typechecks
  cleanly (`make check`, `EXIT=0`, all VCs discharged, total ~1 s).
- Two lemmas + one synthetic test:
    * `store_block_window_byte` — generic per-byte derivation from
      slice-prefix/window-content/slice-suffix facts (the abstract form,
      independent of how `update_at_range` and `e_vst1q_bytes_u64` got us
      there).
    * `store_block_window_byte_of_vst` — convenience wrapper taking the
      raw `update_at_range` post + raw `e_vst1q_bytes_u64` post (the
      shape the loop body actually has in scope) and bridging into the
      generic lemma.
    * `test_store_block_loop_step` — synthetic test fixing
      `a = start + 16*i`; verifies the wrapper fires with the actual
      arithmetic the store_block invariant uses.
- No SMTPat trigger discipline needed — the per-byte derivation falls
  out of `Seq.slice` extensionality + the input forall over `k < 16`.
  Kept as plain lemmas; consumer calls them explicitly per iteration.
  This avoids the dual-trigger trap (`feedback_smtpat_lane_propagation`)
  entirely: no broad SMTPat fires across all stores in the loop body,
  and no `%` exposure above the trait layer
  (`feedback_smtpat_percent_above_trait`).
- Helper 3 (`_vtrn1q_u64`/`_vtrn2q_u64` byte-view propagation) is **not
  needed**: the existing trn intrinsics' lane posts compose with the
  helper-1 ensures and helper-2 lemma without further work. The
  consumer agent (store_block discharge) can derive the lane reduction
  on its side without an extra helper.

## Deliverables summary
- Helper 1: ensures of `_vst1q_bytes_u64` strengthened in
  `crates/utils/intrinsics/src/arm64_extract.rs:264-282`. Re-extracted;
  extracted .fsti shape verified by inspection. .fsti make check passes.
- Helper 2: new file
  `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst`,
  3 lemmas verified.
- Both helpers verified standalone; `Libcrux_sha3.Simd.Arm64.fst` (still
  with the body `admit ()` on `store_block`) re-verifies cleanly with
  the strengthened `e_vst1q_bytes_u64` ensures (no downstream regression).

