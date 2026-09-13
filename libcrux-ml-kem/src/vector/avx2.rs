use super::traits::Operations;
pub(crate) use libcrux_intrinsics::avx2::*;

#[cfg(hax)]
use super::traits::{spec, Repr};

mod arithmetic;
mod compress;
mod ntt;
mod sampling;
mod serialize;

#[cfg(hax)]
use hax_lib::prop::ToProp;

#[derive(Clone, Copy)]
#[hax_lib::fstar::before(interface, "noeq")]
#[hax_lib::fstar::after(interface,"let repr (x:t_SIMD256Vector) : t_Array i16 (sz 16) = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 x.f_elements")]
pub struct SIMD256Vector {
    elements: Vec256,
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"repr ${result} == Seq.create 16 (mk_i16 0)"#))]
// Bring the ml-kem storeu/loadu bit_vec SMTPats into scope (they live
// outside Avx2_extract so sha3's interface stays lean).
#[hax_lib::fstar::before(
    r#"open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views"#
)]
fn vec_zero() -> SIMD256Vector {
    // `mm256_setzero_si256 ()` is a fully ground core-models term, so its
    // view lemma deliberately carries no SMTPat (an SMTPat on a ground term
    // fires everywhere) — it has to be called explicitly, as Compress does.
    proof!(r#"Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_setzero_si256 ()"#);
    SIMD256Vector {
        elements: mm256_setzero_si256(),
    }
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"${result} == repr ${v}"#))]
fn vec_to_i16_array(v: SIMD256Vector) -> [i16; 16] {
    let mut output = [0i16; 16];
    // `output` has length 16; surface that fact, then call the store's view
    // bridge explicitly (over core-models the store is a modeled per-lane
    // spine, so the post does not fall out of the intrinsic's own ensures).
    proof!(
        r#"assert (Core_models.Slice.impl__len #i16 output == mk_usize 16);
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_storeu_si256_i16 (output <: t_Slice i16) ${v}.f_elements"#
    );
    mm256_storeu_si256_i16(&mut output, v.elements);

    output
}

#[inline(always)]
#[hax_lib::requires(array.len() == 16)]
#[hax_lib::ensures(|result| fstar!(r#"repr ${result} == ${array}"#))]
fn vec_from_i16_array(array: &[i16]) -> SIMD256Vector {
    // Dual of `vec_to_i16_array`: the load's view bridge is a proven companion
    // lemma over core-models and must be called explicitly.
    proof!(r#"Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_loadu_si256_i16 ${array}"#);
    SIMD256Vector {
        elements: mm256_loadu_si256_i16(array),
    }
}

#[inline(always)]
#[hax_lib::requires(array.len() >= 32)]
#[hax_lib::ensures(|result| fstar!(r#"
    Core_models.Slice.impl__len #u8 ${array} >=. mk_usize 32 ==>
    (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
     Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
       #(mk_usize 16) head
       (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements))
"#))]
pub(super) fn from_bytes(array: &[u8]) -> SIMD256Vector {
    // Name the loaded sub-slice so the load's bit lemma and the post's
    // `Seq.slice array 0 32` are the SAME term in the proof below.
    let head = &array[0..32];
    let result = SIMD256Vector {
        elements: mm256_loadu_si256_u8(head),
    };
    // Bridge the load to the `from_le_bytes_post_N` post, which unfolds to
    // `bit_vec_of_int_t_array head 8 == bit_vec_of_int_t_array (i16x16 view) 16`
    // -- an equality of two bit_vec FUNCTIONS, so it needs a pointwise intro.
    //
    // Per-index chain, for i < 256:
    //   bit_vec_of_int_t_array (vec256_as_i16x16 elts) 16 i
    //     == bv_bit elts ((i/16)*16 + i%16)        (the i16x16 readback lemma)
    //     == bv_bit elts i                         (euclidean division at 16)
    //     == get_bit head.[i/8] (i%8)              (the u8 load's bit lemma)
    //     == bit_vec_of_int_t_array head 8 i       (definitional, via `on`)
    //
    // NB over core-models `t_BitVec` is not a function, so bit access is
    // `bv_bit`, not a direct `${result}.f_elements i` function application.
    proof!(
        r#"
assert (Seq.length ${head} == 32);
let head_arr : t_Array u8 (mk_usize 32) = ${head} in
introduce forall (i: nat{i < 256}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array head_arr 8 i
    == Rust_primitives.BitVectors.bit_vec_of_int_t_array
         (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) 16 i
with (
  FStar.Math.Lemmas.euclidean_division_definition i 16;
  Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
    ${result}.f_elements 16 i;
  Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_mm256_loadu_si256_u8 ${head} i
);
BitVecEq.bit_vec_equal_intro
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array head_arr 8)
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array
     (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) 16)"#
    );
    result
}

// Carries the trait's `to_le_bytes_post_N` post directly so the trait
// wrapper in `impl Operations` is a one-line call.
//
// There is no panic_free/lax attribute here, so the post below is a
// genuine proof obligation, not an assumption; the proof block below is
// PROVEN and `Vector.Avx2` verifies end to end.
#[inline(always)]
#[hax_lib::requires(bytes.len() >= 32)]
#[hax_lib::ensures(|_| fstar!(r#"
    Core_models.Slice.impl__len #u8 (bytes_future <: t_Slice u8) ==
      Core_models.Slice.impl__len #u8 ${bytes} /\
    (Core_models.Slice.impl__len #u8 ${bytes} >=. mk_usize 32 ==>
     (let head : t_Slice u8 = Seq.slice bytes_future 0 32 in
      Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N
        #(mk_usize 16)
        (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${x}.f_elements) head))
"#))]
pub(super) fn to_bytes(x: SIMD256Vector, bytes: &mut [u8]) {
    // Proof-only snapshot: the store rebinds `bytes`, so without this the
    // store bridge has no way to name its pre-state operand.
    #[cfg(hax)]
    let bytes_pre: &[u8] = &*bytes;

    mm256_storeu_si256_u8(&mut bytes[0..32], x.elements);

    // Mirror of `from_bytes`, plus the sub-slice framing.  `to_le_bytes_post_N`
    // unfolds to an equality of two bit_vec FUNCTIONS, so it needs a pointwise
    // intro.  Per index i < 256:
    //   bit_vec_of_int_t_array (vec256_as_i16x16 elts) 16 i
    //     == bv_bit elts i                    (readback lemma + euclidean at 16)
    //     == bit_vec_of_int_t_array stored 8 i (the u8 store's bit lemma)
    //     == bit_vec_of_int_t_array head_arr 8 i
    // where `update_at_range`'s frame gives `Seq.slice bytes_future 0 32 == stored`.
    proof!(
        r#"
let range = ({ Core_models.Ops.Range.f_start = mk_usize 0;
              Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_Range usize) in
let src : t_Slice u8 = ${bytes_pre}.[ range ] in
assert (Seq.length src == 32);
let stored : t_Slice u8 = Libcrux_intrinsics.Avx2.mm256_storeu_si256_u8 src ${x}.f_elements in
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_storeu_si256_u8 src ${x}.f_elements;
Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range
    ${bytes_pre} range stored;
assert (Seq.length ${bytes} == Seq.length ${bytes_pre});
Seq.lemma_eq_intro (Seq.slice ${bytes} 0 32) stored;
let head_arr : t_Array u8 (mk_usize 32) = Seq.slice ${bytes} 0 32 in
introduce forall (i: nat{i < 256}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array
      (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${x}.f_elements) 16 i
    == Rust_primitives.BitVectors.bit_vec_of_int_t_array head_arr 8 i
with (
  FStar.Math.Lemmas.euclidean_division_definition i 16;
  Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
    ${x}.f_elements 16 i
);
BitVecEq.bit_vec_equal_intro
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array
     (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${x}.f_elements) 16)
  (Rust_primitives.BitVectors.bit_vec_of_int_t_array head_arr 8)"#
    );
}

#[cfg(hax)]
impl crate::vector::traits::Repr for SIMD256Vector {
    fn repr(&self) -> [i16; 16] {
        vec_to_i16_array(self.clone())
    }
}

#[cfg(any(eurydice, not(hax)))]
impl crate::vector::traits::Repr for SIMD256Vector {}

// =====================================================================
// `op_*` wrappers — same pattern as `src/vector/portable.rs`.
//
// Each `op_*` carries the *exact* trait pre/post for its
// `impl Operations for SIMD256Vector` counterpart, so the impl method
// body collapses to a one-line `op_<name>(args)` call (the trait
// subtyping check is then `P ==> P`, trivial).  Each `op_*` verifies in
// its own SMT scope; `impl_1`'s VC is uniformly trivial.
//
// Methods omitted from this section (no `op_*` needed):
//   ZERO, from_i16_array, to_i16_array, from_bytes, to_bytes,
//   add, sub, multiply_by_constant
// — for these the underlying `arithmetic::*` primitive's pre/post
// already matches the trait's exactly, so the impl method body is
// already a one-line call (mirrors portable.rs).
//
// `op_*` for compress/decompress/NTT-layer/ntt_multiply carry
// `panic_free` because the strengthened C4f trait posts (FE-form for
// NTT-layer + ntt_multiply, hacspec FE-equation form for
// compress/decompress) are not yet discharged from the AVX2 body —
// that is the C4′ AVX2 mirror task.  `panic_free` skips the body
// post-condition obligation while still checking panic-freedom and
// the pre-condition of the underlying primitive call.
//
// **Removal plan**: drop `panic_free` and prove each body when the
// corresponding portable proof's helper lemmas (in
// `Hacspec_ml_kem.Commute.Chunk.fst`) close, plus the AVX2-specific
// per-lane lemmas needed to bridge `Vec256` ↔ array form.  Tracked
// in `proofs/commutativity-handoff.md` progress tracker row C4′.
// =====================================================================

// SIMD256Vector→SIMD256Vector wrappers for add/sub/multiply_by_constant.
// The underlying `arithmetic::*` primitives operate on the inner Vec256
// type; without these wrappers, the trait impl methods do an inline
// `{ elements: arithmetic::add(lhs.elements, rhs.elements) }` record
// reconstruction at every method, inflating impl_3's combined-query
// check past rlimit 80.  Hoisting the wrap into op_* keeps each impl
// method a thin direct call.
#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::add_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::add_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) (impl.f_repr ${result})"#))]
fn op_add(lhs: SIMD256Vector, rhs: &SIMD256Vector) -> SIMD256Vector {
    SIMD256Vector {
        elements: arithmetic::add(lhs.elements, rhs.elements),
    }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::sub_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::sub_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) (impl.f_repr ${result})"#))]
fn op_sub(lhs: SIMD256Vector, rhs: &SIMD256Vector) -> SIMD256Vector {
    SIMD256Vector {
        elements: arithmetic::sub(lhs.elements, rhs.elements),
    }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::multiply_by_constant_pre} (impl.f_repr ${vec}) c"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::multiply_by_constant_post} (impl.f_repr ${vec}) c (impl.f_repr ${result})"#))]
fn op_multiply_by_constant(vec: SIMD256Vector, c: i16) -> SIMD256Vector {
    SIMD256Vector {
        elements: arithmetic::multiply_by_constant(vec.elements, c),
    }
}

#[inline(always)]
#[hax_lib::requires(spec::cond_subtract_3329_pre(&vector.repr()))]
#[hax_lib::ensures(|out| spec::cond_subtract_3329_post(&vector.repr(), &out.repr()))]
fn op_cond_subtract_3329(vector: SIMD256Vector) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    let result = SIMD256Vector {
        elements: arithmetic::cond_subtract_3329(vector.elements),
    };
    proof!(
        r#"Seq.lemma_eq_intro (repr ${result})
                  (Spec.Utils.map_array
                     (fun (x:i16) -> if x >=. (mk_i16 3329) then x -! (mk_i16 3329) else x)
                     (repr ${vector}))"#
    );
    // Fold `v y % 3329 == v x % 3329` (provided by the underlying primitive)
    // into the opaque `mod_q_eq` form expected by the trait post.
    proof!(
        r#"
        let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${vector}) i)))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${vector}) i))
        in
        Classical.forall_intro aux
        "#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(spec::barrett_reduce_pre(&vector.repr()))]
