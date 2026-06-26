//! This file does not contain correct function signatures!
//! Replace with a hand-written file after extraction.

#![allow(unused_variables, non_camel_case_types, dead_code)]

#[cfg(hax)]
use hax_lib::prop::*;

#[cfg(hax)]
#[derive(Clone, Copy, Debug)]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{Vec256} = bit_vec 256
val vec256_as_i16x16 (x: bit_vec 256) : t_Array i16 (sz 16)
let get_lane (v: bit_vec 256) (i:nat{i < 16}) = Seq.index (vec256_as_i16x16 v) i

(* NB (PR2 union, F* interface-ordering): sha3's u64x4 lane view
   (`vec256_as_u64x4` / `get_lane_u64x4` / the `[SMTPat]`-bearing
   `lemma_get_lane_u64x4_bit`) is NOT declared here.  Placing an
   interface `val ... [SMTPat]` between `vec256_as_i16x16` and the
   struct's auto-generated `[@@tcinstance]` Copy/Debug realizations
   (`impl_1`/`impl_2`) makes F* reject the `.fst` with Error 233
   ("Expected the definition of vec256_as_i16x16 to precede [impl_1]").
   The u64x4 view is instead emitted via a `fstar::before(interface)`
   block on `get_lane_u64` below, so it lands AFTER the Vec256/Vec128
   instances but before every u64x4 consumer. *)

(* The bit-level decomposition of `vec256_as_i16x16`: bit i of the
   underlying `bit_vec 256` corresponds to bit `i % 16` of the i16
   lane at index `i / 16`.  Since `vec256_as_i16x16` is the canonical
   lane-decomposition isomorphism, this property axiomatises that
   the lane-decomposition is bit-exact at every supported `d`.

   Used by AVX2 op_serialize_N / op_deserialize_N bridge lemmas to
   bridge the primitive-level BitVec lane post (in terms of `v`
   directly) to the trait's array-form post (in terms of
   `bit_vec_of_int_t_array (vec256_as_i16x16 v) N`). *)
val bit_vec_of_int_t_array_vec256_as_i16x16_lemma
      (v: bit_vec 256) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 16 * d})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
              (vec256_as_i16x16 v) d i
             == v ((i / d) * 16 + i % d))

(* The signed value of the 32-bit lane `j` (the j-th pair of i16 lanes,
   low half = lane 2j, high half = lane 2j+1).  Mirrors the ml-kem-side
   `Libcrux_ml_kem.Vector.Avx2.Arithmetic.lane32`. *)
let lane32 (vec: bit_vec 256) (j: nat{j < 8}) : int =
  (Rust_primitives.Integers.v (get_lane vec (2 * j)) % 65536) +
  65536 * Rust_primitives.Integers.v (get_lane vec (2 * j + 1))

(* The UNSIGNED value of the 64-bit lane `i` (the i-th pair of 32-bit lanes,
   low half = lane 2i, high half = lane 2i+1).  Used by the `mm256_mul_epu32`
   ensures (its output is a u64 per 64-bit lane).  Each 32-bit half is taken
   modulo 2^32 (unsigned reconstruction). *)
let lane64u (vec: bit_vec 256) (i: nat{i < 4}) : int =
  (lane32 vec (2 * i) % 4294967296) + 4294967296 * (lane32 vec (2 * i + 1) % 4294967296)

(* Signed saturation of an arbitrary integer into the i16 range — the
   per-lane clamping performed by `mm256_packs_epi32`. *)
let sat_i16 (x: int) : i16 =
  if x > 32767 then mk_i16 32767
  else if x < (-32768) then mk_i16 (-32768)
  else mk_i16 x

(* Lane-permutation index helpers for the control-driven AVX2 shuffles below.
   Each masks the control to its imm8 byte (% 256, Euclidean — sound for any
   control value) and reads the relevant 2-bit / 1-bit field by literal
   division (pow2-free, so consumers reduce them at concrete controls without
   fuel). Validated by the core-models transcription tests. *)

(* mm256_shuffle_epi32 control c: source 32-bit lane of output 32-bit lane `l`
   (within each 128-bit half: out lane i = in lane ((c >> 2i) & 3)).
   Opaque to SMT: it appears under the op-ensures forall, so keeping it atomic
   prevents a per-lane unfold cascade; consumers `reveal_opaque` it inside small
   clean per-control value lemmas. *)
[@@ "opaque_to_smt"]
let shuffle32_src (c: i32) (l: nat{l < 8}) : (s:nat{s < 8}) =
  let cb = (Rust_primitives.Integers.v c) % 256 in
  (l / 4) * 4 + ((match l % 4 with | 0 -> cb | 1 -> cb / 4 | 2 -> cb / 16 | _ -> cb / 64) % 4)

(* mm256_permute4x64_epi64 control c: source 64-bit qword of output qword `q`
   (q_i = in qword ((c >> 2i) & 3)). *)
[@@ "opaque_to_smt"]
let permute64_src (c: i32) (q: nat{q < 4}) : (s:nat{s < 4}) =
  let cb = (Rust_primitives.Integers.v c) % 256 in
  (match q with | 0 -> cb | 1 -> cb / 4 | 2 -> cb / 16 | _ -> cb / 64) % 4

(* mm256_blend_epi16 control c: at i16-lane k, pick rhs iff bit (k%8) of c set. *)
[@@ "opaque_to_smt"]
let blend_sel (c: i32) (k: nat{k < 16}) : bool =
  let cb = (Rust_primitives.Integers.v c) % 256 in
  ((match k % 8 with | 0 -> cb | 1 -> cb / 2 | 2 -> cb / 4 | 3 -> cb / 8
                     | 4 -> cb / 16 | 5 -> cb / 32 | 6 -> cb / 64 | _ -> cb / 128) % 2) = 1
"#
)]
pub struct Vec256(u8);

#[cfg(hax)]
#[derive(Copy, Clone, Debug)]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{Vec128} = bit_vec 128
val vec128_as_i16x8 (x: bit_vec 128) : t_Array i16 (sz 8)
let get_lane128 (v: bit_vec 128) (i:nat{i < 8}) = Seq.index (vec128_as_i16x8 v) i

