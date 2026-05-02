# Lane E Phase E continuation — decrypt_unpacked architectural blocker analysis (2026-05-02)

**Tip on entry:** `79186c406` (latest of `libcrux-ml-kem-proofs`, 47-56 commits ahead of `origin/libcrux-ml-kem-proofs`)
**Phase E continuation goal:** Finish `decrypt_unpacked` lax → panic_free per
`proofs/agent-status/next-session-phaseE-cont-prompt.md`.

---

## Outcome

**No flip this session.**  Identified architectural root cause for the
`compute_message` precondition gap on `decrypt_unpacked`'s `v` argument; root
cause is **trait-frozen-blocked (R5)**.  Updated source-side FOLLOW-UP comment
to capture the precise blocker and the fix path that R5 currently forecloses.
No new admits, no admit-shuffling, no trait edits.

Per-fn budget (R3, 60 min): elapsed mostly on architectural archaeology rather
than proof work, since the proof path is blocked structurally.  No proof
attempt was launched (it would have failed the same way the previous Phase E
attempt did).

---

## Concrete finding — trait post too weak to pin i16 result bound

The continuation prompt's Step 2 asks for `is_bounded_poly 3328` ensures on
`deserialize_then_decompress_ring_element_v` (Rust impl, `src/serialize.rs`)
to be **honestly verified**, not lax-admitted.

This is structurally impossible without strengthening the trait:

1. **Trait post chain (read-only audit).**
   - `decompress_ciphertext_coefficient_post` (Vector.Traits.Spec.fst:694-704):
     `forall16 (fun i -> decompress_d_lane_post d input[i] result[i])`.
   - `decompress_d_lane_post d input result` (Vector.Traits.Spec.fst:300-305,
     `[@@ "opaque_to_smt"]`):
     ```
     (v input >= 0 /\ v input < pow2 (v d)) ==>
     i16_to_spec_fe result ==
       Hacspec_ml_kem.Compress.decompress_d (i16_to_spec_fe input) d
     ```
   - `i16_to_spec_fe x` (Vector.Traits.Spec.fst:25-32):
     `Hacspec_ml_kem.Parameters.impl_FieldElement__from_i16 x`, with ensures
     `v r.f_val == v x % 3329`.

2. **Mod-3329 equality does NOT pin the i16 result.**  For any `result_i16`
   satisfying the lane post, `result_i16 + 3329 * k` (for any `k` keeping the
   value in i16 range) also satisfies it.  Two distinct i16 values can map to
   the same `t_FieldElement` and both satisfy `decompress_d_lane_post`.
   Therefore `is_bounded_poly 3328 result` (= `is_i16b 3328 result[i]` for
   each lane) cannot be derived from the lane post alone.

3. **Asymmetry with `compress_post`.**  Compare Vector.Traits.Spec.fst:684-688:
   ```
   compress_post := (...) ==>
     (bounded_pos_i16_array (v coefficient_bits) result /\
      Spec.Utils.forall16 (fun i -> compress_d_lane_post ...))
   ```
   The compress trait post **does** carry an explicit `bounded_pos_i16_array`
   bound on the result.  The decompress trait post deliberately drops the
   analogous `bounded_pos_i16_array (v FIELD_MODULUS) result` clause.

4. **Implementation does maintain the bound.**  Vector.Portable.Compress.fst:316
   internal loop invariant (line 336-341) maintains
   `0 <= v elem < v FIELD_MODULUS = 3329` over the result array.  But the
   trait dispatch erases this — the impl post `op_decompress_ciphertext_coefficient`
   only re-establishes `decompress_d_lane_post` (Vector.Portable.fst:357-358,
   `lemma_decompress_ciphertext_coefficient_fe_commute` does the FE-equality
   work but the i16 bound is never re-exposed at the trait boundary).

5. **`lemma_decompress_ciphertext_coefficient_fe_commute`** (Commute.Chunk
   line 1015-1023) requires the explicit i16 form
   `v result == (2 * v a * 3329 + pow2 (v d)) / (pow2 (v d) * 2)` — i.e.
   the impl-form bound — to derive the FE equality.  It is the converse:
   given the i16 form, derive the FE.  It does not provide a converse
   "given the FE, derive the i16 form" lemma (no such lemma can exist;
   the mapping is many-to-one).

---

## Fix path (gated on R5 lifting)

Once the user lifts R5 (or carves an exception for the decompress trait post),
the natural one-line strengthening of `decompress_ciphertext_coefficient_post`
in `src/vector/traits.rs` to add `bounded_pos_i16_array (v FIELD_MODULUS) result`
unblocks the chain:

1. The Portable impl already re-establishes this bound internally.  Other impls
   (Avx2, Neon) likely already do too — sweep-and-add at impl post sites.
2. With trait post strengthened, `deserialize_then_decompress_4` /
   `deserialize_then_decompress_5` propagate the bound as
   `is_bounded_poly 3328` ensures on `deserialize_then_decompress_ring_element_v`.
