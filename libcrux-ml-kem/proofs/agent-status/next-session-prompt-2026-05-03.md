# Next-session prompt — ML-KEM proof sprint 2026-05-03

> **⚡ REQUIRED READING before any work:**
> `~/.claude/skills/fstar-for-libcrux/README.md`
> This skill captures the libcrux-specific F* workflow, token discipline rules,
> fstar-mcp usage guide, fsti cascade protocol, and common failure modes.
> Read it before writing a single line of F*.

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** `d916e9563` (keys_from_private_key wrapped in --admit_smt_queries true; is_bounded_polynomial_vector threaded up through decapsulate)

**Full context:** See `proofs/agent-status/sprint-plan-2026-05-03.md` (sprint plan) and `proofs/agent-status/sprint-2026-05-03-rollup.md` (session rollup with audit debt detail).

---

## What was done in the last session

1. Merged Stream 1 (Ind_cca.Unpacked removed from ADMIT_MODULES) and Stream 2 (deserialize_vector lax→panic_free) into `libcrux-ml-kem-proofs`.
2. Threaded `is_bounded_polynomial_vector v_K #v_Vector (mk_usize 3328) secret_as_ntt` through the full decapsulate precondition cascade:
   - `Libcrux_ml_kem.Ind_cpa.fsti` — ensures on `generate_keypair_unpacked`
   - `Libcrux_ml_kem.Ind_cca.Unpacked.fsti` — ensures on `keys_from_private_key`, ensures on `generate_keypair`, requires on `decapsulate`
   - `Libcrux_ml_kem.Ind_cca.Instantiations.{Portable,Avx2,Neon}.Unpacked.fsti` — requires on `decapsulate`
   - All 9 `Libcrux_ml_kem.Mlkem{512,768,1024}.{Portable,Avx2,Neon}.Unpacked.fsti` — requires on `decapsulate`
3. Created `~/.claude/skills/fstar-for-libcrux/README.md` — the libcrux-specific F* skill.
4. `make check/Libcrux_ml_kem.Ind_cca.Unpacked.fst` exits 0 at `d916e9563`.

---

## ⚠️ Audit debt — three --admit_smt_queries true blocks to remove

These are the first priority if time allows, but they are UNBLOCKING (the make passes). Do not remove them without first writing the replacement proof — removing them cold will cause a make failure.

### 1. `Ind_cca.Unpacked.fsti` (untracked, on-disk) — around `val keys_from_private_key`

Root cause: The fsti-level subtyping check for the new `is_bounded_polynomial_vector` ensures on `keys_from_private_key` fires a Z3 query at rlimit 80. Z3 returns "incomplete quantifiers."

Fix path: Add an explicit `assert` hint in the `.fst` body of `keys_from_private_key` that links `deserialize_vector`'s postcondition to the final record field. The body does 5 sequential record updates; the first one sets `f_secret_as_ntt = deserialize_vector ...`; the subsequent 4 touch only public-key and rejection-value fields. A `calc` or `assert (result.f_private_key.f_ind_cpa_private_key.f_secret_as_ntt == deserialize_vector ...)` hint should suffice.

### 2. `Ind_cca.Unpacked.fsti` (untracked, on-disk) — around `val decapsulate`

Root cause: Adding the universally-quantified `is_bounded_polynomial_vector` hypothesis to `requires` confused Z3 when re-checking the existing `ensures` clause (which calls `ind_cca_unpack_decapsulate`). The cached hint is stale; fresh query timed out at rlimit 80.

Fix path: Once the ensures Z3 query stabilizes with the new hypothesis, remove the push/pop. May need an SMTPat or `norm` hint to prevent the quantifier from polluting the ensures-proof context. Try bumping rlimit to 200 first, then add a `norm [delta_only [...]]` to scope the quantifier.

### 3. `Ind_cca.Unpacked.fst` (git-tracked, committed d916e9563) — around `let keys_from_private_key`

Root cause: Same as #1 above. The fsti postcondition requires the fst body to prove the bound flows from `deserialize_vector` through the record update chain. F* can't do this automatically.

Fix path: Same assertion hint as in #1. Once the hint is in place, remove the push/pop from the `.fst` AND from the `.fsti` (both must go together).

