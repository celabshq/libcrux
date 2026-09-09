-- External helpers for `hacspec_sha3` (hand-written).
-- As of hax-lean v0.3.17 (hax v0.4.0-rc.2), CoreModels supplies every model the
-- extraction references, including the `Array` `index_mut` and the
-- `TryFromSliceError` `Debug` instance that used to be defined here. What
-- remains is the `HaxToRange` bridge, a PROOF-side helper for recovering
-- concrete `[start, end)` bounds from CoreModels range values; it is reused by
-- libcrux-iot's proofs via `import HacspecSha3`.
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
-- Needed by consumers of this spec (libcrux-iot sha3 indexes with `..`), not by
-- the spec's own extraction.
instance : HaxToRange CoreModels.core.ops.range.RangeFull where
  toRange _ len := { start := 0#usize, «end» := len }

-- The `Array` `index_mut` model and the `TryFromSliceError` `Debug` instance
-- that used to be hand-written here are supplied by CoreModels natively as of
-- hax-lean v0.3.17 (hax v0.4.0-rc.2): `Array.Insts.CoreOpsIndexIndexMut.index_mut`
-- now routes through `array.Array.as_mut_slice` and composes the write-backs,
-- with no `HaxToRange`/`update_subslice` detour. Redeclaring them at the same
-- names is an error ("has already been declared"), so they are gone; only the
-- `HaxToRange` helper class above remains, because downstream proofs
-- (libcrux-iot sha3) still use it to STATE range facts.

end
