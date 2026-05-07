module Libcrux_ml_kem.Vector.Portable.Vector_type
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_secrets.Int in
  let open Libcrux_secrets.Int.Public_integers in
  let open Libcrux_secrets.Traits in
  ()

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_1': Core_models.Marker.t_Copy t_PortableVector

let impl_1 = impl_1'

let zero (_: Prims.unit) =
  {
    f_elements
    =
    Libcrux_secrets.Traits.f_classify #(t_Array i16 (mk_usize 16))
      #FStar.Tactics.Typeclasses.solve
      (Rust_primitives.Hax.repeat (mk_i16 0) (mk_usize 16) <: t_Array i16 (mk_usize 16))
  }
  <:
  t_PortableVector

let to_i16_array (x: t_PortableVector) = x.f_elements

let from_i16_array (array: t_Slice i16) =
  { f_elements = array } <: t_PortableVector

let from_bytes (array: t_Slice u8) =
  let elements:t_Array i16 (mk_usize 16) =
    Rust_primitives.Hax.repeat (Libcrux_secrets.Int.v_I16 (mk_i16 0) <: i16) (mk_usize 16)
  in
  let elements:t_Array i16 (mk_usize 16) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      Libcrux_ml_kem.Vector.Traits.v_FIELD_ELEMENTS_IN_VECTOR
      (fun elements temp_1_ ->
          let elements:t_Array i16 (mk_usize 16) = elements in
          let _:usize = temp_1_ in
          true)
      elements
      (fun elements i ->
          let elements:t_Array i16 (mk_usize 16) = elements in
          let i:usize = i in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize elements
            i
            (((Libcrux_secrets.Int.f_as_i16 #u8
                    #FStar.Tactics.Typeclasses.solve
                    (array.[ (mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize ] <: u8)
                  <:
                  i16) <<!
                mk_i32 8
                <:
                i16) |.
              (Libcrux_secrets.Int.f_as_i16 #u8
                  #FStar.Tactics.Typeclasses.solve
                  (array.[ mk_usize 2 *! i <: usize ] <: u8)
                <:
                i16)
              <:
              i16)
          <:
          t_Array i16 (mk_usize 16))
  in
  let result:t_PortableVector = { f_elements = elements } <: t_PortableVector in
  let _:Prims.unit = admit () (* Panic freedom *) in
  result

let to_bytes (x: t_PortableVector) (bytes: t_Slice u8) =
  let e_bytes_len:usize = Core_models.Slice.impl__len #u8 bytes in
  let (bytes: t_Slice u8), (result: Prims.unit) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      Libcrux_ml_kem.Vector.Traits.v_FIELD_ELEMENTS_IN_VECTOR
      (fun bytes e_i ->
          let bytes:t_Slice u8 = bytes in
          let e_i:usize = e_i in
          (Core_models.Slice.impl__len #u8 bytes <: usize) =. e_bytes_len <: bool)
      bytes
      (fun bytes i ->
          let bytes:t_Slice u8 = bytes in
          let i:usize = i in
          let bytes:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize bytes
              ((mk_usize 2 *! i <: usize) +! mk_usize 1 <: usize)
              (Libcrux_secrets.Int.f_as_u8 #i16
                  #FStar.Tactics.Typeclasses.solve
                  ((x.f_elements.[ i ] <: i16) >>! mk_i32 8 <: i16)
                <:
                u8)
          in
          let bytes:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize bytes
              (mk_usize 2 *! i <: usize)
              (Libcrux_secrets.Int.f_as_u8 #i16
                  #FStar.Tactics.Typeclasses.solve
                  (x.f_elements.[ i ] <: i16)
                <:
                u8)
          in
          bytes),
    ()
    <:
    (t_Slice u8 & Prims.unit)
  in
  let _:Prims.unit = admit () (* Panic freedom *) in
  let _:Prims.unit = result in
  bytes
