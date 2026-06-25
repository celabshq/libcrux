use crate::constants::FIELD_MODULUS;

#[cfg(hax)]
use crate::specs::simd::portable::sample::*;

#[inline(always)]
#[hax_lib::requires(rejection_sample_less_than_field_modulus_pre(randomness, out))]
#[hax_lib::ensures(|r| fstar!(r#"
    Libcrux_ml_dsa.Specs.Simd.Portable.Sample.rejection_sample_less_than_field_modulus_post $randomness ${out}_future $r /\
    Seq.length ${out}_future == Seq.length $out /\
    v $r <= Seq.length $randomness / 3 /\
    (forall (i:nat{i < Seq.length ${out}_future}). i < v $r ==>
       v (Seq.index ${out}_future i) >= 0 /\ v (Seq.index ${out}_future i) < 8380417)"#))]
pub fn rejection_sample_less_than_field_modulus(randomness: &[u8], out: &mut [i32]) -> usize {
    let mut sampled = 0;

    #[cfg(hax)]
    let _out_len = out.len();
    hax_lib::fstar!(
        r#"Spec.Utils.eq_repeati0 (sz 0) (Spec.MLDSA.Math.rejection_sample_field_modulus_inner $randomness) Seq.empty"#
    );

    for i in 0..randomness.len() / 3 {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              v $sampled <= v $i /\
              Seq.length $out == v $_out_len /\
              (forall (j:nat{j < Seq.length ${out}}). j < v $sampled ==>
                 v (Seq.index $out j) >= 0 /\ v (Seq.index $out j) < 8380417) /\
              (let samples = Spec.Utils.repeati ($i)
                (Spec.MLDSA.Math.rejection_sample_field_modulus_inner $randomness) Seq.empty in
              v $sampled == Seq.length samples /\
              Seq.slice $out 0 (Seq.length samples) == samples)"#
            )
        });

        let b0 = randomness[i * 3] as i32;
        let b1 = randomness[i * 3 + 1] as i32;
        let b2 = randomness[i * 3 + 2] as i32;
        let coefficient = ((b2 << 16) | (b1 << 8) | b0) & 0x00_7F_FF_FF;

        hax_lib::fstar!(
            r#"Spec.MLDSA.Math.rejection_sample_coefficient_lemma $randomness ($i);
            Spec.Utils.unfold_repeati ($i +! sz 1) 
                (Spec.MLDSA.Math.rejection_sample_field_modulus_inner $randomness) Seq.empty ($i)"#
        );

        if coefficient < FIELD_MODULUS {
            out[sampled] = coefficient;
            sampled += 1;
        }

        hax_lib::fstar!(
            r#"let samples = Spec.Utils.repeati ($i +! sz 1)
                (Spec.MLDSA.Math.rejection_sample_field_modulus_inner $randomness) Seq.empty in
            eq_intro (Seq.slice $out 0 (Seq.length samples)) samples"#
        );
    }

    sampled
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --z3refresh")]
#[hax_lib::requires(rejection_sample_less_than_eta_equals_2_pre(randomness, out))]
#[hax_lib::ensures(|r| fstar!(r#"
    Libcrux_ml_dsa.Specs.Simd.Portable.Sample.rejection_sample_less_than_eta_equals_2_post $randomness ${out}_future $r /\
    Seq.length ${out}_future == Seq.length $out /\
    v $r <= Seq.length $randomness * 2 /\
    (forall (i:nat{i < Seq.length ${out}_future}). i < v $r ==>
       v (Seq.index ${out}_future i) >= -2 /\ v (Seq.index ${out}_future i) <= 2)"#))]
