import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
import Utilities
import transpose_u8x16
import shift_rows
import xor_key1
import sub_bytes
import mix_columns
import libcrux_aes

open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false

namespace libcrux_aes.platform.portable.aes_core
set_option maxHeartbeats 1000000
set_option hax_mvcgen.specset "bv" in
theorem transpose_u8x16_correct (input : (RustArray u8 16)) (output : (RustArray u16 8)) :
⦃ ⌜ true = true ⌝ ⦄
transpose_u8x16 input output
⦃ ⇓ ⟨res_output⟩ =>
    ⌜ res_output = transposeU8toU16 input ⌝ ⦄
:= by
    unfold transpose_u8x16
    hax_mvcgen[interleave_u8_1] <;> simp at *
    .
      ext i

      repeat (rename True => h; clear h)
      rename_auto_n 59

      rw [var_48, var_49, var_50, var_51, var_52, var_53, var_54, var_55]

      revert var_56
      match i with
      | n + 8 => intro h; contradiction
      | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 =>
          intros; simp

          unfold transposeU8toU16; simp
          unfold interleave_u8_1_spec; simp

          rw [ <- var_47, <- var_2, <- var_5, <- var_7, <- var_10, <- var_12, <- var_15,
            <- var_17, <- var_20, <- var_22, <- var_25, <- var_27, <- var_30,
            <- var_32, <- var_35, <- var_37]

          bv_decide

    all_goals grind

set_option maxHeartbeats 100000000
set_option hax_mvcgen.specset "bv" in
@[spec]
theorem shift_rows_state_correct (st : (RustArray u16 8)) :
⦃ ⌜ true = true ⌝ ⦄
shift_rows_state st
⦃ ⇓ ⟨res⟩ =>
    ⌜ res = (shift_rows_stat_spec st) ⌝ ⦄
:= by
    unfold shift_rows_state

    hax_mvcgen
    <;> simp at *
    .
      ext i
      match i with
      | n + 8 => intros; omega
      | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 =>
        simp only [shift_rows_stat_spec] at *
        rename_auto_n 41
        simp only [Vector.getElem_map]
        simp only [var_40, var_38, var_36, var_34, var_32, var_30, var_28, var_26]
        simp only [shift_row_u16_spec]
        simp
        simp only [<- var_39, <- var_37, <- var_35, <- var_33, <- var_31, <- var_29, <- var_27, <- var_25]
    all_goals grind

set_option maxHeartbeats 100000000
set_option hax_mvcgen.specset "bv" in
@[spec]
theorem xor_key1_correct (st : (RustArray u16 8))(k : (RustArray u16 8)) :
⦃ ⌜ true = true ⌝ ⦄
xor_key1_state st k
⦃ ⇓ ⟨res⟩ =>
    ⌜ res = (xor_key1_state_spec st.toVec k.toVec) ⌝ ⦄
:= by
    unfold xor_key1_state

    hax_mvcgen
    <;> simp at *
    .
      unfold xor_key1_state_spec
      simp
      rename_auto_n 41
      ext i
      match i with
      | n + 8 => omega
      | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 =>
        simp
        simp only [<- var_29, <- var_5, <- var_9, <- var_13, <- var_17, <- var_21, <- var_25, <- var_32, <- var_33, <- var_34, <- var_35, <- var_36, <- var_37, <- var_38, <- var_39, <- var_40]

    all_goals grind

set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
theorem sub_bytes_correct (n : BitVec 4) (a : BitVec 8) (st : RustArray u16 8) :
⦃ ⌜ a = get_elem st.toVec n ⌝ ⦄
sub_bytes_state st
⦃ ⇓ ⟨res_output⟩ =>
    ⌜ get_elem_SBOX (a.extractLsb 7 4) (a.extractLsb 3 0) = get_elem res_output n  ⌝ ⦄
:= by

  unfold sub_bytes_state
  hax_mvcgen <;> simp at *
  .
    rename_auto_n 45

    simp only [var_0]
    simp only [get_elem, get_elem_SBOX]
    simp only [Nat.reduceAdd, Nat.sub_zero, beq_iff_eq, BitVec.ushiftRight_eq', BitVec.zero_or,
      ne_eq, reduceCtorEq, not_false_eq_true, Vector.getElem_set_ne, Nat.succ_ne_self,
      Vector.getElem_set_self, UInt16.toBitVec_not, UInt16.toBitVec_xor, UInt16.toBitVec_and,
      Nat.reduceEqDiff]
    simp only [<- var_24, <- var_2, <- var_4, <- var_6, <- var_8, <- var_10, <- var_12, <- var_14 ]
    bv_decide
  all_goals grind

set_option maxHeartbeats 100000000
set_option hax_mvcgen.specset "bv" in
theorem mix_columns_state_unrolled_correct (st : RustArray u16 8) :
⦃ ⌜ true = true ⌝ ⦄
mix_columns_state_unrolled st
⦃ ⇓ ⟨res⟩ =>
    ⌜ res = mix_columns_state_spec st.toVec ⌝ ⦄
:= by
    rcases st with ⟨⟨⟨l⟩, hl⟩⟩
    obtain _|⟨a0,_|⟨a1,_|⟨a2,_|⟨a3,_|⟨a4,_|⟨a5,_|⟨a6,_|⟨a7,_|⟨_,_⟩⟩⟩⟩⟩⟩⟩⟩⟩ := l
      <;> simp at hl
    unfold mix_columns_state_unrolled
    hax_mvcgen
    <;> simp at *
    .
      subst_vars
      simp only [mix_columns_state_spec, Nat.fold_succ, Nat.fold_zero, set_elem, get_elem,
        zero_array, GF8.add, GF8.mul_02_eq_xtime_bv, GF8.mul_03_eq_mul3_bv, GF8.mul3_bv,
        GF8.xtime_bv]
      simp
      bv_decide
    all_goals grind

-- MixColumns, stated about the generated function. The proof goes through the
-- unrolled form; see `mix_columns_state_eq_unrolled` in `mix_columns.lean`.
theorem mix_columns_state_correct (st : RustArray u16 8) :
⦃ ⌜ true = true ⌝ ⦄
mix_columns_state st
⦃ ⇓ ⟨res⟩ =>
    ⌜ res = mix_columns_state_spec st.toVec ⌝ ⦄
:= by
  rw [mix_columns_state_eq_unrolled]
  exact mix_columns_state_unrolled_correct st

end libcrux_aes.platform.portable.aes_core