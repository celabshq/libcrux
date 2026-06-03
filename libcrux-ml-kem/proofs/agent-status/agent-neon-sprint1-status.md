# Neon Sprint 1 — status

Session start: 2026-06-03 ~23:00 CEST, tree 318b07e72.

## Stage 0 — recon + baseline ✅
- cargo test simd128 (self): 18/18 pass (native arm64).
- Error 162 notes located: MLKEM_STATUS.md L826 — masked at Vector_type.fsti(10,0-13,1), "decidable-equality module-level issue", Phase 6d admit was a 5-min unblock, never diagnosed.
- arm64_extract.rs diff vs sha3 repo: 13 hunks / 233 lines, fully cataloged.

## Stage 1 — intrinsics sync ✅
- Copied sha3's arm64_extract.rs wholesale (all 13 hunks: 9 vecN_as_TxM_axiom .fst-side definitions, _vst1q_bytes_u64 byte-level post, _vshrq/_vshlq_n_u64 requires+ensures, real fallback bodies for _vxarq/_vbcaxq/_vrax1q/_veor3q).
- Copied fstar-helpers/fstar-bitvec/Bitvec.U64Rotate.fst (NEW FILE; carries ONE `assume val lemma_u64_rotate_left_decomp` — standard rotate-left = shl^shr identity, SMTPat scoped to Arm64_extract consumers; discharge tracked as TODO(bug1-vxarq) in the file).
- NOT touched: avx2.rs / avx2_extract.rs (differ in the other direction; ml-kem ahead).
- `python3 hax.py extract` exit 0; extracted Arm64_extract.{fst,fsti} carry the sync.
- Clean rebuild: rm'd Libcrux_intrinsics.Arm64_extract.fsti.checked, fstar_build exit 0 in 12 s, real Query-stats lines (build 2a077562). NOTE: the Arm64_extract .fst itself is not in any build's root set (same as sha3 repo) — only the .fsti is consumed.

## Stage 2 — Error 162 + vector_type ✅
- **Error 162 has evaporated**: the un-admitted .fsti verifies clean under the
  0.3.7 extraction (the Phase 6d decidable-equality issue was fixed somewhere
  upstream — likely hax 0.3.7's noeq/interface emission). No masking needed.
- Vector.Neon.Vector_type.{fst,fsti} REMOVED from ADMIT_MODULES.
- `let repr = admit()` GONE — repr now transparent in the .fsti
  (`Seq.append (vec128_as_i16x8 low) (vec128_as_i16x8 high)`), AVX2-style.
- to_i16_array / from_i16_array / ZERO: **fully verified** (stronger than
  AVX2's panic_free parity). Proof recipe: introduce-forall per j<16 +
  Seq.lemma_index_slice/app1/app2 seeds + lemma_eq_intro (the update_at_range
  slice-equation posts need the slice-index terms seeded; trigger circularity
  otherwise). All queries < 1 rlimit unit (build 11a82c3c).
- to_bytes / from_bytes: panic_free + BitVecEq to/from_le_bytes_post_N posts
  (EXACT parity with AVX2 and Portable, which are also panic_free here);
  full BitVecEq discharge is Sprint 2 (serialize bridges).
- New intrinsics post: _vst1q_s16 content post (8 lanes + frame), mirroring
  the sha3 _vst1q_bytes_u64 pattern.
- Added lemma_repr_index (SMTPat on `Seq.index (repr x) j`) in Vector_type
  fsti — the per-lane bridge all sibling modules will lean on.

## Stage 3 — arithmetic.rs ✅
- Intrinsics: _vshrq_n_s16 requires+lane post; _vreinterpretq_{s16_u16,
  u16_s16} now carry cross-view cast_mod lane posts (the u16/i16 lane views
  are independent axioms; the cast posts live exactly at the type-pun).
- Contracts added (portable-mirrored, .f_elements→repr): add, sub,
  multiply_by_constant (spec::*_pre/post), bitwise_and_with_constant,
  shift_right (map_array posts), cond_subtract_3329 (mask-chain proof wired
  with logand_lemma per lane), to_unsigned_representative (full portable
  hint chain; discharges add_pre).
- barrett_reduce / montgomery_* + int16x8_t helpers: NO contracts this
  sprint (verify trivially panic-safe; ensures = Sprint 3, needs vqdmulh
  models).
- Param renames v→vec (F* `v` coercion shadowing) + rm0/rm1 let-bindings:
  rename-only changes, accepted precedent.

## Stage 3 results

- **Vector.Neon.Arithmetic.fst VERIFIED FIRST TRY** (build f5eb4b38, 0
  errors, ~40 s): the lemma_repr_index SMTPat + intrinsic lane foralls
  carried add/sub/multiply_by_constant with ZERO body hints; only
  cond_subtract_3329 (logand_lemma per-lane introduce-forall) and
  to_unsigned_representative (portable's assert chain) needed wiring.
- Heavy queries: cond_subtract_3329_ 1×24 s, to_unsigned 1×13.5 s (both
  rlimit 300, succeeded; remedies noted in fstar-perf-top20.md).

## Wrap-up gate ✅

- Full-crate `make all`: exit 0, 0 errors (build bde54056, 56 s warm).
- cargo test simd128 (mlkem512+768+1024, self): 18/18, native arm64.
- Phase 8 admit regression: Arithmetic.fst 0 admits; Vector_type.fst 2
  `admit () (* Panic freedom *)` (to_bytes/from_bytes panic_free —
  intended, exact AVX2/Portable parity). No stray assume/admit in Rust.
- ml_kem_verification_status.md regenerated: Neon 83 fns = 64 lax /
  1 unv / 8 PF / 10 Math (was 82 lax / 1 unv). ADMIT_MODULES 8 → 5
  entries.
- Perf top-20: dated incremental snapshot appended.
- Sprint 2 prompt: next-session-prompt-neon-sprint2-serialize.md.

## Trust-footprint additions this session (flag for review)

1. fstar-helpers/fstar-bitvec/Bitvec.U64Rotate.fst (sha3 sync):
   `assume val lemma_u64_rotate_left_decomp` — rotate-left = shl^shr
   identity, SMTPat-scoped; discharge tracked in-file (TODO bug1-vxarq).
2. arm64_extract.rs model posts (axiomatic by construction, like all
   intrinsics models): _vst1q_s16 content post; _vshrq_n_s16 lane post;
   _vreinterpretq_{s16_u16,u16_s16} cross-view cast_mod posts (the
   u16/i16 lane views are independent `assume val` axioms — these two
   posts are the ONLY bridge, placed exactly at the type-pun).
3. vecN_as_TxM_axiom .fst-side definitions (9 types, sha3 sync) — make
   Arm64_extract.fst well-formed; .fsti unchanged in trust terms.
