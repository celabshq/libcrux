# Cross-chunk forward-NTT layers 3-7 — PROGRESS (2026-06-04)

## ★★ REFINED CLIFF (end of session 2, after createi-fix build via upgraded MCP) ★★
- The createi-fix (unchunked lemma + `v(mk_usize u)==u` seeds) IS in ntt.rs + extracted.
  Its full build (`build_id 112b7671`, via new `fstar_build_status` structured output)
  CHANGED the failure mode: **no more rlimit-saturation** (query 674 used 400.000 → gone);
  instead ntt_at_layer_3 queries now **succeed at rlimit 0.5 but take 16-20 MINUTES each**
  (177s / 1003s / 1253s observed) → an e-matching/quantifier TIME-explosion. The module
  can't finish in the ~30-min wall (no clean F* error — make Error 1 from the reap; all deps
  incl. Commute.Chunk verified first).
- So TWO residual cascades remain (the createi-move addressed the rlimit one, exposed these):
  1. **forall32-of-forall32 framing** — the driver discharges lemma_l3's `forall32` precondition
     from 16 outer_3_plus forall32-posts (2 units/atom, framed). → FIX: **flat-asserts** (16
     explicit per-even-u framed `unit_fe_post_cross …` asserts in the driver body BEFORE the
     compose call; turns it into 16 flat facts like A3, each ~rlimit 11 in isolation).
  2. **createi in the driver ENSURES** — `forall i<256. … simd_units_to_array (chunks_of_re re) …`
     (both createi-based) cascades via createi_lemma's `[SMTPat (Seq.index (createi f) (v i))]`.
     → FIX: opacify chunks_of_re / simd_units_to_array with reveal lemmas — BUT this is RISKY:
     the committed A3 layer-0/1/2 drivers also use them and would need reveal-threading too.
- NEXT SESSION (use the new MCP): `fstar_build` with `--log_queries` (no admit_except) →
  `fstar_build_status` structured `slowest_queries`/`failed_queries` to identify WHICH of the
  two cascades is the 16-20min query (precondition vs ensures) BEFORE choosing the fix. Then
  apply flat-asserts first (cheaper, lower-risk); re-profile; opacify only if the ensures is
  still slow. The driver logic is PROVEN (rlimit ≤12 under --admit_except isolation) — this is
  purely a full-module e-matching performance problem.

## ★ CURRENT STATE / RESUME (end of session 2) ★
- **COMMITTED & VERIFIED: Phase 1** — Commute.Chunk.fst with all 5 cross-chunk poly bridges
  + lemma_cross_idx (`15d480d22`, `Verified module`, admit count 2 unchanged).
- **WIP in ntt.rs (uncommitted, does NOT gate yet)** — saved patch `/tmp/ntt-layer3-wip.patch`
  (+ `~/.claude/plans/ntt-layer3-wip-2026-06-04.patch`). Adds, for LAYER 3 only:
  unit_fe_post_cross atom + lemma_round_cross_intro + lemma_atom_to_bf_cross +
  lemma_l3_cross_driver_compose (before-blocks on simd_unit_ntt_step); outer_3_plus
  round-ensures+body + loop-invariant + function-ensures cross atom; ntt_at_layer_3 snapshot
  + functional ensures + 16 assert_norm + `lemma_l3_cross_driver_compose ${orig_re} ${re}`.
  ALL sub-pieces VERIFY (outer_3_plus, round, both lemmas — confirmed by scratch + by the
  --admit_except isolation build where ntt_at_layer_3 ran at rlimit ≤12).
- **BLOCKER**: full-module Portable.Ntt build — ntt_at_layer_3's driver-body discharge of
  lemma_l3_cross_driver_compose's forall32 precondition saturates (query 674 @ rlimit 400/245s)
  in the pre-createi-fix build. The createi-move fix (lemma takes unchunked orig_re/re; createi
  bridged inside via `v(mk_usize u)==u` seeds) IS applied in ntt.rs but its full build was killed
  at 85 min (AMBIGUOUS: CPU-contended with a parallel ml-kem agent fresh build; don't trust the
  85min as a pure proof signal). The hint file for Portable.Ntt is currently ABSENT (fresh build).

