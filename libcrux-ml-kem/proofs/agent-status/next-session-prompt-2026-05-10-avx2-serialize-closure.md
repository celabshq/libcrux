# Next-session prompt — close the AVX2/serialize lax family

**Branch:** `libcrux-ml-kem-proofs` (or fresh worktree, see Pre-session step 1)
**Tip on entry:** `cc4d8305f` (or later — sprint 2026-05-09 rollup)
**Scope (5 sites, all in `src/vector/avx2/serialize.rs`):**

| Line | Function | Mechanism |
|---|---|---|
| 5   | `serialize_1`    | `verification_status(lax)` |
| 352 | `serialize_5`    | `verification_status(lax)` |
| 468 | `deserialize_5`  | `verification_status(lax)` |
| 694 | `serialize_11`   | `verification_status(lax)` (delegates to PortableVector) |
| 705 | `deserialize_11` | `verification_status(lax)` (delegates to PortableVector) |

**Out of scope:** everything else.  In particular: the upstream `op_serialize_*`
wrappers in `src/vector/avx2.rs` (lines 1067-1102) and the admitted bridge
lemmas in the `before` block of the same file (`op_serialize_5_pre_bridge`,
`op_serialize_5_post_bridge`, `op_deserialize_5_post_bridge`,
`op_serialize_11_pre_bridge`, `op_serialize_11_post_bridge`,
`op_deserialize_11_post_bridge`).  Once the body proofs land, those bridges
become discharge-able from the strengthened body posts — but that's a
follow-up task, not part of this sprint.

## Branch hygiene — mandatory

The user is working in `/Users/karthik/libcrux-trait-opacify` on parallel
tasks.  **Do NOT operate on the shared worktree.** Per
`feedback_branch_means_worktree`:

```bash
# Pick a fresh worktree directory NEXT TO the existing one, not inside it
git -C /Users/karthik/libcrux-trait-opacify worktree add \
    /Users/karthik/libcrux-avx2-serialize-closure \
    -b agent-mlkem-avx2-serialize-2026-05-10
cd /Users/karthik/libcrux-avx2-serialize-closure/libcrux-ml-kem
```

All edits, extraction runs, F* checks, and commits happen in that worktree.
Use `git -C /Users/karthik/libcrux-avx2-serialize-closure …` for git ops, or
`cd` once at session start.  When the sprint closes, the user merges or
cherry-picks the resulting branch back to `libcrux-ml-kem-proofs`.

If the worktree directory already exists (prior abandoned session), prompt
the user before reusing — do not blindly `git worktree remove`.

## Goal

Eliminate all 5 `verification_status(lax)` sites in
`src/vector/avx2/serialize.rs`.  Each site flips lax → at-least-`panic_free`
(body verified, ensures admitted) — ideally to fully proven (BitVec ensures
discharged).  `panic_free` with the existing BitVec ensures is acceptable
**only** if it implies the existing ensures becomes admit-via-panic-free —
in which case downstream bridges in `vector/avx2.rs` may need the existing
`admit ()` retained but the chain is no longer "lax + admitted bridge".

**Definition of done:**

```bash
$ grep -c "verification_status(lax)" src/vector/avx2/serialize.rs
# expect 0

$ make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst
# rc=0

$ bash proofs/generate_verification_status.sh
# Avx2/serialize lax count: 0  (was 5)
```

## Read first (non-negotiable)

1. **`~/.claude/skills/fstar-for-libcrux/README.md`** — Rules 1-8, especially
   Rules 5 (pipe make to log + grep, never `Read` full make log) and 7
   (`fstar-mcp` for symbol lookups, full `make` for verification gate).

2. **`MEMORY.md`** entries:
   - `feedback_panic_free_vs_lax` — `panic_free` admits ensures, `lax` admits
     everything.  Default flip target = `panic_free`.
   - `feedback_proof_debug_budget` — 30-60 min hard cap per function;
     beyond that, mark FOLLOW-UP.  Per-fn budget for this sprint:
     45-60 min for `_5`/`_11` siblings, 60-90 min for `serialize_1`
     (1-bit special).
   - `feedback_branch_means_worktree` — see "Branch hygiene" above.
   - `feedback_grep_make_output` — never `Read` a full `make` log.
   - `feedback_rlimit_cap_800` — hard cap 800 (≤400 with split_queries).
   - `feedback_smtpat_percent_above_trait` — keep SMTPats opaque-atom only.

