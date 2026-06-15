module Spec.Intrinsics
open FStar.Mul
open Core_models
open Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
 
let logand_lemma_forall #t:
  Lemma (forall a. logand ones a == a /\ 
              logand a ones == a /\ 
              logand a zero == zero #t /\ 
              logand zero a == zero #t /\
              logand a a == a) =
  FStar.Classical.forall_intro (fun a -> logand_lemma #t a a)

let logand_mask_lemma_forall #t:
  Lemma (forall a m.
              m < bits t ==>
              (pow2 m < maxint t /\
               logand a (sub #t (mk_int #t (pow2 m)) (mk_int #t 1)) ==
               mk_int (v a % pow2 m))) =
  admit()

let logxor_lemma_forall #t:
  Lemma (forall a. 
    a `logxor` a == zero /\
    zero #t `logxor` a == a /\
    a `logxor` zero #t == a /\
    ones #t `logxor` a == lognot a /\
    a `logxor` ones #t == lognot a) =
  FStar.Classical.forall_intro (fun a -> logxor_lemma #t a a)

let lognot_lemma_forall #t:
  Lemma (forall a. 
   lognot #t zero == ones /\
   lognot #t ones == zero /\
   lognot (lognot a) == a /\
   (signed t ==> v (lognot a) = -1 - v a) /\
   (unsigned t ==> v (lognot a)  = pow2 (bits t) - 1 - v #t a)) =
  FStar.Classical.forall_intro (fun a -> lognot_lemma #t a)


(* Opaque arithmetic operations *)
[@@ "opaque_to_smt"]
let add_mod_opaque #t = add_mod #t

[@@ "opaque_to_smt"]
let sub_mod_opaque #t = sub_mod #t

[@@ "opaque_to_smt"]
let mul_mod_opaque #t = mul_mod #t

[@@ "opaque_to_smt"]
let shift_right_opaque #t = shift_right #t #i32_inttype

[@@ "opaque_to_smt"]
let shift_left_opaque #t = shift_left #t #i32_inttype

[@@ "opaque_to_smt"]
let cast_mod_opaque #t #t' = cast_mod #t #t'

let reveal_opaque_arithmetic_ops #t:
  Lemma (add_mod_opaque #t == add_mod #t /\
         sub_mod_opaque #t == sub_mod #t /\
         mul_mod_opaque #t == mul_mod #t /\
         shift_left_opaque #t == shift_left #t #i32_inttype /\
         shift_right_opaque #t == shift_right #t #i32_inttype) =
  reveal_opaque (`%add_mod_opaque) (add_mod_opaque #t);
  reveal_opaque (`%sub_mod_opaque) (sub_mod_opaque #t);
  reveal_opaque (`%mul_mod_opaque) (mul_mod_opaque #t);
  reveal_opaque (`%shift_left_opaque) (shift_left_opaque #t);
  reveal_opaque (`%shift_right_opaque) (shift_right_opaque #t)
  
let reveal_opaque_cast_ops #t #t':
  Lemma (cast_mod_opaque #t #t' == cast_mod #t #t' /\
         cast_mod_opaque #t' #t == cast_mod #t' #t) =
  reveal_opaque (`%cast_mod_opaque) (cast_mod_opaque #t #t');
  reveal_opaque (`%cast_mod_opaque) (cast_mod_opaque #t' #t)

open FStar.FunctionalExtensionality
module I = Libcrux_intrinsics.Avx2

open Libcrux_core_models.Abstractions.Bit

(**** Utils *)
/// Functional arrays: partial maps
type t_FunArray (n: u64) t = i:u64 {v i < v n} ^-> t
/// Accessor for fun arrays
let ( .() ) #n (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n) i  = Libcrux_core_models.Abstractions.Funarr.impl_5__get n bv._0 i

let eq_pointwise_to_eq #n (a b: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
  : Lemma (requires forall i. a.(i) == b.(i)) (ensures a == b)
  = let Libcrux_core_models.Abstractions.Bitvec.BitVec (Libcrux_core_models.Abstractions.Funarr.FunArray a) = a in
    let Libcrux_core_models.Abstractions.Bitvec.BitVec (Libcrux_core_models.Abstractions.Funarr.FunArray b) = b in
    assert (forall i. a i == b i);
    FStar.FunctionalExtensionality.extensionality _ _ a b

[@@ "opaque_to_smt"]
let i16_mul_32extended (x y: i16): i32 = (cast x <: i32) *! (cast y <: i32)
[@@ "opaque_to_smt"]
let i16_mul_32extended_i16 (x y: i16): i16 = cast (i16_mul_32extended x y)

[@@ "opaque_to_smt"]
let i32_wrapping_add (x y: i32): i32 = x +. y

(**** Bit vector type synonym *)
let bv128 = Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)
let bv256 = Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)

(**** Machine integer arrays *)

let u8x16  = t_FunArray (mk_u64 16) u8

let i8x16  = t_FunArray (mk_u64 16) i8
let i8x32  = t_FunArray (mk_u64 32) i8
let i16x16 = t_FunArray (mk_u64 16) i16
let i32x4  = t_FunArray (mk_u64 4) i32
let i32x8  = t_FunArray (mk_u64 8) i32
let i64x4  = t_FunArray (mk_u64 4) i64

(**** Bit vec to int vec *)
val to_i8x32  (x: bv256): i8x32
val to_i8x16  (x: bv128): i8x16
val to_i16x16 (x: bv256): i16x16
val to_i32x4  (x: bv128): i32x4
val to_i32x8  (x: bv256): i32x8
val to_i64x4  (x: bv256): i64x4

val to_u8x16  (x: bv128): u8x16

(**** Int vec to bit vec *)
val from_i32x8 (x:i32x8):  bv256

(**** Int to bit vecs *)
val i16_to_bv (x: i16): t_FunArray (mk_int 16) t_Bit
val i32_to_bv (x: i32): t_FunArray (mk_int 32) t_Bit
val i64_to_bv (x: i64): t_FunArray (mk_int 64) t_Bit
val u8_to_bv (x: u8): t_FunArray (mk_int 8) t_Bit

(**** Inversion lemmas *)
val to_from_i32x8_inv_lemma (x: i32x8)
  : Lemma (to_i32x8 (from_i32x8 x) == x) [SMTPat (to_i32x8 (from_i32x8 x))]

val i16_to_bv_to_i16x16_inv (vec: bv256) (i: u64 {v i < 16}) (j: u64 {v j < 16})
  : Lemma (i16_to_bv (to_i16x16 vec i) j == vec.(mk_int (v i * 16 + v j)))
    [SMTPat (i16_to_bv (to_i16x16 vec i) j)]

(**** Lemmas about intrinsics *)
val mm256_castsi256_si128_bv_lemma vec i
  : Lemma ((I.mm256_castsi256_si128 vec).(i) == vec.(i))
          [SMTPat (I.mm256_castsi256_si128 vec).(i)]

val mm256_extracti128_si256_bv_lemma (control: i32) vec i
  : Lemma ((I.mm256_extracti128_si256 control vec).(i) == vec.(mk_int (if v control = 0 then 0 else 128) +! i))
          [SMTPat (I.mm256_extracti128_si256 control vec).(i)]

val mm256_extracti128_si256_lemma (control: i32) vec (i: _{v i < 4})
  : Lemma (to_i32x4 (I.mm256_extracti128_si256 control vec) i == to_i32x8 vec (i +! mk_int (if v control = 0 then 0 else 4)))
          [SMTPat (to_i32x4 (I.mm256_extracti128_si256 control vec) i)]

val mm256_and_si256 lhs rhs i
  : Lemma ( (I.mm256_and_si256 lhs rhs).(i) == (
            match lhs.(i), rhs.(i) with
            | Bit_One, Bit_One -> Bit_One
            | _                -> Bit_Zero ))
            [SMTPat (I.mm256_and_si256 lhs rhs).(i)]

val mm_storeu_bytes_si128_lemma out vec (i:nat {i < 16})
  : Lemma (requires Seq.length out >= 16)
          (ensures Seq.index (I.mm_storeu_bytes_si128 out vec) i == to_u8x16 vec (mk_int i))
          [SMTPat (Seq.index (I.mm_storeu_bytes_si128 out vec) i)]

val update_at_range_bv_lemma
    (f_start: usize) (f_end: _ {v f_end == v f_start + 16})
    (bytes: t_Slice u8{Seq.length bytes >= v f_start + 16})
    (dummy_out: _ {Seq.length dummy_out == 16})
    vec i
  : Lemma (
      Seq.index (
        Rust_primitives.Hax.Monomorphized_update_at.update_at_range 
          bytes { f_start; f_end } (I.mm_storeu_bytes_si128 dummy_out vec)
      ) i == (if i >= v f_start && i < v f_end
              then to_u8x16 vec (mk_int (i - v f_start))
              else Seq.index bytes i))
    [SMTPat (Seq.index (
        Rust_primitives.Hax.Monomorphized_update_at.update_at_range 
          bytes { f_start; f_end } (I.mm_storeu_bytes_si128 dummy_out vec)
      ) i)]

val u8_to_bv_to_u8x16_inv (vec: bv128) (i: u64 {v i < 16}) (j: u64 {v j < 8})
  : Lemma (u8_to_bv (to_u8x16 vec i) j == vec.(mk_int (v i * 8 + v j)))
          [SMTPat (u8_to_bv (to_u8x16 vec i) j)]

val i32_to_bv_to_i32x8_inv (vec: bv256) (i: u64 {v i < 8}) (j: u64 {v j < 32})
  : Lemma (i32_to_bv (to_i32x8 vec i) j == vec.(mk_int (v i * 32 + v j)))
         [SMTPat (i32_to_bv (to_i32x8 vec i) j)]

val mm256_bsrli_epi128_lemma (shift: i32 {v shift >= 0}) vector i
  : Lemma (  (I.mm256_bsrli_epi128 shift vector).(i)
          == (
               let lane = v i / 128 in
               let local_index = v i % 128 in
               let shift = v shift * 8 in
               let j = local_index + shift in
               if j < 0 || j >= 128 then Bit_Zero else vector.(mk_int (lane * 128 + j))
             )
          )
          [SMTPat (I.mm256_bsrli_epi128 shift vector).(i)]

val mm256_permutevar8x32_epi32_lemma vector control i
  : Lemma (  (I.mm256_permutevar8x32_epi32 vector control).(i)
          == (
               let nth_i32 = i /! mk_int 32 in
               let local_index = v i % 32 in
               let nth_block = v (to_i32x8 control nth_i32) % 8 in
               vector.(mk_int (nth_block * 32 + local_index))
             )
          )
    [SMTPat (I.mm256_permutevar8x32_epi32 vector control).(i)]

val mm256_srlv_epi32_bv_lemma
    (vector: bv256)
    (shifts: bv256)
    (i: u64 {v i < 256})
  : Lemma
    (ensures
      (I.mm256_srlv_epi32 vector shifts).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 32 in
        let v_SHIFTS = mk_u64 8 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let shift = if nth_chunk <. v_SHIFTS then v (to_i32x8 shifts nth_chunk) else 0 in
        let local_index = v nth_bit + shift in
        if local_index < v v_CHUNK && local_index >= 0
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero)
      )
      [SMTPat (I.mm256_srlv_epi32 vector shifts).(i)]

val mm_sllv_epi32_bv_lemma vector shifts i
  : Lemma
    (ensures
      (I.mm_sllv_epi32 vector shifts).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 32 in
        let v_SHIFTS = mk_u64 8 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let shift = if nth_chunk <. v_SHIFTS then v (to_i32x4 shifts nth_chunk) else 0 in
        let local_index = v nth_bit - shift in
        if local_index < v v_CHUNK && local_index >= 0
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero)
      )
      [SMTPat (I.mm_sllv_epi32 vector shifts).(i)]

val mm256_sllv_epi32_bv_lemma
    (vector: bv256)
    (shifts: bv256)
    (i: u64 {v i < 256})
  : Lemma
    (ensures
      (I.mm256_sllv_epi32 vector shifts).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 32 in
        let v_SHIFTS = mk_u64 8 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let shift = if nth_chunk <. v_SHIFTS then v (to_i32x8 shifts nth_chunk) else 0 in
        let local_index = v nth_bit - shift in
        if local_index < v v_CHUNK && local_index >= 0
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero)
      )
      [SMTPat (I.mm256_sllv_epi32 vector shifts).(i)]

val mm256_srlv_epi64_bv_lemma
    (vector: bv256)
    (shifts: bv256)
    (i: u64 {v i < 256})
  : Lemma
    (ensures
      (I.mm256_srlv_epi64 vector shifts).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 64 in
        let v_SHIFTS = mk_u64 4 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let shift = if nth_chunk <. v_SHIFTS then v (to_i64x4 shifts nth_chunk) else 0 in
        let local_index = v nth_bit + shift in
        if local_index < v v_CHUNK && local_index >= 0
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero)
      )
      [SMTPat (I.mm256_srlv_epi64 vector shifts).(i)]

val mm256_srli_epi64_bv_lemma
    (shift: i32 {v shift > 0 && v shift < 64})
    (vector: bv256)
    (i: u64 {v i < 256})
  : Lemma
    (ensures
      (I.mm256_srli_epi64 shift vector).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 64 in
        let v_SHIFTS = mk_u64 4 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let local_index = v nth_bit + v shift in
        if local_index < 64
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero
       )
    )
      [SMTPat (I.mm256_srli_epi64 shift vector).(i)]

val mm256_slli_epi64_bv_lemma
    (shift: i32 {v shift > 0 && v shift < 64})
    (vector: bv256)
    (i: u64 {v i < 256})
  : Lemma
    (ensures
      (I.mm256_slli_epi64 shift vector).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 64 in
        let v_SHIFTS = mk_u64 4 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let local_index = v nth_bit - v shift in
        if local_index >= 0
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero
       )
    )
    [SMTPat (I.mm256_slli_epi64 shift vector).(i)]

val mm_srli_epi64_bv_lemma
    (shift: i32 {v shift > 0 && v shift < 64})
    (vector: bv128)
    (i: u64 {v i < 128})
  : Lemma
    (ensures
      (I.mm_srli_epi64 shift vector).(i) ==
       (let i:u64 = i in
        let v_CHUNK = mk_u64 64 in
        let v_SHIFTS = mk_u64 4 in
        let nth_bit:u64 = i %! v_CHUNK in
        let nth_chunk:u64 = i /! v_CHUNK in
        let local_index = v nth_bit + v shift in
        if local_index < 64
        then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
        else Bit_Zero
       )
    )
      [SMTPat (I.mm_srli_epi64 shift vector).(i)]

val i16_mul_32extended_bv_lemma (x: i16) (shift: i32 {v shift >= 0 && v shift < 16}) (i: u64 { v i < 32 })
  : Lemma ( i32_to_bv (x `i16_mul_32extended` (mk_i16 1 <<! shift)) i
         == ( let j = v i - v shift in
              if j >= 0 && j < 16 then i16_to_bv x (mk_int j) else Bit_Zero
           ))
  [SMTPat (i32_to_bv (x `i16_mul_32extended` (mk_i16 1 <<! shift)) i)]

val i16_mul_32extended_bv_lemma1 (x: i16) (i: u64 { v i < 32 })
  : Lemma ( i32_to_bv (x `i16_mul_32extended` mk_i16 0) i == Bit_Zero)
  [SMTPat (i32_to_bv (x `i16_mul_32extended` mk_i16 0) i)]

let i16_mul_32extended_bv_lemma0 (x: i16) (i: u64 { v i < 32 })
  : Lemma ( i32_to_bv (x `i16_mul_32extended` mk_i16 1) i == (if v i < 16 then i16_to_bv x (mk_int (v i)) else Bit_Zero))
  [SMTPat (i32_to_bv (x `i16_mul_32extended` (mk_i16 1)) i)]
  = i16_mul_32extended_bv_lemma x (mk_int 0) i

val i16_mul_32extendedi16_bv_lemma (x: i16) (shift: i32 {v shift >= 0 && v shift < 16}) (i: u64 { v i < 16 })
  : Lemma ( i16_to_bv (x `i16_mul_32extended_i16` (mk_i16 1 <<! shift)) i
         == (if v i < v shift then Bit_Zero else i16_to_bv x (mk_int (v i - v shift))))
  [SMTPat (i16_to_bv (x `i16_mul_32extended_i16` (mk_i16 1 <<! shift)) i)]

val mm256_madd_epi16_lemma (a b: bv256) i
  : Lemma (  (I.mm256_madd_epi16 a b).(i)
          == (
               let nth_32_block = v i / 32 in
               let x = (to_i16x16 a (mk_int (nth_32_block * 2)) `i16_mul_32extended` to_i16x16 b (mk_int (nth_32_block * 2))) in
               let y = (to_i16x16 a (mk_int (nth_32_block * 2 + 1)) `i16_mul_32extended` to_i16x16 b (mk_int (nth_32_block * 2 + 1))) in
               i32_to_bv (x `i32_wrapping_add` y) (i %! mk_int 32)
             )
   )

// Note: this lemma has no SMTPat: we usually want more specialized version of this lemma.
// For example, a lemma that requires this `Bit_Zero? .. \/ Bit_Zero? ..` precondition, but for every index: such a precondition is stronger that needed but easier for the SMT.
val mm256_add_epi64_lemma lhs rhs (i: u64 {v i < 256})
  : Lemma
    (requires forall (j:nat{j < v i % 64}). Bit_Zero? lhs.(mk_int ((v i / 64) * 64 + j)) 
                                   \/ Bit_Zero? rhs.(mk_int ((v i / 64) * 64 + j)))
    (ensures (Bit_Zero? lhs.(i) ==> (I.mm256_add_epi64 lhs rhs).(i) == rhs.(i))
           /\ (Bit_Zero? rhs.(i) ==> (I.mm256_add_epi64 lhs rhs).(i) == lhs.(i)))

val mm256_madd_epi16_specialized_lemma vec i:
  Lemma
  (requires
    (forall (i: nat{i < 256}).
      let nth_32_block = i / 32 in
      let local_32 = i / 32 in
      match nth_32_block with
      | 0 | 4 ->
         Bit_Zero? vec.(mk_int (nth_32_block * 32 + local_32))
        \/ Bit_Zero? vec.(mk_int (nth_32_block * 32 + 16 + local_32 - 6))
      | _ -> True
    )
  )
  (ensures
    (I.mm256_madd_epi16 vec (I.mm256_set_epi16 (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 0)
          (mk_i16 0) (mk_i16 0) (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1) (mk_i16 0) (mk_i16 0)
          (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1))).(i)
    == (
      let nth_32_block = v i / 32 in
      let local_32 = v i % 32 in
      match nth_32_block with
      | 0 | 4 ->
        if local_32 < 6
        then vec.(mk_int (nth_32_block * 32 + local_32))
        else vec.(mk_int (nth_32_block * 32 + 16 + local_32 - 6))
      | _ -> Bit_Zero
    )

  )
  [SMTPat ((I.mm256_madd_epi16 vec (I.mm256_set_epi16 (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 0)
          (mk_i16 0) (mk_i16 0) (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1) (mk_i16 0) (mk_i16 0)
          (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1))).(i))]

val i32_to_bv_add_bv_lemma x y i
  : Lemma (requires forall j. Bit_Zero? (i32_to_bv x j) \/ Bit_Zero? (i32_to_bv y j))
          (ensures
            (Bit_Zero? (i32_to_bv x i) ==> i32_to_bv (x `i32_wrapping_add` y) i == i32_to_bv y i) /\
            (Bit_Zero? (i32_to_bv y i) ==> i32_to_bv (x `i32_wrapping_add` y) i == i32_to_bv x i)
          )
    [SMTPat (i32_to_bv (x `i32_wrapping_add` y) i)]

val pow2_lemma shift i
  : Lemma (i32_to_bv (mk_i32 1 <<! shift) i == (if v shift = v i then Bit_One else Bit_Zero))
    [SMTPat (i32_to_bv (mk_i32 1 <<! shift) i)]

val mm256_set_epi8_lemma
  (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31: i8)
  (i: u64 {v i < 32})
  : Lemma (
    to_i8x32 (I.mm256_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31) i
    == (match v i with
       | 31 -> b0 | 30 -> b1 | 29 -> b2 | 28 -> b3 | 27 -> b4 | 26 -> b5 | 25 -> b6 | 24 -> b7
       | 23 -> b8 | 22 -> b9 | 21 -> b10 | 20 -> b11 | 19 -> b12 | 18 -> b13 | 17 -> b14 | 16 -> b15
       | 15 -> b16 | 14 -> b17 | 13 -> b18 | 12 -> b19 | 11 -> b20 | 10 -> b21 | 9 -> b22 | 8 -> b23
       | 7 -> b24 | 6 -> b25 | 5 -> b26 | 4 -> b27 | 3 -> b28 | 2 -> b29 | 1 -> b30 | 0 -> b31
       )
  )
  [SMTPat (to_i8x32 (I.mm256_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31) i)]

val mm_set_epi8_lemma
  (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15: i8)
  (i: u64 {v i < 16})
  : Lemma (
    to_i8x16 (I.mm_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15) i
    == (match v i with
       | 15 -> b0 | 14 -> b1 | 13 -> b2 | 12 -> b3 | 11 -> b4 | 10 -> b5 | 9 -> b6 | 8 -> b7
       | 7 -> b8 | 6 -> b9 | 5 -> b10 | 4 -> b11 | 3 -> b12 | 2 -> b13 | 1 -> b14 | 0 -> b15
       )
  )
  [SMTPat (to_i8x16 (I.mm_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15) i)]
  

val mm256_set_epi16_lemma
  (v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15: i16)
  (i: u64 {v i < 16})
  : Lemma (
    to_i16x16 (I.mm256_set_epi16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15) i
    == (match v i with
       | 15 -> v0 | 14 -> v1 | 13 -> v2 | 12 -> v3 | 11 -> v4 | 10 -> v5 | 9 -> v6 | 8 -> v7
       | 7 -> v8 | 6 -> v9 | 5 -> v10 | 4 -> v11 | 3 -> v12 | 2 -> v13 | 1 -> v14 | 0 -> v15
       )
  )
  [SMTPat (to_i16x16 (I.mm256_set_epi16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15) i)]

val mm256_shuffle_epi8_lemma
  (vec: bv256) indexes i: Lemma (
       (I.mm256_shuffle_epi8 vec indexes).(i)
    == (
         let nth = i /! mk_int 8 in
         let index = to_i8x32 indexes nth in
         if v index < 0
         then Bit_Zero
         else (
           let index = v index % 16 in
           vec.(mk_int ((if v i < 128 then 0 else 128) + index * 8 + v i % 8))
         )
       )
  )
  [SMTPat (I.mm256_shuffle_epi8 vec indexes).(i)]

val mm_shuffle_epi8_lemma vec indexes i: Lemma (
       (I.mm_shuffle_epi8 vec indexes).(i)
    == (
         let nth = i /! mk_int 8 in
         let index = to_i8x16 indexes nth in
         if v index < 0
         then Bit_Zero
         else (
           let index = v index % 16 in
           vec.(mk_int (index * 8 + v i % 8))
         )
       )
  )
  [SMTPat (I.mm_shuffle_epi8 vec indexes).(i)]

val mm256_mullo_epi16_bv_lemma a b i: Lemma (
    let nth_bit = v i % 16 in
    let nth_i16 = v i / 16 in
    (I.mm256_mullo_epi16 a b).(i) ==
    i16_to_bv (to_i16x16 a (mk_int nth_i16) `i16_mul_32extended_i16` to_i16x16 b (mk_int nth_i16)) (mk_int nth_bit)
  )
  [SMTPat (I.mm256_mullo_epi16 a b).(i)]

let mm256_shuffle_epi32_index (a:i32) (i:u64{v i<4}) : u64 =
  cast ((a >>! (i *! mk_u64 2 <: u64) <: i32) %! mk_i32 4 <: i32) <: u64

val mm256_shuffle_epi32_lemma (a:i32) (b:bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 a b) i ==
       (if i <. mk_u64 4 <: bool
        then (to_i32x8 b (mm256_shuffle_epi32_index a i))
        else (to_i32x8 b (mk_u64 4 +! mm256_shuffle_epi32_index a (i -! mk_u64 4)))))
        [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 a b) i)]

val mm256_sub_epi32_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_sub_epi32 a b) i ==
         sub_mod_opaque (to_i32x8 a i) (to_i32x8 b i))
         [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_sub_epi32 a b) i)]

val mm256_add_epi32_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_add_epi32 a b) i ==
         add_mod_opaque (to_i32x8 a i) (to_i32x8 b i))
         [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_add_epi32 a b) i)]

// Check this definition, especially when a * b < -pow2 31
val mm256_mullo_epi32_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_mullo_epi32 a b) i ==
         mul_mod_opaque (to_i32x8 a i) (to_i32x8 b i))
         [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_mullo_epi32 a b) i)]

val mm256_mul_epi32_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_mul_epi32 a b) i ==
         (
           let j = mk_u64 (v i - (v i % 2)) in
           let v64 = mul_mod_opaque (cast_mod_opaque (to_i32x8 a j) <: i64) (cast_mod_opaque (to_i32x8 b j) <: i64) in
           if v i % 2 = 0 then cast_mod_opaque v64 else cast_mod_opaque (shift_right_opaque v64 (mk_i32 32))
         ))
         [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_mul_epi32 a b) i)]


val mm256_srai_epi32_lemma (v_IMM8: i32) (a: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_srai_epi32 v_IMM8 a) i ==
         (
         let imm8:i32 = Core_models.Num.impl_i32__rem_euclid v_IMM8 (mk_i32 256) in
         if imm8 >. mk_i32 31
         then if (to_i32x8 a i) <. mk_i32 0 then mk_i32 (-1) else mk_i32 0
         else shift_right_opaque (to_i32x8 a i) imm8
         ))
         [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_srai_epi32 v_IMM8 a) i)]

val mm256_slli_epi32_lemma (v_IMM8: i32) (a: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_slli_epi32 v_IMM8 a) i ==
         (
         if v_IMM8 <. mk_i32 0 || v_IMM8 >. mk_i32 31
         then mk_i32 0
         else shift_left_opaque (to_i32x8 a i) v_IMM8
         ))
         [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_slli_epi32 v_IMM8 a) i)]

val mm256_and_si256_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_and_si256 a b) i ==
         ((to_i32x8 a i) &. (to_i32x8 b i)))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_and_si256 a b) i)]

val mm256_xor_si256_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_xor_si256 a b) i ==
         ((to_i32x8 a i) ^. (to_i32x8 b i)))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_xor_si256 a b) i)]

val mm256_abs_epi32_lemma (a: bv256) (i:u64{v i < 8}):
  Lemma (requires (v (to_i32x8 a i) > minint i32_inttype))
        (ensures to_i32x8 (Libcrux_intrinsics.Avx2.mm256_abs_epi32 a) i ==
                 abs_int (to_i32x8 a i))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_abs_epi32 a) i)]

val mm256_cmpgt_epi32_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_cmpgt_epi32 a b) i ==
         (if (to_i32x8 a i >. to_i32x8 b i) then ones else zero))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_cmpgt_epi32 a b) i)]

val mm256_testz_si256_lemma (a b: bv256):
  Lemma (let result = Libcrux_intrinsics.Avx2.mm256_testz_si256 a b in
         let conjunct = Libcrux_intrinsics.Avx2.mm256_and_si256 a b in
         ((result == mk_i32 0 \/ result == mk_i32 1) /\
          (result == mk_i32 1 <==> (forall i. to_i32x8 conjunct i == mk_i32 0))))
  [SMTPat (Libcrux_intrinsics.Avx2.mm256_testz_si256 a b)]

val mm256_set_epi64x_lemma x0 x1 x2 x3 i
  : Lemma (  to_i64x4 (I.mm256_set_epi64x x0 x1 x2 x3) i
          == (match v i with | 0 -> x3 | 1 -> x2 | 2 -> x1 | 3 -> x0))
          [SMTPat (to_i64x4 (I.mm256_set_epi64x x0 x1 x2 x3) i)]

val mm256_set_epi64x_bv_lemma x0 x1 x2 x3 i
  : Lemma ( (I.mm256_set_epi64x x0 x1 x2 x3).(i)
         == i64_to_bv
                (match v i / 64 with | 0 -> x3 | 1 -> x2 | 2 -> x1 | 3 -> x0)
                (i %! mk_int 64)
          )
          [SMTPat (I.mm256_set_epi64x x0 x1 x2 x3).(i)]

val mm256_set_epi32_lemma (x0 x1 x2 x3 x4 x5 x6 x7:i32) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_set_epi32 x0 x1 x2 x3 x4 x5 x6 x7) i ==
        (match v i with
        | 0 -> x7 | 1 -> x6  | 2 -> x5 | 3 -> x4
        | 4 -> x3 | 5 -> x2 | 6 -> x1 | 7 -> x0
        ))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_set_epi32 x0 x1 x2 x3 x4 x5 x6 x7) i)]

val mm256_set1_epi32_lemma (x0:i32) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_set1_epi32 x0) i == x0)
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_set1_epi32 x0) i)]

val mm256_set1_epi32_bv_lemma (x0:i32) (i:u64{v i < 256}):
  Lemma ((Libcrux_intrinsics.Avx2.mm256_set1_epi32 x0).(i) == i32_to_bv x0 (i %! mk_int 32))
  [SMTPat ((Libcrux_intrinsics.Avx2.mm256_set1_epi32 x0).(i))]

val mm256_set_epi32_bv_lemma (x0 x1 x2 x3 x4 x5 x6 x7:i32) (i:u64{v i < 256}):
  Lemma ((Libcrux_intrinsics.Avx2.mm256_set_epi32 x0 x1 x2 x3 x4 x5 x6 x7).(i) ==
        i32_to_bv (match v i / 32 with
        | 0 -> x7 | 1 -> x6  | 2 -> x5 | 3 -> x4
        | 4 -> x3 | 5 -> x2 | 6 -> x1 | 7 -> x0
        ) (mk_int (v i % 32)))
  [SMTPat ((Libcrux_intrinsics.Avx2.mm256_set_epi32 x0 x1 x2 x3 x4 x5 x6 x7).(i))]

val mm_set_epi32_lemma (x0 x1 x2 x3:i32) (i:u64{v i < 4}):
  Lemma (to_i32x4 (Libcrux_intrinsics.Avx2.mm_set_epi32 x0 x1 x2 x3) i ==
        (match v i with | 0 -> x3 | 1 -> x2 | 2 -> x1 | 3 -> x0))
  [SMTPat (to_i32x4 (Libcrux_intrinsics.Avx2.mm_set_epi32 x0 x1 x2 x3) i)]

val mm256_blend_epi32_lemma (imm8: i32) (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_blend_epi32 imm8 a b) i ==
        (if (v imm8 / pow2 (v i)) % 2 = 0
         then to_i32x8 a i
         else to_i32x8 b i))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_blend_epi32 imm8 a b) i)]


val mm256_set_m128i_bv_lemma (hi lo: bv128) (i: u64 {v i < 256}):
  Lemma ((Libcrux_intrinsics.Avx2.mm256_set_m128i hi lo).(i) ==
         (if v i < 128 then lo.(i) else hi.(mk_int (v i - 128)))
        )
  [SMTPat ((Libcrux_intrinsics.Avx2.mm256_set_m128i hi lo).(i))]

val mm256_set_m128i_lemma (hi lo: bv128) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_set_m128i hi lo) i ==
        (match i <: u64 with
        | Rust_primitives.Integers.MkInt 0 -> to_i32x4 lo (mk_u64 0)
        | Rust_primitives.Integers.MkInt 1 -> to_i32x4 lo (mk_u64 1)
        | Rust_primitives.Integers.MkInt 2 -> to_i32x4 lo (mk_u64 2)
        | Rust_primitives.Integers.MkInt 3 -> to_i32x4 lo (mk_u64 3)
        | Rust_primitives.Integers.MkInt 4 -> to_i32x4 hi (mk_u64 0)
        | Rust_primitives.Integers.MkInt 5 -> to_i32x4 hi (mk_u64 1)
        | Rust_primitives.Integers.MkInt 6 -> to_i32x4 hi (mk_u64 2)
        | Rust_primitives.Integers.MkInt 7 -> to_i32x4 hi (mk_u64 3)
        | _ ->
          Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic "internal error: entered unreachable code"

              <:
              Rust_primitives.Hax.t_Never)
          <:
          i32))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_set_m128i hi lo) i)]

val mm256_permute2x128_si256_lemma_i32x4 (imm8: i32) (a b: bv256) (j:u64{v j < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_permute2x128_si256 imm8 a b) j ==
        (
          let i:u64 = j /! mk_int 4 in
          let offset = v j % 4 in
          let control:i32 = imm8 >>! (i *! mk_u64 4 <: u64) in
          if ((control >>! mk_i32 3 <: i32) %! mk_i32 2 <: i32) =. mk_i32 1
          then mk_i32 0
          else
            match control %! mk_i32 4 <: i32 with
            | Rust_primitives.Integers.MkInt 0 -> to_i32x8 a (mk_u64 (0 + offset))
            | Rust_primitives.Integers.MkInt 1 -> to_i32x8 a (mk_u64 (4 + offset))
            | Rust_primitives.Integers.MkInt 2 -> to_i32x8 b (mk_u64 (0 + offset))
            | Rust_primitives.Integers.MkInt 3 -> to_i32x8 b (mk_u64 (4 + offset))
            )
        )
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_permute2x128_si256 imm8 a b) j)]

val mm256_castsi256_si128_lemma (a: bv256) (i:u64{v i < 4}):
  Lemma (to_i32x4 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 a) i == to_i32x8 a i)
  [SMTPat (to_i32x4 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 a) i)]

val mm256_unpacklo_epi64_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_unpacklo_epi64 a b) i ==
       (match i <: u64 with
        | Rust_primitives.Integers.MkInt 0 -> (to_i32x8 a) (mk_u64 0)
        | Rust_primitives.Integers.MkInt 1 -> (to_i32x8 a) (mk_u64 1)
        | Rust_primitives.Integers.MkInt 2 -> (to_i32x8 b) (mk_u64 0)
        | Rust_primitives.Integers.MkInt 3 -> (to_i32x8 b) (mk_u64 1)
        | Rust_primitives.Integers.MkInt 4 -> (to_i32x8 a) (mk_u64 4)
        | Rust_primitives.Integers.MkInt 5 -> (to_i32x8 a) (mk_u64 5)
        | Rust_primitives.Integers.MkInt 6 -> (to_i32x8 b) (mk_u64 4)
        | Rust_primitives.Integers.MkInt 7 -> (to_i32x8 b) (mk_u64 5)
        | _ ->
          Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic "internal error: entered unreachable code"

              <:
              Rust_primitives.Hax.t_Never)
          <:
          i32))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_unpacklo_epi64 a b) i)]

val mm256_unpackhi_epi64_lemma (a b: bv256) (i:u64{v i < 8}):
  Lemma (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_unpackhi_epi64 a b) i ==
       (match i <: u64 with
        | Rust_primitives.Integers.MkInt 0 -> (to_i32x8 a) ( mk_u64 2 )
        | Rust_primitives.Integers.MkInt 1 -> (to_i32x8 a) ( mk_u64 3 )
        | Rust_primitives.Integers.MkInt 2 -> (to_i32x8 b) ( mk_u64 2 )
        | Rust_primitives.Integers.MkInt 3 -> (to_i32x8 b) ( mk_u64 3 )
        | Rust_primitives.Integers.MkInt 4 -> (to_i32x8 a) ( mk_u64 6 )
        | Rust_primitives.Integers.MkInt 5 -> (to_i32x8 a) ( mk_u64 7 )
        | Rust_primitives.Integers.MkInt 6 -> (to_i32x8 b) ( mk_u64 6 )
        | Rust_primitives.Integers.MkInt 7 -> (to_i32x8 b) ( mk_u64 7 )
        | _ ->
          Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic "internal error: entered unreachable code"

              <:
              Rust_primitives.Hax.t_Never)
          <:
          i32))
  [SMTPat (to_i32x8 (Libcrux_intrinsics.Avx2.mm256_unpackhi_epi64 a b) i)]


val mm_loadu_si128_lemma (bytes: _{ Seq.length bytes = 16 }) i
  : Lemma ((I.mm_loadu_si128 bytes).(i) == (u8_to_bv (Seq.index bytes ((v i) / 8)))(mk_int ((v i) % 8)) )
  [SMTPat (((I.mm_loadu_si128 bytes).(i)))]

val i32_lt_pow2_n_to_bit_zero_lemma n vec
  : Lemma (forall i. v (to_i32x8 vec (i /! mk_int 32)) <= normalize_term (pow2 n - 1)
                ==> v i % 32 >= n
                ==> vec.(i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero)

val shl_casted_u8_bv_lemma (a b: u8) (i: u64 {v i < 32})
  : Lemma ( i32_to_bv (((cast b <: i32) <<! mk_i32 8 <: i32) |. (cast a <: i32) <: i32) i
        == (if v i >= 16 then Libcrux_core_models.Abstractions.Bit.Bit_Zero
                        else if v i >= 8 then u8_to_bv b (i -! mk_int 8) else u8_to_bv a i))
  [SMTPat (i32_to_bv (((cast b <: i32) <<! mk_i32 8 <: i32) |. (cast a <: i32) <: i32) i)]

val i32_to_bv_cast_lemma (a: u8) (i: _{v i < 32})
  : Lemma (i32_to_bv (cast a) i == (if v i >= 8 then Libcrux_core_models.Abstractions.Bit.Bit_Zero else u8_to_bv a i))
  [SMTPat (i32_to_bv (cast a) i)]

#push-options "--z3rlimit 80"
val i32_to_bv_pow2_min_one_lemma (n: nat {n > 1 /\ n < 31}) (i:u64{v i < 32}):
  Lemma (  i32_to_bv ((mk_i32 1 <<! mk_i32 n <: i32) -! mk_i32 1) i
        == Libcrux_core_models.Abstractions.Bit.(if v i < n then Bit_One else Bit_Zero))
        [SMTPat (i32_to_bv ((mk_i32 1 <<! mk_i32 n <: i32) -! mk_i32 1) i)]
let i32_to_bv_pow2_min_one_lemma_fa (n: nat {n > 1 /\ n < 31}):
  Lemma (forall (i:u64{v i < 32}). i32_to_bv ((mk_i32 1 <<! mk_i32 n <: i32) -! mk_i32 1) i
        == Libcrux_core_models.Abstractions.Bit.(if v i < n then Bit_One else Bit_Zero))
  = ()
#pop-options

val i32_bit_zero_lemma_to_lt_pow2_n_weak (n: nat) vec
  : Lemma (requires forall i. v i % 32 >= n ==> vec.(i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero)
          (ensures forall i. v (to_i32x8 vec i) < pow2 n /\ (n <= 31 ==> v (to_i32x8 vec i) >= 0))

val i32_bit_zero_lemma_to_positive vec
  : Lemma (requires forall i. v i % 32 == 31 ==> vec.(i) == Libcrux_core_models.Abstractions.Bit.Bit_Zero)
          (ensures forall i. v (to_i32x8 vec i) >= 0)

#push-options "--z3rlimit 80"
let rewrite_eq_sub_mod (#t: inttype {signed t}) (a b c: int_t t)
  : Lemma (requires v a > minint t /\ a == b `sub_mod_opaque` c)
          (ensures  neg a `add_mod_opaque` b == c)
          = reveal_opaque_arithmetic_ops #t
#pop-options

val to_i32x8_eq_to_bv_eq (a b: bv256)
  : Lemma (requires forall i. to_i32x8 a i == to_i32x8 b i)
          (ensures a == b)

val lemma_from_i32x8_def_pt (f: (i:u64{v i < 8}) -> i32): Lemma (forall i. to_i32x8 (from_i32x8 (FStar.FunctionalExtensionality.on (i:u64{v i < 8}) f)) i == f i)

let mk_i32x8 (f: (i:u64{v i < 8}) -> i32): r: bv256 {forall i. to_i32x8 r i == f i}
 = lemma_from_i32x8_def_pt f;
   from_i32x8 (FStar.FunctionalExtensionality.on (n:u64{v n < 8}) f)
