-- External helpers for `hacspec_sha3` (hand-written). CoreModels supplies
-- every model the extraction references; `HaxToRange` is a proof-side helper
-- for recovering concrete `[start, end)` bounds from CoreModels range values.
import Aeneas
import CoreModels
import HacspecSha3.Extraction.Types
open CoreModels Aeneas
open Aeneas.Std hiding namespace core alloc
open RustM ControlFlow Error
open Std.Do
set_option linter.dupNamespace false
set_option linter.hashCommand false
set_option linter.unusedVariables false
set_option maxHeartbeats 1000000
set_option maxRecDepth 2048
open hacspec_sha3

noncomputable section

/-- Recover concrete `[start, end)` bounds (as an aeneas `Range`) from the
    `CoreModels` range value used to index; `len` is the length of the indexed
    container, used to close open-ended ranges. -/
class HaxToRange (I : Type) where
  toRange : I → Usize → Aeneas.Std.core.ops.range.Range Usize

instance : HaxToRange (CoreModels.core.ops.range.Range Usize) where
  toRange r _ := { start := r.start, «end» := r.«end» }
instance : HaxToRange (CoreModels.core.ops.range.RangeFrom Usize) where
  toRange r len := { start := r.start, «end» := len }
instance : HaxToRange (CoreModels.core.ops.range.RangeTo Usize) where
  toRange r _ := { start := 0#usize, «end» := r.«end» }
-- `x[..]`: `RangeFull` is `Unit`, so the whole container is the range.
-- Not used by the spec's own extraction.
instance : HaxToRange CoreModels.core.ops.range.RangeFull where
  toRange _ len := { start := 0#usize, «end» := len }

end