#[hax_lib::ensures(|out| spec::barrett_reduce_post(&vector.repr(), &out.repr()))]
fn op_barrett_reduce(vector: SIMD256Vector) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    let result = SIMD256Vector {
        elements: arithmetic::barrett_reduce(vector.elements),
    };
    proof!(
        r#"
        let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${vector}) i)))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${vector}) i))
        in
        Classical.forall_intro aux
        "#
    );
    // Fold per-lane mod_q_eq facts into the opaque `barrett_reduce_lane_post`
    // form expected by the `forall16` post.
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post)
                    (Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post)"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(spec::montgomery_multiply_by_constant_pre(&vector.repr(), constant))]
#[hax_lib::ensures(|out| spec::montgomery_multiply_by_constant_post(&vector.repr(), constant, &out.repr()))]
fn op_montgomery_multiply_by_constant(vector: SIMD256Vector, constant: i16) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    let result = SIMD256Vector {
        elements: arithmetic::montgomery_multiply_by_constant(vector.elements, constant),
    };
    proof!(
        r#"
        let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${vector}) i) * v ${constant} * 169))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${vector}) i) * v ${constant} * 169)
        in
        Classical.forall_intro aux
        "#
    );
    // Fold per-lane mod_q_eq facts into the opaque
    // `montgomery_multiply_lane_post` form expected by the `forall16` post.
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post)
                    (Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post)"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(spec::to_unsigned_representative_pre(&a.repr()))]
