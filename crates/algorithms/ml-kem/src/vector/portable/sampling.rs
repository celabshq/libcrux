use crate::vector::FIELD_MODULUS;

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(a.len() == 24 && result.len() == 16))]
#[cfg_attr(hax, hax_lib::ensures(|res|
        fstar!(r#"Seq.length $result_future == Seq.length $result /\ v $res <= 16"#)
))]
pub(crate) fn rej_sample(a: &[u8], result: &mut [i16]) -> usize {
    let mut sampled = 0;
    for i in 0..a.len() / 3 {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(r#"Seq.length result == 16 /\ v $sampled <= v $i * 2"#)
        });

        let b1 = a[i * 3 + 0] as i16;
        let b2 = a[i * 3 + 1] as i16;
        let b3 = a[i * 3 + 2] as i16;

        let d1 = ((b2 & 0xF) << 8) | b1;
        let d2 = (b3 << 4) | (b2 >> 4);

        if d1 < FIELD_MODULUS && sampled < 16 {
            result[sampled] = d1;
            sampled += 1
        }
        if d2 < FIELD_MODULUS && sampled < 16 {
            result[sampled] = d2;
            sampled += 1
        }
    }
    sampled
}