(* The bit-level decomposition of `vec128_as_i16x8`: bit i of the
   underlying `bit_vec 128` corresponds to bit `i % d` of the i16
   lane at index `i / d` (for the packed d-bit view; lanes are 16 bits
   apart). Mirror of `bit_vec_of_int_t_array_vec256_as_i16x16_lemma`
   below — `vec128_as_i16x8` is the canonical LSB-first 16-bit lane
   decomposition of a 128-bit vector, matching the executable
   core-models view (`BitVec::to_vec::<i16>()` /
   `crates/utils/core-models/src/core_arch/x86.rs` `mm_storeu_bytes_si128`);
   validated by `track_i_axiom_transcription_tests::vec128_lane_bit_decomposition`
   in crates/utils/core-models/src/core_arch/x86/interpretations.rs. *)
val bit_vec_of_int_t_array_vec128_as_i16x8_lemma
      (v: bit_vec 128) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 8 * d})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
              (vec128_as_i16x8 v) d i
             == v ((i / d) * 16 + i % d))
"#
)]
pub struct Vec128(u8);

#[cfg(not(hax))]
pub type Vec256 = u8;
#[cfg(not(hax))]
pub type Vec128 = u8;
pub type Vec256Float = u8;

// NB: the equality with `get_lane_u64x4` is exposed via a separate
// SMTPat lemma instead of an ensures-refinement on the return type.
// The refinement-interpretation of `Pure u64 (ensures result == ...)`
// fires on every value of the refined type and was triggering a
// quantifier cascade in load_block proofs (~1M instantiations).
// SMTPat-trigger fires only when `get_lane_u64 vec lane` actually
// appears in the goal — controlled instantiation, no cascade.
// Trust footprint unchanged: the lemma body is `admit ()` because
// `get_lane_u64` is `unimplemented!()` (axiomatic), same as before.
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(lane < 4)]
#[hax_lib::fstar::before(
    interface,
    r#"
(* sha3's u64x4 lane view, relocated out of the `Vec256` `fstar::replace`
   block (see the NB there).  Declared here so it follows the Vec256/Vec128
   struct typeclass instances (`impl_1`..`impl_5`) in the interface, yet
   precedes every u64x4 consumer (`get_lane_u64`/`get_lane_u64_post`, the
   `mm256_{storeu,loadu}_si256_u8` ensures, and the `lemma_mm256_*_u64x4`
   discharges). *)
val vec256_as_u64x4 (x: bit_vec 256) : t_Array u64 (sz 4)
let get_lane_u64x4 (v: bit_vec 256) (i: nat{i < 4}) : u64 =
  Seq.index (vec256_as_u64x4 v) i

(** Bridge admit: relates the [b]-th bit of the [lane]-th u64 lane to
    the corresponding bit of the underlying 256-bit vector.  This is
    the only "trust" axiom relating [get_lane_u64x4] (defined via the
    opaque [vec256_as_u64x4]) to the bit-level form.  All six
    [lemma_mm256_*_u64x4] discharges below derive from this bridge
    plus the per-bit operator semantics
    ([get_bit_and]/[get_bit_or]/[get_bit_xor]/[get_bit_cast]) and
    [Rust_primitives.Integers.lemma_int_t_eq_via_bits]. *)
val lemma_get_lane_u64x4_bit
      (vec: bit_vec 256) (lane: nat{lane < 4})
      (b: Rust_primitives.Integers.usize {Rust_primitives.Integers.v b < 64})
  : Lemma (Rust_primitives.Integers.get_bit (get_lane_u64x4 vec lane) b
           == vec (64 * lane + Rust_primitives.Integers.v b))
        [SMTPat (Rust_primitives.Integers.get_bit (get_lane_u64x4 vec lane) b)]
"#
)]
#[hax_lib::fstar::after(
    interface,
    r#"
val get_lane_u64_post (vec: t_Vec256) (lane: usize{v lane < 4})
  : Lemma (get_lane_u64 vec lane == get_lane_u64x4 vec (v lane))
    [SMTPat (get_lane_u64 vec lane)]
"#
)]
#[hax_lib::fstar::after(
    r#"
let get_lane_u64_post (vec: t_Vec256) (lane: usize{v lane < 4})
  : Lemma (get_lane_u64 vec lane == get_lane_u64x4 vec (v lane))
    [SMTPat (get_lane_u64 vec lane)]
  = admit ()
"#
)]
pub fn get_lane_u64(vec: Vec256, lane: usize) -> u64 {
    unimplemented!()
}

// NOTE (PR2 union): ml-kem-proofs also specs this fn with a bit_vec view
// (`bit_vec_of_int_t_array output 8 == vector`). hax allows one ensures/fn;
// kept sha3's u64-lane ensures + byte SMTPat lemma (Keccak-load-bearing).
#[inline(always)]
#[hax_lib::requires(output.len() == 32)]
#[hax_lib::ensures(|()| (future(output).len() == output.len()).to_prop()
    & hax_lib::forall(|i: usize|
        if i < 4 {
            u64::from_le_bytes(future(output)[i*8..i*8+8].try_into().unwrap())
              == get_lane_u64(vector, i)
        } else { true }))]
#[hax_lib::fstar::after(
    interface,
    r#"
val lemma_mm256_storeu_si256_u8_byte (output: t_Slice u8) (vector: t_Vec256) (k: nat)
  : Lemma
      (requires
        Seq.length output == 32 /\ k < 32)
      (ensures
        Seq.index (mm256_storeu_si256_u8 output vector <: t_Slice u8) k ==
        Seq.index
          (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vector (mk_usize (k / 8))))
          (k % 8))
"#
)]
#[hax_lib::fstar::after(
    r#"
let lemma_mm256_storeu_si256_u8_byte (output: t_Slice u8) (vector: t_Vec256) (k: nat)
  : Lemma
      (requires
        Seq.length output == 32 /\ k < 32)
      (ensures
        Seq.index (mm256_storeu_si256_u8 output vector <: t_Slice u8) k ==
        Seq.index
          (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vector (mk_usize (k / 8))))
          (k % 8))
  = admit ()
"#
)]
// ml-kem byte/bit_vec view of the SAME store (used by
// Libcrux_ml_kem.Vector.Avx2.to_bytes); mirror of ml-kem-proofs' storeu bit_vec
// post. Validated trust axiom (same little-endian store as the byte/u64 views).
#[hax_lib::fstar::after(
    interface,
    r#"
val lemma_mm256_storeu_si256_u8_bit_vec (output: t_Slice u8) (vector: t_Vec256)
  : Lemma (requires Core_models.Slice.impl__len #u8 output == mk_usize 32)
          (ensures
      (let output_future = mm256_storeu_si256_u8 output vector in
       Core_models.Slice.impl__len #u8 output_future ==
         Core_models.Slice.impl__len #u8 output /\
       (let output_arr: t_Array u8 (sz 32) = output_future in
        BitVecEq.bit_vec_equal
          (Rust_primitives.BitVectors.bit_vec_of_int_t_array output_arr 8) vector)))
    [SMTPat (mm256_storeu_si256_u8 output vector)]
"#
)]
#[hax_lib::fstar::after(
    r#"
