# Sprint 2026-05-12 — fully verify `deserialize_5` (AVX2)

Worktree: `/Users/karthik/libcrux-serialize-1-deserialize-5/libcrux-ml-kem`
Branch:   `agent-mlkem-serialize-1-deserialize-5-2026-05-11` (extending; `mm256_mullo_epi16_specialized4` already committed at `956353981`)

## T+0 — start
- Goal: turn `deserialize_5` from `panic_free` → fully verified.
- Plan: mirror `deserialize_10_vec` (line 631) — `[@@"opaque_to_smt"]`
  inner helper, single `assert_norm (forall256 ...)` on the post-mullo
  state. Differences vs the prompt's outline: the prompt suggested a
  half-/quarter-split via `forall_n N`; `deserialize_10_vec` shows a
  cleaner one-shot `forall256` works once the closure is isolated in an
  opaque helper.
- Sanity step: confirm `mm256_mullo_epi16_specialized4` actually fires
  on the multiplier (4th unify arm matches by byte-level inspection,
  but assert_norm proves it).
- ETA: probe + factor by T+45 min; full make pass by T+75 min.
- Blocker: none yet.
