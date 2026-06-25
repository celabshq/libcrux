module Hacspec_ml_kem.Parameters.Sizes
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

(* Parameterized (rank-indexed) size constants for ML-KEM.  Bodies are
   concrete (NOT Spec.MLKEM wrappers) and reference the file-scope
   constants from extracted `Hacspec_ml_kem.Parameters`
   (`v_BYTES_PER_RING_ELEMENT`, etc.).  `is_rank`/`rank` are defined
   here so consumers don't need Spec.MLKEM.Math.

   Documented exception to R10 (no new namespace-squatting):
   `Parameters.Sizes` predates the Hacspec namespace policy; its bodies
   are real definitions (not unfold-let wrappers).  Future migration
   should replace `Hacspec_ml_kem.Parameters.Sizes.v_*` consumers with
   the extracted `Hacspec_ml_kem.Parameters.{t_MlKemParams,
   impl_MlKemParams__*, v_ML_KEM_*}` symbols at which point this file
   can be deleted.  Until then: do not extend it. *)

module P = Hacspec_ml_kem.Parameters

let is_rank (r:usize) : Type0 = r == sz 2 \/ r == sz 3 \/ r == sz 4

type rank = r:usize{is_rank r}

(*** Top-5 Spec.MLKEM constants, rank-parameterized ***)

val v_ETA1 (r:rank) : u:usize{u == sz 3 \/ u == sz 2}
let v_ETA1 (r:rank) : usize =
  if r = sz 2 then sz 3 else sz 2

val v_ETA1_RANDOMNESS_SIZE (r:rank) : u:usize{u == sz 128 \/ u == sz 192}
let v_ETA1_RANDOMNESS_SIZE (r:rank) : usize = v_ETA1 r *! sz 64

val v_RANKED_BYTES_PER_RING_ELEMENT (r:rank) : u:usize{u = sz 768 \/ u = sz 1152 \/ u = sz 1536}
let v_RANKED_BYTES_PER_RING_ELEMENT (r:rank) : usize = r *! P.v_BYTES_PER_RING_ELEMENT

val v_CPA_PUBLIC_KEY_SIZE (r:rank) : u:usize{u = sz 800 \/ u = sz 1184 \/ u = sz 1568}
let v_CPA_PUBLIC_KEY_SIZE (r:rank) : usize =
  v_RANKED_BYTES_PER_RING_ELEMENT r +! sz 32

let v_CPA_PRIVATE_KEY_SIZE (r:rank) : usize = v_RANKED_BYTES_PER_RING_ELEMENT r

val v_CCA_PRIVATE_KEY_SIZE (r:rank) : u:usize{u = sz 1632 \/ u = sz 2400 \/ u = sz 3168}
let v_CCA_PRIVATE_KEY_SIZE (r:rank) : usize =
  v_CPA_PRIVATE_KEY_SIZE r +! v_CPA_PUBLIC_KEY_SIZE r +! sz 32 +! sz 32
