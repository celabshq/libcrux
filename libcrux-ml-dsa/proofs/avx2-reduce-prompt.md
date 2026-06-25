# AVX2 Operations::reduce — fresh-session prompt

Resuming the ML-DSA proof sprint on branch `ml-dsa-proofs` in
`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`. The previous
session closed **Step 7 Portable** (commit `c91f0b413`) — the impl
method `Libcrux_ml_dsa.Simd.Portable::Operations::reduce` discharges
without a body admit. This session targets **Step 7 AVX2**:
`Libcrux_ml_dsa.Simd.Avx2::Operations::reduce`.

Last commit is `d80d911d4`. Read in this order before doing anything
else:

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current empirical baseline.
2. `libcrux-ml-dsa/proofs/outstanding-admits.md` — the
   "Libcrux_ml_dsa.Simd.Avx2.f_reduce body admit" entry diagnoses the
   gap. Note the **two prior entries** for context:
   "Libcrux_ml_dsa.Simd.Portable + Simd.Avx2 impl-Operations method
   body admits" and the resolved Portable case in commit
   `c91f0b413`'s message.
3. `libcrux-ml-dsa/src/simd/portable.rs:337-378` — the working
   Portable proof. The AVX2 attempt should mirror its loop_invariant
   shape and after-loop bridge pattern. Reproduced in §3 below.
4. `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   — already contains `lemma_reduce_lane_commute`, the bridge that
   converts (centered Barrett bound) + (raw `% 8380417` congruence)
   into `Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post`.
5. `git log --oneline -7` for recent context.

## Pre-flight

```
cd libcrux-ml-dsa
git rev-parse HEAD                    # should print d80d911d4 (or later)
git status                            # should be clean
./hax.sh prove 2>&1 | tail -22        # should show:
                                      # 98 invoked, [CHECK]=42, [ADMIT]=56,
                                      # 98 verified, 0 errors
```

Hints are now enabled in the Makefile (`ENABLE_HINTS = --use_hints
--record_hints`); hint files accumulate in
`.fstar-cache/hints/` as cache invalidates. fstar-mcp server runs
on `http://localhost:3001/`; use it for incremental typechecking
of the `Hacspec_ml_dsa.Commute.Chunk.fst` lemma additions.

## The three-piece AVX2 plan

The AVX2 free fn `Libcrux_ml_dsa.Simd.Avx2.Arithmetic.reduce` already
proves
```
forall i. to_i32x8 ${simd_unit}_future i ==
          Spec.MLDSA.Math.barrett_red (to_i32x8 ${simd_unit} i)
```
The trait per-lane post wants
```
reduce_lane_post (Seq.index (f_repr orig)   i)
                 (Seq.index (f_repr future) i)
```
Two gaps to bridge: **content of `f_repr`** (Vec256 ↔ i32-array view)
and **shape of `barrett_red`** (raw output ↔ centered Barrett bound +
`% q` congruence).

### Piece 1 — strengthen the intrinsic spec (~20 min)

**File**: `crates/utils/intrinsics/src/avx2_extract.rs:50` (and the
matching extracted F* in
`crates/utils/intrinsics/proofs/fstar/extraction/Libcrux_intrinsics.Avx2.fst:24-31`).

**Current state**:
```fstar
assume
val mm256_storeu_si256_i32':
    output: t_Slice i32 ->
    vector: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)
  -> t_Slice i32
```
No content post. Strengthen via a `#[hax_lib::ensures]` attribute on
the Rust shim so the extracted F* `assume val` carries:
```fstar
ensures (fun out_future ->
    Seq.length out_future == Seq.length output /\
    Seq.length output == 8 ==>
      (forall (i:nat). i < 8 ==>
        Seq.index out_future i == FunArray.f_index (Spec.Intrinsics.to_i32x8 vector) (mk_u64 i)))
```
(adjust to actual `to_i32x8` return-element accessor — check
`Spec.Intrinsics.fsti:124` and `Hacspec_ml_kem.Vector.Traits.Spec`
for the conventional lookup syntax).

