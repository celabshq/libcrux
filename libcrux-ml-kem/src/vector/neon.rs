//! Vectors for libcrux using aarch64 (neon) intrinsics

#[cfg(hax)]
use super::traits::{spec, Repr};
use super::Operations;
#[cfg(hax)]
use hax_lib::prop::ToProp;

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
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328))"#);
    let result = ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (8*3328));
           let vec = Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${vector} in
           let out = Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result} in
           reveal_opaque (`%Spec.Utils.ntt_layer_1_butterfly_post) (Spec.Utils.ntt_layer_1_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_0 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_1 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_2 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_3 vec out zeta0 zeta1 zeta2 zeta3"#
    );
    result
}

#[hax_lib::fstar::before(r#"#push-options "--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always"

let lemma_neon_ntt_layer_2_post (vec out: t_Array i16 (mk_usize 16)) (zeta0 zeta1: i16)
    : Lemma
      (requires Spec.Utils.ntt_layer_2_butterfly_post vec out zeta0 zeta1)
      (ensures
        Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
            Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post b vec zeta0 zeta1 out)) =
  reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post)
    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post;
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
  let p_layer_2: b: nat{b < 4} -> Type0 =
    fun (b: nat{b < 4}) ->
      (let z = (if b < 2 then zeta0 else zeta1) in
        let base:nat = if b < 2 then 0 else 8 in
        let off:nat = if b = 0 || b = 2 then 0 else 2 in
        let i1:nat = base + off in
        let j1:nat = i1 + 4 in
        let i2:nat = i1 + 1 in
        let j2:nat = j1 + 1 in
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i1))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  z)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i1))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  z)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i2))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  z)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i2))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  z)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))))
  in
  assert (p_layer_2 0);
  assert (p_layer_2 1);
  assert (p_layer_2 2);
  assert (p_layer_2 3);
  assert (Spec.Utils.forall4 p_layer_2)

#pop-options

#push-options "--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always"

let lemma_neon_ntt_layer_3_post (vec out: t_Array i16 (mk_usize 16)) (zeta: i16)
    : Lemma
      (requires Spec.Utils.ntt_layer_3_butterfly_post vec out zeta)
      (ensures
        Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
            Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post b vec zeta out)) =
  reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post)
    Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post;
  reveal_opaque (`%Spec.Utils.ntt_layer_3_butterfly_post)
    (Spec.Utils.ntt_layer_3_butterfly_post vec);
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 0 8;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 1 9;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 2 10;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 3 11;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 4 12;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 5 13;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 6 14;
  Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 7 15;
  let p_layer_3: b: nat{b < 4} -> Type0 =
    fun (b: nat{b < 4}) ->
      (let i1:nat = 2 * b in
        let j1:nat = 2 * b + 8 in
        let i2:nat = 2 * b + 1 in
        let j2:nat = 2 * b + 9 in
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i1))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  zeta)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i1))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  zeta)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i2))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  zeta)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i2))
          (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  zeta)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2))))
  in
  assert (p_layer_3 0);
  assert (p_layer_3 1);
  assert (p_layer_3 2);
  assert (p_layer_3 3);
  assert (Spec.Utils.forall4 p_layer_3)

#pop-options

#push-options "--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always"

let lemma_neon_inv_ntt_layer_2_post (vec out: t_Array i16 (mk_usize 16)) (zeta0 zeta1: i16)
    : Lemma
      (requires Spec.Utils.inv_ntt_layer_2_butterfly_post vec out zeta0 zeta1)
      (ensures
        Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
            Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post b vec zeta0 zeta1 out)) =
  reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post)
    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post;
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
  let p_inv_2: b: nat{b < 4} -> Type0 =
    fun (b: nat{b < 4}) ->
      (let z = (if b < 2 then zeta0 else zeta1) in
        let base:nat = if b < 2 then 0 else 8 in
        let off:nat = if b = 0 || b = 2 then 0 else 2 in
        let i1:nat = base + off in
        let j1:nat = i1 + 4 in
        let i2:nat = i1 + 1 in
        let j2:nat = j1 + 1 in
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i1))
          (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1)) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              z)
          (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  (Seq.index vec j1))
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i2))
          (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2)) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              z)
          (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  (Seq.index vec j2))
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))))
  in
  assert (p_inv_2 0);
  assert (p_inv_2 1);
  assert (p_inv_2 2);
  assert (p_inv_2 3);
  assert (Spec.Utils.forall4 p_inv_2)