#[hax_lib::ensures(|out| spec::to_unsigned_representative_post(&a.repr(), &out.repr()))]
fn op_to_unsigned_representative(a: SIMD256Vector) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    let result = SIMD256Vector {
        elements: arithmetic::to_unsigned_representative(a.elements),
    };
    proof!(
        r#"
        let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${a}) i)))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${a}) i))
        in
        Classical.forall_intro aux
        "#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"${spec::compress_1_pre} (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::compress_1_post} (impl.f_repr ${vector}) (impl.f_repr ${out})"#))]
fn op_compress_1(vector: SIMD256Vector) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                    (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post"#
    );
    let result_elements = compress::compress_message_coefficient(vector.elements);
    let result = SIMD256Vector {
        elements: result_elements,
    };
    proof!(
        r#"let aux (j: nat{j < 16}) :
              Lemma (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                       (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                                     ${result}.f_elements) j) ==
                     Hacspec_ml_kem.Compress.compress_d
                       (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                          (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                                        ${vector}.f_elements) j))
                       (mk_usize 1)) =
             Hacspec_ml_kem.Commute.Chunk.lemma_compress_message_coefficient_fe_commute
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)
           in
           aux 0;  aux 1;  aux 2;  aux 3;
           aux 4;  aux 5;  aux 6;  aux 7;
           aux 8;  aux 9;  aux 10; aux 11;
           aux 12; aux 13; aux 14; aux 15"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::compress_pre} (impl.f_repr ${vector}) $COEFFICIENT_BITS"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::compress_post} (impl.f_repr ${vector}) $COEFFICIENT_BITS (impl.f_repr ${out})"#))]
