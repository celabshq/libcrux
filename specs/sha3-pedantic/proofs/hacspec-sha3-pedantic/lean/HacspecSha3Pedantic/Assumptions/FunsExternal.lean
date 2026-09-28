-- [hacspec_sha3_pedantic]: external functions.
-- Seeded by hax from Extraction/FunsExternal_Template.lean: fill the holes.
-- hax never modifies this file; after re-extraction, compare it against the
-- regenerated template to see what changed.
import Aeneas
import CoreModels
import HacspecSha3Pedantic.Extraction.Types
open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM ControlFlow Error
open Std.Do
set_option linter.dupNamespace false
set_option linter.hashCommand false
set_option linter.unusedVariables false
set_option linter.style.whitespace false
set_option linter.style.setOption false
set_option linter.style.longLine false

/- You can set the `maxHeartbeats` value with the `-max-heartbeats` CLI option -/
set_option maxHeartbeats 1000000

/- You can set the `maxRecDepth` value with the `-max-recdepth` CLI option -/
set_option maxRecDepth 2048
open hacspec_sha3_pedantic

/-! ## `RangeInclusive`, supplied here rather than by CoreModels

    `core-models` declares the type (`ops.range.RangeInclusive`) but ships no
    `new` and no `Iterator` instance, so `for j in a..=b` does not extract. The
    two definitions below fill exactly that gap, modelled on CoreModels'
    `IteratorRange.next` for the half-open `Range`.

    CAVEAT. Rust's `RangeInclusive` carries an `exhausted` flag, which the
    modelled type does not have: it is a bare `{ start, end }`. So `next`
    cannot distinguish "already yielded `end`" from "start > end" when `end` is
    the largest value of the type, and this model fails (panics on the overflow
    in `forward_checked`) where Rust would yield `A::MAX` and then stop. Every
    inclusive range in this crate is small and fixed (`0..=l` with `l ≤ 6`,
    `1..=t mod 255`, and the round indices of Algorithm 7), so the difference
    is unreachable here -- but a proof about this model is a proof about
    non-saturating ranges only. -/

namespace CoreModels.core.ops.range

/-- `RangeInclusive::new(start, end)`. -/
def RangeInclusive.new {A : Type} (start «end» : A) : Aeneas.Std.RustM (RangeInclusive A) :=
  Aeneas.Std.RustM.ok { start := start, «end» := «end» }

/-- `Iterator::next` for `RangeInclusive<A>`: yield `start` while
    `start ≤ end`, then step forward. The half-open `Range` model tests
    `start < end`; inclusive means `≤`, which is the whole difference. -/
def RangeInclusive.Insts.CoreIterTraitsIteratorIterator.next {A : Type}
    (StepInst : CoreModels.core.iter.range.Step A) :
    RangeInclusive A → Aeneas.Std.RustM ((Option A) × RangeInclusive A) := fun range => do
  let cmp ← StepInst.corecmpPartialOrdInst.partial_cmp range.start range.«end»
  let atOrBelow : Bool := match cmp with
    | Option.some o => match o with
                       | CoreModels.core.cmp.Ordering.Less => true
                       | CoreModels.core.cmp.Ordering.Equal => true
                       | _ => false
    | _ => false
  if atOrBelow then
    let cur ← StepInst.cloneCloneInst.clone range.start
    let next? ← StepInst.forward_checked cur 1#usize
    match next? with
    | Option.none      => .fail .panic
    | Option.some next => .ok (Option.some cur, { range with start := next })
  else .ok (Option.none, range)

end CoreModels.core.ops.range

/-! ## `Copy` for `bool`, likewise missing

    CoreModels has `marker.Copy` instances for every integer type and a
    `clone.Clone` instance for `Bool`, but no `marker.Copy Bool` -- so
    `copy_from_slice` on a `[bool]` does not resolve. This is the instance
    those two already determine; nothing is assumed. -/

namespace CoreModels.core

def Bool.Insts.CoreMarkerCopy : marker.Copy Bool := {
  cloneCloneInst := Bool.Insts.CoreCloneClone
}

end CoreModels.core

/-! ## The operations of `bits::BitStr`

    Models for the opaque methods of the arbitrary-length bit string; see
    `Assumptions/TypesExternal.lean` for what is and is not assumed by them.
    Each is the list operation the Standard's notation stands for, so a reader
    checking `bits.rs` against Sec. 2.3 can check these against the same text.

    Lengths and positions are a `nat.Nat`, so `len` is total: counting the
    bits of a string is not something a machine can fail at here. What can
    fail is `bit`, `trunc` and `slice` outside the string -- conditions the
    Standard states -- and `to_bits`, where a string does not fit a `usize`;
    that last one is not reachable from the specification's own call sites,
    which apply it to `r`-bit blocks with `r < 1600`. -/

