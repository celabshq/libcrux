# Next session prompt — above-trait verification lane

Copy the block below into a new session as the user's first message.

---

We are in the middle of a multi-agent ML-DSA verification sprint with
two parallel branches that meet at the
`Libcrux_ml_dsa.Simd.Traits.{fsti,Specs.fst,fst}` contract:

- **Above-trait lane (you)**: branch `ml-dsa-above-trait`, worktree
  `~/libcrux-ml-dsa-above-trait/`.  Verifies `Polynomial`, `Arithmetic`,
  `Ntt`, `Encoding.*`, `Matrix`, `Sample`, `Pre_hash`, `Hash_functions.*`,
  `Ml_dsa_generic.*`.  All `Simd.{Portable,Avx2}.*` are deliberately
  in ADMIT mode here.
- **Below-trait lane**: branch `ml-dsa-proofs`, worktree
  `~/libcrux-ml-dsa-proofs/`.  Verifies the impl bodies in
  `Simd.{Portable,Avx2}.*`.  All above-trait modules are in ADMIT
  mode there.

The two lanes share only the trait contract.  Trait pre/post changes
are owned by the above-trait lane (this branch).  The protocol is in
`libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md`.

## Where we are

Tip: `b097daf01`.  17 modules on this branch are in CHECK mode:
- 4 pre-existing: `Constants`, `Types`, `Polynomial`, `Arithmetic`
- 3 trait boundary: `Simd.Traits.{fsti,Specs.fst,fst}`
- 10 above-trait promoted in the prior session: `Ntt`, `Pre_hash`,
  `Encoding.{T1,Commitment,Error,Gamma1,T0,Verification_key,
  Signing_key,Signature}`

Several have body admits (panic_free) on the harder offset/length
arithmetic — see `proofs/outstanding-admits.md` "Active admits —
above-trait branch".  Pre/post are strong on all of them; body
admits are local cleanup work.

Arithmetic admit pass (commits `8d532957e`+`b097daf01`):
- `power2round_one_ring_element`: ✅ admit removed.
- `power2round_vector`, `use_hint`: strong post; body admit kept.

## Patterns / lessons from prior arithmetic admit pass

Beyond the patterns in MLDSA_STATUS.md "Promotion pattern":

9. **forall8 inside loop_invariant — let-bind deeply-nested field
   accesses**.  Inside `Spec.Utils.forall8 (fun (m: nat{m < 8}) -> ...)`
   referencing a captured polynomial like `Seq.index t1.f_simd_units k`
   triggers `Error 72: Field name f_simd_units could not be resolved`
   from inside the closure.  Workaround: let-bind the simd_unit
   outside the inner Seq.index, and use the fully-qualified field
   path `Libcrux_ml_dsa.Polynomial.f_simd_units`:
   ```fstar
   forall8 (fun (m: nat{m < 8}) ->
     let t1k = Seq.index t1.Libcrux_ml_dsa.Polynomial.f_simd_units k in
     v (Seq.index (i0._super_i2.f_repr t1k) m) >= 0 /\ ...)
   ```

10. **Avoid parameter names that hax mangles** — if a function takes
    a param `t` AND an `&mut` ensures references `${t}_future`, hax
    extracts as `tt_future` (doubled letter) for disambiguation, but
    the ensures body still references `t_future` → unbound name.
    Rename the param (e.g. `t` -> `t0`) to avoid the conflict.

11. **hax IndexMut quirk on second `&mut` arg in fold body**.  When
    a fold-body calls `f(&mut arr1[i], &mut arr2[i])` with both
    accumulator-tracked, F* may resolve `Index` for `arr1` but fail
    to find an instance for `arr2` (Error 228 typeclass constraint
    on `seq T`).  Same type for both args; asymmetric extraction.
    Pragmatic workaround: keep the wrapper pre/post strong, admit
    the body with `panic_free` + `admit ()`.

## Read these first

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — full state, including the
   "Promotion pattern" section with patterns you'll use.
2. `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` —
   sync protocol with below-trait lane.  Pay particular attention
   to the F-1 finding response (option (d)) — same pattern applies
   to any future `[0, q)`-conditional lane post.
3. `libcrux-ml-dsa/proofs/outstanding-admits.md` — body admits on
   this branch, with mitigation hints.
4. `libcrux-ml-dsa/src/simd/traits.rs` — the strengthened trait
   contract.  Do NOT edit unless a new finding makes it necessary;
   any change here cherry-picks back to the below-trait branch.

## Your task — pick from these in order

### 1. `Libcrux_ml_dsa.Matrix.fst` (HIGH PRIORITY)

The biggest remaining win.  Functions in `src/matrix.rs`:
`compute_as1_plus_s2`, `compute_matrix_x_mask`,
`vector_times_ring_element`, `add_vectors`, `subtract_vectors`,
`compute_w_approx`.

Each chains trait calls: `ntt_multiply_montgomery → add → reduce →
invert_ntt_montgomery → add`.  My commit `94e933eb1` strengthened
the trait posts to chain these without re-deriving bounds.  Each
wrapper needs `#[hax_lib::requires/ensures]` forwarding the chain.

