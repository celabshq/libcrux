# Session prompt — ML-KEM Neon backend Sprint 2 (serialize bridges)

## Goal

Take `Libcrux_ml_kem.Vector.Neon.Serialize.fst` (12 fns, all lax) out of
ADMIT_MODULES, and upgrade Vector_type's `to_bytes`/`from_bytes` from
panic_free to fully verified. This is the vec128 BitVec-bridge sprint.

Scoreboard after Sprint 1 (2026-06-04, commit pending): Neon = 83 fns /
**64 lax** + 1 unverified (was 82 lax). Out of ADMIT_MODULES:
Vector_type.{fst,fsti} + Arithmetic.fst. Whole-crate `make all` green.

## State left by Sprint 1 (verified facts — do not re-derive)

- **Error 162 is DEAD**: the Phase 6d decidable-equality issue does not
  reproduce under hax 0.3.7. Vector_type.fsti verifies clean un-admitted.
- `repr` is transparent in Vector_type.fsti:
  `Seq.append (vec128_as_i16x8 low) (vec128_as_i16x8 high)`.
- **`lemma_repr_index`** (SMTPat on `Seq.index (repr x) j`) lives in
  Vector_type.fsti — per-lane bridge: index j<8 → `get_lane_i16x8 f_low j`,
  else `get_lane_i16x8 f_high (j-8)`. All sibling modules lean on it; it
  made the Arithmetic contracts verify FIRST TRY with no body hints for
  add/sub/multiply (only cond_subtract + to_unsigned needed hint chains).
- Consumer-module pattern: `#[hax_lib::fstar::before(interface,
  r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#)]` on
  the first contracted fn (interface-only! a second copy in the .fst is a
  duplicate-definition error).
- Proof recipe that closed to_i16_array/from_i16_array (use as template
  for any store/load composition): introduce-forall j<16 +
  `Seq.lemma_index_slice` + `Seq.lemma_index_app1/app2` seeds +
  `Seq.lemma_eq_intro`. The update_at_range slice-equation posts have a
  trigger circularity (index-of-slice terms must be seeded) — see
  agent-neon-sprint1-status.md.
- Intrinsics models (crates/utils/intrinsics/src/arm64_extract.rs) now
  carry: _vst1q_s16 content post, _vshrq_n_s16 requires+lane post,
  _vreinterpretq_{s16_u16,u16_s16} cross-view `cast_mod` lane posts
  (the u16/i16 lane views are independent axioms — bridge ONLY via these
  reinterpret posts), plus the sha3-synced vecN_as_TxM_axiom .fst
  definitions and `_vst1q_bytes_u64` byte-level posts.
  File is now AHEAD of sha3's copy — sync back post-sprint.
- to_bytes/from_bytes (vector_type.rs): panic_free with
  `Traits.Spec.{to,from}_le_bytes_post_N` posts (BitVecEq.int_t_array_
  bitwise_eq) — EXACT AVX2/Portable parity; both of those are ALSO still
  panic_free there. Upgrading Neon beyond parity is stretch, not blocker.

## Sprint 2 plan

1. **Recon**: read neon/serialize.rs (12 fns: serialize/deserialize for
   d=1,4,5,10,11,12 presumably). Compare with avx2/serialize.rs — the
   AVX2 backend has all five `op_serialize_N_{pre,post}_bridge` lemma
   pairs proven (op_serialize_1 landed 6e7cb0566; N=4 pair is the
   cleanest template, plus `bit_vec_of_int_t_array_*_lemma` decomposition
   + `BitVecEq.bit_vec_equal_intro`).
2. **Model gaps**: neon serialize uses table lookups / vqtbl / shifts /
   narrowing intrinsics — survey which lack posts. Decide per-intrinsic:
   BitVec-level post (get_bit) vs lane-level. sha3's repo may already
   have `_uint8x16_t` bit-level treatments — check before writing new.
3. **Bridge lemmas**: Neon needs vec128 analogues of the AVX2
   decomposition lemmas (bit_vec 128 over two halves instead of one
   bit_vec 256). The repr-append structure means serialize posts factor
   through `Seq.append` — expect to need an append-aware variant of
   `bit_vec_of_int_t_array` decomposition.
4. **to_bytes/from_bytes upgrade** (stretch): with _vst1q_bytes given a
   byte-level post (mirror _vst1q_bytes_u64's to_le_bytes form at i16),
   drop panic_free. If it cliffs, leave at parity — Portable/AVX2 accept
   it.
5. Per-stage clean rebuild + full `make all` + cargo simd128 tests +
   status doc + perf top-20 + Sprint 3 prompt + commit (no push).

## Sprint 3 (not this session)

- ntt.rs + compress.rs: branch-post opacity recipe (memory:
  feedback_layer2_branch_post_z3_unlock).
- barrett_reduce / montgomery_* ensures: REQUIRES MODELING
  `_vqdmulhq_n_s16` / `_vqdmulhq_s16` (saturating doubling multiply high:
  lane = sat16((2*a*b) >> 16); for ML-KEM operands no saturation occurs —
  model the general form, prove usage under bounds). Mirror portable's
  barrett/montgomery contracts; cross-validate the model with a Python
  bit-sim before F* iteration (ground-literal SIMD recipe).
- impl Operations contract pass on neon.rs (bare requires/ensures under
  `#[hax_lib::attributes]`, one-line dispatchers — check Track B
  discipline; the impl methods currently wrap `Self { low, high }`
  inline, which is the known anti-pattern for the combined impl_N query).
- Vector.Neon.{fst,fsti} + Ntt + Compress out of ADMIT_MODULES;
  `mod sampling` re-enable needs Karthik's explicit OK (CODE change).

## Constraints (unchanged from Sprint 1 — non-negotiable)

- NO runtime code changes (annotations/proof scaffolding only;
  rename-only/let-binding restructures are accepted precedent).
- Max 4 concurrent fstar.exe+z3 (make -j2); never pkill by name; never
  bulk-delete .checked (targeted rm at stage boundaries only).
- z3rlimit cap 800 (400 with split_queries); smtprofiling before ANY
  cliff claim; 30-60 min per-fn budget then mark user-followup.
- Never hand-edit extracted F* as a permanent fix (scratch iteration OK;
  backport verbatim into Rust fstar! blocks). cargo-hax 0.3.7 (opam
  switch hax-0.3.7); `python3 hax.py extract` in libcrux-ml-kem ~2.5 min.
- Ensures cite Hacspec_ml_kem.* / Traits.Spec.* / Spec.Utils.*, never
  Spec.MLKEM.
- The fstar MCP tools may not register in-session: use the curl wrapper
  /tmp/fp.sh against the proxy on port 3002 (skill §3); fstar_typecheck
  requires a `code` field (file content), see /tmp/fptc.sh pattern from
  Sprint 1.
- The proxy is SHARED across parallel agent sessions — build logs under
  ~/.fstar-mcp-logs/builds/ are not all yours; track your own build_ids.
- Param named `v` shadows F*'s `v` coercion in specs — rename to `vec`
  (portable convention) before annotating.
- Status report every 15 min to
  proofs/agent-status/agent-neon-sprint2-status.md.

## Exit criteria

- Vector.Neon.Serialize.fst out of ADMIT_MODULES, real query stats.
- Full-crate `make all` green; cargo simd128 tests green (run on this
  arm64 host natively).
- Status doc regenerated (expect Neon lax 64 → ~52); perf top-20
  refreshed; Sprint 3 prompt written; commit per `agent-mlkem:`
  convention (no push).
