use crate::vector::FIELD_MODULUS;

#[cfg(hax)]
use hax_lib::prop::ToProp;

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 50")]
#[hax_lib::requires(a.len() == 24 && result.len() == 16)]
#[hax_lib::ensures(|res| (future(result).len() == result.len() && res <= 16).to_prop().and(
    hax_lib::forall(|j: usize|
        hax_lib::implies(j < res,
            future(result)[j] >= 0 && future(result)[j] <= 3328))))]
pub(crate) fn rej_sample(a: &[u8], result: &mut [i16]) -> usize {
    let mut sampled = 0;
    for i in 0..a.len() / 3 {
        hax_lib::loop_invariant!(|i: usize| (result.len() == 16 && sampled <= i * 2)
            .to_prop()
            .and(hax_lib::forall(|j: usize| hax_lib::implies(
                j < sampled,
                result[j] >= 0 && result[j] <= 3328
            ))));

        let b1 = a[i * 3 + 0] as i16;
        let b2 = a[i * 3 + 1] as i16;
        let b3 = a[i * 3 + 2] as i16;

        let d1 = ((b2 & 0xF) << 8) | b1;
        let d2 = (b3 << 4) | (b2 >> 4);

        hax_lib::fstar!(r#"logand_lemma $b2 (mk_i16 15)"#);
        hax_lib::fstar!(r#"logor_lemma (($b2 &. mk_i16 15) <<! mk_i32 8) $b1"#);
        hax_lib::fstar!(r#"logor_lemma ($b3 <<! mk_i32 4) ($b2 >>! mk_i32 4)"#);

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