pub fn rejection_sample_less_than_eta_equals_2(randomness: &[u8], out: &mut [i32]) -> usize {
    let mut sampled = 0;

    #[cfg(hax)]
    let _out_len = out.len();
    hax_lib::fstar!(
        r#"Spec.Utils.eq_repeati0 (sz 0) (Spec.MLDSA.Math.rejection_sample_eta_2_inner $randomness) Seq.empty"#
    );

    for i in 0..randomness.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              v $i >= 0 /\ v $i <= Seq.length $randomness /\
              v $sampled <= v $i * 2 /\
              Seq.length $out == v $_out_len /\
              (forall (j:nat{j < Seq.length ${out}}). j < v $sampled ==>
                 v (Seq.index $out j) >= -2 /\ v (Seq.index $out j) <= 2) /\
              (let samples = Spec.Utils.repeati ($i)
                (Spec.MLDSA.Math.rejection_sample_eta_2_inner $randomness) Seq.empty in
              v $sampled == Seq.length samples /\
              Seq.slice $out 0 (Seq.length samples) == samples)"#
            )
        });

        let byte = randomness[i];
        let try_0 = byte & 0xF;
        let try_1 = byte >> 4;

        hax_lib::fstar!(
            r#"Spec.Utils.unfold_repeati ($i +! sz 1) 
                (Spec.MLDSA.Math.rejection_sample_eta_2_inner $randomness) Seq.empty ($i)"#
        );

        if try_0 < 15 {
            let try_0 = try_0 as i32;
            // (try_0 * 26) >> 7 computes ⌊try_0 / 5⌋
            let try_0_mod_5 = try_0 - ((try_0 * 26) >> 7) * 5;

            hax_lib::fstar!(
                r#"assert ($try_0_mod_5 == ($try_0 %! mk_i32 5));
                assert ((mk_i32 2 -. $try_0_mod_5) == (mk_i32 2 -! $try_0_mod_5))"#
            );

            out[sampled] = 2 - try_0_mod_5;
            sampled += 1;
        }

        if try_1 < 15 {
            let try_1 = try_1 as i32;
            let try_1_mod_5 = try_1 - ((try_1 * 26) >> 7) * 5;

            hax_lib::fstar!(
                r#"assert ($try_1_mod_5 == ($try_1 %! mk_i32 5));
                assert ((mk_i32 2 -. $try_1_mod_5) == (mk_i32 2 -! $try_1_mod_5))"#
            );

            out[sampled] = 2 - try_1_mod_5;
            sampled += 1;
        }

        hax_lib::fstar!(
            r#"let samples = Spec.Utils.repeati ($i +! sz 1)
                (Spec.MLDSA.Math.rejection_sample_eta_2_inner $randomness) Seq.empty in
            eq_intro (Seq.slice $out 0 (Seq.length samples)) samples"#
        );
    }

    sampled
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3refresh")]
#[hax_lib::requires(rejection_sample_less_than_eta_equals_4_pre(randomness, out))]
#[hax_lib::ensures(|r| fstar!(r#"
    Libcrux_ml_dsa.Specs.Simd.Portable.Sample.rejection_sample_less_than_eta_equals_4_post $randomness ${out}_future $r /\
    Seq.length ${out}_future == Seq.length $out /\
    v $r <= Seq.length $randomness * 2 /\
    (forall (i:nat{i < Seq.length ${out}_future}). i < v $r ==>
       v (Seq.index ${out}_future i) >= -4 /\ v (Seq.index ${out}_future i) <= 4)"#))]
pub fn rejection_sample_less_than_eta_equals_4(randomness: &[u8], out: &mut [i32]) -> usize {
    let mut sampled = 0;

    #[cfg(hax)]
    let _out_len = out.len();
    hax_lib::fstar!(
        r#"Spec.Utils.eq_repeati0 (sz 0) (Spec.MLDSA.Math.rejection_sample_eta_4_inner $randomness) Seq.empty"#
    );

    for i in 0..randomness.len() {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"
              v $i >= 0 /\ v $i <= Seq.length $randomness /\
              v $sampled <= v $i * 2 /\
              Seq.length $out == v $_out_len /\
              (forall (j:nat{j < Seq.length ${out}}). j < v $sampled ==>
                 v (Seq.index $out j) >= -4 /\ v (Seq.index $out j) <= 4) /\
              (let samples = Spec.Utils.repeati ($i)
                (Spec.MLDSA.Math.rejection_sample_eta_4_inner $randomness) Seq.empty in
              v $sampled == Seq.length samples /\
              Seq.slice $out 0 (Seq.length samples) == samples)"#
            )
        });

        let byte = randomness[i];
        let try_0 = byte & 0xF;
        let try_1 = byte >> 4;

        hax_lib::fstar!(
            r#"Spec.Utils.unfold_repeati ($i +! sz 1) 
                (Spec.MLDSA.Math.rejection_sample_eta_4_inner $randomness) Seq.empty ($i)"#
        );

        if try_0 < 9 {
            out[sampled] = 4 - (try_0 as i32);
            sampled += 1;
        }
        if try_1 < 9 {
            out[sampled] = 4 - (try_1 as i32);
            sampled += 1;
        }

        hax_lib::fstar!(
            r#"let samples = Spec.Utils.repeati ($i +! sz 1)
                (Spec.MLDSA.Math.rejection_sample_eta_4_inner $randomness) Seq.empty in
            eq_intro (Seq.slice $out 0 (Seq.length samples)) samples"#
        );
    }

    sampled
}
