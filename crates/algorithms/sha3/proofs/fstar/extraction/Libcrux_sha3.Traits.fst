module Libcrux_sha3.Traits
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// A Keccak Item for multiplexing arithmetic implementations.
class t_KeccakItem (v_Self: Type0) (v_N: usize) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:Core_models.Clone.t_Clone v_Self;
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i1:Core_models.Marker.t_Copy v_Self;
  f_zero_pre:x: Prims.unit
    -> pred:
      Type0
        { (let _:Prims.unit = x in
            true) ==>
          pred };
  f_zero_post:Prims.unit -> v_Self -> Type0;
  f_zero:x0: Prims.unit -> Prims.Pure v_Self (f_zero_pre x0) (fun result -> f_zero_post x0 result);
  f_xor5_pre:a: v_Self -> b: v_Self -> c: v_Self -> d: v_Self -> e: v_Self
    -> pred: Type0{true ==> pred};
  f_xor5_post:v_Self -> v_Self -> v_Self -> v_Self -> v_Self -> v_Self -> Type0;
  f_xor5:x0: v_Self -> x1: v_Self -> x2: v_Self -> x3: v_Self -> x4: v_Self
    -> Prims.Pure v_Self
        (f_xor5_pre x0 x1 x2 x3 x4)
        (fun result -> f_xor5_post x0 x1 x2 x3 x4 result);
  f_rotate_left1_and_xor_pre:a: v_Self -> b: v_Self -> pred: Type0{true ==> pred};
  f_rotate_left1_and_xor_post:v_Self -> v_Self -> v_Self -> Type0;
  f_rotate_left1_and_xor:x0: v_Self -> x1: v_Self
    -> Prims.Pure v_Self
        (f_rotate_left1_and_xor_pre x0 x1)
        (fun result -> f_rotate_left1_and_xor_post x0 x1 result);
  f_xor_and_rotate_pre:v_LEFT: i32 -> v_RIGHT: i32 -> a: v_Self -> b: v_Self
    -> pred:
      Type0
        { v_LEFT >. mk_i32 0 && v_LEFT <. mk_i32 64 && v_RIGHT >. mk_i32 0 && v_RIGHT <. mk_i32 64 &&
          (v_LEFT +! v_RIGHT <: i32) =. mk_i32 64 ==>
          pred };
  f_xor_and_rotate_post:v_LEFT: i32 -> v_RIGHT: i32 -> v_Self -> v_Self -> v_Self -> Type0;
  f_xor_and_rotate:v_LEFT: i32 -> v_RIGHT: i32 -> x0: v_Self -> x1: v_Self
    -> Prims.Pure v_Self
        (f_xor_and_rotate_pre v_LEFT v_RIGHT x0 x1)
        (fun result -> f_xor_and_rotate_post v_LEFT v_RIGHT x0 x1 result);
  f_and_not_xor_pre:a: v_Self -> b: v_Self -> c: v_Self -> pred: Type0{true ==> pred};
  f_and_not_xor_post:v_Self -> v_Self -> v_Self -> v_Self -> Type0;
  f_and_not_xor:x0: v_Self -> x1: v_Self -> x2: v_Self
    -> Prims.Pure v_Self
        (f_and_not_xor_pre x0 x1 x2)
        (fun result -> f_and_not_xor_post x0 x1 x2 result);
  f_xor_constant_pre:a: v_Self -> c: u64 -> pred: Type0{true ==> pred};
  f_xor_constant_post:v_Self -> u64 -> v_Self -> Type0;
  f_xor_constant:x0: v_Self -> x1: u64
    -> Prims.Pure v_Self (f_xor_constant_pre x0 x1) (fun result -> f_xor_constant_post x0 x1 result);
  f_xor_pre:a: v_Self -> b: v_Self -> pred: Type0{true ==> pred};
  f_xor_post:v_Self -> v_Self -> v_Self -> Type0;
  f_xor:x0: v_Self -> x1: v_Self
    -> Prims.Pure v_Self (f_xor_pre x0 x1) (fun result -> f_xor_post x0 x1 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_N:usize) {|i: t_KeccakItem v_Self v_N|} -> i._super_i0

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_N:usize) {|i: t_KeccakItem v_Self v_N|} -> i._super_i1

let get_ij
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_KeccakItem v_T v_N)
      (arr: t_Array v_T (mk_usize 25))
      (i j: usize)
    : Prims.Pure v_T (requires i <. mk_usize 5 && j <. mk_usize 5) (fun _ -> Prims.l_True) =
  arr.[ (mk_usize 5 *! i <: usize) +! j <: usize ]

