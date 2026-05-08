module Libcrux_ml_kem.Vector.Portable
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_kem.Vector.Portable.Vector_type in
  let open Libcrux_ml_kem.Vector.Traits in
  let open Libcrux_secrets.Int.Classify_public in
  let open Libcrux_secrets.Int.Public_integers in
  let open Libcrux_secrets.Traits in
  ()

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl: Libcrux_ml_kem.Vector.Traits.t_Repr
Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    f_repr_pre = (fun (self: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> true);
    f_repr_post
    =
    (fun
        (self: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array i16 (mk_usize 16))
        ->
        true);
    f_repr
    =
    fun (self: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
      Libcrux_ml_kem.Vector.Portable.Vector_type.to_i16_array (Core_models.Clone.f_clone #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
            #FStar.Tactics.Typeclasses.solve
            self
          <:
          Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
  }

let serialize_1_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit = assert (forall i. Rust_primitives.bounded (Seq.index a.f_elements i) 1) in
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.serialize_1_lemma a in
  Libcrux_secrets.Traits.f_declassify #(t_Array u8 (mk_usize 2))
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Vector.Portable.Serialize.serialize_1_ a <: t_Array u8 (mk_usize 2))

let deserialize_1_ (a: t_Slice u8) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_1_lemma a in
  Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_1_ (Libcrux_secrets.Traits.f_classify_ref #(t_Slice
          u8)
        #FStar.Tactics.Typeclasses.solve
        a
      <:
      t_Slice u8)

let serialize_4_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit = assert (forall i. Rust_primitives.bounded (Seq.index a.f_elements i) 4) in
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.serialize_4_lemma a in
  Libcrux_secrets.Traits.f_declassify #(t_Array u8 (mk_usize 8))
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Vector.Portable.Serialize.serialize_4_ a <: t_Array u8 (mk_usize 8))

let deserialize_4_ (a: t_Slice u8) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_4_lemma a in
  Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_4_ (Libcrux_secrets.Traits.f_classify_ref #(t_Slice
          u8)
        #FStar.Tactics.Typeclasses.solve
        a
      <:
      t_Slice u8)

let serialize_5_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.serialize_5_lemma a in
  Libcrux_secrets.Traits.f_declassify #(t_Array u8 (mk_usize 10))
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Vector.Portable.Serialize.serialize_5_ a <: t_Array u8 (mk_usize 10))

let deserialize_5_ (a: t_Slice u8) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_5_lemma a in
  Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_5_ (Libcrux_secrets.Traits.f_classify_ref #(t_Slice
          u8)
        #FStar.Tactics.Typeclasses.solve
        a
      <:
      t_Slice u8)

let serialize_10_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.serialize_10_lemma a in
  Libcrux_secrets.Traits.f_declassify #(t_Array u8 (mk_usize 20))
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Vector.Portable.Serialize.serialize_10_ a <: t_Array u8 (mk_usize 20))

let deserialize_10_ (a: t_Slice u8) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_10_lemma a in
  Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_10_ (Libcrux_secrets.Traits.f_classify_ref #(t_Slice
          u8)
        #FStar.Tactics.Typeclasses.solve
        a
      <:
      t_Slice u8)

let serialize_11_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.serialize_11_lemma a in
  Libcrux_secrets.Traits.f_declassify #(t_Array u8 (mk_usize 22))
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Vector.Portable.Serialize.serialize_11_ a <: t_Array u8 (mk_usize 22))

let deserialize_11_ (a: t_Slice u8) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_11_lemma a in
  Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_11_ (Libcrux_secrets.Traits.f_classify_ref #(t_Slice
          u8)
        #FStar.Tactics.Typeclasses.solve
        a
      <:
      t_Slice u8)

let serialize_12_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.serialize_12_lemma a in
  Libcrux_secrets.Traits.f_declassify #(t_Array u8 (mk_usize 24))
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Vector.Portable.Serialize.serialize_12_ a <: t_Array u8 (mk_usize 24))

let deserialize_12_ (a: t_Slice u8) =
  let _:Prims.unit = Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_12_lemma a in
  Libcrux_ml_kem.Vector.Portable.Serialize.deserialize_12_ (Libcrux_secrets.Traits.f_classify_ref #(t_Slice
          u8)
        #FStar.Tactics.Typeclasses.solve
        a
      <:
      t_Slice u8)

