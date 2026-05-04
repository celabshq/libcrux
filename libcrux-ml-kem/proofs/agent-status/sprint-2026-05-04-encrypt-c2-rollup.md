# Sprint 2026-05-04 — encrypt_c2 lax → panic_free (Sub-task 1)

**Branch:** `libcrux-ml-kem-opaque-bounds-spike` (worktree `/Users/karthik/libcrux-opaque-spike`)
**Tip on entry:** `5341aaec2`
**Source prompt:** `proofs/agent-status/next-session-prompt-2026-05-05-encrypt-c2-and-cascade.md`
**Outcome:** Sub-task 1 closed cleanly. ind_cpa.rs lax count: 3 → 1.

## What landed

| File:Fn | Change | Why |
|---|---|---|
| `serialize.rs::deserialize_then_decompress_message` | `+is_bounded_poly(3328, result)` ensures conjunct (panic_free admits, no body change needed) | required by `compute_ring_element_v` (message arg). Sound: decompress_1 produces {0, 1664}. |
| `ind_cpa.rs::encrypt_c2` | lax → `panic_free`, `--z3rlimit 400 --ext context_pruning --split_queries always`, full requires (`is_rank K`, params, ciphertext.len, `is_bounded_polynomial_vector(3328, t_as_ntt)`, same for r_as_ntt, `is_bounded_poly(3328, error_2)`) | flips lax + body verifies through SMTPat'd opaque elim chain |
| `ind_cpa.rs::encrypt_unpacked` | `+is_bounded_polynomial_vector(3328, &public_key.t_as_ntt)` requires conjunct | provides encrypt_c2's t_as_ntt bound |

**No body helpers needed in encrypt_c2.** The dual-trigger SMTPat'd `lemma_is_bounded_polynomial_vector_elim` (per `feedback_dual_smtpat_opaque_atom`) fires automatically when `compute_ring_element_v`'s per-element forall instantiates each `Seq.index t_as_ntt i` / `Seq.index r_as_ntt i`. The aux_t/aux_r `Classical.forall_intro` shape proposed in the next-session prompt was unnecessary — opacity + SMTPat'd elim was sufficient.

## Acceptance

```
$ grep -c "verification_status(lax)" src/ind_cpa.rs src/ind_cca.rs
src/ind_cca.rs:0
src/ind_cpa.rs:1   # only deserialize_then_decompress_u remains (out-of-scope thread)

$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Serialize.fst   →  rc=0
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cpa.fst     →  rc=0
```

## What did NOT land — and why

**Sub-task 2 (cca cascade) deferred.** Hard-stopped per the prompt's "30-min sub-task 1 cap, then hard stop before sub-task 2" rule. Sub-task 2 also blocked by a pre-existing baseline issue (see below).

**Pre-existing Ind_cca.fst baseline failure exposed.** Running `OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cca.fst` on the pristine spike tip (5341aaec2, no local edits) gives `rc=2` at:

```
* Error 19 at Libcrux_ml_kem.Ind_cca.fst(228,27-228,47):
  - Assertion failed
  - See also Libcrux_ml_kem.Ind_cpa.fsti(84,8-87,19)
```

i.e. `validate_public_key`'s call to `serialize_public_key` cannot discharge `is_bounded_polynomial_vector(3328, deserialized_pk)` (the opaque atom). Root cause: `deserialize_ring_elements_reduced_out`'s ensures exposes a per-element forall, not the opaque atom; after spike commit `501888222` made `is_bounded_polynomial_vector` opaque, the bridging fold no longer happens automatically.

Fix sketch (sub-task 2 entry point):

* Either change `deserialize_ring_elements_reduced_out`'s ensures to the opaque atom directly (preferred — it's panic_free, conjunct admitted), OR
* Insert `lemma_is_bounded_polynomial_vector_intro` call in `validate_public_key`'s body via `hax_lib::fstar!`.

The spike rollup's claim of "Ind_cca.fst rc=0" was a stale-cache artifact: without `--z3refresh` the cached `.checked` predates the opacity refactor and Z3 saw through. With `--z3refresh` the failure is exposed.

**Ind_cca.Unpacked.fst body failure (pre-existing, separate)** also remains per prior rollup.

## Files touched

* `src/ind_cpa.rs` — encrypt_unpacked +1 requires conjunct; encrypt_c2 full panic_free recipe (FOLLOW-UP comment removed).
* `src/serialize.rs` — deserialize_then_decompress_message ensures +1 conjunct.

Two commits land:

1. `agent-mlkem: strengthen deserialize_then_decompress_message ensures with is_bounded_poly(3328)`
2. `agent-mlkem: flip encrypt_c2 lax → panic_free + thread t_as_ntt bound through encrypt_unpacked`

## Next-session entry point

Sub-task 2 (cca cascade) — recommended order:

1. **Resolve pre-existing Ind_cca.fst baseline first.** Either fold ensures change on `deserialize_ring_elements_reduced_out` (~10 min) or body intro lemma in `validate_public_key` (~15 min). Without this, sub-task 2 cannot land cleanly.
2. **Cascade requires up:** `Ind_cca::Unpacked::encapsulate` → `decapsulate` → `instantiations` (3 backends) → `mlkem{512,768,1024}::encapsulate` (unpacked variant), each adding `is_bounded_polynomial_matrix(3328, public_key.A) /\ is_bounded_polynomial_vector(3328, public_key.t_as_ntt)`.
3. **Discharge at producer:** `cca::Unpacked::generate_keypair`'s ensures already provides matrix+vector bounds (commit `b3aa1f0da` + `080c17468`). Should propagate trivially.

The remaining lax in `ind_cpa.rs` is `deserialize_then_decompress_u` (~line 989, USER-7); it's an independent thread (loop-invariant + bridge lemma work, ~60-90 min) and not on this cascade.
