use crate::simd::portable::vector_type::Coefficients;

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r"Seq.length ${serialized} == 4 /\ (forall i. bounded (Seq.index ${simd_unit.values} i) 4)")))]
#[cfg_attr(hax, hax_lib::ensures(|out| {
    let serialized_future = future(serialized);
    fstar!(r"
Seq.length ${serialized_future} == Seq.length ${serialized} /\
(let inp = bit_vec_of_int_t_array #I32 #(mk_usize 8) ${simd_unit.values} 4 in
 let out = bit_vec_of_int_t_array #U8  #(mk_usize 4) ${serialized_future} 8 in
 forall (i: nat {i < 8 * 4}). inp i == out i)
")
}))]
fn serialize_4(simd_unit: &Coefficients, serialized: &mut [u8]) {
    let coefficient0 = simd_unit.values[0] as u8;
    let coefficient1 = simd_unit.values[1] as u8;
    let coefficient2 = simd_unit.values[2] as u8;
    let coefficient3 = simd_unit.values[3] as u8;
    let coefficient4 = simd_unit.values[4] as u8;
    let coefficient5 = simd_unit.values[5] as u8;
    let coefficient6 = simd_unit.values[6] as u8;
    let coefficient7 = simd_unit.values[7] as u8;

    let byte0 = (coefficient1 << 4) | coefficient0;
    let byte1 = (coefficient3 << 4) | coefficient2;
    let byte2 = (coefficient5 << 4) | coefficient4;
    let byte3 = (coefficient7 << 4) | coefficient6;

    serialized[0] = byte0;
    serialized[1] = byte1;
    serialized[2] = byte2;
    serialized[3] = byte3;
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 300"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r"Seq.length ${serialized} == 6 /\ (forall i. bounded (Seq.index ${simd_unit.values} i) 6)")))]
#[cfg_attr(hax, hax_lib::ensures(|out| {
    let serialized_future = future(serialized);
    fstar!(r"
Seq.length ${serialized_future} == Seq.length ${serialized} /\
(let inp = bit_vec_of_int_t_array #I32 #(mk_usize 8) ${simd_unit.values} 6 in
 let out = bit_vec_of_int_t_array #U8  #(mk_usize 6) ${serialized_future} 8 in
 forall (i: nat {i < 8 * 6}). inp i == out i)
")
}))]
pub fn serialize_6(simd_unit: &Coefficients, serialized: &mut [u8]) {
    // The commitment has coefficients in [0,43] => each coefficient occupies
    // 6 bits.

    let coefficient0 = simd_unit.values[0] as u8;
    let coefficient1 = simd_unit.values[1] as u8;
    let coefficient2 = simd_unit.values[2] as u8;
    let coefficient3 = simd_unit.values[3] as u8;
    let coefficient4 = simd_unit.values[4] as u8;
    let coefficient5 = simd_unit.values[5] as u8;
    let coefficient6 = simd_unit.values[6] as u8;
    let coefficient7 = simd_unit.values[7] as u8;

    let byte0 = (coefficient1 << 6) | coefficient0;
    let byte1 = (coefficient2 << 4) | coefficient1 >> 2;
    let byte2 = (coefficient3 << 2) | coefficient2 >> 4;
    let byte3 = (coefficient5 << 6) | coefficient4;
    let byte4 = (coefficient6 << 4) | coefficient5 >> 2;
    let byte5 = (coefficient7 << 2) | coefficient6 >> 4;

    serialized[0] = byte0;
    serialized[1] = byte1;
    serialized[2] = byte2;
    serialized[3] = byte3;
    serialized[4] = byte4;
    serialized[5] = byte5;
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(
    fstar!(r"
        let d = Seq.length $serialized in
           (d == 4 \/ d == 6)
        /\ (forall i. bounded (Seq.index ${simd_unit.values} i) d)
    ")
))]
#[cfg_attr(hax, hax_lib::ensures(|out| {
    let serialized_future = future(serialized);
    fstar!(r"
let d = Seq.length ${serialized} in
( Seq.length ${serialized_future} == d /\
  (let inp = bit_vec_of_int_t_array #I32 #(mk_usize 8) ${simd_unit.values} d in
   let out = bit_vec_of_int_t_array #U8  #(mk_usize d) ${serialized_future} 8 in
   forall (i: nat {i < 8 * d}). inp i == out i))
")
}))]
pub(in crate::simd::portable) fn serialize(simd_unit: &Coefficients, serialized: &mut [u8]) {
    match serialized.len() as u8 {
        4 => serialize_4(simd_unit, serialized),
        6 => serialize_6(simd_unit, serialized),
        _ => unreachable!(),
    }
}
