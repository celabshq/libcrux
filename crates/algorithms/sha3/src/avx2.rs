/// Performing 4 operations in parallel
pub mod x4 {
    use crate::generic_keccak::simd256::keccak4;

    /// Perform 4 SHAKE256 operations in parallel
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
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
        keccak4::<136, 0x1fu8>(&[input0, input1, input2, input3], out0, out1, out2, out3);
    }

    /// An incremental API to perform 4 operations in parallel
    pub mod incremental {
        use crate::generic_keccak::KeccakState as GenericState;
        use libcrux_intrinsics::avx2::*;

        /// The Keccak state for the incremental API.
        pub struct KeccakState {
            state: GenericState<4, Vec256>,
        }

        #[cfg(not(eurydice))]
        #[cfg_attr(hax, hax_lib::exclude)]
        impl Drop for KeccakState {
            fn drop(&mut self) {
                use libcrux_secrets::Zeroize;
                self.state.zeroize();
            }
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
