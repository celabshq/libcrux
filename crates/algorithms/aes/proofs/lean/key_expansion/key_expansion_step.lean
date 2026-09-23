
-- Experimental lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/lean
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
def key_expand1_spec (p : u16) (n : u16) : BitVec 16 :=
  let p_bv := p.toBitVec
  let a := p_bv.extractLsb 15 12
  let b := p_bv.extractLsb 11 8
  let c := p_bv.extractLsb 7 4
  let d := p_bv.extractLsb 3 0

  n.toBitVec ^^^ ((a ^^^ b ^^^ c ^^^d) ++ (b ^^^ c ^^^ d) ++ (c ^^^ d) ++ d)

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
theorem key_expand1_correct (p : u16) (n : u16) :
⦃ ⌜ true ⌝ ⦄
  key_expand1 p n
⦃ ⇓ ⟨res⟩ =>
    ⌜ key_expand1_spec p n = res ⌝ ⦄ :=
  by
    unfold key_expand1
    hax_mvcgen <;> simp at *
    unfold key_expand1_spec
    simp
    rename_auto_n 7
    simp only [var_4, var_5, var_6, UInt16.xor]
    bv_decide

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
theorem key_expansion_correct (next : (RustArray u16 8)) (prev : (RustArray u16 8)) :
⦃ ⌜ true ⌝ ⦄
  key_expansion_step next prev
⦃ ⇓ ⟨res⟩ =>
    ⌜
    let row_prev i := get_word prev.toVec (3 - i)
    let row_next i := get_word next.toVec (3 - i)
    let row_res i := get_word res (3 - i)
    row_res 3 = xor_word (row_next 3) (row_prev 3) /\
    row_res 2 = xor_word (row_next 2) (xor_word (row_prev 2) (row_prev 3)) /\
    row_res 1 = xor_word (row_next 1) (xor_word (row_prev 1) (xor_word (row_prev 2) (row_prev 3))) /\
    row_res 0 = xor_word (row_next 0) (xor_word (row_prev 0) (xor_word (row_prev 1) (xor_word (row_prev 2) (row_prev 3))))
    ⌝ ⦄ :=
  by
    unfold key_expansion_step
    hax_mvcgen[key_expand1_correct] <;> simp at *
    unfold get_word get_elem xor_word
    .
      intros
      simp only [Nat.sub_zero, Nat.reduceAdd, ne_eq, reduceCtorEq, not_false_eq_true,
        Vector.getElem_set_ne, Nat.succ_ne_self, Vector.getElem_set_self, BitVec.ofNat_eq_ofNat,
        BitVec.mul_zero, BitVec.zero_add, BitVec.truncate_eq_setWidth, Nat.reduceLeDiff,
        Nat.reducePow, Nat.reduceLT, BitVec.setWidth_ofNat_of_le_of_lt, BitVec.ushiftRight_eq',
        BitVec.toNat_ofNat, Nat.reduceMod, BitVec.zero_or, Nat.reduceEqDiff, Nat.one_mod,
        BitVec.add_zero, Nat.zero_lt_succ, Nat.zero_mod, BitVec.ushiftRight_zero, Vector.mk_zip_mk,
        List.zip_toArray, List.zip_cons_cons, List.zip_nil_right, Vector.map_mk, List.map_toArray,
        List.map_cons, List.map_nil, Vector.eq_mk, Array.mk.injEq, List.cons.injEq, and_true,
        BitVec.mul_one, BitVec.reduceAdd, BitVec.reduceMul, Nat.lt_add_one]
      rename_auto_n 67
      simp only [<- var_3, <- var_9, <- var_15, <- var_21, <- var_27, <- var_33, <- var_39, <- var_45]
      unfold key_expand1_spec
      simp only [Nat.reduceSub, Nat.reduceAdd, Nat.sub_zero]
      simp only [ <- var_49, <- var_50, <- var_51, <- var_52, <- var_53, <- var_54, <- var_55, <- var_56]
      simp only [<- var_6, <- var_12, <- var_18, <- var_24, <- var_30, <- var_36, <- var_42, <- var_48]
      bv_decide
    all_goals grind

end libcrux_aes.platform.portable.aes_core