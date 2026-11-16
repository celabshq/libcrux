// C extraction:
// A couple helper functions and definitions -- this file ends up being bundled in
// libcrux_core.{c,h}, so if you need something that has to be shared across multiple mlkem
// instances / implementations, it can go in here.

type PaddedArray<const N: usize> = [u8; N];

/// Pad the `slice` with `0`s at the end.
#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(
    slice.len() <= LEN
))]
#[cfg_attr(hax, hax_lib::ensures(|result|
    fstar!(r#"$result == Seq.append $slice (Seq.create (v $LEN - v (${slice.len()})) (mk_u8 0))"#)))]
pub(crate) fn into_padded_array<const LEN: usize>(slice: &[u8]) -> PaddedArray<LEN> {
    let mut out = [0u8; LEN];
    out[0..slice.len()].copy_from_slice(slice);
    #[cfg(hax)]
    hax_lib::fstar!(r#"assert (Seq.slice out 0 (Seq.length slice) == slice)"#);
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (Seq.slice out (Seq.length slice) (v v_LEN) == Seq.slice (Seq.create (v v_LEN) (mk_u8 0)) (Seq.length slice) (v v_LEN))"#
    );
    #[cfg(hax)]
    hax_lib::fstar!(
        "assert (forall i. i < Seq.length slice ==> Seq.index out i == Seq.index slice i)"
    );
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (forall i. (i >= Seq.length slice && i < v v_LEN) ==> Seq.index out i == Seq.index (Seq.slice out (Seq.length slice) (v v_LEN)) (i - Seq.length slice))"#
    );
    #[cfg(hax)]
    hax_lib::fstar!(
        "Seq.lemma_eq_intro out (Seq.append slice (Seq.create (v v_LEN - Seq.length slice) (mk_u8 0)))"
    );
    out
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 200"))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"range (v $domain_separator + v $K) u8_inttype"#)))]
#[cfg_attr(hax, hax_lib::ensures(|ds|
    fstar!(r#"v $ds == v $domain_separator + v $K /\
            (forall (i:nat). i < v $K ==>
                v (Seq.index (Seq.index ${prf_inputs}_future i) 32) == v $domain_separator + i /\
                Seq.slice (Seq.index ${prf_inputs}_future i) 0 32 == Seq.slice (Seq.index $prf_inputs i) 0 32)"#)
))]
pub(crate) fn prf_input_inc<const K: usize>(
    prf_inputs: &mut [[u8; 33]; K],
    mut domain_separator: u8,
) -> u8 {
    #[cfg(hax)]
    let _domain_separator_init = domain_separator;
    #[cfg(hax)]
    let _prf_inputs_init = prf_inputs.clone();

    for i in 0..K {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"v $domain_separator == v $_domain_separator_init + v $i /\
          (v $i < v $K ==> (forall (j:nat). (j >= v $i /\ j < v $K) ==>
            prf_inputs.[ sz j ] == ${_prf_inputs_init}.[ sz j ])) /\
          (forall (j:nat). j < v $i ==> v (Seq.index (Seq.index prf_inputs j) 32) == v $_domain_separator_init + j /\
            Seq.slice (Seq.index prf_inputs j) 0 32 == Seq.slice (Seq.index $_prf_inputs_init j) 0 32)"#
            )
        });
        prf_inputs[i][32] = domain_separator;
        domain_separator += 1;
    }
    domain_separator
}

// C extraction:
//
// This is only enabled when extracting.
//
// Without these type abbreviations, the monomorphized definitions end up being inserted at the
// first location that they are used, which might be, e.g., the avx2 impl of mlkem512, resulting in
// the portable impl of mlkem512 including the header for the avx2 impl of mlkem512 to have this
// type definition in scope!
//
// To avoid that, we manually place those definitions in this file, which ends up in a shared
// header.
//
// TODO: use proper constants. They don't work right now ...
#[cfg(eurydice)]
mod extraction_helper {
    type Keypair512 = ([u8; 768], [u8; 800]);
    type Keypair768 = ([u8; 1152], [u8; 1184]);
    type Keypair1024 = ([u8; 1536], [u8; 1568]);
}
