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

/-- [hacspec_sha3_pedantic::state_array::{impl core::fmt::Debug for hacspec_sha3_pedantic::state_array::StateArray<W>}::fmt]:
    Source: 'sha3-pedantic/src/state_array.rs', lines 13:37-13:42
    Visibility: public -/
axiom state_array.StateArray.Insts.CoreFmtDebug.fmt
  {W : Std.Usize} :
  state_array.StateArray W → core.fmt.Formatter → RustM
    ((core.result.Result Unit core.fmt.Error) × core.fmt.Formatter ×
    (core.fmt.Formatter → core.fmt.Formatter))

