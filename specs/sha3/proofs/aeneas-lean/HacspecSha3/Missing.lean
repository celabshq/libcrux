import Aeneas
import CoreModels

open Aeneas Aeneas.Std Result

noncomputable section

namespace CoreModels.core

/-! Generic `Index`/`IndexMut` wrappers that the `-core-models-lib` extraction
expects. They take an Aeneas `core.slice.index.SliceIndex` instance and delegate
to Aeneas's native implementations. -/

def Slice.Insts.CoreOpsIndexIndex
  {T I O : Type} (inst : Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O) :
  Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O := inst

def Slice.Insts.CoreOpsIndexIndexMut
  {T I O : Type} (inst : Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O) :
  Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O := inst

def Slice.Insts.CoreOpsIndexIndex.index
  {T I O : Type}
  (inst : Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O)
  (s : Slice T) (i : I) : Result O :=
  Aeneas.Std.core.slice.index.Slice.index inst s i

def Slice.Insts.CoreOpsIndexIndexMut.index_mut
  {T I O : Type}
  (inst : Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O)
  (s : Slice T) (i : I) : Result (O × (O → Slice T)) :=
  Aeneas.Std.core.slice.index.Slice.index_mut inst s i

def Array.Insts.CoreOpsIndexIndex.index
  {T I O : Type} {N : Usize}
  (inst : Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O)
  (arr : Array T N) (i : I) : Result O :=
  Aeneas.Std.core.slice.index.Slice.index inst (Array.to_slice arr) i

def Array.Insts.CoreOpsIndexIndexMut.index_mut
  {T I O : Type} {N : Usize}
  (inst : Aeneas.Std.core.slice.index.SliceIndex I (Slice T) O)
  (arr : Array T N) (i : I) : Result (O × (O → Array T N)) := do
  let (s, to_arr) := Array.to_slice_mut arr
  let (out, to_slice) ← Aeneas.Std.core.slice.index.Slice.index_mut inst s i
  ok (out, fun o => to_arr (to_slice o))

/-! Bridges from `RangeUsize` / `RangeFromUsize` to Aeneas's native
`SliceIndex` instances. -/

def ops.range.RangeUsize.Insts.CoreSliceIndexSliceIndexSliceSlice (T : Type) :
    Aeneas.Std.core.slice.index.SliceIndex
      (CoreModels.core.ops.range.Range Usize) (Slice T) (Slice T) :=
  let toAeneas (r : CoreModels.core.ops.range.Range Usize) :
      Aeneas.Std.core.ops.range.Range Usize :=
    { start := r.start, «end» := r.«end» }
  { sealedInst := {}
    get := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.get (toAeneas r) s
    get_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.get_mut (toAeneas r) s
    get_unchecked := fun _ _ => Result.fail Error.undef
    get_unchecked_mut := fun _ _ => Result.fail Error.undef
    index := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.index (toAeneas r) s
    index_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.index_mut (toAeneas r) s }

def ops.range.RangeFromUsize.Insts.CoreSliceIndexSliceIndexSliceSlice (T : Type) :
    Aeneas.Std.core.slice.index.SliceIndex
      (CoreModels.core.ops.range.RangeFrom Usize) (Slice T) (Slice T) :=
  let toAeneas (r : CoreModels.core.ops.range.RangeFrom Usize)
      (s : Slice T) : Aeneas.Std.core.ops.range.Range Usize :=
    { start := r.start, «end» := Slice.len s }
  { sealedInst := {}
    get := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.get (toAeneas r s) s
    get_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.get_mut (toAeneas r s) s
    get_unchecked := fun _ _ => Result.fail Error.undef
    get_unchecked_mut := fun _ _ => Result.fail Error.undef
    index := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.index (toAeneas r s) s
    index_mut := fun r s =>
      Aeneas.Std.core.slice.index.SliceIndexRangeUsizeSlice.index_mut (toAeneas r s) s }

/-! `Result.unwrap` model — converts the model `result.Result T E` into an
Aeneas `Result T`, panicking on `Err`. -/

def result.Result.unwrap
    {T E : Type} (_dbg : CoreModels.core.fmt.Debug E)
    (r : CoreModels.core.result.Result T E) : Aeneas.Std.Result T :=
  match r with
  | .Ok x => Aeneas.Std.Result.ok x
  | .Err _ => Aeneas.Std.Result.fail Aeneas.Std.Error.panic

/-! `copy_from_slice` — panics on length mismatch. -/

def slice.Slice.copy_from_slice
    {T : Type} (_cpy : CoreModels.core.marker.Copy T)
    (dst : Aeneas.Std.Slice T) (src : Aeneas.Std.Slice T) :
    Aeneas.Std.Result (Aeneas.Std.Slice T) :=
  if Aeneas.Std.Slice.len dst = Aeneas.Std.Slice.len src then
    Aeneas.Std.Result.ok src
  else Aeneas.Std.Result.fail Aeneas.Std.Error.panic

/-! Trivial `Debug` instance for `array.TryFromSliceError` (= `Unit`). -/

@[reducible]
def array.TryFromSliceError.Insts.CoreFmtDebug :
    CoreModels.core.fmt.Debug CoreModels.core.array.TryFromSliceError :=
  { dbg_fmt := fun _ f => Aeneas.Std.Result.ok (.Ok (), f) }

end CoreModels.core

end