fn op_compress<const COEFFICIENT_BITS: i32>(vector: SIMD256Vector) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
             (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array (mk_i16 0)
                 (mk_i16 3328)
                 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements));
           assert (forall (i: nat).
                 {:pattern Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) i}
                 i < 16 ==>
                 v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) i) >= 0 /\
                 v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) i) < 3329)"#
    );
    let result = SIMD256Vector {
        elements: compress::compress_ciphertext_coefficient::<COEFFICIENT_BITS>(vector.elements),
    };
    proof!(
        r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro (mk_i16 0)
             (mk_i16 (pow2 (v $COEFFICIENT_BITS) - 1))
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements);
           let aux (j: nat{j < 16})
               : Lemma
               (Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post (mk_usize (v $COEFFICIENT_BITS))
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)) =
             reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post)
               (Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post (mk_usize (v $COEFFICIENT_BITS))
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j));
             Hacspec_ml_kem.Commute.Chunk.lemma_compress_d_barrett_eq
               (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j))
               (v $COEFFICIENT_BITS);
             Hacspec_ml_kem.Commute.Chunk.lemma_compress_ciphertext_coefficient_fe_commute
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)
               (mk_usize (v $COEFFICIENT_BITS))
           in
           Classical.forall_intro aux"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::decompress_1_pre} (impl.f_repr ${a})"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::decompress_1_post} (impl.f_repr ${a}) (impl.f_repr ${out})"#))]
fn op_decompress_1(a: SIMD256Vector) -> SIMD256Vector {
    proof!(
        r#"assert_norm (pow2 1 - 1 == 1);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
             (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array (mk_i16 0)
                 (mk_i16 1)
                 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements));
           assert (forall (i: nat).
                 {:pattern Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements) i}
                 i < 16 ==>
                 v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements) i) >= 0 /\
                 v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements) i) <= 1)"#
    );
    let result = SIMD256Vector {
        elements: compress::decompress_1(a.elements),
    };
    proof!(
        r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro (mk_i16 0) (mk_i16 3328)
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements);
           let aux (j: nat{j < 16})
               : Lemma
               (Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements) j)
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)) =
             reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post)
               (Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements) j)
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j));
             Hacspec_ml_kem.Commute.Chunk.lemma_decompress_1_fe_commute_int
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${a}.f_elements) j)
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)
           in
           Classical.forall_intro aux"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::decompress_ciphertext_coefficient_pre} (impl.f_repr ${vector}) $COEFFICIENT_BITS"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::decompress_ciphertext_coefficient_post} (impl.f_repr ${vector}) $COEFFICIENT_BITS (impl.f_repr ${out})"#))]
fn op_decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    vector: SIMD256Vector,
) -> SIMD256Vector {
    proof!(
        r#"assert_norm (pow2 11 == 2048);
           FStar.Math.Lemmas.pow2_le_compat 11 (v $COEFFICIENT_BITS);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
             (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
           assert (forall (i: nat). i < 16 ==>
                 0 <= v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) i) /\
                 v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) i) <
                 pow2 (v $COEFFICIENT_BITS))"#
    );
    let result = SIMD256Vector {
        elements: compress::decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(vector.elements),
    };
    proof!(
        r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro (mk_i16 0) (mk_i16 3328)
             (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements);
           let aux (j: nat{j < 16})
               : Lemma
               (Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post (mk_usize (v $COEFFICIENT_BITS))
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)) =
             reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post)
               (Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post (mk_usize (v $COEFFICIENT_BITS))
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
                   (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j));
             Hacspec_ml_kem.Commute.Chunk.lemma_decompress_ciphertext_coefficient_fe_commute
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements) j)
               (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${result}.f_elements) j)
               (mk_usize (v $COEFFICIENT_BITS))
           in
           Classical.forall_intro aux"#
    );
    result
}

// NTT-layer bridges: each admitted lemma asserts that the AVX2
// primitive `ntt::{,inv_}ntt_layer_N_step` produces a result whose
// `vec256_as_i16x16` view satisfies the trait's strengthened
// `{,inv_}ntt_layer_N_step_post` (FE-form butterfly equations +
// `is_i16b_array_opaque` bound).  Removal plan: prove these by
// strengthening the primitive posts in
// `Libcrux_ml_kem.Vector.Avx2.Ntt.fsti` and re-discharging via the
// per-lane Vec256 ↔ array bridges that live in the AVX2
// `arithmetic.rs` lemma library.  `op_inv_ntt_layer_1_step` body
// in `Vector.Avx2.Ntt.fst` currently has `--admit_smt_queries true`,
// which must be addressed before the corresponding bridge proof can
// land.  `op_ntt_multiply` keeps `panic_free` (blocked on C4e
// Layer-0.5 admits — same on portable).
#[hax_lib::fstar::before(
    r#"
// `op_ntt_layer_1_step` needs no separate bridge lemma: the strengthened post of
// `Libcrux_ml_kem.Vector.Avx2.Ntt.ntt_layer_1_step` (`ntt_layer_1_butterfly_post`
// + bound `8*3328`) plus the four `lemma_ntt_layer_1_step_branch_{0..3}`
// (Commute.Chunk) discharge the trait post directly.  Wrapper does this inline.

// `op_ntt_layer_2_step` needs no separate bridge lemma: the strengthened post of
// `Libcrux_ml_kem.Vector.Avx2.Ntt.ntt_layer_2_step` (`ntt_layer_2_butterfly_post`
// + bound `7*3328`) plus 8 `lemma_butterfly_pair_commute` applications discharge
// the trait post directly.  Wrapper does this inline.

