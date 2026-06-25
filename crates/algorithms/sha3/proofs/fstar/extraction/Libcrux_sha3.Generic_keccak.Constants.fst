module Libcrux_sha3.Generic_keccak.Constants
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let v_ROUNDCONSTANTS: t_Array u64 (mk_usize 24) =
  let list =
    [
      mk_u64 1; mk_u64 32898; mk_u64 9223372036854808714; mk_u64 9223372039002292224; mk_u64 32907;
      mk_u64 2147483649; mk_u64 9223372039002292353; mk_u64 9223372036854808585; mk_u64 138;
      mk_u64 136; mk_u64 2147516425; mk_u64 2147483658; mk_u64 2147516555;
      mk_u64 9223372036854775947; mk_u64 9223372036854808713; mk_u64 9223372036854808579;
      mk_u64 9223372036854808578; mk_u64 9223372036854775936; mk_u64 32778;
      mk_u64 9223372039002259466; mk_u64 9223372039002292353; mk_u64 9223372036854808704;
      mk_u64 2147483649; mk_u64 9223372039002292232
    ]
  in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 24);
  Rust_primitives.Hax.array_of_list 24 list
