module Libcrux_ml_kem.Vector.Neon.Compress_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

(* Hand-written proof theory relocated from src/vector/neon/compress.rs
   `hax_lib::fstar::before` blocks (byte-exact raw-string contents).
   Kept in compress.rs (module-cycle-locked, F* Error 308):
   cmp_compress_u32_lane (cites compress_int32x4_t) and the
   lemma_neon_out_lane / lemma_decompress_half_out suffix (cites
   decompress_uint32x4_t). *)

(* The modular-reduction core of compress_1 (the step AVX2's
   compress_message_coefficient leaves as `assume`).  For 0 <= vec_i < 3329,
   the message-compression `((vec_i*4+3329)/6658) % 2` equals the indicator
   `833 <= vec_i <= 2496`, which is precisely the bit-15 extraction the SIMD
   chain computes. *)
let lemma_compress_1_arith (vec_i: int) : Lemma
  (requires vec_i >= 0 /\ vec_i < 3329)
  (ensures ((vec_i * 4 + 3329) / 6658) % 2 == (if 833 <= vec_i && vec_i <= 2496 then 1 else 0))
  = ()

(* >>! 15 on i16 (arithmetic shift) is sign extension: -1 if negative, else 0 *)
let lemma_i16_arith_shr_15 (x: i16) : Lemma
  (ensures v (x >>! mk_i32 15) == (if v x < 0 then -1 else 0))
  [SMTPat (x >>! mk_i32 15)]
  = ()

(* xor of an i16 with all-ones (-1) is bitwise NOT (-x-1); xor with 0 is id. *)
let lemma_i16_xor_neg1 (x: i16) : Lemma
  (ensures v (x ^. mk_i16 (-1)) == -(v x) - 1)
  [SMTPat (x ^. mk_i16 (-1))]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 (-1));
    Rust_primitives.Integers.lognot_lemma x

let lemma_i16_xor_zero (x: i16) : Lemma
  (ensures v (x ^. mk_i16 0) == v x)
  [SMTPat (x ^. mk_i16 0)]
  = Rust_primitives.Integers.logxor_lemma x (mk_i16 0)

(* xor where the MASK (all-ones / all-zeros) is the FIRST operand, as the
   SIMD chain produces it (`mask ^. shifted`).  Mirrors the sign-mask trick:
   m == -1 gives bitwise NOT of s, m == 0 gives s unchanged. *)
let lemma_i16_xor_mask_left (m s: i16) : Lemma
  (requires v m == -1 \/ v m == 0)
  (ensures v (m ^. s) == (if v m = -1 then -(v s) - 1 else v s))
  = Rust_primitives.Integers.logxor_lemma s s;
    Rust_primitives.Integers.lognot_lemma s;
    if v m = -1
    then assert (m == Rust_primitives.Integers.ones)
    else assert (m == Rust_primitives.Integers.zero)

(* The reinterpret/logical-shr-15/reinterpret tail extracts bit 15 of x's
   16-bit representation: 1 iff x < 0, else 0. *)