namespace hacspec_sha3_pedantic

/-- The bits of a byte, least significant first — App. B.1 reads a byte this
    way, and `BitStr::from_bytes` packs it this way. -/
def bits.bitsOfByte (b : Std.U8) : List Bool :=
  (List.range 8).map (fun j => b.bv.getLsbD j)

/-- `h2b(H)` at its maximum `n`: the `8m` bits of the bytes `H`. -/
def bits.bitsOfBytes (bs : List Std.U8) : List Bool :=
  bs.flatMap bits.bitsOfByte

/-- One byte from the bits `f 0 … f 7`, least significant first. -/
def bits.byteOf (f : Nat → Bool) : Std.U8 :=
  ⟨(List.range 8).foldl (fun acc j => if f j then acc ||| BitVec.twoPow 8 j else acc) 0#8⟩

/-- `b2h(S)` — Algorithm 11, in the Standard's own order: pad `S` with
    `0^(-n mod 8)`, then read the result eight bits at a time. -/
def bits.bytesOfBits (s : List Bool) : List Std.U8 :=
  let t := s ++ List.replicate ((8 - s.length % 8) % 8) false
  (List.range (t.length / 8)).map (fun i => bits.byteOf (fun j => t[8 * i + j]!))

def bits.BitStr.Insts.CoreCloneClone.clone (s : bits.BitStr) : RustM bits.BitStr :=
  ok s

def bits.BitStr.empty : RustM bits.BitStr := ok ([] : List Bool)

/-- `len(S)` — Sec. 2.3. Total: the length is a nonnegative integer, and
    `nat.Nat` is where that is taken literally. -/
def bits.BitStr.len (s : bits.BitStr) : RustM nat.Nat :=
  ok (s : List Bool).length

def bits.BitStr.is_empty (s : bits.BitStr) : RustM Bool :=
  ok ((s : List Bool).isEmpty)

/-- `S[i]` — Sec. 2.3. -/
def bits.BitStr.bit (s : bits.BitStr) (i : nat.Nat) : RustM Bool :=
  if i < (s : List Bool).length then ok ((s : List Bool)[i]!) else fail .panic

/-- Private in Rust, and unused by the specification: the pure operations are
    the ones the Standard names. Modelled for completeness. -/
def bits.BitStr.push_mut (s : bits.BitStr) (b : Bool) : RustM bits.BitStr :=
  ok ((s : List Bool) ++ [b])

/-- `0^n` — Sec. 2.3. -/
def bits.BitStr.zeros (n : nat.Nat) : RustM bits.BitStr :=
  ok (List.replicate n false)

def bits.BitStr.from_bits (b : Slice Bool) : RustM bits.BitStr := ok b.val

/-- Fails where the length outruns a `usize`; applied only to `r`-bit blocks. -/
def bits.BitStr.to_bits (s : bits.BitStr) : RustM (alloc.vec.Vec Bool) :=
  if h : (s : List Bool).length ≤ Std.Usize.max then ok ⟨(s : List Bool), h⟩
  else fail .panic

/-- `X || Y` — Sec. 2.3. -/
def bits.BitStr.concat (x : bits.BitStr) (y : bits.BitStr) : RustM bits.BitStr :=
  ok ((x : List Bool) ++ (y : List Bool))

/-- `Trunc_s(X)` — Sec. 2.3. -/
def bits.BitStr.trunc (x : bits.BitStr) (s : nat.Nat) : RustM bits.BitStr :=
  if s ≤ (x : List Bool).length then ok ((x : List Bool).take s)
  else fail .panic

/-- The `n` bits of `X` from `from` — the blocks `P_i` of Algorithm 8. -/
def bits.BitStr.slice (x : bits.BitStr) (from_ : nat.Nat) (n : nat.Nat) :
    RustM bits.BitStr :=
  if from_ + n ≤ (x : List Bool).length then
    ok (((x : List Bool).drop from_).take n)
  else fail .panic

/-- Algorithm 10 at its maximum `n`. -/
def bits.BitStr.from_bytes (h : Slice Std.U8) : RustM bits.BitStr :=
  ok (bits.bitsOfBytes h.val)

