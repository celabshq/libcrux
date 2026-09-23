
-- Experimental lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
import sub_bytes
import key_expansion_step
import Utilities
import libcrux_aes

open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false

namespace libcrux_aes.platform.portable.aes_core
def rotate_bt (u : BitVec 4) : BitVec 4 :=
  let a0 := u.extractLsb 3 3
  let a1 := u.extractLsb 2 2
  let a2 := u.extractLsb 1 1
  let a3 := u.extractLsb 0 0
  (a3 ++ a0 ++ a1 ++ a2)

def aes_keygen_assisti_spec (rcon : u8) (i : Nat) (u : u16) : BitVec 16 :=
  let u_bv := u.toBitVec
  let a0 := u_bv.extractLsb 15 15
  let a1 := u_bv.extractLsb 14 14
  let a2 := u_bv.extractLsb 13 13
  let a3 := u_bv.extractLsb 12 12
  let top_nib := a0 ++ a1 ++ a2 ++ a3
  let n_rot := rotate_bt top_nib
  let ri : BitVec 8 := (rcon.toBitVec >>> i) &&& 1
  let n := (BitVec.zero 4 ++ n_rot) ^^^ ri
  (n.extractLsb 3 0) ++ top_nib ++ BitVec.zero 8

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
@[spec]
theorem aes_keygen_assisti_correct (rcon : u8) (i : Nat) (u : u16) :
⦃ ⌜ i < 8 ⌝ ⦄
  aes_keygen_assisti rcon i.toUSize64 u
⦃ ⇓ ⟨res⟩ =>
    ⌜ aes_keygen_assisti_spec rcon i u = res ⌝ ⦄ :=
  by
    unfold aes_keygen_assisti
    hax_mvcgen <;> simp at *
    .
      unfold aes_keygen_assisti_spec rotate_bt UInt16.xor
      rename_auto_n 12
      simp only [var_11, var_10, var_8, var_7, var_6]
      simp
      match i with
      | n + 8 => grind
      | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 =>
          simp
          bv_decide
    .
      grind

def rotword (v : Vector (BitVec 8) 4) : Vector (BitVec 8) 4 :=
  #v[v[3], v[0], v[1], v[2]]

def subword (v : Vector (BitVec 8) 4) : Vector (BitVec 8) 4 :=
  v.map (fun i => get_elem_SBOX (i.extractLsb 7 4) (i.extractLsb 3 0))

def aes_keygen_first (rcon : u8) (chunk : Vector (BitVec 8) 4) : Vector (BitVec 8) 4 :=
  let new_chunk := (rotword (subword chunk))
  #v[new_chunk[0], new_chunk[1], new_chunk[2], rcon.toBitVec ^^^ new_chunk[3]]

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
@[spec]
theorem aes_keygen_assist_correct (next : (RustArray u16 8)) (prev : (RustArray u16 8)) (rcon : u8) (a0 a1 a2 a3 : BitVec 8):
⦃ ⌜
    true
⌝ ⦄
  aes_keygen_assist next prev rcon
⦃ ⇓ ⟨res⟩ =>
    ⌜
      subword (get_word prev.toVec 3) = get_word res 2 /\
      aes_keygen_first rcon (get_word prev.toVec 3) = get_word res 3
    ⌝ ⦄ :=
  by
    unfold aes_keygen_assist
    hax_mvcgen[sub_bytes_correct_chunk, aes_keygen_assisti_correct] <;> simp at *
    .
      rename_auto_n 70
      unfold get_word aes_keygen_first subword rotword
      simp
      refine ⟨?_, ?_⟩ <;>
      refine ⟨?_, ?_, ?_, ?_⟩ <;>
        simp only [var_1] <;>
        unfold get_elem <;>
        simp only [Nat.sub_zero, Nat.reduceAdd, BitVec.truncate_eq_setWidth, Nat.reduceLeDiff,
          Nat.reducePow, Nat.lt_add_one, BitVec.setWidth_ofNat_of_le_of_lt, BitVec.ushiftRight_eq',
          BitVec.toNat_ofNat, Nat.reduceMod, BitVec.zero_or, ne_eq, reduceCtorEq, not_false_eq_true,
          Vector.getElem_set_ne, Nat.succ_ne_self, Vector.getElem_set_self, UInt16.toBitVec_xor,
          UInt16.toBitVec_shiftLeft, UInt16.toBitVec_and, UInt16.toBitVec_or,
          UInt16.toBitVec_shiftRight, UInt16.toBitVec_ofNat, BitVec.ofNat_eq_ofNat, BitVec.reduceMod,
          Nat.one_mod, BitVec.reduceShiftRightShiftRight, BitVec.shiftLeft_eq',
          UInt8.toBitVec_toUInt16, Nat.reduceLT, Nat.reduceEqDiff] <;>
        simp only [<- var_19, <- var_26, <- var_33, <- var_40, <- var_47, <- var_54, <- var_61, <- var_68] <;>
        bv_decide
    all_goals grind

set_option maxHeartbeats 10000000000000

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
@[spec]
theorem aes_keygen_assist1_correct (next : (RustArray u16 8)) (prev : (RustArray u16 8)) (a0 a1 a2 a3 : BitVec 8):
⦃ ⌜ true ⌝ ⦄
  aes_keygen_assist1 next prev
⦃ ⇓ ⟨res⟩ =>
    ⌜ let temp := subword (get_word prev.toVec 3)
      temp = get_word res 3 /\
      temp = get_word res 2 /\
      temp = get_word res 1 /\
      temp = get_word res 0  ⌝ ⦄ :=
by
    unfold aes_keygen_assist1
    hax_mvcgen[aes_keygen_assist_correct] <;> simp at *
    .
      rename_auto_n 70
      simp only [var_1]
      unfold get_word
      simp
      refine ⟨?_, ?_, ?_, ?_⟩ <;>
      refine ⟨?_, ?_, ?_, ?_⟩ <;>
      unfold get_elem <;>
      simp <;>
      simp only [<- var_19, <- var_22, <- var_25, <- var_28, <- var_31,
        <- var_34, <- var_37, <- var_40] <;>
      bv_decide
    all_goals grind

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
@[spec]
theorem aes_keygen_assist0_correct (rcon : u8) (next : (RustArray u16 8)) (prev : (RustArray u16 8)) (a0 a1 a2 a3 : BitVec 8):
⦃ ⌜ true ⌝ ⦄
  aes_keygen_assist0 next prev rcon
⦃ ⇓ ⟨res⟩ =>
    ⌜ let temp := aes_keygen_first rcon (get_word prev.toVec 3)
      temp = get_word res 3 /\
      temp = get_word res 2 /\
      temp = get_word res 1 /\
      temp = get_word res 0  ⌝ ⦄ :=
by
    unfold aes_keygen_assist0
    hax_mvcgen[aes_keygen_assist_correct] <;> simp at *
    .
      rename_auto_n 70
      simp only [var_1]
      unfold get_word
      simp
      refine ⟨?_, ?_, ?_, ?_⟩ <;>
      refine ⟨?_, ?_, ?_, ?_⟩ <;>
      unfold get_elem <;>
      simp <;>
      simp only [<- var_19, <- var_22, <- var_25, <- var_28, <- var_31,
        <- var_34, <- var_37, <- var_40] <;>
      bv_decide
    all_goals grind

end libcrux_aes.platform.portable.aes_core