let lemma_tail_bit15 (x: i16) : Lemma
  (ensures (let u  = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                       #Rust_primitives.Integers.u16_inttype x in
            let sh = u >>! mk_i32 15 in
            let o  = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
                       #Rust_primitives.Integers.i16_inttype sh in
            v o == (if v x < 0 then 1 else 0)))
  = if v x < 0 then begin
      assert (v (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                   #Rust_primitives.Integers.u16_inttype x) == v x + pow2 16);
      assert ((v x + pow2 16) / pow2 15 == 1)
    end else begin
      assert (v (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                   #Rust_primitives.Integers.u16_inttype x) == v x);
      assert (v x / pow2 15 == 0)
    end

#push-options "--z3rlimit 300 --split_queries always"

(* Per-half characterization of the compress_1 SIMD chain on one i16x8.
   For each lane with input in [0,3329), the chain output lane equals
   `((vec_k*4+3329)/6658) % 2`, and that value is in {0,1}. *)
let lemma_compress_1_half (vin: Libcrux_intrinsics.Arm64_ml_kem_views.t_e_int16x8_t)
    : Lemma
      (requires
        (forall (k: nat{k < 8}).
            v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 vin k) >= 0 /\
            v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 vin k) < 3329))
      (ensures
        (let half = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mk_i16 1664) in
         let quarter = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mk_i16 832) in
         let shifted = Libcrux_intrinsics.Arm64.e_vsubq_s16 half vin in
         let mask = Libcrux_intrinsics.Arm64.e_vshrq_n_s16 (mk_i32 15) shifted in
         let stp = Libcrux_intrinsics.Arm64.e_veorq_s16 mask shifted in
         let spir = Libcrux_intrinsics.Arm64.e_vsubq_s16 stp quarter in
         let out =
           Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u16
             (Libcrux_intrinsics.Arm64.e_vshrq_n_u16 (mk_i32 15)
               (Libcrux_intrinsics.Arm64.e_vreinterpretq_u16_s16 spir)) in
         forall (k: nat{k < 8}).
           (let vec_k = v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 vin k) in
            let res_k = v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 out k) in
            res_k >= 0 /\ res_k < 2 /\ res_k == ((vec_k * 4 + 3329) / 6658) % 2)))
  = let half = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mk_i16 1664) in
    let quarter = Libcrux_intrinsics.Arm64.e_vdupq_n_s16 (mk_i16 832) in
    let shifted = Libcrux_intrinsics.Arm64.e_vsubq_s16 half vin in
    let mask = Libcrux_intrinsics.Arm64.e_vshrq_n_s16 (mk_i32 15) shifted in
    let stp = Libcrux_intrinsics.Arm64.e_veorq_s16 mask shifted in
    let spir = Libcrux_intrinsics.Arm64.e_vsubq_s16 stp quarter in
    let u16v = Libcrux_intrinsics.Arm64.e_vreinterpretq_u16_s16 spir in
    let sh = Libcrux_intrinsics.Arm64.e_vshrq_n_u16 (mk_i32 15) u16v in
    let out = Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u16 sh in
    let aux (k: nat{k < 8}) : Lemma
      (let vec_k = v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 vin k) in
       let res_k = v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 out k) in
       res_k >= 0 /\ res_k < 2 /\ res_k == ((vec_k * 4 + 3329) / 6658) % 2) =
      let vec_k = v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 vin k) in
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_vdupq_n_s16_lane (mk_i16 1664) k;
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_vdupq_n_s16_lane (mk_i16 832) k;
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_vsubq_s16_lane half vin k;
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_veorq_s16_lane mask shifted k;
      Libcrux_intrinsics.Arm64_ml_kem_views.lemma_e_vsubq_s16_lane stp quarter k;
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 half k) == 1664);
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 quarter k) == 832);
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 shifted k) == 1664 - vec_k);
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 mask k) ==
              (if 1664 - vec_k < 0 then -1 else 0));
      lemma_i16_xor_mask_left (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 mask k)
                              (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 shifted k);
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 stp k) ==
              (if 1664 - vec_k < 0 then -(1664 - vec_k) - 1 else 1664 - vec_k));
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 spir k) ==
              (if 1664 - vec_k < 0 then vec_k - 2497 else 832 - vec_k));
      assert ((v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 spir k) < 0) ==
              (833 <= vec_k && vec_k <= 2496));
      lemma_tail_bit15 (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 spir k);
      assert (v (Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 out k) ==
              (if 833 <= vec_k && vec_k <= 2496 then 1 else 0));
      lemma_compress_1_arith vec_k
    in
    Classical.forall_intro aux

#pop-options

module NA = Libcrux_intrinsics.Arm64_ml_kem_views

(* `1 <<! (cb-1)` as u32 has value 2^(d-1). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_neon_twopow_m1 (cb: i32)
  : Lemma (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11))
          (ensures Rust_primitives.Integers.v (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32) ==
                   pow2 (v cb - 1))
  = assert_norm (pow2 10 == 1024); assert_norm (pow2 31 == 2147483648);
    assert_norm (pow2 32 == 4294967296);
    FStar.Math.Lemmas.pow2_le_compat 10 (v cb - 1)
#pop-options

