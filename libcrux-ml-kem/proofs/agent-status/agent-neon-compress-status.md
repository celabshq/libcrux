# Agent status: Libcrux_ml_kem.Vector.Neon.Compress panic-freedom (0-lax)

Status: **DONE** — module verifies clean at panic_free tier, 0 admits.

## Final verification evidence
Build id `5e85c484-3b58-4b0c-a6f9-f76fd5fdd7e2`, status `ok`, exit 0.

```
Verified module: Libcrux_ml_kem.Vector.Neon.Compress
Query-stats (..compress, 1)                          succeeded in 185 ms, fuel 0 ifuel 1 rlimit 80 (used 0.937)
Query-stats (..mask_n_least_significant_bits, 1)     succeeded in 124 ms, fuel 1 ifuel 1 rlimit 80 (used 0.309)
Query-stats (..decompress_uint32x4_t, 1)             succeeded in  86 ms, fuel 0 ifuel 1 rlimit 80 (used 0.052)
Query-stats (..decompress_1_, 1)                     succeeded in  99 ms, fuel 0 ifuel 1 rlimit 80 (used 0.107)
Query-stats (..decompress_1_, 2)                     succeeded in 117 ms, fuel 0 ifuel 1 rlimit 80 (used 0.230)
Query-stats (..decompress_ciphertext_coefficient, 1) succeeded in  80 ms, fuel 0 ifuel 1 rlimit 80 (used 0.325)
Query-stats (..compress_1_, 1)                       succeeded in 127 ms, fuel 0 ifuel 1 rlimit 80
Query-stats (..compress_int32x4_t, 1)                succeeded in  88 ms, fuel 0 ifuel 1 rlimit 80
```
`grep -c 'admit ()'` on the .fst = **0**. `cargo check --features simd128` passes (only pre-existing warnings).

## Panic-freedom obligations found + how discharged
All obligations were PURE F* arithmetic (the arm64 intrinsics used here are all `Prims.l_True` except
`e_vshrq_n_s16` which is only ever called with the constant `mk_i32 15`). No new intrinsic `requires`
needed; no blocker for the parent.

1. `mask_n_least_significant_bits` — catch-all `(1 << x) - 1` shift-range + no-underflow.
2. `compress` — discharges (1)'s precondition via the COEFFICIENT_BITS membership set.
3. `decompress_uint32x4_t` — `1 << (COEFFICIENT_BITS - 1)`: subtraction no-underflow + u32 shiftval [0,32).
4. `decompress_1` — `sub z a` requires `sub_pre` (lane bound `is_intb (2^15-1) (0 - a_i)`).
5. `decompress_ciphertext_coefficient` — discharges (3)'s precondition via membership set.

These mirror the avx2 + portable backends' requires exactly (same {4,5,10,11} domain on the
compress/decompress ciphertext fns; same {0,1}-lanes domain on decompress_1), so cross-backend trait
consistency holds and existing callers already satisfy them.

## EXACT edits to libcrux-ml-kem/src/vector/neon/compress.rs

1. `mask_n_least_significant_bits`:
   - Added `#[hax_lib::requires(fstar!(r#"v $coefficient_bits >= 0 /\ v $coefficient_bits < 15"#))]`
   - Catch-all arm `x => (1 << x) - 1` became a block with
     `hax_lib::fstar!(r#"FStar.Math.Lemmas.pow2_le_compat 14 (v $x); assert_norm (pow2 14 == 16384)"#);`
     before `(1 << x) - 1`.

2. `compress<const COEFFICIENT_BITS: i32>`:
   - Added requires (membership): `Rust_primitives.Integers.v $COEFFICIENT_BITS == 4 \/ == 5 \/ == 10 \/ == 11`
     (qualified `v` because the vector parameter is also named `v`).
   - First body stmt: `hax_lib::fstar!(r#"assert (Rust_primitives.Integers.v (cast ($COEFFICIENT_BITS <: i32) <: i16) == Rust_primitives.Integers.v $COEFFICIENT_BITS)"#);`

3. `decompress_uint32x4_t<const COEFFICIENT_BITS: i32>`:
   - Added `#[hax_lib::requires(fstar!(r#"Rust_primitives.Integers.v $COEFFICIENT_BITS >= 1 /\ Rust_primitives.Integers.v $COEFFICIENT_BITS <= 32"#))]`

4. `decompress_1`:
   - Added `#[hax_lib::fstar::before(interface, r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#)]`
   - Added requires: `forall i. (let x = Seq.index (repr ${a}) i in x == mk_i16 0 \/ x == mk_i16 1)`
   - After `let z = ZERO();`:
     `hax_lib::fstar!(r#"assert (forall i. Seq.index (repr ${z}) i == mk_i16 0); assert (forall i. Spec.Utils.is_intb (pow2 15 - 1) (Rust_primitives.Integers.v (Seq.index (repr ${z}) i) - Rust_primitives.Integers.v (Seq.index (repr ${a}) i)))"#);`

5. `decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>`:
   - Added requires (membership): same {4,5,10,11} form as `compress` (qualified `v`).

## Notes for the parent integration
- `v`-shadowing: `compress`, `decompress_uint32x4_t`, `decompress_ciphertext_coefficient` all have a
  parameter named `v`, which shadows `Rust_primitives.Integers.v`. I qualified `v` as
  `Rust_primitives.Integers.v` in those annotations. If hax's antiquote handles this for `$X` forms
  automatically the qualification is harmless; it is required for the bare `cast ... <: i16` assert.
- The `.fsti` (Libcrux_ml_kem.Vector.Neon.Compress.fsti) was also edited as scratch (requires moved
  into the `val` signatures) to mirror where hax emits requires. The parent's re-extract regenerates
  both .fst and .fsti from the .rs.
- No functional `ensures` added (deferred to a later sprint, per instructions).
