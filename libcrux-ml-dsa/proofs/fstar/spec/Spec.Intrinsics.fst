module Spec.Intrinsics
open FStar.Mul
open Core_models
open Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
open FStar.FunctionalExtensionality
open Libcrux_core_models.Abstractions.Bit

module I      = Libcrux_intrinsics.Avx2
module Canon  = Libcrux_core_models.Intrinsics_views
module IV     = Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp
module Avx2c  = Libcrux_core_models.Core_arch.X86.Avx2
module Avxc   = Libcrux_core_models.Core_arch.X86.Avx
module Funarr = Libcrux_core_models.Abstractions.Funarr
module BV     = Libcrux_core_models.Abstractions.Bitvec
module Ints   = Rust_primitives.Integers
module Extra  = Libcrux_core_models.Core_arch.X86.Extra

#set-options "--fuel 1 --ifuel 1 --z3rlimit 60"

(* ====================================================================
   Spec.Intrinsics.fst — implementations of the 98 vals in the .fsti.
   Views are ._0 field-projections of the canonical core-models record
   views (`Libcrux_core_models.Intrinsics_views` = Canon); op-lemmas are
   discharged against Canon's proven lifts over the differentially-tested
   `Int_vec.Lemmas`.  See Spec.Intrinsics.fsti for the statements.
   Interface-val order MUST match the .fsti (F* enforces it).
   ==================================================================== *)