let set_ij
      (v_N: usize)
      (#v_T: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: t_KeccakItem v_T v_N)
      (arr: t_Array v_T (mk_usize 25))
      (i j: usize)
      (value: v_T)
    : Prims.Pure (t_Array v_T (mk_usize 25))
      (requires i <. mk_usize 5 && j <. mk_usize 5)
      (fun _ -> Prims.l_True) =
  let arr:t_Array v_T (mk_usize 25) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize arr
      ((mk_usize 5 *! i <: usize) +! j <: usize)
      value
  in
  arr

/// Trait to load new bytes into the state.
class t_Absorb (v_Self: Type0) (v_N: usize) = {
  f_load_block_pre:v_RATE: usize -> self_: v_Self -> input: t_Array (t_Slice u8) v_N -> start: usize
    -> pred:
      Type0
        { b2t
          ((v_N <>. mk_usize 0 <: bool) && (Libcrux_sha3.Proof_utils.valid_rate v_RATE <: bool) &&
            (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
                (Rust_primitives.Hax.Int.from_machine v_RATE <: Hax_lib.Int.t_Int)
                <:
                Hax_lib.Int.t_Int) <=
              (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                      (input.[ mk_usize 0 ] <: t_Slice u8)
                    <:
                    usize)
                <:
                Hax_lib.Int.t_Int)
              <:
              bool)) /\ Libcrux_sha3.Proof_utils.slices_same_len v_N input ==>
          pred };
  f_load_block_post:v_RATE: usize -> v_Self -> t_Array (t_Slice u8) v_N -> usize -> v_Self -> Type0;
  f_load_block:v_RATE: usize -> x0: v_Self -> x1: t_Array (t_Slice u8) v_N -> x2: usize
    -> Prims.Pure v_Self
        (f_load_block_pre v_RATE x0 x1 x2)
        (fun result -> f_load_block_post v_RATE x0 x1 x2 result);
  f_load_last_pre:
      v_RATE: usize ->
      v_DELIMITER: u8 ->
      self_: v_Self ->
      input: t_Array (t_Slice u8) v_N ->
      start: usize ->
      len: usize
    -> pred:
      Type0
        { b2t
          ((v_N <>. mk_usize 0 <: bool) && (Libcrux_sha3.Proof_utils.valid_rate v_RATE <: bool) &&
            (len <. v_RATE <: bool) &&
            (((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
                (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
                <:
                Hax_lib.Int.t_Int) <=
              (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8
                      (input.[ mk_usize 0 ] <: t_Slice u8)
                    <:
                    usize)
                <:
                Hax_lib.Int.t_Int)
              <:
              bool)) /\ Libcrux_sha3.Proof_utils.slices_same_len v_N input ==>
          pred };
  f_load_last_post:
      v_RATE: usize ->
      v_DELIMITER: u8 ->
      v_Self ->
      t_Array (t_Slice u8) v_N ->
      usize ->
      usize ->
      v_Self
    -> Type0;
  f_load_last:
      v_RATE: usize ->
      v_DELIMITER: u8 ->
      x0: v_Self ->
      x1: t_Array (t_Slice u8) v_N ->
      x2: usize ->
      x3: usize
    -> Prims.Pure v_Self
        (f_load_last_pre v_RATE v_DELIMITER x0 x1 x2 x3)
        (fun result -> f_load_last_post v_RATE v_DELIMITER x0 x1 x2 x3 result)
}

class t_Squeeze (v_Self: Type0) (v_T: Type0) = {
  // TODO: This super variable is problematic and makes typecheck fail
  // https://github.com/cryspen/hax/issues/1554
  // [@@@ FStar.Tactics.Typeclasses.no_method]_super_18390613159176269294:t_KeccakItem v_T (mk_usize 1);
  f_squeeze_pre:v_RATE: usize -> self_: v_Self -> out: t_Slice u8 -> start: usize -> len: usize
    -> pred:
      Type0
        { Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
          ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) <=
          (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out <: usize)
            <:
            Hax_lib.Int.t_Int) ==>
          pred };
  f_squeeze_post:
      v_RATE: usize ->
      self_: v_Self ->
      out: t_Slice u8 ->
      start: usize ->
      len: usize ->
      out_future: t_Slice u8
    -> pred:
      Type0
        { pred ==>
          (Core_models.Slice.impl__len #u8 out_future <: usize) =. (Core_models.Slice.impl__len #u8 out <: usize)
        };
  f_squeeze:v_RATE: usize -> x0: v_Self -> x1: t_Slice u8 -> x2: usize -> x3: usize
    -> Prims.Pure (t_Slice u8)
        (f_squeeze_pre v_RATE x0 x1 x2 x3)
        (fun result -> f_squeeze_post v_RATE x0 x1 x2 x3 result)
}

