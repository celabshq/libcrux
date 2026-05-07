use super::Operations;
use libcrux_secrets::*;

mod arithmetic;
mod compress;
mod ntt;
mod sampling;
mod serialize;
mod vector_type;

use arithmetic::*;
use compress::*;
use ntt::*;
use sampling::*;
use vector_type::*;

pub(crate) use vector_type::PortableVector;

#[cfg(hax)]
use super::traits::{spec, Repr};
#[cfg(hax)]
use hax_lib::prop::ToProp;

#[cfg(hax)]
impl crate::vector::traits::Repr for PortableVector {
    fn repr(&self) -> [i16; 16] {
        to_i16_array(self.clone())
    }
}

#[cfg(any(eurydice, not(hax)))]
impl crate::vector::traits::Repr for PortableVector {}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $a) ==>
                                 Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr $a) $out"#))]
fn serialize_1(a: PortableVector) -> [u8; 2] {
    hax_lib::fstar!(
        r#"assert (forall i. Rust_primitives.bounded (Seq.index ${a}.f_elements i) 1)"#
    );
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.serialize_1_lemma $a"#);
    serialize::serialize_1(a).declassify()
}

#[hax_lib::requires(a.len() == 2)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 2 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 $a (impl.f_repr $out)"#))]
fn deserialize_1(a: &[u8]) -> PortableVector {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_1_lemma $a"#);
    serialize::deserialize_1(a.classify_ref())
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr $a) $out"#))]
fn serialize_4(a: PortableVector) -> [u8; 8] {
    hax_lib::fstar!(
        r#"assert (forall i. Rust_primitives.bounded (Seq.index ${a}.f_elements i) 4)"#
    );
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.serialize_4_lemma $a"#);
    serialize::serialize_4(a).declassify()
}

#[hax_lib::requires(a.len() == 8)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 8 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 $a (impl.f_repr $out)"#))]
fn deserialize_4(a: &[u8]) -> PortableVector {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_4_lemma $a"#);
    serialize::deserialize_4(a.classify_ref())
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5 (impl.f_repr $a) $out"#))]
fn serialize_5(a: PortableVector) -> [u8; 10] {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.serialize_5_lemma $a"#);
    serialize::serialize_5(a).declassify()
}

#[hax_lib::requires(a.len() == 10)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 10 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 $a (impl.f_repr $out)"#))]
fn deserialize_5(a: &[u8]) -> PortableVector {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_5_lemma $a"#);
    serialize::deserialize_5(a.classify_ref())
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr $a) $out"#))]
fn serialize_10(a: PortableVector) -> [u8; 20] {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.serialize_10_lemma $a"#);
    serialize::serialize_10(a).declassify()
}

#[hax_lib::requires(a.len() == 20)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 20 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 $a (impl.f_repr $out)"#))]
fn deserialize_10(a: &[u8]) -> PortableVector {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_10_lemma $a"#);
    serialize::deserialize_10(a.classify_ref())
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11 (impl.f_repr $a) $out"#))]
fn serialize_11(a: PortableVector) -> [u8; 22] {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.serialize_11_lemma $a"#);
    serialize::serialize_11(a).declassify()
}

#[hax_lib::requires(a.len() == 22)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 22 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 $a (impl.f_repr $out)"#))]
fn deserialize_11(a: &[u8]) -> PortableVector {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_11_lemma $a"#);
    serialize::deserialize_11(a.classify_ref())
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr $a) $out"#))]
fn serialize_12(a: PortableVector) -> [u8; 24] {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.serialize_12_lemma $a"#);
    serialize::serialize_12(a).declassify()
}

#[hax_lib::requires(a.len() == 24)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 24 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 $a (impl.f_repr $out)"#))]
fn deserialize_12(a: &[u8]) -> PortableVector {
    hax_lib::fstar!(r#"Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_12_lemma $a"#);
    serialize::deserialize_12(a.classify_ref())
}

// =====================================================================
// `op_*` wrappers — Phase 1 of the impl-flattening refactor.
//
// Each `op_*` carries the *exact* trait pre/post for its
// `impl Operations for PortableVector` counterpart, so the impl method
// body collapses to a one-line `op_<name>(args)` call (the trait
// subtyping check is then `P ==> P`, trivial).  Each op_* verifies in
// its own SMT scope; all proof bridging that used to live in the impl
// method body lives here.
//
// Methods omitted from this section (no `op_*` needed):
//   ZERO, from_i16_array, to_i16_array, from_bytes, to_bytes,
//   add, sub, multiply_by_constant, serialize_*, deserialize_*, rej_sample
// — for these the underlying primitive's pre/post already matches the
// trait's exactly, so the impl method body is already a one-line call.
//
// Phase 2 (deferred): for some `op_*` we may be able to fold the
// bridging directly into the underlying primitive's annotations and
// drop the wrapper.  Tracked separately.
// =====================================================================

#[hax_lib::requires(spec::cond_subtract_3329_pre(&vec.repr()))]
#[hax_lib::ensures(|out| spec::cond_subtract_3329_post(&vec.repr(), &out.repr()))]
fn op_cond_subtract_3329(vec: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#
    );
    let result = cond_subtract_3329(vec);
    // Underlying primitive provides `v y % 3329 == v x % 3329`; fold to
    // the opaque `mod_q_eq` form expected by the trait post.
    hax_lib::fstar!(
        r#"
        let aux (i: nat) : Lemma (i < 16 ==>
            Hacspec_ml_kem.ModQ.mod_q_eq
              (v (Seq.index (impl.f_repr ${result}) i))
              (v (Seq.index (impl.f_repr ${vec}) i)))
          = if i < 16 then
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${vec}) i))
        in
        Classical.forall_intro aux
        "#
    );
    result
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 28296 (impl.f_repr ${vector})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328 (impl.f_repr ${result}) /\
            (forall i. Hacspec_ml_kem.ModQ.mod_q_eq
                         (v (Seq.index (impl.f_repr ${result}) i))
                         (v (Seq.index (impl.f_repr ${vector}) i)))"#))]