let op_cond_subtract_3329_ (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Arithmetic.cond_subtract_3329_ vec
  in
  let _:Prims.unit =
    let aux (i: nat)
        : Lemma
        (i < 16 ==>
          Hacspec_ml_kem.ModQ.mod_q_eq (v (Seq.index (impl.f_repr result) i))
            (v (Seq.index (impl.f_repr vec) i))) =
      if i < 16
      then
        Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro (v (Seq.index (impl.f_repr result) i))
          (v (Seq.index (impl.f_repr vec) i))
    in
    Classical.forall_intro aux
  in
  result

let op_barrett_reduce (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 28296)
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Arithmetic.barrett_reduce vector
  in
  let _:Prims.unit =
    let aux (i: nat)
        : Lemma
        (i < 16 ==>
          Hacspec_ml_kem.ModQ.mod_q_eq (v (Seq.index (impl.f_repr result) i))
            (v (Seq.index (impl.f_repr vector) i))) =
      if i < 16
      then
        Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro (v (Seq.index (impl.f_repr result) i))
          (v (Seq.index (impl.f_repr vector) i))
    in
    Classical.forall_intro aux
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post)
      (Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post)
  in
  result

let op_montgomery_multiply_by_constant
      (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (constant: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Arithmetic.montgomery_multiply_by_constant vector
      (Libcrux_secrets.Traits.f_classify #i16 #FStar.Tactics.Typeclasses.solve constant <: i16)
  in
  let _:Prims.unit =
    let aux (i: nat)
        : Lemma
        (i < 16 ==>
          Hacspec_ml_kem.ModQ.mod_q_eq (v (Seq.index (impl.f_repr result) i))
            (v (Seq.index (impl.f_repr vector) i) * v constant * 169)) =
      if i < 16
      then
        Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro (v (Seq.index (impl.f_repr result) i))
          (v (Seq.index (impl.f_repr vector) i) * v constant * 169)
    in
    Classical.forall_intro aux
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post)
      (Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post)
  in
  result

let op_to_unsigned_representative (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Arithmetic.to_unsigned_representative a
  in
  let _:Prims.unit =
    let aux (i: nat)
        : Lemma
        (i < 16 ==>
          Hacspec_ml_kem.ModQ.mod_q_eq (v (Seq.index (impl.f_repr result) i))
            (v (Seq.index (impl.f_repr a) i))) =
      if i < 16
      then
        Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro (v (Seq.index (impl.f_repr result) i))
          (v (Seq.index (impl.f_repr a) i))
    in
    Classical.forall_intro aux
  in
  result

#push-options "--z3rlimit 200"

let op_compress_1_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
      (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post)
      Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Compress.compress_1_ a
  in
  let _:Prims.unit =
    let aux (j: nat{j < 16})
        : Lemma
        (Libcrux_ml_kem.Vector.Traits.Spec.compress_1_lane_post (Seq.index a.f_elements j)
            (Seq.index result.f_elements j)) =
      Hacspec_ml_kem.Commute.Chunk.lemma_compress_message_coefficient_fe_commute (Seq.index a
              .f_elements
            j)
        (Seq.index result.f_elements j)
    in
    Classical.forall_intro aux
  in
  result

#pop-options

#push-options "--z3rlimit 300"

let op_compress
      (v_COEFFICIENT_BITS: i32)
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
      (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post)
      Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Compress.compress v_COEFFICIENT_BITS a
  in
  let _:Prims.unit =
    let aux (j: nat{j < 16})
        : Lemma
        (Libcrux_ml_kem.Vector.Traits.Spec.compress_d_lane_post (mk_usize (v v_COEFFICIENT_BITS))
            (Seq.index a.f_elements j)
            (Seq.index result.f_elements j)) =
      Hacspec_ml_kem.Commute.Chunk.lemma_compress_ciphertext_coefficient_fe_commute (Seq.index a
              .f_elements
            j)
        (Seq.index result.f_elements j)
        (mk_usize (v v_COEFFICIENT_BITS))
    in
    Classical.forall_intro aux
  in
  result

#pop-options

#push-options "--z3rlimit 200 --split_queries always"

let op_decompress_1_ (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
      (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
    assert_norm (pow2 1 - 1 == 1);
    assert (forall (i: nat). {:pattern Seq.index a.f_elements i}
          i < 16 ==> v (Seq.index a.f_elements i) >= 0 /\ v (Seq.index a.f_elements i) <= 1);
    assert (forall (i: nat). {:pattern Seq.index a.f_elements i}
          i < 16 ==>
          v (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index a.f_elements i)).f_val < 2);
    assert (forall (i: nat). {:pattern Seq.index a.f_elements i}
          i < 16 ==>
          (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index a.f_elements i)).f_val <.
          (mk_u16 1 <<! sz 1));
    assert (mk_usize 1 <. mk_usize 12)
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Compress.decompress_1_ a
  in
  let _:Prims.unit =
    let aux (j: nat)
        : Lemma (requires j < 16)
          (ensures
            (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe (Seq.index result.f_elements j) ==
              Hacspec_ml_kem.Compress.decompress_d (Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
                    (Seq.index a.f_elements j))
                (mk_usize 1))) =
      Hacspec_ml_kem.Commute.Chunk.lemma_decompress_1_fe_commute_int (Seq.index a.f_elements j)
        (Seq.index result.f_elements j)
    in
    aux 0;
    aux 1;
    aux 2;
    aux 3;
    aux 4;
    aux 5;
    aux 6;
    aux 7;
    aux 8;
    aux 9;
    aux 10;
    aux 11;
    aux 12;
    aux 13;
    aux 14;
    aux 15;
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post)
      Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_lane_post
  in
  result

#pop-options

#push-options "--z3rlimit 300"

let op_decompress_ciphertext_coefficient
      (v_COEFFICIENT_BITS: i32)
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array)
      (Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array);
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post)
      Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post;
    assert (forall (i: nat).
          i < 16 ==>
          0 <= v (Seq.index a.f_elements i) /\
          v (Seq.index a.f_elements i) < pow2 (v v_COEFFICIENT_BITS))
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Compress.decompress_ciphertext_coefficient v_COEFFICIENT_BITS a
  in
  let _:Prims.unit =
    FStar.Math.Lemmas.pow2_plus (v v_COEFFICIENT_BITS) 1;
    assert (pow2 (v v_COEFFICIENT_BITS + 1) == pow2 (v v_COEFFICIENT_BITS) * 2)
  in
  let _:Prims.unit =
    let aux (j: nat{j < 16})
        : Lemma
        (Libcrux_ml_kem.Vector.Traits.Spec.decompress_d_lane_post (mk_usize (v v_COEFFICIENT_BITS))
            (Seq.index a.f_elements j)
            (Seq.index result.f_elements j)) =
      Hacspec_ml_kem.Commute.Chunk.lemma_decompress_ciphertext_coefficient_fe_commute (Seq.index a
              .f_elements
            j)
        (Seq.index result.f_elements j)
        (mk_usize (v v_COEFFICIENT_BITS))
    in
    Classical.forall_intro aux
  in
  let _:Prims.unit =
    Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro (mk_i16 0)
      (mk_i16 3328)
      result.f_elements
  in
  result

