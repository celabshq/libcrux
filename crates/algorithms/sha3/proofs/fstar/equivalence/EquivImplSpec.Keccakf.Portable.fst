module EquivImplSpec.Keccakf.Portable

(* ================================================================
   Portable (N=1, v_T=u64) instantiation of the generic keccak_f
   equivalence proof.

   This module constructs the 7-field [lane_correctness] record for
   [Libcrux_sha3.Simd.Portable.impl] and derives the concrete theorem

     (keccakf1600 ks).f_st == keccak_f ks.f_st

   directly from [EquivImplSpec.Keccakf.Generic.lemma_keccakf1600_to_spec]
   by exploiting that [extract_lane] is the identity when v_N = 1.

   All 7 [lc_*] lemmas are trivial (= ()) because the portable
   [KeccakItem u64 1] instance's methods are definitionally the scalar
   u64 operations the spec uses.
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"

open FStar.Mul
open Core_models

module G = EquivImplSpec.Keccakf.Generic

(* Bring the Portable typeclass instance into scope so
   t_KeccakItem u64 (mk_usize 1) resolves to Libcrux_sha3.Simd.Portable.impl. *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()

(* ================================================================
   Portable lane extraction

   For N=1, a SIMD element of type u64 has a single lane which is
   the element itself.
   ================================================================ *)

let portable_lane (x: u64) (l: nat{l < 1}) : u64 = x

(* ================================================================
   Lane-correctness field proofs

   Each lemma is `= ()` because the portable [f_*] method is defined
   in terms of [e_*] helpers that are themselves the scalar u64
   operation.
   ================================================================ *)

let portable_lc_zero (l: nat{l < 1})
  : Lemma (portable_lane (Libcrux_sha3.Traits.f_zero #u64 #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve ()) l == mk_u64 0)
  = ()

let portable_lc_xor5 (a b c d e: u64) (l: nat{l < 1})
  : Lemma (portable_lane (Libcrux_sha3.Traits.f_xor5 #u64 #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve a b c d e) l ==
           (((portable_lane a l ^. portable_lane b l) ^. portable_lane c l)
             ^. portable_lane d l) ^. portable_lane e l)
  = ()

let portable_lc_rotate_left1_and_xor (a b: u64) (l: nat{l < 1})
  : Lemma (portable_lane (Libcrux_sha3.Traits.f_rotate_left1_and_xor #u64
             #(mk_usize 1) #FStar.Tactics.Typeclasses.solve a b) l ==
           portable_lane a l ^.
           Core_models.Num.impl_u64__rotate_left (portable_lane b l) (mk_u32 1))
  = ()

let portable_lc_xor_and_rotate (v_LEFT v_RIGHT: i32) (a b: u64) (l: nat{l < 1})
  : Lemma
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) /\
        v_RIGHT >. mk_i32 0 /\
        v_RIGHT <. mk_i32 64)
      (ensures
        portable_lane (Libcrux_sha3.Traits.f_xor_and_rotate #u64 #(mk_usize 1)
          #FStar.Tactics.Typeclasses.solve v_LEFT v_RIGHT a b) l ==
        Core_models.Num.impl_u64__rotate_left
          (portable_lane a l ^. portable_lane b l) (cast (v_LEFT <: i32) <: u32))
  = ()

let portable_lc_and_not_xor (a b c: u64) (l: nat{l < 1})
  : Lemma (portable_lane (Libcrux_sha3.Traits.f_and_not_xor #u64 #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve a b c) l ==
           portable_lane a l ^. (portable_lane b l &. (~. (portable_lane c l))))
  = ()

let portable_lc_xor_constant (a: u64) (c: u64) (l: nat{l < 1})
  : Lemma (portable_lane (Libcrux_sha3.Traits.f_xor_constant #u64 #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve a c) l ==
           portable_lane a l ^. c)
  = ()

let portable_lc_xor (a b: u64) (l: nat{l < 1})
  : Lemma (portable_lane (Libcrux_sha3.Traits.f_xor #u64 #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve a b) l ==
           portable_lane a l ^. portable_lane b l)
  = ()

(* ================================================================
   Assemble the [lane_correctness] record
   ================================================================ *)

let lc_portable : G.lane_correctness (mk_usize 1) #u64 =
  {
    lane = portable_lane;
    lc_zero = portable_lc_zero;
    lc_xor5 = portable_lc_xor5;
    lc_rotate_left1_and_xor = portable_lc_rotate_left1_and_xor;
    lc_xor_and_rotate = portable_lc_xor_and_rotate;
    lc_and_not_xor = portable_lc_and_not_xor;
    lc_xor_constant = portable_lc_xor_constant;
    lc_xor = portable_lc_xor;
  }

(* ================================================================
   For N=1, [extract_lane] is the identity on the state array.
   ================================================================ *)

let lemma_extract_lane_portable_identity
      (state: t_Array u64 (mk_usize 25))
  : Lemma (G.extract_lane (mk_usize 1) lc_portable state 0 == state)
  = let lhs = G.extract_lane (mk_usize 1) lc_portable state 0 in
    let aux (i: nat{i < 25}) : Lemma (Seq.index lhs i == Seq.index state i) =
      let k: usize = mk_usize i in
      assert (v k == i);
      G.lemma_extract_lane_index (mk_usize 1) lc_portable state 0 k;
      assert (lhs.[k] == lc_portable.lane state.[k] 0);
      assert (lc_portable.lane state.[k] 0 == state.[k])
    in
    Classical.forall_intro aux;
    Rust_primitives.Arrays.eq_intro lhs state

(* ================================================================
   MAIN THEOREM: portable keccakf1600 ≡ spec keccak_f

   Derived from [lemma_keccakf1600_to_spec] at v_N = 1, lane = 0,
   using the identity lemma above to collapse [extract_lane] on both
   sides.
   ================================================================ *)

let lemma_keccakf1600_portable
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
  : Lemma
      ((Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 ks)
         .Libcrux_sha3.Generic_keccak.f_st ==
       Hacspec_sha3.Keccak_f.keccak_f ks.Libcrux_sha3.Generic_keccak.f_st)
  = let state = ks.Libcrux_sha3.Generic_keccak.f_st in
    let ks_out = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 1) #u64 ks in
    let out_state = ks_out.Libcrux_sha3.Generic_keccak.f_st in
    G.lemma_keccakf1600_to_spec (mk_usize 1) lc_portable ks 0;
    lemma_extract_lane_portable_identity state;
    lemma_extract_lane_portable_identity out_state
