# Session prompt — Neon functional parity + impl close (finishes Neon)

## Where we are (verified facts)

After the 0-lax wave (commit 9b11790cf) + barrett/montgomery functional
increment, Neon stands at: **all LEAF modules out of ADMIT_MODULES**
(Vector_type, Arithmetic, Compress, Ntt, Serialize), with ONLY
`Vector.Neon.{fst,fsti}` (the `impl Operations for SIMD128Vector`, 38 fns,
the `vector` status row) still admitted.

THE impl gate is the whole game now. It cannot be un-admitted until every
free fn it dispatches to is FUNCTIONAL, because the `Operations` trait
(libcrux-ml-kem/src/vector/traits.rs) declares functional pre/post on EVERY
method (add_post, ntt_layer_1_step_post, serialize_1_post, compress_post,
...). The Neon impl methods are already clean one-line dispatchers
(`fn add(l,r)=add(l,r)` etc. — NO Track-B inline-record anti-pattern), so
once the callees carry the trait posts, the impl should fall out with little
extra work + `--split_queries always` on the impl block.

### Functional status of the leaf fns (what still needs Math/Bounds)
- Arithmetic: add/sub/multiply_by_constant/cond_subtract_3329/
  to_unsigned_representative = FUNCTIONAL (sprint 1). barrett_reduce +
  montgomery_multiply_by_constant = FUNCTIONAL if the barrett/montgomery
  increment landed (check git log / status doc); else still panic_free.
- Compress (7 fns), Ntt (7 fns), Serialize (12 fns) = panic_free only.
  These are the bulk of the remaining work and need new intrinsic models.

## The hard part: intrinsic models still to write (shared arm64_extract.rs)

These live in crates/utils/intrinsics/src/arm64_extract.rs. Each addition =
re-extract (whole crate, ~2.5 min) + rebuild Libcrux_intrinsics.Arm64_extract
+ cascade. THIS IS A SERIALIZATION POINT: agents that add intrinsic posts
CANNOT run in parallel (they'd clobber arm64_extract.rs + race the extract).
Either (a) one agent does ALL intrinsic models up-front in one pass, extract
once, commit, THEN fan out functional proof agents (which only edit their own
.rs + iterate .fst-direct, no more intrinsic edits) — RECOMMENDED; or (b) do
each module fully sequentially.

Models needed, by consumer:
- **Compress functional**: `_vqdmulhq_n_s32` (32-bit doubling-mul-high,
  saturating — same shape as the s16 version the barrett increment modeled,
  but i32→i64 intermediate, sat32). compress_int32x4_t uses it.
- **Ntt functional** (the widening-MAC group — hardest after table lookup):
  `_vmull_s16`/`_vmull_high_s16` (widening multiply: 4×i16→4×i32, low/high
  halves), `_vmlal_s16`/`_vmlal_high_s16` (widening multiply-accumulate).
  Plus the zip/transpose used to interleave: `_vtrn1q_s16/s32/s64`,
  `_vtrn2q_s16/s32/s64`, `_vget_low_s16`, and the byte-table `_vqtbl1q_u8`
  with the s16↔u8 reinterprets. The NTT butterfly's functional spec is the
  per-lane FE-algebra (see AVX2 ntt.rs which reaches Bounds — mirror its
  ntt_layer_N_step posts and Spec.Utils butterfly helpers).
- **Serialize functional** (Math, 22 in AVX2): `_vqtbl1q_u8` (arbitrary byte
  permute by a runtime index vector — THE hard model; bit/byte-level, model
  as `result[i] = if idx[i] < 16 then table[idx[i]] else 0`), `_vsliq_n_s32`
  /`_vsliq_n_s64` (shift-left-and-insert: `(dst & low_mask) | (src << n)`),
  `_vaddv_u16`/`_vaddvq_s16` (horizontal add across lanes), `_vget_high_u16`
  /`_vget_low_u16` (64-bit half extraction), `_vshlq_s16`/`_vshlq_u16`
  (per-lane variable shift), and the many reinterprets (s16↔u16↔u8↔s32↔s64↔
  u32). AVX2 serialize (avx2/serialize.rs + the op_serialize_N_{pre,post}_
  bridge lemmas, all five N proven, commit 6e7cb0566) is the blueprint;
  Neon needs vec128 (two-i16x8-halves) analogues of the bit_vec decomposition
  lemmas, factoring through repr's `Seq.append`.

The realistic effort here is multiple sessions — `_vqtbl1q_u8` and the
widening-MAC chain are the same class of work that took AVX2's serialize/ntt
several sprints. Cross-validate every model with a bit-exact Python sim
before F* iteration (ground-literal SIMD recipe; memory:
project_avx2_ntt_leaf_cascade_recipe, feedback_ground_literal_simd_proofs).

## Plan
1. **Intrinsic-model pass** (1 agent, foreground, serial): model the above in
   arm64_extract.rs. Faithful saturating semantics for the *hq* ops; bit/byte
   for qtbl/sli; lane for vmull/vmlal/vtrn/vget/vaddv/vshl. Re-extract once,
   re-verify Libcrux_intrinsics.Arm64_extract, commit.
2. **Fan out functional proof agents** (compress / ntt / serialize), now
   independent (no more shared intrinsic edits): each edits ONLY its .rs,
   iterates .fst-direct, NO whole-crate extract. Parent integrates with one
   extract + make all. Mirror AVX2's posts (compress→Math, ntt→Bounds,
   serialize→Math) stated over `(repr ${v})` via lemma_repr_index.
3. **Close the impl** (parent): remove Vector.Neon.{fst,fsti} from
   ADMIT_MODULES; add the trait method contracts (bare #[requires]/#[ensures]
   under #[hax_lib::attributes], citing spec::*_pre/post over impl.f_repr) —
   the dispatchers already forward, so this should mostly propagate; add
   `#[cfg_attr(hax, hax_lib::fstar::options("--split_queries always"))]` on
   the impl block (Track-B companion fix). Re-extract, make all. Neon → 0 lax
   (except the 1 `sampling` unverified fn, which needs Karthik's OK to
   re-enable `mod sampling` — a CODE change, out of scope).

## Constraints (unchanged)
NO runtime code changes (annotations + rename/let-binding only). Max make -j2,
≤4 fstar+z3 per agent. rlimit 800/400-split. smtprofiling before any cliff
claim. 30-60 min/fn budget. Ensures cite Hacspec_ml_kem.*/Traits.Spec.*/
Spec.Utils.*, never Spec.MLKEM. cargo-hax 0.3.7. Proxy on :3002, fstar tools
may need the /tmp/fp.sh curl wrapper (fstar_typecheck needs a `code` field).
`v`-named params shadow F*'s `v` coercion — rename to `vec` or fully-qualify
`Rust_primitives.Integers.v`. hax emits #[requires] into the .fsti `val` (a
requires in the .fst `let` is silently ignored). Commit per agent-mlkem:
convention (no push); per-stage clean rebuild (rm touched .checked, real
Query-stats).

## Exit criteria
Vector.Neon.{fst,fsti} out of ADMIT_MODULES; Neon `vector` row 38 lax → 0
(modulo the 1 sampling-unverified fn). Full make all green; cargo simd128
512/768/1024 green. Status doc + perf top-20 refreshed.
