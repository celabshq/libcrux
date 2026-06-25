module Avx2NttCompose
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Spec.Intrinsics
open Spec.MLDSA.NttConstants
open Spec.MLDSA.Math

#push-options "--z3rlimit 400 --split_queries always"

let lemma_modq_eq (xa xb : i64) : Lemma
    (requires (v xa) % 8380417 == (v xb) % 8380417)
    (ensures Hacspec_ml_dsa.Arithmetic.mod_q xa == Hacspec_ml_dsa.Arithmetic.mod_q xb)
  = Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v xa; Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v xb

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_bf_even_cong (z: i64) (x y x' y': i32) : Lemma
    (requires (v z) >= -2147483648 /\ (v z) <= 2147483647 /\
              (v x) % 8380417 == (v x') % 8380417 /\ (v y) % 8380417 == (v y') % 8380417)
    (ensures
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x <: i64) +! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64))) <: i64)) ==
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x' <: i64) +! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64))) <: i64)))
  = FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y') 8380417;
    lemma_modq_eq (z *! (cast y <: i64)) (z *! (cast y' <: i64));
    let ta = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64)) in
    let tb = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64)) in
    assert (ta == tb);
    FStar.Math.Lemmas.lemma_mod_add_distr (v (cast ta <: i64)) (v x) 8380417;
    FStar.Math.Lemmas.lemma_mod_add_distr (v (cast tb <: i64)) (v x') 8380417;
    lemma_modq_eq ((cast x <: i64) +! (cast ta <: i64)) ((cast x' <: i64) +! (cast tb <: i64))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_bf_odd_cong (z: i64) (x y x' y': i32) : Lemma
    (requires (v z) >= -2147483648 /\ (v z) <= 2147483647 /\
              (v x) % 8380417 == (v x') % 8380417 /\ (v y) % 8380417 == (v y') % 8380417)
    (ensures
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x <: i64) -! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64))) <: i64)) ==
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x' <: i64) -! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64))) <: i64)))
  = FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y') 8380417;
    lemma_modq_eq (z *! (cast y <: i64)) (z *! (cast y' <: i64));
    let ta = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64)) in
    let tb = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64)) in
    assert (ta == tb);
    FStar.Math.Lemmas.lemma_mod_sub_distr (v x) (v (cast ta <: i64)) 8380417;
    FStar.Math.Lemmas.lemma_mod_sub_distr (v x') (v (cast tb <: i64)) 8380417;
    lemma_modq_eq ((cast x <: i64) -! (cast ta <: i64)) ((cast x' <: i64) -! (cast tb <: i64))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_0_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_0_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_0_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 2 in
    assert (v round < 128);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 128 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 2;
    FStar.Math.Lemmas.lemma_div_mod i 2;
    let parity : (n:nat{n < 2}) = i % 2 in
    assert (v (ii %! mk_usize 2) == parity);
    if parity < 1 then begin
      assert (ii %! mk_usize 2 <. mk_usize 1);
      assert (i + 1 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 1))
                           (Seq.index b i) (Seq.index b (i + 1))
    end else begin
      assert (~(ii %! mk_usize 2 <. mk_usize 1));
      assert (i >= 1);
      lemma_bf_odd_cong z (Seq.index a (i - 1)) (Seq.index a i)
                          (Seq.index b (i - 1)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_0_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 0) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 0))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 0)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 0)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_0_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_0_lane b ii;
      lemma_layer_0_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 0)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 0))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_1_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_1_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_1_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 4 in
    assert (v round < 64);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 64 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 4;
    FStar.Math.Lemmas.lemma_div_mod i 4;
    let parity : (n:nat{n < 4}) = i % 4 in
    assert (v (ii %! mk_usize 4) == parity);
    if parity < 2 then begin
      assert (ii %! mk_usize 4 <. mk_usize 2);
      assert (i + 2 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 2))
                           (Seq.index b i) (Seq.index b (i + 2))
    end else begin
      assert (~(ii %! mk_usize 4 <. mk_usize 2));
      assert (i >= 2);
      lemma_bf_odd_cong z (Seq.index a (i - 2)) (Seq.index a i)
                          (Seq.index b (i - 2)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_1_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 1) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 1))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 1)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 1)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_1_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_1_lane b ii;
      lemma_layer_1_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 1)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 1))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_2_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_2_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_2_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 8 in
    assert (v round < 32);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 32 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 8;
    FStar.Math.Lemmas.lemma_div_mod i 8;
    let parity : (n:nat{n < 8}) = i % 8 in
    assert (v (ii %! mk_usize 8) == parity);
    if parity < 4 then begin
      assert (ii %! mk_usize 8 <. mk_usize 4);
      assert (i + 4 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 4))
                           (Seq.index b i) (Seq.index b (i + 4))
    end else begin
      assert (~(ii %! mk_usize 8 <. mk_usize 4));
      assert (i >= 4);
      lemma_bf_odd_cong z (Seq.index a (i - 4)) (Seq.index a i)
                          (Seq.index b (i - 4)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_2_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 2) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 2))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 2)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 2)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_2_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_2_lane b ii;
      lemma_layer_2_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 2)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 2))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_3_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_3_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_3_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 16 in
    assert (v round < 16);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 16 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 16;
    FStar.Math.Lemmas.lemma_div_mod i 16;
    let parity : (n:nat{n < 16}) = i % 16 in
    assert (v (ii %! mk_usize 16) == parity);
    if parity < 8 then begin
      assert (ii %! mk_usize 16 <. mk_usize 8);
      assert (i + 8 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 8))
                           (Seq.index b i) (Seq.index b (i + 8))
    end else begin
      assert (~(ii %! mk_usize 16 <. mk_usize 8));
      assert (i >= 8);
      lemma_bf_odd_cong z (Seq.index a (i - 8)) (Seq.index a i)
                          (Seq.index b (i - 8)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_3_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 3) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 3))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 3)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 3)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_3_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_3_lane b ii;
      lemma_layer_3_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 3)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 3))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_4_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_4_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_4_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 32 in
    assert (v round < 8);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 8 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 32;
    FStar.Math.Lemmas.lemma_div_mod i 32;
    let parity : (n:nat{n < 32}) = i % 32 in
    assert (v (ii %! mk_usize 32) == parity);
    if parity < 16 then begin
      assert (ii %! mk_usize 32 <. mk_usize 16);
      assert (i + 16 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 16))
                           (Seq.index b i) (Seq.index b (i + 16))
    end else begin
      assert (~(ii %! mk_usize 32 <. mk_usize 16));
      assert (i >= 16);
      lemma_bf_odd_cong z (Seq.index a (i - 16)) (Seq.index a i)
                          (Seq.index b (i - 16)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_4_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 4) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 4))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 4)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 4)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_4_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_4_lane b ii;
      lemma_layer_4_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 4)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 4))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_5_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_5_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_5_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 64 in
    assert (v round < 4);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 4 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 64;
    FStar.Math.Lemmas.lemma_div_mod i 64;
    let parity : (n:nat{n < 64}) = i % 64 in
    assert (v (ii %! mk_usize 64) == parity);
    if parity < 32 then begin
      assert (ii %! mk_usize 64 <. mk_usize 32);
      assert (i + 32 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 32))
                           (Seq.index b i) (Seq.index b (i + 32))
    end else begin
      assert (~(ii %! mk_usize 64 <. mk_usize 32));
      assert (i >= 32);
      lemma_bf_odd_cong z (Seq.index a (i - 32)) (Seq.index a i)
                          (Seq.index b (i - 32)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_5_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 5) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 5))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 5)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 5)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_5_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_5_lane b ii;
      lemma_layer_5_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 5)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 5))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_6_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_6_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_6_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 128 in
    assert (v round < 2);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 2 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 128;
    FStar.Math.Lemmas.lemma_div_mod i 128;
    let parity : (n:nat{n < 128}) = i % 128 in
    assert (v (ii %! mk_usize 128) == parity);
    if parity < 64 then begin
      assert (ii %! mk_usize 128 <. mk_usize 64);
      assert (i + 64 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 64))
                           (Seq.index b i) (Seq.index b (i + 64))
    end else begin
      assert (~(ii %! mk_usize 128 <. mk_usize 64));
      assert (i >= 64);
      lemma_bf_odd_cong z (Seq.index a (i - 64)) (Seq.index a i)
                          (Seq.index b (i - 64)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_6_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 6) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 6))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 6)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 6)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_6_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_6_lane b ii;
      lemma_layer_6_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 6)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 6))
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_ntt_compose_avx2 (f0 s76 s53 s2 s1 ffinal : t_Array i32 (mk_usize 256)) : Lemma
    (requires
      (forall (i:nat). i < 256 ==> (v (Seq.index s76 i)) % 8380417 ==
         (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer f0 (mk_usize 7)) (mk_usize 6)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index s53 i)) % 8380417 ==
         (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer s76 (mk_usize 5)) (mk_usize 4)) (mk_usize 3)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index s2 i)) % 8380417 ==
         (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer s53 (mk_usize 2)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index s1 i)) % 8380417 ==
         (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer s2 (mk_usize 1)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 ==
         (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer s1 (mk_usize 0)) i)) % 8380417))
    (ensures
      (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 ==
         (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt f0) i)) % 8380417))
  = let g7 = Hacspec_ml_dsa.Ntt.ntt_layer f0 (mk_usize 7) in
    let g6 = Hacspec_ml_dsa.Ntt.ntt_layer g7 (mk_usize 6) in
    assert (forall (i:nat). i < 256 ==> (v (Seq.index s76 i)) % 8380417 == (v (Seq.index g6 i)) % 8380417);
    lemma_ntt_layer_5_cong s76 g6;
    let g5 = Hacspec_ml_dsa.Ntt.ntt_layer g6 (mk_usize 5) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer s76 (mk_usize 5) == g5);
    lemma_ntt_layer_4_cong (Hacspec_ml_dsa.Ntt.ntt_layer s76 (mk_usize 5)) g5;
    let g4 = Hacspec_ml_dsa.Ntt.ntt_layer g5 (mk_usize 4) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer s76 (mk_usize 5)) (mk_usize 4) == g4);
    lemma_ntt_layer_3_cong (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer s76 (mk_usize 5)) (mk_usize 4)) g4;
    let g3 = Hacspec_ml_dsa.Ntt.ntt_layer g4 (mk_usize 3) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer (Hacspec_ml_dsa.Ntt.ntt_layer s76 (mk_usize 5)) (mk_usize 4)) (mk_usize 3) == g3);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index s53 i)) % 8380417 == (v (Seq.index g3 i)) % 8380417);
    lemma_ntt_layer_2_cong s53 g3;
    let g2 = Hacspec_ml_dsa.Ntt.ntt_layer g3 (mk_usize 2) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer s53 (mk_usize 2) == g2);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index s2 i)) % 8380417 == (v (Seq.index g2 i)) % 8380417);
    lemma_ntt_layer_1_cong s2 g2;
    let g1 = Hacspec_ml_dsa.Ntt.ntt_layer g2 (mk_usize 1) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer s2 (mk_usize 1) == g1);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index s1 i)) % 8380417 == (v (Seq.index g1 i)) % 8380417);
    lemma_ntt_layer_0_cong s1 g1;
    let g0 = Hacspec_ml_dsa.Ntt.ntt_layer g1 (mk_usize 0) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer s1 (mk_usize 0) == g0);
    assert (Hacspec_ml_dsa.Ntt.ntt f0 == g0)
#pop-options
#pop-options
