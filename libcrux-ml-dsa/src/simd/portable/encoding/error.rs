use crate::{constants::Eta, helper::cloop, simd::portable::vector_type::Coefficients};

#[inline(always)]
#[hax_lib::requires(fstar!(r#"(forall i. bounded (Seq.index ${simd_unit.values} i) 2) /\ Seq.length $serialized == 3"#))]
#[hax_lib::ensures(|_| fstar!(r#"Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
fn serialize_when_eta_is_2(simd_unit: &Coefficients, serialized: &mut [u8]) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 3);

    const ETA: i32 = 2;

    let coefficient0 = (ETA - simd_unit.values[0]) as u8;
    let coefficient1 = (ETA - simd_unit.values[1]) as u8;
    let coefficient2 = (ETA - simd_unit.values[2]) as u8;
    let coefficient3 = (ETA - simd_unit.values[3]) as u8;
    let coefficient4 = (ETA - simd_unit.values[4]) as u8;
    let coefficient5 = (ETA - simd_unit.values[5]) as u8;
    let coefficient6 = (ETA - simd_unit.values[6]) as u8;
    let coefficient7 = (ETA - simd_unit.values[7]) as u8;

    serialized[0] = (coefficient2 << 6) | (coefficient1 << 3) | coefficient0;
    serialized[1] =
        (coefficient5 << 7) | (coefficient4 << 4) | (coefficient3 << 1) | (coefficient2 >> 2);
    serialized[2] = (coefficient7 << 5) | (coefficient6 << 2) | (coefficient5 >> 1);
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"(forall i. bounded (Seq.index ${simd_unit.values} i) 4) /\ Seq.length $serialized == 4"#))]
#[hax_lib::ensures(|_| fstar!(r#"Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
fn serialize_when_eta_is_4(simd_unit: &Coefficients, serialized: &mut [u8]) {
    const ETA: i32 = 4;

    cloop! {
        for (i, coefficients) in simd_unit.values.chunks_exact(2).enumerate() {
            hax_lib::loop_invariant!(|_i: usize| serialized.len() == 4);
            let coefficient0 = (ETA - coefficients[0]) as u8;
            let coefficient1 = (ETA - coefficients[1]) as u8;

            serialized[i] = (coefficient1 << 4) | coefficient0;
        }
    }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length serialized == (match eta with | Libcrux_ml_dsa.Constants.Eta_Two -> 3 | Libcrux_ml_dsa.Constants.Eta_Four -> 4)
 /\ (forall i. bounded (Seq.index simd_unit.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) (match eta with | Libcrux_ml_dsa.Constants.Eta_Two -> 2 | Libcrux_ml_dsa.Constants.Eta_Four -> 4))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"Seq.length ${serialized}_future == Seq.length ${serialized}"#))]
pub(crate) fn serialize(eta: Eta, simd_unit: &Coefficients, serialized: &mut [u8]) {
    // [eurydice] injects an unused variable here in the C code for some reason.
    match eta {
        Eta::Two => serialize_when_eta_is_2(simd_unit, serialized),
        Eta::Four => serialize_when_eta_is_4(simd_unit, serialized),
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(serialized.len() == 3)]
#[hax_lib::ensures(|_| fstar!(r#"
    (forall (i: nat). i < 8 ==>
      v (Seq.index ${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) >= -5 /\
      v (Seq.index ${simd_unit}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) <= 2)"#))]
fn deserialize_when_eta_is_2(serialized: &[u8], simd_unit: &mut Coefficients) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 3);

    const ETA: i32 = 2;

    let byte0 = serialized[0] as i32;
    let byte1 = serialized[1] as i32;
    let byte2 = serialized[2] as i32;

    hax_lib::fstar!("let (): squash (forall (x: int_t I32). get_bit x (mk_int 31) == 0 ==> v x >= 0) = reveal_opaque (`%get_bit) (get_bit #I32) in ()");
    hax_lib::fstar!(
        r#"assert (mk_i32 7 == (mk_i32 (pow2 3) -! mk_i32 1)) by (FStar.Tactics.norm [primops])"#
    );

    simd_unit.values[0] = ETA - (byte0 & 7);
    simd_unit.values[1] = ETA - ((byte0 >> 3) & 7);
    simd_unit.values[2] = ETA - (((byte0 >> 6) | (byte1 << 2)) & 7);
    simd_unit.values[3] = ETA - ((byte1 >> 1) & 7);
    simd_unit.values[4] = ETA - ((byte1 >> 4) & 7);
    simd_unit.values[5] = ETA - (((byte1 >> 7) | (byte2 << 1)) & 7);
    simd_unit.values[6] = ETA - ((byte2 >> 2) & 7);
    simd_unit.values[7] = ETA - ((byte2 >> 5) & 7);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(serialized.len() == 4)]
#[hax_lib::ensures(|_| fstar!(r#"
    (forall (i: nat). i < 8 ==>
      v (Seq.index ${simd_units}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) >= -11 /\
      v (Seq.index ${simd_units}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) <= 4)"#))]
fn deserialize_when_eta_is_4(serialized: &[u8], simd_units: &mut Coefficients) {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == 4);

    const ETA: i32 = 4;

    hax_lib::fstar!(
        r#"assert (mk_u8 15 == (mk_u8 (pow2 4) -! mk_u8 1)) by (FStar.Tactics.norm [primops])"#
    );

    cloop! {
        for (i, byte) in serialized.iter().enumerate() {
            hax_lib::loop_invariant!(|i: usize| fstar!(r#"
                Seq.length $serialized == 4 /\ v $i <= 4 /\
                (forall (j: nat). j < 2 * v $i ==>
                  v (Seq.index ${simd_units}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) >= -11 /\
                  v (Seq.index ${simd_units}.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values j) <= 4)"#));

            hax_lib::fstar!("let (): squash (forall (x: int_t I32). get_bit x (mk_int 31) == 0 ==> v x >= 0) = reveal_opaque (`%get_bit) (get_bit #I32) in ()");

            simd_units.values[2 * i] = ETA - ((byte & 0xF) as i32);
            simd_units.values[2 * i + 1] = ETA - ((byte >> 4) as i32);
        }
    }
}
#[inline(always)]
#[hax_lib::requires(serialized.len() == (match eta { Eta::Two => 3, Eta::Four => 4 }))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
      ($eta == Libcrux_ml_dsa.Constants.Eta_Two ==>
          v (Seq.index ${out}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) >= -5 /\
          v (Seq.index ${out}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) <= 2) /\
      ($eta == Libcrux_ml_dsa.Constants.Eta_Four ==>
          v (Seq.index ${out}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) >= -11 /\
          v (Seq.index ${out}_future.Libcrux_ml_dsa.Simd.Portable.Vector_type.f_values i) <= 4))"#))]
pub(crate) fn deserialize(eta: Eta, serialized: &[u8], out: &mut Coefficients) {
    // [eurydice] injects an unused variable here in the C code for some reason.
    //            That's why we don't match here.
    match eta {
        Eta::Two => deserialize_when_eta_is_2(serialized, out),
        Eta::Four => deserialize_when_eta_is_4(serialized, out),
    }
}
