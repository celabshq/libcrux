use super::*;

#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[cfg(hax)]
use hax_lib::int::*;

#[cfg(hax)]
use hax_lib::prop::*;

use libcrux_intrinsics::avx2::Vec256;

/// Absorb phase of `keccak4`: initialise a four-lane Keccak state,
/// absorb all full rate-byte blocks of `data[0..4]` in parallel,
/// then pad and absorb each lane's final partial block with
/// domain-separation byte `DELIM` and the pad10*1 terminator.
///
/// The ensures clause asserts per-lane equality with the scalar spec
/// function `Hacspec_sha3.Sponge.absorb`.  The loop invariant uses
/// `absorb_blocks` per lane, mirroring the Arm64 backend at N=2.
#[inline]
#[cfg_attr(hax, hax_lib::requires(
    valid_rate(RATE) &&
    data[0].len() == data[1].len() &&
    data[0].len() == data[2].len() &&
    data[0].len() == data[3].len()
))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
       EquivImplSpec.Keccakf.Avx2.lc_avx2 $result.st 0) ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 0)) /\
    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
       EquivImplSpec.Keccakf.Avx2.lc_avx2 $result.st 1) ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 1)) /\
    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
       EquivImplSpec.Keccakf.Avx2.lc_avx2 $result.st 2) ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 2)) /\
    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
       EquivImplSpec.Keccakf.Avx2.lc_avx2 $result.st 3) ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 3))
"#)))]
#[cfg_attr(
    hax,
    hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always")
)]
pub(crate) fn absorb4<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 4],
) -> KeccakState<4, Vec256> {
    let mut s = KeccakState::<4, Vec256>::new();
    let data_len = data[0].len();
    let data_blocks = data_len / RATE;
    let rem = data_len % RATE;
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"let zeros : t_Array u64 (mk_usize 25) =
               Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
           EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 0;
           EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 1;
           EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 2;
           EquivImplSpec.Keccakf.Avx2.lemma_extract_lane_zero_avx2 3;
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) (Core_models.Ops.Index.f_index $data (mk_usize 0));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) (Core_models.Ops.Index.f_index $data (mk_usize 1));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) (Core_models.Ops.Index.f_index $data (mk_usize 2));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) (Core_models.Ops.Index.f_index $data (mk_usize 3))"#
    );
    for i in 0..data_blocks {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"let zeros : t_Array u64 (mk_usize 25) =
                       Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
                   v $i <= v $data_blocks /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 0) ==
                     Hacspec_sha3.Sponge.Lemmas.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 0)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 1) ==
                     Hacspec_sha3.Sponge.Lemmas.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 1)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 2) ==
                     Hacspec_sha3.Sponge.Lemmas.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 2)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 3) ==
                     Hacspec_sha3.Sponge.Lemmas.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 3))"#
            )
        });
        #[cfg(hax)]
        lemma_mul_succ_le(i, data_blocks, RATE);

        #[cfg(hax)]
        hax_lib::fstar!(
            r#"let zeros : t_Array u64 (mk_usize 25) =
                   Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
               assert (Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) $data);
               EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2
                   $RATE $s $data ($i *! $RATE) 0;
               EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2
                   $RATE $s $data ($i *! $RATE) 1;
               EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2
                   $RATE $s $data ($i *! $RATE) 2;
               EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_block_avx2
                   $RATE $s $data ($i *! $RATE) 3;
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1)
                   (Core_models.Ops.Index.f_index $data (mk_usize 0));
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1)
                   (Core_models.Ops.Index.f_index $data (mk_usize 1));
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1)
                   (Core_models.Ops.Index.f_index $data (mk_usize 2));
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1)
                   (Core_models.Ops.Index.f_index $data (mk_usize 3))"#
        );

        s.absorb_block::<RATE>(data, i * RATE);
    }
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"let zeros : t_Array u64 (mk_usize 25) =
               Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
           assert (Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) $data);
           EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2
               $RATE $DELIM $s $data ($data_len -! $rem) $rem 0;
           EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2
               $RATE $DELIM $s $data ($data_len -! $rem) $rem 1;
           EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2
               $RATE $DELIM $s $data ($data_len -! $rem) $rem 2;
           EquivImplSpec.Sponge.Avx2.Steps.lemma_absorb_last_avx2
               $RATE $DELIM $s $data ($data_len -! $rem) $rem 3;
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 0));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 1));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 2));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 3))"#
    );
    s.absorb_final::<RATE, DELIM>(data, data_len - rem, rem);
    s
}

