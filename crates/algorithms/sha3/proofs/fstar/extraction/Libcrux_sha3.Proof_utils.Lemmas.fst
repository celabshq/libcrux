module Libcrux_sha3.Proof_utils.Lemmas
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let lemma_div_mul_mod (a b: usize)
    : Lemma
        (requires b <>. mk_usize 0)
        (ensures (a /! b) *! b +! (a %! b) =. a)
    = ()

let rec lemma_mul_succ_le (k n d: usize)
  : Lemma
    (requires (v k) < (v n))
    (ensures (v k) * (v d) + (v d) <= (v n) * (v d))
    (decreases (v n)) =
  if v n = 0 then ()
  else if v k = v n - 1 then ()
  else lemma_mul_succ_le k (n -! mk_usize 1) d

(* `Core_models.Num.impl_u64__rotate_left` is a shifts+xor model
   (`if (n%64)=0 then x else (x<<!m)^.(x>>!(64-m))`) marked `[@@ "opaque_to_smt"]`
   (see hax#2235) — it is an ATOM for consumers (so e.g. Keccak `rho`, whose two
   sides are the SAME rotate application, keeps reflexivity instead of re-proving
   25 nonlinear shift+xor equalities).  Here we DO need the body, so we
   `reveal_opaque` it; the bit-wise `get_bit_shl`/`get_bit_shr`/xor SMTPats then
   discharge the per-bit equality on both sides (m = v_LEFT, 64-m = v_RIGHT), and
   `lemma_int_t_eq_via_bits` lifts it to value equality — no scaffold axiom. *)
#push-options "--fuel 1"
let lemma_shl_xor_shr_is_rotate_left (x: u64) (v_LEFT v_RIGHT: i32)
  : Lemma
      (requires
        v v_LEFT >= 0 /\ v v_LEFT < 64 /\
        v v_RIGHT > 0 /\ v v_RIGHT < 64 /\
        v v_LEFT + v v_RIGHT == 64)
      (ensures
        ((x <<! v_LEFT) ^. (x >>! v_RIGHT)) ==
        Core_models.Num.impl_u64__rotate_left x (cast (v_LEFT <: i32) <: u32))
  = reveal_opaque (`%Core_models.Num.impl_u64__rotate_left)
                  Core_models.Num.impl_u64__rotate_left;
    let lhs = (x <<! v_LEFT) ^. (x >>! v_RIGHT) in
    let n: u32 = cast (v_LEFT <: i32) in
    let rhs = Core_models.Num.impl_u64__rotate_left x n in
    let aux (i: usize {Rust_primitives.Integers.v i < 64})
      : Lemma (Rust_primitives.Integers.get_bit lhs i ==
               Rust_primitives.Integers.get_bit rhs i) =
      reveal_opaque (`%Core_models.Num.impl_u64__rotate_left)
                    Core_models.Num.impl_u64__rotate_left
    in
    FStar.Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits lhs rhs
#pop-options
