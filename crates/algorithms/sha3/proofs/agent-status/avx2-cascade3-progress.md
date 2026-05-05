# AVX2 cascade3 progress log

Mission: close third quantifier-cascade source in
`Libcrux_sha3.Simd.Avx2.load_block` (forall-25 cliff). Worktree
`/Users/karthik/libcrux-sha3-avx2`, branch `avx2-cascade`.

## 2026-05-05, T+0 (start)

- Worktree clean, branch `avx2-cascade` confirmed off
  `sha3-byteform-migration`.
- Verified prereq commits inherited:
  - `7bb581f8b` – `[@@ "opaque_to_smt"]` on `createi`
    (visible in `specs/sha3/proofs/fstar/extraction/Hacspec_sha3.fst`).
  - `8203c9ace` – `get_lane_u64_post` SMTPat lemma
    (verified diff in `crates/utils/intrinsics/src/avx2_extract.rs`;
     the F* extraction will be regenerated below since the
     `crates/utils/intrinsics/proofs/fstar/extraction/` directory has
     not yet been populated in this worktree).
- Next sub-task: enable `--log_queries --z3refresh --query_stats`
  on the load_block hax options in
  `crates/algorithms/sha3/src/simd/avx2.rs`, run
  `bash crates/algorithms/sha3/hax.sh extract` to populate the F*
  extraction directories, then run `make check/...Avx2.fst` with
  `--admit_except` set on `load_block` to capture the failing
  `.smt2`.
- Blocker: none yet.
- ETA: ~10 min for extraction; ~5 min for first failing query
  identification.

## 2026-05-05, T+15 (after profiling)

- Extraction succeeded (`/tmp/avx2-extract.log` exit 0). Intrinsics
  F* in `crates/utils/intrinsics/proofs/fstar/extraction/` regenerated;
  `get_lane_u64_post` SMTPat lemma confirmed in extracted .fsti.
- `make check/Libcrux_sha3.Simd.Avx2.fst OTHERFLAGS="--admit_except
  Libcrux_sha3.Simd.Avx2.load_block"` reproduces the 4 failing
  sub-queries (qs 692-695 at line 1091 first per-iteration assert,
  q 796 at line 1164 second assert). Each cancels at 400/400 in
  ~80-100s.
- z3 qi.profile of `queries-Libcrux_sha3.Simd.Avx2-692.smt2` (1.7MB,
  the i=0 entry into the unrolled 4-lane assertion):
  - `k!61` 1,096,670 instantiations (max gen 11).
  - `refinement_interpretation_Tm_refine_cda1...` 560k (Slice fsti
    line 20:65 — array_from_fn index refinement).
  - `Tm_refine_8143...` 493k (Slice fsti line 20:7 — array_from_fn
    body).
  - `lemma_get_lane_u64_post` only 159k — both inherited fixes ARE
    biting (createi opaque, get_lane_u64 SMTPat).
- Z3 `trace=true` decoded `k!61` to the F* prelude axiom
  `;;fuel irrelevance` (Pulse/F* SMT prelude, declared right under
  `(declare-fun HasTypeFuel ...)`):
  ```
  (forall ((f Fuel) (x Term) (t Term))
   (! (= (HasTypeFuel (SFuel f) x t) (HasTypeZ x t))
    :pattern ((HasTypeFuel (SFuel f) x t))))
  ```
  No `:qid`, single pattern. Fires on every `HasTypeFuel (SFuel _)`
  in the goal — and the iterated `array_from_fn` refinement-interp
  axioms are the producer.
- This is upstream of hax-lib (it lives in the F* SMT prelude).
  The fix is to suppress the array_from_fn-tagged refinements that
  funnel `HasTypeFuel (SFuel _)` terms into k!61. Trying option (c)
  from brief: per-fn `--using_facts_from '* -Rust_primitives.Slice.array_from_fn'`.
- Next sub-task: enable that filter, drop the debug flags, retry.
- Blocker: none.
- ETA: ~5 min retry.

## 2026-05-05, T+30 (filter results — early)

- Re-extracted with
  `--using_facts_from '* -Rust_primitives.Slice.array_from_fn'`
  on the load_block hax options. Debug flags removed.
- `make check/Libcrux_sha3.Simd.Avx2.fst OTHERFLAGS="--admit_except
  load_block"` past sub-query 692 (the previously-failing first
  iteration assert) — succeeded at ~80ms, used_rlimit 0.56. Same
  for 690 / 691.
- Still running for the remaining ~200+ sub-queries; will report
  final EXIT and any remaining failures.