// Full-blocks loop engine of `squeeze4` (mirrors the Arm64
// `squeeze2_blocks` shape at N=4): first block (no keccakf) + the
// `1..blocks` loop only — NO `blocks==0` case and NO trailing partial
// block.  Both of those live in `squeeze4`, so each function's VC stays
// small and there is no branch-merge inside this loop-bearing body
// (which saturates a monolithic VC).  Ensures: per-lane state
// advanced to `iterate_keccak_f (blocks-1)` plus the opaque
// `squeezed_upto` prefix at `blocks*RATE`.
#[inline]
#[cfg_attr(hax, hax_lib::requires(
    valid_rate(RATE) &&
    out0.len() == out1.len() &&
    out0.len() == out2.len() &&
    out0.len() == out3.len() &&
    blocks > 0 &&
    blocks == out0.len() / RATE
))]
#[cfg_attr(hax, hax_lib::ensures(|_| (future(out0).len() == out0.len()
    && future(out1).len() == out1.len()
    && future(out2).len() == out2.len()
    && future(out3).len() == out3.len()).to_prop() & {
    fstar!(r#"
        let outlen = Core_models.Slice.impl__len #u8 $out0 in
        v outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
          ((EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_future.st 0) ==
             Hacspec_sha3.Sponge.iterate_keccak_f ($blocks -! mk_usize 1)
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 0) /\
           (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_future.st 1) ==
             Hacspec_sha3.Sponge.iterate_keccak_f ($blocks -! mk_usize 1)
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 1) /\
           (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_future.st 2) ==
             Hacspec_sha3.Sponge.iterate_keccak_f ($blocks -! mk_usize 1)
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 2) /\
           (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
              EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_future.st 3) ==
             Hacspec_sha3.Sponge.iterate_keccak_f ($blocks -! mk_usize 1)
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 3) /\
           EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
             (out0_future <: Seq.seq u8)
             (Hacspec_sha3.Sponge.squeeze outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                   EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 0) $RATE <: Seq.seq u8)
             (v $blocks * v $RATE) /\
           EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
             (out1_future <: Seq.seq u8)
             (Hacspec_sha3.Sponge.squeeze outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                   EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 1) $RATE <: Seq.seq u8)
             (v $blocks * v $RATE) /\
           EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
             (out2_future <: Seq.seq u8)
             (Hacspec_sha3.Sponge.squeeze outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                   EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 2) $RATE <: Seq.seq u8)
             (v $blocks * v $RATE) /\
           EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
             (out3_future <: Seq.seq u8)
             (Hacspec_sha3.Sponge.squeeze outlen
                (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                   EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 3) $RATE <: Seq.seq u8)
             (v $blocks * v $RATE))
    "#)
}))]
#[cfg_attr(hax, hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane'"))]
fn squeeze4_blocks<const RATE: usize>(
    s: &mut KeccakState<4, Vec256>,
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    blocks: usize,
) {
    #[cfg(hax)]
    let out0_len = out0.len();
    #[cfg(hax)]
    let out1_len = out1.len();
    #[cfg(hax)]
    let out2_len = out2.len();
    #[cfg(hax)]
    let out3_len = out3.len();
    #[cfg(hax)]
    let s_init_st = s.st;

    #[cfg(hax)]
    let outlen = out0.len();

    #[cfg(hax)]
    hax_lib::fstar!(
        r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
             (if v $outlen < v $RATE
              then FStar.Math.Lemmas.small_div (v $outlen) (v $RATE));
             assert (v $RATE <= v $outlen);
             EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_iterate_keccak_f_zero
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0);
             EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_iterate_keccak_f_zero
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1);
             EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_iterate_keccak_f_zero
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2);
             EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_iterate_keccak_f_zero
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3);
             EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_first_driver_avx2
               $RATE $s $out0 $out1 $out2 $out3 $RATE
           end"#
    );
    s.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);
    for i in 1..blocks {
        #[cfg(hax)]
        hax_lib::loop_invariant!(|i: usize| (out0.len() == out0_len
            && out1.len() == out1_len
            && out2.len() == out2_len
            && out3.len() == out3_len)
            .to_prop()
            & {
                fstar!(
                    r#"v $i >= 1 /\ v $i <= v $blocks /\
                       (v $outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
                         (v $i * v $RATE <= v $outlen /\
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 0) ==
                            Hacspec_sha3.Sponge.iterate_keccak_f ($i -! mk_usize 1)
                              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                 EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) /\
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 1) ==
                            Hacspec_sha3.Sponge.iterate_keccak_f ($i -! mk_usize 1)
                              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                 EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) /\
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 2) ==
                            Hacspec_sha3.Sponge.iterate_keccak_f ($i -! mk_usize 1)
                              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                 EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) /\
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 3) ==
                            Hacspec_sha3.Sponge.iterate_keccak_f ($i -! mk_usize 1)
                              (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                 EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) /\
                          EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
                            ($out0 <: Seq.seq u8)
                            (Hacspec_sha3.Sponge.squeeze
                               (Core_models.Slice.impl__len #u8 $out0)
                               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0)
                               $RATE <: Seq.seq u8)
                            (v $i * v $RATE) /\
                          EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
                            ($out1 <: Seq.seq u8)
                            (Hacspec_sha3.Sponge.squeeze
                               (Core_models.Slice.impl__len #u8 $out1)
                               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1)
                               $RATE <: Seq.seq u8)
                            (v $i * v $RATE) /\
                          EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
                            ($out2 <: Seq.seq u8)
                            (Hacspec_sha3.Sponge.squeeze
                               (Core_models.Slice.impl__len #u8 $out2)
                               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2)
                               $RATE <: Seq.seq u8)
                            (v $i * v $RATE) /\
                          EquivImplSpec.Sponge.Avx2.SqueezeDriver.squeezed_upto
                            ($out3 <: Seq.seq u8)
                            (Hacspec_sha3.Sponge.squeeze
                               (Core_models.Slice.impl__len #u8 $out3)
                               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3)
                               $RATE <: Seq.seq u8)
                            (v $i * v $RATE)))"#
                )
            });
        #[cfg(hax)]
        lemma_mul_succ_le(i, blocks, RATE);

        #[cfg(hax)]
        hax_lib::fstar!(
            r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
                 Libcrux_sha3.Proof_utils.Lemmas.lemma_div_mul_mod $outlen $RATE;
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_mid_driver_avx2
                   $RATE $s_init_st $s $out0 $out1 $out2 $out3 $i
               end"#
        );

        s.keccakf1600();
        s.squeeze4::<RATE>(out0, out1, out2, out3, i * RATE, RATE);
    }
}

