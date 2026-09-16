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