fn op_barrett_reduce(vector: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 28296)"#
    );
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    let result = barrett_reduce(vector);
    hax_lib::fstar!(
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

#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 $constant"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328 (impl.f_repr ${result}) /\
            (forall i. i < 16 ==>
              Hacspec_ml_kem.ModQ.mod_q_eq
                (v (Seq.index (impl.f_repr ${result}) i))
                (v (Seq.index (impl.f_repr ${vector}) i) * v ${constant} * 169))"#))]
fn op_montgomery_multiply_by_constant(vector: PortableVector, constant: i16) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    let result = montgomery_multiply_by_constant(vector, constant.classify());
    hax_lib::fstar!(
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
    result
}

#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328 (impl.f_repr $a)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
                            (let x = Seq.index (impl.f_repr ${a}) i in
                             let y = Seq.index (impl.f_repr ${result}) i in
                             (v y >= 0 /\ v y <= 3328 /\ Hacspec_ml_kem.ModQ.mod_q_eq (v y) (v x)))"#))]
fn op_to_unsigned_representative(a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    let result = to_unsigned_representative(a);
    hax_lib::fstar!(
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

// Compress / decompress.  Wrappers bridge the underlying primitive's
// integer-form post to the trait's FE-form `Spec.Utils.forall16`-shaped
// post via per-element fe_commute lemmas A5/A6/A7.
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"${spec::compress_1_pre} ${a}.f_elements"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::compress_1_post} ${a}.f_elements ${out}.f_elements"#))]
fn op_compress_1(a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                    (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post"#
    );
    let result = compress_1(a);
    // Single per-lane lemma + `Classical.forall_intro` to lift to the
    // full forall16 in one shot — replaces the 16 manual `aux 0; ...; aux 15`
    // invocations.  Z3 sees one universally-quantified obligation rather
    // than 16 sequential lemma instantiations.
    hax_lib::fstar!(
        r#"let aux (j: nat{j < 16}) :
              Lemma (Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post
                       (Seq.index ${a}.f_elements j)
                       (Seq.index ${result}.f_elements j)) =
             Hacspec_ml_kem.Commute.Chunk.lemma_compress_message_coefficient_fe_commute
               (Seq.index ${a}.f_elements j)
               (Seq.index ${result}.f_elements j)
           in
           Classical.forall_intro aux"#
    );
    result
}

#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"${spec::compress_pre} ${a}.f_elements $COEFFICIENT_BITS"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::compress_post} ${a}.f_elements $COEFFICIENT_BITS ${out}.f_elements"#))]
fn op_compress<const COEFFICIENT_BITS: i32>(a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                    (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post"#
    );
    let result = compress::<COEFFICIENT_BITS>(a);
    hax_lib::fstar!(
        r#"let aux (j: nat{j < 16}) :
              Lemma (Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post
                       (mk_usize (v $COEFFICIENT_BITS))
                       (Seq.index ${a}.f_elements j)
                       (Seq.index ${result}.f_elements j)) =
             Hacspec_ml_kem.Commute.Chunk.lemma_compress_ciphertext_coefficient_fe_commute
               (Seq.index ${a}.f_elements j)
               (Seq.index ${result}.f_elements j)
               (mk_usize (v $COEFFICIENT_BITS))
           in
           Classical.forall_intro aux"#
    );
    result
}

#[hax_lib::fstar::options("--z3rlimit 200 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::decompress_1_pre} ${a}.f_elements"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::decompress_1_post} ${a}.f_elements ${out}.f_elements"#))]
fn op_decompress_1(a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                    (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
           assert_norm (pow2 1 - 1 == 1);
           assert(forall (i: nat). {:pattern Seq.index a.f_elements i} i < 16 ==>
                     v (Seq.index a.f_elements i) >= 0 /\
                     v (Seq.index a.f_elements i) <= 1);
           assert(forall (i:nat). {:pattern Seq.index a.f_elements i} i < 16 ==>
                     v (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                           (Seq.index a.f_elements i)).f_val < 2);
           assert(forall (i:nat). {:pattern Seq.index a.f_elements i} i < 16 ==>
                     (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                           (Seq.index a.f_elements i)).f_val <. (mk_u16 1 <<! sz 1));
           assert(mk_usize 1 <. mk_usize 12)
          "#
    );
    let result = decompress_1(a);
    hax_lib::fstar!(
        r#"let aux (j: nat) :
              Lemma (requires j < 16)
                    (ensures (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                       (Seq.index ${result}.f_elements j) ==
                     Hacspec_ml_kem.Compress.decompress_d
                       (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                          (Seq.index ${a}.f_elements j))
                       (mk_usize 1))) =
             Hacspec_ml_kem.Commute.Chunk.lemma_decompress_1_fe_commute_int
               (Seq.index ${a}.f_elements j)
               (Seq.index ${result}.f_elements j)
           in
           aux 0;  aux 1;  aux 2;  aux 3;
           aux 4;  aux 5;  aux 6;  aux 7;
           aux 8;  aux 9;  aux 10; aux 11;
           aux 12; aux 13; aux 14; aux 15;
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post"#
    );
    result
}

#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"${spec::decompress_ciphertext_coefficient_pre} ${a}.f_elements $COEFFICIENT_BITS"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::decompress_ciphertext_coefficient_post} ${a}.f_elements $COEFFICIENT_BITS ${out}.f_elements"#))]
fn op_decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
                    (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post;
           assert (forall (i:nat). i < 16 ==>
                     0 <= v (Seq.index ${a}.f_elements i) /\
                     v (Seq.index ${a}.f_elements i) < pow2 (v $COEFFICIENT_BITS))"#
    );
    let result = decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(a);
    // Bridge: primitive's post uses `pow2 (D+1)` denominator; lemma's precondition
    // uses `pow2 D * 2`.  Same value, but Z3 doesn't auto-unfold `pow2 (n+1)`.
    hax_lib::fstar!(
        r#"FStar.Math.Lemmas.pow2_plus (v $COEFFICIENT_BITS) 1;
           assert (pow2 (v $COEFFICIENT_BITS + 1) == pow2 (v $COEFFICIENT_BITS) * 2)"#
    );
    hax_lib::fstar!(
        r#"let aux (j: nat{j < 16}) :
              Lemma (Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post
                       (mk_usize (v $COEFFICIENT_BITS))
                       (Seq.index ${a}.f_elements j)
                       (Seq.index ${result}.f_elements j)) =
             Hacspec_ml_kem.Commute.Chunk.lemma_decompress_ciphertext_coefficient_fe_commute
               (Seq.index ${a}.f_elements j)
               (Seq.index ${result}.f_elements j)
               (mk_usize (v $COEFFICIENT_BITS))
           in
           Classical.forall_intro aux"#
    );
    // Strengthened post (2026-05-02): expose the i16 result bound
    // `bounded_i16_array (mk_i16 0) (mk_i16 3328) result.f_elements`
    // (= [0, FIELD_MODULUS - 1]).  The inner `decompress_ciphertext_coefficient`'s
    // ensures already gives `forall i. 0 <= v result.f_elements[i] < FIELD_MODULUS`;
    // we just intro that into the opaque `bounded_i16_array` form.
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro
             (mk_i16 0) (mk_i16 3328)
             ${result}.f_elements"#
    );
    result
}