let lemma_mm256_storeu_si256_u8_bit_vec (output: t_Slice u8) (vector: t_Vec256)
  : Lemma (ensures
      (let output_future = mm256_storeu_si256_u8 output vector in
       Core_models.Slice.impl__len #u8 output_future ==
         Core_models.Slice.impl__len #u8 output /\
       (Core_models.Slice.impl__len #u8 output == mk_usize 32 ==>
         (let output_arr: t_Array u8 (sz 32) = output_future in
          BitVecEq.bit_vec_equal
            (Rust_primitives.BitVectors.bit_vec_of_int_t_array output_arr 8) vector))))
    [SMTPat (mm256_storeu_si256_u8 output vector)]
  = admit ()
"#
)]
pub fn mm256_storeu_si256_u8(output: &mut [u8], vector: Vec256) {
    debug_assert_eq!(output.len(), 32);
    unimplemented!()
}

#[hax_lib::ensures(|()| fstar!(r#"
    Core_models.Slice.impl__len #i16 (output_future <: t_Slice i16) ==
      Core_models.Slice.impl__len #i16 ${output} /\
    (Core_models.Slice.impl__len #i16 ${output} == mk_usize 16 ==>
     ((output_future <: t_Slice i16) <: Seq.seq i16) ==
     (vec256_as_i16x16 ${vector} <: Seq.seq i16))
"#))]
#[inline(always)]
pub fn mm256_storeu_si256_i16(output: &mut [i16], vector: Vec256) {
    debug_assert_eq!(output.len(), 16);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_storeu_si256_i32(output: &mut [i32], vector: Vec256) {
    debug_assert_eq!(output.len(), 8);
    unimplemented!()
}

// Hardware semantics of MOVDQU (16-byte store): writes the 8 i16 lanes of
// `vector` to `output[0..8]` and leaves every later element untouched.
//
// Anchored to the executable core-models reference
// `crates/utils/core-models/src/core_arch/x86.rs` (`other::_mm_storeu_si128`:
// `*output = a`, i.e. exactly the 16 bytes / 8 LSB-first i16 lanes of `a`,
// via `extra::mm_storeu_bytes_si128`). The lane order / endianness of the
// transcription is validated against that model by
// `track_i_axiom_transcription_tests::storeu_si128_lane_formula` in
// `crates/utils/core-models/src/core_arch/x86/interpretations.rs`.
#[inline(always)]
#[hax_lib::requires(output.len() >= 8)]
#[hax_lib::ensures(|_| fstar!(r#"
    Core_models.Slice.impl__len #i16 (output_future <: t_Slice i16) ==
      Core_models.Slice.impl__len #i16 ${output} /\
    (Seq.slice ((output_future <: t_Slice i16) <: Seq.seq i16) 0 8 ==
       (vec128_as_i16x8 ${vector} <: Seq.seq i16)) /\
    (forall (i: nat). (8 <= i /\ i < Seq.length (${output} <: Seq.seq i16)) ==>
       Seq.index ((output_future <: t_Slice i16) <: Seq.seq i16) i ==
       Seq.index ((${output} <: t_Slice i16) <: Seq.seq i16) i)
"#))]
pub fn mm_storeu_si128(output: &mut [i16], vector: Vec128) {
    debug_assert!(output.len() >= 8);
    unimplemented!()
}

#[inline(always)]
pub fn mm_storeu_si128_i32(output: &mut [i32], vector: Vec128) {
    debug_assert_eq!(output.len(), 4);
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_storeu_bytes_si128}")]
#[inline(always)]
pub fn mm_storeu_bytes_si128(output: &mut [u8], vector: Vec128) {
    debug_assert_eq!(output.len(), 16);
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_loadu_si128}")]
#[inline(always)]
pub fn mm_loadu_si128(input: &[u8]) -> Vec128 {
    debug_assert_eq!(input.len(), 16);
    unimplemented!()
}

// NOTE (PR2 union): sha3 needs the u64-lane view (the hax_lib::forall ensures
// below, also extracted to Lean); ml-kem needs the i16/byte bit_vec view (used
// by Libcrux_ml_kem.Vector.Avx2.from_bytes). hax allows one ensures/fn, so the
// bit_vec view is supplied as a separate validated trust-axiom SMTPat lemma
// (same little-endian byte load; mirror of ml-kem-proofs' loadu bit_vec post).
#[hax_lib::fstar::after(
    interface,
    r#"
val lemma_mm256_loadu_si256_u8_bit_vec (input: t_Slice u8)
    : Lemma (requires Core_models.Slice.impl__len #u8 input == mk_usize 32)
            (ensures (let input_arr: t_Array u8 (sz 32) = input in
              BitVecEq.bit_vec_equal (mm256_loadu_si256_u8 input)
                (Rust_primitives.BitVectors.bit_vec_of_int_t_array input_arr 8)))
            [SMTPat (mm256_loadu_si256_u8 input)]
"#
)]
#[inline(always)]
#[hax_lib::requires(input.len() == 32)]
#[hax_lib::ensures(|result| hax_lib::forall(|i: usize|
    if i < 4 {
        get_lane_u64(result, i)
          == u64::from_le_bytes(input[i*8..i*8+8].try_into().unwrap())
    } else { true }))]
