# Agent: Track B applied to `impl Operations for AVX2SIMDUnit`

Branch: `ml-dsa-proofs`
Started: 2026-05-08

## Goal
Mirror Portable's Track B layout in `src/simd/avx2.rs`. Each non-trivial impl method body becomes a one-line dispatcher to a `<method>_with_proof` free function.

## Baseline (Snapshot 2026-05-08b)
- `Simd.Avx2::impl_1`: **11.80s**, **40 queries**, max 4101 ms.
- `Simd.Portable::impl_1`: **0.52s**, **4 queries** (target shape).

## Already-extracted (in HEAD)
- `infinity_norm_exceeds_with_proof` (impl method already one-line dispatch)
- `shift_left_then_reduce_with_proof` (impl method already one-line dispatch)
- `power2round_with_proof` (impl method already one-line dispatch)
- `reduce_with_proof` (impl method already one-line dispatch)

## Target methods to extract
| method | size | priority |
|---|---:|---|
| `add` | medium | A |
| `subtract` | medium | A |
| `decompose` | large | A |
| `compute_hint` | admit | C (rename) |
| `use_hint` | admit | C (rename) |
| `montgomery_multiply` | medium | A |
| `gamma1_deserialize` | small | B |
| `error_deserialize` | small | B |
| `t0_serialize` | small | B |
| `t0_deserialize` | small | B |
| `t1_deserialize` | small | B |
| `rejection_sample_*` (3) | admit | C (rename) |
| `ntt`, `invert_ntt_montgomery` | admit | C (rename) |

## Status
- [DONE] batch 1: `add`, `subtract`, `montgomery_multiply` — extracted, F* clean.
- [DONE] batch 2: `decompose`, `compute_hint` (admit), `use_hint` (admit) — extracted, F* clean.
- [SKIP] batch 3 (inline-fstar small methods): tried `gamma1_deserialize`, `error_deserialize`, `t0_serialize`, `t0_deserialize`, `t1_deserialize` extraction; reverted because `t1_deserialize_with_proof` saturates rlimit 80 in free-fn context (the inline form was passing). Inline fstar! methods need to keep full impl-block context.
- [SKIP] batch 4 (admit-based methods): `rejection_sample_*`, `ntt`, `invert_ntt_montgomery` would just rename the admit; minimal value.
- [In progress] cold-cache full prove + perf comparison vs baseline.

## Findings during the work
- Initial extracts using `specs::add_pre`-style refs in `fstar!()` raw blocks broke (specs:: doesn't resolve in raw F* literal — must use `Libcrux_ml_dsa.Simd.Traits.Specs.*`). Fixed.
- Extracting small inline-fstar methods (e.g. `t1_deserialize`) into free-fns regressed: outside the impl-block's `--split_queries always` context, the post becomes harder to discharge (rlimit-sat 80/80). Reverted those.
- Methods with non-trivial proof blocks that mirror Portable's pattern (`add`, `subtract`, `montgomery_multiply`, `decompose`) extracted cleanly.

## ETA
Cold-cache prove running; report after that.