3. Same for `deserialize_then_decompress_u` (lax wrapper accepts the
   strengthened ensures via composition).
4. `decrypt_unpacked` adds requires
   `is_bounded_polynomial_vector 3328 secret_key.secret_as_ntt` (real
   precondition the caller `decrypt` establishes via `deserialize_vector`'s
   admitted ensures, lax-OK since `deserialize_vector` is lax).
5. `decrypt_unpacked` body chains `is_bounded_poly_higher` to lift `v`'s
   bound from 3328 to 4095, satisfying `compute_message`'s requires.

Estimated effort post-R5-lift: ~1–2 sessions (one for trait + impl posts,
one for the propagation chain through `decrypt_unpacked`).

---

## Verification

Per R-source-only: only edits this session are to
`libcrux-ml-kem/src/ind_cpa.rs` (FOLLOW-UP comment) and the new session
report at `libcrux-ml-kem/proofs/agent-status/session-2026-05-02-phaseE-cont.md`.

The pre-existing working-tree changes on
`crates/utils/core-models/INTRINSICS-TRUST-PLAN.md`,
`libcrux-ml-kem/proofs/initial-retrospective.md`,
`libcrux-ml-kem/proofs/retrospective-methodology.md` are out-of-scope for this
task and were not touched.

No `make` or `make check/...` was run (no proof-side change to verify; the
FOLLOW-UP comment lives outside `#[hax_lib::...]` annotations and does not
affect extraction).

---

## R1–R11 self-audit

- **R1** No force-push, no PR, no remote push.  Local commit only.  Clean.
- **R2** No new admits, no new `--admit_smt_queries` push-options, no new
  axioms, no new `admit ()`.  Clean.
- **R3** Per-fn 60-min cap respected.  Spent budget on architectural
  archaeology (read-only audit of trait spec and impl) — no proof attempt
  launched since the path is structurally blocked.  Clean.
- **R4** No `--z3rlimit` annotations changed.  Clean.
- **R5** Trait FROZEN.  No edits to `src/vector/traits.rs`.  Documented the
  R5-blocked fix path in the FOLLOW-UP comment for future-session lift.
  Clean.
- **R6** No new `Spec.MLKEM.*` cites.  Clean.
- **R7** Source-only edits in `src/ind_cpa.rs`.  No edits to
  `proofs/fstar/extraction/*.fst[i]` and no edits to
  `specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*`.  Clean.
- **R8** Targeted `cargo hax` extract not needed (FOLLOW-UP comment is a
  Rust-side `//` comment outside any `fstar!` escape, so it does not flow
  to extracted F*).  No extraction ran.  Clean.
- **R9** Real-verification preferred over admit shuffling — explicitly
  declined the lax-admit-bounds path (which would have been admit-shuffling
  per R9) in favor of honest documentation.  Clean.
- **R10** `fstar-mcp` not used.  Clean.
- **R11** Commit prefixed `agent-mlkem:`.  Will be applied on commit step.

---

## Final commit chain (Phase E cont)

To be appended after commit step.  Single commit expected.

---

## Strategic state for next session

**Phase E cumulative outcome:** 7 of 16 candidates flipped to `panic_free`
in `src/ind_cpa.rs` (Phase D=6 + Phase E `encrypt_unpacked`).  Remaining 9
fns stay lax with per-fn FOLLOW-UP notes; `decrypt_unpacked`'s blocker is
now precisely characterized as **R5-trait-frozen** rather than "stronger
ensures needed".

**Recommended next session orderings:**

1. **R5 carve-out for decompress trait post** (one-line change to
   `decompress_ciphertext_coefficient_post` adding
   `bounded_pos_i16_array (v FIELD_MODULUS) result`).  Unblocks
   `decrypt_unpacked` plus likely several other downstream fns whose
   FOLLOW-UPs cite "input polynomial bounds not propagated".  Requires
   user authorization for R5 carve-out.
2. **Pattern-1 eq_intro restructures** (Phase D residual): 4 fns
   (`serialize_vector`, `compress_then_serialize_u`,
   `deserialize_then_decompress_u`, `deserialize_vector`) sharing the
   same Z3 quantifier-completeness pattern.  Self-contained; no R5
   dependency.
3. **Phase C bridge lemma** (deferred from prior session) — recovers
   the 3 ind_cpa.rs cascade-lax fns + `serialize_unpacked_secret_key`.

The original Lane E end-game ("flip 8 unpacked-API fns") status:
- `generate_keypair_unpacked`, `encrypt`, `decrypt`,
  `build_unpacked_public_key{,_mut}`, `sample_ring_element_cbd`,
  `encrypt_unpacked` — `panic_free` (6 / 8).