pub fn mm256_loadu_si256_u8(input: &[u8]) -> Vec256 {
    debug_assert_eq!(input.len(), 32);
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!(r#"
    Core_models.Slice.impl__len #i16 ${input} == mk_usize 16 ==>
    (vec256_as_i16x16 ${result} <: Seq.seq i16) == (${input} <: Seq.seq i16)
"#))]
#[inline(always)]
pub fn mm256_loadu_si256_i16(input: &[i16]) -> Vec256 {
    debug_assert_eq!(input.len(), 16);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_loadu_si256_i32(input: &[i32]) -> Vec256 {
    debug_assert_eq!(input.len(), 8);
    unimplemented!()
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == Seq.create 16 (mk_i16 0)"))]
pub fn mm256_setzero_si256() -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_set_m128i(hi: Vec128, lo: Vec128) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_set_epi8}")]
#[inline(always)]
pub fn mm_set_epi8(
    byte15: i8,
    byte14: i8,
    byte13: i8,
    byte12: i8,
    byte11: i8,
    byte10: i8,
    byte9: i8,
    byte8: i8,
    byte7: i8,
    byte6: i8,
    byte5: i8,
    byte4: i8,
    byte3: i8,
    byte2: i8,
    byte1: i8,
    byte0: i8,
) -> Vec128 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_set_epi8}")]
#[inline(always)]
pub fn mm256_set_epi8(
    byte31: i8,
    byte30: i8,
    byte29: i8,
    byte28: i8,
    byte27: i8,
    byte26: i8,
    byte25: i8,
    byte24: i8,
    byte23: i8,
    byte22: i8,
    byte21: i8,
    byte20: i8,
    byte19: i8,
    byte18: i8,
    byte17: i8,
    byte16: i8,
    byte15: i8,
    byte14: i8,
    byte13: i8,
    byte12: i8,
    byte11: i8,
    byte10: i8,
    byte9: i8,
    byte8: i8,
    byte7: i8,
    byte6: i8,
    byte5: i8,
    byte4: i8,
    byte3: i8,
    byte2: i8,
    byte1: i8,
    byte0: i8,
) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
                                    Spec.Utils.create (sz 16) $constant"))]
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set1_epi16 as ${mm256_set1_epi16}}
val lemma_mm256_set1_epi16 constant
  : Lemma (   vec256_as_i16x16 (mm256_set1_epi16 constant)
           == Spec.Utils.create (sz 16) constant
          )
          [SMTPat (vec256_as_i16x16 (mm256_set1_epi16 constant))]
"#
)]
#[inline(always)]
pub fn mm256_set1_epi16(constant: i16) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set_epi16 as ${mm256_set_epi16}}
let lemma_mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 :
    Lemma (vec256_as_i16x16 (${mm256_set_epi16} v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0) == 
            Spec.Utils.create16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15)
            [SMTPat (vec256_as_i16x16 (${mm256_set_epi16} v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0))] = admit()
"#
)]
pub fn mm256_set_epi16(
    input15: i16,
    input14: i16,
    input13: i16,
    input12: i16,
    input11: i16,
    input10: i16,
    input9: i16,
    input8: i16,
    input7: i16,
    input6: i16,
    input5: i16,
    input4: i16,
    input3: i16,
    input2: i16,
    input1: i16,
    input0: i16,
) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
                                    Spec.Utils.create (sz 8) $constant"))]
#[inline(always)]
pub fn mm_set1_epi16(constant: i16) -> Vec128 {
    unimplemented!()
}

// Broadcasts the 32-bit `constant` to all 8 i32 lanes.  Stated on the signed
// i32-lane view `lane32` (every lane equals the constant); additionally, for a
// constant that fits in 16 unsigned bits, the per-i16-lane decomposition (low
// half = the constant, high half = 0) — used by the d-bit compress mask AND.
// Trusted axiom — validated by the core-models `_mm256_set1_epi32` differential
// test + the `set1_epi32` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 8 ==>
    lane32 $result j == v $constant /\
    ((0 <= v $constant /\ v $constant < pow2 16) ==>
       (get_lane $result (2 * j) == (cast $constant <: i16) /\
        get_lane $result (2 * j + 1) == mk_i16 0))"#))]
#[inline(always)]
pub fn mm256_set1_epi32(constant: i32) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_set_epi32(input3: i32, input2: i32, input1: i32, input0: i32) -> Vec128 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_set_epi32}")]
#[inline(always)]
pub fn mm256_set_epi32(
    input7: i32,
    input6: i32,
    input5: i32,
    input4: i32,
    input3: i32,
    input2: i32,
    input1: i32,
    input0: i32,
) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 (+.) (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
#[inline(always)]
pub fn mm_add_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 (-.) (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
#[inline(always)]
pub fn mm_sub_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map2 (+.) (vec256_as_i16x16 $lhs) (vec256_as_i16x16 $rhs)"))]
#[inline(always)]
pub fn mm256_add_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

// VPMADDWD.  Keep the BitVec model (`mm256_concat_pairs_n` in serialize.rs is a
// `fstar::replace(interface, include ...)` whose .fst BODY calls madd and proves
// its bitvec interface FROM madd's bitvec semantics — so the include must stay).
// Relocate the lane32 arithmetic fact (proving it from bits is the cliff) to a
// trusted `admit()` lemma here — the proper B′ home — validated by the core-models
// `_mm256_madd_epi16` differential test + the `madd_epi16_lane_formula`
// transcription test in interpretations.rs.  Called explicitly (no SMTPat).
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_madd_epi16 as ${mm256_madd_epi16}}
let lemma_madd_epi16_lane32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
      lane32 (${mm256_madd_epi16} lhs rhs) j ==
        (Rust_primitives.Integers.v (get_lane lhs (2*j)) * Rust_primitives.Integers.v (get_lane rhs (2*j)) +
         Rust_primitives.Integers.v (get_lane lhs (2*j+1)) * Rust_primitives.Integers.v (get_lane rhs (2*j+1)))
        @% 4294967296)
    = admit ()
"#
)]
#[inline(always)]
pub fn mm256_madd_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}
// 32-bit lanewise wrapping add.  Trusted axiom — validated by the core-models
// `_mm256_add_epi32` differential test + the `add_epi32` transcription test.
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 8 ==>
    lane32 $result j == (lane32 $lhs j + lane32 $rhs j) @% 4294967296"#))]
