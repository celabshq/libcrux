# Next session — post-ntt_multiply follow-ups (pick one)

Context: AVX2 `ntt_multiply` CLOSED at `1772bb715` (AVX2 NTT layer 0-lax;
`lemma_ntt_multiply_chunk_commutes` proven; both `op_ntt_multiply` wrappers
fully verified). Read `ntt-multiply-design-2026-06-03.md` §Session-learnings
and memory `feedback_ground_literal_simd_proofs` before touching SIMD proofs.

Follow-ups in priority order:

1. **mlkem.rs extraction filter** (cheap, flips 36 Unverified→PF): add
   `-libcrux_ml_kem::mlkem::alloc::**` (and `::rand` if needed) to
   libcrux-ml-kem/hax.py includes — mirrors the existing incremental::alloc
   exclusions. Same pass: consider `pqcp.rs` (16 fns) and `lib.rs` (3).
2. **Split Commute.Chunk.fst** (~3,000 lines, ~2,500 queries): every edit
   costs a full re-verify (~5 min warm / 26 min cold) + invalidates
   Ntt_bridge/Invert_ntt_bridge/consumers. Natural seams: Chunk.Base
   (scalar FE/mod-q cores + lifts), Chunk.Layers (branch lemmas + layer
   bridges + ntt_multiply block), Chunk.Poly (Phase-7a/7b per-poly lifts).
   Cascade: Rust fstar! references `Hacspec_ml_kem.Commute.Chunk.lemma_*`
   in avx2.rs/portable.rs (rename → re-extract); budget a full downstream
   rebuild.
3. **montgomery_reduce_i32s body proof** (drop panic_free): spec is now the
   corrected lane32 form (requires True + bounds ==> 8 ground
   mont_red_i32_lane triples; sim-validated). Body proof mirrors
   montgomery_multiply_m128i_by_constants + needs per-lane axioms for
   srli_epi32<16>/srai_epi32<16>/sub_epi16 cross-lane forms (same accepted
   pattern; keep them index-parameterized + ground per
   feedback_ground_literal_simd_proofs).
4. **ind_cca/incremental.rs proofs** (45 fns extracted-but-lax).
5. **D6.5 intrinsics-trust sprint** (prove the admit() stubs in
   avx2_extract.rs fstar::replace/before blocks — would also discharge the
   12 new ntt_multiply per-lane axioms' trust).

Environment: cargo-hax 0.3.7; all F* via fstar_build MCP; iterate leaf
changes on extracted .fst first; `--admit_except` takes ONE name; rm tainted
.checked + finish with full no-admit builds (real `used rlimit` lines).
NOTE: `libcrux-ml-kem/verification_result.txt` has uncommitted pre-existing
modifications from another session — left untouched, ask Karthik.