/-- Algorithm 11. -/
def bits.BitStr.to_bytes (s : bits.BitStr) : RustM (alloc.vec.Vec Std.U8) :=
  let bs := bits.bytesOfBits (s : List Bool)
  if h : bs.length ≤ Std.Usize.max then ok ⟨bs, h⟩ else fail .panic

/-! ## The operations of `nat::Nat`

    Models for the opaque operations of the nonnegative integers; see
    `Assumptions/TypesExternal.lean` for what the type is and what its `u128`
    implementation does and does not promise.

    Three of these can fail, and each is written to fail exactly where the
    Rust does, since a model that succeeded where the implementation panics
    would prove nothing: `sub` below zero, `div` and `rem` by zero, and
    `to_usize` on a value the fixed-width layer cannot hold. None of the
    three is a bound on the length of a message -- `pad10*1` is the only
    caller of `sub`, and reduces `m` modulo `x` first; the divisor is always
    the rate; and `to_usize` is applied to the rate and the width. `add` and
    `mul` are total, and that is the point of the type: `8 * len(M)` and
    `len(N) + len(pad)` are unconditional, so no proof about this
    specification carries a precondition on how long its input may be. -/

def nat.Nat.Insts.CoreCloneClone.clone (n : nat.Nat) : RustM nat.Nat := ok n

/-- `Nat::new(x)`: a literal, widened from the `u64` the Standard's constants
    fit in. -/
def nat.Nat.new (x : Std.U64) : RustM nat.Nat := ok x.val

/-- `Nat::from_usize(x)`: a count the machine made — a byte count, a slice
    length, the width `b`. -/
def nat.Nat.from_usize (x : Std.Usize) : RustM nat.Nat := ok x.val

/-- `Nat::to_usize`: back down to the fixed-width layer, and partial there. -/
def nat.Nat.to_usize (n : nat.Nat) : RustM Std.Usize := Std.UScalar.tryMk .Usize n

def nat.Nat.Insts.CoreOpsArithAddNatNat.add (a b : nat.Nat) : RustM nat.Nat :=
  ok (a + b)

/-- Partial in the model as in the mathematics: the nonnegative integers are
    not closed under subtraction. Not `Nat`'s own truncating `-`, which would
    succeed where the `u128` panics. -/
def nat.Nat.Insts.CoreOpsArithSubNatNat.sub (a b : nat.Nat) : RustM nat.Nat :=
  if b ≤ a then ok (a - b) else fail .panic

def nat.Nat.Insts.CoreOpsArithMulNatNat.mul (a b : nat.Nat) : RustM nat.Nat :=
  ok (a * b)

/-- Truncating division, partial at zero. Not `Nat`'s own `/`, which returns
    zero there. -/
def nat.Nat.Insts.CoreOpsArithDivNatNat.div (a b : nat.Nat) : RustM nat.Nat :=
  if b = 0 then fail .panic else ok (a / b)

/-- `a mod b`, partial at zero for the same reason. -/
def nat.Nat.Insts.CoreOpsArithRemNatNat.rem (a b : nat.Nat) : RustM nat.Nat :=
  if b = 0 then fail .panic else ok (a % b)

def nat.Nat.Insts.CoreCmpPartialEqNat.eq (a b : nat.Nat) : RustM Bool :=
  ok (a == b)

def nat.Nat.Insts.CoreCmpPartialEqNat.ne (a b : nat.Nat) : RustM Bool :=
  ok (a != b)

def nat.Nat.Insts.CoreCmpPartialOrdNat.lt (a b : nat.Nat) : RustM Bool :=
  ok (decide (a < b))

def nat.Nat.Insts.CoreCmpPartialOrdNat.le (a b : nat.Nat) : RustM Bool :=
  ok (decide (a ≤ b))

def nat.Nat.Insts.CoreCmpPartialOrdNat.gt (a b : nat.Nat) : RustM Bool :=
  ok (decide (b < a))

def nat.Nat.Insts.CoreCmpPartialOrdNat.ge (a b : nat.Nat) : RustM Bool :=
  ok (decide (b ≤ a))

/-- Total: a `PartialOrd` that is an `Ord`, so `partial_cmp` is always
    `Some`. Unused by the specification, which compares with `<` and `≤`. -/
def nat.Nat.Insts.CoreCmpPartialOrdNat.partial_cmp (a b : nat.Nat) :
    RustM (core.option.Option core.cmp.Ordering) :=
  ok (core.option.Option.Some
       (if a < b then core.cmp.Ordering.Less
        else if a = b then core.cmp.Ordering.Equal
        else core.cmp.Ordering.Greater))

end hacspec_sha3_pedantic