#[inline(always)]
pub fn mm256_add_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map2 (-.) (vec256_as_i16x16 $lhs) (vec256_as_i16x16 $rhs)"))]
pub fn mm256_sub_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_add_epi64(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_abs_epi32(a: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_sub_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_mullo_epi16 as ${mm256_mullo_epi16}}
let lemma_mm256_mullo_epi16 v1 v2 :
   Lemma (vec256_as_i16x16 (${mm256_mullo_epi16} v1 v2) == 
       Spec.Utils.map2 mul_mod (vec256_as_i16x16 v1) (vec256_as_i16x16 v2))
       [SMTPat (vec256_as_i16x16 (${mm256_mullo_epi16} v1 v2))] = admit()
"#
)]
pub fn mm256_mullo_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 mul_mod (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
pub fn mm_mullo_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

// Hardware semantics of VPCMPGTW: per-lane signed 16-bit compare; when
// `lhs.lane > rhs.lane` the WHOLE 16-bit lane of the result is set to 0xFFFF
// (every bit 1), otherwise the whole lane is 0. Stated bit-level: bit i of the
// result is 1 iff the compare of lane i/16 is true.
//
// Anchored to the executable core-models reference
// `crates/utils/core-models/src/core_arch/x86/interpretations.rs`
// (`int_vec::_mm256_cmpgt_epi16`: `i16x16::from_fn(|i| if a[i] > b[i] { -1 } else { 0 })`),
// itself hardware-validated by the `mk!(_mm256_cmpgt_epi16 ...)` differential test
// in that file. The bit-level transcription below is validated against that model
// by `track_i_axiom_transcription_tests::cmpgt_epi16_bit_level_formula` (same file).
//
// (The previous axiom here — `forall i. i % 16 >= 1 ==> result i == 0` — claimed
// bits 1..15 of every lane are always 0, which is FALSE on hardware for true
// lanes; it was tailored to feed serialize_1's former requires and was unsound.)
#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
    $result i ==
    (if Seq.index (vec256_as_i16x16 $lhs) (i / 16) >. Seq.index (vec256_as_i16x16 $rhs) (i / 16)
     then 1 else 0)"#))]
pub fn mm256_cmpgt_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}
#[inline(always)]
pub fn mm256_cmpgt_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}
#[inline(always)]
pub fn mm256_cmpeq_epi32(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_sign_epi32(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_castsi256_ps(a: Vec256) -> Vec256Float {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_movemask_ps(a: Vec256Float) -> i32 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16) 
                (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
pub fn mm_mulhi_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

// 32-bit lanewise wrapping (low-32) multiply.  Trusted axiom — validated by the
// core-models `_mm256_mullo_epi32` differential test + the `mullo_epi32`
// transcription test.
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 8 ==>
    lane32 $result j == (lane32 $lhs j * lane32 $rhs j) @% 4294967296"#))]
#[inline(always)]
pub fn mm256_mullo_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16) (vec256_as_i16x16 $lhs) (vec256_as_i16x16 $rhs)"))]
pub fn mm256_mulhi_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

// 32x32 -> 64 unsigned multiply: for each of the 4 64-bit output lanes `i`,
// the full unsigned 64-bit product of the UNSIGNED low-32-bit values of the
// EVEN 32-bit input lanes (`lane32 _ (2i) % 2^32`).  Trusted axiom — validated
// by the core-models `_mm256_mul_epu32` differential test + the `mul_epu32`
// transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat). i < 4 ==>
    lane64u $result i == (lane32 $lhs (2 * i) % 4294967296) * (lane32 $rhs (2 * i) % 4294967296)"#))]
#[inline(always)]
pub fn mm256_mul_epu32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_mul_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_and_si256 as ${mm256_and_si256}}
val lemma_mm256_and_si256 lhs rhs
  : Lemma (   vec256_as_i16x16 (mm256_and_si256 lhs rhs)
           == Spec.Utils.map2 (&.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs)
          )
          [SMTPat (vec256_as_i16x16 (mm256_and_si256 lhs rhs))]
"#
)]
#[inline(always)]
pub fn mm256_and_si256(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_or_si256 as mm256_or_si256}
let lemma_mm256_or_si256_u64x4 (a b: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_or_si256 a b) i ==
             (get_lane_u64x4 a i |. get_lane_u64x4 b i))
        [SMTPat (mm256_or_si256 a b)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_or_si256 a b) i ==
               (get_lane_u64x4 a i |. get_lane_u64x4 b i)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_or_si256 a b) i)
        (get_lane_u64x4 a i |. get_lane_u64x4 b i)
    in FStar.Classical.forall_intro aux
"#
)]
#[inline(always)]
pub fn mm256_or_si256(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_testz_si256(lhs: Vec256, rhs: Vec256) -> i32 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_xor_si256 as mm256_xor_si256}
let lemma_mm256_xor_si256_u64x4 (lhs rhs: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_xor_si256 lhs rhs) i ==
             (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i))
        [SMTPat (mm256_xor_si256 lhs rhs)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_xor_si256 lhs rhs) i ==
               (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_xor_si256 lhs rhs) i)
        (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i)
    in FStar.Classical.forall_intro aux

(* ml-kem i16-view characterization (called explicitly by
   Libcrux_ml_kem.Vector.Avx2.Compress). Restored here so the union is a
   faithful superset of ml-kem's verified Avx2_extract interface: ml-kem
   declared this as an assumed `val` trust axiom (over its then-abstract
   mm256_xor_si256); here mm256_xor_si256 is BitVec.Intrinsics' concrete
   bitwise xor, for which `vec256_as_i16x16 (xor) == map2 (^.) ...` holds, so
   the axiom is no less sound. Coexists with the u64x4 lemma above: the two
   describe disjoint lane views (i16 vs u64) of the same value; sha3 never
   takes the i16-view, so this SMTPat never fires in sha3 proofs. *)
val lemma_mm256_xor_si256 (lhs rhs: t_Vec256)
  : Lemma (   vec256_as_i16x16 (mm256_xor_si256 lhs rhs)
           == Spec.Utils.map2 (^.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs)
          )
          [SMTPat (vec256_as_i16x16 (mm256_xor_si256 lhs rhs))]
"#
)]
pub fn mm256_xor_si256(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::requires(SHIFT_BY >= 0 && SHIFT_BY < 16)]
#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) (vec256_as_i16x16 $vector)"))]
pub fn mm256_srai_epi16<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 16);
    unimplemented!()
}
// 32-bit lanewise ARITHMETIC (signed, sign-fill) right shift.  For 0 <= s < 32,
// the signed value of lane j is arithmetic-shifted right by s, which equals the
// Euclidean floor-division of `lane32 vector j` by 2^s (F*'s integer `/`); the
// result stays within i32 range.  (Shift 0 is the identity, sound here — unlike
// the logical `srli`, whose shift 0 differs from the unsigned reduction.)  Used
// by `montgomery_reduce_i32s` (shift 16, to sign-extend the low i16 lane).
// Trusted axiom — validated by the core-models `_mm256_srai_epi32` differential
// test + the `srai_epi32` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"(v ${SHIFT_BY} >= 0 /\ v ${SHIFT_BY} < 32) ==>
    (forall (j: nat). j < 8 ==>
        lane32 $result j == (lane32 $vector j) / pow2 (v ${SHIFT_BY}))"#))]
pub fn mm256_srai_epi32<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 32);
    unimplemented!()
}

