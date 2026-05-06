module Libcrux_sha3.Simd.Avx2
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let module_anchor (_: Prims.unit)
    : Prims.Pure Prims.unit
      Prims.l_True
      (ensures
        fun temp_0_ ->
          let _:Prims.unit = temp_0_ in
          true) = ()

include Libcrux_sha3.Simd.Avx2.Wrappers
include Libcrux_sha3.Simd.Avx2.Load
include Libcrux_sha3.Simd.Avx2.Store