// `op_ntt_layer_3_step` needs no separate bridge lemma: the strengthened
// post of `Libcrux_ml_kem.Vector.Avx2.Ntt.ntt_layer_3_step` (per-lane
// butterfly residue + bound) plus 8 `lemma_butterfly_pair_commute`
// applications (one per pair (i, i+8)) discharge the trait post
// directly.  The wrapper does this inline; no bridge needed.

// `op_inv_ntt_layer_1_step` needs no separate bridge lemma: the strengthened
// post of `Libcrux_ml_kem.Vector.Avx2.Ntt.inv_ntt_layer_1_step`
// (`inv_ntt_layer_1_butterfly_post` + bound `3328`) plus the four
// `lemma_inv_ntt_layer_1_step_branch_{0..3}` (Commute.Chunk) discharge the
// trait post directly.  The wrapper does this inline; no bridge needed.

// `op_inv_ntt_layer_2_step` needs no separate bridge lemma: the strengthened
// post of `Libcrux_ml_kem.Vector.Avx2.Ntt.inv_ntt_layer_2_step`
// (`inv_ntt_layer_2_butterfly_post` + bound `2*3328`) plus 8
// `lemma_inv_butterfly_pair_commute` applications discharge the trait post
// directly.  The wrapper does this inline; no bridge needed.

// `op_inv_ntt_layer_3_step` needs no separate bridge lemma: the strengthened
// post of `Libcrux_ml_kem.Vector.Avx2.Ntt.inv_ntt_layer_3_step` (per-lane
// inv-butterfly residue + bound `4*3328`) plus 8
// `lemma_inv_butterfly_pair_commute` applications discharge the trait post
// directly.  The wrapper does this inline; no bridge needed.
"#
)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_ntt_layer_1_step(
    vector: SIMD256Vector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328))"#
    );
    let elements = ntt::ntt_layer_1_step(vector.elements, zeta0, zeta1, zeta2, zeta3);
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (8*3328));
           let vec = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           reveal_opaque (`%Spec.Utils.ntt_layer_1_butterfly_post)
                    (Spec.Utils.ntt_layer_1_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_0 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_1 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_2 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_3 vec out zeta0 zeta1 zeta2 zeta3"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_ntt_layer_2_step(vector: SIMD256Vector, zeta0: i16, zeta1: i16) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post"#
    );
    let elements = ntt::ntt_layer_2_step(vector.elements, zeta0, zeta1);
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328));
           let vec = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           reveal_opaque (`%Spec.Utils.ntt_layer_2_butterfly_post)
                    (Spec.Utils.ntt_layer_2_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 0 4;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 1 5;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 2 6;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 3 7;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 8 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 9 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 10 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 11 15;
           let p_layer_2 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let z = (if b < 2 then zeta0 else zeta1) in
                let base : nat = if b < 2 then 0 else 8 in
                let off : nat = if b = 0 || b = 2 then 0 else 2 in
                let i1 : nat = base + off in
                let j1 : nat = i1 + 4 in
                let i2 : nat = i1 + 1 in
                let j2 : nat = j1 + 1 in
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2)))) in
           assert (p_layer_2 0);
           assert (p_layer_2 1);
           assert (p_layer_2 2);
           assert (p_layer_2 3);
           assert (Spec.Utils.forall4 p_layer_2)"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_ntt_layer_3_step(vector: SIMD256Vector, zeta: i16) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post"#
    );
    let elements = ntt::ntt_layer_3_step(vector.elements, zeta);
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328));
           let vec = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 0 8;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 1 9;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 2 10;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 3 11;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 4 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 5 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 6 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 7 15;
           let p_layer_3 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let i1 : nat = 2 * b in
                let j1 : nat = 2 * b + 8 in
                let i2 : nat = 2 * b + 1 in
                let j2 : nat = 2 * b + 9 in
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2)))) in
           assert (p_layer_3 0);
           assert (p_layer_3 1);
           assert (p_layer_3 2);
           assert (p_layer_3 3);
           assert (Spec.Utils.forall4 p_layer_3)"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_1_step(
    vector: SIMD256Vector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328))"#
    );
    let elements = ntt::inv_ntt_layer_1_step(vector.elements, zeta0, zeta1, zeta2, zeta3);
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           let vec = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           reveal_opaque (`%Spec.Utils.inv_ntt_layer_1_butterfly_post)
                    (Spec.Utils.inv_ntt_layer_1_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_0 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_1 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_2 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_3 vec out zeta0 zeta1 zeta2 zeta3"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_2_step(vector: SIMD256Vector, zeta0: i16, zeta1: i16) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post"#
    );
    let elements = ntt::inv_ntt_layer_2_step(vector.elements, zeta0, zeta1);
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328));
           let vec = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           reveal_opaque (`%Spec.Utils.inv_ntt_layer_2_butterfly_post)
                    (Spec.Utils.inv_ntt_layer_2_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 0 4;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 1 5;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 2 6;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 3 7;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 8 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 9 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 10 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 11 15;
           let p_inv_2 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let z = (if b < 2 then zeta0 else zeta1) in
                let base : nat = if b < 2 then 0 else 8 in
                let off : nat = if b = 0 || b = 2 then 0 else 2 in
                let i1 : nat = base + off in
                let j1 : nat = i1 + 4 in
                let i2 : nat = i1 + 1 in
                let j2 : nat = j1 + 1 in
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1)) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2)) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2)))) in
           assert (p_inv_2 0);
           assert (p_inv_2 1);
           assert (p_inv_2 2);
           assert (p_inv_2 3);
           assert (Spec.Utils.forall4 p_inv_2)"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_3_step(vector: SIMD256Vector, zeta: i16) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post"#
    );
    let elements = ntt::inv_ntt_layer_3_step(vector.elements, zeta);
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328));
           let vec = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 0 8;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 1 9;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 2 10;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 3 11;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 4 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 5 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 6 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 7 15;
           let p_inv_layer_3 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let i1 : nat = 2 * b in
                let j1 : nat = 2 * b + 8 in
                let i2 : nat = 2 * b + 1 in
                let j2 : nat = 2 * b + 9 in
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1)) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2)) /\
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2)))) in
           assert (p_inv_layer_3 0);
           assert (p_inv_layer_3 1);
           assert (p_inv_layer_3 2);
           assert (p_inv_layer_3 3);
           assert (Spec.Utils.forall4 p_inv_layer_3)"#
    );
    SIMD256Vector { elements }
}