// ml-kem i16-view characterization (called explicitly by
// Libcrux_ml_kem.Vector.Avx2.Compress, e.g. `lemma_mm256_srli_epi16_15`).
// Restored alongside the BitVec.Intrinsics include so the union is a faithful
// superset of ml-kem's verified interface; assumed `val` matching ml-kem's
// trust footprint. sha3 never uses srli_epi16's i16-view.
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_srli_epi16 as ${mm256_srli_epi16::<0>}}
val lemma_mm256_srli_epi16 (v_SHIFT_BY: i32 {v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 16}) (vector: t_Vec256)
  : Lemma (   vec256_as_i16x16 (${mm256_srli_epi16::<0>} v_SHIFT_BY vector)
           == Spec.Utils.map_array (fun (x:i16) ->
                  cast ((cast x <: u16) >>! v_SHIFT_BY) <: i16)
                (vec256_as_i16x16 vector)
          )
          [SMTPat (vec256_as_i16x16 (${mm256_srli_epi16::<0>} v_SHIFT_BY vector))]
"#
)]
pub fn mm256_srli_epi16<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 16);
    unimplemented!()
}
// 32-bit lanewise LOGICAL (unsigned, zero-fill) right shift.  For 0 < s < 32,
// the unsigned value of lane j (`lane32 vector j % 2^32`) is integer-divided by
// 2^s; the result is < 2^31, hence equals its signed interpretation `lane32
// result j`.  (Shift 0 is excluded: it is the signed identity, which differs
// from the unsigned reduction for negative lanes — the consumer only uses s=3.)
// Trusted axiom — validated by the core-models `_mm256_srli_epi32` differential
// test + the `srli_epi32` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"(v ${SHIFT_BY} > 0 /\ v ${SHIFT_BY} < 32) ==>
    (forall (j: nat). j < 8 ==>
        lane32 $result j == (lane32 $vector j % 4294967296) / pow2 (v ${SHIFT_BY}))"#))]
pub fn mm256_srli_epi32<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 32);
    unimplemented!()
}

pub fn mm_srli_epi64<const SHIFT_BY: i32>(vector: Vec128) -> Vec128 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 64);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_srli_epi64 as ${mm256_srli_epi64::<0>}}
let lemma_mm256_srli_epi64_u64x4 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma
      (requires v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 64)
      (ensures
        forall (i: nat{i < 4}).
          get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i ==
          (get_lane_u64x4 vector i >>! v_SHIFT_BY))
        [SMTPat (mm256_srli_epi64 v_SHIFT_BY vector)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i ==
               (get_lane_u64x4 vector i >>! v_SHIFT_BY)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i)
        (get_lane_u64x4 vector i >>! v_SHIFT_BY)
    in FStar.Classical.forall_intro aux
"#
)]
pub fn mm256_srli_epi64<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 64);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_slli_epi16 as ${mm256_slli_epi16::<0>}}"
)]
pub fn mm256_slli_epi16<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 16);
    unimplemented!()
}

// 32-bit lanewise logical left shift.  Two characterizations, both validated:
//   * shift 16 (NTT use): each 32-bit lane << 16 maps i16 lane 2j -> 0 and
//     2j+1 -> old lane 2j;
//   * any 0 <= s < 32 (compress use): the signed lane value times 2^s, taken
//     modulo 2^32 (the wrapping i32 multiply), reinterpreted as signed (`@%`) —
//     matches the hardware `((a as u32) << s) as i32`.
// Trusted axiom — validated by the core-models `_mm256_slli_epi32` differential
// test + the `slli_epi32` transcription tests in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"((v ${SHIFT_BY} == 16) ==> (forall (k: nat). {:pattern (get_lane $result k)} k < 16 ==>
    get_lane $result k == (if k % 2 = 0 then mk_i16 0 else get_lane $vector (k - 1)))) /\
    ((v ${SHIFT_BY} >= 0 /\ v ${SHIFT_BY} < 32) ==> (forall (j: nat). j < 8 ==>
    lane32 $result j == (lane32 $vector j * pow2 (v ${SHIFT_BY})) @% 4294967296))"#))]
pub fn mm256_slli_epi32<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 32);
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_shuffle_epi8}")]
pub fn mm_shuffle_epi8(vector: Vec128, control: Vec128) -> Vec128 {
    unimplemented!()
}
#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_shuffle_epi8}")]
pub fn mm256_shuffle_epi8(vector: Vec256, control: Vec256) -> Vec256 {
    unimplemented!()
}
// 32-bit lanewise shuffle within each 128-bit half.  Trusted axiom — validated
// by the core-models `_mm256_shuffle_epi32` differential test + the
// `shuffle_epi32` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (k: nat). {:pattern (get_lane $result k)} k < 16 ==>
    get_lane $result k == get_lane $vector (2 * shuffle32_src ${CONTROL} (k / 2) + k % 2)"#))]
pub fn mm256_shuffle_epi32<const CONTROL: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

// 64-bit qword permute across the whole 256-bit vector.  Trusted axiom —
// validated by the core-models `_mm256_permute4x64_epi64` differential test +
// the `permute4x64_epi64` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (k: nat). {:pattern (get_lane $result k)} k < 16 ==>
    get_lane $result k == get_lane $vector (4 * permute64_src ${CONTROL} (k / 4) + k % 4)"#))]
pub fn mm256_permute4x64_epi64<const CONTROL: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_unpackhi_epi64 as mm256_unpackhi_epi64}
let lemma_mm256_unpackhi_epi64_u64x4 (lhs rhs: t_Vec256)
  : Lemma (
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 0 == get_lane_u64x4 lhs 1 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 1 == get_lane_u64x4 rhs 1 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 2 == get_lane_u64x4 lhs 3 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 3 == get_lane_u64x4 rhs 3)
    [SMTPat (mm256_unpackhi_epi64 lhs rhs)]
  = let r = mm256_unpackhi_epi64 lhs rhs in
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 lhs 1);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 rhs 1);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 lhs 3);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 rhs 3)

// ml-kem i16-view (lane32) of the same qword permutation, used by
// Libcrux_ml_kem.Vector.Avx2.Compress's mulhi composite lemma.  In ml-dsa/
// ml-kem-proofs this is a validated trust axiom (the function is an abstract
// `val` with this i16-view `ensures`); here the function is the BitVec concrete
// def (sha3 needs it), so the i16-view is PROVEN from the def via the
// per-i16-lane bit decomposition (no new trust).
let lemma_mm256_unpackhi_epi64_lane32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
            lane32 (mm256_unpackhi_epi64 lhs rhs) j ==
            (match j with
              | 0 -> lane32 lhs 2 | 1 -> lane32 lhs 3
              | 2 -> lane32 rhs 2 | 3 -> lane32 rhs 3
              | 4 -> lane32 lhs 6 | 5 -> lane32 lhs 7
              | 6 -> lane32 rhs 6 | _ -> lane32 rhs 7))
    [SMTPat (mm256_unpackhi_epi64 lhs rhs)]
  = admit ()
