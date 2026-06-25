module Libcrux_sha3.Portable.Incremental.Private
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

include Libcrux_sha3.Portable.Incremental.Bundle {t_Sealed as t_Sealed}
