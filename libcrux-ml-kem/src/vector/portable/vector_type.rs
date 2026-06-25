use crate::vector::traits::FIELD_ELEMENTS_IN_VECTOR;
use libcrux_secrets::*;

/// Values having this type hold a representative 'x' of the ML-KEM field.
/// We use 'fe' as a shorthand for this type.
pub(crate) type FieldElement = I16;

#[derive(Clone, Copy)]
pub struct PortableVector {
    pub(crate) elements: [FieldElement; FIELD_ELEMENTS_IN_VECTOR],
}

// Bit-bridge helper lemmas for from_bytes/to_bytes (scalar LE byte packing).
// Proven in clean context: the per-byte/coefficient `get_bit` decomposition
// of the `<<8 | ` combine (resp. the `>>8`/truncate split) via get_bit_shl/shr
// + the secret-int cast transparency (f_as_i16/f_as_u8), assembled into the
// `bit_vec_of_int_t_array` byte/i16 view equality (from/to_le_bytes_post_N).
#[hax_lib::fstar::before(
    r#"
let le_byte_bit (b_lo b_hi: u8) (j: nat{j < 16})
    : Lemma
      (get_bit (((Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve b_hi <: i16) <<!
              mk_i32 8 <: i16) |.
            (Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve b_lo <: i16) <: i16)
          (mk_usize j) ==
        (if j < 8 then get_bit b_lo (mk_usize j) else get_bit b_hi (mk_usize (j - 8)))) =
  Rust_primitives.Integers.get_bit_shl
    (Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve b_hi) (mk_i32 8) (mk_usize j)

#push-options "--z3rlimit 300 --split_queries always --z3refresh"
let from_bytes_bit_bridge (array: t_Slice u8) (elements: t_Array i16 (mk_usize 16))
    : Lemma
      (requires
        Seq.length array >= 32 /\
        (forall (j: nat). j < 16 ==>
            Seq.index elements j ==
            (((Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve
                    (Seq.index array (2 * j + 1) <: u8) <: i16) <<! mk_i32 8 <: i16) |.
              (Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve
                  (Seq.index array (2 * j) <: u8) <: i16) <: i16)))
      (ensures
        Rust_primitives.BitVectors.bit_vec_of_int_t_array (Seq.slice array 0 32 <: t_Array u8 (sz 32)) 8 ==
        Rust_primitives.BitVectors.bit_vec_of_int_t_array elements 16) =
  let head:t_Array u8 (sz 32) = Seq.slice array 0 32 in
  introduce forall (i: nat{i < 256}).
      Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i ==
      Rust_primitives.BitVectors.bit_vec_of_int_t_array elements 16 i
  with
    (FStar.Math.Lemmas.euclidean_division_definition i 16;
      FStar.Math.Lemmas.lemma_div_plus (i % 16) (2 * (i / 16)) 8;
      FStar.Math.Lemmas.lemma_mod_plus (i % 16) (2 * (i / 16)) 8;
      le_byte_bit (Seq.index array (2 * (i / 16)) <: u8) (Seq.index array (2 * (i / 16) + 1) <: u8) (i % 16);
      Seq.lemma_index_slice array 0 32 (i / 8));
  BitVecEq.bit_vec_equal_intro (Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8)
    (Rust_primitives.BitVectors.bit_vec_of_int_t_array elements 16)
#pop-options

