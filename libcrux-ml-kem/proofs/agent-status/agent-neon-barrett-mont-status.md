# Neon barrett / montgomery — functional upgrade status (FINAL)

Base: Neon-0-lax milestone 9b11790cf. Goal: upgrade neon `barrett_reduce` +
`montgomery_multiply_by_constant` panic_free -> fully functional by modeling
`_vqdmulhq_*` and mirroring AVX2/portable barrett+montgomery proofs.

## Outcome

- DELIVERED: a sound, faithful F* model of the saturating doubling
  multiply-high intrinsics `_vqdmulhq_n_s16` / `_vqdmulhq_s16` in
  `crates/utils/intrinsics/src/arm64_extract.rs` (previously empty `ensures`).
  The intrinsics fsti `Libcrux_intrinsics.Arm64_extract.fsti` verifies clean.
- DEFERRED (cliff): the functional `ensures` on neon `barrett_reduce` and
  `montgomery_multiply_by_constant`. Both remain panic_free (unchanged from the
  base). See cliff below. `make all` exit 0, cargo simd128 self test 18/18 pass,
  `grep -c 'admit ()'` on Arithmetic.fst == 0.

## (a) Functions now functional

None upgraded to Math tier. barrett_reduce / montgomery_multiply_by_constant
stay panic_free. The NET improvement vs base is the sound `_vqdmulhq_*`
intrinsic model (was no-spec), which unblocks a future functional proof once
the vector-forall composition wall (below) is cracked.

`make all`: build f9eb68c2, status ok, exit 0.
intrinsics fsti: build 06a3f924, status ok, exit 0.
Neon Arithmetic.fst: build 26253472, status ok, exit 0 (panic_free, 0 admits).

## (b) The `_vqdmulhq_*` model settled on (SOUND, saturating)

Per lane i, `result_i = sat16( (2 * a_i * b) >> 16 )`, shift = `/ pow2 16`,
`sat16(x) = if x>32767 then 32767 else if x < -32768 then -32768 else x`:

```
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 8}).
    (let prod = (2 * (v (get_lane_i16x8 $k i)) * (v $b)) / pow2 16 in
     get_lane_i16x8 $result i ==
       (if prod > 32767 then mk_i16 32767
        else if prod < (-32768) then mk_i16 (-32768)
        else mk_i16 prod))"))]
pub fn _vqdmulhq_n_s16(k: _int16x8_t, b: i16) -> _int16x8_t
```
`_vqdmulhq_s16(a, c)` identical with `v b` -> `v (get_lane_i16x8 $c i)`.
NOTE: `_vqdmulhq_s16`'s first param was renamed `v`->`a` (the F* value-coercion
`v` shadowed it, breaking the post). The Rust rename is positional-safe.

In barrett the input bound `is_i16b 28296` makes `prod` in [-17414,17414] (no
sat); in montgomery `|k|<=2^15,|c|<=1664` keeps the doubling-high product in
range too — so the sat conditionals collapse to the middle branch. This was
PROVEN: the helper lemma `lemma_neon_barrett_lane` (floor identity
`(2x*20159)>>16 == (x*20159)>>15`, and the +1024/>>11 vs spec +512/>>10 collapse
exactly) verified standalone, as did a factored per-lane lemma
`lemma_barrett_reduce_int16x8_t_lane` proving
`get_lane (barrett_reduce_int16x8_t a) i == Spec.Utils.barrett_red (get_lane a i)`
and thus `is_i16b 3328` + `% 3329` equality.

## (c) Gates

- `make all` (targets ["all"]): exit 0 (build f9eb68c2-119b-4296-bcef-4fb42e6d2d8c).
- `cargo test --features simd128,mlkem512 --test self`: 18 passed; 0 failed
  (includes consistency_unpacked_*_neon and modified_ciphertext paths).
- `grep -c 'admit ()' Libcrux_ml_kem.Vector.Neon.Arithmetic.fst` == 0.
- Vector.Neon.{fst,fsti} remain in ADMIT_MODULES (untouched, per scope).

## (f) The cliff (smtprofile-grade diagnosis)

barrett (and identically montgomery): the PER-LANE functional fact is fully
proven (the factored top-level lemma verifies in its own query). The wall is the
VECTOR-level composition: the function postcondition `forall i. P (Seq.index
(repr result) i)` (or the int16x8 helper's `forall i<8. P (get_lane result i)`)
cannot be discharged from the per-lane `introduce forall` / `Classical.forall_intro`
/ 8 ground `aux k` calls / `Spec.Utils.forall8`.

Evidence (every variant, ~10 builds): the failing query is ALWAYS the final
(post) query, reason `unknown because (incomplete quantifiers)`, failing FAST at
rlimit 1.4-7.5 (e.g. 65/95/127 ms; never "canceled"/"resource limits reached").
Per the smtprofiling taxonomy this is a quantifier-INSTANTIATION/trigger failure
(the trigger linking the introduced per-lane forall to the post's skolem index
does not fire), NOT a quantifier cascade (would burn high rlimit) and NOT a
missing fact (the per-lane lemma proves it standalone). The same `introduce
forall (j:nat{j<16}) ... with lemma_repr_index` pattern that the verified
`cond_subtract_3329_` uses in this very file did NOT transfer — the difference is
that cond_subtract's per-lane work is a single `logand_lemma`, whereas barrett's
per-lane work routes through `lemma_barrett_red`/`lemma_neon_barrett_lane` whose
`pow2`-SMTPat baggage appears to poison trigger selection on the outer forall
even after factoring into a top-level lemma and with `--ext context_pruning`.

Variants tried (all hit the same post query): bound+mod ensures vs barrett_red
ensures; `introduce forall` vs `Classical.forall_intro` vs `move_requires` vs 8
ground `aux k` vs `Spec.Utils.forall8`; ifuel 1/2; `--split_queries always`
(localized failure to the LAST query = the post); `{:pattern}` on the ensures
forall; bounded `forall (i:nat). i<16 ==>` vs unbounded `forall i`; helper
post over `get_lane` vs whole proof over `repr` with `lemma_repr_index`; helper
lemma inlined vs factored top-level. `--split_queries always` + `move_requires`
also caused a pathological >6-min hang (cancelled).

### Suggested next-session approach
Treat this as the documented "vector-level forall saturates Z3 / no first-order
congruence" class (MEMORY: ground-literal SIMD proofs; createi cascade). Likely
fix: a per-INDEX opaque-atom wrapper around the lane post (so the outer forall's
trigger is the opaque atom, not the raw `get_lane`/`%`), mirroring the
`barrett_reduce_lane_post` opaque atom that the trait layer already uses — i.e.
do the bridge AT THE TRAIT boundary (in a neon `op_barrett_reduce` wrapper like
portable.rs has) rather than on the free int16x8 helper, where `lemma_repr_index`
+ the opaque `barrett_reduce_lane_post` fold cleanly. Montgomery is structurally
identical (route `montgomery_reduce_int16x8_t` lanes through `mont_mul_red_i16`
via the u16-detour reinterpret bridge `cast_mod(cast_mod low *. 62209) == low *.
neg 3327`, plus `((2P)>>16)>>1 == P>>16`), so the same fix unblocks both.

## Files touched
- crates/utils/intrinsics/src/arm64_extract.rs  (vqdmulh model + param rename)
- (extraction regenerated all proofs/fstar/extraction/*.fst{,i}; only the
  intrinsic fsti content changed semantically)
NOT committed (parent commits).