#pop-options

#push-options "--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always"

let lemma_neon_inv_ntt_layer_3_post (vec out: t_Array i16 (mk_usize 16)) (zeta: i16)
    : Lemma
      (requires Spec.Utils.inv_ntt_layer_3_butterfly_post vec out zeta)
      (ensures
        Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
            Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post b vec zeta out)) =
  reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post)
    Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post;
  reveal_opaque (`%Spec.Utils.inv_ntt_layer_3_butterfly_post)
    (Spec.Utils.inv_ntt_layer_3_butterfly_post vec);
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 0 8;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 1 9;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 2 10;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 3 11;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 4 12;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 5 13;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 6 14;
  Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 7 15;
  let p_inv_layer_3: b: nat{b < 4} -> Type0 =
    fun (b: nat{b < 4}) ->
      (let i1:nat = 2 * b in
        let j1:nat = 2 * b + 8 in
        let i2:nat = 2 * b + 1 in
        let j2:nat = 2 * b + 9 in
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i1))
          (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j1)) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j1) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              zeta)
          (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  (Seq.index vec j1))
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i1))) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out i2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index vec i2))
          (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec j2)) /\
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out j2) ==
        Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              zeta)
          (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  (Seq.index vec j2))
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec i2))))
  in
  assert (p_inv_layer_3 0);
  assert (p_inv_layer_3 1);
  assert (p_inv_layer_3 2);
  assert (p_inv_layer_3 3);
  assert (Spec.Utils.forall4 p_inv_layer_3)

#pop-options"#)]

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --fuel 0 --ifuel 1 --split_queries always --using_facts_from '* -Libcrux_ml_kem.Vector.Neon.Vector_type.lemma_repr_index'")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328))"#);
    let result = ntt_layer_2_step(vector, zeta0, zeta1);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328));
           lemma_neon_ntt_layer_2_post (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${vector}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result}) zeta0 zeta1"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --fuel 0 --ifuel 1 --split_queries always --using_facts_from '* -Libcrux_ml_kem.Vector.Neon.Vector_type.lemma_repr_index'")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5*3328))"#);
    let result = ntt_layer_3_step(vector, zeta);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328));
           lemma_neon_ntt_layer_3_post (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${vector}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result}) zeta"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328))"#);
    let result = inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           let vec = Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${vector} in
           let out = Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result} in
           reveal_opaque (`%Spec.Utils.inv_ntt_layer_1_butterfly_post) (Spec.Utils.inv_ntt_layer_1_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_0 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_1 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_2 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_3 vec out zeta0 zeta1 zeta2 zeta3"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --fuel 0 --ifuel 1 --split_queries always --using_facts_from '* -Libcrux_ml_kem.Vector.Neon.Vector_type.lemma_repr_index'")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (3328))"#);
    let result = inv_ntt_layer_2_step(vector, zeta0, zeta1);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328));
           lemma_neon_inv_ntt_layer_2_post (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${vector}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result}) zeta0 zeta1"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --fuel 0 --ifuel 1 --split_queries always --using_facts_from '* -Libcrux_ml_kem.Vector.Neon.Vector_type.lemma_repr_index'")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector {
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328))"#);
    let result = inv_ntt_layer_3_step(vector, zeta);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328));
           lemma_neon_inv_ntt_layer_3_post (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${vector}) (Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result}) zeta"#
    );
    result
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
#[hax_lib::fstar::options("--z3rlimit 50")]
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

// Rejection sampling delegates to the verified portable scalar sampler; the
// dedicated Neon SIMD path in neon/sampling.rs is currently disabled (see the
// FIXME in the trait method), so this re-uses portable's proof directly.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 50")]
#[hax_lib::requires(a.len() == 24 && result.len() == 16)]
#[hax_lib::ensures(|res| (future(result).len() == result.len() && res <= 16).to_prop().and(
    hax_lib::forall(|j: usize|
        hax_lib::implies(j < res,
            future(result)[j] >= 0 && future(result)[j] <= 3328))))]
pub(crate) fn rej_sample(a: &[u8], result: &mut [i16]) -> usize {
    crate::vector::portable::sampling::rej_sample(a, result)
}
