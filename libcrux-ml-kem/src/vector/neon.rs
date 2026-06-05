//! Vectors for libcrux using aarch64 (neon) intrinsics

#[cfg(hax)]
use super::traits::{spec, Repr};
use super::{Operations, FIELD_MODULUS};
#[cfg(hax)]
use hax_lib::prop::ToProp;

// mod sampling;
mod arithmetic;
mod compress;
mod ntt;
mod serialize;
mod vector_type;

use arithmetic::*;
use compress::*;
use ntt::*;
use serialize::*;
pub(crate) use vector_type::SIMD128Vector;
use vector_type::*;

#[cfg(hax)]
impl crate::vector::traits::Repr for SIMD128Vector {
    fn repr(&self) -> [i16; 16] {
        to_i16_array(self.clone())
    }
}

#[cfg(any(eurydice, not(hax)))]
impl crate::vector::traits::Repr for SIMD128Vector {}


// =====================================================================
// `op_*` wrappers — Track B trait-layer plumbing (mirrors avx2.rs).
//
// Each `op_*` carries the *exact* trait pre/post for its
// `impl Operations for SIMD128Vector` counterpart so the impl method is a
// one-line `op_<name>(args)` call (trait subtyping is then `P ==> P`).
// Methods whose underlying `SIMD128Vector -> SIMD128Vector` primitive
// already carries the trait pre/post (add/sub/multiply_by_constant,
// barrett_reduce, montgomery_multiply_by_constant, serialize_5/11,
// deserialize_1/4/5/10/11, ZERO/from/to/bytes) need no wrapper — the impl
// method calls the primitive directly and is fully verified.
//
// `op_ntt_multiply` is fully PROVEN: the primitive's
// `ntt_multiply_butterfly_post` (closed this sprint) plus the four
// backend-agnostic `Commute.Chunk.lemma_ntt_multiply_branch_{0..3}`
// discharge the trait post, exactly as portable.rs.
//
// The remaining wrappers carry `panic_free` (body panic-checked + the
// primitive's precondition discharged via the `is_i16b_array_opaque`
// reveal; the strengthened FE-form / mod_q_eq trait post is admitted at
// this layer — same rung AVX2 uses for its compress/decompress/NTT-layer
// wrappers).  `op_rej_sample` is `lax` (the portable-fallback loop's
// panic-freedom is out of scope here).  Upgrading these to fully-proven
// is the Neon mirror of the portable C4f work.
// =====================================================================

// PROVEN (mirrors avx2.rs): the primitive gives `v y % 3329 == v x % 3329`
// per lane; fold into the opaque `mod_q_eq` form the trait post expects.
#[inline(always)]
#[hax_lib::requires(spec::cond_subtract_3329_pre(&vector.repr()))]
#[hax_lib::ensures(|out| spec::cond_subtract_3329_post(&vector.repr(), &out.repr()))]
fn op_cond_subtract_3329(vector: SIMD128Vector) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    let result = cond_subtract_3329(vector);
    hax_lib::fstar!(
        r#"let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${vector}) i)))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${vector}) i))
        in
        Classical.forall_intro aux"#
    );
    result
}

// PROVEN (mirrors avx2.rs): same mod_q_eq fold.
#[inline(always)]
#[hax_lib::requires(spec::to_unsigned_representative_pre(&a.repr()))]
#[hax_lib::ensures(|out| spec::to_unsigned_representative_post(&a.repr(), &out.repr()))]
fn op_to_unsigned_representative(a: SIMD128Vector) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    let result = to_unsigned_representative(a);
    hax_lib::fstar!(
        r#"let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${a}) i)))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${a}) i))
        in
        Classical.forall_intro aux"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::compress_1_pre} (impl.f_repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::compress_1_post} (impl.f_repr $vector) (impl.f_repr $out)"#))]
fn op_compress_1(vector: SIMD128Vector) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    compress_1(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::compress_pre} (impl.f_repr $vector) $COEFFICIENT_BITS"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::compress_post} (impl.f_repr $vector) $COEFFICIENT_BITS (impl.f_repr $out)"#))]
fn op_compress<const COEFFICIENT_BITS: i32>(vector: SIMD128Vector) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    compress::<COEFFICIENT_BITS>(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::decompress_1_pre} (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::decompress_1_post} (impl.f_repr $a) (impl.f_repr $out)"#))]
fn op_decompress_1(a: SIMD128Vector) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    decompress_1(a)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::decompress_ciphertext_coefficient_pre} (impl.f_repr $vector) $COEFFICIENT_BITS"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::decompress_ciphertext_coefficient_post} (impl.f_repr $vector) $COEFFICIENT_BITS (impl.f_repr $out)"#))]
fn op_decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(vector: SIMD128Vector) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    ntt_layer_2_step(vector, zeta0, zeta1)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    ntt_layer_3_step(vector, zeta)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    inv_ntt_layer_2_step(vector, zeta0, zeta1)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    inv_ntt_layer_3_step(vector, zeta)
}

