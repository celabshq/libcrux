module Hacspec_ml_kem.Commute.Hash_reveal

open Core_models
open FStar.Mul

module SU = Spec.Utils
module SP = Hacspec_sha3.Sponge

/// `friend Spec.Utils` exposes the concrete `let v_* = keccak ...` bodies here
/// only; consumers see just this module's abstract interface.
friend Spec.Utils

#set-options "--fuel 0 --ifuel 1 --z3rlimit 40"

let lemma_v_G_eq input = ()
let lemma_v_H_eq input = ()
let lemma_v_PRF_eq len input = ()
let lemma_v_J_eq input = ()

(* v_PRFxN = map_array (fun row -> keccak len 136 31 row); createi index (SMTPat
   `lemma_createi_index`) gives the i-th entry = keccak len 136 31 input[i] = v_PRF. *)
let lemma_v_PRFxN_pointwise r len input i = ()