(* clean-context bound: (a*3329 + 2^(d-1)) / 2^d < 3329 for a < 2^d. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 80"
let lemma_decompress_u32_bound (a dd: nat)
  : Lemma (requires a < pow2 dd /\ (dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11))
          (ensures a * 3329 + pow2 (dd - 1) < 4294967296 /\
                   (a * 3329 + pow2 (dd - 1)) / pow2 dd < 3329)
  = assert_norm (pow2 4 == 16); assert_norm (pow2 5 == 32);
    assert_norm (pow2 10 == 1024); assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 dd;
    FStar.Math.Lemmas.pow2_le_compat 10 (dd - 1);
    FStar.Math.Lemmas.lemma_mult_le_right 3329 a (pow2 dd - 1);
    let n = a * 3329 + pow2 (dd - 1) in
    FStar.Math.Lemmas.pow2_double_mult (dd - 1);
    assert (n < 3329 * pow2 dd);
    FStar.Math.Lemmas.lemma_div_plus (-1) 3329 (pow2 dd);
    FStar.Math.Lemmas.lemma_div_le n (3329 * pow2 dd - 1) (pow2 dd)
#pop-options

(* per-u32-lane decompress core, proven standalone (param `vv`, no `v` shadow). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_decompress_u32_lane (vv: NA.t_e_uint32x4_t) (cb: i32) (k: nat{k < 4})
  : Lemma
    (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
              Rust_primitives.Integers.v (NA.get_lane_u32x4 vv k) < pow2 (v cb))
    (ensures
      (let coeff = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32) in
       let d1 = Libcrux_intrinsics.Arm64.e_vmulq_n_u32 vv (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: u32) in
       let d2 = Libcrux_intrinsics.Arm64.e_vaddq_u32 d1 coeff in
       let r = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 cb d2 in
       let a = Rust_primitives.Integers.v (NA.get_lane_u32x4 vv k) in
       Rust_primitives.Integers.v (NA.get_lane_u32x4 r k) ==
         (a * 3329 + pow2 (v cb - 1)) / pow2 (v cb) /\
       Rust_primitives.Integers.v (NA.get_lane_u32x4 r k) < 3329))
  = let a = Rust_primitives.Integers.v (NA.get_lane_u32x4 vv k) in
    assert_norm (pow2 11 == 2048);
    FStar.Math.Lemmas.pow2_le_compat 11 (v cb);
    assert_norm (Rust_primitives.Integers.v
      (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: u32) == 3329);
    lemma_neon_twopow_m1 cb;
    lemma_decompress_u32_bound a (v cb);
    let coeff = Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32) in
    let d1 = Libcrux_intrinsics.Arm64.e_vmulq_n_u32 vv (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS <: i16) <: u32) in
    let d2 = Libcrux_intrinsics.Arm64.e_vaddq_u32 d1 coeff in
    (* coeff lane k == 2^(d-1) *)
    assert (NA.get_lane_u32x4 coeff k == (mk_u32 1 <<! (cb -! mk_i32 1 <: i32) <: u32));
    (* a * 3329 < 2^32 so the u32 mul does not wrap *)
    FStar.Math.Lemmas.lemma_mult_le_right 3329 a 2047;
    assert (Rust_primitives.Integers.v (NA.get_lane_u32x4 d1 k) == a * 3329);
    assert (Rust_primitives.Integers.v (NA.get_lane_u32x4 d2 k) == a * 3329 + pow2 (v cb - 1))
#pop-options

