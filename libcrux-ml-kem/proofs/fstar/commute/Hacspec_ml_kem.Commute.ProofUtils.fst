module Hacspec_ml_kem.Commute.ProofUtils
#set-options "--fuel 0 --ifuel 1 --z3rlimit 20"
open FStar.Mul
open Core_models

(* Generic F* proof utilities used by libcrux-ml-kem trait-post specs
   and bridge lemmas.  No ml-kem-specific content; candidates for
   upstream to hax-lib's F* proof-lib once a stable home for generic
   array helpers exists.

   TODO(upstream-to-hax-lib): move `map_array` (and any future generic
   helpers added here) into hax-lib once the proof-lib gains a
   generic-array-helpers module.  Until then, this commute submodule
   acts as the staging area. *)

(* Pointwise array map.  Used by `Libcrux_ml_kem.Vector.Traits.Spec`'s
   bitwise / shift trait-post specs (`bitwise_and_with_constant_constant_post`,
   `shift_right_post`, etc.) to express `result == map_array f vec`. *)
let map_array (#a #b: Type) (#len: usize)
    (f: a -> b)
    (s: t_Array a len)
    : t_Array b len
    = createi (length s) (fun i -> f (Seq.index s (v i)))
