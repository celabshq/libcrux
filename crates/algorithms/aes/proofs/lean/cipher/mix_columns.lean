
-- Experimental lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
import Utilities
import gf8
import libcrux_aes

open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false

namespace libcrux_aes.platform.portable.aes_core
-- MixColumns over GF(2^8), written straight from FIPS-197 section 5.1.3.
def mix_columns_state_spec (st : Vector u16 8) : Vector u16 8 :=
  (4 : Nat).fold (init := zero_array) fun group_indx _ res =>
  let index := group_indx * 4
  let s_0 := get_elem st index
  let s_1 := get_elem st (index + 1)
  let s_2 := get_elem st (index + 2)
  let s_3 := get_elem st (index + 3)

  let s_0' := GF8.add (GF8.add (GF8.add (GF8.mul (0x02#8) s_0) (GF8.mul (0x03#8) s_1)) s_2) s_3
  let s_1' := GF8.add (GF8.add (GF8.add (GF8.mul (0x02#8) s_1) (GF8.mul (0x03#8) s_2)) s_3) s_0
  let s_2' := GF8.add (GF8.add (GF8.add (GF8.mul (0x02#8) s_2) (GF8.mul (0x03#8) s_3)) s_0) s_1
  let s_3' := GF8.add (GF8.add (GF8.add (GF8.mul (0x02#8) s_3) (GF8.mul (0x03#8) s_0)) s_1) s_2

  let set_0 := set_elem res index s_0'
  let set_1 := set_elem set_0 (index + 1) s_1'
  let set_2 := set_elem set_1 (index + 2) s_2'
  set_elem set_2 (index + 3) s_3'

/-!
## Unrolling the generated loop

hax turns the `for i in 0..8` loop of `mix_columns_state` into a
`fold_range 0 8` carrying `(last_col, st)`. The bounds are literals, so the
fold can be unrolled completely by rewriting with its defining equation.

`mix_columns_state_unrolled` is the result written out by hand, one block per
bit-plane. It is not trusted: `mix_columns_state_eq_unrolled` below proves it
equal to the generated function, and the correctness theorem in
`cipher_theorems.lean` is stated about the generated function.
-/

set_option maxRecDepth 10000 in
def mix_columns_state_unrolled (__st__ : (RustArray u16 8)) :
    RustM (RustArray u16 8) := do
  let col0 : u16 ←
    ((← __st__[(0 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(0 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(0 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (0 : usize)
      (← ((← ((← ((← __st__[(0 : usize)]_?) ^^^? (0 : u16))) ^^^? col0))
        ^^^? (← ((← ((← (col0 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col0 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col1 : u16 ←
    ((← __st__[(1 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(1 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(1 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (1 : usize)
      (← ((← ((← ((← __st__[(1 : usize)]_?) ^^^? col0)) ^^^? col1))
        ^^^? (← ((← ((← (col1 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col1 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col2 : u16 ←
    ((← __st__[(2 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(2 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(2 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (2 : usize)
      (← ((← ((← ((← __st__[(2 : usize)]_?) ^^^? col1)) ^^^? col2))
        ^^^? (← ((← ((← (col2 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col2 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col3 : u16 ←
    ((← __st__[(3 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(3 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(3 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (3 : usize)
      (← ((← ((← ((← __st__[(3 : usize)]_?) ^^^? col2)) ^^^? col3))
        ^^^? (← ((← ((← (col3 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col3 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col4 : u16 ←
    ((← __st__[(4 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(4 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(4 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (4 : usize)
      (← ((← ((← ((← __st__[(4 : usize)]_?) ^^^? col3)) ^^^? col4))
        ^^^? (← ((← ((← (col4 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col4 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col5 : u16 ←
    ((← __st__[(5 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(5 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(5 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (5 : usize)
      (← ((← ((← ((← __st__[(5 : usize)]_?) ^^^? col4)) ^^^? col5))
        ^^^? (← ((← ((← (col5 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col5 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col6 : u16 ←
    ((← __st__[(6 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(6 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(6 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (6 : usize)
      (← ((← ((← ((← __st__[(6 : usize)]_?) ^^^? col5)) ^^^? col6))
        ^^^? (← ((← ((← (col6 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col6 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let col7 : u16 ←
    ((← __st__[(7 : usize)]_?)
      ^^^? (← ((← ((← ((← __st__[(7 : usize)]_?) &&&? (61166 : u16)))
          >>>? (1 : i32)))
        |||? (← ((← ((← __st__[(7 : usize)]_?) &&&? (4369 : u16)))
          <<<? (3 : i32))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (7 : usize)
      (← ((← ((← ((← __st__[(7 : usize)]_?) ^^^? col6)) ^^^? col7))
        ^^^? (← ((← ((← (col7 &&&? (52428 : u16))) >>>? (2 : i32)))
          |||? (← ((← (col7 &&&? (13107 : u16))) <<<? (2 : i32))))))));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (0 : usize)
      (← ((← __st__[(0 : usize)]_?) ^^^? col7)));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (1 : usize)
      (← ((← __st__[(1 : usize)]_?) ^^^? col7)));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (3 : usize)
      (← ((← __st__[(3 : usize)]_?) ^^^? col7)));
  let __st__ : (RustArray u16 8) ←
    (rust_primitives.hax.monomorphized_update_at.update_at_usize
      __st__
      (4 : usize)
      (← ((← __st__[(4 : usize)]_?) ^^^? col7)));
  (pure __st__)

/-- One step of a `usize` `fold_range` whose range is not yet empty. -/
theorem fold_range_usize_step {α} (s e : usize) (inv : α → usize → RustM Prop) (init : α)
    (body : α → usize → RustM α) (p) (h : s < e) :
    USize64.fold_range s e inv init body p =
      (do USize64.fold_range (s + 1) e inv (← body init s) body p) := by
  rw [USize64.fold_range]; simp [h]

/-- A `usize` `fold_range` over an empty range returns its accumulator. -/
theorem fold_range_usize_done {α} (s e : usize) (inv : α → usize → RustM Prop) (init : α)
    (body : α → usize → RustM α) (p) (h : ¬ s < e) :
    USize64.fold_range s e inv init body p = pure init := by
  rw [USize64.fold_range]; simp [h]

set_option maxRecDepth 10000 in
/-- The hax-generated `mix_columns_state` and its hand-unrolled form are the
same function. Proved by unfolding the fold eight times, without SAT solving. -/
theorem mix_columns_state_eq_unrolled (st : RustArray u16 8) :
    mix_columns_state st = mix_columns_state_unrolled st := by
  unfold mix_columns_state mix_columns_state_unrolled
  simp only [rust_primitives.hax.folds.fold_range]
  simp (config := {decide := true}) only [fold_range_usize_step, fold_range_usize_done,
    bind_assoc, pure_bind, USize64.reduceAdd]

end libcrux_aes.platform.portable.aes_core