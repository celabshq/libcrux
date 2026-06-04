# Agent: Neon.Ntt panic-freedom (0-lax) — DONE

Module: `Libcrux_ml_kem.Vector.Neon.Ntt`
Source: `libcrux-ml-kem/src/vector/neon/ntt.rs`
Status: **CLOSED — verifies clean out of ADMIT_MODULES at panic_free tier, 0 admits.**

## Result
Clean from-scratch build (`.checked` deleted first): `status: ok`, `failed_modules: []`,
`Verified module: Libcrux_ml_kem.Vector.Neon.Ntt`. `grep -c 'admit ()'` == 0 in both `.fst` and `.fsti`.

Query-stats (all real, rlimit lines):
- ntt_layer_1_step      succeeded (with hint), used rlimit 0.061
- ntt_layer_2_step      succeeded, used rlimit 0.058
- ntt_layer_3_step      trivial body, no SMT query
- inv_ntt_layer_1_step  succeeded (with hint), used rlimit 0.061
- inv_ntt_layer_2_step  succeeded, used rlimit 0.058
- inv_ntt_layer_3_step  trivial body, no SMT query
- ntt_multiply          succeeded (with hint), used rlimit 0.563

## Root cause / fix
The ONLY panic-freedom obligation in the whole module was the four `Rust_primitives.Arithmetic.neg`
calls in `ntt_multiply` (negating i16 zetas; overflow at i16::MIN). Every other function uses only
`e_vld1q_s16` (length-8 literal arrays → length pre auto-discharged) plus `Prims.l_True`-pre
intrinsics, so layers 1/2/3 (fwd+inv) verified untouched.

Key gotcha: the requires must live in the **`.fsti` `val`**, not in the `.fst` `let`. hax emits the
`#[hax_lib::requires]` as the `Prims.Pure (requires ...)` on the interface `val`; the `.fst` body
verifies against that. (An hour was spent because adding `Prims.Pure (requires …)` directly to the
`.fst` `let` is ignored — the `.fsti` `val`'s `Prims.l_True` pre wins, so `neg` kept failing.)

## EXACT .rs deliverable (the only edit — 2 lines)
In `libcrux-ml-kem/src/vector/neon/ntt.rs`, immediately above `pub(crate) fn ntt_multiply`:
```rust
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\
                            Spec.Utils.is_i16b 1664 zeta3 /\ Spec.Utils.is_i16b 1664 zeta4"#))]
```
This is identical to the bound AVX2 and Portable `ntt_multiply` already require, so the trait
contract (`ntt_multiply_pre`) and all callers already satisfy it.

`.fst` body left byte-identical to the pristine extraction (confirmed via diff against /tmp backup).
The matching `.fsti` change (requires on `val ntt_multiply`) is scratch the parent regenerates from
the `.rs` annotation on the next extract.

## Note for parent (NOT a blocker)
The Neon trait wrapper `vector/neon.rs::ntt_multiply` (line 126, outside my lane) forwards to this fn.
Since the trait `Operations::ntt_multiply` precondition already carries `is_i16b 1664` for AVX2/Portable,
the wrapper composes with no extra annotation needed. No intrinsic `requires` had to be added
(all Arm64 intrinsics used here already have adequate pre: `e_vld1q_s16` len≥8, `e_vld1q_u8` len≥16,
both satisfied by literal arrays; everything else is `Prims.l_True`).
