# Agent: Neon.Serialize panic-free (0-lax) — DONE

Module: Libcrux_ml_kem.Vector.Neon.Serialize (12 fns) — VERIFIED, 0 admits.

## Result
`Verified module: Libcrux_ml_kem.Vector.Neon.Serialize`, exit 0, 0 failed queries,
0 `admit ()` in .fst and .fsti. All 12 functions have real Query-stats lines
(rlimit usage 0.04-3.0, well under cap 80). Slowest: serialize_10/serialize_12 ~5.5s.
Build id 602167af (log archived). cargo check --features simd128 also clean.

## KEY FINDING (workflow)
The module has a **.fsti interface**. Panic-free preconditions go in the **.fsti**
`val` signatures (as `requires`) — when a .fsti exists F* checks the .fst body against
the interface signature, so a requires placed only in the .fst is ignored. Mirror the
AVX2 Serialize .fsti, whose requires are exactly the trait-spec preconditions.

## Edits to serialize.rs (the real deliverable)
- Added `#[cfg(hax)] use crate::vector::traits::spec;` after the imports.
- deserialize_1 : `#[hax_lib::requires(a.len() == 2)]`
- deserialize_4 : `#[hax_lib::requires(v.len() == 8)]`
- deserialize_5 : `#[hax_lib::requires(v.len() == 10)]`
- deserialize_10: `#[hax_lib::requires(v.len() == 20)]`
- deserialize_11: `#[hax_lib::requires(v.len() == 22)]`
- deserialize_12: `#[hax_lib::requires(v.len() == 24)]`
- serialize_5 : `#[hax_lib::fstar::before(interface, r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#)]`
                + `#[hax_lib::requires(fstar!(r#"${spec::serialize_5_pre} (repr ${v})"#))]`
- serialize_11 : `#[hax_lib::requires(fstar!(r#"${spec::serialize_11_pre} (repr ${v})"#))]`
- serialize_1/4/10/12: take SIMD128Vector, no slice/bound obligation, NO requires needed.

## .fsti edits (mirror of what the .rs re-extraction will regenerate; for parent to confirm)
deserialize_1: `requires impl__len a =. mk_usize 2`
deserialize_4/5/10/11/12: `requires Seq.length v == 8/10/20/22/24`
serialize_5: `requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_5_pre (repr v)`
serialize_11: `requires Libcrux_ml_kem.Vector.Traits.Spec.serialize_11_pre (repr v)`

## Notes for parent integration
- NO intrinsic-requires blockers: all arm64 intrinsics here are either l_True
  (e_vshlq_s16/u16, e_vsliq_n_s32/s64, e_vqtbl1q_u8, e_vst1q_u8) or already carry
  their own slice-length requires (e_vld1q_s16 len>=8, e_vld1q_u8 len>=16) which the
  function-level requires + the t_Array(16) sizes discharge automatically.
- The .rs `serialize_5_pre`/`serialize_11_pre` form is == the trait-spec
  `serialize_pre_N 5/11 (repr v)`; verified under both spellings.
- No body asserts/loop_invariants were needed — all obligations close from the
  function-level requires alone.
- Remove `Neon.Serialize` from ADMIT_MODULES (already done in working-tree Makefile);
  this module is now genuinely panic-free.
