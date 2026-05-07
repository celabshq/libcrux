use crate::vector::traits::FIELD_ELEMENTS_IN_VECTOR;
use libcrux_secrets::*;

/// Values having this type hold a representative 'x' of the ML-KEM field.
/// We use 'fe' as a shorthand for this type.
pub(crate) type FieldElement = I16;

#[derive(Clone, Copy)]
pub struct PortableVector {
    pub(crate) elements: [FieldElement; FIELD_ELEMENTS_IN_VECTOR],
}

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
#[hax_lib::fstar::verification_status(panic_free)]
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
        elements[i] = (array[2 * i + 1].as_i16()) << 8 | array[2 * i].as_i16();
    }
    PortableVector { elements }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
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
        hax_lib::loop_invariant!(|_i: usize| bytes.len() == _bytes_len);
        bytes[2 * i + 1] = (x.elements[i] >> 8).as_u8();
        bytes[2 * i] = x.elements[i].as_u8();
    }
}