#pop-options

#push-options "--admit_smt_queries true"

let op_ntt_layer_1_step
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta0 zeta1 zeta2 zeta3: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7 * 3328));
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_1_step_branch_post)
      Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_1_step_branch_post
  in
  let out:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.ntt_layer_1_step a zeta0 zeta1 zeta2 zeta3
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (8 * 3328))
  in
  let _:Prims.unit =
    reveal_opaque (`%Spec.Utils.ntt_layer_1_butterfly_post)
      (Spec.Utils.ntt_layer_1_butterfly_post a.f_elements);
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta0 0 2;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta0 1 3;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta1 4 6;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta1 5 7;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta2 8 10;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta2 9 11;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta3
      12
      14;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta3
      13
      15;
    let p_layer_1: b: nat{b < 4} -> Type0 =
      fun (b: nat{b < 4}) ->
        (let z =
            (if b = 0 then zeta0 else if b = 1 then zeta1 else if b = 2 then zeta2 else zeta3)
          in
          let i1:nat = 4 * b in
          let j1:nat = 4 * b + 2 in
          let i2:nat = 4 * b + 1 in
          let j2:nat = 4 * b + 3 in
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2)))
        )
    in
    assert (p_layer_1 0);
    assert (p_layer_1 1);
    assert (p_layer_1 2);
    assert (p_layer_1 3);
    assert (Spec.Utils.forall4 p_layer_1)
  in
  out

#pop-options

#push-options "--z3rlimit 800 --fuel 1 --ifuel 1 --split_queries always"

let op_ntt_layer_2_step
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta0 zeta1: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6 * 3328));
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post)
      Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post
  in
  let out:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.ntt_layer_2_step a zeta0 zeta1
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7 * 3328))
  in
  let _:Prims.unit =
    reveal_opaque (`%Spec.Utils.ntt_layer_2_butterfly_post)
      (Spec.Utils.ntt_layer_2_butterfly_post a.f_elements);
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta0 0 4;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta0 1 5;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta0 2 6;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta0 3 7;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta1 8 12;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta1 9 13;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      10
      14;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      11
      15;
    let p_layer_2: b: nat{b < 4} -> Type0 =
      fun (b: nat{b < 4}) ->
        (let z = (if b < 2 then zeta0 else zeta1) in
          let base:nat = if b < 2 then 0 else 8 in
          let off:nat = if b = 0 || b = 2 then 0 else 2 in
          let i1:nat = base + off in
          let j1:nat = i1 + 4 in
          let i2:nat = i1 + 1 in
          let j2:nat = j1 + 1 in
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    z)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2)))
        )
    in
    assert (p_layer_2 0);
    assert (p_layer_2 1);
    assert (p_layer_2 2);
    assert (p_layer_2 3);
    assert (Spec.Utils.forall4 p_layer_2)
  in
  out

