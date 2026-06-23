/- # Hand-written stubs for symbols the current rust-core-models doesn't define -/
import Aeneas
import CoreModels
import HacspecSha3

open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open Result

noncomputable section

namespace CoreModels.core

/-- `RangeTo Usize → RangeTo Usize` identity bridge. -/
def cmRangeToUsizeToAeneas (r : ops.range.RangeTo Aeneas.Std.Usize) :
    Aeneas.Std.core.ops.range.RangeTo Aeneas.Std.Usize :=
  { «end» := r.«end» }

/-- `SliceIndex` for `RangeTo Usize` (`s[..end]`). Forwards to
    Aeneas's `SliceIndexRangeToUsizeSlice.*`. -/
@[reducible] def ops.range.RangeToUsize.Insts.CoreSliceIndexSliceIndexSliceSlice
  (T : Type) : Aeneas.Std.core.slice.index.SliceIndex
    (ops.range.RangeTo Aeneas.Std.Usize) (Aeneas.Std.Slice T) (Aeneas.Std.Slice T) :=
  { sealedInst := {}
    get := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeToUsizeSlice.get (cmRangeToUsizeToAeneas r) s
    get_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeToUsizeSlice.get_mut (cmRangeToUsizeToAeneas r) s
    get_unchecked := fun _ _ => Aeneas.Std.Result.fail Aeneas.Std.Error.undef
    get_unchecked_mut := fun _ _ => Aeneas.Std.Result.fail Aeneas.Std.Error.undef
    index := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeToUsizeSlice.index (cmRangeToUsizeToAeneas r) s
    index_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeToUsizeSlice.index_mut (cmRangeToUsizeToAeneas r) s }

