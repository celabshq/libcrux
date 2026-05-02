module EquivImplSpec.Sponge.Portable.SqueezeBytes

(* Per-byte aux lemmas for the Portable.squeeze proof, lifted out of
   the in-body [forall_intro] closures so each lemma is verified
   standalone with one quantifier scope.

   This is the structural fix sketched in the USER-2 stability admit
   on [Libcrux_sha3.Generic_keccak.Portable.squeeze]: each per-byte
   aux is verified once here, and the squeeze body cites them by
   name in [forall_intro]. *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 200"

open FStar.Mul
open Core_models

module KP = EquivImplSpec.Keccakf.Portable

(* Per-byte step write-region lemma.

   Pre: at iteration [i], the loop invariant holds (per-byte forall
        over the prefix and tail), [v i >= 1], and [(v i + 1) * v rate
        <= v outlen].
   Post: the new output (after [keccakf1600] + [f_squeeze] at offset
        [i*rate], len [rate]) agrees with the byteform spec at byte [k]
        for every [k] in the new prefix range. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_step_byte_write
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array u64 (mk_usize 25))
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (output_pre: t_Slice u8)
      (output_initial: t_Array u8 (Core_models.Slice.impl__len #u8 output_pre))
      (i: usize)
      (k: nat)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 output_pre in
        v i >= 1 /\
        v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        v (i +! mk_usize 1) <= v outlen / v rate /\
        v i <= v outlen / v rate /\
        ks_pre.Libcrux_sha3.Generic_keccak.f_st ==
          Hacspec_sha3.Sponge.iterate_keccak_f (i -! mk_usize 1) s_init_st /\
        (forall (kp: nat). kp < v i * v rate /\ kp < v outlen ==>
          Seq.index (output_pre <: Seq.seq u8) kp ==
          Seq.index
            (Hacspec_sha3.Sponge.squeeze outlen s_init_st rate <: Seq.seq u8) kp) /\
        k < v outlen /\
        k < (v i + 1) * v rate))
      (ensures (
        let ks_post =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 1) #u64 ks_pre in
        let output_post =
          Libcrux_sha3.Traits.f_squeeze
            #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
            #u64 #FStar.Tactics.Typeclasses.solve
            rate ks_post output_pre (i *! rate) rate in
        let outlen = Core_models.Slice.impl__len #u8 output_pre in
        Seq.index (output_post <: Seq.seq u8) k ==
        Seq.index
          (Hacspec_sha3.Sponge.squeeze outlen s_init_st rate <: Seq.seq u8) k))
  = let outlen = Core_models.Slice.impl__len #u8 output_pre in
    KP.lemma_keccakf1600_portable ks_pre;
    let ks_post =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 1) #u64 ks_pre in
    let output_post =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        rate ks_post output_pre (i *! rate) rate in
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    let kk : usize = mk_usize k in
    assert (v kk == k);
    if k < v i * v rate then ()
    else begin
      assert (v i * v rate <= k);
      assert ((v i + 1) * v rate == v i * v rate + v rate);
      assert (k - v i * v rate < v rate);
      assert ((k - v i * v rate) / 8 < 25);
      FStar.Math.Lemmas.small_div (k - v i * v rate) (v rate);
      FStar.Math.Lemmas.lemma_div_plus
        (k - v i * v rate) (v i) (v rate);
      let b : usize = kk /! rate in
      assert (v b == v i);
      let j : usize = kk -! (b *! rate) in
      assert (v j == k - v i * v rate);
      assert (v j / 8 < 25);
      ()
    end
#pop-options


(* Per-byte step tail-region lemma.  Mirrors [_byte_write] for the
   k >= (i+1)*rate range — same i/o framing. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_step_byte_tail
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array u64 (mk_usize 25))
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (output_pre: t_Slice u8)
      (output_initial: t_Array u8 (Core_models.Slice.impl__len #u8 output_pre))
      (i: usize)
      (k: nat)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 output_pre in
        v i >= 1 /\
        v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        ks_pre.Libcrux_sha3.Generic_keccak.f_st ==
          Hacspec_sha3.Sponge.iterate_keccak_f (i -! mk_usize 1) s_init_st /\
        (forall (kp: nat). v i * v rate <= kp /\ kp < v outlen ==>
          Seq.index (output_pre <: Seq.seq u8) kp ==
          Seq.index (output_initial <: Seq.seq u8) kp) /\
        k < v outlen /\
        (v i + 1) * v rate <= k))
      (ensures (
        let ks_post =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 1) #u64 ks_pre in
        let output_post =
          Libcrux_sha3.Traits.f_squeeze
            #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
            #u64 #FStar.Tactics.Typeclasses.solve
            rate ks_post output_pre (i *! rate) rate in
        Seq.index (output_post <: Seq.seq u8) k ==
        Seq.index (output_initial <: Seq.seq u8) k))
  = KP.lemma_keccakf1600_portable ks_pre;
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    let kk : usize = mk_usize k in
    assert (v kk == k);
    assert ((v i + 1) * v rate == v i * v rate + v rate);
    assert (v i * v rate + v rate <= k)
#pop-options
