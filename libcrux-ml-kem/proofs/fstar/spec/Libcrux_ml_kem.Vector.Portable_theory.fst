module Libcrux_ml_kem.Vector.Portable_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

(* Hand-written proof theory relocated from src/vector/portable.rs
   `hax_lib::fstar::before` blocks (byte-exact raw-string contents, verified
   verbatim against the green extracted module). Consumed only by that module. *)

(* Clean-context bridge: the inner `decompress_1`'s per-lane `{0, 1665}`
   disjunction to the opaque `[0, 3328]` bound atom that `decompress_1_post`
   carries.  Standalone top-level lemma so the
   literal range checks + the opaque-atom intro stay out of `op_decompress_1`'s
   heavy VC context (inline, even the trivial `mk_i16 3328` range sub-query
   saturates at rlimit 200 under `--split_queries always`). *)
let lemma_decompress_1_bound (x: t_Array i16 (mk_usize 16))
    : Lemma
      (requires
        forall (i: nat). i < 16 ==> (v (Seq.index x i) == 0 \/ v (Seq.index x i) == 1665))
      (ensures Libcrux_ml_kem.Vector.Traits.Spec.bounded_i16_array (mk_i16 0) (mk_i16 3328) x) =
  Libcrux_ml_kem.Vector.Traits.Spec.lemma_bounded_i16_array_intro (mk_i16 0)
    (mk_i16 3328)
    x