#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 0 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_multiply_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_multiply_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
#[inline(always)]
fn op_ntt_multiply(lhs: &SIMD128Vector, rhs: &SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);
    let out = ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Spec.Utils.ntt_multiply_butterfly_post)
             (Spec.Utils.ntt_multiply_butterfly_post (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${lhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${rhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${out}) zeta0 zeta1 zeta2 zeta3);
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_0 (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${lhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${rhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${out}) zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_1 (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${lhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${rhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${out}) zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_2 (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${lhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${rhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${out}) zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_3 (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${lhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${rhs}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${out}) zeta0 zeta1 zeta2 zeta3"#
    );
    out
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr $vector) $out"#))]
fn op_serialize_1(vector: SIMD128Vector) -> [u8; 2] {
    serialize_1(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr $vector) $out"#))]
fn op_serialize_4(vector: SIMD128Vector) -> [u8; 8] {
    serialize_4(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr $vector) $out"#))]
fn op_serialize_10(vector: SIMD128Vector) -> [u8; 20] {
    serialize_10(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr $vector) $out"#))]
fn op_serialize_12(vector: SIMD128Vector) -> [u8; 24] {
    serialize_12(vector)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(bytes.len() == 24)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 24 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 $bytes (impl.f_repr $out)"#))]
fn op_deserialize_12(bytes: &[u8]) -> SIMD128Vector {
    deserialize_12(bytes)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(input.len() == 24 && out.len() == 16)]
#[hax_lib::ensures(|result| (future(out).len() == 16 && result <= 16).to_prop() & (
        hax_lib::forall(|j: usize|
            hax_lib::implies(j < result,
                future(out)[j] >= 0 && future(out)[j] <= 3328))))]
fn op_rej_sample(input: &[u8], out: &mut [i16]) -> usize {
    rej_sample(input, out)
}

#[hax_lib::attributes]
#[cfg_attr(hax, hax_lib::fstar::options("--split_queries always"))]
impl Operations for SIMD128Vector {
    #[inline(always)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == Seq.create 16 (mk_i16 0)"#))]
    fn ZERO() -> Self {
        ZERO()
    }

    #[requires(array.len() == 16)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == $array"#))]
    fn from_i16_array(array: &[i16]) -> Self {
        from_i16_array(array)
    }

    #[ensures(|out| fstar!(r#"out == impl.f_repr $x"#))]
    fn to_i16_array(x: Self) -> [i16; 16] {
        to_i16_array(x)
    }

    #[requires(array.len() >= 32)]
    #[ensures(|out| fstar!(r#"sz (Seq.length ${array}) >=. sz 32 ==>
                              (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
                               Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
                                 #(mk_usize 16) head (impl.f_repr ${out}))"#))]
    fn from_bytes(array: &[u8]) -> Self {
        from_bytes(array)
    }

    #[requires(bytes.len() >= 32)]
    #[ensures(|_| fstar!(r#"sz (Seq.length bytes_future) =. sz (Seq.length ${bytes}) /\
                            (sz (Seq.length bytes_future) >=. sz 32 ==>
                             (let head : t_Slice u8 = Seq.slice bytes_future 0 32 in
                              Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N
                                #(mk_usize 16) (impl.f_repr ${x}) head))"#))]
    fn to_bytes(x: Self, bytes: &mut [u8]) {
        to_bytes(x, bytes)
    }

    #[requires(fstar!(r#"${spec::add_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs})"#))]
    #[ensures(|result| fstar!(r#"${spec::add_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) (impl.f_repr ${result})"#))]
    fn add(lhs: Self, rhs: &Self) -> Self {
        add(lhs, rhs)
    }

    #[requires(fstar!(r#"${spec::sub_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs})"#))]
    #[ensures(|result| fstar!(r#"${spec::sub_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) (impl.f_repr ${result})"#))]
    fn sub(lhs: Self, rhs: &Self) -> Self {
        sub(lhs, rhs)
    }

    #[requires(fstar!(r#"${spec::multiply_by_constant_pre} (impl.f_repr ${vec}) c"#))]
    #[ensures(|result| fstar!(r#"${spec::multiply_by_constant_post} (impl.f_repr ${vec}) c (impl.f_repr ${result})"#))]
    fn multiply_by_constant(vec: Self, c: i16) -> Self {
        multiply_by_constant(vec, c)
    }

    #[requires(spec::cond_subtract_3329_pre(&vector.repr()))]
    #[ensures(|out| spec::cond_subtract_3329_post(&vector.repr(), &out.repr()))]
    fn cond_subtract_3329(vector: Self) -> Self {
        op_cond_subtract_3329(vector)
    }

    #[requires(spec::barrett_reduce_pre(&vector.repr()))]
    #[ensures(|result| spec::barrett_reduce_post(&vector.repr(), &result.repr()))]
    fn barrett_reduce(vector: Self) -> Self {
        barrett_reduce(vector)
    }

    #[requires(spec::montgomery_multiply_by_constant_pre(&vector.repr(), constant))]
    #[ensures(|result| spec::montgomery_multiply_by_constant_post(&vector.repr(), constant, &result.repr()))]
    fn montgomery_multiply_by_constant(vector: Self, constant: i16) -> Self {
        montgomery_multiply_by_constant(vector, constant)
    }

    #[requires(spec::to_unsigned_representative_pre(&a.repr()))]
    #[ensures(|result| spec::to_unsigned_representative_post(&a.repr(), &result.repr()))]
    fn to_unsigned_representative(a: Self) -> Self {
        op_to_unsigned_representative(a)
    }

    #[requires(fstar!(r#"${spec::compress_1_pre} (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"${spec::compress_1_post} (impl.f_repr $vector) (impl.f_repr $out)"#))]
    fn compress_1(vector: Self) -> Self {
        op_compress_1(vector)
    }

    #[requires(fstar!(r#"${spec::compress_pre} (impl.f_repr $vector) $COEFFICIENT_BITS"#))]
    #[ensures(|out| fstar!(r#"${spec::compress_post} (impl.f_repr $vector) $COEFFICIENT_BITS (impl.f_repr $out)"#))]
    fn compress<const COEFFICIENT_BITS: i32>(vector: Self) -> Self {
        op_compress::<COEFFICIENT_BITS>(vector)
    }

    #[requires(fstar!(r#"${spec::decompress_1_pre} (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"${spec::decompress_1_post} (impl.f_repr $a) (impl.f_repr $out)"#))]
    fn decompress_1(a: Self) -> Self {
        op_decompress_1(a)
    }

    #[requires(fstar!(r#"${spec::decompress_ciphertext_coefficient_pre} (impl.f_repr $vector) $COEFFICIENT_BITS"#))]
    #[ensures(|out| fstar!(r#"${spec::decompress_ciphertext_coefficient_post} (impl.f_repr $vector) $COEFFICIENT_BITS (impl.f_repr $out)"#))]
    fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(vector: Self) -> Self {
        op_decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(vector)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
    fn ntt_layer_1_step(vector: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
    fn ntt_layer_2_step(vector: Self, zeta0: i16, zeta1: i16) -> Self {
        op_ntt_layer_2_step(vector, zeta0, zeta1)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
    fn ntt_layer_3_step(vector: Self, zeta: i16) -> Self {
        op_ntt_layer_3_step(vector, zeta)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
    fn inv_ntt_layer_1_step(vector: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
    fn inv_ntt_layer_2_step(vector: Self, zeta0: i16, zeta1: i16) -> Self {
        op_inv_ntt_layer_2_step(vector, zeta0, zeta1)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
    fn inv_ntt_layer_3_step(vector: Self, zeta: i16) -> Self {
        op_inv_ntt_layer_3_step(vector, zeta)
    }

    #[requires(fstar!(r#"${spec::ntt_multiply_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_multiply_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
    fn ntt_multiply(lhs: &Self, rhs: &Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr $vector) $out"#))]
    fn serialize_1(vector: Self) -> [u8; 2] {
        op_serialize_1(vector)
    }

    #[requires(bytes.len() == 2)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 2 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 $bytes (impl.f_repr $out)"#))]
    fn deserialize_1(bytes: &[u8]) -> Self {
        deserialize_1(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr $vector) $out"#))]
    fn serialize_4(vector: Self) -> [u8; 8] {
        op_serialize_4(vector)
    }

    #[requires(bytes.len() == 8)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 8 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 $bytes (impl.f_repr $out)"#))]
    fn deserialize_4(bytes: &[u8]) -> Self {
        deserialize_4(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5 (impl.f_repr $vector) $out"#))]
    fn serialize_5(vector: Self) -> [u8; 10] {
        serialize_5(vector)
    }

    #[requires(bytes.len() == 10)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 10 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 $bytes (impl.f_repr $out)"#))]
    fn deserialize_5(bytes: &[u8]) -> Self {
        deserialize_5(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr $vector) $out"#))]
    fn serialize_10(vector: Self) -> [u8; 20] {
        op_serialize_10(vector)
    }

    #[requires(bytes.len() == 20)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 20 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 $bytes (impl.f_repr $out)"#))]
    fn deserialize_10(bytes: &[u8]) -> Self {
        deserialize_10(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11 (impl.f_repr $vector) $out"#))]
    fn serialize_11(vector: Self) -> [u8; 22] {
        serialize_11(vector)
    }

    #[requires(bytes.len() == 22)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 22 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 $bytes (impl.f_repr $out)"#))]
    fn deserialize_11(bytes: &[u8]) -> Self {
        deserialize_11(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr $vector) $out"#))]
    fn serialize_12(vector: Self) -> [u8; 24] {
        op_serialize_12(vector)
    }

    #[requires(bytes.len() == 24)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 24 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 $bytes (impl.f_repr $out)"#))]
    fn deserialize_12(bytes: &[u8]) -> Self {
        op_deserialize_12(bytes)
    }

    #[requires(input.len() == 24 && out.len() == 16)]
    #[ensures(|result| (future(out).len() == 16 && result <= 16).to_prop() & (
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(out)[j] >= 0 && future(out)[j] <= 3328))))]
    fn rej_sample(input: &[u8], out: &mut [i16]) -> usize {
        op_rej_sample(input, out)
    }
}

// Portable-fallback rejection sampler (the Neon SIMD path is disabled — see the
// FIXME in the trait method).  Its `chunks(3)` indexing is not panic-free for
// arbitrary input lengths, and its functional post is out of scope at this
// layer; kept `lax` (was previously hidden by the whole-module admit).
#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
pub(crate) fn rej_sample(a: &[u8], result: &mut [i16]) -> usize {
    let mut sampled = 0;
    for bytes in a.chunks(3) {
        let b1 = bytes[0] as i16;
        let b2 = bytes[1] as i16;
        let b3 = bytes[2] as i16;

        let d1 = ((b2 & 0xF) << 8) | b1;
        let d2 = (b3 << 4) | (b2 >> 4);

        if d1 < FIELD_MODULUS && sampled < 16 {
            result[sampled] = d1;
            sampled += 1
        }
        if d2 < FIELD_MODULUS && sampled < 16 {
            result[sampled] = d2;
            sampled += 1
        }
    }
    sampled
}