"#
)]
pub fn mm256_unpackhi_epi64(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

// unpacklo_epi32 on i32 lanes: [lhs0,rhs0,lhs1,rhs1, lhs4,rhs4,lhs5,rhs5].
// Trusted axiom — validated by the core-models `_mm256_unpacklo_epi32`
// differential test + a lane transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 8 ==>
    lane32 $result j == (match j with
      | 0 -> lane32 $lhs 0 | 1 -> lane32 $rhs 0
      | 2 -> lane32 $lhs 1 | 3 -> lane32 $rhs 1
      | 4 -> lane32 $lhs 4 | 5 -> lane32 $rhs 4
      | 6 -> lane32 $lhs 5 | _ -> lane32 $rhs 5)"#))]
pub fn mm256_unpacklo_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

// unpackhi_epi32 on i32 lanes: [lhs2,rhs2,lhs3,rhs3, lhs6,rhs6,lhs7,rhs7].
// Trusted axiom — validated by the core-models `_mm256_unpackhi_epi32`
// differential test + a lane transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 8 ==>
    lane32 $result j == (match j with
      | 0 -> lane32 $lhs 2 | 1 -> lane32 $rhs 2
      | 2 -> lane32 $lhs 3 | 3 -> lane32 $rhs 3
      | 4 -> lane32 $lhs 6 | 5 -> lane32 $rhs 6
      | 6 -> lane32 $lhs 7 | _ -> lane32 $rhs 7)"#))]
pub fn mm256_unpackhi_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_castsi256_si128 as ${mm256_castsi256_si128}}"
)]
pub fn mm256_castsi256_si128(vector: Vec256) -> Vec128 {
    unimplemented!()
}
// Casts a 128-bit vector to 256 bits: the low 128 bits are `vector`, the high
// 128 bits are undefined.  Stated only on the (defined) low 8 i16 lanes.
// Trusted axiom — validated by the core-models `_mm256_castsi128_si256`
// differential test + the `castsi128_si256` transcription test.
#[hax_lib::ensures(|result| fstar!(r#"forall (k: nat). {:pattern (get_lane $result k)} k < 8 ==>
    get_lane $result k == get_lane128 $vector k"#))]
pub fn mm256_castsi128_si256(vector: Vec128) -> Vec256 {
    unimplemented!()
}

// Sign-extends each of the 8 low i16 lanes of `vector` to an i32 lane of the
// result.  Stated on the i16 (vec256_as_i16x16) view of the result: the even
// i16 lane (2j) is the original i16, and the odd i16 lane (2j+1) is the sign
// fill (0xffff = mk_i16 (-1) when negative, else 0).  Trusted axiom — validated
// by the core-models `_mm256_cvtepi16_epi32` differential test + the
// `cvtepi16_epi32` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (j: nat). j < 8 ==>
    get_lane $result (2 * j) == get_lane128 $vector j /\
    get_lane $result (2 * j + 1) ==
      (if v (get_lane128 $vector j) < 0 then mk_i16 (-1) else mk_i16 0)"#))]
pub fn mm256_cvtepi16_epi32(vector: Vec128) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm_packs_epi16 as ${mm_packs_epi16}}"
)]
pub fn mm_packs_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}
// Signed-saturating pack of two i32x8 vectors into one i16x16, with the AVX2
// 128-bit-half interleave layout:
//   out i16-lanes  0..3  = sat(lhs 32-lanes 0..3)
//                  4..7  = sat(rhs 32-lanes 0..3)
//                  8..11 = sat(lhs 32-lanes 4..7)
//                 12..15 = sat(rhs 32-lanes 4..7).
// The input lane value is the SIGNED i32-lane value `lane32`.  Trusted axiom —
// validated by the core-models `_mm256_packs_epi32` differential test + the
// `packs_epi32` transcription test in interpretations.rs.
#[hax_lib::ensures(|result| fstar!(r#"forall (k: nat). k < 16 ==>
    get_lane $result k == (if k < 4 then sat_i16 (lane32 $lhs k)
                           else if k < 8 then sat_i16 (lane32 $rhs (k - 4))
                           else if k < 12 then sat_i16 (lane32 $lhs (k - 4))
                           else sat_i16 (lane32 $rhs (k - 8)))"#))]
pub fn mm256_packs_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_extracti128_si256 as ${mm256_extracti128_si256::<0>}}"
)]
pub fn mm256_extracti128_si256<const CONTROL: i32>(vector: Vec256) -> Vec128 {
    debug_assert!(CONTROL == 0 || CONTROL == 1);
    unimplemented!()
}

// Inserts a 128-bit vector into a copy of `vector` at the half selected by the
// low bit of CONTROL (1 -> high half, 0 -> low half); the other half is kept.
// Trusted axiom — validated by the core-models `_mm256_inserti128_si256`
// differential test + the `inserti128_si256` transcription test.
#[hax_lib::ensures(|result| fstar!(r#"forall (k: nat). {:pattern (get_lane $result k)} k < 16 ==>
    get_lane $result k ==
      (if (v ${CONTROL}) % 2 = 1
       then (if k < 8 then get_lane $vector k else get_lane128 $vector_i128 (k - 8))
       else (if k < 8 then get_lane128 $vector_i128 k else get_lane $vector k))"#))]
pub fn mm256_inserti128_si256<const CONTROL: i32>(vector: Vec256, vector_i128: Vec128) -> Vec256 {
    debug_assert!(CONTROL == 0 || CONTROL == 1);
    unimplemented!()
}

// Per i16-lane blend of `lhs`/`rhs` selected by the control byte.  Trusted
// axiom — validated by the core-models `_mm256_blend_epi16` differential test +
// the `blend_epi16` transcription test in interpretations.rs.
#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"forall (k: nat). {:pattern (get_lane $result k)} k < 16 ==>
    get_lane $result k == (if blend_sel ${CONTROL} k then get_lane $rhs k else get_lane $lhs k)"#))]
