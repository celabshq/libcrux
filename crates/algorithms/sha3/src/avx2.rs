/// Performing 4 operations in parallel
pub mod x4 {
    #[cfg(hax)]
    use hax_lib::int::ToInt;
    #[cfg(hax)]
    use hax_lib::prop::*;

    use crate::generic_keccak::simd256::keccak4;

    /// Perform 4 SHAKE256 operations in parallel
    ///
    /// The ensures below are F*-verified: each lane equals the scalar
    /// `keccak` of its respective input (via the four-lane AVX2 driver
    /// `lemma_keccak4_avx2`).  Mirrors the Neon (x2) parallel API analogue.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    #[hax_lib::requires(
        out0.len() < usize::MAX - 200 &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len() &&
        input0.len() == input1.len() &&
        input0.len() == input2.len() &&
        input0.len() == input3.len()
    )]
    #[hax_lib::ensures(|_| (future(out0).len() == out0.len()
        && future(out1).len() == out1.len()
        && future(out2).len() == out2.len()
        && future(out3).len() == out3.len()).to_prop() & {
        fstar!(r#"
            (out0_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $out0)
                 (mk_usize 136) (mk_u8 31) $input0 <: t_Slice u8) /\
            (out1_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $out1)
                 (mk_usize 136) (mk_u8 31) $input1 <: t_Slice u8) /\
            (out2_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $out2)
                 (mk_usize 136) (mk_u8 31) $input2 <: t_Slice u8) /\
            (out3_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $out3)
                 (mk_usize 136) (mk_u8 31) $input3 <: t_Slice u8)
        "#)
    })]
    #[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
    pub fn shake256(
        input0: &[u8],
        input1: &[u8],
        input2: &[u8],
        input3: &[u8],
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        hax_lib::fstar!(
            r#"let inputs : t_Array (t_Slice u8) (mk_usize 4) =
                   let l : list (t_Slice u8) = [ $input0; $input1; $input2; $input3 ] in
                   FStar.Pervasives.assert_norm (List.Tot.length l == 4);
                   Rust_primitives.Hax.array_of_list 4 l in
               FStar.Pervasives.assert_norm (inputs.[ mk_usize 0 ] == $input0);
               FStar.Pervasives.assert_norm (inputs.[ mk_usize 1 ] == $input1);
               FStar.Pervasives.assert_norm (inputs.[ mk_usize 2 ] == $input2);
               FStar.Pervasives.assert_norm (inputs.[ mk_usize 3 ] == $input3);
               EquivImplSpec.Sponge.Avx2.Driver.lemma_slices_same_len4 inputs;
               EquivImplSpec.Sponge.Avx2.Driver.lemma_keccak4_avx2
                   (mk_usize 136) (mk_u8 31) inputs
                   ($out0 <: t_Slice u8) ($out1 <: t_Slice u8)
                   ($out2 <: t_Slice u8) ($out3 <: t_Slice u8)"#
        );
        keccak4::<136, 0x1fu8>(&[input0, input1, input2, input3], out0, out1, out2, out3);
    }

    /// An incremental API to perform 4 operations in parallel
    pub mod incremental {
        #[cfg(hax)]
        use hax_lib::int::ToInt;

        use crate::generic_keccak::KeccakState as GenericState;
        use libcrux_intrinsics::avx2::*;

        /// The Keccak state for the incremental API.
        pub struct KeccakState {
            state: GenericState<4, Vec256>,
        }

        /// Initialise the [`KeccakState`].
        #[inline(always)]
        pub fn init() -> KeccakState {
            KeccakState {
                state: GenericState::new(),
            }
        }

        /// Absorb
        #[inline(always)]
        #[hax_lib::requires(
            data0.len().to_int() < hax_lib::int!(168) &&
            data0.len() == data1.len() &&
            data0.len() == data2.len() &&
            data0.len() == data3.len()
        )]
        pub fn shake128_absorb_final(
            s: &mut KeccakState,
            data0: &[u8],
            data1: &[u8],
            data2: &[u8],
            data3: &[u8],
        ) {
            s.state
                .absorb_final::<168, 0x1fu8>(&[data0, data1, data2, data3], 0, data0.len());
        }

        /// Absorb
        #[inline(always)]
        #[hax_lib::requires(
            data0.len().to_int() < hax_lib::int!(136) &&
            data0.len() == data1.len() &&
            data0.len() == data2.len() &&
            data0.len() == data3.len()
        )]
        pub fn shake256_absorb_final(
            s: &mut KeccakState,
            data0: &[u8],
            data1: &[u8],
            data2: &[u8],
            data3: &[u8],
        ) {
            s.state
                .absorb_final::<136, 0x1fu8>(&[data0, data1, data2, data3], 0, data0.len());
        }

        /// Squeeze block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(136) &&
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
        pub fn shake256_squeeze_first_block(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
            out2: &mut [u8],
            out3: &mut [u8],
        ) {
            s.state.squeeze_first_block::<136>(out0, out1, out2, out3);
        }

        /// Squeeze next block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(136) &&
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
        pub fn shake256_squeeze_next_block(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
            out2: &mut [u8],
            out3: &mut [u8],
        ) {
            s.state.squeeze_next_block::<136>(out0, out1, out2, out3, 0);
        }

        /// Squeeze three blocks
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(504) && // 3 * 168 = 504
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
        pub fn shake128_squeeze_first_three_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
            out2: &mut [u8],
            out3: &mut [u8],
        ) {
            s.state
                .squeeze_first_three_blocks::<168>(out0, out1, out2, out3);
        }

        /// Squeeze five blocks
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(840) && // 5 * 168 = 840
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
        pub fn shake128_squeeze_first_five_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
            out2: &mut [u8],
            out3: &mut [u8],
        ) {
            s.state
                .squeeze_first_five_blocks::<168>(out0, out1, out2, out3);
        }

        /// Squeeze another block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(168) &&
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
        pub fn shake128_squeeze_next_block(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
            out2: &mut [u8],
            out3: &mut [u8],
        ) {
            s.state.squeeze_next_block::<168>(out0, out1, out2, out3, 0);
        }
    }
}