#pop-options

#push-options "--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always"

let op_ntt_layer_3_step (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (zeta: i16) =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5 * 3328));
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post)
      Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post
  in
  let out:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.ntt_layer_3_step a zeta
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6 * 3328))
  in
  let _:Prims.unit =
    reveal_opaque (`%Spec.Utils.ntt_layer_3_butterfly_post)
      (Spec.Utils.ntt_layer_3_butterfly_post a.f_elements);
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 0 8;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 1 9;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 2 10;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 3 11;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 4 12;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 5 13;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 6 14;
    Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute a.f_elements out.f_elements zeta 7 15;
    let p_layer_3: b: nat{b < 4} -> Type0 =
      fun (b: nat{b < 4}) ->
        (let i1:nat = 2 * b in
          let j1:nat = 2 * b + 8 in
          let i2:nat = 2 * b + 1 in
          let j2:nat = 2 * b + 9 in
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    zeta)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    zeta)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    zeta)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    zeta)
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2)))
        )
    in
    assert (p_layer_3 0);
    assert (p_layer_3 1);
    assert (p_layer_3 2);
    assert (p_layer_3 3);
    assert (Spec.Utils.forall4 p_layer_3)
  in
  out

#pop-options

#push-options "--admit_smt_queries true"

let op_inv_ntt_layer_1_step
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta0 zeta1 zeta2 zeta3: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4 * 3328));
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_1_step_branch_post)
      Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_1_step_branch_post
  in
  let out:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.inv_ntt_layer_1_step a zeta0 zeta1 zeta2 zeta3
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)
  in
  let _:Prims.unit =
    reveal_opaque (`%Spec.Utils.inv_ntt_layer_1_butterfly_post)
      (Spec.Utils.inv_ntt_layer_1_butterfly_post a.f_elements);
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta0
      0
      2;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta0
      1
      3;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      4
      6;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      5
      7;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta2
      8
      10;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta2
      9
      11;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta3
      12
      14;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta3
      13
      15;
    let p_inv_1: b: nat{b < 4} -> Type0 =
      fun (b: nat{b < 4}) ->
        (let z =
            (if b = 0 then zeta0 else if b = 1 then zeta1 else if b = 2 then zeta2 else zeta3)
          in
          let i1:nat = 4 * b in
          let j1:nat = 4 * b + 2 in
          let i2:nat = 4 * b + 1 in
          let j2:nat = 4 * b + 3 in
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1)) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                z)
            (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    (Seq.index a.f_elements j1))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements i1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2)) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                z)
            (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    (Seq.index a.f_elements j2))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements i2)))
        )
    in
    assert (p_inv_1 0);
    assert (p_inv_1 1);
    assert (p_inv_1 2);
    assert (p_inv_1 3);
    assert (Spec.Utils.forall4 p_inv_1)
  in
  out

#pop-options

#push-options "--z3rlimit 800 --fuel 1 --ifuel 1 --split_queries always"

