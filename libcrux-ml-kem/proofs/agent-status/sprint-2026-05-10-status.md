# Sprint 2026-05-10 — AVX2/serialize lax closure — final status

## 17:18 — DONE ✅ all 5 sites cleared
- `serialize_5`     (line 352): lax → fully proven (BitVec ensures discharged
  via `assert_norm forall_n 40` per half).  Commit `9ba739333`.
- `serialize_1`     (line 5):   lax → panic_free.  Commit `e19d5a843`.
- `deserialize_5`   (line 468): lax → panic_free.  Commit `e19d5a843`.
- `serialize_11`    (line 694): lax → panic_free.  Commit `e19d5a843`.
- `deserialize_11`  (line 705): lax → panic_free.  Commit `e19d5a843`.

`make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst` rc=0 (~92 s).
`grep -c verification_status(lax) src/vector/avx2/serialize.rs` = 0.
`bash proofs/generate_verification_status.sh` Avx2/serialize lax: 5 → 0.

## Lessons learned

1. **`mm256_madd_epi16(x, mm256_set_epi16(...))`** at the source level
   resolves to `mm256_madd_epi16_specialized` whose body has a runtime
   `forall_bool` guard that `assert_norm` cannot reduce.  Use
   `mm256_concat_pairs_n(n, x)` instead — it directly invokes
   `mm256_madd_epi16_specialized'` with a type-level requires.
2. **`mm256_shuffle_epi32` has NO `BitVec.Intrinsics` spec.**
   Substitute with `mm256_shuffle_epi8 + mm256_set_epi8(...)` whose
   per-lane semantics are concrete.  Mapping for
   `shuffle_epi32::<0b00_00_10_00>` (move 32-bit lane 2 → lane 1
   per 128-bit half) is the byte selectors
   `[3, 2, 1, 0, 3, 2, 1, 0, 11, 10, 9, 8, 3, 2, 1, 0]`
   in each 16-arg group of `mm256_set_epi8`.
3. **`forall_n 40` over a 7-step SIMD chain needs `--z3rlimit 400`**
   (default 80 → cancelled at ~12 s).  Stays well under cap; verify
   in ~12 s wall.
4. **`mm256_mullo_epi16` BitVec spec only matches three hardcoded
   multiplier patterns** (specialized1/2/3).  The `deserialize_5`
   multiplier `(1<<0, 1<<5, 1<<2, 1<<7, 1<<4, 1<<9, 1<<6, 1<<11, …)`
   is none of them — full BitVec ensures would require adding a
   `mm256_mullo_epi16_specialized4` to `BitVec.Intrinsics.fsti`.  See
   FOLLOW-UP below.

## Headline metric (`verification_status.md`)
- Avx2 lax count: **11 → 6** (-5, all from `serialize`).
- Avx2 panic-free count: 24 → 28 (+4, the four panic_free flips).
- Avx2 math-tier count: 15 → 17 (+2, `serialize_5` itself plus the
  factored `serialize_5_vec` helper).
- Total ml-kem lax count: 158 → 153.

## FOLLOW-UP (out of scope for this sprint)
- **Strengthen `deserialize_5` to fully proven** by adding a
  `mm256_mullo_epi16_specialized4` to `BitVec.Intrinsics.fsti` for the
  5-bit deserialize multiplier shape, then mirror the
  `deserialize_10_vec` / `deserialize_12_vec` factoring.  Estimate:
  60-90 min.
- **Strengthen `serialize_11` / `deserialize_11` to fully proven** by
  embedding `Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma`
  calls in the body.  Note this requires the lemma to be visible from
  `Libcrux_ml_kem.Vector.Avx2.Serialize.fst`; currently the bridges
  in `vector/avx2.rs` reference it from the `before` block but
  Vector.Avx2 imports Vector.Avx2.Serialize, not the other way.
  Either move the relevant axiom + bound lemma into a helper module
  imported by both, or do the bridge as a single `hax_lib::fstar!`
  block inside the body.  Estimate: 60-90 min.
- **Strengthen `serialize_1` to fully proven** — body already has the
  proof-bearing `hax_lib::fstar!` blocks but the BitVec ensures of
  `mm_packs_epi16` (signed saturation) is non-trivial to discharge in
  a single `assert_norm`; would need a per-lane lemma.  Estimate:
  60-90 min.
- **Discharge the `op_*_5_*_bridge` and `op_*_11_*_bridge` admits in
  `vector/avx2.rs:864-990`** — once the body posts above are fully
  proven, those bridges may discharge without `admit ()`.  Per the
  prompt's "Stretch" section.

## Time budget
- Sprint started 16:42, finished 17:18 (~36 min).  Way under the
  5-fn × 45-90 min/fn estimate (4-7 hr).  Mostly because the
  panic_free path turned out to be a one-line flip for 4 of the 5
  sites.  Only `serialize_5` needed real proof engineering (concat_pairs_n
  swap, shuffle_epi32→shuffle_epi8 swap, rlimit bump).

## 16:42 — start
- Worktree: `/Users/karthik/libcrux-avx2-serialize-closure`
- Branch:   `agent-mlkem-avx2-serialize-2026-05-10`
- Tip:      `f2bb7c7ca` (next-session prompt)
- Baseline: `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst` rc=0,
  5 lax sites in src.