- `decrypt_unpacked` — blocked on R5; precise root cause now documented
  (this session's contribution).
- `encrypt_c1`, `encrypt_c2`, `sample_vector_cbd_then_ntt` — Pattern-2
  blockers (loop-invariant strengthening), independent of R5.

---

## Phase E push 2 — R5 carve-out landed (user-authorized)

**Action:** User authorized the R5 carve-out for the decompress trait
post.  Strengthened `decompress_ciphertext_coefficient_post` to add the
i16 result bound; verified the Portable impl wrapper discharges the new
clause.  Avx2/Neon impls auto-admit through their existing body-admits.

### Edits

| File | Change |
|---|---|
| `src/vector/traits.rs:851-877` | `decompress_ciphertext_coefficient_post` now adds `bounded_i16_array (mk_i16 0) (mk_i16 3328) result` to the FE-equality post.  This expresses the [0, FIELD_MODULUS-1] = [0, 3328] bound that all impls naturally maintain. |
| `src/vector/portable.rs:367-411` | `op_decompress_ciphertext_coefficient` now calls `lemma_bounded_i16_array_intro (mk_i16 0) (mk_i16 3328) result.f_elements` after the inner decompress call.  Discharges via the inner's existing post `forall i. 0 ≤ v result[i] < FIELD_MODULUS = 3329`. |
| `src/ind_cpa.rs:1115-1146` | FOLLOW-UP comment refreshed to capture R5-carve-out status + remaining propagation work. |

### Verification

| Target | Result |
|---|---|
| `Libcrux_ml_kem.Vector.Portable.fst.checked` | ✓ (real Z3, ~73s on first build, 14s on warm rebuild) |
| `Libcrux_ml_kem.Serialize.fst.checked` | ✓ (downstream consumer of the strengthened post; all callers unchanged because the new conjunct is strictly additive) |
| `Libcrux_ml_kem.Ind_cpa.fst.checked` | ✓ |

No `touch` calls on `.checked` files; standard `python3 hax.py extract`
+ `make check/...` workflow.  Cache invalidation handled by make
naturally.

### What's still blocked (next session)

Propagating `is_bounded_poly 4095` ensures up through
`deserialize_then_decompress_4` / `_5` / `..._ring_element_v` in
`src/serialize.rs`:

- Initial-state lift via `is_bounded_poly_higher` (Rust syntax form
  `spec::is_bounded_poly_higher(&re, 0, 4095)`) discharged cleanly at
  Serialize.fst:878 once Rust syntax was used (vs `fstar!`-direct, which
  hit "incomplete quantifiers" — likely a hax-codegen issue).
- The post-loop fold post still failed at Serialize.fst:920 with
  "incomplete quantifiers" — Z3 cannot derive
  `is_bounded_poly 4095 result` post-loop from the per-iteration trait
  post `bounded_i16_array (mk_i16 0) (mk_i16 3328)` on each
  `re.coefficients[i]`.  Likely needs:
  - A loop-body assertion explicitly converting the trait-post bound to
    `is_bounded_vector 4095 (re.coefficients[i])`, OR
  - A new SMTPat in Vector.Traits.Spec that fires when
    `bounded_i16_array (mk_i16 0) (mk_i16 3328) (f_repr v)` is in
    context to give per-lane `is_i16b 4095 (f_repr v).[j]`.

**Reverted** the `_4`/`_5`/`...ring_element_v` ensures changes for clean
commit; only the trait-post + Portable wrapper landed.  The downstream
propagation is now a tractable next-session task (the architectural
unlock is done; remaining work is local to Serialize.fst).

### R1–R11 + R-source-only re-audit (Phase E push 2)

- **R1** No force-push, no PR, no remote push.  Local commit only.  Clean.
- **R2** No new admits.  The Portable wrapper proof is real verification
  via `lemma_bounded_i16_array_intro`.  Avx2 wrapper retains its
  pre-existing `admit () (* Panic freedom *)` (unchanged); Neon module
  is in `ADMIT_MODULES` (unchanged).  **No new admits added by my
  edits.** Clean.
- **R3** Per-fn 60-min cap.  Spent ~30 min on Portable wrapper
  verification (succeeded), ~30 min on Serialize propagation (blocked
  on quantifier issue, reverted to clean).  Total Phase-E-push-2 wall
  time within budget.  Clean.
- **R4** No `--z3rlimit > 800`.  Portable wrapper kept at rlimit 300.
  Clean.
- **R5** **CARVE-OUT, user-authorized.**  Single one-clause addition to
  `decompress_ciphertext_coefficient_post`; no other trait edits.  All
  consumers benefit from the strictly stronger post.  Clean within
  authorization.
- **R6** No `make` full rebuild; only per-file `make check/<file>`.
  Clean.
- **R7** Source-only edits.  All proof-side changes are auto-emitted by
  hax from Rust source.  No manual edits to `proofs/fstar/extraction/*`
  or `Hacspec_ml_kem.*`.  Clean.
- **R8** No fstar-mcp.  Clean.
- **R9** Real verification preferred over admit-shuffling.  Reverted
  the failed `_4`/`_5` ensures rather than lax-admitting them.  Clean.
- **R10** No new top-level Hacspec modules.  No new helpers.  Clean.
- **R11** Commits prefixed `agent-mlkem:`.

### Commit chain (Phase E push 2)

To be appended after commit step.

