import Std.Tactic.BVDecide
/-!
# Formalization of GF(2^8)

Elements of GF(2⁸) are represented as `BitVec 8`

The primitive polynomial
  p(x) = x⁸ + x⁴ + x³ + x + 1
is represented by its non-leading terms x⁴ + x³ + x + 1, occupiying positions
4, 3, 1, 0. That is:

  polyRed = 0x1B

-/

abbrev GF8 := BitVec 8

namespace GF8

-- Constants

def zero : GF8 := 0#8

def one : GF8 := BitVec.ofNat 8 1

-- Addition

def add (a b : GF8) : GF8 := a ^^^ b

-- Multiplication

def polyRed : GF8 := 0x1B#8

def xtime (a : GF8) : GF8 :=
  let shifted := a <<< 1#8
  if a.getLsbD 7 then shifted ^^^ polyRed else shifted

def mulStep (b : GF8) (state : GF8 × GF8) (bitIdx : Nat)
    : GF8 × GF8 :=
  let (acc, cur_a) := state
  (if b.getLsbD bitIdx then acc ^^^ cur_a else acc, xtime cur_a)

def mul (a b : GF8) : GF8 :=
  let bitIndices := List.range 8   -- [0, 1, …, 7]
  (bitIndices.foldl (mulStep b) (zero, a)).1

/-! Lemmas to make it easier for bv_decide -/

def xtime_bv (a : GF8) : GF8 :=
  (a <<< 1#8) ^^^ (if a.getMsbD 0 then 0x1B#8 else 0x00#8)

def mul3_bv (a : GF8) : GF8 :=
  xtime_bv a ^^^ a

-- xtime_bv equals GF8.xtime
theorem xtime_bv_eq_xtime (a : GF8) : xtime_bv a = GF8.xtime a := by
  simp [xtime_bv, GF8.xtime, GF8.polyRed]
  grind

-- Multiplication by 0x02 equals xtime_bv.
--
-- Both sides are functions of a single byte, so the claim is decided by
-- kernel reduction over all 256 values. `bv_decide` cannot be used here:
-- `xtime` shifts by a `BitVec` rather than a `Nat` literal, which the
-- bitblaster treats as an opaque term.
theorem mul_02_eq_xtime_bv (a : GF8) : GF8.mul 0x02#8 a = xtime_bv a := by
  revert a; decide +kernel

-- Multiplication by 0x03 equals mul3_bv.
theorem mul_03_eq_mul3_bv (a : GF8) : GF8.mul 0x03#8 a = mul3_bv a := by
  revert a; decide +kernel

/-- Expand xtime_bv into a form that bv_decide can close.
    After rewriting with this, the goal is a pure BitVec expression. -/
theorem xtime_bv_bv (a : GF8) :
    xtime_bv a =
      (a <<< 1#8) ^^^ (BitVec.ofBool (a.getMsbD 0)).zeroExtend 8 * 0x1B#8 := by
  simp [xtime_bv, BitVec.ofBool]
  split <;> simp [*]

end GF8