// NTT-layer wrappers.  Each bridges from the primitive's
// `Spec.Utils.is_i16b_array N` (non-opaque) + `*_butterfly_post`
// residue post to the trait's `is_i16b_array_opaque N` + `forall4
// (FE-algebra equality)` post.  Bridge = 8 `lemma_butterfly_pair_commute`
// calls + per-group `assert (p_layer_X N)` + final
// `assert (forall4 p_layer_X)`.
//
// `op_ntt_layer_1_step`: admit-with-comment, Phase 6 agent A 2026-04-27.
//   What was tried: same recipe that closes layers 2 / 3 (drop the
//   `--admit_smt_queries true` and replace with
//   `--z3rlimit N --split_queries always`) at N ∈ {200, 400, 800}.
//   Outcome: at rlimit 800 with split_queries, F* completed 60 sub-queries
//   in ~16 ms each, then a single sub-query (one of the 4 per-branch
//   `p_layer_1 b` asserts, b ∈ {0..3}) ran for >10 min without resolving
//   before manual cancel.  Earlier rlimit 400 run failed cleanly with
//   Error 19 "Assertion failed" at the same block in 4 min.
//   Hypothesis: the per-branch `p_layer_1` body uses a 4-way conditional
//   `if b=0 then zeta0 else if b=1 then zeta1 else if b=2 then zeta2 else
//   zeta3` to pick the right zeta, which Z3 case-splits on every
//   instantiation.  Compounded with the 16 underlying lemma facts, the
//   per-branch goal exceeds Z3's reasoning budget at rlimit 800.
//   Layers 2 / 3 verify because layer 3 has 0 zeta dispatching (single
//   zeta) and layer 2 has only a 2-way conditional (`if b<2 then zeta0
//   else zeta1`).
//   Next step (USER): refactor by replacing the 4-way `if/else` ladder
//   with 4 separate per-zeta let-rebinds, e.g. assert each
//   `p_layer_1_zeta{0..3}` first, then introduce `p_layer_1` as a
//   case-split lemma combining them.  Or factor each branch into a
//   dedicated `lemma_op_ntt_layer_1_step_branch_b` lemma in
//   `Hacspec_ml_kem.Commute.Chunk.fst` that takes `b` and the matching
//   `zeta_b` explicitly, then close the wrapper via 4 such calls
//   followed by `Spec.Utils.forall4` intro.
#[hax_lib::fstar::options("--admit_smt_queries true")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_1_step_pre} ${a}.f_elements zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} ${a}.f_elements zeta0 zeta1 zeta2 zeta3 ${out}.f_elements"#))]
fn op_ntt_layer_1_step(a: PortableVector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_1_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_1_step_branch_post"#
    );
    let out = ntt_layer_1_step(a, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (8*3328))"#
    );
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.Utils.ntt_layer_1_butterfly_post)
                      (Spec.Utils.ntt_layer_1_butterfly_post ${a}.f_elements);
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 0 2;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 1 3;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 4 6;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 5 7;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta2 8 10;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta2 9 11;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta3 12 14;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta3 13 15;
        let p_layer_1 : (b: nat{b < 4}) -> Type0 =
          fun (b: nat{b < 4}) ->
            (let z = (if b = 0 then zeta0
                      else if b = 1 then zeta1
                      else if b = 2 then zeta2
                      else zeta3) in
             let i1 : nat = 4 * b in
             let j1 : nat = 4 * b + 2 in
             let i2 : nat = 4 * b + 1 in
             let j2 : nat = 4 * b + 3 in
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2)))) in
        assert (p_layer_1 0);
        assert (p_layer_1 1);
        assert (p_layer_1 2);
        assert (p_layer_1 3);
        assert (Spec.Utils.forall4 p_layer_1)
        "#
    );
    out
}