Then update `Libcrux_ml_dsa.Simd.Avx2.Vector_type.fst:35-44`'s
`to_coefficient_array` to carry the content guarantee through.

**Caveat**: the intrinsic crate is shared with ml-kem.  Verify
ml-kem still passes (`cd ../libcrux-ml-kem && ./hax.py prove`)
before commit.  Strengthening an `assume val`'s post is monotone (we
only add information), so it should not break consumers — but verify.

**If 20 min runs out**: revert and document.  This is the gating piece;
without it Pieces 2+3 don't help.

### Piece 2 — Barrett-reduction math bridge (~20-30 min)

**File**: append to
`specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`.

**Goal**: prove
```fstar
let lemma_barrett_red_bound_and_mod_q (x: i32)
    : Lemma
        (requires Spec.Utils.is_i32b 2143289343 x)
        (ensures
          Spec.Utils.is_i32b 8380416 (Spec.MLDSA.Math.barrett_red x) /\
          (v (Spec.MLDSA.Math.barrett_red x)) % 8380417 == (v x) % 8380417)
```

`Spec.MLDSA.Math.barrett_red` is defined at
`libcrux-ml-dsa/proofs/fstar/spec/Spec.MLDSA.Math.fst:46-48`:
```fstar
[@@ "opaque_to_smt"]
let barrett_red (x:i32) : i32 =
  let q = shift_right_opaque (add_mod_opaque x (shift_left (mk_i32 1) (mk_i32 22))) (mk_i32 23) in
  sub_mod_opaque x (mul_mod_opaque q v_FIELD_MODULUS)
```
This is centered Barrett reduction by `2^23` (with `q = (x+2^22)/2^23`,
output = `x - q·8380417`).  Math identity: for `|x| ≤ 2^31 - 2^22 - 1
= 2143289343`, `|barrett_red x| < 8380417` and `barrett_red x ≡ x (mod
8380417)`.

**Proof tactic**: reveal `barrett_red`, `add_mod_opaque`,
`sub_mod_opaque`, `mul_mod_opaque`, `shift_right_opaque` opacities;
the `is_i32b` bound follows from arithmetic on `q`'s range and Z3 can
finish; the `% 8380417` congruence is `x - q·8380417 ≡ x (mod 8380417)`
which is just `(x - q*m) % m == x % m`, available as a standard math
lemma in `FStar.Math.Lemmas`.

If Z3 needs more rlimit for the bound, push options up to 200 (do NOT
go above 300 per project policy).  If it still doesn't close,
factor into a `lemma_q_bound (x: i32{is_i32b 2143289343 x}) : Lemma
(let q = ... in v q in [-256, 255])` first.

**Use fstar-mcp**: create a session for `Hacspec_ml_dsa.Commute.Chunk.fst`
and iterate `typecheck_buffer` until the lemma lands.  Do NOT modify the
existing `lemma_reduce_lane_commute` — append.

### Piece 3 — AVX2 impl-method discharge (~15-20 min)

**File**: `libcrux-ml-dsa/src/simd/avx2.rs:363-368`.

The pattern mirrors the Portable proof at
`libcrux-ml-dsa/src/simd/portable.rs:337-378` (commit `c91f0b413`).
The differences:
- `f_repr` for AVX2 returns the i32-array view of `value.f_value`,
  not the `f_values` field of a Coefficients struct.  Reference shape:
  `Seq.index (Libcrux_ml_dsa.Simd.Traits.f_repr X) k` rather than
  `Seq.index X.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k`.
- The free-fn post uses `to_i32x8 X i` not `Seq.index X.f_values i`,
  so the loop_invariant carries the `to_i32x8` shape (after Piece 1
  strengthening, these are equivalent).
