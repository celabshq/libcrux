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

    Only `len` and `to_bits` can fail, and only where a machine would: `len`
    when the length does not fit a `u64`, `to_bits` when it does not fit a
    `usize`. Neither is reachable from the specification's own call sites --
    `to_bits` is applied to `r`-bit blocks with `r < 1600`. -/

namespace hacspec_sha3_pedantic

/-- The bits of a byte, least significant first — App. B.1 reads a byte this
    way, and `BitStr::from_bytes` packs it this way. -/
def bits.bitsOfByte (b : Std.U8) : List Bool :=
  (List.range 8).map (fun j => b.bv.getLsbD j)

/-- `h2b(H)` at its maximum `n`: the `8m` bits of the bytes `H`. -/
def bits.bitsOfBytes (bs : List Std.U8) : List Bool :=
  bs.flatMap bits.bitsOfByte

/-- One byte from eight bits, least significant first; short groups are padded
    with zeros, which is the `S || 0^(-n mod 8)` of Algorithm 11. -/
def bits.byteOfBits (l : List Bool) : Std.U8 :=
  ⟨BitVec.ofNat 8 ((List.range 8).foldl
    (fun acc j => if l.getD j false then acc + 2 ^ j else acc) 0)⟩

/-- `b2h(S)` — Algorithm 11. -/
def bits.bytesOfBits (l : List Bool) : List Std.U8 :=
  (List.range ((l.length + 7) / 8)).map
    (fun i => bits.byteOfBits ((l.drop (8 * i)).take 8))

def bits.BitStr.Insts.CoreCloneClone.clone (s : bits.BitStr) : RustM bits.BitStr :=
  ok s

def bits.BitStr.empty : RustM bits.BitStr := ok ([] : List Bool)

/-- `len(S)` — Sec. 2.3. Fails where the length outruns a `u64`, which is what
    the Rust `len` field can hold; the model itself has no bound. -/
def bits.BitStr.len (s : bits.BitStr) : RustM Std.U64 :=
  if (s : List Bool).length < 2 ^ 64 then
    ok ⟨BitVec.ofNat 64 (s : List Bool).length⟩
  else fail .panic

def bits.BitStr.is_empty (s : bits.BitStr) : RustM Bool :=
  ok ((s : List Bool).isEmpty)

/-- `S[i]` — Sec. 2.3. -/
def bits.BitStr.bit (s : bits.BitStr) (i : Std.U64) : RustM Bool :=
  if i.val < (s : List Bool).length then ok ((s : List Bool)[i.val]!) else fail .panic

/-- Private in Rust, and unused by the specification: the pure operations are
    the ones the Standard names. Modelled for completeness. -/
def bits.BitStr.push_mut (s : bits.BitStr) (b : Bool) : RustM bits.BitStr :=
  ok ((s : List Bool) ++ [b])

/-- `0^n` — Sec. 2.3. -/
def bits.BitStr.zeros (n : Std.U64) : RustM bits.BitStr :=
  ok (List.replicate n.val false)

def bits.BitStr.from_bits (b : Slice Bool) : RustM bits.BitStr := ok b.val

/-- Fails where the length outruns a `usize`; applied only to `r`-bit blocks. -/
def bits.BitStr.to_bits (s : bits.BitStr) : RustM (alloc.vec.Vec Bool) :=
  if h : (s : List Bool).length ≤ Std.Usize.max then ok ⟨(s : List Bool), h⟩
  else fail .panic

/-- `X || Y` — Sec. 2.3. -/
def bits.BitStr.concat (x : bits.BitStr) (y : bits.BitStr) : RustM bits.BitStr :=
  ok ((x : List Bool) ++ (y : List Bool))

/-- `Trunc_s(X)` — Sec. 2.3. -/
def bits.BitStr.trunc (x : bits.BitStr) (s : Std.U64) : RustM bits.BitStr :=
  if s.val ≤ (x : List Bool).length then ok ((x : List Bool).take s.val)
  else fail .panic

/-- The `n` bits of `X` from `from` — the blocks `P_i` of Algorithm 8. -/
def bits.BitStr.slice (x : bits.BitStr) (from_ : Std.U64) (n : Std.U64) :
    RustM bits.BitStr :=
  if from_.val + n.val ≤ (x : List Bool).length then
    ok (((x : List Bool).drop from_.val).take n.val)
  else fail .panic

/-- Algorithm 10 at its maximum `n`. -/
def bits.BitStr.from_bytes (h : Slice Std.U8) : RustM bits.BitStr :=
  ok (bits.bitsOfBytes h.val)

/-- Algorithm 11. -/
def bits.BitStr.to_bytes (s : bits.BitStr) : RustM (alloc.vec.Vec Std.U8) :=
  let bs := bits.bytesOfBits (s : List Bool)
  if h : bs.length ≤ Std.Usize.max then ok ⟨bs, h⟩ else fail .panic

end hacspec_sha3_pedantic