#[hax_lib::fstar::options("--z3rlimit 800 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_2_step_pre} ${a}.f_elements zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} ${a}.f_elements zeta0 zeta1 ${out}.f_elements"#))]
fn op_ntt_layer_2_step(a: PortableVector, zeta0: i16, zeta1: i16) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post"#
    );
    let out = ntt_layer_2_step(a, zeta0, zeta1);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328))"#
    );
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.Utils.ntt_layer_2_butterfly_post)
                      (Spec.Utils.ntt_layer_2_butterfly_post ${a}.f_elements);
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 0 4;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 1 5;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 2 6;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 3 7;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 8 12;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 9 13;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 10 14;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 11 15;
        let p_layer_2 : (b: nat{b < 4}) -> Type0 =
          fun (b: nat{b < 4}) ->
            (let z = (if b < 2 then zeta0 else zeta1) in
             let base : nat = if b < 2 then 0 else 8 in
             let off  : nat = if b = 0 || b = 2 then 0 else 2 in
             let i1 : nat = base + off in
             let j1 : nat = i1 + 4 in
             let i2 : nat = i1 + 1 in
             let j2 : nat = j1 + 1 in
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2)))) in
        assert (p_layer_2 0);
        assert (p_layer_2 1);
        assert (p_layer_2 2);
        assert (p_layer_2 3);
        assert (Spec.Utils.forall4 p_layer_2)
        "#
    );
    out
}

