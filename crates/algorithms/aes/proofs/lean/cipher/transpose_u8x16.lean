import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
import Utilities
import libcrux_aes

open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false

namespace libcrux_aes.platform.portable.aes_core
def interleave_u8_1_spec (i0 : u8) (i1 : u8) : u16 :=
  UInt16.ofBitVec
  (
    let i0_btv := i0.toBitVec
    let i1_btv := i1.toBitVec
    (16 : Nat).fold (init := (0 : BitVec 16)) fun i _ res =>
      res ||| (((i0_btv >>> i) &&& 1).setWidth 16 <<< (2 * i)) ||| (((i1_btv >>> i) &&& 1).setWidth 16 <<< (2 * i + 1))
  )

set_option hax_mvcgen.specset "bv" in
@[hax_spec]
def interleave_u8_1.spec (i0 : u8) (i1 : u8) :
    Spec
      (requires := do pure True)
      (ensures := fun res => do (res ==? (interleave_u8_1_spec i0 i1)))
      (interleave_u8_1 (i0 : u8) (i1 : u8)) := {
  pureRequires := by hax_construct_pure <;> bv_decide
  pureEnsures := by hax_construct_pure <;> bv_decide
  contract := by
                hax_mvcgen [interleave_u8_1] <;> simp
                simp [interleave_u8_1_spec]
                bv_decide
}

def transposeU8toU16 (input : RustArray u8 16) : Vector u16 8 :=
  Vector.ofFn fun (bt_num : Fin 8) =>
    (16 : Nat).fold (init := (0 : u16)) fun mtx_elem_num _ bt_plane =>
      UInt16.ofBitVec (bt_plane.toBitVec ||| (((input.toVec[mtx_elem_num].toBitVec >>> bt_num.val) &&& 1).setWidth 16 <<< mtx_elem_num))

--Theorem to ensure that we can obtain the same elements from the input
set_option maxHeartbeats 10000000
theorem transpose_correct (input : (RustArray u8 16)) (i : Nat) (hi : i < 16) :
  get_elem (transposeU8toU16 input) (BitVec.ofFin ⟨i, hi⟩) = input.toVec[i].toBitVec := by
  simp
  match i with
  | n + 16 => grind
  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 =>

    simp only [get_elem, transposeU8toU16]
    simp
    generalize h0 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 0 _ = var_0
    generalize h1 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 1 _ = var_1
    generalize h2 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 2 _ = var_2
    generalize h3 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 3 _ = var_3
    generalize h4 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 4 _ = var_4
    generalize h5 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 5 _ = var_5
    generalize h6 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 6 _ = var_6
    generalize h7 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 7 _ = var_7
    generalize h8 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 8 _ = var_8
    generalize h9 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 9 _ = var_9
    generalize h10 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 10 _ = var_10
    generalize h11 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 11 _ = var_11
    generalize h12 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 12 _ = var_12
    generalize h13 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 13 _ = var_13
    generalize h14 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 14 _ = var_14
    generalize h15 : @getElem (Vector u8 16) Nat u8 (fun x i => i < 16) Vector.instGetElemNatLt input.toVec 15 _ = var_15

    bv_decide

end libcrux_aes.platform.portable.aes_core