(**** (#1-7) Bit vec -> int vec view adapters. *)
let to_i8x32  (x: bv256): i8x32  = (Canon.to_i8x32 x)._0
let to_i8x16  (x: bv128): i8x16  = (Canon.to_i8x16 x)._0
let to_i16x16 (x: bv256): i16x16 = (Canon.to_i16x16 x)._0
let to_i32x4  (x: bv128): i32x4  = (Canon.to_i32x4 x)._0
let to_i32x8  (x: bv256): i32x8  = (Canon.to_i32x8 x)._0
let to_i64x4  (x: bv256): i64x4  = (Canon.to_i64x4 x)._0
let to_u8x16  (x: bv128): u8x16  = (IVi.e_ee_17__impl__to_u8x16 x)._0

(**** (#8) Int vec -> bit vec *)
let from_i32x8 (x:i32x8):  bv256 = Canon.from_i32x8 (Funarr.FunArray x)

(**** (#9-12) Int -> bit vecs: bit j of the two's-complement rep of the scalar. *)
let i16_to_bv (x: i16): t_FunArray (mk_int 16) t_Bit =
  on (i:u64{v i < 16}) (fun i -> IVi.encode_bit Ints.I16 x (v i))
let i32_to_bv (x: i32): t_FunArray (mk_int 32) t_Bit =
  on (i:u64{v i < 32}) (fun i -> IVi.encode_bit Ints.I32 x (v i))
let i64_to_bv (x: i64): t_FunArray (mk_int 64) t_Bit =
  on (i:u64{v i < 64}) (fun i -> IVi.encode_bit Ints.I64 x (v i))
let u8_to_bv (x: u8): t_FunArray (mk_int 8) t_Bit =
  on (i:u64{v i < 8}) (fun i -> IVi.encode_bit Ints.U8 x (v i))

(* ---- foundational helpers (private; not interface vals) ---- *)

let bval_inj (a b: t_Bit) : Lemma (requires IVi.bval a == IVi.bval b) (ensures a == b) = ()

let tc_of_u_mod (t: Ints.inttype) (u: nat{u < pow2 (Ints.bits t)})
  : Lemma (IVi.tc_of_u t u % pow2 (Ints.bits t) == u) =
  let p = pow2 (Ints.bits t) in
  if Ints.signed t && u >= pow2 (Ints.bits t - 1)
  then (FStar.Math.Lemmas.lemma_mod_plus u (-1) p; FStar.Math.Lemmas.small_mod u p)
  else FStar.Math.Lemmas.small_mod u p

#push-options "--fuel 1 --ifuel 2 --z3rlimit 120"
(* encode_bit inverts decode_lane pointwise on a bit function. *)
let lemma_encode_decode_bit (t: Ints.inttype) (f: nat -> t_Bit) (b: nat{b < Ints.bits t})
  : Lemma (IVi.encode_bit t (IVi.decode_lane t f) b == f b) =
  let n = Ints.bits t in
  IVi.dsum2_bound f 0 n;
  let u = IVi.dsum2 f 0 n in
  IVi.lemma_tc_range t u;
  tc_of_u_mod t u;
  IVi.ebit_bit u b;
  Canon.dsum2_digit f 0 n b;
  bval_inj (IVi.encode_bit t (IVi.decode_lane t f) b) (f b)
#pop-options

(* the raw bit of a bitvec at position (w*i+b) via lane_reader, for b<w in range. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 120"
let lemma_lane_reader_raw (n: u64) (w: nat) (bv: BV.t_BitVec n) (i: u64) (b: nat)
  : Lemma (requires b < w /\ w * v i + b < v n)
          (ensures IVi.lane_reader n w bv i b ==
                   Funarr.impl_5__get n #t_Bit bv._0 (mk_u64 (w * v i + b))) = ()
#pop-options

(* generic bit-view inversion: bit j of the i-th w-bit lane view == raw bit w*i+j. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let bit_view_inv (t: Ints.inttype) (n m: u64) (vec: BV.t_BitVec n)
      (i: u64{v i < v m}) (j: u64{v j < Ints.bits t})
  : Lemma (requires v n == v m * Ints.bits t)
          (ensures IVi.encode_bit t ((IVi.to_iv t n m vec)._0 i) (v j) ==
                   Funarr.impl_5__get n #t_Bit vec._0 (mk_u64 (Ints.bits t * v i + v j))) =
  reveal_opaque (`%IVi.to_iv) IVi.to_iv;
  lemma_lane_reader_raw n (Ints.bits t) vec i (v j);
  lemma_encode_decode_bit t (IVi.lane_reader n (Ints.bits t) vec i) (v j)
#pop-options

(* ---- get_bit bridge toolkit (private; not interface vals) ----
   The core-models bit codec `IVi.encode_bit t x b` has bval equal to the
   rust_primitives `get_bit x b`, so the whole get_bit algebra (get_bit_and/
   or/xor/shl/shr/cast/pow2, all SMTPat-tagged) discharges the `*_to_bv`
   arithmetic family. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let ebit_is_get_bit (t: Ints.inttype) (x: Ints.int_t t) (b: nat{b < Ints.bits t})
  : Lemma (IVi.bval (IVi.encode_bit t x b) == Ints.get_bit x (Ints.mk_usize b)) =
  let n = Ints.bits t in
  IVi.ebit_bit (Ints.v x % pow2 n) b;
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #t);
  if Ints.v x >= 0
  then FStar.Math.Lemmas.small_mod (Ints.v x) (pow2 n)
  else (FStar.Math.Lemmas.lemma_mod_plus (Ints.v x) 1 (pow2 n);
        FStar.Math.Lemmas.small_mod (Ints.v x + pow2 n) (pow2 n))

(* encode_bit is determined by get_bit: read a bit off get_bit *)
let bit_of_get_bit (t: Ints.inttype) (x: Ints.int_t t) (b: nat{b < Ints.bits t})
  : Lemma (IVi.encode_bit t x b ==
           (if Ints.get_bit x (Ints.mk_usize b) = 1 then Bit_One else Bit_Zero)) =
  ebit_is_get_bit t x b;
  bval_inj (IVi.encode_bit t x b)
           (if Ints.get_bit x (Ints.mk_usize b) = 1 then Bit_One else Bit_Zero)

(* u8 -> i32 `cast` coincides with `cast_mod` (value in range), so the
   get_bit_cast / get_bit_cast_extend SMTPats (stated on cast_mod) fire. *)
let cast_u8_i32 (a: u8) : Lemma ((cast a <: i32) == Ints.cast_mod #Ints.U8 #Ints.I32 a) = ()
let cast_u8_i8 (a: u8) : Lemma ((cast a <: i8) == Ints.cast_mod #Ints.U8 #Ints.I8 a) = ()
#pop-options

(* u8x32 view of a bv256 (private; the 256-bit analogue of to_u8x16). *)
let to_u8x32p (x: bv256) : t_FunArray (mk_u64 32) u8 = (IVi.e_ee_8__impl__to_u8x32 x)._0
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let u8_to_bv_to_u8x32_inv (vec: bv256) (i: u64{v i < 32}) (j: u64{v j < 8})
  : Lemma (u8_to_bv (to_u8x32p vec i) j == vec.(mk_int (v i * 8 + v j))) =
  bit_view_inv Ints.U8 (mk_u64 256) (mk_u64 32) vec i j
#pop-options

(* i64 analogue of i32_to_bv_to_i32x4_inv (private helper). *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let i64_to_bv_to_i64x4_inv (vec: bv256) (i: u64{v i < 4}) (j: u64{v j < 64})
  : Lemma (i64_to_bv (to_i64x4 vec i) j == vec.(mk_int (v i * 64 + v j))) =
  bit_view_inv Ints.I64 (mk_u64 256) (mk_u64 4) vec i j
#pop-options

(* i64x2 view of a bv128 (private; the Sse2 128-bit analogue of to_i64x4). *)
let to_i64x2p (x: bv128) : t_FunArray (mk_u64 2) i64 = (Canon.to_i64x2 x)._0
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let i64_to_bv_to_i64x2_inv (vec: bv128) (i: u64{v i < 2}) (j: u64{v j < 64})
  : Lemma (i64_to_bv (to_i64x2p vec i) j == vec.(mk_int (v i * 64 + v j))) =
  bit_view_inv Ints.I64 (mk_u64 128) (mk_u64 2) vec i j
#pop-options

(* ---- i64 lane <-> its two i32 sub-lanes (private; the cross-view bridge) ----
   Both views read the SAME raw bits: bit `b` of the i32 lane `2k+h` and bit
   `32*h+b` of the i64 lane `k` are both raw bit `64*k + 32*h + b`. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let lane32_bit_of_lane64 (vec: bv256) (k: u64{v k < 4}) (h: nat{h < 2}) (b: nat{b < 32})
  : Lemma (IVi.encode_bit Ints.I32 (to_i32x8 vec (mk_u64 (2 * v k + h))) b ==
           IVi.encode_bit Ints.I64 (to_i64x4 vec k) (32 * h + b)) =
  bit_view_inv Ints.I32 (mk_u64 256) (mk_u64 8) vec (mk_u64 (2 * v k + h)) (mk_u64 b);
  bit_view_inv Ints.I64 (mk_u64 256) (mk_u64 4) vec k (mk_u64 (32 * h + b))
#pop-options

(* The two i32 sub-lanes of an i64 lane ARE the truncation / high-half of it. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let lemma_i32_sub_lanes (vec: bv256) (k: u64{v k < 4})
  : Lemma (to_i32x8 vec (mk_u64 (2 * v k)) == Ints.cast_mod #Ints.I64 #Ints.I32 (to_i64x4 vec k) /\
           to_i32x8 vec (mk_u64 (2 * v k + 1)) ==
             Ints.cast_mod #Ints.I64 #Ints.I32
               (Ints.shift_right #Ints.I64 #Ints.I32 (to_i64x4 vec k) (mk_i32 32))) =
  let y  : i64 = to_i64x4 vec k in
  let lo : i32 = to_i32x8 vec (mk_u64 (2 * v k)) in
  let hi : i32 = to_i32x8 vec (mk_u64 (2 * v k + 1)) in
  let ylo : i32 = Ints.cast_mod #Ints.I64 #Ints.I32 y in
  let yhi : i32 = Ints.cast_mod #Ints.I64 #Ints.I32
                    (Ints.shift_right #Ints.I64 #Ints.I32 y (mk_i32 32)) in
  let auxlo (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit lo jj == Ints.get_bit ylo jj) =
    lane32_bit_of_lane64 vec k 0 (v jj);
    ebit_is_get_bit Ints.I32 lo (v jj);
    ebit_is_get_bit Ints.I64 y (v jj)
  in
  let auxhi (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit hi jj == Ints.get_bit yhi jj) =
    lane32_bit_of_lane64 vec k 1 (v jj);
    ebit_is_get_bit Ints.I32 hi (v jj);
    ebit_is_get_bit Ints.I64 y (32 + v jj)
  in
  FStar.Classical.forall_intro auxlo;
  Ints.lemma_int_t_eq_via_bits lo ylo;
  FStar.Classical.forall_intro auxhi;
  Ints.lemma_int_t_eq_via_bits hi yhi
#pop-options

(* ---- i128 lane <-> its four i32 sub-lanes (private) ---- *)
let to_i128x2p (x: bv256) : t_FunArray (mk_u64 2) i128 = (Canon.to_i128x2 x)._0

#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let lemma_i32_sub_lane_of_i128 (vec: bv256) (k: u64{v k < 2}) (h: nat{h < 4})
  : Lemma (to_i32x8 vec (mk_u64 (4 * v k + h)) ==
           Ints.cast_mod #Ints.I128 #Ints.I32
             (Ints.shift_right #Ints.I128 #Ints.I32 (to_i128x2p vec k) (mk_i32 (32 * h)))) =
  let y : i128 = to_i128x2p vec k in
  let l : i32 = to_i32x8 vec (mk_u64 (4 * v k + h)) in
  let r : i32 = Ints.cast_mod #Ints.I128 #Ints.I32
                  (Ints.shift_right #Ints.I128 #Ints.I32 y (mk_i32 (32 * h))) in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit l jj == Ints.get_bit r jj) =
    (* both sides read raw bit 128*k + 32*h + jj *)
    bit_view_inv Ints.I32 (mk_u64 256) (mk_u64 8) vec (mk_u64 (4 * v k + h)) (mk_u64 (v jj));
    bit_view_inv Ints.I128 (mk_u64 256) (mk_u64 2) vec k (mk_u64 (32 * h + v jj));
    ebit_is_get_bit Ints.I32 l (v jj);
    ebit_is_get_bit Ints.I128 y (32 * h + v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits l r
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 250"
let lemma_i32_from_i128_transfer (x y: bv256) (kx ky: u64{v kx < 2 /\ v ky < 2}) (h: nat{h < 4})
  : Lemma (requires to_i128x2p x kx == to_i128x2p y ky)
          (ensures to_i32x8 x (mk_u64 (4 * v kx + h)) == to_i32x8 y (mk_u64 (4 * v ky + h))) =
  lemma_i32_sub_lane_of_i128 x kx h;
  lemma_i32_sub_lane_of_i128 y ky h

let lemma_i32_of_i128_zero (x: bv256) (k: u64{v k < 2}) (h: nat{h < 4})
  : Lemma (requires to_i128x2p x k == mk_i128 0)
          (ensures to_i32x8 x (mk_u64 (4 * v k + h)) == mk_i32 0) =
  lemma_i32_sub_lane_of_i128 x k h
#pop-options

(* i64-lane equality transfers to both i32 sub-lanes. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_i32_lane_transfer (x y: bv256) (kx ky: u64{v kx < 4 /\ v ky < 4}) (h: nat{h < 2})
  : Lemma (requires to_i64x4 x kx == to_i64x4 y ky)
          (ensures to_i32x8 x (mk_u64 (2 * v kx + h)) == to_i32x8 y (mk_u64 (2 * v ky + h))) =
  lemma_i32_sub_lanes x kx;
  lemma_i32_sub_lanes y ky
#pop-options

(* ---- (#13) inversion / definitional ---- *)
let to_from_i32x8_inv_lemma x =
  IVi.lemma_conv_rt Ints.I32 (mk_u64 256) (mk_u64 8) (Funarr.FunArray x)

(* ---- (#14) ---- *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let i16_to_bv_to_i16x16_inv vec i j =
  bit_view_inv Ints.I16 (mk_u64 256) (mk_u64 16) vec i j
#pop-options

(* ---- (#15-98): admitted stubs, to be discharged batch by batch ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let mm256_castsi256_si128_bv_lemma vec i =
  reveal_opaque (`%I.mm256_castsi256_si128) I.mm256_castsi256_si128;
  Canon.lemma_bv_index vec i
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let mm256_extracti128_si256_bv_lemma control vec i =
  reveal_opaque (`%I.mm256_extracti128_si256) I.mm256_extracti128_si256;
  Canon.lemma_bv_index vec (mk_int (if v control = 0 then 0 else 128) +! i)
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_extracti128_si256_lemma control vec i =
  let off : u64 = mk_int (if v control = 0 then 0 else 4) in
  let l = to_i32x4 (I.mm256_extracti128_si256 control vec) i in
  let r = to_i32x8 vec (i +! off) in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit l jj == Ints.get_bit r jj) =
    let j = mk_u64 (v jj) in
    mm256_extracti128_si256_bv_lemma control vec (mk_u64 (v i * 32 + v jj));
    bit_view_inv Ints.I32 (mk_u64 128) (mk_u64 4) (I.mm256_extracti128_si256 control vec) i j;
    bit_view_inv Ints.I32 (mk_u64 256) (mk_u64 8) vec (i +! off) j;
    ebit_is_get_bit Ints.I32 l (v jj);
    ebit_is_get_bit Ints.I32 r (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits l r
#pop-options
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let mm256_and_si256 lhs rhs i =
  reveal_opaque (`%I.mm256_and_si256) I.mm256_and_si256;
  Canon.lemma_and_si256_lift lhs rhs;
  Canon.lemma_and_funarr lhs rhs i
#pop-options
(* ---- slice-I/O CONTENT, via the core-models `Extra` memory-op models ----
   `Libcrux_intrinsics.Avx2`'s store/load ops are `[@@ "opaque_to_smt"] let op =
   Extra.<op>_model` (their `#[cfg(hax)]` bodies), so revealing both sides turns
   each content lemma into a codec round-trip / `update_at_usize` chain readout —
   the mechanism ml-kem's `lemma_mm256_storeu_si256_i16` already uses. *)
(* The model is a 16-step `update_at_usize` (= `Seq.upd`) chain guarded by
   `len >= 16`.  Three separated contexts, because the two obligations fight:
   the `reveal_opaque` norm-equations need the UNSPLIT context (they bail under
   `--split_queries always`), while reading the chain off at a SYMBOLIC index
   needs the literal case split.  So: unfold in one lemma, dispatch in another.
   `store16` is OPAQUE: left transparent, its 16-deep `Seq.upd` chain inflates
   every later decl's context (measured: it took `mm256_mullo_epi16_bv_lemma`
   from 62 s to a >5 min grind).  Only its two consumers below reveal it. *)
[@@ "opaque_to_smt"]
let store16 (out: t_Slice u8 {Seq.length out >= 16}) (vec: bv128)
  : (r: t_Slice u8 {Seq.length r == Seq.length out}) =
  let upd = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize in
  let s0  = upd out (mk_usize 0)  (to_u8x16 vec (mk_u64 0))  in
  let s1  = upd s0  (mk_usize 1)  (to_u8x16 vec (mk_u64 1))  in
  let s2  = upd s1  (mk_usize 2)  (to_u8x16 vec (mk_u64 2))  in
  let s3  = upd s2  (mk_usize 3)  (to_u8x16 vec (mk_u64 3))  in
  let s4  = upd s3  (mk_usize 4)  (to_u8x16 vec (mk_u64 4))  in
  let s5  = upd s4  (mk_usize 5)  (to_u8x16 vec (mk_u64 5))  in
  let s6  = upd s5  (mk_usize 6)  (to_u8x16 vec (mk_u64 6))  in
  let s7  = upd s6  (mk_usize 7)  (to_u8x16 vec (mk_u64 7))  in
  let s8  = upd s7  (mk_usize 8)  (to_u8x16 vec (mk_u64 8))  in
  let s9  = upd s8  (mk_usize 9)  (to_u8x16 vec (mk_u64 9))  in
  let s10 = upd s9  (mk_usize 10) (to_u8x16 vec (mk_u64 10)) in
  let s11 = upd s10 (mk_usize 11) (to_u8x16 vec (mk_u64 11)) in
  let s12 = upd s11 (mk_usize 12) (to_u8x16 vec (mk_u64 12)) in
  let s13 = upd s12 (mk_usize 13) (to_u8x16 vec (mk_u64 13)) in
  let s14 = upd s13 (mk_usize 14) (to_u8x16 vec (mk_u64 14)) in
  upd s14 (mk_usize 15) (to_u8x16 vec (mk_u64 15))

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let storeu_bytes_unfold (out: t_Slice u8 {Seq.length out >= 16}) (vec: bv128)
  : Lemma (I.mm_storeu_bytes_si128 out vec == store16 out vec) =
  reveal_opaque (`%store16) store16;
  reveal_opaque (`%I.mm_storeu_bytes_si128) I.mm_storeu_bytes_si128;
  reveal_opaque (`%Extra.mm_storeu_bytes_si128_model) Extra.mm_storeu_bytes_si128_model
#pop-options

#restart-solver
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400 --split_queries always"
let store16_index (out: t_Slice u8 {Seq.length out >= 16}) (vec: bv128) (i: nat{i < 16})
  : Lemma (Seq.index (store16 out vec) i == to_u8x16 vec (mk_int i)) =
  reveal_opaque (`%store16) store16;
  match i with
  | 0  -> () | 1  -> () | 2  -> () | 3  -> () | 4  -> () | 5  -> () | 6  -> () | 7  -> ()
  | 8  -> () | 9  -> () | 10 -> () | 11 -> () | 12 -> () | 13 -> () | 14 -> () | _ -> ()
#pop-options

#restart-solver
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let mm_storeu_bytes_si128_lemma out vec i =
  storeu_bytes_unfold out vec;
  store16_index out vec i
#pop-options
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let update_at_range_bv_lemma f_start f_end bytes dummy_out vec i =
  Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range
    bytes ({ f_start; f_end }) (I.mm_storeu_bytes_si128 dummy_out vec);
  if i >= v f_start && i < v f_end
  then mm_storeu_bytes_si128_lemma dummy_out vec (i - v f_start)
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let u8_to_bv_to_u8x16_inv vec i j =
  bit_view_inv Ints.U8 (mk_u64 128) (mk_u64 16) vec i j
let i32_to_bv_to_i32x8_inv vec i j =
  bit_view_inv Ints.I32 (mk_u64 256) (mk_u64 8) vec i j
#pop-options
(* Byte-shift right within each 128-bit lane. With the (call-site-satisfied)
   precondition `0 <= v shift < 256`, `rem_euclid v_IMM8 256 == v shift`, so the
   model's `tmp > 15 -> 0` guard and its u128 `>> (tmp*8)` logical shift coincide
   with the axiom's `j >= 128 -> Bit_Zero` byte-shift; the bit is read off via the
   i128 lift + get_bit_shr/get_bit_cast, exactly as the immediate 64-bit shifts. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_bsrli_epi128_lemma shift vector i =
  reveal_opaque (`%I.mm256_bsrli_epi128) I.mm256_bsrli_epi128;
  Canon.lemma_rem_euclid256 shift;
  let lane = i /! mk_u64 128 in
  let bit  = i %! mk_u64 128 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 128;
  Canon.lemma_mm256_bsrli_epi128 shift vector;
  bit_view_inv Ints.I128 (mk_u64 256) (mk_u64 2) (I.mm256_bsrli_epi128 shift vector) lane bit;
  bit_of_get_bit Ints.I128 (to_i128x2p (I.mm256_bsrli_epi128 shift vector) lane) (v bit);
  if v shift <= 15 && v bit + v shift * 8 < 128
  then (bit_view_inv Ints.I128 (mk_u64 256) (mk_u64 2) vector lane (mk_u64 (v bit + v shift * 8));
        bit_of_get_bit Ints.I128 (to_i128x2p vector lane) (v bit + v shift * 8))
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_permutevar8x32_epi32_lemma vector control i =
  reveal_opaque (`%I.mm256_permutevar8x32_epi32) I.mm256_permutevar8x32_epi32;
  let lane = i /! mk_u64 32 in
  let bit  = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  Canon.lemma_mm256_permutevar8x32_epi32 vector control;
  let nth_block : u64 = mk_u64 (v (to_i32x8 control lane) % 8) in
  i32_to_bv_to_i32x8_inv (I.mm256_permutevar8x32_epi32 vector control) lane bit;
  i32_to_bv_to_i32x8_inv vector nth_block bit
#pop-options
(* Variable per-lane shifts. With the (call-site-satisfied) precondition that every
   shift lane is >= 0, the model's `b < 0 -> 0` branch is dead and the axiom's signed
   shift equals the model's unsigned count; the bit is read off via the lift +
   get_bit_shr/get_bit_cast (u32/u64 routed), exactly as the immediate shifts. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_srlv_epi32_bv_lemma vector shifts i =
  reveal_opaque (`%I.mm256_srlv_epi32) I.mm256_srlv_epi32;
  let chunk = i /! mk_u64 32 in
  let bit = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  Canon.lemma_mm256_srlv_epi32 vector shifts;
  let r = I.mm256_srlv_epi32 vector shifts in
  i32_to_bv_to_i32x8_inv r chunk bit;
  bit_of_get_bit Ints.I32 (to_i32x8 r chunk) (v bit);
  let sh : nat = v (to_i32x8 shifts chunk) in
  if v bit + sh < 32
  then (i32_to_bv_to_i32x8_inv vector chunk (mk_u64 (v bit + sh));
        bit_of_get_bit Ints.I32 (to_i32x8 vector chunk) (v bit + sh))
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm_sllv_epi32_bv_lemma vector shifts i =
  reveal_opaque (`%I.mm_sllv_epi32) I.mm_sllv_epi32;
  let chunk = i /! mk_u64 32 in
  let bit = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  Canon.lemma_mm_sllv_epi32 vector shifts;
  let r = I.mm_sllv_epi32 vector shifts in
  bit_view_inv Ints.I32 (mk_u64 128) (mk_u64 4) r chunk bit;
  bit_of_get_bit Ints.I32 (to_i32x4 r chunk) (v bit);
  let sh : nat = v (to_i32x4 shifts chunk) in
  if v bit >= sh
  then (bit_view_inv Ints.I32 (mk_u64 128) (mk_u64 4) vector chunk (mk_u64 (v bit - sh));
        bit_of_get_bit Ints.I32 (to_i32x4 vector chunk) (v bit - sh))
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_sllv_epi32_bv_lemma vector shifts i =
  reveal_opaque (`%I.mm256_sllv_epi32) I.mm256_sllv_epi32;
  let chunk = i /! mk_u64 32 in
  let bit = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  Canon.lemma_mm256_sllv_epi32 vector shifts;
  let r = I.mm256_sllv_epi32 vector shifts in
  i32_to_bv_to_i32x8_inv r chunk bit;
  bit_of_get_bit Ints.I32 (to_i32x8 r chunk) (v bit);
  let sh : nat = v (to_i32x8 shifts chunk) in
  if v bit >= sh
  then (i32_to_bv_to_i32x8_inv vector chunk (mk_u64 (v bit - sh));
        bit_of_get_bit Ints.I32 (to_i32x8 vector chunk) (v bit - sh))
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_srlv_epi64_bv_lemma vector shifts i =
  reveal_opaque (`%I.mm256_srlv_epi64) I.mm256_srlv_epi64;
  let chunk = i /! mk_u64 64 in
  let bit = i %! mk_u64 64 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 64;
  Canon.lemma_mm256_srlv_epi64 vector shifts;
  let r = I.mm256_srlv_epi64 vector shifts in
  i64_to_bv_to_i64x4_inv r chunk bit;
  bit_of_get_bit Ints.I64 (to_i64x4 r chunk) (v bit);
  let sh : nat = v (to_i64x4 shifts chunk) in
  if v bit + sh < 64
  then (i64_to_bv_to_i64x4_inv vector chunk (mk_u64 (v bit + sh));
        bit_of_get_bit Ints.I64 (to_i64x4 vector chunk) (v bit + sh))
#pop-options
(* The three 64-bit immediate shifts. All carry `0 < shift < 64` in the .fsti, so
   the model's `rem_euclid IMM8 256` is the identity and its `> 63` guard is dead:
   each lane is the LOGICAL (u64-routed) shift, whose bits are read off by the
   get_bit_shr / get_bit_shl / get_bit_cast algebra. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_srli_epi64_bv_lemma shift vector i =
  reveal_opaque (`%I.mm256_srli_epi64) I.mm256_srli_epi64;
  Canon.lemma_rem_euclid256 shift;
  let lane = i /! mk_u64 64 in
  let bit  = i %! mk_u64 64 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 64;
  Canon.lemma_mm256_srli_epi64 shift vector;
  i64_to_bv_to_i64x4_inv (I.mm256_srli_epi64 shift vector) lane bit;
  bit_of_get_bit Ints.I64 (to_i64x4 (I.mm256_srli_epi64 shift vector) lane) (v bit);
  if v bit + v shift < 64
  then (i64_to_bv_to_i64x4_inv vector lane (mk_u64 (v bit + v shift));
        bit_of_get_bit Ints.I64 (to_i64x4 vector lane) (v bit + v shift))

let mm256_slli_epi64_bv_lemma shift vector i =
  reveal_opaque (`%I.mm256_slli_epi64) I.mm256_slli_epi64;
  Canon.lemma_rem_euclid256 shift;
  let lane = i /! mk_u64 64 in
  let bit  = i %! mk_u64 64 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 64;
  Canon.lemma_mm256_slli_epi64 shift vector;
  i64_to_bv_to_i64x4_inv (I.mm256_slli_epi64 shift vector) lane bit;
  bit_of_get_bit Ints.I64 (to_i64x4 (I.mm256_slli_epi64 shift vector) lane) (v bit);
  if v bit >= v shift
  then (i64_to_bv_to_i64x4_inv vector lane (mk_u64 (v bit - v shift));
        bit_of_get_bit Ints.I64 (to_i64x4 vector lane) (v bit - v shift))

let mm_srli_epi64_bv_lemma shift vector i =
  reveal_opaque (`%I.mm_srli_epi64) I.mm_srli_epi64;
  Canon.lemma_rem_euclid256 shift;
  let lane = i /! mk_u64 64 in
  let bit  = i %! mk_u64 64 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 64;
  Canon.lemma_mm_srli_epi64 shift vector;
  i64_to_bv_to_i64x2_inv (I.mm_srli_epi64 shift vector) lane bit;
  bit_of_get_bit Ints.I64 (to_i64x2p (I.mm_srli_epi64 shift vector) lane) (v bit);
  if v bit + v shift < 64
  then (i64_to_bv_to_i64x2_inv vector lane (mk_u64 (v bit + v shift));
        bit_of_get_bit Ints.I64 (to_i64x2p vector lane) (v bit + v shift))
#pop-options
(* ---- pure-nat core for the multiply-by-2^k bit fact (gated to Prims/FStar so
   the module's get_bit / SIMD SMTPats cannot enter this nonlinear VC). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 300 --using_facts_from 'Prims FStar'"
let lemma_mul_pow2_bit_nat (rep k n: nat)
  : Lemma ((rep * pow2 k / pow2 n) % 2 == (if n >= k then (rep / pow2 (n - k)) % 2 else 0)) =
  if n >= k then begin
    FStar.Math.Lemmas.pow2_plus k (n - k);
    assert (pow2 n == pow2 k * pow2 (n - k));
    FStar.Math.Lemmas.division_multiplication_lemma (rep * pow2 k) (pow2 k) (pow2 (n - k));
    FStar.Math.Lemmas.cancel_mul_div rep (pow2 k);
    assert (rep * pow2 k / pow2 n == rep / pow2 (n - k))
  end
  else begin
    FStar.Math.Lemmas.pow2_plus (k - n) n;
    assert (pow2 k == pow2 (k - n) * pow2 n);
    FStar.Math.Lemmas.paren_mul_right rep (pow2 (k - n)) (pow2 n);
    assert (rep * pow2 k == (rep * pow2 (k - n)) * pow2 n);
    FStar.Math.Lemmas.cancel_mul_div (rep * pow2 (k - n)) (pow2 n);
    assert (rep * pow2 k / pow2 n == rep * pow2 (k - n));
    FStar.Math.Lemmas.pow2_double_mult (k - n - 1);
    assert (pow2 (k - n) == pow2 (k - n - 1) * 2);
    FStar.Math.Lemmas.paren_mul_right rep (pow2 (k - n - 1)) 2;
    FStar.Math.Lemmas.multiple_modulo_lemma (rep * pow2 (k - n - 1)) 2
  end
#pop-options
(* v (1 << shift : i16) reduced mod 2^16 is exactly 2^shift, for shift < 16
   (even shift=15, where 1<<15 wraps to -32768 == 32768 mod 2^16). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_i16_one_shl_mod (shift: i32 {v shift >= 0 && v shift < 16})
  : Lemma (v (mk_i16 1 <<! shift) % pow2 16 == pow2 (v shift)) =
  FStar.Math.Lemmas.pow2_lt_compat 16 (v shift);
  Rust_primitives.Integers.shift_left_positive_lemma (mk_i16 1) shift;
  assert_norm (pow2 16 == 65536);
  FStar.Math.Lemmas.lemma_mod_plus (pow2 (v shift)) (-1) (pow2 16);
  FStar.Math.Lemmas.small_mod (pow2 (v shift)) (pow2 16)
#pop-options
(* v (1 << shift : i16) == 2^shift EXACTLY, for shift < 15 (1<<shift positive). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_i16_one_shl_exact (shift: i32 {v shift >= 0 && v shift < 15})
  : Lemma (v (mk_i16 1 <<! shift) == pow2 (v shift)) =
  FStar.Math.Lemmas.pow2_lt_compat 15 (v shift);
  assert_norm (pow2 15 == 32768); assert_norm (pow2 16 == 65536);
  Rust_primitives.Integers.shift_left_positive_lemma (mk_i16 1) shift;
  FStar.Math.Lemmas.small_mod (pow2 (v shift)) (pow2 16)
#pop-options
(* Untruncated (i32) multiply-by-2^shift = sign-extend(x) * 2^shift.  For x >= 0 and
   shift < 15 (so 1<<shift is a POSITIVE i16) this is a clean left shift; the axiom is
   FALSE for negative x (sign extension) or shift=15 (1<<15 wraps negative), hence the
   .fsti `v x >= 0` + `v shift < 15`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let i16_mul_32extended_bv_lemma x shift i =
  reveal_opaque (`%i16_mul_32extended) i16_mul_32extended;
  assert_norm (pow2 15 == 32768); assert_norm (pow2 16 == 65536);
  let m : i16 = mk_i16 1 <<! shift in
  lemma_i16_one_shl_exact shift;                 (* v m == pow2 (v shift) *)
  let prod : i32 = x `i16_mul_32extended` m in   (* v prod == v x * pow2 (v shift), >= 0 *)
  let j : int = v i - v shift in
  bit_of_get_bit Ints.I32 prod (v i);
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32);
  lemma_mul_pow2_bit_nat (v x) (v shift) (v i);
  if j >= 0 && j < 16 then begin
    bit_of_get_bit Ints.I16 x j;
    reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I16);
    FStar.Math.Lemmas.small_mod (v x) (pow2 16)
  end
  else if j >= 16 then begin                     (* v x / 2^j == 0 since v x < 2^15 <= 2^j *)
    FStar.Math.Lemmas.pow2_le_compat j 16;
    FStar.Math.Lemmas.small_division_lemma_1 (v x) (pow2 j)
  end
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let i16_mul_32extended_bv_lemma1 x i =
  reveal_opaque (`%i16_mul_32extended) i16_mul_32extended;
  assert ((x `i16_mul_32extended` mk_i16 0) == mk_i32 0);
  bit_of_get_bit Ints.I32 (mk_i32 0) (v i);
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32)
#pop-options
(* pure-nat core for the TRUNCATED (mod-2^16) multiply-by-2^k: bit ii of
   ((xv*mv) mod 2^16) is bit (ii-sh) of (xv mod 2^16), for ii<16, given mv==2^sh
   mod 2^16.  ALL nonlinear/mod arithmetic isolated here (gated to Prims/FStar). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --using_facts_from 'Prims FStar'"
let lemma_trunc_mul_pow2 (xv mv: int) (sh ii: nat)
  : Lemma (requires mv % pow2 16 == pow2 sh /\ sh < 16 /\ ii < 16)
          (ensures (((xv * mv) % pow2 16) / pow2 ii) % 2
                   == (if ii >= sh then ((xv % pow2 16) / pow2 (ii - sh)) % 2 else 0)) =
  let n16 = pow2 16 in
  let x16 : nat = xv % n16 in
  FStar.Math.Lemmas.lemma_mod_mul_distr_r xv mv n16;
  FStar.Math.Lemmas.lemma_mod_mul_distr_l xv (pow2 sh) n16;
  assert ((xv * mv) % n16 == (x16 * pow2 sh) % n16);
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 (x16 * pow2 sh) ii 16;
  FStar.Math.Lemmas.pow2_double_mult (16 - ii - 1);
  FStar.Math.Lemmas.modulo_modulo_lemma ((x16 * pow2 sh) / pow2 ii) 2 (pow2 (16 - ii - 1));
  lemma_mul_pow2_bit_nat x16 sh ii
#pop-options
(* get_bit of an i16 as a pure-nat readout on its 2^16 representative (get_bit
   reveal isolated to this small VC). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_get_bit_i16_nat (y: i16) (j: nat {j < 16})
  : Lemma (Ints.get_bit y (Ints.mk_usize j) == get_bit_nat (v y % pow2 16) j) =
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I16);
  if v y >= 0 then FStar.Math.Lemmas.small_mod (v y) (pow2 16)
  else (FStar.Math.Lemmas.lemma_mod_plus (v y) 1 (pow2 16);
        FStar.Math.Lemmas.small_mod (pow2 16 + v y) (pow2 16))
#pop-options
(* the i16 truncated product's value mod 2^16 (cast/reveal isolated). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_i16_mul_val (x m: i16)
  : Lemma (v (i16_mul_32extended_i16 x m) % pow2 16 == (v x * v m) % pow2 16) =
  reveal_opaque (`%i16_mul_32extended_i16) i16_mul_32extended_i16;
  reveal_opaque (`%i16_mul_32extended) i16_mul_32extended
#pop-options
(* Truncated (i16) multiply-by-2^shift: bit i of (x *[i16] 2^shift) is bit (i-shift)
   of x, or 0 below shift.  True for ALL x (the mod-2^16 truncation kills the sign). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let i16_mul_32extendedi16_bv_lemma x shift i =
  let m : i16 = mk_i16 1 <<! shift in
  let res : i16 = i16_mul_32extended_i16 x m in
  lemma_i16_one_shl_mod shift;
  lemma_trunc_mul_pow2 (v x) (v m) (v shift) (v i);
  lemma_i16_mul_val x m;
  lemma_get_bit_i16_nat res (v i);
  bit_of_get_bit Ints.I16 res (v i);
  if v i >= v shift then begin
    lemma_get_bit_i16_nat x (v i - v shift);
    bit_of_get_bit Ints.I16 x (v i - v shift)
  end
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_madd_epi16_lemma a b i =
  reveal_opaque (`%I.mm256_madd_epi16) I.mm256_madd_epi16;
  reveal_opaque (`%i16_mul_32extended) i16_mul_32extended;
  reveal_opaque (`%i32_wrapping_add) i32_wrapping_add;
  let lane = i /! mk_u64 32 in
  let bit  = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  Canon.lemma_mm256_madd_epi16 a b;
  i32_to_bv_to_i32x8_inv (I.mm256_madd_epi16 a b) lane bit
#pop-options
(* ---- carry-free addition of bit-disjoint values (pure nat, gated) ----
   If for every position at least one operand's bit is 0, addition produces no
   carries, so bit i of the sum is bit_i(x) + bit_i(y). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --using_facts_from 'Prims FStar'"
(* split a modulus off its top bit *)
let lemma_mod_pow2_split (bx: nat) (m: pos)
  : Lemma (bx % pow2 m == (bx % pow2 (m - 1)) + ((bx / pow2 (m - 1)) % 2) * pow2 (m - 1)) =
  assert_norm (pow2 1 == 2);
  FStar.Math.Lemmas.pow2_double_mult (m - 1);
  FStar.Math.Lemmas.modulo_modulo_lemma bx (pow2 (m - 1)) 2;
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 bx (m - 1) m;
  FStar.Math.Lemmas.euclidean_division_definition (bx % pow2 m) (pow2 (m - 1))
(* disjoint low bits => the low-m-bit sums never overflow (no carry out of pos m).
   Disjointness threaded as a per-index Lemma VALUE `pf` (NOT a `forall` requires):
   a `forall (j). j<m ==> …pow2 j…` cascades badly (Z3 e-matches pow2 at every term),
   and proving the j<m-1 subset at the recursive call is the worst offender. *)
let rec lemma_carryfree_low (bx byv: nat) (m: nat)
      (pf: (j:nat{j < m}) -> Lemma ((bx / pow2 j) % 2 == 0 \/ (byv / pow2 j) % 2 == 0))
  : Lemma (ensures (bx % pow2 m) + (byv % pow2 m) < pow2 m)
          (decreases m) =
  if m = 0 then ()
  else begin
    lemma_carryfree_low bx byv (m - 1) (fun j -> pf j);
    lemma_mod_pow2_split bx m;
    lemma_mod_pow2_split byv m;
    FStar.Math.Lemmas.pow2_double_mult (m - 1);
    pf (m - 1)
  end
#pop-options
(* the nonlinear div step over an ABSTRACT divisor d:pos — kept free of any `pow2`
   nesting so Z3's nonlinear reasoning is minimal and context-independent. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 200 --using_facts_from 'Prims FStar'"
let lemma_add_div_nocarry (a b: nat) (d: pos)
  : Lemma (requires a % d + b % d < d)
          (ensures (a + b) / d == a / d + b / d) =
  let kk : nat = (a / d) + (b / d) in
  FStar.Math.Lemmas.euclidean_division_definition a d;
  FStar.Math.Lemmas.euclidean_division_definition b d;
  FStar.Math.Lemmas.distributivity_add_left (a / d) (b / d) d;
  assert (a + b == (a % d + b % d) + kk * d);
  FStar.Math.Lemmas.lemma_div_plus (a % d + b % d) kk d;
  FStar.Math.Lemmas.small_div (a % d + b % d) d
#pop-options
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --using_facts_from 'Prims FStar'"
(* bit i of a carry-free sum is the sum of the operand bits (disjointness as pf) *)
let lemma_disjoint_add_bit (bx byv n i: nat)
      (pf: (j:nat{j < n}) -> Lemma ((bx / pow2 j) % 2 == 0 \/ (byv / pow2 j) % 2 == 0))
  : Lemma (requires i < n)
          (ensures ((bx + byv) / pow2 i) % 2 == ((bx / pow2 i) % 2) + ((byv / pow2 i) % 2)) =
  lemma_carryfree_low bx byv i (fun j -> pf j);     (* bx%2^i + byv%2^i < 2^i *)
  lemma_add_div_nocarry bx byv (pow2 i);            (* (bx+byv)/2^i == bx/2^i + byv/2^i *)
  FStar.Math.Lemmas.modulo_distributivity (bx / pow2 i) (byv / pow2 i) 2;
  pf i
#pop-options
(* the signed wrap-around @%. is a mod-2^32 identity on the low 32 bits *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200 --using_facts_from 'Prims FStar Rust_primitives.Integers'"
let lemma_atpct_mod32 (k: int)
  : Lemma ((k @%. Ints.I32) % pow2 32 == k % pow2 32) =
  assert_norm (Ints.modulus Ints.I32 == pow2 32);
  let p = pow2 32 in
  let m = k % p in
  if m >= p / 2 then FStar.Math.Lemmas.lemma_mod_plus m (-1) p
  else FStar.Math.Lemmas.small_mod m p
#pop-options
(* the wrapping add agrees with the true sum on the low 32 bits *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_wrapadd_rep (x y: i32)
  : Lemma (v (x `i32_wrapping_add` y) % pow2 32 == (v x + v y) % pow2 32) =
  reveal_opaque (`%i32_wrapping_add) i32_wrapping_add;
  reveal_opaque_arithmetic_ops #i32_inttype;
  lemma_atpct_mod32 (v x + v y)
#pop-options
(* get_bit of an i32 as a pure-nat readout on its 2^32 representative. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_get_bit_i32_nat (z: i32) (j: nat {j < 32})
  : Lemma (Ints.get_bit z (Ints.mk_usize j) == get_bit_nat (v z % pow2 32) j) =
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32);
  if v z >= 0 then FStar.Math.Lemmas.small_mod (v z) (pow2 32)
  else (FStar.Math.Lemmas.lemma_mod_plus (v z) 1 (pow2 32);
        FStar.Math.Lemmas.small_mod (pow2 32 + v z) (pow2 32))
#pop-options

(* ===== (F) i64-lane carry-free bit addition — mm256_add_epi64 ===== *)

(* a low bit `bit` of a nat is unaffected by an outer mod 2^64 (bit < 64) *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 200 --using_facts_from 'Prims FStar'"
let lemma_bit_of_mod64 (a: nat) (bit: nat{bit < 64})
  : Lemma (((a % pow2 64) / pow2 bit) % 2 == (a / pow2 bit) % 2) =
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 a bit 64;
  FStar.Math.Lemmas.pow2_double_mult (63 - bit);
  FStar.Math.Lemmas.modulo_modulo_lemma (a / pow2 bit) 2 (pow2 (63 - bit))
#pop-options

(* bit `bit` of a carry-free (low-bit disjoint) sum, taken mod 2^64.
   Disjointness threaded as a per-index Lemma VALUE `pf` (only j < bit). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --using_facts_from 'Prims FStar'"
let lemma_condadd_bit (bx byv: nat) (bit: nat{bit < 64})
      (pf: (j:nat{j < bit}) -> Lemma ((bx / pow2 j) % 2 == 0 \/ (byv / pow2 j) % 2 == 0))
  : Lemma ((((bx + byv) % pow2 64) / pow2 bit) % 2 ==
           ((bx / pow2 bit) % 2 + (byv / pow2 bit) % 2) % 2) =
  lemma_carryfree_low bx byv bit pf;
  lemma_add_div_nocarry bx byv (pow2 bit);
  lemma_bit_of_mod64 (bx + byv) bit;
  FStar.Math.Lemmas.modulo_distributivity (bx / pow2 bit) (byv / pow2 bit) 2
#pop-options

(* get_bit of an i64 as a pure-nat readout on its 2^64 representative *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_get_bit_i64_nat (z: i64) (j: nat {j < 64})
  : Lemma (Ints.get_bit z (Ints.mk_usize j) == get_bit_nat (v z % pow2 64) j) =
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I64);
  if v z >= 0 then FStar.Math.Lemmas.small_mod (v z) (pow2 64)
  else (FStar.Math.Lemmas.lemma_mod_plus (v z) 1 (pow2 64);
        FStar.Math.Lemmas.small_mod (pow2 64 + v z) (pow2 64))
#pop-options

(* the signed wrap-around @%. is a mod-2^64 identity on the low 64 bits *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200 --using_facts_from 'Prims FStar Rust_primitives.Integers'"
let lemma_atpct_mod64 (k: int)
  : Lemma ((k @%. Ints.I64) % pow2 64 == k % pow2 64) =
  assert_norm (Ints.modulus Ints.I64 == pow2 64);
  let p = pow2 64 in
  let m = k % p in
  if m >= p / 2 then FStar.Math.Lemmas.lemma_mod_plus m (-1) p
  else FStar.Math.Lemmas.small_mod m p
#pop-options

(* the wrapping i64 add agrees with the true sum on the low 64 bits *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_wrapadd_rep_i64 (x y: i64)
  : Lemma (v (x `add_mod` y) % pow2 64 == (v x + v y) % pow2 64) =
  reveal_opaque_arithmetic_ops #Ints.I64;
  lemma_atpct_mod64 (v x + v y)
#pop-options

(* the add_epi64 result's i64 lane = wrapping add of the two operand lanes *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 250"
let lemma_add_epi64_lane (lhs rhs: bv256) (lane: u64{v lane < 4})
  : Lemma (to_i64x4 (I.mm256_add_epi64 lhs rhs) lane ==
           (to_i64x4 lhs lane) `add_mod` (to_i64x4 rhs lane)) =
  reveal_opaque (`%I.mm256_add_epi64) I.mm256_add_epi64;
  Canon.lemma_mm256_add_epi64 lhs rhs
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 400"
let mm256_add_epi64_lemma lhs rhs i =
  let lane : u64 = i /! mk_u64 64 in
  let bit  : u64 = i %! mk_u64 64 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 64;
  let add = I.mm256_add_epi64 lhs rhs in
  i64_to_bv_to_i64x4_inv lhs lane bit;
  i64_to_bv_to_i64x4_inv rhs lane bit;
  i64_to_bv_to_i64x4_inv add lane bit;
  let xl : i64 = to_i64x4 lhs lane in
  let yl : i64 = to_i64x4 rhs lane in
  let zl : i64 = to_i64x4 add lane in
  lemma_add_epi64_lane lhs rhs lane;
  let bx : nat = v xl % pow2 64 in
  let byv : nat = v yl % pow2 64 in
  let disj (j: nat{j < v bit}) : Lemma ((bx / pow2 j) % 2 == 0 \/ (byv / pow2 j) % 2 == 0) =
    let jj : u64 = mk_u64 j in
    i64_to_bv_to_i64x4_inv lhs lane jj;
    i64_to_bv_to_i64x4_inv rhs lane jj;
    assert (Bit_Zero? (i64_to_bv xl jj) \/ Bit_Zero? (i64_to_bv yl jj));
    assert (i64_to_bv xl jj == IVi.encode_bit Ints.I64 xl j);
    assert (i64_to_bv yl jj == IVi.encode_bit Ints.I64 yl j);
    bit_of_get_bit Ints.I64 xl j;
    bit_of_get_bit Ints.I64 yl j;
    lemma_get_bit_i64_nat xl j;
    lemma_get_bit_i64_nat yl j
  in
  lemma_condadd_bit bx byv (v bit) disj;
  lemma_wrapadd_rep_i64 xl yl;
  FStar.Math.Lemmas.lemma_mod_plus_distr_l (v xl) (v yl) (pow2 64);
  FStar.Math.Lemmas.lemma_mod_plus_distr_r bx (v yl) (pow2 64);
  lemma_get_bit_i64_nat zl (v bit);
  lemma_get_bit_i64_nat xl (v bit);
  lemma_get_bit_i64_nat yl (v bit);
  bit_of_get_bit Ints.I64 zl (v bit);
  bit_of_get_bit Ints.I64 xl (v bit);
  bit_of_get_bit Ints.I64 yl (v bit)
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let i32_to_bv_add_bv_lemma x y i =
  reveal_opaque (`%i32_wrapping_add) i32_wrapping_add;
  let z : i32 = x `i32_wrapping_add` y in
  let bx : nat = v x % pow2 32 in
  let byv : nat = v y % pow2 32 in
  (* the requires forall (over i32_to_bv=encode_bit) => the nat-bit disjointness *)
  let disj (j: nat{j < 32}) : Lemma ((bx / pow2 j) % 2 == 0 \/ (byv / pow2 j) % 2 == 0) =
    let jj : u64 = mk_u64 j in
    assert (Bit_Zero? (i32_to_bv x jj) \/ Bit_Zero? (i32_to_bv y jj));
    assert (i32_to_bv x jj == IVi.encode_bit Ints.I32 x j);
    assert (i32_to_bv y jj == IVi.encode_bit Ints.I32 y j);
    bit_of_get_bit Ints.I32 x j;
    bit_of_get_bit Ints.I32 y j;
    lemma_get_bit_i32_nat x j;
    lemma_get_bit_i32_nat y j
  in
  lemma_disjoint_add_bit bx byv 32 (v i) disj;
  (* v z % 2^32 == (v x + v y) % 2^32 == (bx + byv) % 2^32 == bx + byv  (carry-free) *)
  lemma_carryfree_low bx byv 32 disj;
  lemma_wrapadd_rep x y;
  FStar.Math.Lemmas.lemma_mod_plus_distr_l (v x) (v y) (pow2 32);
  FStar.Math.Lemmas.lemma_mod_plus_distr_r bx (v y) (pow2 32);
  FStar.Math.Lemmas.small_mod (bx + byv) (pow2 32);
  lemma_get_bit_i32_nat z (v i);
  lemma_get_bit_i32_nat x (v i);
  lemma_get_bit_i32_nat y (v i);
  bit_of_get_bit Ints.I32 z (v i);
  bit_of_get_bit Ints.I32 x (v i);
  bit_of_get_bit Ints.I32 y (v i)
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let pow2_lemma shift i =
  bit_of_get_bit Ints.I32 (mk_i32 1 <<! shift <: i32) (v i)
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let mm256_set_epi8_lemma b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31 i =
  reveal_opaque (`%I.mm256_set_epi8) I.mm256_set_epi8;
  Canon.lemma_mm256_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31;
  Canon.lemma_iv_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31 (v i)
let mm_set_epi8_lemma b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 i =
  reveal_opaque (`%I.mm_set_epi8) I.mm_set_epi8;
  Canon.lemma_mm_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15;
  Canon.lemma_iv_mm_set_epi8 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 (v i)
let mm256_set_epi16_lemma v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15 i =
  reveal_opaque (`%I.mm256_set_epi16) I.mm256_set_epi16;
  Canon.lemma_mm256_set_epi16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15;
  Canon.lemma_iv_set_epi16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15 (v i)
#pop-options
(* i16 with a zero sign bit (bit 15) is non-negative. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_i16_nonneg (x: i16)
  : Lemma (requires Bit_Zero? (i16_to_bv x (mk_u64 15)))
          (ensures v x >= 0) =
  bit_of_get_bit Ints.I16 x 15;
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I16);
  assert_norm (pow2 15 == 32768);
  assert_norm (pow2 16 == 65536)
#pop-options

(* The 6-bit packing constant.  `unfold` so that every use below is syntactically
   the SAME set_epi16 application the .fsti's ensures / SMTPat mention (a plain
   `let` — or an `[@inline_let]` inside the lemma body — leaves an opaque binder
   in the nested lemma's VC, which is what stopped the set_epi16 SMTPat firing). *)
unfold let madd_pack_const : bv256 =
  I.mm256_set_epi16 (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 0)
    (mk_i16 0) (mk_i16 0) (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1) (mk_i16 0) (mk_i16 0)
    (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 0) (mk_i16 1 <<! mk_i32 6 <: i16) (mk_i16 1)

(* Lane values of the packing constant, extracted in a CLEAN context: lanes 0/8
   hold 1, lanes 1/9 hold 2^6, the rest 0.  mm256_set_epi16_lemma's SMTPat does
   not fire on this term inside the specialized lemma's heavy context, so the
   fact is discharged here and CALLED explicitly there. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_madd_pack_const_lane (k: nat{k < 16})
  : Lemma (to_i16x16 madd_pack_const (mk_int k)
           == (if k = 0 || k = 8 then mk_i16 1
               else if k = 1 || k = 9 then (mk_i16 1 <<! mk_i32 6 <: i16)
               else mk_i16 0)) = ()
#pop-options

(* ONE packed i32 block: i16 lanes (nb2, nb2+1) of `vec` against const lanes
   (1, 2^6).  Everything block-specific — WHICH lane pair, and the raw-bit base
   `base = nb2*16` — is supplied EQUATIONALLY by the caller, so this lemma is
   generic in nb2 and proves in a CLEAN context.  (An earlier in-body version
   parameterized by `nb2 = 2*nb /\ (nb = 0 \/ nb = 4)` starved: the disjunctive
   block index left `nb2*16` and the requires' div/mod irreducible.)
   Given the caller's requires — low lane is a 6-bit value (bits 6..15 zero) and
   high lane is non-negative (bit 15 zero) — the sum lo + hi*2^6 is carry-free,
   so its bits are lo's below 6, then hi's shifted up by 6, then nothing. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_madd_pack_block
      (vec: bv256) (nb2: nat{nb2 + 1 < 16}) (base: nat{base == nb2 * 16})
      (p: u64{v p < 32})
  : Lemma
    (requires
      (forall (j: nat). 6 <= j /\ j < 16 ==> Bit_Zero? vec.(mk_int (base + j)))
      /\ Bit_Zero? vec.(mk_int (base + 31))
      /\ to_i16x16 madd_pack_const (mk_int nb2) == mk_i16 1
      /\ to_i16x16 madd_pack_const (mk_int (nb2 + 1)) == (mk_i16 1 <<! mk_i32 6 <: i16))
    (ensures
      i32_to_bv ((to_i16x16 vec (mk_int nb2) `i16_mul_32extended`
                    to_i16x16 madd_pack_const (mk_int nb2))
                 `i32_wrapping_add`
                 (to_i16x16 vec (mk_int (nb2 + 1)) `i16_mul_32extended`
                    to_i16x16 madd_pack_const (mk_int (nb2 + 1)))) p
      == (if v p < 6 then vec.(mk_int (base + v p))
          else if v p < 22 then vec.(mk_int (base + 16 + v p - 6))
          else Bit_Zero)) =
  let lo : i16 = to_i16x16 vec (mk_int nb2) in
  let hi : i16 = to_i16x16 vec (mk_int (nb2 + 1)) in
  (* sign bits zero -> both lanes non-negative (the i16-mul bit lemmas need it) *)
  i16_to_bv_to_i16x16_inv vec (mk_u64 nb2) (mk_u64 15);
  i16_to_bv_to_i16x16_inv vec (mk_u64 (nb2 + 1)) (mk_u64 15);
  lemma_i16_nonneg lo;
  lemma_i16_nonneg hi;
  (* bit-disjointness (the i32_to_bv_add_bv precondition): lo*1 occupies [0,6),
     hi*2^6 occupies [6,22).  The guarded-binder shape is what that lemma wants. *)
  let disj (j: u64{v j < 32}) : Lemma
    (Bit_Zero? (i32_to_bv (lo `i16_mul_32extended` mk_i16 1) j)
     \/ Bit_Zero? (i32_to_bv (hi `i16_mul_32extended` (mk_i16 1 <<! mk_i32 6)) j)) =
    if v j < 6 then ()
    else if v j < 16
    then (i16_to_bv_to_i16x16_inv vec (mk_u64 nb2) j;
          i16_to_bv_to_i16x16_inv vec (mk_u64 nb2) (mk_u64 (v j)))
    else ()
  in
  FStar.Classical.forall_intro disj;
  (* Called EXPLICITLY: its SMTPat does not deliver here, even though its
     `requires` forall is exactly what forall_intro just established. *)
  i32_to_bv_add_bv_lemma (lo `i16_mul_32extended` mk_i16 1)
                         (hi `i16_mul_32extended` (mk_i16 1 <<! mk_i32 6)) p;
  if v p < 6
  then (i16_to_bv_to_i16x16_inv vec (mk_u64 nb2) p;
        i16_to_bv_to_i16x16_inv vec (mk_u64 nb2) (mk_u64 (v p)))
  else if v p < 22
  then i16_to_bv_to_i16x16_inv vec (mk_u64 (nb2 + 1)) (mk_u64 (v p - 6))
  else ()
#pop-options

(* madd against the 6-bit-packing const set_epi16(..,2^6,1,..,2^6,1): i32 lanes 0
   and 4 hold `lo + hi*2^6` (lo = low i16 lane, hi = high i16 lane); the other six
   have both const lanes 0, so their product bits — hence the sum's — are all zero. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 600"
let mm256_madd_epi16_specialized_lemma vec i =
  mm256_madd_epi16_lemma vec madd_pack_const i;
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  let nb : nat = v i / 32 in
  lemma_madd_pack_const_lane (nb * 2);
  lemma_madd_pack_const_lane (nb * 2 + 1);
  (* Dispatch on a LITERAL block index so the const-lane if-chain and the
     requires' div/mod both reduce.  Each `aux` re-packages the module requires
     (whose trigger is `vec.(mk_int i)`) into the block lemma's guarded shape. *)
  if nb = 0 then begin
    lemma_madd_pack_const_lane 0;
    lemma_madd_pack_const_lane 1;
    let aux (j: nat) : Lemma (6 <= j /\ j < 16 ==> Bit_Zero? vec.(mk_int (0 + j))) = () in
    FStar.Classical.forall_intro aux;
    lemma_madd_pack_block vec 0 0 (i %! mk_int 32)
  end
  else if nb = 1 then ()
  else if nb = 2 then ()
  else if nb = 3 then ()
  else if nb = 4 then begin
    lemma_madd_pack_const_lane 8;
    lemma_madd_pack_const_lane 9;
    let aux (j: nat) : Lemma (6 <= j /\ j < 16 ==> Bit_Zero? vec.(mk_int (128 + j))) = () in
    FStar.Classical.forall_intro aux;
    lemma_madd_pack_block vec 8 128 (i %! mk_int 32)
  end
  else if nb = 5 then ()
  else if nb = 6 then ()
  else if nb = 7 then ()
  else ()
#pop-options
(* ===== byte-granular PSHUFB (256-bit): raw-bit view via to_i8x32 sel/neg ===== *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_shuffle_epi8_lemma vec indexes i =
  reveal_opaque (`%I.mm256_shuffle_epi8) I.mm256_shuffle_epi8;
  let nth : u64 = i /! mk_u64 8 in
  let bit : u64 = i %! mk_u64 8 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 8;
  let result = I.mm256_shuffle_epi8 vec indexes in
  Canon.lemma_mm256_shuffle_epi8 vec indexes;
  bit_view_inv Ints.I8 (mk_u64 256) (mk_u64 32) result nth bit;
  let index = to_i8x32 indexes nth in
  if v index < 0 then begin
    Canon.lemma_iv_shuffle_epi8_neg (Canon.to_i8x32 vec) (Canon.to_i8x32 indexes) (v nth);
    bit_of_get_bit Ints.I8 (mk_i8 0) (v bit)
  end else begin
    Canon.lemma_iv_shuffle_epi8_sel (Canon.to_i8x32 vec) (Canon.to_i8x32 indexes) (v nth);
    let sb : nat = 16 * (v nth / 16) + (v index % 16) in
    bit_view_inv Ints.I8 (mk_u64 256) (mk_u64 32) vec (mk_u64 sb) bit;
    FStar.Math.Lemmas.division_multiplication_lemma (v i) 8 16
  end
#pop-options
(* ===== byte-granular PSHUFB (128-bit): raw-bit view via to_i8x16 sel/neg ===== *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm_shuffle_epi8_lemma vec indexes i =
  reveal_opaque (`%I.mm_shuffle_epi8) I.mm_shuffle_epi8;
  let nth : u64 = i /! mk_u64 8 in
  let bit : u64 = i %! mk_u64 8 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 8;
  let result = I.mm_shuffle_epi8 vec indexes in
  Canon.lemma_mm_shuffle_epi8 vec indexes;
  bit_view_inv Ints.I8 (mk_u64 128) (mk_u64 16) result nth bit;
  let index = to_i8x16 indexes nth in
  if v index < 0 then begin
    Canon.lemma_iv_mm_shuffle_epi8_neg (Canon.to_i8x16 vec) (Canon.to_i8x16 indexes) (v nth);
    bit_of_get_bit Ints.I8 (mk_i8 0) (v bit)
  end else begin
    Canon.lemma_iv_mm_shuffle_epi8_sel (Canon.to_i8x16 vec) (Canon.to_i8x16 indexes) (v nth);
    let sb : nat = v index % 16 in
    bit_view_inv Ints.I8 (mk_u64 128) (mk_u64 16) vec (mk_u64 sb) bit
  end
#pop-options
(* proof-residence: locked(cold-gate) — this decl is the module's heaviest and is
   sensitive to accumulated solver state (it grinds without the restart). *)
#restart-solver
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_mullo_epi16_bv_lemma a b i =
  reveal_opaque (`%I.mm256_mullo_epi16) I.mm256_mullo_epi16;
  reveal_opaque (`%i16_mul_32extended_i16) i16_mul_32extended_i16;
  reveal_opaque (`%i16_mul_32extended) i16_mul_32extended;
  let lane = i /! mk_u64 16 in
  let bit  = i %! mk_u64 16 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 16;
  Canon.lemma_mm256_mullo_epi16 a b;
  i16_to_bv_to_i16x16_inv (I.mm256_mullo_epi16 a b) lane bit
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let mm256_shuffle_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_shuffle_epi32) I.mm256_shuffle_epi32;
  Canon.lemma_mm256_shuffle_epi32 a b
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_sub_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_sub_epi32) I.mm256_sub_epi32;
  reveal_opaque_arithmetic_ops #i32_inttype;
  Canon.lemma_mm256_sub_epi32 a b
let mm256_add_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_add_epi32) I.mm256_add_epi32;
  reveal_opaque_arithmetic_ops #i32_inttype;
  Canon.lemma_mm256_add_epi32 a b
let mm256_mullo_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_mullo_epi32) I.mm256_mullo_epi32;
  reveal_opaque_arithmetic_ops #i32_inttype;
  Canon.lemma_mm256_mullo_epi32 a b
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_mul_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_mul_epi32) I.mm256_mul_epi32;
  reveal_opaque_arithmetic_ops #Ints.I64;
  reveal_opaque_cast_ops #Ints.I32 #Ints.I64;
  reveal_opaque_cast_ops #Ints.I64 #Ints.I32;
  let k = i /! mk_u64 2 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 2;
  Canon.lemma_mm256_mul_epi32 a b;
  lemma_i32_sub_lanes (I.mm256_mul_epi32 a b) k
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_srai_epi32_lemma v_IMM8 a i =
  reveal_opaque (`%I.mm256_srai_epi32) I.mm256_srai_epi32;
  reveal_opaque_arithmetic_ops #i32_inttype;
  Canon.lemma_mm256_srai_epi32 v_IMM8 a
#pop-options
(* Under the (new, always-satisfied) `0 <= IMM8 <= 31` precondition the model's
   `rem_euclid IMM8 256` is the identity and its `> 31` guard is dead, so both
   sides are the same left shift — one routed through u32, matched bit by bit. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_slli_epi32_lemma v_IMM8 a i =
  reveal_opaque (`%I.mm256_slli_epi32) I.mm256_slli_epi32;
  reveal_opaque_arithmetic_ops #i32_inttype;
  Canon.lemma_rem_euclid256 v_IMM8;
  Canon.lemma_mm256_slli_epi32 v_IMM8 a;
  let x : i32 = to_i32x8 a i in
  let l : i32 = to_i32x8 (I.mm256_slli_epi32 v_IMM8 a) i in
  let r : i32 = Ints.shift_left #Ints.I32 #Ints.I32 x v_IMM8 in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit l jj == Ints.get_bit r jj) = () in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits l r
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_and_si256_lemma a b i =
  let r = to_i32x8 (I.mm256_and_si256 a b) i in
  let xa = to_i32x8 a i in
  let xb = to_i32x8 b i in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma
    (Ints.get_bit r jj == Ints.get_bit (xa &. xb) jj) =
    let j = mk_u64 (v jj) in
    mm256_and_si256 a b (mk_u64 (v i * 32 + v jj));
    i32_to_bv_to_i32x8_inv (I.mm256_and_si256 a b) i j;
    i32_to_bv_to_i32x8_inv a i j;
    i32_to_bv_to_i32x8_inv b i j;
    ebit_is_get_bit Ints.I32 r (v jj);
    ebit_is_get_bit Ints.I32 xa (v jj);
    ebit_is_get_bit Ints.I32 xb (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits r (xa &. xb)
#pop-options
(* raw-bit semantics of the hardware xor (mirror of Canon.lemma_and_funarr). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 250"
let mm256_xor_bv (a b: bv256) (k: u64{v k < 256})
  : Lemma ((I.mm256_xor_si256 a b).(k) ==
           (match a.(k), b.(k) with
            | Bit_Zero, Bit_Zero -> Bit_Zero
            | Bit_One,  Bit_One  -> Bit_Zero
            | _ -> Bit_One)) =
  reveal_opaque (`%I.mm256_xor_si256) I.mm256_xor_si256;
  Canon.lemma_xor_si256_lift a b;
  let f : (i: u64{v i < 256}) -> t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: t_Bit), (b.[ i ] <: t_Bit) with
              | Bit_Zero, Bit_Zero -> Bit_Zero
              | Bit_One,  Bit_One  -> Bit_Zero
              | _ -> Bit_One) in
  assert (IV.e_mm256_xor_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%IV.e_mm256_xor_si256]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  Canon.lemma_impl9_index f k;
  Canon.lemma_bv_index a k;
  Canon.lemma_bv_index b k
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_xor_si256_lemma a b i =
  let r = to_i32x8 (I.mm256_xor_si256 a b) i in
  let xa = to_i32x8 a i in
  let xb = to_i32x8 b i in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma
    (Ints.get_bit r jj == Ints.get_bit (xa ^. xb) jj) =
    let j = mk_u64 (v jj) in
    mm256_xor_bv a b (mk_u64 (v i * 32 + v jj));
    i32_to_bv_to_i32x8_inv (I.mm256_xor_si256 a b) i j;
    i32_to_bv_to_i32x8_inv a i j;
    i32_to_bv_to_i32x8_inv b i j;
    ebit_is_get_bit Ints.I32 r (v jj);
    ebit_is_get_bit Ints.I32 xa (v jj);
    ebit_is_get_bit Ints.I32 xb (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits r (xa ^. xb)
#pop-options
(* core-models `e_mm256_abs_epi32` delegates to `Core_models.Num.impl_i32__abs`,
   which used to be `Rust_primitives.Arithmetic.abs_i32` — an uninterpreted `val`
   with no ensures anywhere in the hax proof-libs — so this was admitted (hax#2107).
   hax now models `i32::abs` directly and `Core_models.Num.Abs_spec` bridges it to
   `abs_int`; the `requires` rules out the `i32::MIN` lane the model special-cases. *)
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_abs_epi32_lemma a i =
  reveal_opaque (`%I.mm256_abs_epi32) I.mm256_abs_epi32;
  Canon.lemma_mm256_abs_epi32 a;
  Core_models.Num.Abs_spec.abs_i32 (to_i32x8 a i)
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_cmpgt_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_cmpgt_epi32) I.mm256_cmpgt_epi32;
  Canon.lemma_mm256_cmpgt_epi32 a b
#pop-options
(* bit (32i+j) of a bv256 is Bit_Zero iff bit j of its i-th i32 lane view is 0. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let testz_lane_bit (conjunct: bv256) (i: nat{i < 8}) (j: nat{j < 32})
  : Lemma (conjunct.(mk_u64 (32 * i + j)) == Bit_Zero <==>
           Ints.get_bit (to_i32x8 conjunct (mk_u64 i)) (mk_usize j) == 0) =
  i32_to_bv_to_i32x8_inv conjunct (mk_u64 i) (mk_u64 j);
  bit_of_get_bit Ints.I32 (to_i32x8 conjunct (mk_u64 i)) j
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_testz_si256_lemma a b =
  reveal_opaque (`%I.mm256_testz_si256) I.mm256_testz_si256;
  Canon.lemma_testz_si256_lift a b;
  reveal_opaque (`%I.mm256_and_si256) I.mm256_and_si256;
  Canon.lemma_and_si256_lift a b;
  Canon.lemma_testz_funarr a b;
  reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32);
  let conjunct = I.mm256_and_si256 a b in
  (* forward: all conjunct bits zero  ==>  all i32 lanes zero *)
  introduce (forall (k:u64). v k < 256 ==> conjunct.(k) == Bit_Zero) ==>
            (forall (i:u64). v i < 8 ==> to_i32x8 conjunct i == mk_i32 0)
  with _. introduce forall (i:u64). v i < 8 ==> to_i32x8 conjunct i == mk_i32 0
       with introduce v i < 8 ==> to_i32x8 conjunct i == mk_i32 0
       with _. begin
         introduce forall (jj:Ints.usize). v jj < 32 ==>
                     Ints.get_bit (to_i32x8 conjunct i) jj == Ints.get_bit (mk_i32 0) jj
         with introduce v jj < 32 ==>
                        Ints.get_bit (to_i32x8 conjunct i) jj == Ints.get_bit (mk_i32 0) jj
         with _. begin
           assert (32 * v i + v jj < 256);
           testz_lane_bit conjunct (v i) (v jj)
         end;
         Ints.lemma_int_t_eq_via_bits (to_i32x8 conjunct i) (mk_i32 0)
       end;
  (* backward: all i32 lanes zero  ==>  all conjunct bits zero *)
  introduce (forall (i:u64). v i < 8 ==> to_i32x8 conjunct i == mk_i32 0) ==>
            (forall (k:u64). v k < 256 ==> conjunct.(k) == Bit_Zero)
  with _. introduce forall (k:u64). v k < 256 ==> conjunct.(k) == Bit_Zero
       with introduce v k < 256 ==> conjunct.(k) == Bit_Zero
       with _. begin
         let i = v k / 32 in
         let j = v k % 32 in
         FStar.Math.Lemmas.lemma_div_mod (v k) 32;
         testz_lane_bit conjunct i j
       end
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_set_epi64x_lemma x0 x1 x2 x3 i =
  reveal_opaque (`%I.mm256_set_epi64x) I.mm256_set_epi64x;
  Canon.lemma_mm256_set_epi64x x0 x1 x2 x3
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 250"
let mm256_set_epi64x_bv_lemma x0 x1 x2 x3 i =
  let lane = i /! mk_u64 64 in
  let bit = i %! mk_u64 64 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 64;
  i64_to_bv_to_i64x4_inv (I.mm256_set_epi64x x0 x1 x2 x3) lane bit;
  mm256_set_epi64x_lemma x0 x1 x2 x3 lane
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_set_epi32_lemma x0 x1 x2 x3 x4 x5 x6 x7 i =
  reveal_opaque (`%I.mm256_set_epi32) I.mm256_set_epi32;
  Canon.lemma_mm256_set_epi32 x0 x1 x2 x3 x4 x5 x6 x7
let mm256_set1_epi32_lemma x0 i =
  reveal_opaque (`%I.mm256_set1_epi32) I.mm256_set1_epi32;
  Canon.lemma_mm256_set1_epi32 x0
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 250"
let mm256_set1_epi32_bv_lemma x0 i =
  let lane = i /! mk_u64 32 in
  let bit = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  i32_to_bv_to_i32x8_inv (I.mm256_set1_epi32 x0) lane bit;
  mm256_set1_epi32_lemma x0 lane
let mm256_set_epi32_bv_lemma x0 x1 x2 x3 x4 x5 x6 x7 i =
  let lane = i /! mk_u64 32 in
  let bit = i %! mk_u64 32 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 32;
  i32_to_bv_to_i32x8_inv (I.mm256_set_epi32 x0 x1 x2 x3 x4 x5 x6 x7) lane bit;
  mm256_set_epi32_lemma x0 x1 x2 x3 x4 x5 x6 x7 lane
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm_set_epi32_lemma x0 x1 x2 x3 i =
  reveal_opaque (`%I.mm_set_epi32) I.mm_set_epi32;
  Canon.lemma_mm_set_epi32 x0 x1 x2 x3
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let mm256_blend_epi32_lemma imm8 a b i =
  reveal_opaque (`%I.mm256_blend_epi32) I.mm256_blend_epi32;
  Canon.lemma_mm256_blend_epi32 imm8 a b
#pop-options
(* set_m128i is a bit-level concat; its lift lives in Trusted.Intrinsics. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 250"
let mm256_set_m128i_bv_lemma hi lo i =
  reveal_opaque (`%I.mm256_set_m128i) I.mm256_set_m128i;
  Canon.lemma_set_m128i_lift hi lo;
  let f : (k: u64{v k < 256}) -> t_Bit =
    fun k -> (let k:u64 = k in
              if k <. mk_u64 128 then lo.[ k ] <: t_Bit else hi.[ k -! mk_u64 128 <: u64 ] <: t_Bit) in
  assert (IV.e_mm256_set_m128i hi lo ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%IV.e_mm256_set_m128i]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  Canon.lemma_impl9_index f i;
  if v i < 128 then Canon.lemma_bv_index_n #(mk_u64 128) lo i
               else Canon.lemma_bv_index_n #(mk_u64 128) hi (i -! mk_u64 128)
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_set_m128i_lemma hi lo i =
  let r = I.mm256_set_m128i hi lo in
  let l : i32 = to_i32x8 r i in
  let src : bv128 = if v i < 4 then lo else hi in
  let sl  : u64   = if v i < 4 then i else i -! mk_u64 4 in
  let rr : i32 = to_i32x4 src sl in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit l jj == Ints.get_bit rr jj) =
    let j = mk_u64 (v jj) in
    mm256_set_m128i_bv_lemma hi lo (mk_u64 (v i * 32 + v jj));
    i32_to_bv_to_i32x8_inv r i j;
    bit_view_inv Ints.I32 (mk_u64 128) (mk_u64 4) src sl j;
    ebit_is_get_bit Ints.I32 l (v jj);
    ebit_is_get_bit Ints.I32 rr (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits l rr
#pop-options
#restart-solver
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_permute2x128_si256_lemma_i32x4 imm8 a b j =
  reveal_opaque (`%I.mm256_permute2x128_si256) I.mm256_permute2x128_si256;
  Canon.lemma_mm256_permute2x128_si256 imm8 a b;
  let r = I.mm256_permute2x128_si256 imm8 a b in
  let i : u64 = j /! mk_u64 4 in
  let offset : nat = v j % 4 in
  FStar.Math.Lemmas.lemma_div_mod (v j) 4;
  let control : i32 = imm8 >>! (i *! mk_u64 4 <: u64) in
  if ((control >>! mk_i32 3 <: i32) %! mk_i32 2 <: i32) =. mk_i32 1
  then lemma_i32_of_i128_zero r i offset
  else (match v (control %! mk_i32 4 <: i32) with
        | 0 -> lemma_i32_from_i128_transfer r a i (mk_u64 0) offset
        | 1 -> lemma_i32_from_i128_transfer r a i (mk_u64 1) offset
        | 2 -> lemma_i32_from_i128_transfer r b i (mk_u64 0) offset
        | _ -> lemma_i32_from_i128_transfer r b i (mk_u64 1) offset)
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_castsi256_si128_lemma a i =
  let l = to_i32x4 (I.mm256_castsi256_si128 a) i in
  let r = to_i32x8 a i in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit l jj == Ints.get_bit r jj) =
    let j = mk_u64 (v jj) in
    mm256_castsi256_si128_bv_lemma a (mk_u64 (v i * 32 + v jj));
    bit_view_inv Ints.I32 (mk_u64 128) (mk_u64 4) (I.mm256_castsi256_si128 a) i j;
    i32_to_bv_to_i32x8_inv a i j;
    ebit_is_get_bit Ints.I32 l (v jj);
    ebit_is_get_bit Ints.I32 r (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits l r
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_unpacklo_epi64_lemma a b i =
  reveal_opaque (`%I.mm256_unpacklo_epi64) I.mm256_unpacklo_epi64;
  Canon.lemma_mm256_unpacklo_epi64 a b;
  let r = I.mm256_unpacklo_epi64 a b in
  (match v i with
   | 0 -> lemma_i32_lane_transfer r a (mk_u64 0) (mk_u64 0) 0
   | 1 -> lemma_i32_lane_transfer r a (mk_u64 0) (mk_u64 0) 1
   | 2 -> lemma_i32_lane_transfer r b (mk_u64 1) (mk_u64 0) 0
   | 3 -> lemma_i32_lane_transfer r b (mk_u64 1) (mk_u64 0) 1
   | 4 -> lemma_i32_lane_transfer r a (mk_u64 2) (mk_u64 2) 0
   | 5 -> lemma_i32_lane_transfer r a (mk_u64 2) (mk_u64 2) 1
   | 6 -> lemma_i32_lane_transfer r b (mk_u64 3) (mk_u64 2) 0
   | _ -> lemma_i32_lane_transfer r b (mk_u64 3) (mk_u64 2) 1)

let mm256_unpackhi_epi64_lemma a b i =
  reveal_opaque (`%I.mm256_unpackhi_epi64) I.mm256_unpackhi_epi64;
  Canon.lemma_mm256_unpackhi_epi64 a b;
  let r = I.mm256_unpackhi_epi64 a b in
  (match v i with
   | 0 -> lemma_i32_lane_transfer r a (mk_u64 0) (mk_u64 1) 0
   | 1 -> lemma_i32_lane_transfer r a (mk_u64 0) (mk_u64 1) 1
   | 2 -> lemma_i32_lane_transfer r b (mk_u64 1) (mk_u64 1) 0
   | 3 -> lemma_i32_lane_transfer r b (mk_u64 1) (mk_u64 1) 1
   | 4 -> lemma_i32_lane_transfer r a (mk_u64 2) (mk_u64 3) 0
   | 5 -> lemma_i32_lane_transfer r a (mk_u64 2) (mk_u64 3) 1
   | 6 -> lemma_i32_lane_transfer r b (mk_u64 3) (mk_u64 3) 0
   | _ -> lemma_i32_lane_transfer r b (mk_u64 3) (mk_u64 3) 1)
#pop-options
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let mm_loadu_si128_lemma bytes i =
  reveal_opaque (`%I.mm_loadu_si128) I.mm_loadu_si128;
  reveal_opaque (`%Extra.mm_loadu_si128_model) Extra.mm_loadu_si128_model;
  let lane = i /! mk_u64 8 in
  let bit  = i %! mk_u64 8 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 8;
  u8_to_bv_to_u8x16_inv (I.mm_loadu_si128 bytes) lane bit
#pop-options
(* Non-negative lane whose value < 2^n has all bits >= n zero (the axiom is FALSE
   for a NEGATIVE lane — its high bits are 1 — hence the `0 <= v lane` antecedent
   added to the .fsti). Read off via i32 lift + Rust_primitives lemma_get_bit_bounded. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let i32_lt_pow2_n_to_bit_zero_lemma n vec =
  let aux (i: u64{v i < 256}) : Lemma
    (0 <= v (to_i32x8 vec (i /! mk_int 32))
     ==> v (to_i32x8 vec (i /! mk_int 32)) <= normalize_term (pow2 n - 1)
     ==> v i % 32 >= n
     ==> vec.(i) == Bit_Zero) =
    let lane = i /! mk_int 32 in
    let b : nat = v i % 32 in
    FStar.Math.Lemmas.lemma_div_mod (v i) 32;
    let x = to_i32x8 vec lane in
    if 0 <= v x && v x <= pow2 n - 1 && b >= n then begin
      i32_to_bv_to_i32x8_inv vec lane (mk_u64 b);
      bit_of_get_bit Ints.I32 x b;
      if n = 0 then reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32)
      else Rust_primitives.BitVectors.lemma_get_bit_bounded #Ints.I32 x n (Ints.mk_usize b)
    end
  in FStar.Classical.forall_intro aux
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let shl_casted_u8_bv_lemma a b i =
  cast_u8_i32 a; cast_u8_i32 b;
  bit_of_get_bit Ints.I32 (((cast b <: i32) <<! mk_i32 8 <: i32) |. (cast a <: i32) <: i32) (v i);
  if v i >= 16 then ()
  else if v i >= 8 then bit_of_get_bit Ints.U8 b (v i - 8)
  else bit_of_get_bit Ints.U8 a (v i)
let i32_to_bv_cast_lemma a i =
  cast_u8_i32 a;
  bit_of_get_bit Ints.I32 (cast a <: i32) (v i);
  if v i < 8 then bit_of_get_bit Ints.U8 a (v i)
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let i32_to_bv_pow2_min_one_lemma n i =
  FStar.Math.Lemmas.pow2_lt_compat 31 n;
  Rust_primitives.BitVectors.get_bit_pow2_minus_one #Ints.I32 n (Ints.mk_usize (v i));
  assert ((mk_i32 1 <<! mk_i32 n <: i32) -! mk_i32 1 == Ints.mk_int #Ints.I32 (pow2 n - 1));
  bit_of_get_bit Ints.I32 ((mk_i32 1 <<! mk_i32 n <: i32) -! mk_i32 1 <: i32) (v i)
#pop-options
(* nat toolkit: if bits [lo,hi) of a value < 2^hi are all zero, the value < 2^lo.
   Pure nat arithmetic; facts pruned to FStar/Prims so the module's SIMD/get_bit
   cascade cannot enter this VC. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200 --using_facts_from 'Prims FStar'"
let rec nat_high_zero_bound (rep lo hi: nat)
  : Lemma (requires rep < pow2 hi /\ lo <= hi /\
                    (forall (j:nat). lo <= j /\ j < hi ==> (rep / pow2 j) % 2 == 0))
          (ensures rep < pow2 lo)
          (decreases (hi - lo)) =
  if hi = lo then ()
  else (
    FStar.Math.Lemmas.pow2_plus 1 (hi - 1);          (* pow2 hi == 2 * pow2 (hi-1) *)
    FStar.Math.Lemmas.lemma_div_lt rep hi (hi - 1);  (* rep/2^(hi-1) < 2 *)
    (* bit (hi-1) is 0 (forall at j=hi-1) and rep/2^(hi-1) < 2 ==> rep < 2^(hi-1) *)
    nat_high_zero_bound rep lo (hi - 1)
  )
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let i32_bit_zero_lemma_to_lt_pow2_n_weak n vec =
  let aux (lane: u64{v lane < 8})
    : Lemma (v (to_i32x8 vec lane) < pow2 n /\ (n <= 31 ==> v (to_i32x8 vec lane) >= 0)) =
    let x = to_i32x8 vec lane in
    if n >= 32 then FStar.Math.Lemmas.pow2_le_compat n 31
    else (
      assert_norm (pow2 31 == 2147483648);
      assert_norm (pow2 32 == 4294967296);
      reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32);
      let rep : nat = if v x >= 0 then v x else pow2 32 + v x in
      assert (rep < pow2 32);
      let hbit (j: nat{n <= j /\ j < 32}) : Lemma ((rep / pow2 j) % 2 == 0) =
        i32_to_bv_to_i32x8_inv vec lane (mk_u64 j);
        ebit_is_get_bit Ints.I32 x j
      in
      FStar.Classical.forall_intro hbit;
      nat_high_zero_bound rep n 32
    )
  in FStar.Classical.forall_intro aux
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let i32_bit_zero_lemma_to_positive vec =
  let aux (lane: u64{v lane < 8}) : Lemma (v (to_i32x8 vec lane) >= 0) =
    let x = to_i32x8 vec lane in
    assert (v (mk_int #Ints.U64 (v lane * 32 + 31)) % 32 == 31);
    i32_to_bv_to_i32x8_inv vec lane (mk_u64 31);
    ebit_is_get_bit Ints.I32 x 31;
    reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32)
  in
  FStar.Classical.forall_intro aux
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let to_i32x8_eq_to_bv_eq a b =
  let aux (k: u64{v k < 256}) : Lemma (a.(k) == b.(k)) =
    let i : u64 = k /! mk_u64 32 in
    let j : u64 = k %! mk_u64 32 in
    FStar.Math.Lemmas.lemma_div_mod (v k) 32;
    i32_to_bv_to_i32x8_inv a i j;
    i32_to_bv_to_i32x8_inv b i j
  in
  FStar.Classical.forall_intro aux;
  eq_pointwise_to_eq a b
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 300"
let lemma_from_i32x8_def_pt f =
  let g : i32x8 = FStar.FunctionalExtensionality.on (i:u64{v i < 8}) f in
  let aux (i: u64{v i < 8})
    : Lemma (to_i32x8 (from_i32x8 (FStar.FunctionalExtensionality.on (i:u64{v i < 8}) f)) i == f i) =
    to_from_i32x8_inv_lemma g;
    assert (to_i32x8 (from_i32x8 g) i == g i);
    assert (g i == f i)
  in
  FStar.Classical.forall_intro aux
#pop-options
(* 256-bit i32 store: 8-deep update_at_usize ladder (mirror of `store16`).
   OPAQUE — its ladder inflates later decls' context; only the two consumers
   below reveal it. *)
[@@ "opaque_to_smt"]
let store8_i32 (out: t_Slice i32 {Seq.length out >= 8}) (vec: bv256)
  : (r: t_Slice i32 {Seq.length r == Seq.length out}) =
  let upd = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize in
  let s0 = upd out (mk_usize 0) (to_i32x8 vec (mk_u64 0)) in
  let s1 = upd s0  (mk_usize 1) (to_i32x8 vec (mk_u64 1)) in
  let s2 = upd s1  (mk_usize 2) (to_i32x8 vec (mk_u64 2)) in
  let s3 = upd s2  (mk_usize 3) (to_i32x8 vec (mk_u64 3)) in
  let s4 = upd s3  (mk_usize 4) (to_i32x8 vec (mk_u64 4)) in
  let s5 = upd s4  (mk_usize 5) (to_i32x8 vec (mk_u64 5)) in
  let s6 = upd s5  (mk_usize 6) (to_i32x8 vec (mk_u64 6)) in
  upd s6 (mk_usize 7) (to_i32x8 vec (mk_u64 7))

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let store8_i32_unfold (out: t_Slice i32 {Seq.length out >= 8}) (vec: bv256)
  : Lemma (I.mm256_storeu_si256_i32 out vec == store8_i32 out vec) =
  reveal_opaque (`%store8_i32) store8_i32;
  reveal_opaque (`%I.mm256_storeu_si256_i32) I.mm256_storeu_si256_i32;
  reveal_opaque (`%Extra.mm256_storeu_si256_i32_model) Extra.mm256_storeu_si256_i32_model
#pop-options

#restart-solver
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400 --split_queries always"
let store8_i32_index (out: t_Slice i32 {Seq.length out >= 8}) (vec: bv256) (i: nat{i < 8})
  : Lemma (Seq.index (store8_i32 out vec) i == to_i32x8 vec (mk_int i)) =
  reveal_opaque (`%store8_i32) store8_i32;
  match i with
  | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | _ -> ()
#pop-options

#restart-solver
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let mm256_storeu_si256_i32_lemma out vec i =
  store8_i32_unfold out vec;
  store8_i32_index out vec i
#pop-options
let mm256_storeu_si256_i32_len_lemma out vec = ()
#push-options "--fuel 1 --ifuel 2 --z3rlimit 250"
(* every raw bit of setzero is 0 (mirror of Canon.lemma_setzero_raw, at bit level) *)
let setzero_bv (k: u64{v k < 256}) : Lemma ((I.mm256_setzero_si256 ()).(k) == Bit_Zero) =
  Canon.lemma_setzero_si256_lift ();
  let f : (i: u64{v i < 256}) -> t_Bit = fun temp_0_ -> (let _:u64 = temp_0_ in Bit_Zero) in
  assert (IV.e_mm256_setzero_si256 () ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%IV.e_mm256_setzero_si256]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  Canon.lemma_impl9_index f k
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_setzero_si256_lemma i =
  let z = I.mm256_setzero_si256 () in
  let r = to_i32x8 z i in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma (Ints.get_bit r jj == Ints.get_bit (mk_i32 0) jj) =
    let j = mk_u64 (v jj) in
    setzero_bv (mk_u64 (v i * 32 + v jj));
    i32_to_bv_to_i32x8_inv z i j;
    ebit_is_get_bit Ints.I32 r (v jj);
    reveal_opaque (`%Ints.get_bit) (Ints.get_bit #Ints.I32)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits r (mk_i32 0)
#pop-options
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let mm256_loadu_si256_i32_lemma input i =
  reveal_opaque (`%I.mm256_loadu_si256_i32) I.mm256_loadu_si256_i32;
  reveal_opaque (`%Extra.mm256_loadu_si256_i32_model) Extra.mm256_loadu_si256_i32_model;
  Canon.rt_i32x8 (Libcrux_core_models.Abstractions.Funarr.impl_5__from_fn (mk_u64 8) #i32 #(u64 -> i32)
    (fun j -> let j:u64 = j in
              if (cast (j <: u64) <: usize) <. (Core_models.Slice.impl__len #i32 input <: usize)
              then input.[ cast (j <: u64) <: usize ] <: i32
              else mk_i32 0))
#pop-options
(* `vec256_blendv_epi32 a b m = castps_si256 (blendv_ps (castsi256_ps a) …)`, and
   both casts are the IDENTITY on the bit vector (lifts in Trusted.Intrinsics), so
   this reduces to `Canon.lemma_mm256_blendv_ps`, whose model is lane-wise
   `if mask[i] < 0 then b[i] else a[i]` — exactly the .fsti statement. *)
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let vec256_blendv_epi32_lemma a b mask i =
  reveal_opaque (`%I.vec256_blendv_epi32) I.vec256_blendv_epi32;
  Canon.lemma_castsi256_ps_lift a;
  Canon.lemma_castsi256_ps_lift b;
  Canon.lemma_castsi256_ps_lift mask;
  Canon.lemma_castps_si256_lift (Avxc.e_mm256_blendv_ps a b mask);
  Canon.lemma_mm256_blendv_ps a b mask
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_cmpeq_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_cmpeq_epi32) I.mm256_cmpeq_epi32;
  Canon.lemma_mm256_cmpeq_epi32 a b
#pop-options
(* raw-bit semantics of the hardware `or` (mirror of mm256_xor_bv); the
   lift lives in Trusted.Intrinsics. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 250"
let mm256_or_bv (a b: bv256) (k: u64{v k < 256})
  : Lemma ((I.mm256_or_si256 a b).(k) ==
           (match a.(k), b.(k) with
            | Bit_Zero, Bit_Zero -> Bit_Zero
            | _ -> Bit_One)) =
  reveal_opaque (`%I.mm256_or_si256) I.mm256_or_si256;
  Canon.lemma_or_si256_lift a b;
  let f : (i: u64{v i < 256}) -> t_Bit =
    fun i -> (let i:u64 = i in
              match (a.[ i ] <: t_Bit), (b.[ i ] <: t_Bit) with
              | Bit_Zero, Bit_Zero -> Bit_Zero
              | _ -> Bit_One) in
  assert (IV.e_mm256_or_si256 a b ==
          Libcrux_core_models.Abstractions.Bitvec.impl_9__from_fn (mk_u64 256) #(u64 -> t_Bit) f)
    by (FStar.Tactics.norm [delta_only [`%IV.e_mm256_or_si256]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  Canon.lemma_impl9_index f k;
  Canon.lemma_bv_index a k;
  Canon.lemma_bv_index b k
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let mm256_or_si256_lemma a b i =
  let r = to_i32x8 (I.mm256_or_si256 a b) i in
  let xa = to_i32x8 a i in
  let xb = to_i32x8 b i in
  let aux (jj: Ints.usize{v jj < 32}) : Lemma
    (Ints.get_bit r jj == Ints.get_bit (xa |. xb) jj) =
    let j = mk_u64 (v jj) in
    mm256_or_bv a b (mk_u64 (v i * 32 + v jj));
    i32_to_bv_to_i32x8_inv (I.mm256_or_si256 a b) i j;
    i32_to_bv_to_i32x8_inv a i j;
    i32_to_bv_to_i32x8_inv b i j;
    ebit_is_get_bit Ints.I32 r (v jj);
    ebit_is_get_bit Ints.I32 xa (v jj);
    ebit_is_get_bit Ints.I32 xb (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits r (xa |. xb)
#pop-options
#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let mm256_sign_epi32_lemma a b i =
  reveal_opaque (`%I.mm256_sign_epi32) I.mm256_sign_epi32;
  reveal_opaque_arithmetic_ops #i32_inttype;
  Canon.lemma_mm256_sign_epi32 a b
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 250"
let i32_to_bv_to_i32x4_inv vec i j =
  bit_view_inv Ints.I32 (mk_u64 128) (mk_u64 4) vec i j
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 300"
let i32_to_bv_ext a c =
  let h (k: nat{k < 32})
    : Lemma (IVi.bval (IVi.encode_bit Ints.I32 a k) == IVi.bval (IVi.encode_bit Ints.I32 c k)) =
    assert (i32_to_bv a (mk_u64 k) == i32_to_bv c (mk_u64 k)) in
  Canon.dsum2_shift (fun b -> IVi.encode_bit Ints.I32 a b)
                    (fun b -> IVi.encode_bit Ints.I32 c b) 0 0 32 h;
  IVi.lemma_decode_encode Ints.I32 a;
  IVi.lemma_decode_encode Ints.I32 c
#pop-options
#push-options "--fuel 2 --ifuel 2 --z3rlimit 400"
let to_i8x16_mm_loadu_si128_lemma bytes nth =
  reveal_opaque (`%I.mm_loadu_si128) I.mm_loadu_si128;
  reveal_opaque (`%Extra.mm_loadu_si128_model) Extra.mm_loadu_si128_model;
  let m = I.mm_loadu_si128 bytes in
  let l : i8 = to_i8x16 m nth in
  let r : i8 = cast (Seq.index bytes (v nth)) <: i8 in
  cast_u8_i8 (Seq.index bytes (v nth));
  let aux (jj: Ints.usize{v jj < 8}) : Lemma (Ints.get_bit l jj == Ints.get_bit r jj) =
    let j = mk_u64 (v jj) in
    bit_view_inv Ints.I8 (mk_u64 128) (mk_u64 16) m nth j;
    bit_view_inv Ints.U8 (mk_u64 128) (mk_u64 16) m nth j;
    ebit_is_get_bit Ints.I8 l (v jj);
    ebit_is_get_bit Ints.U8 (to_u8x16 m nth) (v jj)
  in
  FStar.Classical.forall_intro aux;
  Ints.lemma_int_t_eq_via_bits l r
#pop-options
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let mm256_loadu_si256_u8_lemma bytes i =
  reveal_opaque (`%I.mm256_loadu_si256_u8) I.mm256_loadu_si256_u8;
  reveal_opaque (`%Extra.mm256_loadu_si256_u8_model) Extra.mm256_loadu_si256_u8_model;
  let lane = i /! mk_u64 8 in
  let bit  = i %! mk_u64 8 in
  FStar.Math.Lemmas.lemma_div_mod (v i) 8;
  u8_to_bv_to_u8x32_inv (I.mm256_loadu_si256_u8 bytes) lane bit
#pop-options
(* 128-bit i32 store: 4-deep update_at_usize ladder (mirror of `store8_i32`). *)
[@@ "opaque_to_smt"]
let store4_i32 (out: t_Slice i32 {Seq.length out >= 4}) (vec: bv128)
  : (r: t_Slice i32 {Seq.length r == Seq.length out}) =
  let upd = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize in
  let s0 = upd out (mk_usize 0) (to_i32x4 vec (mk_u64 0)) in
  let s1 = upd s0  (mk_usize 1) (to_i32x4 vec (mk_u64 1)) in
  let s2 = upd s1  (mk_usize 2) (to_i32x4 vec (mk_u64 2)) in
  upd s2 (mk_usize 3) (to_i32x4 vec (mk_u64 3))

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let store4_i32_unfold (out: t_Slice i32 {Seq.length out >= 4}) (vec: bv128)
  : Lemma (I.mm_storeu_si128_i32 out vec == store4_i32 out vec) =
  reveal_opaque (`%store4_i32) store4_i32;
  reveal_opaque (`%I.mm_storeu_si128_i32) I.mm_storeu_si128_i32;
  reveal_opaque (`%Extra.mm_storeu_si128_i32_model) Extra.mm_storeu_si128_i32_model
#pop-options

#restart-solver
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400 --split_queries always"
let store4_i32_index (out: t_Slice i32 {Seq.length out >= 4}) (vec: bv128) (i: nat{i < 4})
  : Lemma (Seq.index (store4_i32 out vec) i == to_i32x4 vec (mk_int i)) =
  reveal_opaque (`%store4_i32) store4_i32;
  match i with
  | 0 -> () | 1 -> () | 2 -> () | _ -> ()
#pop-options

#restart-solver
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let mm_storeu_si128_i32_lemma out vec i =
  store4_i32_unfold out vec;
  store4_i32_index out vec i
#pop-options
let mm_storeu_si128_i32_len_lemma out vec = ()
(* ===== movemask_ps: 8-bit base-2 fold value == flat sign-bit sum ===== *)
(* castsi256_ps is bit-identity; movemask model value = e_movemask_bit_sum_i32,
   which fuel-8 unrolls to the flat sum of lane-sign bits weighted by 2^i. *)
#push-options "--fuel 8 --ifuel 1 --z3rlimit 400"
let mm256_movemask_ps_lemma a =
  reveal_opaque (`%I.mm256_movemask_ps) I.mm256_movemask_ps;
  reveal_opaque (`%I.mm256_castsi256_ps) I.mm256_castsi256_ps;
  Canon.lemma_castsi256_ps_lift a;
  Canon.lemma_mm256_movemask_ps a;
  IV.e_movemask_bit_sum_i32_bound (Canon.to_i32x8 a) 0 8;
  assert_norm (pow2 8 == 256)
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let coeff_gather_bv_lemma a b c i =
  cast_u8_i32 a; cast_u8_i32 b; cast_u8_i32 c;
  assert_norm (pow2 23 == 8388608);
  Rust_primitives.BitVectors.get_bit_pow2_minus_one #Ints.I32 23 (Ints.mk_usize (v i));
  bit_of_get_bit Ints.I32
    (((((cast c <: i32) <<! mk_i32 16 <: i32) |. ((cast b <: i32) <<! mk_i32 8 <: i32) <: i32)
      |. (cast a <: i32) <: i32) &. mk_i32 8388607 <: i32) (v i);
  if v i >= 23 then ()
  else if v i >= 16 then bit_of_get_bit Ints.U8 c (v i - 16)
  else if v i >= 8 then bit_of_get_bit Ints.U8 b (v i - 8)
  else bit_of_get_bit Ints.U8 a (v i)
#pop-options
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let u8_to_bv_logand15_lemma x i =
  bit_of_get_bit Ints.U8 (x &. mk_u8 15) (v i);
  if v i < 4 then bit_of_get_bit Ints.U8 x (v i)
let u8_to_bv_shr4_lemma x i =
  bit_of_get_bit Ints.U8 (x >>! mk_u8 4 <: u8) (v i);
  if v i < 4 then bit_of_get_bit Ints.U8 x (v i + 4)
#pop-options
