# loop-invariant-opacify — agent status (T+0..T+nn)

Branch: `loop-invariant-opacify` (worktree: `/Users/karthik/libcrux-loop-inv-opacify`)
Base: `95ca5782c` = AVX2 structural split + 3 wrappers + helpers (load_block cliff already
closed by createi opacity + load_lane_u64_lane_extensionality on the original branch).

## T+0 — Stage A kickoff (Arm64 load_block trivial fix)

Read AVX2 load.rs end-to-end: opacity markers at `load_lane_u64`, `load_u64x4`, `load_u64x4x4`
plus the `load_lane_u64_lane_extensionality` SMTPat lemma injected via fstar::after.
Plan: mirror exactly. Arm64 is N=2; lane bound becomes `lane < 2`, blocks array `[t_Slice u8; 2]`.

ETA: opacity ports landed and re-extracted in 30 min; clean Arm64.Load make in ~3 min.
