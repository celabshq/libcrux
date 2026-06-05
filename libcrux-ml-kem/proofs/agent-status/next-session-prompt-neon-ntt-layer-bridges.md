# Next session — prove the 6 Neon NTT-layer trait bridges (panic_free → proven)

## Mission
Upgrade the six Neon `op_{,inv_}ntt_layer_{1,2,3}_step` trait wrappers in
`libcrux-ml-kem/src/vector/neon.rs` from `panic_free` to **fully proven**, so the
Neon trait layer matches AVX2/portable for the NTT-layer methods. This is a
focused F* perf-debug task (~1 session), NOT hint-seeding and NOT a rewrite.

## Where things stand
- Branch `libcrux-ml-kem-proofs`, tip `a650ae39c` (pushed). ntt_multiply post +
  Phase C (Neon out of ADMIT_MODULES) + `op_cond_subtract_3329` /
  `op_to_unsigned_representative` proven are all DONE. memory
  [[project_neon_ntt_multiply_phasec_done]]. Vector.Neon verifies green;
  `make all` green; cargo simd128 23/23.
- The 6 NTT-layer wrappers are currently `panic_free` (trait post admitted).
- A previous attempt to prove all 6 at once ran **71 min without completing**.

## The diagnosis (already established — do NOT redo from scratch / do NOT hint-seed)
AVX2's IDENTICAL `op_ntt_layer_2/3` + `op_inv_ntt_layer_2/3` verify in **5–9 s
each cold** (see `agent-status/fstar-perf-top20.md`), and `.fstar-cache/hints/`
is **git-ignored / 0 tracked** — AVX2 verifies cold-fast, does not rely on
committed hints. So the 71-min non-completion is a **saturation DEFECT in the
Neon port**, not cold-slowness.
**Prime suspect:** Neon `repr x = Seq.append (vec128_as_i16x8 x.f_low)
(vec128_as_i16x8 x.f_high)` is a STRUCTURED term, whereas AVX2's
`vec256_as_i16x16 x.f_elements` is an ATOMIC opaque array. Feeding `repr vector`
into the `forall4 p_layer_N` FE-form + 8 `lemma_butterfly_pair_commute` calls
produces `Seq.index (repr ..)` terms that the `lemma_repr_index` SMTPat
re-expands per index, exploding the context.
(Note: the Neon ntt.rs PRIMITIVES already prove `*_butterfly_post` over `repr`
fine — repr is provable; the blowup is the bridge-layer FE-form × append.)

## The proof text (ready to apply)
`agent-status/neon-ntt-layer-bridge-port.py` is the Python splicer that replaces
the 6 panic_free stubs in `neon.rs` with the proven bodies (AVX2 mirror,
`repr`-adapted). Run it on the current `neon.rs`, or read it to see the exact
bodies. AVX2 templates: `src/vector/avx2.rs` op_ntt_layer_1 (~L423), layer_2
(~L454), layer_3 (~L523), inv_1 (~L587), inv_2 (~L618), inv_3 (~L683).

## Step-by-step
1. Apply the proven bodies (`python3 proofs/agent-status/neon-ntt-layer-bridge-port.py`
   from `libcrux-ml-kem/`), `cd libcrux-ml-kem && ./hax.py extract`.
2. **Isolate each of the 6** (find which actually saturate). For each:
   `rm /.fstar-cache/checked/Libcrux_ml_kem.Vector.Neon.fst.checked` then
   `fstar_build check/Libcrux_ml_kem.Vector.Neon.fst` with
   `make_args=["OTHERFLAGS=--admit_except \"Libcrux_ml_kem.Vector.Neon.op_ntt_layer_2_step\""]`
   (one name at a time — comma lists admit everything → false clean). Expect
   layer_1 / inv_1 (branch-lemma, rlimit 400) to pass fast; layer_2/3 + inv_2/3
   (FE-form, rlimit 600) to be the saturating ones.
3. **smtprofiling** (qi.profile) ONE saturating proof before concluding — confirm
   the dominant quantifier is `lemma_repr_index` (or a Commute trigger).
4. **Fix the term structure** (the load-bearing step). Candidates, cheapest first:
   a. Bind an atomic 16-array view the bridge treats opaquely instead of live
      `repr`: `let v16 = to_i16_array vector` (Vector_type.fsti: `to_i16_array`
      ensures `result == repr v`, but as a returned `t_Array i16 16`, not a live
      append), then run the Commute lemmas + FE-form over `v16`/`o16`. In the
      fstar! body use `let vec = ...` bound to the to_i16_array result.
      Equivalent F* form: `Seq.lemma_eq_intro`/`assert (repr vector == v16)` so
      the FE-form sees an atomic array.
   b. If (a) insufficient: a small clean-context Neon helper
      `lemma_neon_layer_N_lanes` taking the butterfly_post(repr) + concluding the
      8/16 per-lane `Seq.index` facts the Commute lemmas need, so the append
      never enters the forall4 query (mirrors the clean-context post-helpers
      already in neon/ntt.rs).
   c. Keep rlimit at AVX2's values (layer-1/inv-1: 400; layer-2/3, inv-2/3: 600,
      split_queries always) — do NOT bump higher (it's saturation, not budget;
      [[feedback_rlimit_cap_800]]).
5. Full no-admit build `check/Libcrux_ml_kem.Vector.Neon.fst` green (rm tainted
   .checked first), then `make all` green; cargo simd128 23/23.
6. Re-run `python3 proofs/generate_verification_status.py`; commit (agent-mlkem,
   source-only) + update [[project_neon_ntt_multiply_phasec_done]].

## Workflow constraints (from memory)
F* via fstar-proxy MCP `fstar_build`/`fstar_build_status` (never Bash make);
real cache `/.fstar-cache/checked/`; rm the module .checked before each real
build (false-clean otherwise); judge by `all_vcs_discharged:true` + real
`rlimit_used`. Max 4 fstar/z3 per agent; NEVER global-pkill (parallel ml-dsa
session) — kill only your own build's process GROUP by pgid. Per-fn debug budget
30–60 min then `smtprofiling` + cliff note. No code changes for proofs.

## Leftover panic_free after this (separate, lower priority)
compress/decompress×3 (AVX2 also panic_frees), compress_1 (AVX2 proves — Neon
bridge TODO), serialize_1/4/10/12 + deserialize_12 (underlying Neon free-fns are
l_True — need the underlying Neon serialize proofs first), rej_sample (lax).