(* ---- Reinterpret round-trip bit facts (pure crate-helper, NO trust; mirror
   the analogous lemmas in Vector.Neon.Ntt, which this module cannot import) ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 120"
let lemma_i16_bits_as_u32_bit (a: i16) (i: usize {v i < 32}) : Lemma
  (ensures get_bit (NA.i16_bits_as_u32 a) i == (if v i < 16 then get_bit a i else 0))
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
              #Rust_primitives.Integers.u16_inttype a in
    FStar.Math.Lemmas.small_mod (v w) (pow2 32);
    assert (NA.i16_bits_as_u32 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
              #Rust_primitives.Integers.u32_inttype w)

(* value of i16_bits_as_u32 on a non-negative i16 (so v a < 2^15 < 2^16). *)
let lemma_i16_bits_as_u32_val (a: i16) : Lemma
  (requires 0 <= v a)
  (ensures v (NA.i16_bits_as_u32 a) == v a)
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
              #Rust_primitives.Integers.u16_inttype a in
    FStar.Math.Lemmas.small_mod (v a) (pow2 16);
    assert (NA.i16_bits_as_u32 a ==
            Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
              #Rust_primitives.Integers.u32_inttype w);
    FStar.Math.Lemmas.small_mod (v w) (pow2 32)

(* the deinterleave: AND with 0xffff extracts the low (even) i16 lane, SHR 16 the
   odd lane, from the u32 reinterpret `i16_bits_as_u32 a |. (i16_bits_as_u32 b <<! 16)`. *)
let lemma_deint_lo (a b: i16) : Lemma
  (requires 0 <= v a /\ 0 <= v b)
  (ensures v ((NA.i16_bits_as_u32 a |. (NA.i16_bits_as_u32 b <<! mk_u32 16) <: u32) &. mk_u32 65535)
           == v a)
  = let x = NA.i16_bits_as_u32 a in
    let y = NA.i16_bits_as_u32 b in
    let r = (x |. (y <<! mk_u32 16) <: u32) &. mk_u32 65535 in
    assert_norm (pow2 16 == 65536);
    let aux (i: usize {v i < 32}) : Lemma (get_bit r i == get_bit x i) =
      lemma_i16_bits_as_u32_bit a i;
      lemma_i16_bits_as_u32_bit b i;
      Rust_primitives.BitVectors.get_bit_pow2_minus_one #Rust_primitives.Integers.u32_inttype 16 i
    in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r x;
    lemma_i16_bits_as_u32_val a

let lemma_deint_hi (a b: i16) : Lemma
  (requires 0 <= v a /\ 0 <= v b)
  (ensures v ((NA.i16_bits_as_u32 a |. (NA.i16_bits_as_u32 b <<! mk_u32 16) <: u32) >>! mk_u32 16)
           == v b)
  = let x = NA.i16_bits_as_u32 a in
    let y = NA.i16_bits_as_u32 b in
    let r = (x |. (y <<! mk_u32 16) <: u32) >>! mk_u32 16 in
    let aux (i: usize {v i < 32}) : Lemma (get_bit r i == get_bit y i) =
      lemma_i16_bits_as_u32_bit a (if v i < 16 then sz (v i + 16) else i);
      lemma_i16_bits_as_u32_bit b i
    in
    Classical.forall_intro aux;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits r y;
    lemma_i16_bits_as_u32_val b

(* reinterpret_s16_u32 back: lo16 of a small u32 is its value as i16; hi16 is 0. *)
let lemma_u32_lo16_val (d: u32) : Lemma
  (requires v d < pow2 15)
  (ensures NA.u32_lo16_as_i16 d == mk_i16 (v d) /\ v (NA.u32_lo16_as_i16 d) == v d)
  = let w = Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u32_inttype
              #Rust_primitives.Integers.u16_inttype d in
    FStar.Math.Lemmas.small_mod (v d) (pow2 16);
    FStar.Math.Lemmas.small_mod (v w) (pow2 16)

let lemma_u32_hi16_zero (d: u32) : Lemma
  (requires v d < pow2 16)
  (ensures NA.u32_hi16_as_i16 d == mk_i16 0)
  = FStar.Math.Lemmas.small_div (v d) (pow2 16);
    assert (v (d >>! mk_u32 16) == 0);
    assert ((d >>! mk_u32 16) == mk_u32 0)
#pop-options

(* the Neon spine value (a*3329+2^(d-1))/2^d equals the bridge / AVX2 / portable
   form (2a*3329+2^d)/(2^d*2) by the 2x/2y == x/y cancellation. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 60"
let lemma_decompress_form_eq (a dd: nat)
  : Lemma (requires dd == 4 \/ dd == 5 \/ dd == 10 \/ dd == 11)
          (ensures (a * 3329 + pow2 (dd - 1)) / pow2 dd ==
                   (2 * a * 3329 + pow2 dd) / (pow2 dd * 2))
  = FStar.Math.Lemmas.pow2_double_mult (dd - 1);
    let zz = a * 3329 + pow2 (dd - 1) in
    FStar.Math.Lemmas.division_multiplication_lemma (2 * zz) 2 (pow2 dd)
#pop-options

(* per-output-lane assembly: vtrn1q_s16 (reinterpret_s16_u32 l0d) (reinterpret_s16_u32 l1d)
   places the even/odd decompressed u32 lanes back in order.  Free-param (no
   decompress recomputation) so it composes lane-by-lane (no 16-lane saturation). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_assemble_lane (l0d l1d: NA.t_e_uint32x4_t) (j: nat{j < 8}) : Lemma
  (requires v (NA.get_lane_u32x4 l0d (j / 2)) < pow2 15 /\
            v (NA.get_lane_u32x4 l1d (j / 2)) < pow2 15)
  (ensures
    (let out = Libcrux_intrinsics.Arm64.e_vtrn1q_s16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0d) (Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1d) in
     0 <= v (NA.get_lane_i16x8 out j) /\ v (NA.get_lane_i16x8 out j) < pow2 15 /\
     v (NA.get_lane_i16x8 out j) ==
       (if j % 2 = 0
        then v (NA.get_lane_u32x4 l0d (j / 2))
        else v (NA.get_lane_u32x4 l1d (j / 2)))))
  = let aa = Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l0d in
    let bb = Libcrux_intrinsics.Arm64.e_vreinterpretq_s16_u32 l1d in
    let k = j / 2 in
    FStar.Math.Lemmas.lemma_div_mod j 2;
    NA.lemma_e_vtrn1q_s16_lane aa bb j;
    if j % 2 = 0
    then lemma_u32_lo16_val (NA.get_lane_u32x4 l0d k)
    else lemma_u32_lo16_val (NA.get_lane_u32x4 l1d k)
#pop-options

(* clean-context: the 4 deinterleaved lanes equal the even/odd input lanes AND are
   < 2^d.  Proven away from the decompress/assemble context (which otherwise
   saturates when this 4-lane forall is derived inline). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_deint_bounds (hv: NA.t_e_int16x8_t) (cb: i32) : Lemma
  (requires (v cb == 4 \/ v cb == 5 \/ v cb == 10 \/ v cb == 11) /\
            (forall (m: nat). m < 8 ==>
              0 <= v (NA.get_lane_i16x8 hv m) /\ v (NA.get_lane_i16x8 hv m) < pow2 (v cb)))
  (ensures
    (let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
     let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r (Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535)) in
     let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
     forall (m: nat). m < 4 ==>
       v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
       v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
       v (NA.get_lane_u32x4 l0 m) < pow2 (v cb) /\ v (NA.get_lane_u32x4 l1 m) < pow2 (v cb)))
  = let r = Libcrux_intrinsics.Arm64.e_vreinterpretq_u32_s16 hv in
    let l0 = Libcrux_intrinsics.Arm64.e_vandq_u32 r (Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535)) in
    let l1 = Libcrux_intrinsics.Arm64.e_vshrq_n_u32 (mk_i32 16) r in
    let aux (m: nat{m < 4})
      : Lemma (v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
               v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
               v (NA.get_lane_u32x4 l0 m) < pow2 (v cb) /\
               v (NA.get_lane_u32x4 l1 m) < pow2 (v cb)) =
      assert (2 * m < 8 /\ 2 * m + 1 < 8);
      NA.lemma_e_vandq_u32_lane r (Libcrux_intrinsics.Arm64.e_vdupq_n_u32 (mk_u32 65535)) m;
      NA.lemma_e_vdupq_n_u32_lane (mk_u32 65535) m;
      lemma_deint_lo (NA.get_lane_i16x8 hv (2 * m)) (NA.get_lane_i16x8 hv (2 * m + 1));
      lemma_deint_hi (NA.get_lane_i16x8 hv (2 * m)) (NA.get_lane_i16x8 hv (2 * m + 1))
    in
    introduce forall (m: nat). m < 4 ==>
      (v (NA.get_lane_u32x4 l0 m) == v (NA.get_lane_i16x8 hv (2 * m)) /\
       v (NA.get_lane_u32x4 l1 m) == v (NA.get_lane_i16x8 hv (2 * m + 1)) /\
       v (NA.get_lane_u32x4 l0 m) < pow2 (v cb) /\ v (NA.get_lane_u32x4 l1 m) < pow2 (v cb))
    with (if m < 4 then aux m)
#pop-options
