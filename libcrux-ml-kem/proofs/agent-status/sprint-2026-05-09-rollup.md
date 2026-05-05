# Sprint 2026-05-09 — Portable+Avx2+impl-layer lax cleanup — rollup

**Branch:** `libcrux-ml-kem-proofs`
**Worktree:** `/Users/karthik/libcrux-trait-opacify`
**Tip on entry:** `1c94eae53`
**Tip on exit:**  `2b0f159d1`

## Outcome — 5 of 22 lax sites flipped

| Module             | Baseline | Final | Delta |
|--------------------|----------|-------|-------|
| Generic/serialize  | 4        | 0     | -4    |
| Generic/invert_ntt | 2        | 1     | -1    |
| Generic/ntt        | 1        | 1     | 0     |
| Portable/vector    | 4        | 4     | 0     |
| Avx2/serialize     | 5        | 5     | 0     |
| Avx2/ntt           | 2        | 2     | 0     |
| Avx2/vector        | 4        | 4     | 0     |
| **Generic total**  | **66**   | **61**| **-5**|

Out-of-scope (sampling-while, Neon, Incremental.*) unchanged.

## What landed (in tip order)

### Stage 1 — `serialize.rs` _5/_11 wrappers (commit `05dd2bd05`)

Eliminated 4 lax sites in `src/serialize.rs`:

- `compress_then_serialize_11` — mirrored `compress_then_serialize_10`'s
  panic_free pattern (loop_invariant + `assert_norm (pow2 11 == 2048)` +
  `lemma_bounded_i16_array_intro 0 3328`).
- `compress_then_serialize_5` — mirrored `compress_then_serialize_4`.
- `deserialize_then_decompress_11` — mirrored `deserialize_then_decompress_10`
  (`assert_norm (pow2 11 - 1 == 2047)` + `lemma_bounded_i16_array_intro 0 2047`).
- `deserialize_then_decompress_5` — mirrored `deserialize_then_decompress_4`,
  including options `--z3rlimit 400 --ext context_pruning --split_queries always`,
  `is_bounded_poly_higher` lift after ZERO, and
  `lemma_decompress_post_to_is_bounded_vector` to preserve the loop invariant.

Acceptance: `make check/Libcrux_ml_kem.Serialize.fst rc=0`.

### Stage 3 partial — `invert_ntt_montgomery` (commit `2b0f159d1`)

Flipped `--admit_smt_queries true` → `verification_status(panic_free)` with
`--z3rlimit 200 --ext context_pruning --split_queries always`.  The body is
a sequence of `invert_ntt_at_layer_{1,2,3,4_plus×4}` calls + four
`is_bounded_poly_higher` widenings; F* discharges panic-freedom +
preconditions in 245s wall.  Functional ensures
(`to_spec_poly_mont eq ntt_inverse_butterflies`) admitted via `panic_free`'s
ensures admit.

Acceptance: `make check/Libcrux_ml_kem.Invert_ntt.fst rc=0`,
`make check/Libcrux_ml_kem.Ntt.fst rc=0` (no regression).

## What deferred (and why)

### Stage 2 — Avx2 vector serialize/ntt + Portable NTT step (9 sites)

