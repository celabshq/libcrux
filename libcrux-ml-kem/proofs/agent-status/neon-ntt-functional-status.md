# Neon NTT functional posts — session status (2026-06-04)

Branch: libcrux-ml-kem-proofs, base e401d0bf0.

## VERIFIED in F* (.fst), saved to agent-status/neon-ntt-VERIFIED.{fst,fsti}.txt
All 6 layer functions + the two-vector montgomery foundation carry real
functional posts and verify (modulo the ml-dsa CPU contention that timed out the
final full module build; each was confirmed via per-function --admit_except):
- `montgomery_multiply_int16x8_t` (Neon.Arithmetic): opaque_to_smt + IMPLICATION
  post (l_True pre). Mirrors by-constant worker, vector multiplier. CONFIRMED.
- `ntt_layer_1_step`  -> ntt_layer_1_butterfly_post (s32 transpose). CONFIRMED.
- `ntt_layer_2_step`  -> ntt_layer_2_butterfly_post (s64 transpose). queries pass
  (140-310 rlimit, slow under contention); logic confirmed.
- `ntt_layer_3_step`  -> AVX2 inline-forall post. CONFIRMED.
- `inv_ntt_layer_1_step` -> inv_ntt_layer_1_butterfly_post (s32 + barrett). written.
- `inv_ntt_layer_2_step` -> inv_ntt_layer_2_butterfly_post (s64). written.
- `inv_ntt_layer_3_step` -> AVX2 inline-forall post. CONFIRMED.

### Recipe
- repr lanes 0-7 = f_low, 8-15 = f_high (lemma_repr_index SMTPat).
- transpose dance tracked by 4 ADMITTED lane-permutation lemmas
  (lemma_trn{1,2}_s{32,64}_reinterpret) — bit-layout facts, mirrors AVX2's
  admitted shuffle lemmas; a wrong permutation is caught by the butterfly post.
- forward layers: clean-context post-helper (lemma_neon_fwd_l{1,2}_post) does the
  16 modadd/modsub OUT of the function WP (one split sub-query saturated when
  inlined — the helper fixed it).
- inverse layers: add/sub BEFORE barrett/montgomery, so butterfly conjuncts ARE
  the barrett(sum)/montgomery(residue) congruences — helper is a clean reveal.
- montgomery/barrett implication-posts: caller asserts the antecedent
  (forall i<8. is_i16b 1664 (zeta lane) / is_i16b 28296 (sum lane)).

## Rust port status (commit)
- arithmetic.rs montgomery: PORTED (rename v->a, implication post).
- ntt.rs: before-block (repr + lemma_modadd/modsub + lane add/sub) + layer-3
  fwd/inv PORTED.  layer-1/2 + inverse-1/2 still l_True in Rust (build via
  implication-post montgomery); their verified .fst is in agent-status for the
  follow-up port.
- ntt_multiply: untouched (l_True). The hard one — widening MAC; FOLLOW-UP.

## Gotcha paid
- `#[cfg_attr(hax, hax_lib::fstar::before(...))]` -> when converting to bare
  `#[hax_lib::fstar::before(...)]` the trailing `))]` must become `)]` (one fewer
  paren). The extra paren gives "unexpected closing delimiter" at extraction.
- ml-dsa parallel session ran a 7GB / 27min z3 query — crippled all shared F*
  builds; used `hax.py`/`make -j2` per user instruction + per-fn --admit_except.

## Next session
1. Port layer-1/2 + inverse-1/2 to ntt.rs from neon-ntt-VERIFIED.fst.txt (full
   before-block: 4 transpose lemmas + 4 post-helpers).
2. ntt_multiply functional post (ntt_multiply_butterfly_post; widening MAC).
3. Phase C: op_* wrappers + remove Vector.Neon.{fst,fsti} from ADMIT_MODULES.