3. **Recent context:**
   - `commit 107c76641` ("USER-9b: AVX2 5-bit serialize/deserialize SIMD↔BitVec
     bridge") — the prior commit author who explicitly chose
     "verification_status(lax) on body but gain BitVec pre/post in their
     signatures … SIMD body is treated as a signature-level axiom".  This
     sprint REVERSES that decision per the 2026-05-09 rollup discussion
     (user wants below-traits.rs fully verified).
   - `commit cc4d8305f` ("agent-mlkem: sprint 2026-05-09 rollup …") —
     summary of why these 5 sites were deferred from the broader sprint.
     Read `proofs/agent-status/sprint-2026-05-09-rollup.md` for context
     ("What deferred (and why)" → "Stage 2").

## The 5 fns and their templates

The trick the user hinted at: **the portable equivalents tell you what
needs to be proven (the BitVec invariants); the AVX2 siblings tell you
how to prove it (the SIMD-tactic recipe).**

### Site 1: `serialize_5` (line 352)

**Template:** `serialize_4` (line 185) — proven panic_free, same intrinsic
family, simpler shape.  Also `serialize_10` (line 532) and `serialize_12`
(line 721) for non-power-of-2-byte widths.

**What's already there:**
- BitVec pre: `forall (i: nat{i < 256}). i % 16 < 5 || vector i = 0`
- BitVec ensures: `forall (i: nat{i < 80}). bit_vec_of_int_t_array r 8 i ==
  vector ((i/5) * 16 + i%5)`
- Body: `mm256_madd_epi16` (5-bit-specific multiply) → `mm256_sllv_epi32` →
  `mm256_srli_epi64::<22>` → `mm256_shuffle_epi32` → `mm256_sllv_epi32`
  again → `mm256_srli_epi64::<12>` → cast/extract → `mm_storeu_bytes_si128`
  twice → `serialized[0..10].try_into().unwrap()`.

**What to add:**
1. `#[hax_lib::fstar::options("--ext context_pruning --split_queries always")]`
   (or keep with `--z3rlimit 400`).
2. Drop `verification_status(lax)`.
3. Factor an inner `serialize_5_vec(vector) -> (Vec128, Vec128)` helper
   carrying the 80-bit-pair invariant (mirror `serialize_10_vec` at
   line 542 + `serialize_12_vec` at line 732).  Use
   `introduce forall (i:nat{i<40}). lower_8_ i = vector ((i/5)*16 + i%5)
   with assert_norm (BitVec.Utils.forall_n 40 (fun i -> ...))` at the end
   of the helper.
4. Final byte-store handles `mm_storeu_bytes_si128` panic-freedom.
5. The `serialized[0..10].try_into().unwrap()` may need a length assert.

**Expected difficulty:** medium.  Bigger `assert_norm` (5-bit packing has
fewer lane-pairs per 32-bit slot than 4 or 12).

### Site 2: `deserialize_5` (line 468)

**Template:** `deserialize_4` (line 243), `deserialize_10` (line 626),
`deserialize_12` (line 773).

**What's already there:**
- BitVec pre: `Seq.length bytes == 10`
- BitVec ensures: `forall (i: nat{i < 256}). $result i = (if i % 16 >= 5
  then 0 else …)`
- Body: `mm_set_epi8` of bytes → `mm256_si256_from_two_si128` → big
  `mm256_shuffle_epi8` → `mm256_mullo_epi16` (5-bit multiply mask) →
  `mm256_srli_epi16::<11>` → `mm256_and_si256(_, mm256_set1_epi16((1<<5)-1))`.

**What to add:**
1. Factor inner `deserialize_5_vec(lower_coefficients0, upper_coefficients0)
   -> Vec256` helper (mirror `deserialize_10_vec` line 636,
   `deserialize_12_vec` line 783).  Mark with
   `#[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]` per the existing
   pattern.
2. Inside the helper, end with
   `hax_lib::fstar!(r#"assert_norm(BitVec.Utils.forall256 (fun i -> ...))`#)`
   that pins the BitVec output to the input `lower_coefficients0` /
   `upper_coefficients0` per-byte view.
3. Outer fn passes through `mm_loadu_si128(&bytes[0..16])` and
   `mm_loadu_si128(&bytes[5..21])` (or whatever offsets the SIMD requires).

**Expected difficulty:** medium.

### Site 3: `serialize_11` (line 694) — DECISION POINT

**Current body:**
```rust
pub(crate) fn serialize_11(vector: Vec256) -> [u8; 22] {
    let mut array = [0i16; 16];
    mm256_storeu_si256_i16(&mut array, vector);
    let input = PortableVector::from_i16_array(&array);
    PortableVector::serialize_11(input)
}
```

**Two paths:**

**(A) Pure SIMD body.**  Replace the PortableVector delegation with a
real SIMD chain like `serialize_10` (line 532) / `serialize_12` (line 721).
The 11-bit packing is irregular — see `compress_then_serialize_11` in
`src/serialize.rs` for the chunk size (22 bytes per 16 lanes).  This is
the cleanest option proof-wise but the most code.

**(B) Keep delegation, prove the bridge inline.**  Add hax_lib::fstar!
calls inside the body that invoke
`Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma`
+ `lemma_vec256_lane_bounded ${vector} 11 i` to bridge:
- input: `forall i. i % 16 < 11 || vector i = 0` ⇒
  `forall j. 0 <= array[j] < 2048` (PortableVector::serialize_11's pre).
- output: PortableVector::serialize_11's BitVec post ⇒ this fn's
  BitVec post (via the SAME lemma in reverse).

These bridge lemmas already exist parametrically and are admitted in the
`before` block of `vector/avx2.rs` (lines 864-902).  Path (B) effectively
moves them into the body, where they may be proven against the actual
intrinsic specs in `core_models`.

**Decision criterion:** if
`bit_vec_of_int_t_array_vec256_as_i16x16_lemma` is genuinely proven in
`core_models` (i.e., not itself an `admit ()`), prefer **(B)** — it
inherits the proof.  If it's admitted, **(A)** is the only path that
genuinely closes this site.  Check first:

```bash
grep -A 3 "bit_vec_of_int_t_array_vec256_as_i16x16_lemma" \
    /Users/karthik/.cargo/registry/src/index.crates.io-*/core-models-*/proofs/fstar/extraction/Libcrux_intrinsics.Avx2_extract.fsti \
    /Users/karthik/.cargo/registry/src/index.crates.io-*/core-models-*/src/avx2_extract.rs
```

**Expected difficulty:** path (B) ~45 min if lemma is real; path (A)
~90 min.

### Site 4: `deserialize_11` (line 705)

Symmetric to `serialize_11`; same decision point.  `PortableVector::
deserialize_11` is at `src/vector/portable/serialize.rs:768` (proven
panic_free).  Bridge lemmas: `op_deserialize_11_post_bridge` (admitted in
`vector/avx2.rs:901`).

### Site 5: `serialize_1` (line 5)

**Templates:** `serialize_4` for general structure; `mm_movemask_epi8_bv`
+ `mm_packs_epi16` are 1-bit-specific so the inner-loop structure differs.

The body already has TWO proof-bearing `hax_lib::fstar!` blocks (lines
50-61 and 69-73) that establish the BitVec view of `bits_packed` and
the high-byte split of the i32.  The only thing missing for panic_free
is the `bits_packed >> 8` shift not overflowing — which it can't, since
`bits_packed: i32`.

**What to do:**
1. Drop `verification_status(lax)`.
2. Add `--ext context_pruning --compat_pre_core 0` (already there).
3. Try a clean rebuild.  If subtyping fails on the `as u8` casts of
   `bits_packed` and `bits_packed >> 8`, surface the failure — it may
   point to a missing trivial bound assertion.

**Expected difficulty:** low-medium.  Most of the proof body is already
written.  Risk is around the byte-cast subtyping checks.

## Stage acceptance + commit hygiene

Per fn:
1. `python3 hax.py extract` (re-extract).
2. `cd proofs/fstar/extraction && rm -f .depend && make
   check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst > /tmp/avx2-ser.log 2>&1;
   echo rc=$?` (NEVER `Read /tmp/avx2-ser.log` in full — grep instead).
3. Once rc=0, commit per fn or per pair (e.g., `_5` + `_5`-deser together).

Suggested commit messages:
- `agent-mlkem: discharge serialize_5 SIMD body (lax → panic_free, mirror serialize_4)`
- `agent-mlkem: discharge deserialize_5 SIMD body (lax → panic_free, mirror deserialize_10)`
- `agent-mlkem: discharge serialize_11/deserialize_11 via inline bridge (lax → panic_free)`
- `agent-mlkem: discharge serialize_1 SIMD body (lax → panic_free)`

Final rollup commit if multiple sites land:
- `agent-mlkem: sprint 2026-05-10 — AVX2/serialize 5/5 lax → panic_free`

If only some sites land within budget, document deferred sites in
`proofs/agent-status/sprint-2026-05-10-rollup.md` per
`feedback_agent_status_reports`.

## Pre-session checklist

- [ ] Worktree created: `git worktree add /Users/karthik/libcrux-avx2-serialize-closure
      -b agent-mlkem-avx2-serialize-2026-05-10` from the shared repo.
      Confirm `pwd` shows the new worktree before any edits.
- [ ] Tip: `git log -1 --oneline` shows `cc4d8305f` or later.
- [ ] Read `fstar-for-libcrux` skill.
- [ ] Read MEMORY.md entries listed above.
- [ ] Run baseline `bash proofs/generate_verification_status.sh` and
      record Avx2/serialize lax count (expected: 5).
- [ ] Run `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst` from
      `proofs/fstar/extraction/` to confirm baseline rc=0 (with current
      lax markers).
- [ ] Pick ONE site to start with — recommended order:
      `serialize_5` (medium, exemplar) → `deserialize_5` (parallel) →
      `serialize_11`/`deserialize_11` (decision point) → `serialize_1` (last).

## Status reports (live)

Per `feedback_agent_status_reports`: every 15 min, append a 3-line status
to `proofs/agent-status/sprint-2026-05-10-status.md`:
- Current site (which fn).
- Blocker if any (specific Z3 query / lemma).
- ETA for current site.

## Stretch — bridge discharge in vector/avx2.rs

If all 5 sites land cleanly with budget remaining: examine the admitted
`op_serialize_5_pre_bridge` etc. in `src/vector/avx2.rs:864-902`.  Now
that the SIMD body posts are genuinely proven, the bridges may discharge
without `admit ()`.  Each successful discharge there flips one more
`Avx2/vector` audit-line entry from "admit() in before block" to
"proven".  Likely 30-60 min per bridge.

## Out-of-scope / explicit non-goals

- The `to_bytes` / `from_bytes` lax sites in `src/vector/avx2.rs` (line 65,
  1140, 1153) — blocked on hax-lib `&mut` slice modeling, surfaced to user
  as a hax-lib upstream task.
- The `op_ntt_layer_1_step` / `op_inv_ntt_layer_1_step` admits in
  `src/vector/portable.rs` — separate sprint (4-way branch refactor).
- The Generic-side `ntt_vector_u` / `invert_ntt_at_layer_4_plus` admits —
  separate sprint (Hacspec_ml_kem.Commute.Chunk bridge ladder).

## Key file paths quick reference

- AVX2 SIMD primitives: `src/vector/avx2/serialize.rs` (this sprint's surface).
- AVX2 op-wrappers + bridges: `src/vector/avx2.rs:864-902` (out of scope).
- AVX2 NTT primitives: `src/vector/avx2/ntt.rs` (out of scope).
- Portable equivalents (proof intent reference, not body source):
  `src/vector/portable/serialize.rs:76` (serialize_1),
  `:379` (serialize_5), `:461` (deserialize_5),
  `:682` (serialize_11), `:768` (deserialize_11).
- Existing AVX2 SIMD-body templates (mirror these proof patterns):
  `serialize_4` line 185, `serialize_10` line 532,
  `serialize_12` line 721 (with inner `_vec` helpers showing the
  `assert_norm (BitVec.Utils.forall_n N)` tactic).
- BitVec lemma surface: `Libcrux_intrinsics.Avx2_extract` in
  `core-models-*/proofs/fstar/extraction/`.
