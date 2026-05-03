# Stream 2 — ind_cpa Pattern-1 eq_intro cluster

Worktree: /Users/karthik/libcrux-stream2-ind_cpa
Branch: agent-mlkem/phase-f-stream2-ind_cpa
Start: 07:55 (2026-05-03)

## Status log
- 07:55 — extraction baseline ran; reading ind_cpa.rs + spec module heads.
- target order: deserialize_vector → deserialize_then_decompress_u → serialize_vector → compress_then_serialize_u
- 08:03 — **LIGHTHOUSE FLIPPED.** deserialize_vector lax→panic_free. Pattern: per-index Lemma w/ Classical.move_requires + try_into-array-vs-slice eq_intro bridge, then post eq_intro. Query 2 used 1.273 rlimit / 800. Module verified 16.87s.
- 08:30 — deserialize_then_decompress_u attempt 1: same pattern fails. Query 2 timeout (canceled at rlimit 800.000 in 142s). Diagnosed root cause: ntt_vector_u in src/ntt.rs has its functional ensures COMMENTED OUT (line `// #[hax_lib::ensures(|_| fstar!(... ntt poly_to_spec re ...))]`) — only carries `is_bounded_poly(3328, future(re))`. Without the ntt functional ensure, the loop invariant maintenance step cannot derive `poly_to_spec(u_as_ntt[i]) == ntt(decompress(byte_decode_dyn(slice)) du)` after `ntt_vector_u(&mut u_as_ntt[i])`. Reverted to lax with FOLLOW-UP. Out of stream scope — needs ntt_vector_u ensures strengthening.
- 08:35 — Z3 still grinding (29 min wall, 28 min CPU on z3 79289), output buffered since Q30. Waiting for build to finish before re-extracting reverted source.
- 08:48 — Z3 79289 reached 41 min wall / 41 min CPU, still stuck. Killed (own session worktree path verified). Re-extracted reverted source.
- 08:52 — Final clean rebuild green. deserialize_vector lax→panic_free committed.  Family A (serialize_vector, compress_then_serialize_u) NOT ATTEMPTED — structural blocker is spec module's `Hax.Folds.fold_range`-based `serialize_secret_key`/`compress_then_serialize_u` having `True` postcondition; needs unfold lemma in spec module or consumer file. Out of per-fn budget.
- Final: 1/4 flipped (lighthouse). 3/4 documented as out-of-scope (structural blockers in upstream specs / src/ntt.rs).
