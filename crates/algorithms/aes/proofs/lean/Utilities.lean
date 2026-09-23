import Lean
import Hax
import libcrux_aes

open Lean Meta Elab Tactic

/--
Generates a name based on an index (0 -> a, 1 -> b, etc.)
-/
def getAutoName (i : Nat) : Name :=
    s!"var_{i}" |>.toName

syntax (name := renameAutoN) "rename_auto_n " num : tactic

@[tactic renameAutoN]
def evalRenameAutoN : Tactic := fun stx => do
  -- Parse the number 'n' from the tactic call
  let n := stx[1].isNatLit?.getD 0

  liftMetaTactic fun mvarId => do
    let lctx ← getLCtx
    let mut mvarId := mvarId
    let mut foundCount := 0

    for ldecl in lctx do
      if foundCount >= n then break

      -- Check if the hypothesis is inaccessible (has macro scopes or is a dagger/internal name)
      if ldecl.userName.hasMacroScopes || ldecl.userName.toString.startsWith "✝" then
        let newName := getAutoName foundCount
        mvarId ← mvarId.rename ldecl.fvarId newName
        foundCount := foundCount + 1

    return [mvarId]

--Some functions to work with how the AES matrix is represented.

def zero_array : Vector u16 8 :=
  Vector.replicate 8 (UInt16.ofBitVec <| BitVec.zero 16)

-- Get an element from the bit map
def get_elem (st : Vector u16 8) (elem_indx : BitVec 4) : BitVec 8 :=
  let shift : BitVec 16 := elem_indx.zeroExtend 16
  let b0 := ((st[0].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b1 := ((st[1].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b2 := ((st[2].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b3 := ((st[3].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b4 := ((st[4].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b5 := ((st[5].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b6 := ((st[6].toBitVec >>> shift) &&& 1#16).extractLsb 0 0
  let b7 := ((st[7].toBitVec >>> shift) &&& 1#16).extractLsb 0 0

  (0#8
    ||| (0#7 ++ b0)
    ||| (0#6 ++ b1 ++ 0#1)
    ||| (0#5 ++ b2 ++ 0#2)
    ||| (0#4 ++ b3 ++ 0#3)
    ||| (0#3 ++ b4 ++ 0#4)
    ||| (0#2 ++ b5 ++ 0#5)
    ||| (0#1 ++ b6 ++ 0#6)
    ||| (b7 ++ 0#7))

def set_elem (st : Vector u16 8) (elem_indx : BitVec 4) (new_elem : BitVec 8) : Vector u16 8 :=
  let shift : BitVec 16 := elem_indx.zeroExtend 16
  let mask : BitVec 16 := 1#16 <<< shift
  let bt0 := (0#15 ++ (new_elem &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt1 := (0#15 ++ ((new_elem >>> 1#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt2 := (0#15 ++ ((new_elem >>> 2#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt3 := (0#15 ++ ((new_elem >>> 3#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt4 := (0#15 ++ ((new_elem >>> 4#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt5 := (0#15 ++ ((new_elem >>> 5#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt6 := (0#15 ++ ((new_elem >>> 6#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let bt7 := (0#15 ++ ((new_elem >>> 7#8) &&& 0x01#8).extractLsb 0 0) <<< shift
  let new0 := (st[0].toBitVec &&& ~~~mask) ||| bt0
  let new1 := (st[1].toBitVec &&& ~~~mask) ||| bt1
  let new2 := (st[2].toBitVec &&& ~~~mask) ||| bt2
  let new3 := (st[3].toBitVec &&& ~~~mask) ||| bt3
  let new4 := (st[4].toBitVec &&& ~~~mask) ||| bt4
  let new5 := (st[5].toBitVec &&& ~~~mask) ||| bt5
  let new6 := (st[6].toBitVec &&& ~~~mask) ||| bt6
  let new7 := (st[7].toBitVec &&& ~~~mask) ||| bt7
  #v[UInt16.ofBitVec new0, UInt16.ofBitVec new1, UInt16.ofBitVec new2, UInt16.ofBitVec new3,
     UInt16.ofBitVec new4, UInt16.ofBitVec new5, UInt16.ofBitVec new6, UInt16.ofBitVec new7]

def get_word (st : Vector u16 8) (index : BitVec 4) : Vector (BitVec 8) 4 :=
  #v[
    get_elem st (4 * index + 3), get_elem st (4 * index + 2),
    get_elem st (4 * index + 1), get_elem st (4 * index + 0)
  ]

def xor_word (a b : Vector (BitVec 8) 4) : Vector (BitVec 8) 4 :=
  Vector.map
    (fun (p : BitVec 8 × BitVec 8) => p.1 ^^^ p.2) (Vector.zip a b)