#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_3_step_pre} ${a}.f_elements zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} ${a}.f_elements zeta ${out}.f_elements"#))]
fn op_ntt_layer_3_step(a: PortableVector, zeta: i16) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post"#
    );
    let out = ntt_layer_3_step(a, zeta);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328))"#
    );
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.Utils.ntt_layer_3_butterfly_post)
                      (Spec.Utils.ntt_layer_3_butterfly_post ${a}.f_elements);
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 0 8;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 1 9;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 2 10;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 3 11;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 4 12;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 5 13;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 6 14;
        Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 7 15;
        let p_layer_3 : (b: nat{b < 4}) -> Type0 =
          fun (b: nat{b < 4}) ->
            (let i1 : nat = 2 * b in
             let j1 : nat = 2 * b + 8 in
             let i2 : nat = 2 * b + 1 in
             let j2 : nat = 2 * b + 9 in
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2)))) in
        assert (p_layer_3 0);
        assert (p_layer_3 1);
        assert (p_layer_3 2);
        assert (p_layer_3 3);
        assert (Spec.Utils.forall4 p_layer_3)
        "#
    );
    out
}

// `op_inv_ntt_layer_1_step`: admit-with-comment, Phase 6 agent A
// 2026-04-27.  Same shape as `op_ntt_layer_1_step` — 4-way zeta dispatch
// in the per-branch predicate `p_inv_1`.  Not attempted directly (would
// exceed wall-clock budget); admitted by analogy.  Same hypothesis and
// next-step refactor recipe as `op_ntt_layer_1_step` above:
// replace the 4-way `if/else` ladder with 4 explicit per-zeta lemmas
// + `forall4` intro.
#[hax_lib::fstar::options("--admit_smt_queries true")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} ${a}.f_elements zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} ${a}.f_elements zeta0 zeta1 zeta2 zeta3 ${out}.f_elements"#))]
fn op_inv_ntt_layer_1_step(a: PortableVector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_1_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_1_step_branch_post"#
    );
    let out = inv_ntt_layer_1_step(a, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.Utils.inv_ntt_layer_1_butterfly_post)
                      (Spec.Utils.inv_ntt_layer_1_butterfly_post ${a}.f_elements);
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 0 2;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 1 3;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 4 6;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 5 7;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta2 8 10;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta2 9 11;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta3 12 14;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta3 13 15;
        let p_inv_1 : (b: nat{b < 4}) -> Type0 =
          fun (b: nat{b < 4}) ->
            (let z = (if b = 0 then zeta0
                      else if b = 1 then zeta1
                      else if b = 2 then zeta2
                      else zeta3) in
             let i1 : nat = 4 * b in
             let j1 : nat = 4 * b + 2 in
             let i2 : nat = 4 * b + 1 in
             let j2 : nat = 4 * b + 3 in
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1)) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2)) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2))
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2)))) in
        assert (p_inv_1 0);
        assert (p_inv_1 1);
        assert (p_inv_1 2);
        assert (p_inv_1 3);
        assert (Spec.Utils.forall4 p_inv_1)
        "#
    );
    out
}

#[hax_lib::fstar::options("--z3rlimit 800 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} ${a}.f_elements zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} ${a}.f_elements zeta0 zeta1 ${out}.f_elements"#))]
fn op_inv_ntt_layer_2_step(a: PortableVector, zeta0: i16, zeta1: i16) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post"#
    );
    let out = inv_ntt_layer_2_step(a, zeta0, zeta1);
    // Output bound from primitive is `3328` (Barrett still in portable);
    // widen to the trait's loosened `2*3328` post.  Once portable also
    // drops Barrett, this widen is a no-op.
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328))"#
    );
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.Utils.inv_ntt_layer_2_butterfly_post)
                      (Spec.Utils.inv_ntt_layer_2_butterfly_post ${a}.f_elements);
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 0 4;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 1 5;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 2 6;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta0 3 7;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 8 12;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 9 13;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 10 14;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta1 11 15;
        let p_inv_2 : (b: nat{b < 4}) -> Type0 =
          fun (b: nat{b < 4}) ->
            (let z = (if b < 2 then zeta0 else zeta1) in
             let base : nat = if b < 2 then 0 else 8 in
             let off  : nat = if b = 0 || b = 2 then 0 else 2 in
             let i1 : nat = base + off in
             let j1 : nat = i1 + 4 in
             let i2 : nat = i1 + 1 in
             let j2 : nat = j1 + 1 in
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1)) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2)) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe z)
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2))
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2)))) in
        assert (p_inv_2 0);
        assert (p_inv_2 1);
        assert (p_inv_2 2);
        assert (p_inv_2 3);
        assert (Spec.Utils.forall4 p_inv_2)
        "#
    );
    out
}

