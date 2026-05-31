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
#[hax_lib::requires(
    valid_rate(RATE) &&
    data[0].len() == data[1].len() &&
    data[0].len() == data[2].len() &&
    data[0].len() == data[3].len()
)]
#[hax_lib::ensures(|result| fstar!(r#"
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
"#))]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always")]
pub(crate) fn absorb4<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 4],
) -> KeccakState<4, Vec256> {
    let mut s = KeccakState::<4, Vec256>::new();
    let data_len = data[0].len();
    let data_blocks = data_len / RATE;
    let rem = data_len % RATE;
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
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"let zeros : t_Array u64 (mk_usize 25) =
                       Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
                   v $i <= v $data_blocks /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 0) ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 0)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 1) ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 1)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 2) ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 2)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                      EquivImplSpec.Keccakf.Avx2.lc_avx2 $s.st 3) ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 3))"#
            )
        });
        #[cfg(hax)]
        lemma_mul_succ_le(i, data_blocks, RATE);

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

/// Squeeze phase of `keccak4`: extract `out0.len()` bytes from each
/// lane of `s` into `out0..out3`, applying Keccak-f between each
/// full rate-byte block of output.
///
/// **Per-lane spec-equivalence ensures NOT YET proved.**  The
/// monolithic inline-ensures approach (mirror of Portable.squeeze at
/// N=4) hits the same Z3 BoxBool cascade documented in the
/// 2026-04-25 squeeze2 post-mortem; at N=4 it is even worse (8 forall
/// conjuncts in the loop invariant vs 4 at N=2).  See HANDOFF.md
/// "2026-04-25 (later)" section for the path forward via per-lane
/// `Sponge.Avx2.Steps` lemmas (Option B).  Until that lands,
/// `lemma_squeeze4_avx2` in `EquivImplSpec.Sponge.Avx2.API.fst`
/// remains an `assume val`.  The `squeeze_last4` helper above and the
/// `out0.len() < usize::MAX - 200` precondition propagation up to
/// `keccak4` and `avx2::shake256` are kept — they are infrastructure
/// for the eventual proof.
#[inline]
#[hax_lib::requires(
    valid_rate(RATE) &&
    out0.len() < usize::MAX - 200 &&
    out0.len() == out1.len() &&
    out0.len() == out2.len() &&
    out0.len() == out3.len()
)]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len()
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
})]
#[hax_lib::fstar::options("--z3rlimit 600 --split_queries always --admit_smt_queries true")]
pub(crate) fn squeeze4<const RATE: usize>(
    mut s: KeccakState<4, Vec256>,
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
) {
    #[cfg(hax)]
    let out0_len = out0.len();
    #[cfg(hax)]
    let out1_len = out1.len();
    #[cfg(hax)]
    let out2_len = out2.len();
    #[cfg(hax)]
    let out3_len = out3.len();

    let outlen = out0.len();
    let blocks = outlen / RATE;
    let last = outlen - (outlen % RATE);

    if blocks == 0 {
        s.squeeze4::<RATE>(out0, out1, out2, out3, 0, outlen);
    } else {
        s.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);
        for i in 1..blocks {
            hax_lib::loop_invariant!(|_: usize| out0.len() == out0_len
                && out1.len() == out1_len
                && out2.len() == out2_len
                && out3.len() == out3_len);
            #[cfg(hax)]
            lemma_mul_succ_le(i, blocks, RATE);

            s.keccakf1600();
            s.squeeze4::<RATE>(out0, out1, out2, out3, i * RATE, RATE);
        }
        if last < outlen {
            s.keccakf1600();
            s.squeeze4::<RATE>(out0, out1, out2, out3, last, outlen - last);
        }
    }
}

#[inline(always)]
#[hax_lib::requires(
    valid_rate(RATE) &&
    out0.len() < usize::MAX - 200 &&
    out0.len() == out1.len() &&
    out0.len() == out2.len() &&
    out0.len() == out3.len() &&
    data[0].len() == data[1].len() &&
    data[0].len() == data[2].len() &&
    data[0].len() == data[3].len()
)]
#[hax_lib::ensures(|_|
    future(out0).len() == out0.len() &&
    future(out1).len() == out1.len() &&
    future(out2).len() == out2.len() &&
    future(out3).len() == out3.len()
)]
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

