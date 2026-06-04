# Campaign prompt — migrate libcrux sha3 proofs to hax-lib 0.3.7

## Mission
Make the **sha3** F* proof tree verify under **hax-lib 0.3.7** (`tag=cargo-hax-v0.3.7`,
rev `d8b5b3d`), up from the current 0.3.6 (`branch=integer-lemmas`, rev `952bee0`).
Today 154/156 sha3 modules already verify under 0.3.7; **2 proofs block**. Close them,
then gate the full tree, then hand back a clean diff for review (do NOT merge to the
0.3.6 line yourself).

## Where everything is
- **Work here (isolated):** worktree `/Users/karthik/libcrux-sha3-hax037`, branch
  `sha3-hax-0.3.7-migration` (off the squeeze4 tip `75721cd1f`). Already set up:
  Cargo.toml pinned `tag=cargo-hax-v0.3.7` + `hpke-rs` commented (line 191);
  `bash hax.sh extract` run (extraction `.fst` are gitignored → must be regenerated
  in a fresh checkout); the squeeze-side byte_eq already factored (see below).
- **Do NOT touch** the main worktree `/Users/karthik/libcrux-sha3-proofs`
  (`sha3-proofs-focused`) — it's the clean, committed+pushed 0.3.6 state (sha3 at zero
  admits, commit `75721cd1f`). The 0.3.7 pin must NOT be committed there.
- **`hpke-rs` gotcha (mandatory):** the top-level `libcrux` crate's `hpke-rs` dep pulls
  crates.io `libcrux-*` pinned to hax 0.3.6. Under a 0.3.7 pin that yields TWO hax-lib
  versions → both proof-lib trees on the F* include path → `Error 308` recursive-dep
  cycle. Keep `hpke-rs` commented in this worktree (already done). `Cargo.lock` is
  untracked.

## What actually changed 0.3.6 → 0.3.7 (the whole delta: 11 files)
Only ONE functional change: **`rotate_left` went opaque → concrete.**
- `Core_models.Num.fst`: u8/16/32/128 `impl_uN__rotate_left` was `assume val` (opaque),
  now a concrete `let` delegating to new `Rust_primitives.Arithmetic.rotate_left_uN`
  aliases (and normalizes `n %! bits`). u64 similar (lost its `unfold`).
- `Rust_primitives.Arithmetic.fsti`: +`unfold let rotate_left_uN = rotate_left_u #UN`.
- `core/num/mod.rs`: `rotate_left` lost `#[hax_lib::opaque]` + normalizes the shift.
- `Core_models.Array.fst` / `array.rs` `from_fn`: **whitespace-only** in F* (Rust gained a
  `fstar::replace` attr that pins the identical output) — NO semantic change, ruled out.
- Rest: version bump 0.3.6→0.3.7, CHANGELOG, blog, opam/dune. Cosmetic.

Consequence: proofs/hints recorded against the *opaque* `rotate_left` (anything in the
keccak permutation cone) no longer replay once it's concrete. That's why hint-dependent
proofs break even though the change looks tiny.

## The 2 blocking proofs
1. **`EquivImplSpec.Sponge.Avx2.fst` → `lemma_sq_lane_avx2_eq_squeeze_state`**, specifically
   its per-byte core (now factored into the standalone **`lemma_sq_lane_byte_eq_avx2`** —
   the avx2 `sq_lane_avx2[i] == squeeze_state[i]` byte bridge). `Error 19`,
   "incomplete quantifiers".
2. **`Libcrux_sha3.Generic_keccak.Simd128.fst` → `squeeze2`** composer (arm64 N=2).
   `Error 19`, "assertion failed". (Not yet deep-dived — likely the same class.)
   Everything else that "doesn't verify" is just blocked downstream of these two.

## Diagnosis (do not re-derive — confirmed this session)
- Both are **hint-dependent**: `lemma_sq_lane_avx2_eq_squeeze_state` fails hint-free
  **even at 0.3.6** (fresh cache: 629/800, incomplete quantifiers). They pass in CI only
  by replaying recorded hints. 0.3.7 makes `rotate_left` concrete → the 0.3.6 hints don't
  replay → from-scratch must work, and it doesn't yet.
- **Option tuning cannot close it** (all tried, all fail): `--using_facts_from` variants,
  `--fuel 2 --ifuel 0` (the hint's recorded config; ifuel 0 < module's ifuel 1),
  standalone factoring, and combinations.
- **`--using_facts_from '* -Rust_primitives.Slice.array_from_fn
  -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'`**
  (mirrors the exclusion the sibling `Simd.Avx2.Store.fst` proofs at lines 1293/1577
  already use) is a **KEEPER** — it cut the byte_eq from 18 min → 46 s (the real
  `array_from_fn` cascade). It is currently applied to `lemma_sq_lane_byte_eq_avx2`.
