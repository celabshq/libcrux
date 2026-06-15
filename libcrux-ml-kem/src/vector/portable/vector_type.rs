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
pub(super) fn from_bytes(array: &[U8]) -> PortableVector {
    let mut elements = [I16(0); FIELD_ELEMENTS_IN_VECTOR];
    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        elements[i] = (array[2 * i + 1].as_i16()) << 8 | array[2 * i].as_i16();
    }
    PortableVector { elements }
}

#[inline(always)]
#[hax_lib::requires(bytes.len() >= 32)]
#[hax_lib::ensures(|_| future(bytes).len() == bytes.len())]
pub(super) fn to_bytes(x: PortableVector, bytes: &mut [U8]) {
    #[cfg(hax)]
    let _bytes_len = bytes.len();

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|_i: usize| bytes.len() == _bytes_len);
        bytes[2 * i + 1] = (x.elements[i] >> 8).as_u8();
        bytes[2 * i] = x.elements[i].as_u8();
    }
}