let op_inv_ntt_layer_2_step
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta0 zeta1: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post)
      Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post
  in
  let out:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.inv_ntt_layer_2_step a zeta0 zeta1
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2 * 3328))
  in
  let _:Prims.unit =
    reveal_opaque (`%Spec.Utils.inv_ntt_layer_2_butterfly_post)
      (Spec.Utils.inv_ntt_layer_2_butterfly_post a.f_elements);
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta0
      0
      4;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta0
      1
      5;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta0
      2
      6;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta0
      3
      7;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      8
      12;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      9
      13;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      10
      14;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta1
      11
      15;
    let p_inv_2: b: nat{b < 4} -> Type0 =
      fun (b: nat{b < 4}) ->
        (let z = (if b < 2 then zeta0 else zeta1) in
          let base:nat = if b < 2 then 0 else 8 in
          let off:nat = if b = 0 || b = 2 then 0 else 2 in
          let i1:nat = base + off in
          let j1:nat = i1 + 4 in
          let i2:nat = i1 + 1 in
          let j2:nat = j1 + 1 in
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1)) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                z)
            (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    (Seq.index a.f_elements j1))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements i1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2)) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                z)
            (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    (Seq.index a.f_elements j2))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements i2)))
        )
    in
    assert (p_inv_2 0);
    assert (p_inv_2 1);
    assert (p_inv_2 2);
    assert (p_inv_2 3);
    assert (Spec.Utils.forall4 p_inv_2)
  in
  out

#pop-options

#push-options "--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always"

let op_inv_ntt_layer_3_step
      (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2 * 3328));
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post)
      Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post
  in
  let out:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.inv_ntt_layer_3_step a zeta
  in
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4 * 3328))
  in
  let _:Prims.unit =
    reveal_opaque (`%Spec.Utils.inv_ntt_layer_3_butterfly_post)
      (Spec.Utils.inv_ntt_layer_3_butterfly_post a.f_elements);
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      0
      8;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      1
      9;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      2
      10;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      3
      11;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      4
      12;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      5
      13;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      6
      14;
    Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute a.f_elements
      out.f_elements
      zeta
      7
      15;
    let p_inv_3: b: nat{b < 4} -> Type0 =
      fun (b: nat{b < 4}) ->
        (let i1:nat = 2 * b in
          let j1:nat = 2 * b + 8 in
          let i2:nat = 2 * b + 1 in
          let j2:nat = 2 * b + 9 in
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i1))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j1)) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j1) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                zeta)
            (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    (Seq.index a.f_elements j1))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements i1))) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements i2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index a.f_elements i2))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements j2)) /\
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out.f_elements j2) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                zeta)
            (Hacspec_ml_kem.Parameters.impl_FieldElement__sub (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                    (Seq.index a.f_elements j2))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a.f_elements i2)))
        )
    in
    assert (p_inv_3 0);
    assert (p_inv_3 1);
    assert (p_inv_3 2);
    assert (p_inv_3 3);
    assert (Spec.Utils.forall4 p_inv_3)
  in
  out

#pop-options