/// Squeeze phase of `keccak4`: extract `out0.len()` bytes from each
/// lane of `s` into `out0..out3`, applying Keccak-f between each
/// full rate-byte block of output.  Mirrors the Arm64 `squeeze2`
/// shape: branch on `blocks==0`, delegate the full-blocks loop to
/// `squeeze4_blocks`, handle the trailing partial block, and discharge
/// the full `Seq`-equality functional spec via `lemma_squeezed_upto_full`
/// — closing within each branch so no VC carries both the loop and the
/// byteform `squeeze` equality.
#[inline]
#[cfg_attr(hax, hax_lib::requires(
    valid_rate(RATE) &&
    out0.len() < usize::MAX - 200 &&
    out0.len() == out1.len() &&
    out0.len() == out2.len() &&
    out0.len() == out3.len()
))]
#[cfg_attr(hax, hax_lib::ensures(|_| (future(out0).len() == out0.len()
    && future(out1).len() == out1.len()
    && future(out2).len() == out2.len()
    && future(out3).len() == out3.len()).to_prop() & {
    fstar!(r#"
        let outlen = Core_models.Slice.impl__len #u8 $out0 in
        v outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
          (out0_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 0) $RATE <: t_Slice u8) /\
          (out1_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 1) $RATE <: t_Slice u8) /\
          (out2_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 2) $RATE <: t_Slice u8) /\
          (out3_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                  EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 3) $RATE <: t_Slice u8)
    "#)
}))]
#[cfg_attr(hax, hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane'"))]
pub(crate) fn squeeze4<const RATE: usize>(
    mut s: KeccakState<4, Vec256>,
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
) {
    #[cfg(hax)]
    let s_init_st = s.st;

    let outlen = out0.len();
    let blocks = outlen / RATE;
    let last = outlen - (outlen % RATE);

    if blocks == 0 {
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_first_driver_avx2
                   $RATE $s $out0 $out1 $out2 $out3 $outlen
               end"#
        );
        s.squeeze4::<RATE>(out0, out1, out2, out3, 0, outlen);
        #[cfg(hax)]
        hax_lib::fstar!(
            r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) $RATE;
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) $RATE;
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) $RATE;
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) $RATE;
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                   ($out0 <: Seq.seq u8)
                   (Hacspec_sha3.Sponge.squeeze $outlen
                      (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                         EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) $RATE <: Seq.seq u8);
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                   ($out1 <: Seq.seq u8)
                   (Hacspec_sha3.Sponge.squeeze $outlen
                      (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                         EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) $RATE <: Seq.seq u8);
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                   ($out2 <: Seq.seq u8)
                   (Hacspec_sha3.Sponge.squeeze $outlen
                      (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                         EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) $RATE <: Seq.seq u8);
                 EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                   ($out3 <: Seq.seq u8)
                   (Hacspec_sha3.Sponge.squeeze $outlen
                      (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                         EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) $RATE <: Seq.seq u8)
               end"#
        );
    } else {
        squeeze4_blocks::<RATE>(&mut s, out0, out1, out2, out3, blocks);
        if last < outlen {
            #[cfg(hax)]
            hax_lib::fstar!(
                r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
                     Math.Lemmas.lemma_div_mod (v $outlen) (v $RATE);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_blocks_rate_split $outlen $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_tail_driver_avx2
                       $RATE $s_init_st $s $out0 $out1 $out2 $out3 $blocks
                   end"#
            );
            s.keccakf1600();
            s.squeeze4::<RATE>(out0, out1, out2, out3, last, outlen - last);
            #[cfg(hax)]
            hax_lib::fstar!(
                r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out0 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) $RATE <: Seq.seq u8);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out1 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) $RATE <: Seq.seq u8);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out2 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) $RATE <: Seq.seq u8);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out3 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) $RATE <: Seq.seq u8)
                   end"#
            );
        } else {
            #[cfg(hax)]
            hax_lib::fstar!(
                r#"if v $outlen < v Core_models.Num.impl_usize__MAX - 200 then begin
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_exact_multiple $outlen $RATE;
                     assert (v $blocks * v $RATE == v $outlen);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeeze_length $outlen
                       (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                          EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) $RATE;
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out0 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 0) $RATE <: Seq.seq u8);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out1 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 1) $RATE <: Seq.seq u8);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out2 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 2) $RATE <: Seq.seq u8);
                     EquivImplSpec.Sponge.Avx2.SqueezeDriver.lemma_squeezed_upto_full
                       ($out3 <: Seq.seq u8)
                       (Hacspec_sha3.Sponge.squeeze $outlen
                          (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                             EquivImplSpec.Keccakf.Avx2.lc_avx2 $s_init_st 3) $RATE <: Seq.seq u8)
                   end"#
            );
        }
    }
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(
    valid_rate(RATE) &&
    out0.len() < usize::MAX - 200 &&
    out0.len() == out1.len() &&
    out0.len() == out2.len() &&
    out0.len() == out3.len() &&
    data[0].len() == data[1].len() &&
    data[0].len() == data[2].len() &&
    data[0].len() == data[3].len()
))]
#[cfg_attr(hax, hax_lib::ensures(|_|
    future(out0).len() == out0.len() &&
    future(out1).len() == out1.len() &&
    future(out2).len() == out2.len() &&
    future(out3).len() == out3.len()
))]
pub(crate) fn keccak4<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 4],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
) {
    #[cfg(not(eurydice))]
    debug_assert!(out0.len() == out1.len() && out0.len() == out2.len() && out0.len() == out3.len());
    #[cfg(not(eurydice))]
    debug_assert!(
        data[0].len() == data[1].len()
            && data[0].len() == data[2].len()
            && data[0].len() == data[3].len()
    );

    let s = absorb4::<RATE, DELIM>(data);
    squeeze4::<RATE>(s, out0, out1, out2, out3);
}

// FIXME(hax#1698): `fstar::options` on inherent-impl methods fails to
// expand, so the push-options go on a hax-only dummy function before the
// impl.  The free functions after it set their own options, so no
// `#pop-options` is needed.
#[cfg(hax)]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always""#
    )
)]
fn _keccak_state_impl4_opts() {}

#[cfg_attr(hax, hax_lib::attributes)]
impl KeccakState<4, Vec256> {
    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    ))]
    pub(crate) fn squeeze_next_block<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
        start: usize,
    ) {
        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, start, RATE);
    }

    /// Write out the first block of Keccak output.
    ///
    /// This function MUST NOT be called after any of the other `squeeze_*`
    /// functions have been called, since that would result in a duplicate output
    /// block.
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    ))]
    pub(crate) fn squeeze_first_block<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        self.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    ))]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        self.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 2 * RATE, RATE);
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    ))]
    pub(crate) fn squeeze_first_five_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        self.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 2 * RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 3 * RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 4 * RATE, RATE);
    }
}
