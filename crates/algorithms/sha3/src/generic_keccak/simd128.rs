use super::*;

#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[cfg(hax)]
use hax_lib::prop::*;

use libcrux_intrinsics::arm64::_uint64x2_t;

/// Absorb phase of `keccak2`: initialise a two-lane Keccak state,
/// absorb all full rate-byte blocks of `data[0]` and `data[1]` in
/// parallel, then pad and absorb each lane's final partial block
/// with domain-separation byte `DELIM` and the pad10*1 terminator.
///
/// The ensures clause asserts per-lane equality with the scalar spec
/// function `Hacspec_sha3.Sponge.absorb`.  The loop invariant uses
/// `absorb_blocks` per lane, mirroring the Portable backend.
#[inline]
#[hax_lib::requires(valid_rate(RATE) && data[0].len() == data[1].len())]
#[hax_lib::ensures(|result| fstar!(r#"
    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
       EquivImplSpec.Keccakf.Arm64.lc_arm64 $result.st 0) ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 0)) /\
    (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
       EquivImplSpec.Keccakf.Arm64.lc_arm64 $result.st 1) ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 1))
"#))]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always")]
pub(crate) fn absorb2<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 2],
) -> KeccakState<2, _uint64x2_t> {
    let mut s = KeccakState::<2, _uint64x2_t>::new();
    let data_len = data[0].len();
    let data_blocks = data_len / RATE;
    let rem = data_len % RATE;
    hax_lib::fstar!(
        r#"let zeros : t_Array u64 (mk_usize 25) =
               Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
           EquivImplSpec.Keccakf.Arm64.lemma_extract_lane_zero_arm64 0;
           EquivImplSpec.Keccakf.Arm64.lemma_extract_lane_zero_arm64 1;
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) (Core_models.Ops.Index.f_index $data (mk_usize 0));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) (Core_models.Ops.Index.f_index $data (mk_usize 1))"#
    );
    for i in 0..data_blocks {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"let zeros : t_Array u64 (mk_usize 25) =
                       Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
                   v $i <= v $data_blocks /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                      EquivImplSpec.Keccakf.Arm64.lc_arm64 $s.st 0) ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 0)) /\
                   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                      EquivImplSpec.Keccakf.Arm64.lc_arm64 $s.st 1) ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i (Core_models.Ops.Index.f_index $data (mk_usize 1))"#
            )
        });
        #[cfg(hax)]
        lemma_mul_succ_le(i, data_blocks, RATE);

        hax_lib::fstar!(
            r#"let zeros : t_Array u64 (mk_usize 25) =
                   Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
               EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_block_arm64
                   $RATE $s $data ($i *! $RATE) 0;
               EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_block_arm64
                   $RATE $s $data ($i *! $RATE) 1;
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1)
                   (Core_models.Ops.Index.f_index $data (mk_usize 0));
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1)
                   (Core_models.Ops.Index.f_index $data (mk_usize 1))"#
        );

        s.absorb_block::<RATE>(data, i * RATE);
    }
    hax_lib::fstar!(
        r#"let zeros : t_Array u64 (mk_usize 25) =
               Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
           EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_last_arm64
               $RATE $DELIM $s $data ($data_len -! $rem) $rem 0;
           EquivImplSpec.Sponge.Arm64.Steps.lemma_absorb_last_arm64
               $RATE $DELIM $s $data ($data_len -! $rem) $rem 1;
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 0));
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM (Core_models.Ops.Index.f_index $data (mk_usize 1))"#
    );
    s.absorb_final::<RATE, DELIM>(data, data_len - rem, rem);
    s
}

/// Squeeze phase of `keccak2`: extract `out0.len()` bytes from each
/// lane of `s` into `out0` and `out1`, applying Keccak-f between
/// each full rate-byte block of output.
#[inline]
#[hax_lib::requires(valid_rate(RATE) && out0.len() == out1.len())]
#[hax_lib::ensures(|_| (future(out0).len() == out0.len() && future(out1).len() == out1.len()).to_prop() & {
    fstar!(r#"
        let outlen = Core_models.Slice.impl__len #u8 $out0 in
        v outlen < v Core_models.Num.impl_usize__MAX - 200 ==>
          (out0_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64 $s.st 0)
               $RATE <: t_Slice u8) /\
          (out1_future <: t_Slice u8) ==
            (Hacspec_sha3.Sponge.squeeze outlen
               (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2)
                  EquivImplSpec.Keccakf.Arm64.lc_arm64 $s.st 1)
               $RATE <: t_Slice u8)
    "#)
})]
#[hax_lib::fstar::options("--z3rlimit 300 --admit_smt_queries true")]
pub(crate) fn squeeze2<const RATE: usize>(
    mut s: KeccakState<2, _uint64x2_t>,
    out0: &mut [u8],
    out1: &mut [u8],
) {
    #[cfg(hax)]
    let out0_len = out0.len();
    #[cfg(hax)]
    let out1_len = out1.len();

    let outlen = out0.len();
    let blocks = outlen / RATE;
    let last = outlen - (outlen % RATE);

    if blocks == 0 {
        s.squeeze2::<RATE>(out0, out1, 0, outlen);
    } else {
        s.squeeze2::<RATE>(out0, out1, 0, RATE);
        for i in 1..blocks {
            hax_lib::loop_invariant!(|_: usize| out0.len() == out0_len && out1.len() == out1_len);
            #[cfg(hax)]
            lemma_mul_succ_le(i, blocks, RATE);

            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, i * RATE, RATE);
        }
        if last < outlen {
            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, last, outlen - last);
        }
    }
}

/// Two-lane Keccak driver.  The function-level ensures keep only the
/// bounds (length-preservation) here; the per-lane functional spec is
/// proved at the Neon-wrapper level (in `src/neon.rs`) using
/// `EquivImplSpec.Sponge.Arm64.API.lemma_keccak2_arm64`, which composes
/// `lemma_absorb2_arm64` + `lemma_squeeze2_arm64`.  Wiring the
/// per-lane functional spec here would create a circular dependency:
/// `EquivImplSpec.Sponge.Arm64.API` depends on this module's
/// `absorb2`, `squeeze2`, `keccak2`, so it cannot itself be cited
/// from this module's body.
#[inline]
#[hax_lib::requires(
    valid_rate(RATE) &&
    out0.len() == out1.len() &&
    data[0].len() == data[1].len()
)]
#[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
pub(crate) fn keccak2<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 2],
    out0: &mut [u8],
    out1: &mut [u8],
) {
    #[cfg(not(eurydice))]
    debug_assert!(out0.len() == out1.len());
    #[cfg(not(eurydice))]
    debug_assert!(data[0].len() == data[1].len());

    let s = absorb2::<RATE, DELIM>(data);
    squeeze2::<RATE>(s, out0, out1);
}

#[hax_lib::attributes]
impl KeccakState<2, _uint64x2_t> {
    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
    pub(crate) fn squeeze_next_block<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        start: usize,
    ) {
        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, start, RATE);
    }

    /// Write out the first block of Keccak output.
    ///
    /// This function MUST NOT be called after any of the other `squeeze_*`
    /// functions have been called, since that would result in a duplicate output
    /// block.
    #[hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out0.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
    pub(crate) fn squeeze_first_block<const RATE: usize>(&self, out0: &mut [u8], out1: &mut [u8]) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out0.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
    ) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 2 * RATE, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out0.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
    pub(crate) fn squeeze_first_five_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
    ) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 2 * RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 3 * RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 4 * RATE, RATE);
    }
}