// Workaround for hax#1698: `fstar::options` on methods inside an
// inherent impl triggers a macro-expansion error.  Push options on a
// hax-only dummy function before the impl, mirroring Portable's
// `_keccak_state_impl_opts`.  No `#pop-options` is needed — the
// last items in this file are the free `keccak4` etc. with their own
// options.
#[cfg(hax)]
#[hax_lib::fstar::before(
    r#"#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always""#
)]
fn _keccak_state_impl4_opts() {}

#[hax_lib::attributes]
impl KeccakState<4, Vec256> {
    /// Trailing partial-block squeeze for 4-lane state.  If
    /// `output_rem != 0` apply one Keccak-f permutation and squeeze
    /// the trailing `output_rem` bytes into each lane's output;
    /// otherwise a no-op.  Mirrors the Portable
    /// `KeccakState<1,u64>::squeeze_last` factor-out so the final
    /// reconcile in `squeeze4` lands in a small VC.
    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        out0.len() < usize::MAX - 200 &&
        output_rem < RATE &&
        output_rem <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_| (future(out0).len() == out0.len() &&
                             future(out1).len() == out1.len() &&
                             future(out2).len() == out2.len() &&
                             future(out3).len() == out3.len()).to_prop() & {
        fstar!(r#"
            let out_len = Core_models.Slice.impl__len #u8 $out0 in
            let lane_st_pre  (l: nat{l<4}) =
                EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                    EquivImplSpec.Keccakf.Avx2.lc_avx2 $self_.st l in
            let lane_st_post (l: nat{l<4}) =
                EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4)
                    EquivImplSpec.Keccakf.Avx2.lc_avx2
                    self_e_future.Libcrux_sha3.Generic_keccak.f_st l in
            let (st_spec_0, out_spec_0) =
                Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 0) $out0 $RATE $output_rem in
            let (st_spec_1, out_spec_1) =
                Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 1) $out1 $RATE $output_rem in
            let (st_spec_2, out_spec_2) =
                Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 2) $out2 $RATE $output_rem in
            let (st_spec_3, out_spec_3) =
                Hacspec_sha3.Sponge.squeeze_last out_len (lane_st_pre 3) $out3 $RATE $output_rem in
            lane_st_post 0 == st_spec_0 /\ (out0_future <: Seq.seq u8) == (out_spec_0 <: Seq.seq u8) /\
            lane_st_post 1 == st_spec_1 /\ (out1_future <: Seq.seq u8) == (out_spec_1 <: Seq.seq u8) /\
            lane_st_post 2 == st_spec_2 /\ (out2_future <: Seq.seq u8) == (out_spec_2 <: Seq.seq u8) /\
            lane_st_post 3 == st_spec_3 /\ (out3_future <: Seq.seq u8) == (out_spec_3 <: Seq.seq u8)
        "#)
    })]
    pub(crate) fn squeeze_last4<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
        output_rem: usize,
    ) {
        #[cfg(hax)]
        let out0_original = out0.to_vec();
        #[cfg(hax)]
        let out1_original = out1.to_vec();
        #[cfg(hax)]
        let out2_original = out2.to_vec();
        #[cfg(hax)]
        let out3_original = out3.to_vec();
        #[cfg(hax)]
        let self_original_st = self.st;
        if output_rem != 0 {
            self.keccakf1600();
            let out_len = out0.len();
            self.squeeze4::<RATE>(out0, out1, out2, out3, out_len - output_rem, output_rem);
            hax_lib::fstar!(
                r#"
                let outputs : t_Array (t_Slice u8) (mk_usize 4) =
                    let l : list (t_Slice u8) =
                        [ Alloc.Vec.impl_1__as_slice $out0_original;
                          Alloc.Vec.impl_1__as_slice $out1_original;
                          Alloc.Vec.impl_1__as_slice $out2_original;
                          Alloc.Vec.impl_1__as_slice $out3_original ] in
                    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length l) 4);
                    Rust_primitives.Hax.array_of_list 4 l in
                let ks_pre : Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4)
                                Libcrux_intrinsics.Avx2_extract.t_Vec256 =
                    { Libcrux_sha3.Generic_keccak.f_st = $self_original_st } in
                let start : usize = (Core_models.Slice.impl__len #u8 $out0) -! $output_rem in
                EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2
                    $RATE ks_pre outputs start $output_rem 0;
                EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2
                    $RATE ks_pre outputs start $output_rem 1;
                EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2
                    $RATE ks_pre outputs start $output_rem 2;
                EquivImplSpec.Sponge.Avx2.Steps.lemma_squeeze_last_avx2
                    $RATE ks_pre outputs start $output_rem 3"#
            );
        }
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
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
    #[hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
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
    #[hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
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
    #[hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
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