- **smtprofile of the RESIDUAL** (`queries-EquivImplSpec.Sponge.Avx2-1.smt2`, z3 4.13.3
  `smt.qi.profile=true`): peak single-quantifier count = **169** over 181 snapshots
  (NOT a cascade — contrast load_block's 195M). Query quits at 360/800 rlimit. Dominant
  qids: `projection_inverse_BoxInt_proj_0`, `Prims_pretyping`, `int_typing`,
  `pow2_values`, `op_Multiply` → **integer/byte arithmetic** (the `(i-start)/8`, `%8`,
  `to_le_bytes` shift/pow2 math). So the residual is a genuine **missing instantiation**
  (trigger gap), NOT noise — the fix is to *spell out the byte-arithmetic bridge*, not
  prune more facts.

## The fix to write (prime hypothesis)
Compare to the **load-side** byte_eq `lemma_load_block_byte_eq_avx2` (same file, ~line 95)
— it CLOSES from scratch and is the working twin. It does what the squeeze side is
MISSING:
- `reveal_opaque (\`%...Load.load_lane_u64) ...load_lane_u64` (reveals the per-lane
  opaque load function), and
- `SP.lemma_subslice_bytes_eq blocks offset rate ii` (the byte/subslice arithmetic
  bridge).
The squeeze side (`lemma_sq_lane_byte_eq_avx2`) currently only seeds `get_lane_u64`
mentions + `avx2_lane` unfold — it has **no analogous `reveal_opaque` of the store-lane
function and no byte-decomposition lemma**. Almost certainly the missing instantiation:
spell out, for the in-range index `i`, the equation
`sq_lane_avx2[i] == to_le_bytes(get_lane_u64 state.[(i-start)/8] (mk_usize l))[(i-start)%8]`
and the matching `squeeze_state[i]`, revealing whatever store-side opaque
(`Simd.Avx2.Store.store_lane_*` / the `f_squeeze4` per-byte ensures) the load side's
`load_lane_u64` reveal mirrors, plus a subslice/byte lemma analogous to
`lemma_subslice_bytes_eq`. Then `Classical.forall_intro` + `eq_intro` already compose
(that part verifies). Apply the same shape to `Simd128.squeeze2` (and likely the arm64
`lemma_sq_lane_arm64_eq_squeeze_state` if it has the same inline form).

## Fast iteration recipe (use this — don't fight `make`)
The 154 passing 0.3.7 `.checked` are in `/tmp/sha3-037-cache`. Verify ONE module via
`fstar.exe` direct (content-hash dep reuse; `make` would mtime-rebuild everything after
re-extraction):
```
cd /Users/karthik/libcrux-sha3-hax037/crates/algorithms/sha3/proofs/fstar/extraction
PREFIX=$(cat /tmp/wt_prefix_037.txt)        # worktree paths + d8b5b3d includes + /tmp cache
$PREFIX ../equivalence/EquivImplSpec.Sponge.Avx2.fst \
    --admit_except 'EquivImplSpec.Sponge.Avx2.lemma_sq_lane_byte_eq_avx2'   # isolate, ~1 min
```
Judge by EXIT 0 + "All verification conditions discharged" (cache writes blocked by a
corrupt repo `Core_models.Num.fst.checked`, but `/tmp/sha3-037-cache` is writable).
To re-profile: append `--log_queries --z3refresh`, then
`z3 (.../z3-4.13.3/bin/z3) smt.qi.profile=true smt.qi.profile_freq=20000 queries-...-1.smt2`.

## Gating + finish
1. Close `lemma_sq_lane_byte_eq_avx2` (then the full `lemma_sq_lane_avx2_eq_squeeze_state`
   + `avx2_sc_store_block` consumers verify).
2. Close `Simd128.squeeze2`.
3. Full-tree gate under 0.3.7 (fresh writable cache; the repo cache is corrupt-blocked):
   `make -k -j2 all CACHE_DIR=/tmp/sha3-037-cache/checked HINT_DIR=/tmp/sha3-037-cache/hints`
   (hint-free; expect all green). `-j2` to respect the ≤4 fstar/z3 process cap.
4. `cargo test --features simd256` is **x86-only** — won't compile on this arm64 host
   (libcrux-intrinsics x86 AVX2/AES); run on x86/CI.
5. Hand back the diff (the fixed `.fst` proofs + the Cargo.toml pin) for the maintainer to
   decide on merging the 0.3.7 bump workspace-wide. Whether to keep `hpke-rs` dropped or
   bump the crates.io libcrux deps is a packaging decision for them.

## Rules
- Per-function F* debug budget 30–60 min; if a proof cliffs, `fstar_note(level=cliff,...)`
  + document and move on. ≤4 concurrent fstar.exe/z3, ≤24 GB RSS.
- Don't bulk-delete or mtime-touch `.checked`. Don't commit the 0.3.7 pin to
  `sha3-proofs-focused`.
- Full prior context: this file + `proofs/agent-status/hax-0.3.7-migration-2026-05-31.md`.