// `op_ntt_multiply` needs no separate bridge lemma: the strengthened post of
// `Libcrux_ml_kem.Vector.Avx2.Ntt.ntt_multiply` (`ntt_multiply_butterfly_post`
// + bound `3328`) plus the four `lemma_ntt_multiply_branch_{0..3}`
// (Commute.Chunk) discharge the trait post directly.  Wrapper does this
// inline; no bridge needed.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 0 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_multiply_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_multiply_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_ntt_multiply(
    lhs: &SIMD256Vector,
    rhs: &SIMD256Vector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> SIMD256Vector {
    proof!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 4096);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    let elements = ntt::ntt_multiply(lhs.elements, rhs.elements, zeta0, zeta1, zeta2, zeta3);
    proof!(
        r#"let l = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${lhs}.f_elements in
           let r = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${rhs}.f_elements in
           let out = Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${elements} in
           reveal_opaque (`%Spec.Utils.ntt_multiply_butterfly_post)
                         (Spec.Utils.ntt_multiply_butterfly_post l r out zeta0 zeta1 zeta2 zeta3);
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_0 l r out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_1 l r out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_2 l r out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_multiply_branch_3 l r out zeta0 zeta1 zeta2 zeta3"#
    );
    SIMD256Vector { elements }
}

// `op_serialize_*` / `op_deserialize_*`: bridge from the underlying
// `Avx2.Serialize.*` primitives' BitVec post (per-bit equation
// `bit_vec_of_int_t_array r 8 i == vector ((i/N)*16 + i%N)`) up to
// the trait's array-form `serialize_post_N` /
// `deserialize_post_N` (which is `BitVecEq.int_t_array_bitwise_eq`
// at depth N on the i16-side, depth 8 on the u8-side).  Each bridge
// lemma is named `op_{serialize,deserialize}_N_{pre,post}_bridge`
// and is fully proven (not admitted): the per-bit equality is
// discharged via the `vec256_as_i16x16` bit-decomposition axiom
// (`bit_vec_of_int_t_array_vec256_as_i16x16_lemma`) and lifted to
// the array-form post by `BitVecEq.bit_vec_equal_intro`.  The
// wrapper invokes the pre-bridge before the primitive call and the
// post-bridge after, so the `op_*` wrapper itself verifies against
// the trait spec with no admit.

// Bridge lemmas: connect the AVX2 primitives' BitVec posts to the
// trait's array-form posts.  Discharged via the `vec256_as_i16x16`
// bit-decomposition axiom in `Libcrux_intrinsics.Avx2_extract`.
// The lemma_vec256_lane_bounded + op_*_bridge family (serialize/deserialize
// byte-layout theory) now lives in the hand-written companion
// Libcrux_ml_kem.Vector.Avx2_theory.
#[hax_lib::fstar::before(r#"open Libcrux_ml_kem.Vector.Avx2_theory"#)]
#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr ${vector}) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr ${vector}) ${out}"#))]
fn op_serialize_1(vector: SIMD256Vector) -> [u8; 2] {
    proof!(r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_1_pre_bridge ${vector}.f_elements"#);
    let result = serialize::serialize_1(vector.elements);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_1_post_bridge ${vector}.f_elements ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 2)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 2 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 $bytes (impl.f_repr ${out})"#))]
fn op_deserialize_1(bytes: &[u8]) -> SIMD256Vector {
    let elements = serialize::deserialize_1(bytes);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_deserialize_1_post_bridge ${bytes} ${elements}"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr ${vector}) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr ${vector}) ${out}"#))]
fn op_serialize_4(vector: SIMD256Vector) -> [u8; 8] {
    proof!(r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_4_pre_bridge ${vector}.f_elements"#);
    let result = serialize::serialize_4(vector.elements);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_4_post_bridge ${vector}.f_elements ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 8)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 8 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 $bytes (impl.f_repr ${out})"#))]
fn op_deserialize_4(bytes: &[u8]) -> SIMD256Vector {
    let elements = serialize::deserialize_4(bytes);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_deserialize_4_post_bridge ${bytes} ${elements}"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr ${vector}) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr ${vector}) ${out}"#))]
fn op_serialize_10(vector: SIMD256Vector) -> [u8; 20] {
    proof!(r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_10_pre_bridge ${vector}.f_elements"#);
    let result = serialize::serialize_10(vector.elements);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_10_post_bridge ${vector}.f_elements ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 20)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 20 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 $bytes (impl.f_repr ${out})"#))]
fn op_deserialize_10(bytes: &[u8]) -> SIMD256Vector {
    let elements = serialize::deserialize_10(bytes);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_deserialize_10_post_bridge ${bytes} ${elements}"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr ${vector}) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr ${vector}) ${out}"#))]
fn op_serialize_12(vector: SIMD256Vector) -> [u8; 24] {
    proof!(r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_12_pre_bridge ${vector}.f_elements"#);
    let result = serialize::serialize_12(vector.elements);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_12_post_bridge ${vector}.f_elements ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 24)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 24 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 $bytes (impl.f_repr ${out})"#))]
fn op_deserialize_12(bytes: &[u8]) -> SIMD256Vector {
    let elements = serialize::deserialize_12(bytes);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_deserialize_12_post_bridge ${bytes} ${elements}"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr ${vector}) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11 (impl.f_repr ${vector}) ${out}"#))]
fn op_serialize_11(vector: SIMD256Vector) -> [u8; 22] {
    proof!(r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_11_pre_bridge ${vector}.f_elements"#);
    let result = serialize::serialize_11(vector.elements);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_11_post_bridge ${vector}.f_elements ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 22)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 22 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 $bytes (impl.f_repr ${out})"#))]
fn op_deserialize_11(bytes: &[u8]) -> SIMD256Vector {
    let elements = serialize::deserialize_11(bytes);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_deserialize_11_post_bridge ${bytes} ${elements}"#
    );
    SIMD256Vector { elements }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr ${vector}) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5 (impl.f_repr ${vector}) ${out}"#))]
fn op_serialize_5(vector: SIMD256Vector) -> [u8; 10] {
    proof!(r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_5_pre_bridge ${vector}.f_elements"#);
    let result = serialize::serialize_5(vector.elements);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_serialize_5_post_bridge ${vector}.f_elements ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 10)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 10 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 $bytes (impl.f_repr ${out})"#))]
fn op_deserialize_5(bytes: &[u8]) -> SIMD256Vector {
    let elements = serialize::deserialize_5(bytes);
    proof!(
        r#"Libcrux_ml_kem.Vector.Avx2_theory.op_deserialize_5_post_bridge ${bytes} ${elements}"#
    );
    SIMD256Vector { elements }
}

// =====================================================================
// `impl Operations for SIMD256Vector`
//
// Every method is a one-line call to its `op_*` (with matching trait
// pre/post — subtyping check is `P ==> P`) or to a free helper
// (`vec_zero`, `vec_to_i16_array`, `vec_from_i16_array`, `from_bytes`,
// `to_bytes`).  No proof code lives inside the impl.
// =====================================================================
// The combined t_Operations record-WF query is heavy and sensitive to solver
// state accumulated by the sibling lemmas.  Restart the solver and split it
// into per-field sub-queries so it verifies deterministically.
#[hax_lib::fstar::before(r#"#restart-solver
#push-options "--split_queries always --z3rlimit 300""#)]
#[hax_lib::fstar::after(r#"#pop-options"#)]
#[hax_lib::attributes]
impl Operations for SIMD256Vector {
    #[inline(always)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == Seq.create 16 (mk_i16 0)"#))]
    fn ZERO() -> Self {
        vec_zero()
    }

    #[requires(array.len() == 16)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == $array"#))]
    #[inline(always)]
    fn from_i16_array(array: &[i16]) -> Self {
        vec_from_i16_array(array)
    }

    #[ensures(|out| fstar!(r#"out == impl.f_repr $x"#))]
    #[inline(always)]
    fn to_i16_array(x: Self) -> [i16; 16] {
        vec_to_i16_array(x)
    }

    #[requires(array.len() >= 32)]
    #[ensures(|out| fstar!(r#"sz (Seq.length ${array}) >=. sz 32 ==>
                              (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
                               Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
                                 #(mk_usize 16) head (impl.f_repr ${out}))"#))]
    #[inline(always)]
    fn from_bytes(array: &[u8]) -> Self {
        from_bytes(array)
    }

    #[requires(bytes.len() >= 32)]
    #[inline(always)]
    #[ensures(|_| fstar!(r#"sz (Seq.length bytes_future) =. sz (Seq.length ${bytes}) /\
                            (sz (Seq.length bytes_future) >=. sz 32 ==>
                             (let head : t_Slice u8 = Seq.slice bytes_future 0 32 in
                              Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N
                                #(mk_usize 16) (impl.f_repr ${x}) head))"#))]
    fn to_bytes(x: Self, bytes: &mut [u8]) {
        to_bytes(x, bytes);
    }

    #[requires(fstar!(r#"${spec::add_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs})"#))]
    #[ensures(|result| fstar!(r#"${spec::add_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) (impl.f_repr ${result})"#))]
    #[inline(always)]
    fn add(lhs: Self, rhs: &Self) -> Self {
        op_add(lhs, rhs)
    }

    #[requires(fstar!(r#"${spec::sub_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs})"#))]
    #[ensures(|result| fstar!(r#"${spec::sub_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) (impl.f_repr ${result})"#))]
    #[inline(always)]
    fn sub(lhs: Self, rhs: &Self) -> Self {
        op_sub(lhs, rhs)
    }

    #[requires(fstar!(r#"${spec::multiply_by_constant_pre} (impl.f_repr ${vec}) c"#))]
    #[ensures(|result| fstar!(r#"${spec::multiply_by_constant_post} (impl.f_repr ${vec}) c (impl.f_repr ${result})"#))]
    #[inline(always)]
    fn multiply_by_constant(vec: Self, c: i16) -> Self {
        op_multiply_by_constant(vec, c)
    }

    #[requires(spec::cond_subtract_3329_pre(&vector.repr()))]
    #[ensures(|out| spec::cond_subtract_3329_post(&vector.repr(), &out.repr()))]
    #[inline(always)]
    fn cond_subtract_3329(vector: Self) -> Self {
        op_cond_subtract_3329(vector)
    }

    #[requires(spec::barrett_reduce_pre(&vector.repr()))]
    #[ensures(|result| spec::barrett_reduce_post(&vector.repr(), &result.repr()))]
    #[inline(always)]
    fn barrett_reduce(vector: Self) -> Self {
        op_barrett_reduce(vector)
    }

    #[inline(always)]
    #[requires(spec::montgomery_multiply_by_constant_pre(&vector.repr(), constant))]
    #[ensures(|result| spec::montgomery_multiply_by_constant_post(&vector.repr(), constant, &result.repr()))]
    fn montgomery_multiply_by_constant(vector: Self, constant: i16) -> Self {
        op_montgomery_multiply_by_constant(vector, constant)
    }

    #[requires(spec::to_unsigned_representative_pre(&a.repr()))]
    #[ensures(|result| spec::to_unsigned_representative_post(&a.repr(), &result.repr()))]
    fn to_unsigned_representative(a: Self) -> Self {
        op_to_unsigned_representative(a)
    }

    #[requires(fstar!(r#"${spec::compress_1_pre} (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"${spec::compress_1_post} (impl.f_repr $vector) (impl.f_repr $out)"#))]
    #[inline(always)]
    fn compress_1(vector: Self) -> Self {
        op_compress_1(vector)
    }

    #[requires(fstar!(r#"${spec::compress_pre} (impl.f_repr $vector) $COEFFICIENT_BITS"#))]
    #[ensures(|out| fstar!(r#"${spec::compress_post} (impl.f_repr $vector) $COEFFICIENT_BITS (impl.f_repr $out)"#))]
    #[inline(always)]
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
    #[inline(always)]
    fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(vector: Self) -> Self {
        op_decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(vector)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn ntt_layer_1_step(vector: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn ntt_layer_2_step(vector: Self, zeta0: i16, zeta1: i16) -> Self {
        op_ntt_layer_2_step(vector, zeta0, zeta1)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn ntt_layer_3_step(vector: Self, zeta: i16) -> Self {
        op_ntt_layer_3_step(vector, zeta)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn inv_ntt_layer_1_step(vector: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn inv_ntt_layer_2_step(vector: Self, zeta0: i16, zeta1: i16) -> Self {
        op_inv_ntt_layer_2_step(vector, zeta0, zeta1)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn inv_ntt_layer_3_step(vector: Self, zeta: i16) -> Self {
        op_inv_ntt_layer_3_step(vector, zeta)
    }

    #[requires(fstar!(r#"${spec::ntt_multiply_pre} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_multiply_post} (impl.f_repr ${lhs}) (impl.f_repr ${rhs}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
    #[inline(always)]
    fn ntt_multiply(
        lhs: &Self,
        rhs: &Self,
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
    ) -> Self {
        op_ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_1(vector: Self) -> [u8; 2] {
        op_serialize_1(vector)
    }

    #[requires(bytes.len() == 2)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 2 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_1(bytes: &[u8]) -> Self {
        op_deserialize_1(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_4(vector: Self) -> [u8; 8] {
        op_serialize_4(vector)
    }

    #[requires(bytes.len() == 8)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 8 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_4(bytes: &[u8]) -> Self {
        op_deserialize_4(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_5(vector: Self) -> [u8; 10] {
        op_serialize_5(vector)
    }

    #[requires(bytes.len() == 10)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 10 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_5(bytes: &[u8]) -> Self {
        op_deserialize_5(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_10(vector: Self) -> [u8; 20] {
        op_serialize_10(vector)
    }

    #[requires(bytes.len() == 20)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 20 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_10(bytes: &[u8]) -> Self {
        op_deserialize_10(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_11(vector: Self) -> [u8; 22] {
        op_serialize_11(vector)
    }

    #[requires(bytes.len() == 22)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 22 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_11(bytes: &[u8]) -> Self {
        op_deserialize_11(bytes)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $vector) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_12(vector: Self) -> [u8; 24] {
        op_serialize_12(vector)
    }

    #[requires(bytes.len() == 24)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 24 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_12(bytes: &[u8]) -> Self {
        op_deserialize_12(bytes)
    }

    #[inline(always)]
    #[requires(input.len() == 24 && out.len() == 16)]
    #[ensures(|result| (future(out).len() == 16 && result <= 16).to_prop() & (
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(out)[j] >= 0 && future(out)[j] <= 3328))))]
    fn rej_sample(input: &[u8], out: &mut [i16]) -> usize {
        sampling::rejection_sample(input, out)
    }
}