- `vector/avx2/serialize.rs` `serialize_1` / `serialize_5` / `deserialize_5`
  / `serialize_11` / `deserialize_11` — by-design lax per prior commit
  `107c76641` ("SIMD body treated as a signature-level axiom; the BitVec
  post is now visible to callers").  Flipping requires either
  rewriting the body as pure SIMD (mirror `serialize_10` / `serialize_12`'s
  ~80-line BitVec-tactic body) or wiring a BitVec ↔ i16-array bridge for
  the PortableVector delegation.  Both are 30-90 min per fn — exceeds
  prompt's "small ensures strengthening" estimate.
- `vector/avx2/ntt.rs` `inv_ntt_layer_1_step` and `ntt_multiply` — body
  proofs through SIMD bound chains; 60-90 min per fn per the prompt.
- `vector/portable.rs` `op_ntt_layer_1_step` and `op_inv_ntt_layer_1_step` —
  the inline comment + `feedback_layer2_branch_post_z3_unlock` document
  the required 4-way per-branch helper refactor (60-90 min each).

### Stage 3 deferred — `invert_ntt_at_layer_4_plus` and `ntt_vector_u`

- `invert_ntt_at_layer_4_plus` (`invert_ntt.rs:528`): inner-loop invariant
  maintenance VC saturates rlimit 400 with `--split_queries always` — both
  query 188 and query 204 canceled at the cap (157s and 200s respectively
  before timeout).  Per `feedback_rlimit_cap_800`, bumping rlimit hides
  structural problems; a per-branch helper refactor (mirroring portable
  `op_ntt_layer_2_step`'s working pattern) is the right next step.
- `ntt_vector_u` (`ntt.rs:559`): `panic_free` flip fails on the precondition
  checks of sequential `ntt_at_layer_{2,1}` calls (`6*3328` / `7*3328`
  bound args) at lines 673, 679 in the extracted file.  The proven sibling
  `ntt_binomially_sampled_ring_element` verifies the identical chain *without*
  `verification_status(panic_free)` — i.e., it uses default verify-ensures
  with bound-only ensures.  Switching `ntt_vector_u` to that pattern
  requires dropping the functional Hacspec ensures conjunct (deferred to
  the same sprint that closes the layer_4_plus body discharge — they
  share the `Hacspec_ml_kem.Commute.Chunk` bridge dependency).

### Stage 4 — `op_*` glue + `to_bytes`/`from_bytes` (6 sites)

All 6 sites blocked on architectural concerns outside the proof-side fix
surface:

- `vector/avx2.rs:65` (`to_bytes`), `vector/avx2.rs:1140`
  (`Operations::from_bytes` impl), `vector/avx2.rs:1153`
  (`Operations::to_bytes` impl), `vector/portable.rs:974`
  (`Operations::from_bytes` impl), `vector/portable.rs:994`
  (`Operations::to_bytes` impl) — all blocked on the hax-lib `&mut` slice
  modeling issue around `update_at_range`/`classify_mut_slice`.  Per
  prompt: "this is hax-lib level, not a proof-side fix".  **Surfaced to
  user as FOLLOW-UP.**
- `vector/avx2.rs:994` (`op_serialize_1`) — calls `serialize::serialize_1`
  which is in the Stage 2 deferred set; will follow whenever the SIMD
  primitive proof story is closed.

## Full-tree state

`make` from `proofs/fstar/extraction` rc=2 with 1 error:

```
Error 47 at Libcrux_ml_kem.Hash_functions.fst(7,4-7,16)
```

Pre-existing — present in the session-start `verification_result.txt`
baseline (alongside `Vector.Rej_sample_table.fst` and
`Ind_cca.Instantiations.Neon{,.Unpacked}.fsti` Error 72s, all also
unchanged by this sprint).

## Suggested next sprint

1. Close `invert_ntt_at_layer_4_plus` body by mirroring
   `op_ntt_layer_2_step`'s 4-way per-branch helper structure (file
   `Hacspec_ml_kem.Commute.Chunk.fst` lemmas) — 60-90 min.
2. Drop functional Hacspec ensures from `ntt_vector_u` and use the
   `ntt_binomially_sampled_ring_element` pattern (no
   `verification_status(panic_free)`, bound-only ensures, `--z3rlimit 200`)
   — 30-45 min once layer_4_plus closes upstream.
3. The 9 Stage 2 SIMD-axiom sites remain a separate, larger workstream
   (BitVec lemma surface + per-fn 30-90 min × 5 + per-fn 60-90 min × 4 = 6h+).
4. Stage 4's hax-lib `&mut` slice modeling needs a hax-lib-side fix; not
   a proof-side sprint.

## Commit hygiene check

- Commit `05dd2bd05` — `serialize.rs` only, 1 file changed.
- Commit `2b0f159d1` — `invert_ntt.rs` only, 1 file changed.
- No stale `--admit_smt_queries true` / `panic_free` left from spike
  attempts.  `git diff` against tip = clean modulo verification_status.md
  and sprint agent-status notes.
