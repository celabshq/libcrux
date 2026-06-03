# Next session — AVX2 `ntt_multiply` (Step C, last AVX2-NTT leaf)

## Context
The AVX2 NTT layer-1/2 (fwd+inv) functional posts + 4 bridge-admit removals are
**done and landed on `libcrux-ml-kem-proofs`, verified under cargo-hax 0.3.7**
(commit `5b2b12f2b`). `ntt_multiply` is the **only remaining lax AVX2-NTT
function**. Read the skill `fstar-for-libcrux` first — especially §7
"createi-free lifting lemmas for `mm256` add/mullo lane foralls" and the 4-layer
cascade recipe in memory `project_avx2_ntt_leaf_cascade_recipe`.

## Target
`libcrux-ml-kem/src/vector/avx2/ntt.rs:1185` — `pub(crate) fn ntt_multiply`,
currently `#[hax_lib::fstar::verification_status(lax)]`. It computes the four
degree-1 base-case polynomial products in the NTT domain
`(a0+a1·X)(b0+b1·X) mod (X²-ζ)` for the 4 binomials sharing zeta0..3, using
**i32-widening arithmetic** (this is what makes it harder than the layer leaves).

## Goal (two deliverables)
1. **Remove the `lax` on `ntt_multiply`** and give it a functional post — the
   AVX2 analogue of the trait-level binomial post. Mirror the **portable
   parity** (Phase-2 sibling template, already proven):
   `libcrux-ml-kem/src/vector/portable/ntt.rs:347` `ntt_multiply_binomials`
   carries `ntt_multiply_binomials_post a b zeta i out_future`. AVX2 does all 4
   binomials at once, so its post is the 4-way conjunction (one per binomial /
   zeta), i.e. the per-pair: `out_even = (a_even·b_even + a_odd·b_odd·ζ)·R⁻¹`,
   `out_odd = (a_even·b_odd + a_odd·b_even)·R⁻¹`, mod q, with the `is_i16b_array
   3328` bound.
2. **Close `assume val lemma_ntt_multiply_chunk_commutes`** at
   `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst:1399`. It
   bridges `T.f_ntt_multiply lhs rhs zeta0..3` (given `ntt_multiply_pre`) to the
   Hacspec spec `N.ntt_multiply_n (mk_usize 16) … (zetas_4_ …)` via
   `mont_i16_to_spec_array (sz 16)`. **Heads-up:** `mont_i16_to_spec_array` is
   createi-based → the [[project_mlkem_createi_smtpat_cascade]] applies here;
   mirror however the *layer* chunk-commute lemmas in the same file were closed
   (the inverse/forward `lemma_*_layer_*_step` entries just above it — most are
   now `= ()` after the leaf posts landed).

## Modelless intrinsics needing per-lane axioms (the work that's new vs layers)
`ntt_multiply` uses a wider set than the layer leaves. Add `= admit ()` per-lane
axiom lemmas (same accepted pattern as `lemma_shuffle_245` etc.) for whichever
lack an `Avx2_extract.fsti` model:
`mm256_shuffle_epi8`, `mm256_permute4x64_epi64`, `mm256_castsi256_si128`,
`mm256_cvtepi16_epi32` (**16→32-bit sign-extend** — the key widening),
`mm256_extracti128_si256`, `mm256_mullo_epi32` (**32-bit** mul, not the 16-bit
`mullo_epi16` the layers used), `mm256_madd_epi16` (pairwise i16 mul + i32 add),
`mm256_slli_epi32::<16>`, `mm256_add_epi32`, `mm256_blend_epi16`.
**Already proven, reuse directly:** `arithmetic::montgomery_reduce_i32s`
(`avx2/arithmetic.rs:343`) has a real post — per-lane
`v (get_lane result i) % 3329 == (v (get_lane vec i) * 169) % 3329` + bounds.
The two `mm256_blend_epi16` controls and the layer leaves' blend axioms are a
template for the final combine.

## Recipe (same architecture, adapt shapes to i32)
Follow the 4-layer recipe: (1) admit modelless intrinsics per concrete control;
(2) pay createi ONCE in a `lemma_*_sums`-style uniform forall over the i32 lanes;
(3) substitute shuffle/widen/perm facts to express lanes in terms of input lanes;
(4) derive the post over **plain i16/i32 arrays** (no `mm256` ⇒ cascade-free).
The cross-term structure (madd for one half, mullo+reduce for the other, then
`slli<16>` + `blend` to interleave even/odd) means the "sums" step is more
involved than the layers — consider one helper per half (`products_left`,
`products_right`) before the blend.

## Environment / workflow
- Develop where you like, but **0.3.7 is the integration target**. The fast path
  is the bridge worktree (`/Users/karthik/libcrux-user14-bridge`, old hax
  `d8b5b3d`, ~250 s warm builds) then merge to `libcrux-ml-kem-proofs`; BUT
  budget for a **0.3.7 hardening pass** at merge time (createi-free lifting
  lemmas WITHOUT SMTPat + the split-query re-assert trick — §7). To skip the
  surprise, iterate directly on a worktree off the `libcrux-ml-kem-proofs` tip
  (slower per build ~600 s but 0.3.7-faithful).
- ml-kem avx2 `.fst`/`.fsti` are **gitignored** (regenerated from `src/`); edit
  the extracted `.fst` for fast isolation, then mirror verbatim into the Rust
  `#[hax_lib::fstar::before(r#"…"#)]` block.
- Isolation-test single functions with `--admit_except '<ONE fully-qualified
  name>'` (multi-name = false-clean trap); `rm` the tainted `.checked`, finish
  with a full no-admit build showing real `used rlimit` lines.
- All F* via `fstar_build` MCP (never shell `make`); [[feedback_use_fstar_mcp]].

## Budget / expectation
This is **materially harder** than the layer leaves (i32 widening, `madd`,
cross-term interleave, the 4-binomial spec bridge). Decompose into sub-targets
(each modelless axiom → the per-half sums → the leaf post → the chunk lemma);
30–60 min per sub-target. If a sub-target stalls past budget,
`fstar_note(level="cliff")` and hand off — don't grind. Last AVX2-NTT item;
closing it makes the AVX2 NTT layer 0-lax.