#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} ${a}.f_elements zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} ${a}.f_elements zeta ${out}.f_elements"#))]
fn op_inv_ntt_layer_3_step(a: PortableVector, zeta: i16) -> PortableVector {
    // Trait pre is now `is_i16b_array_opaque (2*3328)` (previously `3328`);
    // see comment on `inv_ntt_layer_3_step_post` in `src/vector/traits.rs`.
    // Reveal at the new bound so the underlying primitive's pre
    // (which we also loosen to `is_i16b_array (2*3328)`) discharges.
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post)
                    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post"#
    );
    let out = inv_ntt_layer_3_step(a, zeta);
    // Output bound from primitive is `3328` (Barrett still in portable);
    // widen to the trait's `4*3328` post.  Once portable also drops Barrett,
    // this widen is a no-op.
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328))"#
    );
    hax_lib::fstar!(
        r#"
        reveal_opaque (`%Spec.Utils.inv_ntt_layer_3_butterfly_post)
                      (Spec.Utils.inv_ntt_layer_3_butterfly_post ${a}.f_elements);
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 0 8;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 1 9;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 2 10;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 3 11;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 4 12;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 5 13;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 6 14;
        Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute
          ${a}.f_elements ${out}.f_elements zeta 7 15;
        let p_inv_3 : (b: nat{b < 4}) -> Type0 =
          fun (b: nat{b < 4}) ->
            (let i1 : nat = 2 * b in
             let j1 : nat = 2 * b + 8 in
             let i2 : nat = 2 * b + 1 in
             let j2 : nat = 2 * b + 9 in
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1)) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j1) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j1))
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i1))) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements i2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__add
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2))
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2)) /\
             Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${out}.f_elements j2) ==
               Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                 (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta)
                 (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements j2))
                   (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index ${a}.f_elements i2)))) in
        assert (p_inv_3 0);
        assert (p_inv_3 1);
        assert (p_inv_3 2);
        assert (p_inv_3 3);
        assert (Spec.Utils.forall4 p_inv_3)
        "#
    );
    out
}

// `op_ntt_multiply` — `panic_free` (C4f deferred from C4e admits).
// The primitive's pre is `Spec.Utils.is_i16b_array 3328 ...`
// (non-opaque); the wrapper's pre is `is_i16b_array_opaque 3328 ...`.
// Reveal needed for the call's precondition.  The full forall4-FE
// post proof was drafted but Z3 did not converge at rlimit 400; left
// as `panic_free` until the Commute.Chunk Layer-0.5 lemmas' admits are
// closed.  **Removal plan**: drop `panic_free` and prove the body when
// `lemma_base_case_mult_{even,odd}_{mod_core,fe_commute}` close.
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"${spec::ntt_multiply_pre} ${lhs}.f_elements ${rhs}.f_elements zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_multiply_post} ${lhs}.f_elements ${rhs}.f_elements zeta0 zeta1 zeta2 zeta3 ${out}.f_elements"#))]
fn op_ntt_multiply(
    lhs: &PortableVector,
    rhs: &PortableVector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> PortableVector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                    (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2, zeta3)
}

// =====================================================================
// `impl Operations for PortableVector`
//
// After Phase 1 of the impl-flattening refactor, every method body is
// either (a) a one-line call to a free function with matching pre/post
// (so the impl-method VC is `P ==> P`), or (b) a one-line call to an
// underlying primitive whose pre/post already match the trait's
// (e.g. `add`, `sub`, `multiply_by_constant`).  No proof code lives
// inside the impl block.  Each `op_*` and underlying primitive
// verifies in its own SMT scope, with its own `#push-options` if
// needed; `impl_1`'s VC is now uniformly trivial.
// =====================================================================
#[hax_lib::attributes]
impl Operations for PortableVector {
    #[ensures(|out| fstar!(r#"impl.f_repr out == Seq.create 16 (mk_i16 0)"#))]
    fn ZERO() -> Self {
        zero()
    }

    #[requires(array.len() == 16)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == $array"#))]
    fn from_i16_array(array: &[i16]) -> Self {
        from_i16_array(array.classify_ref())
    }

    #[ensures(|out| fstar!(r#"out == impl.f_repr $x"#))]
    fn to_i16_array(x: Self) -> [i16; 16] {
        to_i16_array(x).declassify()
    }

    #[requires(array.len() >= 32)]
    #[ensures(|out| fstar!(r#"sz (Seq.length ${array}) >=. sz 32 ==>
                              (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
                               Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
                                 #(mk_usize 16) head (impl.f_repr ${out}))"#))]
    fn from_bytes(array: &[u8]) -> Self {
        from_bytes(array.classify_ref())
    }

    #[requires(bytes.len() >= 32)]
    #[ensures(|_| fstar!(r#"sz (Seq.length bytes_future) =. sz (Seq.length ${bytes}) /\
                            (sz (Seq.length bytes_future) >=. sz 32 ==>
                             (let head : t_Slice u8 = Seq.slice bytes_future 0 32 in
                              Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N
                                #(mk_usize 16) (impl.f_repr ${x}) head))"#))]
    fn to_bytes(x: Self, bytes: &mut [u8]) {
        #[cfg(not(hax))]
        to_bytes(x, classify_mut_slice(bytes));

        // hax does not support &mut returning functions like `classify_mut_slice`
        #[cfg(hax)]
        {
            to_bytes(x, bytes);
        }
    }

    #[requires(fstar!(r#"${spec::add_pre} ${lhs}.f_elements ${rhs}.f_elements"#))]
    #[ensures(|result| fstar!(r#"${spec::add_post} ${lhs}.f_elements ${rhs}.f_elements ${result}.f_elements"#))]
    fn add(lhs: Self, rhs: &Self) -> Self {
        add(lhs, rhs)
    }

    #[requires(fstar!(r#"${spec::sub_pre} ${lhs}.f_elements ${rhs}.f_elements"#))]
    #[ensures(|result| fstar!(r#"${spec::sub_post} ${lhs}.f_elements ${rhs}.f_elements ${result}.f_elements"#))]
    fn sub(lhs: Self, rhs: &Self) -> Self {
        sub(lhs, rhs)
    }

    #[requires(fstar!(r#"${spec::multiply_by_constant_pre} ${vec}.f_elements c"#))]
    #[ensures(|result| fstar!(r#"${spec::multiply_by_constant_post} ${vec}.f_elements c ${result}.f_elements"#))]
    fn multiply_by_constant(vec: Self, c: i16) -> Self {
        multiply_by_constant(vec, c)
    }

    #[requires(spec::cond_subtract_3329_pre(&vec.repr()))]
    #[ensures(|out| spec::cond_subtract_3329_post(&vec.repr(), &out.repr()))]
    fn cond_subtract_3329(vec: Self) -> Self {
        op_cond_subtract_3329(vec)
    }

    #[requires(spec::barrett_reduce_pre(&vector.repr()))]
    #[ensures(|result| spec::barrett_reduce_post(&vector.repr(), &result.repr()))]
    fn barrett_reduce(vector: Self) -> Self {
        op_barrett_reduce(vector)
    }

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

    #[requires(fstar!(r#"${spec::compress_1_pre} ${a}.f_elements"#))]
    #[ensures(|out| fstar!(r#"${spec::compress_1_post} ${a}.f_elements ${out}.f_elements"#))]
    fn compress_1(a: Self) -> Self {
        op_compress_1(a)
    }

    #[requires(fstar!(r#"${spec::compress_pre} ${a}.f_elements $COEFFICIENT_BITS"#))]
    #[ensures(|out| fstar!(r#"${spec::compress_post} ${a}.f_elements $COEFFICIENT_BITS ${out}.f_elements"#))]
    fn compress<const COEFFICIENT_BITS: i32>(a: Self) -> Self {
        op_compress::<COEFFICIENT_BITS>(a)
    }

    #[requires(fstar!(r#"${spec::decompress_1_pre} ${a}.f_elements"#))]
    #[ensures(|out| fstar!(r#"${spec::decompress_1_post} ${a}.f_elements ${out}.f_elements"#))]
    fn decompress_1(a: Self) -> Self {
        op_decompress_1(a)
    }

    #[requires(fstar!(r#"${spec::decompress_ciphertext_coefficient_pre} ${a}.f_elements $COEFFICIENT_BITS"#))]
    #[ensures(|out| fstar!(r#"${spec::decompress_ciphertext_coefficient_post} ${a}.f_elements $COEFFICIENT_BITS ${out}.f_elements"#))]
    fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(a: Self) -> Self {
        op_decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(a)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_1_step_pre} ${a}.f_elements zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} ${a}.f_elements zeta0 zeta1 zeta2 zeta3 ${out}.f_elements"#))]
    fn ntt_layer_1_step(a: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_ntt_layer_1_step(a, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_2_step_pre} ${a}.f_elements zeta0 zeta1"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} ${a}.f_elements zeta0 zeta1 ${out}.f_elements"#))]
    fn ntt_layer_2_step(a: Self, zeta0: i16, zeta1: i16) -> Self {
        op_ntt_layer_2_step(a, zeta0, zeta1)
    }

    #[requires(fstar!(r#"${spec::ntt_layer_3_step_pre} ${a}.f_elements zeta"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} ${a}.f_elements zeta ${out}.f_elements"#))]
    fn ntt_layer_3_step(a: Self, zeta: i16) -> Self {
        op_ntt_layer_3_step(a, zeta)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} ${a}.f_elements zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} ${a}.f_elements zeta0 zeta1 zeta2 zeta3 ${out}.f_elements"#))]
    fn inv_ntt_layer_1_step(a: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        op_inv_ntt_layer_1_step(a, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} ${a}.f_elements zeta0 zeta1"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} ${a}.f_elements zeta0 zeta1 ${out}.f_elements"#))]
    fn inv_ntt_layer_2_step(a: Self, zeta0: i16, zeta1: i16) -> Self {
        op_inv_ntt_layer_2_step(a, zeta0, zeta1)
    }

    #[requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} ${a}.f_elements zeta"#))]
    #[ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} ${a}.f_elements zeta ${out}.f_elements"#))]
    fn inv_ntt_layer_3_step(a: Self, zeta: i16) -> Self {
        op_inv_ntt_layer_3_step(a, zeta)
    }

    #[requires(fstar!(r#"${spec::ntt_multiply_pre} ${lhs}.f_elements ${rhs}.f_elements zeta0 zeta1 zeta2 zeta3"#))]
    #[ensures(|out| fstar!(r#"${spec::ntt_multiply_post} ${lhs}.f_elements ${rhs}.f_elements zeta0 zeta1 zeta2 zeta3 ${out}.f_elements"#))]
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

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr $a) $out"#))]
    fn serialize_1(a: Self) -> [u8; 2] {
        serialize_1(a)
    }

    #[requires(a.len() == 2)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 2 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 $a (impl.f_repr $out)"#))]
    fn deserialize_1(a: &[u8]) -> Self {
        deserialize_1(a)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr $a) $out"#))]
    fn serialize_4(a: Self) -> [u8; 8] {
        serialize_4(a)
    }

    #[requires(a.len() == 8)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 8 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 $a (impl.f_repr $out)"#))]
    fn deserialize_4(a: &[u8]) -> Self {
        deserialize_4(a)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5 (impl.f_repr $a) $out"#))]
    fn serialize_5(a: Self) -> [u8; 10] {
        serialize_5(a)
    }

    #[requires(a.len() == 10)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 10 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 $a (impl.f_repr $out)"#))]
    fn deserialize_5(a: &[u8]) -> Self {
        deserialize_5(a)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr $a) $out"#))]
    fn serialize_10(a: Self) -> [u8; 20] {
        serialize_10(a)
    }

    #[requires(a.len() == 20)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 20 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 $a (impl.f_repr $out)"#))]
    fn deserialize_10(a: &[u8]) -> Self {
        deserialize_10(a)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11 (impl.f_repr $a) $out"#))]
    fn serialize_11(a: Self) -> [u8; 22] {
        serialize_11(a)
    }

    #[requires(a.len() == 22)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 22 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 $a (impl.f_repr $out)"#))]
    fn deserialize_11(a: &[u8]) -> Self {
        deserialize_11(a)
    }

    #[requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $a)"#))]
    #[ensures(|out| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr $a) ==> Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr $a) $out"#))]
    fn serialize_12(a: Self) -> [u8; 24] {
        serialize_12(a)
    }

    #[requires(a.len() == 24)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $a) =. sz 24 ==> Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 $a (impl.f_repr $out)"#))]
    fn deserialize_12(a: &[u8]) -> Self {
        deserialize_12(a)
    }

    #[requires(a.len() == 24 && out.len() == 16)]
    #[ensures(|result| (future(out).len() == 16 && result <= 16).to_prop().and(
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(out)[j] >= 0 && future(out)[j] <= 3328))))]
    fn rej_sample(a: &[u8], out: &mut [i16]) -> usize {
        rej_sample(a, out)
    }
}
