-- [hacspec_sha3_pedantic]: external types.
-- Seeded by hax from Extraction/TypesExternal_Template.lean: fill the holes.
-- hax never modifies this file; after re-extraction, compare it against the
-- regenerated template to see what changed.
import Aeneas
import CoreModels
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


namespace hacspec_sha3_pedantic

/-! ## `BitStr`, the arbitrary-length bit string

    `bits::BitStr` is opaque to the extraction, so this is what the proofs
    reason about: a list of bits, of any length whatsoever. That is the object
    FIPS 202 talks about -- the Standard bounds `len(M)` nowhere -- and it is
    the whole reason the type is opaque. A transparent Rust type would extract
    as a `Slice`, whose model carries `length ≤ Usize.max`, and on a 32-bit
    target that is 512 MB of message.

    What is assumed here is the *correspondence*: that the bit-packed `Vec<u8>`
    in `bits.rs` implements these list operations. Nothing below is an axiom --
    they are ordinary definitions, so Lean's axiom set is untouched -- but the
    Rust side of the correspondence is not proved, and `bits.rs` carries a
    property test against a naive `Vec<bool>` reference for exactly that
    reason. See also the `RangeInclusive` caveat in `FunsExternal.lean`: this
    is the same kind of gap, and the same kind of mitigation.

    The Rust implementation is the partial one. Its length is a `u64`, so it
    stops at `2^64` bits where this model does not stop at all; the `len`
    model below fails there rather than wrapping, which is what a machine that
    cannot count its own string would do. Two exabytes is not a limit anyone
    will meet, and -- unlike the `usize` it replaced -- it does not move with
    the target. -/
-- `abbrev`, not `def`: the model has to be reducible for `List`'s own
-- instances (`++`, `getElem!`, `Inhabited`) to apply to it.
abbrev bits.BitStr : Type := List Bool

end hacspec_sha3_pedantic
