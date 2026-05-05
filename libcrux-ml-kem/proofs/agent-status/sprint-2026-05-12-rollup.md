# Sprint 2026-05-12 — `deserialize_5` (AVX2): partial progress

**Worktree:** `/Users/karthik/libcrux-serialize-1-deserialize-5/libcrux-ml-kem`
**Base branch:** `agent-mlkem-serialize-1-deserialize-5-2026-05-11`
**Final make:** `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst` rc=0 in 2:18.

## Outcome

| Item | Before | After |
|---|---|---|
| `deserialize_5_vec(c: Vec128)` (new helper, opaque) | n/a | **fully verified** |
| `deserialize_5(bytes: &[u8])` (outer fn) | `panic_free` | `panic_free` (with helper inside) |
| `mm256_mullo_epi16_specialized4` spec correctness | reversed (lane-0=0, lane-7=11) | **corrected** to lane-0=11, lane-7=0 |

Net: outer status unchanged, but the verification surface is dramatically reduced.
The hard part — proving the SIMD chain over a Vec128 input — is now closed.
What's left is a pure-arithmetic byte-mapping bridge from `mm_set_epi8` to bytes.

## What worked

1. **Bug fix in `BitVec.Intrinsics.fsti`**: the `mm256_mullo_epi16_specialized4`
   spec had the per-lane shift cycle reversed.  Convention A
   (`mm256_set_epi16` first arg = lane 15) means the multiplier source
   `(1<<0, 1<<5, 1<<2, 1<<7, 1<<4, 1<<9, 1<<6, 1<<11, …)` puts shift=0 at
   lane 15 and shift=11 at lane 0.  Old spec computed `shift = (k%2)*5 +
   (k/2)*2` (i.e. `[0;5;2;7;4;9;6;11]` for k=0..7), which is the reverse
   of the actual.  Corrected to `shift = 11 - ((k%2)*5 + (k/2)*2)`.

2. **Vec128-input helper factor (the key insight from the user's nudge)**:
   the unique blocker for `deserialize_5` vs the other deserializers is
   `mm_set_epi8(bytes[i] as i8, …)` — 16 individual `Seq.index bytes _`
   lookups feeding a Vec128.  The other deserializers use
   `mm_loadu_si128(slice)` which has a single `bit_vec_of_int_t_array`
   spec (no 16-arm byte case-split per output bit).  Factoring the
   helper to take the post-`mm_set_epi8` Vec128 directly (rather than
   bytes) eliminates this blowup inside the helper.

   Helper signature (now in Rust source):
   ```rust
   #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
   #[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
       $result i = (if i % 16 >= 5 then 0 else
           let shift_inv = ((i / 16) % 2) * 5 + (((i / 16) % 8) / 2) * 2 in
           let j = i + shift_inv in
           let byte_pos = j / 8 in
           let c_byte =
             if byte_pos < 16
             then (byte_pos / 4) * 2 + (byte_pos % 2)
             else ((byte_pos - 16) / 4) * 2 + ((byte_pos - 16) % 2) + 8 in
           $c (c_byte * 8 + j % 8))"#))]
   fn deserialize_5_vec(c: Vec128) -> Vec256 { … }
   ```

   Discharge inside the helper:
   ```fstar
   assert_norm (BitVec.Utils.forall256 (fun i -> $result i = (… same expression …)))
   ```
   Result: 105 queries, all pass; max rlimit usage 28/400, 79 ms max
   single-query wall.  This is the fully-verified piece.

## What didn't work — the outer bridge from `c` to `bytes`

The outer fn must derive `result i = bit_vec_of_int_t_array bytes 8
((i/16)*5 + i%16)` (for i%16<5) from:
- helper's ensures: `result i = c (c_byte(i) * 8 + j(i)%8)`
- `mm_set_epi8` spec: `c (8*k + b) = bit_vec_of_int_t_array bytes 8
  (byte_map[k]*8 + b)` where `byte_map = [0;1;1;2;2;3;3;4;5;6;6;7;7;8;8;9]`.

Three attempts, all timed out at rlimit 400:
1. `assert_norm (forall256 (… coefficients (…) = bit_vec_of_int_t_array bytes 8 (…)))`
   → 152s timeout, query 41 of `deserialize_5_`.
2. Same predicate split into 4 quarters of `forall_n 64 (…)`.  All four
   quarters timed out: 463s, 144s, 135s, 138s, 172s.  (Total run 18 min.)
3. Pure arithmetic identity (no `coefficients`, only `byte_map_val * 8 + j%8 =
   (i/16)*5 + i%16`) — `forall256` even of that pure-arithmetic form
   timed out at 105s (the if-cascade in `byte_map_val` is itself
   expensive over 256 conjuncts).  Quarters not tried because the
   pure-arithmetic form alone is necessary-but-not-sufficient.

Per `feedback_proof_debug_budget` (30–60 min cap), we stopped after the
3rd attempt and reverted to `panic_free` for the outer.  The outer body
is now a trivial composition of `mm_set_epi8` + `deserialize_5_vec`,
plus the injected `admit ()` for the ensures.

## Files touched

- `fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti` — fixed
  `mm256_mullo_epi16_specialized4` shift formula and updated comment.
- `libcrux-ml-kem/src/vector/avx2/serialize.rs` — refactored
  `deserialize_5` to call inner helper `deserialize_5_vec(c: Vec128)`
  with `[@@"opaque_to_smt"]` and a complete c-form ensures.  Marked
  outer with `verification_status(panic_free)`.

## Why outer status didn't worsen

The outer was already `panic_free` at sprint start.  It remains so.
The structural improvement (factoring + opaque helper + corrected
specialized4 spec) cleans the verification surface for the next
attempt without touching ABI, runtime semantics, or other functions.
