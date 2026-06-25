use super::traits::Operations;
pub(crate) use libcrux_intrinsics::avx2::*;

mod arithmetic;
mod compress;
mod ntt;
mod sampling;
mod serialize;

#[derive(Clone, Copy)]
#[hax_lib::fstar::before(interface, "noeq")]
#[hax_lib::fstar::after(interface,"let repr (x:t_SIMD256Vector) : t_Array i16 (sz 16) = Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 x.f_elements")]
pub struct SIMD256Vector {
    pub(crate) elements: Vec256,
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::ensures(|result| fstar!(r#"repr ${result} == Seq.create 16 (mk_i16 0)"#))]
fn vec_zero() -> SIMD256Vector {
    SIMD256Vector {
        elements: mm256_setzero_si256(),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::ensures(|result| fstar!(r#"${result} == repr ${v}"#))]
fn vec_to_i16_array(v: SIMD256Vector) -> [i16; 16] {
    let mut output = [0i16; 16];
    mm256_storeu_si256_i16(&mut output, v.elements);

    output
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::ensures(|result| fstar!(r#"repr ${result} == ${array}"#))]
fn vec_from_i16_array(array: &[i16]) -> SIMD256Vector {
    SIMD256Vector {
        elements: mm256_loadu_si256_i16(array),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array (pow2 12 - 1) (repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"repr out == Spec.Utils.map_array (fun x -> if x >=. (mk_i16 3329) then x -! (mk_i16 3329) else x) (repr $vector)"#))]
fn cond_subtract_3329(vector: SIMD256Vector) -> SIMD256Vector {
    SIMD256Vector {
        elements: arithmetic::cond_subtract_3329(vector.elements),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"forall (i:nat). i < 16 ==> v (Seq.index (repr $vector) i) >= 0 /\
    v (Seq.index (repr $vector) i) < 3329"#))]
#[hax_lib::ensures(|out| fstar!(r#"forall (i:nat). i < 16 ==> bounded (Seq.index (repr $out) i) 1"#))]
fn compress_1(vector: SIMD256Vector) -> SIMD256Vector {
    SIMD256Vector {
        elements: compress::compress_message_coefficient(vector.elements),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
    v $COEFFICIENT_BITS == 5 \/
    v $COEFFICIENT_BITS == 10 \/
    v $COEFFICIENT_BITS == 11) /\
    (forall (i:nat). i < 16 ==> v (Seq.index (repr $vector) i) >= 0 /\
    v (Seq.index (repr $vector) i) < 3329)"#))]
#[hax_lib::ensures(|out| fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
    v $COEFFICIENT_BITS == 5 \/
    v $COEFFICIENT_BITS == 10 \/
    v $COEFFICIENT_BITS == 11) ==>
        (forall (i:nat). i < 16 ==> bounded (Seq.index (repr $out) i) (v $COEFFICIENT_BITS))"#))]
fn compress<const COEFFICIENT_BITS: i32>(vector: SIMD256Vector) -> SIMD256Vector {
    SIMD256Vector {
        elements: compress::compress_ciphertext_coefficient::<COEFFICIENT_BITS>(vector.elements),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ 
                    Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                    Spec.Utils.is_i16b_array (11207+5*3328) (repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array (11207+6*3328) (repr $out)"#))]
fn ntt_layer_1_step(
    vector: SIMD256Vector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::ntt_layer_1_step(vector.elements, zeta0, zeta1, zeta2, zeta3),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                    Spec.Utils.is_i16b_array (11207+4*3328) (repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array (11207+5*3328) (repr $out)"#))]
fn ntt_layer_2_step(vector: SIMD256Vector, zeta0: i16, zeta1: i16) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::ntt_layer_2_step(vector.elements, zeta0, zeta1),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
                    Spec.Utils.is_i16b_array (11207+3*3328) (repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array (11207+4*3328) (repr $out)"#))]
fn ntt_layer_3_step(vector: SIMD256Vector, zeta: i16) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::ntt_layer_3_step(vector.elements, zeta),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ 
                    Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3  /\
                    Spec.Utils.is_i16b_array (4*3328) (repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (repr $out)"#))]
fn inv_ntt_layer_1_step(
    vector: SIMD256Vector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::inv_ntt_layer_1_step(vector.elements, zeta0, zeta1, zeta2, zeta3),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                    Spec.Utils.is_i16b_array 3328 (repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (repr $out)"#))]
fn inv_ntt_layer_2_step(vector: SIMD256Vector, zeta0: i16, zeta1: i16) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::inv_ntt_layer_2_step(vector.elements, zeta0, zeta1),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
                    Spec.Utils.is_i16b_array 3328 (repr ${vector})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (repr $out)"#))]
fn inv_ntt_layer_3_step(vector: SIMD256Vector, zeta: i16) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::inv_ntt_layer_3_step(vector.elements, zeta),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                    Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                    Spec.Utils.is_i16b_array 3328 (repr ${lhs}) /\
                    Spec.Utils.is_i16b_array 3328 (repr ${rhs})"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (repr $out)"#))]
fn ntt_multiply(
    lhs: &SIMD256Vector,
    rhs: &SIMD256Vector,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> SIMD256Vector {
    SIMD256Vector {
        elements: ntt::ntt_multiply(lhs.elements, rhs.elements, zeta0, zeta1, zeta2, zeta3),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.serialize_pre 1 (repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 1 (repr $vector) ==> Spec.MLKEM.serialize_post 1 (repr $vector) $out"#))]
fn serialize_1(vector: SIMD256Vector) -> [u8; 2] {
    serialize::serialize_1(vector.elements)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(bytes.len() == 2)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 2 ==> Spec.MLKEM.deserialize_post 1 $bytes (repr $out)"#))]
fn deserialize_1(bytes: &[u8]) -> SIMD256Vector {
    SIMD256Vector {
        elements: serialize::deserialize_1(bytes),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.serialize_pre 4 (repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 4 (repr $vector) ==> Spec.MLKEM.serialize_post 4 (repr $vector) $out"#))]
fn serialize_4(vector: SIMD256Vector) -> [u8; 8] {
    serialize::serialize_4(vector.elements)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(bytes.len() == 8)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 8 ==> Spec.MLKEM.deserialize_post 4 $bytes (repr $out)"#))]
fn deserialize_4(bytes: &[u8]) -> SIMD256Vector {
    SIMD256Vector {
        elements: serialize::deserialize_4(bytes),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.serialize_pre 10 (repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 10 (repr $vector) ==> Spec.MLKEM.serialize_post 10 (repr $vector) $out"#))]
fn serialize_10(vector: SIMD256Vector) -> [u8; 20] {
    serialize::serialize_10(vector.elements)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(bytes.len() == 20)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 20 ==> Spec.MLKEM.deserialize_post 10 $bytes (repr $out)"#))]
fn deserialize_10(bytes: &[u8]) -> SIMD256Vector {
    SIMD256Vector {
        elements: serialize::deserialize_10(bytes),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.serialize_pre 12 (repr $vector)"#))]
#[hax_lib::ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 12 (repr $vector) ==> Spec.MLKEM.serialize_post 12 (repr $vector) $out"#))]
fn serialize_12(vector: SIMD256Vector) -> [u8; 24] {
    serialize::serialize_12(vector.elements)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(bytes.len() == 24)]
#[hax_lib::ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 24 ==> Spec.MLKEM.deserialize_post 12 $bytes (repr $out)"#))]
fn deserialize_12(bytes: &[u8]) -> SIMD256Vector {
    SIMD256Vector {
        elements: serialize::deserialize_12(bytes),
    }
}

#[cfg(hax)]
impl crate::vector::traits::Repr for SIMD256Vector {
    fn repr(&self) -> [i16; 16] {
        vec_to_i16_array(self.clone())
    }
}

#[cfg(any(eurydice, not(hax)))]
impl crate::vector::traits::Repr for SIMD256Vector {}

#[inline(always)]
#[hax_lib::requires(array.len() >= 32)]
pub(super) fn from_bytes(array: &[u8]) -> SIMD256Vector {
    SIMD256Vector {
        elements: mm256_loadu_si256_u8(&array[0..32]),
    }
}

#[inline(always)]
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::requires(bytes.len() >= 32)]
#[hax_lib::ensures(|_| future(bytes).len() == bytes.len())]
pub(super) fn to_bytes(x: SIMD256Vector, bytes: &mut [u8]) {
    mm256_storeu_si256_u8(&mut bytes[0..32], x.elements)
}

#[hax_lib::attributes]
impl Operations for SIMD256Vector {
    #[inline(always)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == Seq.create 16 (mk_i16 0)"#))]
    fn ZERO() -> Self {
        vec_zero()
    }

    #[requires(array.len() == 16)]
    #[ensures(|out| fstar!(r#"impl.f_repr out == $array"#))]
    #[inline(always)]
    fn from_i16_array(array: &[i16]) -> Self {
        vec_from_i16_array(array)
    }

    #[ensures(|out| fstar!(r#"out == impl.f_repr $x"#))]
    #[inline(always)]
    fn to_i16_array(x: Self) -> [i16; 16] {
        vec_to_i16_array(x)
    }

    #[requires(array.len() >= 32)]
    #[inline(always)]
    fn from_bytes(array: &[u8]) -> Self {
        from_bytes(array)
    }

    #[requires(bytes.len() >= 32)]
    #[inline(always)]
    #[ensures(|_| future(bytes).len() == bytes.len())]
    fn to_bytes(x: Self, bytes: &mut [u8]) {
        to_bytes(x, bytes)
    }

    #[requires(fstar!(r#"forall i. i < 16 ==> 
        Spec.Utils.is_intb (pow2 15 - 1) (v (Seq.index (impl.f_repr ${lhs}) i) + v (Seq.index (impl.f_repr ${rhs}) i))"#))]
    #[ensures(|result| fstar!(r#"forall i. i < 16 ==> 
        (v (Seq.index (impl.f_repr ${result}) i) == 
         v (Seq.index (impl.f_repr ${lhs}) i) + v (Seq.index (impl.f_repr ${rhs}) i))"#))]
    #[inline(always)]
    fn add(lhs: Self, rhs: &Self) -> Self {
        Self {
            elements: arithmetic::add(lhs.elements, rhs.elements),
        }
    }

    #[requires(fstar!(r#"forall i. i < 16 ==> 
        Spec.Utils.is_intb (pow2 15 - 1) (v (Seq.index (impl.f_repr ${lhs}) i) - v (Seq.index (impl.f_repr ${rhs}) i))"#))]
    #[ensures(|result| fstar!(r#"forall i. i < 16 ==> 
        (v (Seq.index (impl.f_repr ${result}) i) == 
         v (Seq.index (impl.f_repr ${lhs}) i) - v (Seq.index (impl.f_repr ${rhs}) i))"#))]
    #[inline(always)]
    fn sub(lhs: Self, rhs: &Self) -> Self {
        Self {
            elements: arithmetic::sub(lhs.elements, rhs.elements),
        }
    }

    #[requires(fstar!(r#"forall i. i < 16 ==> 
        Spec.Utils.is_intb (pow2 15 - 1) (v (Seq.index (impl.f_repr ${vec}) i) * v c)"#))]
    #[ensures(|result| fstar!(r#"forall i. i < 16 ==> 
        (v (Seq.index (impl.f_repr ${result}) i) == 
         v (Seq.index (impl.f_repr ${vec}) i) * v c)"#))]
    #[inline(always)]
    fn multiply_by_constant(vec: Self, c: i16) -> Self {
        Self {
            elements: arithmetic::multiply_by_constant(vec.elements, c),
        }
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b_array (pow2 12 - 1) (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"impl.f_repr out == Spec.Utils.map_array (fun x -> if x >=. (mk_i16 3329) then x -! (mk_i16 3329) else x) (impl.f_repr $vector)"#))]
    #[inline(always)]
    fn cond_subtract_3329(vector: Self) -> Self {
        cond_subtract_3329(vector)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b_array 28296 (impl.f_repr ${vector})"#))]
    #[ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr ${result}) /\
                (forall i. (v (Seq.index (impl.f_repr ${result}) i) % 3329) == 
                           (v (Seq.index (impl.f_repr ${vector})i) % 3329))"#))]
    #[inline(always)]
    fn barrett_reduce(vector: Self) -> Self {
        Self {
            elements: arithmetic::barrett_reduce(vector.elements),
        }
    }

    #[inline(always)]
    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 $constant"#))]
    #[ensures(|result| fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr ${result}) /\
                (forall i. i < 16 ==> ((v (Seq.index (impl.f_repr ${result}) i) % 3329)==
                                       (v (Seq.index (impl.f_repr ${vector}) i) * v ${constant} * 169) % 3329))"#))]
    fn montgomery_multiply_by_constant(vector: Self, constant: i16) -> Self {
        Self {
            elements: arithmetic::montgomery_multiply_by_constant(vector.elements, constant),
        }
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr $a)"#))]
    #[ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
                                (let x = Seq.index (impl.f_repr ${a}) i in
                                 let y = Seq.index (impl.f_repr ${result}) i in
                                 (v y >= 0 /\ v y <= 3328 /\ (v y % 3329 == v x % 3329)))"#))]
    fn to_unsigned_representative(a: Self) -> Self {
        Self {
            elements: arithmetic::to_unsigned_representative(a.elements),
        }
    }

    #[requires(fstar!(r#"forall (i:nat). i < 16 ==> v (Seq.index (impl.f_repr $vector) i) >= 0 /\
        v (Seq.index (impl.f_repr $vector) i) < 3329"#))]
    #[ensures(|out| fstar!(r#"forall (i:nat). i < 16 ==> bounded (Seq.index (impl.f_repr $out) i) 1"#))]
    #[inline(always)]
    fn compress_1(vector: Self) -> Self {
        compress_1(vector)
    }

    #[requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
            v $COEFFICIENT_BITS == 5 \/
            v $COEFFICIENT_BITS == 10 \/
            v $COEFFICIENT_BITS == 11) /\
        (forall (i:nat). i < 16 ==> v (Seq.index (impl.f_repr $vector) i) >= 0 /\
            v (Seq.index (impl.f_repr $vector) i) < 3329)"#))]
    #[ensures(|out| fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
            v $COEFFICIENT_BITS == 5 \/
            v $COEFFICIENT_BITS == 10 \/
            v $COEFFICIENT_BITS == 11) ==>
                (forall (i:nat). i < 16 ==> bounded (Seq.index (impl.f_repr $out) i) (v $COEFFICIENT_BITS))"#))]
    #[inline(always)]
    fn compress<const COEFFICIENT_BITS: i32>(vector: Self) -> Self {
        compress::<COEFFICIENT_BITS>(vector)
    }

    #[requires(fstar!(r#"forall (i:nat). i < 16 ==> 
                                    (let x = Seq.index (impl.f_repr $a) i in 
                                     (x == mk_i16 0 \/ x == mk_i16 1))"#))]
    fn decompress_1(a: Self) -> Self {
        Self {
            elements: compress::decompress_1(a.elements),
        }
    }

    #[requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
        v $COEFFICIENT_BITS == 5 \/
        v $COEFFICIENT_BITS == 10 \/
        v $COEFFICIENT_BITS == 11) /\
    (forall (i:nat). i < 16 ==> v (Seq.index (impl.f_repr $vector) i) >= 0 /\
        v (Seq.index (impl.f_repr $vector) i) < pow2 (v $COEFFICIENT_BITS))"#))]
    #[inline(always)]
    fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(vector: Self) -> Self {
        Self {
            elements: compress::decompress_ciphertext_coefficient::<COEFFICIENT_BITS>(
                vector.elements,
            ),
        }
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ 
                       Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                       Spec.Utils.is_i16b_array (11207+5*3328) (impl.f_repr ${vector})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array (11207+6*3328) (impl.f_repr $out)"#))]
    #[inline(always)]
    fn ntt_layer_1_step(vector: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                       Spec.Utils.is_i16b_array (11207+4*3328) (impl.f_repr ${vector})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array (11207+5*3328) (impl.f_repr $out)"#))]
    #[inline(always)]
    fn ntt_layer_2_step(vector: Self, zeta0: i16, zeta1: i16) -> Self {
        ntt_layer_2_step(vector, zeta0, zeta1)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
                       Spec.Utils.is_i16b_array (11207+3*3328) (impl.f_repr ${vector})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array (11207+4*3328) (impl.f_repr $out)"#))]
    #[inline(always)]
    fn ntt_layer_3_step(vector: Self, zeta: i16) -> Self {
        ntt_layer_3_step(vector, zeta)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ 
                       Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3  /\
                       Spec.Utils.is_i16b_array (4*3328) (impl.f_repr ${vector})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr $out)"#))]
    #[inline(always)]
    fn inv_ntt_layer_1_step(vector: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self {
        inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                       Spec.Utils.is_i16b_array 3328 (impl.f_repr ${vector})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr $out)"#))]
    #[inline(always)]
    fn inv_ntt_layer_2_step(vector: Self, zeta0: i16, zeta1: i16) -> Self {
        inv_ntt_layer_2_step(vector, zeta0, zeta1)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta /\
                       Spec.Utils.is_i16b_array 3328 (impl.f_repr ${vector})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr $out)"#))]
    #[inline(always)]
    fn inv_ntt_layer_3_step(vector: Self, zeta: i16) -> Self {
        inv_ntt_layer_3_step(vector, zeta)
    }

    #[requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                       Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                       Spec.Utils.is_i16b_array 3328 (impl.f_repr ${lhs}) /\
                       Spec.Utils.is_i16b_array 3328 (impl.f_repr ${rhs})"#))]
    #[ensures(|out| fstar!(r#"Spec.Utils.is_i16b_array 3328 (impl.f_repr $out)"#))]
    #[inline(always)]
    fn ntt_multiply(
        lhs: &Self,
        rhs: &Self,
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
    ) -> Self {
        ntt_multiply(lhs, rhs, zeta0, zeta1, zeta2, zeta3)
    }

    #[requires(fstar!(r#"Spec.MLKEM.serialize_pre 1 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 1 (impl.f_repr $vector) ==> Spec.MLKEM.serialize_post 1 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_1(vector: Self) -> [u8; 2] {
        serialize_1(vector)
    }

    #[requires(bytes.len() == 2)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 2 ==> Spec.MLKEM.deserialize_post 1 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_1(bytes: &[u8]) -> Self {
        deserialize_1(bytes)
    }

    #[requires(fstar!(r#"Spec.MLKEM.serialize_pre 4 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 4 (impl.f_repr $vector) ==> Spec.MLKEM.serialize_post 4 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_4(vector: Self) -> [u8; 8] {
        serialize_4(vector)
    }

    #[requires(bytes.len() == 8)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 8 ==> Spec.MLKEM.deserialize_post 4 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_4(bytes: &[u8]) -> Self {
        deserialize_4(bytes)
    }

    #[inline(always)]
    fn serialize_5(vector: Self) -> [u8; 10] {
        serialize::serialize_5(vector.elements)
    }

    #[requires(bytes.len() == 10)]
    #[inline(always)]
    fn deserialize_5(bytes: &[u8]) -> Self {
        hax_lib::fstar!(r#"assert (v (Core_models.Slice.impl__len $bytes) == Seq.length $bytes)"#);
        Self {
            elements: serialize::deserialize_5(bytes),
        }
    }

    #[requires(fstar!(r#"Spec.MLKEM.serialize_pre 10 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 10 (impl.f_repr $vector) ==> Spec.MLKEM.serialize_post 10 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_10(vector: Self) -> [u8; 20] {
        serialize_10(vector)
    }

    #[requires(bytes.len() == 20)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 20 ==> Spec.MLKEM.deserialize_post 10 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_10(bytes: &[u8]) -> Self {
        deserialize_10(bytes)
    }

    #[inline(always)]
    fn serialize_11(vector: Self) -> [u8; 22] {
        serialize::serialize_11(vector.elements)
    }

    #[requires(bytes.len() == 22)]
    #[inline(always)]
    fn deserialize_11(bytes: &[u8]) -> Self {
        Self {
            elements: serialize::deserialize_11(bytes),
        }
    }

    #[requires(fstar!(r#"Spec.MLKEM.serialize_pre 12 (impl.f_repr $vector)"#))]
    #[ensures(|out| fstar!(r#"Spec.MLKEM.serialize_pre 12 (impl.f_repr $vector) ==> Spec.MLKEM.serialize_post 12 (impl.f_repr $vector) $out"#))]
    #[inline(always)]
    fn serialize_12(vector: Self) -> [u8; 24] {
        serialize_12(vector)
    }

    #[requires(bytes.len() == 24)]
    #[ensures(|out| fstar!(r#"sz (Seq.length $bytes) =. sz 24 ==> Spec.MLKEM.deserialize_post 12 $bytes (impl.f_repr $out)"#))]
    #[inline(always)]
    fn deserialize_12(bytes: &[u8]) -> Self {
        deserialize_12(bytes)
    }

    #[requires(input.len() == 24 && output.len() == 16)]
    #[ensures(|result|
        fstar!(r#"Seq.length $output_future == Seq.length $output /\ v $result <= 16"#)
    )]
    #[inline(always)]
    fn rej_sample(input: &[u8], output: &mut [i16]) -> usize {
        sampling::rejection_sample(input, output)
    }
}