// TODO: See above
// [@@ FStar.Tactics.Typeclasses.tcinstance]
// let _ = fun (v_Self:Type0) (v_T:Type0) {|i: t_Squeeze v_Self v_T|} -> i._super_18390613159176269294

/// Trait to squeeze bytes out of the state.
/// Note that this is implemented for each platform (1, 2, 4) because hax can\'t
/// handle the mutability required for a generic implementation.
/// Store blocks `N = 2`
class t_Squeeze2 (v_Self: Type0) (v_T: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_KeccakItem v_T (mk_usize 2);
  f_squeeze2_pre:
      v_RATE: usize ->
      self_: v_Self ->
      out0: t_Slice u8 ->
      out1: t_Slice u8 ->
      start: usize ->
      len: usize
    -> pred:
      Type0
        { Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
          ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) <=
          (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
            <:
            Hax_lib.Int.t_Int) &&
          (Core_models.Slice.impl__len #u8 out0 <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) ==>
          pred };
  f_squeeze2_post:
      v_RATE: usize ->
      self_: v_Self ->
      out0: t_Slice u8 ->
      out1: t_Slice u8 ->
      start: usize ->
      len: usize ->
      x: (t_Slice u8 & t_Slice u8)
    -> pred:
      Type0
        { pred ==>
          (let (out0_future: t_Slice u8), (out1_future: t_Slice u8) = x in
            (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out0 <: usize) &&
            (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out1 <: usize)) };
  f_squeeze2:
      v_RATE: usize ->
      x0: v_Self ->
      x1: t_Slice u8 ->
      x2: t_Slice u8 ->
      x3: usize ->
      x4: usize
    -> Prims.Pure (t_Slice u8 & t_Slice u8)
        (f_squeeze2_pre v_RATE x0 x1 x2 x3 x4)
        (fun result -> f_squeeze2_post v_RATE x0 x1 x2 x3 x4 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_T:Type0) {|i: t_Squeeze2 v_Self v_T|} -> i._super_i0

/// Trait to squeeze bytes out of the state.
/// Note that this is implemented for each platform (1, 2, 4) because hax can\'t
/// handle the mutability required for a generic implementation.
/// Store blocks `N = 4`
class t_Squeeze4 (v_Self: Type0) (v_T: Type0) = {
  [@@@ FStar.Tactics.Typeclasses.no_method]_super_i0:t_KeccakItem v_T (mk_usize 4);
  f_squeeze4_pre:
      v_RATE: usize ->
      self_: v_Self ->
      out0: t_Slice u8 ->
      out1: t_Slice u8 ->
      out2: t_Slice u8 ->
      out3: t_Slice u8 ->
      start: usize ->
      len: usize
    -> pred:
      Type0
        { Libcrux_sha3.Proof_utils.valid_rate v_RATE && len <=. v_RATE &&
          ((Rust_primitives.Hax.Int.from_machine start <: Hax_lib.Int.t_Int) +
            (Rust_primitives.Hax.Int.from_machine len <: Hax_lib.Int.t_Int)
            <:
            Hax_lib.Int.t_Int) <=
          (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
            <:
            Hax_lib.Int.t_Int) &&
          (Core_models.Slice.impl__len #u8 out0 <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out0 <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out0 <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize) ==>
          pred };
  f_squeeze4_post:
      v_RATE: usize ->
      self_: v_Self ->
      out0: t_Slice u8 ->
      out1: t_Slice u8 ->
      out2: t_Slice u8 ->
      out3: t_Slice u8 ->
      start: usize ->
      len: usize ->
      x: (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
    -> pred:
      Type0
        { pred ==>
          (let
            (out0_future: t_Slice u8),
            (out1_future: t_Slice u8),
            (out2_future: t_Slice u8),
            (out3_future: t_Slice u8) =
              x
            in
            (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out0 <: usize) &&
            (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out1 <: usize) &&
            (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out2 <: usize) &&
            (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
            (Core_models.Slice.impl__len #u8 out3 <: usize)) };
  f_squeeze4:
      v_RATE: usize ->
      x0: v_Self ->
      x1: t_Slice u8 ->
      x2: t_Slice u8 ->
      x3: t_Slice u8 ->
      x4: t_Slice u8 ->
      x5: usize ->
      x6: usize
    -> Prims.Pure (t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
        (f_squeeze4_pre v_RATE x0 x1 x2 x3 x4 x5 x6)
        (fun result -> f_squeeze4_post v_RATE x0 x1 x2 x3 x4 x5 x6 result)
}

[@@ FStar.Tactics.Typeclasses.tcinstance]
let _ = fun (v_Self:Type0) (v_T:Type0) {|i: t_Squeeze4 v_Self v_T|} -> i._super_i0