### RESUME STEPS (next session, fresh budget, ideally no parallel agent contention)
1. `cd libcrux-ml-dsa && ./hax.sh extract` (ntt.rs already has the fix; confirm `[hax.sh] done`).
2. `rm .fstar-cache/checked/Libcrux_ml_dsa.Simd.Portable.Ntt.fst.checked`; gate-build via curl
   `--max-time 2400` (build_args in /tmp/build_ntt.json). Watch `.checked`, not the curl body.
3. IF it passes (the createi fix may suffice once not CPU-contended): commit layer 3, then wire
   layers 4-7 (artifacts /tmp/ntt_l4567_blocks.txt + /tmp/l{4..7}_compose.txt — but REGENERATE
   them with the createi-fix design: unchunked lemma params + 2 createi seeds in aux_bf + unchunked
   driver call site, mirroring the now-fixed layer-3 lemma).
4. IF query 674 STILL saturates: apply the **flat-asserts** fix — in ntt_at_layer_3 body, after
   the 16 outer_3_plus calls and BEFORE the compose call, add 16 explicit asserts, one per even u
   (u=0,2,..,30): `assert (unit_fe_post_cross (Seq.index ${orig_re} u).f_values (Seq.index ${orig_re} (u+1)).f_values (Seq.index ${re} u).f_values (Seq.index ${re} (u+1)).f_values <ZETA_lit_c>)`
   — each is ONE atom's frame (light), turning the lemma's forall32 precondition into 16 flat
   facts (mirrors A3's flat-atom composition; avoids the forall32-of-forall32 auto-match).
   Validate the driver-body discharge via --admit_except isolation first (it already passed there).
5. Diagnostic if needed: full-module + `--log_queries --z3refresh` (NO admit_except), profile
   query 674's .smt2 with `smt.qi.profile=true` to confirm framing vs residual createi.

---

# Cross-chunk forward-NTT layers 3-7 — PROGRESS (2026-06-04)

## ROOT CAUSE of layer-3 driver saturation = createi cascade (FIXED)
- ntt_at_layer_3 driver logic is CORRECT (verifies at rlimit 11 when siblings admitted; the
  iso build's "Error 1" was a Z3 IPC crash `Killing old z3proc` from 1300 queries, NOT a VC
  failure). Stale-hint was a RED HERRING (fresh no-hint full build ALSO saturated query 674).
- REAL cause: the driver passed `chunks_of_re ${orig_re}`/`${re}` to lemma_l3_cross_driver_compose,
  whose requires referenced `(chunks_of_re _).[u]` for u AND u+1 (2 units/atom × 16 even-u +
  framing). `chunks_of_re` is createi-based; `Hacspec_ml_dsa.createi_lemma` has
  `[SMTPat (Seq.index (createi f) (v i))]` → the cascade ([[project_mlkem_createi_smtpat_cascade]])
  fires across the driver's forall32, saturating ONE sub-query (674) at rlimit 400/195s.
  (A3 escaped it: its atoms were 1-unit, halving the createi surface.)
- FIX (validated in scratch + applied): lemma_lL_cross_driver_compose takes UNCHUNKED
  `(orig_re re : t_Array Coefficients 32)`; requires atoms about `(Seq.index re u).f_values`
  (matches outer_3_plus posts EXACTLY → driver discharges by FRAME only, no createi). The
  `chunks_of_re` bridge moves INTO the lemma's clean aux_bf, where the createi_lemma SMTPat is
  enabled per-u by the `assert (v (mk_usize u) == u)` + `assert (v (mk_usize (u+1)) == u+1)`
  seeds (the createi SMTPat trigger is `(v i)`, doesn't match a bare nat `u` without the seed).
  Driver call site: `lemma_l3_cross_driver_compose ${orig_re} ${re}` (NO chunks_of_re).
- **Apply this design to layers 4-7**: unchunked lemma params + the 2 createi seeds in aux_bf
  (for u and u+S) + unchunked driver call site. Regenerate /tmp/l{4..7} artifacts accordingly.

## OPERATIONAL NOTES (session 2)
- Proxy archives the build log only at COMPLETION — "no log mid-build" is normal; detect
  completion via the `.checked` file or the curl return, not the log.
- `fp.sh` curl has `--max-time 600`; for gate builds use a raw curl with `--max-time 1800`
  (`/tmp/build_ntt.json` holds the args). Even so the response sometimes comes back EMPTY for
  long builds (transport) — judge success by `.checked` written + grep the archived log for
  `Verified module`, NOT the curl body.
- Deleting the WHOLE module hint file forces fresh solving of the ENTIRE 1900-line module
  (~30-40 min), not just the changed fn. Unavoidable (hints are per-module-file) but slow;
  budget for it. After the fresh build records good hints, subsequent builds are fast again.

## UPDATE (session 2, evening) — STALE HINT was the layer-3 blocker
- **outer_3_plus (round + loop + ensures cross atom) VERIFIES.** lemma_round_cross_intro
  (round-body bridge: add_post/sub_post usize-foralls → ground atom via `v(mk_usize l)==l`
  e-match seed + mmbc nat-foralls + `mod_q` reveal) VERIFIES. The ONLY layer-3 blocker was a
  **stale hint**: `.fstar-cache/hints/Libcrux_ml_dsa.Simd.Portable.Ntt.fst.hints` (from the
  old bounds-only ntt_at_layer_3) made the full build's query 674 SATURATE at rlimit 400/195s,
  while an isolated `--z3refresh` build verified the SAME function at rlimit **11** (all 673
  queries pass). FIX = `rm` the stale hint (+ tainted .checked) and rebuild fresh.
- **GOTCHA for layers 4-7 + final**: after wiring each new layer, the Portable.Ntt hint file is
  stale for the newly-changed functions → `rm .fstar-cache/hints/Libcrux_ml_dsa.Simd.Portable.Ntt.fst.hints`
  before each gate build (the build re-records correct hints). Diagnose stale-hint saturation by
  re-running with `--z3refresh` (OTHERFLAGS) — if it passes fresh but saturates with hints, it's the hint.
- The round needed: snapshot `re_in` at round entry; lemma_round_cross_intro called with
  `(re_in[idx]).f_values (re_in[idx+step]).f_values (re[idx]).f_values (re[idx+step]).f_values ${tmp}.f_values $zeta`.

## UPDATE (later 2026-06-04, session 2)
- **Phase 1 DONE + committed `15d480d22`**: all 5 cross-chunk poly bridges + lemma_cross_idx
  ported into Commute.Chunk.fst, `Verified module` clean (695s build), admit count still 2.
- **Reducer-wrap fix**: each `layer_L_lane` needed `#push-options "--z3rlimit 200 --split_queries
  always"` (the `z *! cast(p[i])` i64-overflow is a nonlinear VC flaky as a plain let; L5 failed
  without it). Applied to all 5.
- **F* propagates `==>` hypothesis to index refinement** (tested): so the loop-invariant / ensures
  cross-atom can index `orig_re[u+STEP_BY]` EXACTLY (no %32) given ambient `OFFSET+2*STEP_BY<=32` +
  guard. (The total t-witness inside driver-compose still uses `(u+S)%32` + small_mod.)
- **Layer 3 wired into ntt.rs** (atom unit_fe_post_cross + lemma_atom_to_bf_cross +
  lemma_l3_cross_driver_compose before-blocks on simd_unit_ntt_step; outer_3_plus round
  ensures+body-reveals + loop-invariant + function-ensures cross atom; ntt_at_layer_3 snapshot +
  functional ensures + 16 assert_norm + compose). Extracted clean (`[hax.sh] done`). **Portable.Ntt
  gate-build IN PROGRESS.**
- Layers 4-7 wiring artifacts pre-generated: /tmp/ntt_l4567_blocks.txt (driver-compose
  before-blocks), /tmp/l{4,5,6,7}_compose.txt (assert_norms + compose calls). zidx = c+k,
  k=128/len = 8/4/2/1 for L4/5/6/7. Apply after L3 gates.
- Driver-scratch validation abandoned (curl 590s cap; full-module response empty). The
  Portable.Ntt build IS the gate for outer_3_plus + drivers (only way to check Phase 2/3 anyway).

---

# Cross-chunk forward-NTT layers 3-7 — PROGRESS (paused 2026-06-04)

Worktree `/Users/karthik/libcrux-ml-dsa-proofs`, branch `ml-dsa-proofs`, **clean
tree at HEAD 7d45863e6** (the plan-doc commit). NOTHING tracked modified yet.
Plan: `ntt-functional-layers37-plan-2026-06-04.md`. A3 recipe:
`ntt-functional-drivers012-2026-06-04.md`.

## STATE: Phase 1 (the new spec bridge) is DESIGNED + (mostly) VALIDATED in scratch

All work so far lives in **untracked scratch `.fst` files** in
`specs/ml-dsa/proofs/fstar/commute/`:
- `Scratch_cross.fst` — THE deliverable: `lemma_cross_idx` + all 5 layers'
  cross-chunk poly bridges. (Throwaway: `Scratch_gen_test.fst`,
  `Scratch_idx_test.fst` — generic-reducer experiments that FAILED; ignore/delete.)

### VALIDATED (clean F* typecheck, status ok):
1. **`lemma_cross_idx (s:pos{16%s==0 /\ s<=16}) (ulo:nat{ulo<32 /\ ulo%(2*s)<s}) (l:nat{l<8})`**
   — generic flat-index arithmetic for one lo-unit pair. Discharges round/idx
   facts at flat indices `8*ulo+l` (lo, idx<len) and `8*ulo+8s+l` (hi, idx>=len),
   plus the partner bound `ulo+s<32`. Proven for SYMBOLIC s (nonlinear modular
   reasoning solved ONCE for all 5 layers via lemma_div_mod + lemma_div_plus +
   lemma_mod_plus + small_div/small_mod + lemma_mult_lt/le_left). **rlimit 300.**
2. **Layer 3 full bridge** (`layer_3_lane`, `lemma_ntt_layer_3_lane` [createi `()`],
   `lemma_layer_3_cross_pair`, `lemma_ntt_layer_3_cross_to_hacspec_poly`) — validated
   in the FIRST scratch run (every fragment ok). The cross-chunk algebra reuses
   `C.lemma_layer_0_pair_spec` (layer-agnostic butterfly) EXACTLY — worked first try.

### WRITTEN but UNCONFIRMED (re-validation in progress):
- **Layers 4,5,6,7 bridges** in `Scratch_cross.fst` (structural copies of layer 3
  with constants per layer; each calls generic `lemma_cross_idx` at concrete s).
  Per-layer constants (step_by S, len, k=128/len, lo-pred, partner, 2*len divisor):
  | L | S | len | k | lo-pred | partner | 2*len |
  |---|---|-----|---|---------|---------|-------|
  | 3 | 1 | 8   |16 | u%2==0  | u+1     | 16    |
  | 4 | 2 | 16  | 8 | u%4<2   | u+2     | 32    |
  | 5 | 4 | 32  | 4 | u%8<4   | u+4     | 64    |
  | 6 | 8 | 64  | 2 | u%16<8  | u+8     | 128   |
  | 7 |16 | 128 | 1 | u%32<16 | u+16    | 256   |
- A full `fstar_typecheck full` of the 5-layer module took ~10 min and **completed
  exit 0**, but I corrupted its output by killing a z3 PID mid-query (LESSON: never
  kill procs while a curl typecheck is in flight). A clean re-run is the background
  task — **on resume, FIRST re-validate `Scratch_cross.fst`** (open fresh session;
  see recipe below) and confirm all fragments `ok` before porting.

## RESUME STEPS (in order)

### 0. Re-validate Scratch_cross.fst (confirm layers 4-7)
fstar_open recipe (saved in `/tmp/dsa_open.json`; regenerate if gone — derived from
`make --dry-run` in `libcrux-ml-dsa/proofs/fstar/extraction`):
- fstar_exe `/Users/karthik/.local/bin/fstar.exe` (F* 2026.03.24, on PATH; what make uses)
- cwd `libcrux-ml-dsa/proofs/fstar/extraction`
- includes (18): `../spec` + hax-lib(d8b5b3d) core/rust_primitives/extraction +
  aesgcm/sha3/platform/core-models/intrinsics extraction + fstar-bitvec +
  libcrux-intrinsics + libcrux-ml-dsa + libcrux-ml-kem extraction + ml-kem/spec +
  specs/ml-dsa/{commute,extraction} + specs/ml-kem/extraction + specs/sha3/extraction
- options: `--warn_error -321-331-241-274-239-271 --ext context_pruning --z3version 4.13.3
  --cache_checked_modules --cache_dir <repo>/.fstar-cache/checked
  --already_cached +Prims+FStar+LowStar+C+Spec.Loops+TestLib --query_stats`
- **MANDATORY per-layer validation**: the FULL 5-layer module typecheck completes
  (fstar exit 0, ~10 min) but the proxy's curl/SSE response comes back EMPTY (0 bytes)
  for a response that large — confirmed TWICE. The small layer-3-only and idx-only
  scratches returned clean JSON fine. So on resume, validate each layer in its OWN tiny
  scratch (`module Scratch_L4` = lemma_cross_idx + layer_4_lane + its 3 lemmas), which
  yields a small parseable response. (lemma_cross_idx + layer 3 ALREADY confirmed ok.)

### 1. Port Phase-1 bridge into Commute.Chunk.fst (ONE edit -> ~20 min cascade)
Insert the contents of `Scratch_cross.fst` (drop the module header / `module C =`
alias; rename `C.` -> direct refs since it'll BE in Commute.Chunk) just before the
zeta-table section (~line 1665, after `lemma_ntt_layer_2_step_to_hacspec_poly`).
i.e. add: `lemma_cross_idx` + 5x(`layer_L_lane` + `lemma_ntt_layer_L_lane` +
`lemma_layer_L_cross_pair` + `lemma_ntt_layer_L_cross_to_hacspec_poly`).
Baseline admits in Commute.Chunk = **2** (unrelated: lemma_decompose_spec_eq_decompose;
the file's other admit) — must NOT increase.

### 2. Phase 2+3 (ntt.rs `outer_3_plus`) — DESIGNED, not yet written
Add to the before-block cluster on `simd_unit_ntt_step` (the FIRST fn, so helpers
precede the hoisted `___round` fns — A3 ordering rule):
- **`unit_fe_post_cross (ci_lo ci_hi co_lo co_hi : t_Array i32 8) (zeta:i32{is_i32b 4190208 zeta})`**
  = `[@@ "opaque_to_smt"]` GROUND 8-conjunction: for each lane l in 0..7,
  `let t_l = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi l) zeta in
   v co_lo[l]==v ci_lo[l]+v t_l /\ v co_hi[l]==v ci_lo[l]-v t_l /\
   (v t_l)%8380417==(v ci_hi[l]*v zeta*8265825)%8380417`.
- **`lemma_atom_to_bf_cross`** (reveal + `introduce forall l<8 with match l`) ->
  `forall l<8. <butterfly with mont_mul ci_hi[l] zeta>`. Mirrors `lemma_atom_to_bf`.
- **5x `lemma_lL_cross_driver_compose (orig fut : t_Array (t_Array i32 8) 32)`**:
  requires `forall32 (fun u -> <lo-pred u> ==> unit_fe_post_cross (orig[u]) (orig[u+S])
  (fut[u]) (fut[u+S]) (mk_i32 (zeta_r (u/(2S)+k))))`; ensures the cross poly congruence.
  Body mirrors `lemma_l2_driver_compose`: `forall32_elim_1d` -> per-u `lemma_atom_to_bf_cross`
  (Classical.forall_intro) -> per-u zeta cong (reveal mod_q + zeta_r + lemma_v_zetas_eq_zeta)
  -> `Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_L_cross_to_hacspec_poly orig fut t zm`
  with `t u l = mont_mul (orig[u+S][l]) (zm u)`, `zm u = mk_i32 (zeta_r (u/(2S)+k))`.
  **VALIDATE these in a scratch (referencing the now-real Commute.Chunk lemmas) before
  baking into ntt.rs** — they are F*-only, testable without extraction.

`outer_3_plus` ENSURES — add conjunct (alongside existing modifies_range_32 + bounds):
`forall32 (fun u -> (u >= OFFSET /\ u < OFFSET+STEP_BY) ==> unit_fe_post_cross
  (Seq.index re u).f_values (Seq.index re (u+STEP_BY)).f_values
  (Seq.index re_future u).f_values (Seq.index re_future (u+STEP_BY)).f_values ZETA)`.

`outer_3_plus` inner `round` ENSURES — add (for the single processed pair):
`unit_fe_post_cross (re[index]) (re[index+step_by]) (re_future[index]) (re_future[index+step_by]) zeta`.
DISCHARGE in round body: after the existing reveal_is_i32b_array_opaque + the 3 leaf
calls (montgomery_multiply_by_constant on tmp, then re[i+s]=re[i], subtract, add), add
`reveal add_post; reveal sub_post; reveal unit_fe_post_cross`. The leaf posts give per-lane:
mmbc post `forall l. tmp[l]==mont_mul(old_re[i+s][l],zeta)` + mod_q cong; add_post/sub_post
`forall l. ...`. ci_lo=re_pre[index], ci_hi=re_pre[index+step_by], co_lo=re_future[index]
= re_pre[index]+tmp, co_hi=re_future[index+step_by]= re_pre[index]-tmp (slot was copied to
re[index] before subtract). t_l=tmp[l]=mont_mul(ci_hi[l],zeta). All conjuncts follow.

`outer_3_plus` LOOP INVARIANT — add (alongside existing modifies_range2_32 + bounds):
`forall32 (fun u -> (u >= OFFSET /\ u < j) ==> unit_fe_post_cross (orig_re[u]) (orig_re[u+STEP_BY])
  (re[u]) (re[u+STEP_BY]) ZETA)`. (orig_re = the existing #[cfg(hax)] snapshot.)
KEY GEOMETRY (proven on paper): for STEP_BY>=2, round j modifies ONLY units j (lo) and
j+STEP_BY (hi); for u in [OFFSET,j) neither u nor u+STEP_BY equals j or j+STEP_BY (needs
j<OFFSET+STEP_BY), so old atoms are FRAMED; the new u=j atom comes from the round post,
with re_j[j]==orig_re[j], re_j[j+STEP_BY]==orig_re[j+STEP_BY] via the invariant's
modifies_range2_32 (j, j+STEP_BY not in the already-processed ranges). For STEP_BY=1
(layer 3) the loop runs ONCE -> nearly the A3 ground case (lowest risk -> do first).

### 3. Phase 4 (drivers ntt_at_layer_{3..7}) — DESIGNED
Each driver: `#[cfg(hax)] let orig_re = re.clone();` then the N outer_3_plus calls
(already present), then `hax_lib::fstar!(r#" <N assert_norm (zeta_r (c+k) == <ZETA_c literal>)>
lemma_lL_cross_driver_compose (chunks_of_re ${orig_re}) (chunks_of_re ${re}) "#);`. Add the
functional ensures `forall i<256. (simd_units_to_array (chunks_of_re re_future))[i] %q ==
(ntt_layer (simd_units_to_array (chunks_of_re re)) L)[i] %q` (alongside the existing bounds
post). The N zeta literals are already in the outer_3_plus calls (ntt.rs:1056-1146);
zeta_r(c+k) must equal call c's ZETA literal — VERIFY each with assert_norm.
**DO LAYER 3 FIRST**, gate clean (rm Portable.Ntt.checked, full fstar_build
check/Libcrux_ml_dsa.Simd.Portable.Ntt.fst, NO --admit_except, exit 0 + Query-stats>0),
commit, then 4-7.

### 4. Phase 5 (top `ntt` driver) — 8-layer compose -> == Hacspec_ml_dsa.Ntt.ntt
`Hacspec_ml_dsa.Ntt.ntt` (extraction/Hacspec_ml_dsa.Ntt.fst:124) = the fixed chain
ntt_layer 7;6;5;4;3;2;1;0. With all 8 layer drivers carrying `== ntt_layer _ L` (mod q),
chain via a `lemma_compose_8` threading mod-q through each ntt_layer (its output is already
mod_q-reduced; may need a per-step "both sides mod_q" framing). Add functional ensures to
`ntt`. Then full-crate `JOBS=2 ./hax.sh prove` (0 errors), regen
`python3 proofs/generate_verification_status.py` -> cp to `ml_dsa_verification_status.md`.

## GOTCHAS confirmed this session
- Generic reducer (`layer_lane_gen` over symbolic len) does NOT work — overflow walls on
  `2*len`, `i+len`, `round+k`. Per-layer CONCRETE reducers avoid all of it. (cross_pair/
  cross_poly stay per-layer too because createi reduction `()` needs concrete layer.)
- NEVER kill fstar/z3 while a curl typecheck is in flight — corrupts the result (empty
  output, false "exit 0"). Wait it out (full check of 30 split-query lemmas ~10 min) or
  validate per-layer in smaller scratch modules.
- Another agent runs ml-kem sessions on this host; stay <=4 procs, kill only own PIDs.
- Scratch files in commute/ are untracked + not make-deps; harmless to leave, but DELETE
  before any final `./hax.sh prove` (a stray broken module could confuse a glob build).

---

# SESSION 3 (2026-06-04, resume) — flat-asserts applied

## ACTION: cascade #1 fix (flat-asserts) applied + extracted
- Added 16 explicit per-even-u flat-asserts to `ntt_at_layer_3` body (ntt.rs:1290-1305,
  in the fstar! block AFTER the 16 assert_norms, BEFORE `lemma_l3_cross_driver_compose`).
  Each: `assert (unit_fe_post_cross (Seq.index ${orig_re} U).f_values ... (Seq.index ${re} (U+1)).f_values (mk_i32 (Spec.MLDSA.Ntt.zeta_r (U / 2 + 16))))` for U=0,2,..,30.
  Zeta form matches the compose-lemma precondition EXACTLY (trivial forall32 assembly).
- Re-extracted clean (`[hax.sh] done`); flat-asserts present in extracted Ntt.fst:1965-2040,
  compose at 2041. `.checked`/hints ABSENT (fresh).
- Isolation build (--admit_except ntt_at_layer_3_) RUNNING to validate well-formedness;
  then full gate build with --log_queries (no admit_except) to confirm cascade #1 fixed +
  see if cascade #2 (createi ensures) remains.

## LAYERS 4-7 design pinned (each gives 16 lo-atoms via S calls × S units):
- L4 S=2 OFFSET=4c lo-units {4c,4c+1} zeta_r(u/4+8); L5 S=4 OFFSET=8c {8c..8c+3} zeta_r(u/8+4);
  L6 S=8 OFFSET=16c {16c..16c+7} zeta_r(u/16+2); L7 S=16 OFFSET=0 {0..15} zeta_r(u/32+1).
- cross_to_hacspec_poly lemmas exist in Commute.Chunk (1789/1896/2004/2112/2220).
- Reused as-is across layers: unit_fe_post_cross, lemma_round_cross_intro, lemma_atom_to_bf_cross,
  outer_3_plus. NEW per layer: lemma_lL_cross_driver_compose + driver functional ensures + flat-asserts.

---

# SESSION 3 (cont.) — Phase 5 top-ntt compose

## LAYERS 4-7 DONE + COMMITTED (32b696cca)
- All 4 driver-compose lemmas + drivers replicated from L3 (createi-fix + 16 flat-asserts each);
  one batch full build "Verified module ... All verification conditions discharged", 0 admits.

## TOP NTT (== Hacspec_ml_dsa.Ntt.ntt) — compose infra PROVEN (scratch), wired, gate-building
- Design: lemma_ntt_compose_8 (9 flat arrays + 8 mod-q post hyps) chains via 7 per-layer
  ntt_layer congruence lemmas (layers 0-6): a==b (mod q) elementwise ==> ntt_layer a L == ntt_layer b L
  (EXACT, since ntt_layer output is mod_q-reduced). Then ntt f0 == g0 by unfolding the 8-layer chain.
- Per layer: lemma_layer_L_lane_cong (STANDALONE, NO createi — clean context) proves
  layer_L_lane a ii == layer_L_lane b ii via parity case-split + 2 layer-independent butterfly
  congruence helpers (lemma_bf_even/odd_cong, built on lemma_mod_q_v + FStar.Math.Lemmas mod lemmas);
  lemma_ntt_layer_L_cong does the createi reduction (lemma_ntt_layer_L_lane) + Seq.lemma_eq_intro,
  calling the clean lane-cong.
- ALL 15 lemmas + compose VERIFY in scratch (direct fstar.exe, EXIT 0 "Verified module").

## KEY F* LESSONS (top-ntt cong, this session)
1. **createi cascade poisons trivial parity asserts** — calling lemma_ntt_layer_L_lane (createi
   reduction) in the SAME function as the parity/%! reasoning poisoned `v(ii%!2)==parity` asserts
   in the odd branch (even branch passed; asymmetric). FIX = split lane-cong into a STANDALONE
   lemma with NO createi (clean context); the ntt-layer-cong calls reduction + lane-cong separately.
   (Cascade-pollution recipe.) This was THE unlock after ~10 failed in-fn attempts.
2. **modulo bound needs refined type** — `i % twoLen < twoLen` not derivable from `~(i%twoLen<len)`;
   bind `let parity:(n:nat{n<twoLen}) = i%twoLen` so the bound is intrinsic (context_pruning-proof);
   + FStar.Math.Lemmas.lemma_mod_lt i twoLen.
3. **`<.`/`<=.` are unfold but `lt`/`lte` (bodies) are plain `let`** — need --ifuel 2 to unfold to
   `v a < v b`; the layer_L_lane if-branch resolution needs the POSITIVE `<.` (even) / `~(<.)` (odd)
   asserted explicitly so F* resolves the createi-body if.
4. **Session reaping** — proxy interactive sessions reaped on cumulative timeout during edit latency;
   for the tight cong edit-loop, used direct fstar.exe on a scratch (reference_sha3_fstar_local_invocation
   pattern) for a clean EXIT-0 signal. Faster + reliable than re-opening sessions.
- Wired into ntt.rs: compose block as before-block on top ntt; 8 #[cfg(hax)] snapshots (s0,s7..s1);
  functional ensures (forall i<256 out_flat[i]%q == (ntt in_flat)[i]%q); compose call. Extracted clean
  (15 lemmas present). Gate build IN PROGRESS (build_id 4f934652).
