# agent op_serialize_1 status

## Task
Remove last AVX2 lax fn `op_serialize_1` via trait-bridge lemmas.

## 2026-06-03 — t0
- Sub-task: Rust edits done. Added `op_serialize_1_pre_bridge` +
  `op_serialize_1_post_bridge` (fully proven, mirroring N=4/5/10/11/12
  siblings which are NOT admitted — framework comment at ~785 was stale,
  now corrected). Rewired wrapper body, dropped `verification_status(lax)`.
  Bridges placed before `op_serialize_4_pre_bridge`.
- Key contract facts: serialize_1_ requires `forall i. i%16>=1 ==> v i==0`;
  ensures `forall i. bit_vec_of_int_t_array r 8 i == v (i*16)`. Output array
  is `t_Array u8 (mk_usize 2)`, post-bridge forall over `i<16`.
- Blocker: none yet.
- ETA: cargo check -> extract (~2.5m) -> make module -> full gate (~13m).

## 2026-06-03 — t+~10m: MODULE VERIFIED
- cargo check (simd128,incremental): pass (only pre-existing unused-mut warns).
- hax extract: clean, bridges + wrapper extracted correctly.
- make check/Libcrux_ml_kem.Vector.Avx2.fst: EXIT 0, 81.6s, "All verification
  conditions discharged successfully", .checked written, 485 Query-stats lines.
- op_serialize_1_pre_bridge succeeded (776ms, rlimit 8.99), op_serialize_1_post_bridge
  succeeded (1314ms, rlimit 12.98), wrapper op_serialize_1_ succeeded (rlimit 1.63).
- Zero real (without-hint) failures. The 2x failed-(with-hint) on op_ntt_layer_3_step /
  op_inv_ntt_layer_3_step are pre-existing benign stale-hint replay (75 succeeded each,
  module discharges all VCs); NOT touched by this change.
- Sub-task: running full gate `make -k -j2 all`.
- ETA: ~13m.

## 2026-06-03 — FINAL: DONE
- Full gate `make -k -j2 all`: EXIT 0, 0 failures. Closure already cached
  except Vector.Portable.Vector_type (touched by extraction patch) which
  re-verified clean. My only changed module Vector.Avx2 verified from scratch
  earlier (no .checked existed, real 81.6s build, 485 Query-stats).
- Status doc regenerated: Avx2 total Lax 1 -> 0; op_serialize_1 reclassified
  Lax -> Hacspec. Whole-crate Lax 88 -> 87. AVX2 backend now 0 lax.
- Files changed (tracked): libcrux-ml-kem/src/vector/avx2.rs,
  libcrux-ml-kem/proofs/ml_kem_verification_status.md.
- Bridges proven fully (mirrors N=4/5/10/11/12 siblings; none admitted).
- Committing now.
