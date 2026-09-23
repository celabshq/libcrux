
-- Experimental lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
import Utilities
import libcrux_aes

open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false

namespace libcrux_aes.platform.portable.aes_core
def xor_key1_state_spec (st : (Vector u16 8)) (k : (Vector u16 8)) : (Vector u16 8) :=
  (Vector.zip st k).map (fun (x, y) => x ^^^ y)

end libcrux_aes.platform.portable.aes_core