---

## Pending U-tasks (from sprint plan, priority order)

### U-task 3 — Verify noeq fix propagated (QUICK CHECK)

```bash
make check/Libcrux_ml_kem.Mlkem512.Portable.Unpacked.fst > /tmp/make-noeq.log 2>&1
grep -nE '(error|Error|Failed|Cannot)' /tmp/make-noeq.log | head -20
```

The `noeq` duplicate annotation fix (commit `d51105087`) was applied to `Libcrux_ml_kem.Vector.Neon.Vector_type.fsti`. This check confirms the downstream Mlkem512.Portable.Unpacked module is clean.

### U-task 4 — Draft Phase C bridge lemma signature

File: `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst` (or a new sibling)

Goal: Write a `val` (body can be `admit ()` initially) for the slice-equality connecting `byte_encode 12` over a vector to `serialize_secret_key`. Shape:
```fstar
val serialize_secret_key_byte_encode_commute
      (v_K: usize) (#v_Vector: Type0) (#[...] _: ...) (v: t_PolynomialRingElement v_Vector)
    : Lemma
      (ensures
        forall j. j < v_K ==>
        Seq.slice (Hacspec_ml_kem.Serialize.serialize_secret_key v_K v_Vector ...) (j * 384) ((j+1) * 384)
        == Hacspec_ml_kem.Serialize.byte_encode 12 v.[j])
```

This unblocks 4 cascade-lax fns in `src/ind_cpa.rs` (Family A cluster, Day 2 agent work per sprint plan).

### U-task 5 — USER-12 NTT layer 1 attempt

Function: `op_ntt_layer_1_step` in `Libcrux_ml_kem.Vector.Portable.fst`

Pattern: 4 per-branch concrete-`b` helpers + per-lane wrapper + `--split_queries always` on the per-vector composition. Documented in memory `feedback_layer2_branch_post_z3_unlock`. This has been Z3-saturating since Phase 6 — the branch-helper refactor is the documented fix.

Budget: 60 min hard cap. If not closed, document precise blocker in `proofs/agent-status/stream0-status.md` and move on.

### U-task 6 — Family A inductive unfolding lemma

File: `specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.Serialize.fst` (or a sibling commute module)

Goal: An inductive unfolding lemma asserting `Seq.slice (serialize_secret_key K T v) (j*B) ((j+1)*B) == byte_encode v[j]` for all `j < K`. Once landed, both Family A fns (`serialize_vector`, `compress_then_serialize_u` in `src/ind_cpa.rs`) can flip mechanically.

### U-task 7 — `ntt_vector_u` functional ensure restoration

File: `src/ntt.rs`, lines 560-561 (functional ensure commented out on `ntt_vector_u`)

Goal: Uncomment the ensure and verify `ntt_vector_u` against it. The loop invariant for `deserialize_then_decompress_u` in `src/ind_cpa.rs` cannot be maintained across the in-place NTT call without this ensure. Once `ntt_vector_u` has a real functional ensure, `deserialize_then_decompress_u` flips mechanically.

---

## Untracked fsti situation

All `.fsti` files in `proofs/fstar/extraction/` are NOT git-tracked. The 13 files modified for the `is_bounded_polynomial_vector` cascade exist on disk only. On re-extraction via `hax.py extract`, they will be overwritten.

**Options:**
1. `git add proofs/fstar/extraction/*.fsti && git commit` — preserves the edits in git
2. Leave on disk — safe as long as extraction is not re-run

**Recommendation:** Before running `hax.py extract` for any reason, check if the fsti edits need to be preserved.

---

## Quick orientation commands (do not Read make logs)

```bash
# Check current branch and tip
git log --oneline -5

# Verify Ind_cca still passes after any edits
make check/Libcrux_ml_kem.Ind_cca.Unpacked.fst > /tmp/make-ind_cca.log 2>&1
grep -nE '(^\* Error|^make\[|Failed)' /tmp/make-ind_cca.log | head -30

# Check which modules are still in ADMIT_MODULES
grep 'ADMIT_MODULES' proofs/fstar/extraction/Makefile | head -5
```