let op_ntt_multiply
      (lhs rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
      (zeta0 zeta1 zeta2 zeta3: i16)
     =
  let _:Prims.unit =
    reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
      (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)
  in
  let result:Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
    Libcrux_ml_kem.Vector.Portable.Ntt.ntt_multiply lhs rhs zeta0 zeta1 zeta2 zeta3
  in
  let _:Prims.unit = admit () (* Panic freedom *) in
  result

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1: Libcrux_ml_kem.Vector.Traits.t_Operations
Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector =
  {
    _super_i0 = FStar.Tactics.Typeclasses.solve;
    _super_i1 = FStar.Tactics.Typeclasses.solve;
    _super_i2 = FStar.Tactics.Typeclasses.solve;
    f_ZERO_pre = (fun (_: Prims.unit) -> true);
    f_ZERO_post
    =
    (fun (_: Prims.unit) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        impl.f_repr out == Seq.create 16 (mk_i16 0));
    f_ZERO = (fun (_: Prims.unit) -> Libcrux_ml_kem.Vector.Portable.Vector_type.zero ());
    f_from_i16_array_pre
    =
    (fun (array: t_Slice i16) -> (Core_models.Slice.impl__len #i16 array <: usize) =. mk_usize 16);
    f_from_i16_array_post
    =
    (fun (array: t_Slice i16) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        impl.f_repr out == array);
    f_from_i16_array
    =
    (fun (array: t_Slice i16) ->
        Libcrux_ml_kem.Vector.Portable.Vector_type.from_i16_array (Libcrux_secrets.Traits.f_classify_ref
              #(t_Slice i16)
              #FStar.Tactics.Typeclasses.solve
              array
            <:
            t_Slice i16));
    f_to_i16_array_pre
    =
    (fun (x: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> true);
    f_to_i16_array_post
    =
    (fun
        (x: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array i16 (mk_usize 16))
        ->
        out == impl.f_repr x);
    f_to_i16_array
    =
    (fun (x: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_secrets.Traits.f_declassify #(t_Array i16 (mk_usize 16))
          #FStar.Tactics.Typeclasses.solve
          (Libcrux_ml_kem.Vector.Portable.Vector_type.to_i16_array x <: t_Array i16 (mk_usize 16)));
    f_from_bytes_pre
    =
    (fun (array: t_Slice u8) -> (Core_models.Slice.impl__len #u8 array <: usize) >=. mk_usize 32);
    f_from_bytes_post
    =
    (fun (array: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length array) >=. sz 32 ==>
        (let head:t_Slice u8 = Seq.slice array 0 32 in
          Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N #(mk_usize 16)
            head
            (impl.f_repr out)));
    f_from_bytes
    =
    (fun (array: t_Slice u8) ->
        Libcrux_ml_kem.Vector.Portable.Vector_type.from_bytes (Libcrux_secrets.Traits.f_classify_ref
              #(t_Slice u8)
              #FStar.Tactics.Typeclasses.solve
              array
            <:
            t_Slice u8));
    f_to_bytes_pre
    =
    (fun (x: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (bytes: t_Slice u8) ->
        (Core_models.Slice.impl__len #u8 bytes <: usize) >=. mk_usize 32);
    f_to_bytes_post
    =
    (fun
        (x: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (bytes: t_Slice u8)
        (bytes_future: t_Slice u8)
        ->
        sz (Seq.length bytes_future) =. sz (Seq.length bytes) /\
        (sz (Seq.length bytes_future) >=. sz 32 ==>
          (let head:t_Slice u8 = Seq.slice bytes_future 0 32 in
            Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N #(mk_usize 16) (impl.f_repr x) head
          )));
    f_to_bytes
    =
    (fun (x: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (bytes: t_Slice u8) ->
        let bytes:t_Slice u8 = Libcrux_ml_kem.Vector.Portable.Vector_type.to_bytes x bytes in
        let _:Prims.unit = () in
        bytes);
    f_add_pre
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.add_pre lhs.f_elements rhs.f_elements);
    f_add_post
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (result: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.add_post lhs.f_elements rhs.f_elements result.f_elements);
    f_add
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Portable.Arithmetic.add lhs rhs);
    f_sub_pre
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.sub_pre lhs.f_elements rhs.f_elements);
    f_sub_post
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (result: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.sub_post lhs.f_elements rhs.f_elements result.f_elements);
    f_sub
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Portable.Arithmetic.sub lhs rhs);
    f_multiply_by_constant_pre
    =
    (fun (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (c: i16) ->
        Libcrux_ml_kem.Vector.Traits.Spec.multiply_by_constant_pre vec.f_elements c);
    f_multiply_by_constant_post
    =
    (fun
        (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (c: i16)
        (result: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.multiply_by_constant_post vec.f_elements
          c
          result.f_elements);
    f_multiply_by_constant
    =
    (fun (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (c: i16) ->
        Libcrux_ml_kem.Vector.Portable.Arithmetic.multiply_by_constant vec c);
    f_cond_subtract_3329__pre
    =
    (fun (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.cond_subtract_3329_pre (Libcrux_ml_kem.Vector.Traits.f_repr
              #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              vec
            <:
            t_Array i16 (mk_usize 16)));
    f_cond_subtract_3329__post
    =
    (fun
        (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.cond_subtract_3329_post (Libcrux_ml_kem.Vector.Traits.f_repr
              #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              vec
            <:
            t_Array i16 (mk_usize 16))
          (Libcrux_ml_kem.Vector.Traits.f_repr #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              out
            <:
            t_Array i16 (mk_usize 16)));
    f_cond_subtract_3329_
    =
    (fun (vec: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        op_cond_subtract_3329_ vec);
    f_barrett_reduce_pre
    =
    (fun (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_pre (Libcrux_ml_kem.Vector.Traits.f_repr #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              vector
            <:
            t_Array i16 (mk_usize 16)));
    f_barrett_reduce_post
    =
    (fun
        (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (result: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_post (Libcrux_ml_kem.Vector.Traits.f_repr #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              vector
            <:
            t_Array i16 (mk_usize 16))
          (Libcrux_ml_kem.Vector.Traits.f_repr #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              result
            <:
            t_Array i16 (mk_usize 16)));
    f_barrett_reduce
    =
    (fun (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        op_barrett_reduce vector);
    f_montgomery_multiply_by_constant_pre
    =
    (fun (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (constant: i16) ->
        Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_by_constant_pre (Libcrux_ml_kem.Vector.Traits.f_repr
              #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              vector
            <:
            t_Array i16 (mk_usize 16))
          constant);
    f_montgomery_multiply_by_constant_post
    =
    (fun
        (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (constant: i16)
        (result: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_by_constant_post (Libcrux_ml_kem.Vector.Traits.f_repr
              #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              vector
            <:
            t_Array i16 (mk_usize 16))
          constant
          (Libcrux_ml_kem.Vector.Traits.f_repr #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              result
            <:
            t_Array i16 (mk_usize 16)));
    f_montgomery_multiply_by_constant
    =
    (fun (vector: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (constant: i16) ->
        op_montgomery_multiply_by_constant vector constant);
    f_to_unsigned_representative_pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.to_unsigned_representative_pre (Libcrux_ml_kem.Vector.Traits.f_repr
              #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              a
            <:
            t_Array i16 (mk_usize 16)));
    f_to_unsigned_representative_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (result: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.to_unsigned_representative_post (Libcrux_ml_kem.Vector.Traits.f_repr
              #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              a
            <:
            t_Array i16 (mk_usize 16))
          (Libcrux_ml_kem.Vector.Traits.f_repr #Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector
              #FStar.Tactics.Typeclasses.solve
              result
            <:
            t_Array i16 (mk_usize 16)));
    f_to_unsigned_representative
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        op_to_unsigned_representative a);
    f_compress_1__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.compress_1_pre a.f_elements);
    f_compress_1__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.compress_1_post a.f_elements out.f_elements);
    f_compress_1_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> op_compress_1_ a);
    f_compress_pre
    =
    (fun
        (v_COEFFICIENT_BITS: i32)
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.compress_pre a.f_elements v_COEFFICIENT_BITS);
    f_compress_post
    =
    (fun
        (v_COEFFICIENT_BITS: i32)
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.compress_post a.f_elements
          v_COEFFICIENT_BITS
          out.f_elements);
    f_compress
    =
    (fun
        (v_COEFFICIENT_BITS: i32)
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        op_compress v_COEFFICIENT_BITS a);
    f_decompress_1__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_pre a.f_elements);
    f_decompress_1__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.decompress_1_post a.f_elements out.f_elements);
    f_decompress_1_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> op_decompress_1_ a);
    f_decompress_ciphertext_coefficient_pre
    =
    (fun
        (v_COEFFICIENT_BITS: i32)
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.decompress_ciphertext_coefficient_pre a.f_elements
          v_COEFFICIENT_BITS);
    f_decompress_ciphertext_coefficient_post
    =
    (fun
        (v_COEFFICIENT_BITS: i32)
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.decompress_ciphertext_coefficient_post a.f_elements
          v_COEFFICIENT_BITS
          out.f_elements);
    f_decompress_ciphertext_coefficient
    =
    (fun
        (v_COEFFICIENT_BITS: i32)
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        op_decompress_ciphertext_coefficient v_COEFFICIENT_BITS a);
    f_ntt_layer_1_step_pre
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_1_step_pre a.f_elements zeta0 zeta1 zeta2 zeta3);
    f_ntt_layer_1_step_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_1_step_post a.f_elements
          zeta0
          zeta1
          zeta2
          zeta3
          out.f_elements);
    f_ntt_layer_1_step
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        ->
        op_ntt_layer_1_step a zeta0 zeta1 zeta2 zeta3);
    f_ntt_layer_2_step_pre
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_pre a.f_elements zeta0 zeta1);
    f_ntt_layer_2_step_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_post a.f_elements
          zeta0
          zeta1
          out.f_elements);
    f_ntt_layer_2_step
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        ->
        op_ntt_layer_2_step a zeta0 zeta1);
    f_ntt_layer_3_step_pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (zeta: i16) ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_pre a.f_elements zeta);
    f_ntt_layer_3_step_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_post a.f_elements zeta out.f_elements);
    f_ntt_layer_3_step
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (zeta: i16) ->
        op_ntt_layer_3_step a zeta);
    f_inv_ntt_layer_1_step_pre
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_1_step_pre a.f_elements
          zeta0
          zeta1
          zeta2
          zeta3);
    f_inv_ntt_layer_1_step_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_1_step_post a.f_elements
          zeta0
          zeta1
          zeta2
          zeta3
          out.f_elements);
    f_inv_ntt_layer_1_step
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        ->
        op_inv_ntt_layer_1_step a zeta0 zeta1 zeta2 zeta3);
    f_inv_ntt_layer_2_step_pre
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_pre a.f_elements zeta0 zeta1);
    f_inv_ntt_layer_2_step_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_post a.f_elements
          zeta0
          zeta1
          out.f_elements);
    f_inv_ntt_layer_2_step
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        ->
        op_inv_ntt_layer_2_step a zeta0 zeta1);
    f_inv_ntt_layer_3_step_pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (zeta: i16) ->
        Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_pre a.f_elements zeta);
    f_inv_ntt_layer_3_step_post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_post a.f_elements zeta out.f_elements
    );
    f_inv_ntt_layer_3_step
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) (zeta: i16) ->
        op_inv_ntt_layer_3_step a zeta);
    f_ntt_multiply_pre
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_multiply_pre lhs.f_elements
          rhs.f_elements
          zeta0
          zeta1
          zeta2
          zeta3);
    f_ntt_multiply_post
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.ntt_multiply_post lhs.f_elements
          rhs.f_elements
          zeta0
          zeta1
          zeta2
          zeta3
          out.f_elements);
    f_ntt_multiply
    =
    (fun
        (lhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (rhs: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (zeta0: i16)
        (zeta1: i16)
        (zeta2: i16)
        (zeta3: i16)
        ->
        op_ntt_multiply lhs rhs zeta0 zeta1 zeta2 zeta3);
    f_serialize_1__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr a));
    f_serialize_1__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array u8 (mk_usize 2))
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (impl.f_repr a) ==>
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (impl.f_repr a) out);
    f_serialize_1_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> serialize_1_ a);
    f_deserialize_1__pre
    =
    (fun (a: t_Slice u8) -> (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 2);
    f_deserialize_1__post
    =
    (fun (a: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length a) =. sz 2 ==>
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 1 a (impl.f_repr out));
    f_deserialize_1_ = (fun (a: t_Slice u8) -> deserialize_1_ a);
    f_serialize_4__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr a));
    f_serialize_4__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array u8 (mk_usize 8))
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (impl.f_repr a) ==>
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (impl.f_repr a) out);
    f_serialize_4_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> serialize_4_ a);
    f_deserialize_4__pre
    =
    (fun (a: t_Slice u8) -> (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 8);
    f_deserialize_4__post
    =
    (fun (a: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length a) =. sz 8 ==>
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 4 a (impl.f_repr out));
    f_deserialize_4_ = (fun (a: t_Slice u8) -> deserialize_4_ a);
    f_serialize_5__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr a));
    f_serialize_5__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array u8 (mk_usize 10))
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 5 (impl.f_repr a) ==>
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 5 (impl.f_repr a) out);
    f_serialize_5_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> serialize_5_ a);
    f_deserialize_5__pre
    =
    (fun (a: t_Slice u8) -> (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 10);
    f_deserialize_5__post
    =
    (fun (a: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length a) =. sz 10 ==>
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 5 a (impl.f_repr out));
    f_deserialize_5_ = (fun (a: t_Slice u8) -> deserialize_5_ a);
    f_serialize_10__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr a));
    f_serialize_10__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array u8 (mk_usize 20))
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (impl.f_repr a) ==>
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (impl.f_repr a) out);
    f_serialize_10_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> serialize_10_ a);
    f_deserialize_10__pre
    =
    (fun (a: t_Slice u8) -> (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 20);
    f_deserialize_10__post
    =
    (fun (a: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length a) =. sz 20 ==>
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 10 a (impl.f_repr out));
    f_deserialize_10_ = (fun (a: t_Slice u8) -> deserialize_10_ a);
    f_serialize_11__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr a));
    f_serialize_11__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array u8 (mk_usize 22))
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 11 (impl.f_repr a) ==>
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 11 (impl.f_repr a) out);
    f_serialize_11_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> serialize_11_ a);
    f_deserialize_11__pre
    =
    (fun (a: t_Slice u8) -> (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 22);
    f_deserialize_11__post
    =
    (fun (a: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length a) =. sz 22 ==>
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 11 a (impl.f_repr out));
    f_deserialize_11_ = (fun (a: t_Slice u8) -> deserialize_11_ a);
    f_serialize_12__pre
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr a));
    f_serialize_12__post
    =
    (fun
        (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector)
        (out: t_Array u8 (mk_usize 24))
        ->
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (impl.f_repr a) ==>
        Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (impl.f_repr a) out);
    f_serialize_12_
    =
    (fun (a: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) -> serialize_12_ a);
    f_deserialize_12__pre
    =
    (fun (a: t_Slice u8) -> (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 24);
    f_deserialize_12__post
    =
    (fun (a: t_Slice u8) (out: Libcrux_ml_kem.Vector.Portable.Vector_type.t_PortableVector) ->
        sz (Seq.length a) =. sz 24 ==>
        Libcrux_ml_kem.Vector.Traits.Spec.deserialize_post_N 12 a (impl.f_repr out));
    f_deserialize_12_ = (fun (a: t_Slice u8) -> deserialize_12_ a);
    f_rej_sample_pre
    =
    (fun (a: t_Slice u8) (out: t_Slice i16) ->
        (Core_models.Slice.impl__len #u8 a <: usize) =. mk_usize 24 &&
        (Core_models.Slice.impl__len #i16 out <: usize) =. mk_usize 16);
    f_rej_sample_post
    =
    (fun (a: t_Slice u8) (out: t_Slice i16) (out_future, result: (t_Slice i16 & usize)) ->
        b2t
        (((Core_models.Slice.impl__len #i16 out_future <: usize) =. mk_usize 16 <: bool) &&
          (result <=. mk_usize 16 <: bool)) /\
        (forall (j: usize).
            b2t (j <. result <: bool) ==>
            b2t
            (((out_future.[ j ] <: i16) >=. mk_i16 0 <: bool) &&
              ((out_future.[ j ] <: i16) <=. mk_i16 3328 <: bool))));
    f_rej_sample
    =
    fun (a: t_Slice u8) (out: t_Slice i16) ->
      let (tmp0: t_Slice i16), (out1: usize) =
        Libcrux_ml_kem.Vector.Portable.Sampling.rej_sample a out
      in
      let out:t_Slice i16 = tmp0 in
      let hax_temp_output:usize = out1 in
      out, hax_temp_output <: (t_Slice i16 & usize)
  }