**Caller-side fix A.6**: at `src/matrix.rs:117` (in
`compute_w_approx`), insert `reduce(&mut t1[i])` before
`ntt(&mut t1[i])`.  `shift_left_then_reduce` post is
`is_i32b 8380416` (centered Barrett, FIELD_MAX), but ntt's pre is
`NTT_BASE_BOUND = FIELD_MID = 4190208`.  An intermediate reduce
brings the value into the right range.  This is the deferred A.6
from the prior session.

Rough estimate: 30-60 min.

### 2. `Libcrux_ml_dsa.Sample.fst` (medium)

`src/sample.rs` (842 lines).  Uses
`hash_functions::shake128/shake256` Xof traits, both in ADMIT.
Also uses cloop! macros heavily — convert to plain for-loops with
loop_invariants.  rejection_sample_* trait methods are bounds-only
(intentional), so the wrapper-level posts are also bounds-only.

May need length-preservation ensures on more Xof methods (per the
pattern established in `b68738411`: `#[ensures(|_| future(arg).len()
== arg.len())]` for any &mut [u8] arg).

### 3. `Libcrux_ml_dsa.Hash_functions.{Portable,Simd256,Neon}.fst`
(low-medium)

When Sample needs them.  Same length-preservation pattern.  Likely
admit body of Xof impls — these are platform-specific FFI shims.

### 4. `Libcrux_ml_dsa.Ml_dsa_generic.fst` and instantiations (high)

Top-level orchestrator: keygen, sign, verify, sign_pre_hashed,
verify_pre_hashed.  Largest blast radius; tackle after #1-#3 land.

## Patterns / gotchas (from prior session)

1. **Use plain for-loops, not cloop!**.  cloop's
   fold-of-tuple-accumulator gives a `True` loop_invariant that loses
   length/bound facts.  Replace
   `cloop! { for (i, x) in arr.iter().enumerate() { ... } }` with
   `for i in 0..arr.len() { let x = &arr[i]; ... }` and add
   `hax_lib::loop_invariant!` inside.

2. **`#[hax_lib::attributes]` on trait declarations**.  Without it,
   `#[requires]/[ensures]` don't propagate to extracted F*'s
   `f_*_pre`/`f_*_post`.  Same on impl blocks.

3. **Length-preservation for `&mut [u8]` args**: add
   `#[ensures(|_| future(arg).len() == LITERAL)]` (literal length,
   not `arg.len()` — hax sometimes emits `true` for the `arg.len()`
   form when the body's length-preservation isn't trivial).

4. **gamma2-conditional bounds**: when a bound depends on γ₂, write
   it as `(v gamma2 == 95232 ==> bound1) /\ (v gamma2 == 261888
   ==> bound2)` instead of `is_i32b_array_opaque (v gamma2) ...`.
   The `(v gamma2)` form fails F* subtyping (expects nat).  See
   `36fe89b18` for an example.

5. **Targeted make**:
   ```
   cd libcrux-ml-dsa/proofs/fstar/extraction
   make /Users/karthik/libcrux-ml-dsa-above-trait/.fstar-cache/checked/$MODULE.fst.checked
   ```
   Don't run `./hax.sh prove` — it verifies all 80+ modules and
   takes too long.  Cache is warm from prior session; targeted
   builds are 5-30s typically.

6. **F-1 pattern**: `decompose`/`compute_hint`/`use_hint` lane posts
   are `==>`-conditional on `[0, q)`.  This is FINE for the trait
   contract.  Do NOT tighten the trait pre — instead, when below-trait
   reports a stuck-query, the impl-side commute lemma should match
   the `==>` shape.  Same for any future similar finding.

7. **Cache-warm typecheck only**.  Never `rm -rf .fstar-cache/`
   — cascade re-prove costs ~12-15 min.  If make says "nothing to
   do" but you know a transitive dep changed: `touch` the changed
   file then re-make.

8. **Develop locally, upstream once**.  New spec lemmas go in the
   consumer file (or a sandbox), not in `Specs.fst` until the shape
   is final.  Touching shared spec files cascades 30+ min.

## Build commands

```bash
# Full extract (~5 sec)
cd ~/libcrux-ml-dsa-above-trait/libcrux-ml-dsa
./hax.sh extract

# Targeted check
cd proofs/fstar/extraction
make ~/libcrux-ml-dsa-above-trait/.fstar-cache/checked/Libcrux_ml_dsa.MODULE.fst.checked

# After landing each module: extend VERIFIED_MODULES in
# proofs/fstar/extraction/Makefile, commit (one commit per module
# or per logical batch).
```

## Hard rules (carried forward)

1. Trait pre/post is final at `36fe89b18` unless a concrete
   stuck-query reappears after impl-side restructuring (option (d)).
2. Cite `Hacspec_ml_dsa.*` only in new posts; never `Spec.MLDSA.Math`.
3. Body admit (`panic_free` + `admit ()`) is acceptable when the
   body has multi-step offset arithmetic / multi-write copy_from_slice
   that doesn't fit a simple loop_invariant.  Document each in
   `outstanding-admits.md`.
4. Never `verification_status(lax)`.
5. Each module promotion = one commit.  Commit message:
   `above-trait C.X: promote Libcrux_ml_dsa.MODULE.fst to CHECK`
   plus what worked / what's admitted.
6. `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
   on agent-authored commits.

Start with #1 (Matrix).  Use auto mode unless you're at a real
decision point that needs human input.