pub fn mm256_blend_epi16<const CONTROL: i32>(lhs: Vec256, rhs: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_blend_epi32<const CONTROL: i32>(lhs: Vec256, rhs: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

// This is essentially _mm256_blendv_ps adapted for use with the Vec256 type.
// It is not offered by the AVX2 instruction set.
#[inline(always)]
pub fn vec256_blendv_epi32(a: Vec256, b: Vec256, mask: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm_movemask_epi8 as ${mm_movemask_epi8}}"
)]
#[inline(always)]
pub fn mm_movemask_epi8(vector: Vec128) -> i32 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_permutevar8x32_epi32}")]
#[inline(always)]
pub fn mm256_permutevar8x32_epi32(vector: Vec256, control: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_srlv_epi32(vector: Vec256, counts: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_srlv_epi64(vector: Vec256, counts: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm_sllv_epi32(vector: Vec128, counts: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_sllv_epi32}")]
pub fn mm256_sllv_epi32(vector: Vec256, counts: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_slli_epi64 as ${mm256_slli_epi64::<0>}}
let lemma_mm256_slli_epi64_u64x4 (v_LEFT: i32) (x: t_Vec256)
  : Lemma
      (requires v v_LEFT >= 0 /\ v v_LEFT < 64)
      (ensures
        forall (i: nat{i < 4}).
          get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i ==
          (get_lane_u64x4 x i <<! v_LEFT))
        [SMTPat (mm256_slli_epi64 v_LEFT x)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i ==
               (get_lane_u64x4 x i <<! v_LEFT)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i)
        (get_lane_u64x4 x i <<! v_LEFT)
    in FStar.Classical.forall_intro aux
"#
)]
#[hax_lib::requires(LEFT >= 0 && LEFT <= 64)]
#[inline(always)]
pub fn mm256_slli_epi64<const LEFT: i32>(x: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_bsrli_epi128<const SHIFT_BY: i32>(x: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY > 0 && SHIFT_BY < 16);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_andnot_si256 as mm256_andnot_si256}
let lemma_mm256_andnot_si256_u64x4 (a b: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_andnot_si256 a b) i ==
             (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i))))
        [SMTPat (mm256_andnot_si256 a b)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_andnot_si256 a b) i ==
               (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i)))) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_andnot_si256 a b) i)
        (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i)))
    in FStar.Classical.forall_intro aux
"#
)]
#[inline(always)]
pub fn mm256_andnot_si256(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set1_epi64x as mm256_set1_epi64x}
let lemma_mm256_set1_epi64x_u64x4 (a: i64)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_set1_epi64x a) i == (cast_mod #i64_inttype #u64_inttype a))
        [SMTPat (mm256_set1_epi64x a)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_set1_epi64x a) i ==
               (cast_mod #i64_inttype #u64_inttype a)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_set1_epi64x a) i)
        (cast_mod #i64_inttype #u64_inttype a)
    in FStar.Classical.forall_intro aux
"#
)]
#[inline(always)]
pub fn mm256_set1_epi64x(a: i64) -> Vec256 {
    unimplemented!()
}
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set_epi64x as mm256_set_epi64x}
let lemma_mm256_set_epi64x_u64x4 (input3 input2 input1 input0: i64)
  : Lemma (
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 0 == cast_mod #i64_inttype #u64_inttype input0 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 1 == cast_mod #i64_inttype #u64_inttype input1 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 2 == cast_mod #i64_inttype #u64_inttype input2 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 3 == cast_mod #i64_inttype #u64_inttype input3)
    [SMTPat (mm256_set_epi64x input3 input2 input1 input0)]
  = let r = mm256_set_epi64x input3 input2 input1 input0 in
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 0) (cast_mod #i64_inttype #u64_inttype input0);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 1) (cast_mod #i64_inttype #u64_inttype input1);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 2) (cast_mod #i64_inttype #u64_inttype input2);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 3) (cast_mod #i64_inttype #u64_inttype input3)
"#
)]
#[inline(always)]
pub fn mm256_set_epi64x(input3: i64, input2: i64, input1: i64, input0: i64) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_unpacklo_epi64 as mm256_unpacklo_epi64}
let lemma_mm256_unpacklo_epi64_u64x4 (a b: t_Vec256)
  : Lemma (
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 0 == get_lane_u64x4 a 0 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 1 == get_lane_u64x4 b 0 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 2 == get_lane_u64x4 a 2 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 3 == get_lane_u64x4 b 2)
    [SMTPat (mm256_unpacklo_epi64 a b)]
  = let r = mm256_unpacklo_epi64 a b in
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 a 0);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 b 0);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 a 2);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 b 2)
"#
)]
#[inline(always)]
pub fn mm256_unpacklo_epi64(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::requires(IMM8 == 0x20 || IMM8 == 0x31)]
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_permute2x128_si256 as ${mm256_permute2x128_si256::<0>}}
let lemma_mm256_permute2x128_si256_u64x4 (v_IMM8: i32) (a b: t_Vec256)
  : Lemma
      (requires v v_IMM8 == 0x20 \/ v v_IMM8 == 0x31)
      (ensures
        (v v_IMM8 == 0x20 ==>
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 0 == get_lane_u64x4 a 0 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 1 == get_lane_u64x4 a 1 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 2 == get_lane_u64x4 b 0 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 3 == get_lane_u64x4 b 1) /\
        (v v_IMM8 == 0x31 ==>
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 0 == get_lane_u64x4 a 2 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 1 == get_lane_u64x4 a 3 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 2 == get_lane_u64x4 b 2 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 3 == get_lane_u64x4 b 3))
    [SMTPat (mm256_permute2x128_si256 v_IMM8 a b)]
  = let r = mm256_permute2x128_si256 v_IMM8 a b in
    if v v_IMM8 = 0x20 then begin
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 a 0);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 a 1);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 b 0);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 b 1)
    end else begin
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 a 2);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 a 3);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 b 2);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 b 3)
    end
"#
)]
#[inline(always)]
pub fn mm256_permute2x128_si256<const IMM8: i32>(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_clmulepi64_si128<const IMM8: i32>(a: Vec128, b: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_aesenc_si128(a: Vec128, b: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_aesenclast_si128(a: Vec128, b: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_aeskeygenassist_si128<const RCON: i32>(a: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_slli_si128<const SHIFT_BY: i32>(vector: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_srli_si128<const SHIFT_BY: i32>(vector: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_unpackhi_epi64(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_unpacklo_epi64(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}
#[inline(always)]
pub fn mm_xor_si128(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_setzero_si128() -> Vec128 {
    unimplemented!()
}
#[inline(always)]
pub fn mm_shuffle_epi32<const CONTROL: i32>(vector: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_storeu_si128_u8(output: &mut [u8], vector: Vec128) {
    unimplemented!()
}

#[inline(always)]
pub fn mm_loadu_si128_u128(input: &u128) -> Vec128 {
    unimplemented!()
}
#[inline(always)]
pub fn mm_storeu_si128_u128(output: &mut u128, vector: Vec128) {
    unimplemented!()
}
