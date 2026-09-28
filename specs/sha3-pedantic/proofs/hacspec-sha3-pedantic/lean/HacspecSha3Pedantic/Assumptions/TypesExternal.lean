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

    `len` is total: it hands back a `nat.Nat`, which is this same unbounded
    reading one level down, so counting the bits of a string is an operation
    that cannot fail and no proof against this specification carries a bound
    on the length of its input. -/
-- `abbrev`, not `def`: the model has to be reducible for `List`'s own
-- instances (`++`, `getElem!`, `Inhabited`) to apply to it.
abbrev bits.BitStr : Type := List Bool

/-! ## `Nat`, the nonnegative integers

    `nat::Nat` is opaque for the same reason `BitStr` is, one level down: the
    Standard's `len(M)`, its `d` and the `j` of Algorithm 9 are nonnegative
    integers with no upper bound, and a machine word in their place puts a
    bound on the input into the specification, where the Standard has none.
    The model is therefore Lean's `Nat`, and the operations in
    `FunsExternal.lean` are total except where the mathematics itself is
    partial (`a - b` for `b > a`, division by zero) or where a value is handed
    back down to the fixed-width layer (`to_usize`).

    The Rust implementation is the partial one, as it is for `BitStr`: a
    `u128`, which stops at `2^128` where this model does not stop at all.
    That difference is unreachable rather than merely unlikely, and the
    argument is the allocator's: the bits counted here live in a `Vec<u8>` of
    at most `isize::MAX` bytes, so no `BitStr` that can be built has as many
    as `2^67` bits, `pad10*1` adds under `2^11` more, and the sponge's cursor
    stays below the length of the padded string. `nat.rs` carries the same
    argument in full, and writes every operation that could overflow as a
    `checked_*` that panics rather than wraps. -/
-- `abbrev`, not `def`, for the same reason as `BitStr` above: `Nat`'s own
-- instances (`+`, `<`, numerals) have to apply to it.
abbrev nat.Nat : Type := _root_.Nat

end hacspec_sha3_pedantic