/-- `SliceIndex` for `RangeFull` (`s[..]`). The full slice is just `s`. -/
def cmRangeFullToAeneas {T : Type} (_r : ops.range.RangeFull)
    (s : Aeneas.Std.Slice T) :
    Aeneas.Std.core.ops.range.Range Aeneas.Std.Usize :=
  { start := 0#usize, «end» := Aeneas.Std.Slice.len s }

@[reducible] def ops.range.RangeFull.Insts.CoreSliceIndexSliceIndexSliceSlice
  (T : Type) : Aeneas.Std.core.slice.index.SliceIndex
    ops.range.RangeFull (Aeneas.Std.Slice T) (Aeneas.Std.Slice T) :=
  { sealedInst := {}
    get := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.get (cmRangeFullToAeneas r s) s
    get_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.get_mut (cmRangeFullToAeneas r s) s
    get_unchecked := fun _ _ => Aeneas.Std.Result.fail Aeneas.Std.Error.undef
    get_unchecked_mut := fun _ _ => Aeneas.Std.Result.fail Aeneas.Std.Error.undef
    index := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.index (cmRangeFullToAeneas r s) s
    index_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.index_mut (cmRangeFullToAeneas r s) s }

/-- `TryFrom<&[T]>` for `[T; N]`. -/
def SharedAArray.Insts.CoreConvertTryFromSharedASliceTryFromSliceError.try_from
    {T : Type} (N : Aeneas.Std.Usize) (s : Aeneas.Std.Slice T) :
    Aeneas.Std.Result (result.Result (Aeneas.Std.Array T N) array.TryFromSliceError) :=
  if h: s.len = N then
    Aeneas.Std.Result.ok (result.Result.Ok ⟨s.val, by scalar_tac⟩)
  else
    Aeneas.Std.Result.ok (result.Result.Err ())

/-- `Try::branch` for `Result<T, E>` (residual = `Result<Infallible, E>`).
    The `?`-operator desugar — branch is the "is this Ok or Err" split. -/
def result.Result.Insts.CoreOpsTry_traitTry.branch
    {T E : Type} (r : result.Result T E) :
    Aeneas.Std.Result (ops.control_flow.ControlFlow (result.Result convert.Infallible E) T) :=
  match r with
  | .Ok x => Aeneas.Std.Result.ok (.Continue x)
  | .Err e => Aeneas.Std.Result.ok (.Break (.Err e))

/-- `FromResidual::from_residual` for `Result<T, F>` from residual
    `Result<Infallible, E>` via `F: From<E>`. The other half of the
    `?`-operator desugar — lifts a Residual back into the carrier
    monad through the `From` instance. -/
def result.Result.Insts.CoreOpsTry_traitFromResidualResultInfallibleE.from_residual
    {E F : Type} (T : Type) (FromInst : convert.From F E)
    (residual : result.Result convert.Infallible E) :
    Aeneas.Std.Result (result.Result T F) :=
  match residual with
  | .Err e => do
    let f ← FromInst.«from» e
    Aeneas.Std.Result.ok (.Err f)
  | .Ok _ => Aeneas.Std.Result.fail Aeneas.Std.Error.panic  -- Infallible has no inhabitants

/-- `Slice::split_at` — routes to Aeneas's `core.slice.Slice.split_at`. -/
def slice.Slice.split_at {T : Type} (s : Aeneas.Std.Slice T) (mid : Aeneas.Std.Usize) :
    Aeneas.Std.Result (Aeneas.Std.Slice T × Aeneas.Std.Slice T) :=
  Aeneas.Std.core.slice.Slice.split_at s mid

/-- `PartialEq<Bool>` for `Bool`. -/
instance Bool.Insts.CoreCmpPartialEqBool : cmp.PartialEq Bool Bool :=
  { eq := fun x y => Aeneas.Std.Result.ok (x == y) }

/-- `Ord` for `U16`. -/
instance U16.Insts.CoreCmpOrd : cmp.Ord Aeneas.Std.U16 :=
  { EqInst := { PartialEqInst := U16.Insts.CoreCmpPartialEqU16 }
    PartialOrdInst := U16.Insts.CoreCmpPartialOrdU16
    cmp := fun x y => Aeneas.Std.Result.ok
      (match compare x.val y.val with
       | .lt => cmp.Ordering.Less
       | .eq => cmp.Ordering.Equal
       | .gt => cmp.Ordering.Greater) }

/-- `PartialEq` not-equal for `&A` vs `&B`. Forwards to the underlying
    `PartialEq A B` instance. -/
def Shared1A.Insts.CoreCmpPartialEqShared0B.ne
    {A B : Type} (inst : cmp.PartialEq A B) (a : A) (b : B) :
    Aeneas.Std.Result Bool := do
  let eq ← inst.eq a b
  Aeneas.Std.Result.ok (!eq)

/-- `PartialEq Slice<T> Slice<T>` from elementwise `PartialEq T T`. -/
def Slice.Insts.CoreCmpPartialEqSlice
    {T : Type} (inst : cmp.PartialEq T T) :
    cmp.PartialEq (Aeneas.Std.Slice T) (Aeneas.Std.Slice T) :=
  { eq := fun a0 a1 =>
      if a0.length = a1.length then
        List.allM (fun (x, y) => inst.eq x y) (List.zip a0.val a1.val)
      else .ok false }

/-- `Array<T, N>::as_slice`. Routes to Aeneas's `Array.to_slice`. -/
def array.Array.as_slice {T : Type} {N : Aeneas.Std.Usize}
    (a : Aeneas.Std.Array T N) : Aeneas.Std.Result (Aeneas.Std.Slice T) :=
  Aeneas.Std.Result.ok (Aeneas.Std.Array.to_slice a)

/-- `Formatter.write_str` taking `Aeneas.Std.Str`. No-op body; the
    formatter state is `Unit` so writes carry no information. -/
def fmt.Formatter.write_str
  (f : fmt.Formatter) (_ : Aeneas.Std.Str) :
  Aeneas.Std.Result (result.Result Unit fmt.Error × fmt.Formatter) :=
  Aeneas.Std.Result.ok (.Ok (), f)

/-- `Formatter::debug_struct_field1_finish`. No-op stub returning Ok. -/
def fmt.Formatter.debug_struct_field1_finish
    {T : Type} (f : fmt.Formatter)
    (_name : Aeneas.Std.Slice Aeneas.Std.U8)
    (_field : Aeneas.Std.Slice Aeneas.Std.U8)
    (_value : T) :
    Aeneas.Std.Result ((result.Result Unit fmt.Error) × fmt.Formatter) :=
  Aeneas.Std.Result.ok (result.Result.Ok (), f)

/-- `slice.Slice.chunks_exact`. Builds the
    ChunksExact iterator state from a slice and chunk size. -/
def slice.Slice.chunks_exact {T : Type} (s : Aeneas.Std.Slice T)
    (chunk_size : Aeneas.Std.Usize) :
    Aeneas.Std.Result (slice.iter.ChunksExact T) :=
  Aeneas.Std.Result.ok { cs := chunk_size, elements := s }

/-- `slice.iter.ChunksExact.…SharedASlice.next`.
    Yields the first `cs` bytes paired with the rest, when at least `cs`
    bytes remain; otherwise returns `none` (and leaves iterator state). -/
def slice.iter.ChunksExact.Insts.CoreIterTraitsIteratorIteratorSharedASlice.next
    {T : Type} (it : slice.iter.ChunksExact T) :
    Aeneas.Std.Result ((option.Option (Aeneas.Std.Slice T)) ×
      (slice.iter.ChunksExact T)) := do
  let s := it.elements
  let cs := it.cs
  if cs.val ≤ s.val.length then
    let (s0, s1) ← Aeneas.Std.core.slice.Slice.split_at s cs
    Aeneas.Std.Result.ok (option.Option.Some s0,
      { cs := cs, elements := s1 })
  else
    Aeneas.Std.Result.ok (option.Option.None, it)

end CoreModels.core

end