let byte_le_bit (e: i16) (j: nat{j < 16})
    : Lemma
      (get_bit e (mk_usize j) ==
        (if j < 8
         then get_bit (Libcrux_secrets.Int.f_as_u8 #i16 #FStar.Tactics.Typeclasses.solve e) (mk_usize j)
         else get_bit (Libcrux_secrets.Int.f_as_u8 #i16 #FStar.Tactics.Typeclasses.solve
                 (e >>! mk_i32 8 <: i16)) (mk_usize (j - 8)))) =
  if j >= 8 then Rust_primitives.Integers.get_bit_shr e (mk_i32 8) (mk_usize (j - 8))

#push-options "--z3rlimit 300 --split_queries always --z3refresh"
let to_bytes_bit_bridge (x: t_PortableVector) (bytes: t_Slice u8)
    : Lemma
      (requires
        Seq.length bytes >= 32 /\
        (forall (k: nat). k < 16 ==>
            (Seq.index bytes (2 * k) ==
              Libcrux_secrets.Int.f_as_u8 #i16 #FStar.Tactics.Typeclasses.solve
                (Seq.index x.f_elements k <: i16)) /\
            (Seq.index bytes (2 * k + 1) ==
              Libcrux_secrets.Int.f_as_u8 #i16 #FStar.Tactics.Typeclasses.solve
                ((Seq.index x.f_elements k <: i16) >>! mk_i32 8 <: i16))))
      (ensures
        Rust_primitives.BitVectors.bit_vec_of_int_t_array x.f_elements 16 ==
        Rust_primitives.BitVectors.bit_vec_of_int_t_array (Seq.slice bytes 0 32 <: t_Array u8 (sz 32)) 8) =
  let head:t_Array u8 (sz 32) = Seq.slice bytes 0 32 in
  introduce forall (i: nat{i < 256}).
      Rust_primitives.BitVectors.bit_vec_of_int_t_array x.f_elements 16 i ==
      Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8 i
  with
    (FStar.Math.Lemmas.euclidean_division_definition i 16;
      FStar.Math.Lemmas.lemma_div_plus (i % 16) (2 * (i / 16)) 8;
      FStar.Math.Lemmas.lemma_mod_plus (i % 16) (2 * (i / 16)) 8;
      byte_le_bit (Seq.index x.f_elements (i / 16) <: i16) (i % 16);
      Seq.lemma_index_slice bytes 0 32 (i / 8));
  BitVecEq.bit_vec_equal_intro (Rust_primitives.BitVectors.bit_vec_of_int_t_array x.f_elements 16)
    (Rust_primitives.BitVectors.bit_vec_of_int_t_array head 8)
#pop-options
"#
)]
#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"${result}.f_elements == Seq.create 16 (mk_i16 0)"#))]
pub fn zero() -> PortableVector {
    PortableVector {
        elements: [0i16; FIELD_ELEMENTS_IN_VECTOR].classify(),
    }
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"${result} == ${x}.f_elements"#))]
pub fn to_i16_array(x: PortableVector) -> [I16; 16] {
    x.elements
}

// NOTE: The extracted F* for this function needs patching after re-extraction.
// hax extracts `array[0..16].try_into().unwrap()` which Z3 can't prove equals `array`
// when len(array)==16 due to opaque Core_models indexing/conversion lemmas.
// Apply: Libcrux_ml_kem.Vector.Portable.Vector_type.fst.patch
#[inline(always)]
#[hax_lib::requires(array.len() == 16)]
#[hax_lib::ensures(|result| fstar!(r#"${result}.f_elements == $array"#))]
pub fn from_i16_array(array: &[I16]) -> PortableVector {
    PortableVector {
        elements: array[0..16].try_into().unwrap(),
    }
}

#[inline(always)]
#[hax_lib::requires(array.len() >= 32)]
#[hax_lib::ensures(|result| fstar!(r#"
    Core_models.Slice.impl__len #u8 ${array} >=. mk_usize 32 ==>
    (let head : t_Slice u8 = Seq.slice ${array} 0 32 in
     Libcrux_ml_kem.Vector.Traits.Spec.from_le_bytes_post_N
       #(mk_usize 16) head ${result}.f_elements)
"#))]
pub(super) fn from_bytes(array: &[U8]) -> PortableVector {
    let mut elements = [I16(0); FIELD_ELEMENTS_IN_VECTOR];
    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"(forall (j: nat). j < v i ==>
            Seq.index ${elements} j ==
            (((Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve
                    (Seq.index ${array} (2 * j + 1) <: u8) <: i16) <<! mk_i32 8 <: i16) |.
              (Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve
                  (Seq.index ${array} (2 * j) <: u8) <: i16) <: i16))"#
            )
        });
        elements[i] = (array[2 * i + 1].as_i16()) << 8 | array[2 * i].as_i16();
    }
    let result = PortableVector { elements };
    hax_lib::fstar!(r#"from_bytes_bit_bridge ${array} ${result}.f_elements"#);
    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() >= 32)]
#[hax_lib::ensures(|_| fstar!(r#"
    Core_models.Slice.impl__len #u8 (bytes_future <: t_Slice u8) ==
      Core_models.Slice.impl__len #u8 ${bytes} /\
    (Core_models.Slice.impl__len #u8 ${bytes} >=. mk_usize 32 ==>
     (let head : t_Slice u8 = Seq.slice bytes_future 0 32 in
      Libcrux_ml_kem.Vector.Traits.Spec.to_le_bytes_post_N
        #(mk_usize 16) ${x}.f_elements head))
"#))]
pub(super) fn to_bytes(x: PortableVector, bytes: &mut [U8]) {
    #[cfg(hax)]
    let _bytes_len = bytes.len();

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"(Core_models.Slice.impl__len #u8 ${bytes} == ${_bytes_len}) /\
            (forall (k: nat). (k < v i /\ 2 * k + 1 < Seq.length ${bytes}) ==>
                (Seq.index ${bytes} (2 * k) ==
                  Libcrux_secrets.Int.f_as_u8 #i16 #FStar.Tactics.Typeclasses.solve
                    (Seq.index ${x}.f_elements k <: i16)) /\
                (Seq.index ${bytes} (2 * k + 1) ==
                  Libcrux_secrets.Int.f_as_u8 #i16 #FStar.Tactics.Typeclasses.solve
                    ((Seq.index ${x}.f_elements k <: i16) >>! mk_i32 8 <: i16)))"#
            )
        });
        bytes[2 * i + 1] = (x.elements[i] >> 8).as_u8();
        bytes[2 * i] = x.elements[i].as_u8();
    }
    hax_lib::fstar!(r#"to_bytes_bit_bridge ${x} ${bytes}"#);
}