- The bridge after the loop applies BOTH
  `lemma_reduce_lane_commute` (existing) AND
  `lemma_barrett_red_bound_and_mod_q` (Piece 2's new lemma).

**Replace the body**:
```rust
fn reduce(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]) {
    hax_lib::fstar!("admit ()");
    for i in 0..simd_units.len() {
        arithmetic::reduce(&mut simd_units[i].value);
    }
}
```

with the loop_invariant + after-loop bridge pattern below.  See §3
for the Portable template; the AVX2 version's loop_invariant
threads:
- (j < v $i) ==> `forall k<8. to_i32x8 (Seq.index simd_units j).value k
  == barrett_red (to_i32x8 (Seq.index _orig j).value k)`
- (j >= v $i) ==> `Seq.index simd_units j == Seq.index _orig j`

After the loop, two `Classical.forall_intro` calls (over j and k):
1. Apply `lemma_barrett_red_bound_and_mod_q` to lift the
   `barrett_red` shape to `is_i32b 8380416 + (v r) % q == (v in) % q`.
2. Apply `lemma_reduce_lane_commute` (existing) to convert into
   `reduce_lane_post`.

**Pre-conditions threaded**: the trait pre is
`is_i32b_array_opaque 2143289343 (f_repr (Seq.index simd_units j))`
for each `j<32`; you'll need a `reveal_opaque` in the loop body or
before the call to convert this into per-element `is_i32b 2143289343`,
which is the precondition of `lemma_barrett_red_bound_and_mod_q`.

## §3 The Portable proof (reference template)

From `libcrux-ml-dsa/src/simd/portable.rs:337-378` (committed at
`c91f0b413`):

```rust
fn reduce(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let _orig = simd_units.clone();

    for i in 0..simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(r#"
            v $i <= 32 /\
            (forall (j:nat{j < 32}). j < v $i ==>
                Spec.Utils.is_i32b_array_opaque 8380416
                    (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values /\
                (forall (k:nat{k < 8}).
                    Spec.MLDSA.Math.mod_q
                        (v (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)) ==
                    Spec.MLDSA.Math.mod_q
                        (v (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)))) /\
            (forall (j:nat{j < 32}). j >= v $i ==>
                Seq.index ${simd_units} j == Seq.index ${_orig} j)"#));

        arithmetic::reduce(&mut simd_units[i]);
    }

    hax_lib::fstar!(r#"
        reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let pf (j: nat{j < 32}) : Lemma
            (ensures Spec.Utils.forall8 (fun (k: nat{k < 8}) ->
                Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
                    (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
                    (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k))) =
            reveal_opaque (`%Spec.Utils.is_i32b_array_opaque)
                (Spec.Utils.is_i32b_array_opaque 8380416
                    (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values);
            let pfk (k: nat{k < 8}) : Lemma
                (ensures Libcrux_ml_dsa.Simd.Traits.Specs.reduce_lane_post
                    (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
                    (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)) =
                Hacspec_ml_dsa.Commute.Chunk.lemma_reduce_lane_commute
                    (Seq.index (Seq.index ${_orig} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
                    (Seq.index (Seq.index ${simd_units} j).Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values k)
            in
            Classical.forall_intro pfk
        in
        Classical.forall_intro pf
    "#);
}
```

**Key lessons from Step 7 (Portable):**

- A first attempt that put `reduce_lane_post` directly in the loop
  invariant (and applied the bridge per-iteration) Z3-cancelled at
  rlimit 80 on the per-iteration subtyping check — too complex per
  step.  The working refinement carries the **free-fn-post shape** in
  the invariant (what `arithmetic::reduce` directly proves) and
  applies the bridge **once after the loop** via two nested
  `Classical.forall_intro` calls.  Mirror this for AVX2.
- Quantified vars must be refined inline (`forall (j:nat{j<32}). ...`)
  not as plain `nat` with implication-side bounds — the latter
  triggers a `Seq.index`-well-formedness Z3 cancel.
- The bridge `reveal_opaque` on `is_i32b_array_opaque` happens
  **per-`j`** inside the outer lemma body (not once outside), so each
  `is_i32b 8380416 (Seq.index … k)` SMTPat fires for the right array
  instance.
- Cloning `_orig` requires a `#[cfg(hax)]` guard so the runtime build
  doesn't compile it.

## Hard rules (binding)

1. **20-minute wall-clock per proof attempt.** On overrun, REVERT
   your edits to that piece, document the obstacle in
   `outstanding-admits.md`, and pick a different piece.  Three Pieces
   above means three independent budgets; do NOT collapse them.
2. **Every function we touch leaves in final form.** No
   "weak-post + admit-removed" passes.  If Piece 1 partially lands
   (post strengthened but consumer not retuned), revert to the stub
   spec until you have a verified end-to-end chain.
3. **Cite `Hacspec_ml_dsa.*` only in new posts**; never
   `Spec.MLDSA.Math` (obsolete, deleted in Phase 4).  EXCEPTION: the
   Piece-2 lemma's `requires`/`ensures` necessarily mentions
   `Spec.MLDSA.Math.barrett_red` since that's the symbol the AVX2
   free-fn post cites.  This is a transitional bridge lemma — same
   exception that `lemma_reduce_lane_commute` already takes for
   `mod_q` reveal at the call site.
4. **Per-element commute lemmas** go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`.
   Do NOT add them elsewhere.
5. **Never `verification_status(lax)`** anywhere.  Only allowed admit
   shapes are `panic_free` on free fns and `hax_lib::fstar!("admit ()")`
   in free-fn bodies, and only for pre-budgeted USER-lane items.
6. **impls are thin one-line wrappers** with `#[requires]+#[ensures]`
   identical to the underlying free fn's; the free fn carries the
   proof body / admit / panic_free.

## Cross-crate verification

After Piece 1's intrinsic spec change, verify ml-kem still passes:

```
cd ../libcrux-ml-kem
./hax.py prove 2>&1 | tail -22
```

If ml-kem regresses, the strengthening was incompatible — investigate
which ml-kem caller relies on the weaker post.  In practice, post
strengthening is monotone for consumers, so a regression means a
ml-kem caller was using the intrinsic spec in a way that contradicts
the new content guarantee, which would be a soundness bug worth
flagging.

## Documentation cadence

After each Piece lands cleanly:
1. Update `proofs/outstanding-admits.md` to remove the now-resolved
   admits (or mark them resolved with the commit SHA).
2. Bump the tip pointer in `MLDSA_STATUS.md` and update the
   per-method P/A column for `reduce`.
3. Commit each Piece as its own discrete commit so a mid-session
   handoff resumes cleanly.

## What success looks like

End-of-session baseline: **98 invoked / [CHECK]=42 / [ADMIT]=56 / 0 errors**
unchanged.  AVX2 column for `reduce` flips from 🟡 to ✅ in
`MLDSA_STATUS.md`'s per-method table.  Body admit at
`src/simd/avx2.rs:364` is gone.  The same approach then unblocks
`add` / `subtract` / `montgomery_multiply` / `shift_left_then_reduce`
/ `power2round` / `decompose` / `compute_hint` / `use_hint` /
`infinity_norm_exceeds` for AVX2 — Step 8/9 work in subsequent
sessions once Pieces 1+2 are in place.

## Stretch goal (only if Pieces 1-3 land cleanly with budget left)

Step 8: try `Operations::add` / `subtract` end-to-end on AVX2.  Both
have the same trait-post structure (per-lane integer arithmetic) and
their AVX2 free fns operate on Vec256.  After Piece 1's intrinsic
post strengthening, the `f_repr` content guarantee is in place;
the bridge for `add`/`subtract` is just per-lane integer addition
(no Barrett reduction involved), so a one-line bridge lemma in
`Commute.Chunk.fst` plus the same loop_invariant pattern should
close both.  Estimate ~30 min if everything is set up.
