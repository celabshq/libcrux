/// Performing 2 operations in parallel
pub mod x2 {
    use crate::generic_keccak::simd128::keccak2;

    /// Run SHAKE256 on both inputs in parallel.
    ///
    /// Writes the two results into `out0` and `out1`
    #[allow(unused_variables)]
    #[hax_lib::requires(
        input0.len() == input1.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len()
    )]
    #[inline(always)]
    pub fn shake256(input0: &[u8], input1: &[u8], out0: &mut [u8], out1: &mut [u8]) {
        keccak2::<136, 0x1fu8>(&[input0, input1], out0, out1);
    }

    /// An incremental API to perform 2 operations in parallel
    pub mod incremental {
        #[cfg(hax)]
        use hax_lib::ToInt;

        use crate::generic_keccak::KeccakState as GenericState;

        /// The Keccak state for the incremental API.
        #[hax_lib::fstar::before("noeq")]
        pub struct KeccakState {
            state: GenericState<2, crate::simd::arm64::uint64x2_t>,
        }

        type KeccakState2Internal = GenericState<2, crate::simd::arm64::uint64x2_t>;

        /// Initialise the `KeccakState2`.
        #[inline(always)]
        pub fn init() -> KeccakState {
            KeccakState {
                state: KeccakState2Internal::new(),
            }
        }

        /// Shake128 absorb `data0` and `data1` in the [`KeccakState`] `s`.
        #[hax_lib::requires(
            data0.len().to_int() < hax_lib::int!(168) &&
            data0.len() == data1.len()
        )]
        #[inline(always)]
        pub fn shake128_absorb_final(s: &mut KeccakState, data0: &[u8], data1: &[u8]) {
            s.state
                .absorb_final::<168, 0x1fu8>(&[data0, data1], 0, data0.len());
        }

        /// Shake256 absorb `data0` and `data1` in the [`KeccakState`] `s`.
        #[inline(always)]
        #[hax_lib::requires(
            data0.len().to_int() < hax_lib::int!(136) &&
            data0.len() == data1.len()
        )]
        pub fn shake256_absorb_final(s: &mut KeccakState, data0: &[u8], data1: &[u8]) {
            s.state
                .absorb_final::<136, 0x1fu8>(&[data0, data1], 0, data0.len());
        }

        /// Squeeze 2 times the first three blocks in parallel in the
        /// [`KeccakState`] and return the output in `out0` and `out1`.
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(504) && // 3 * 168 = 504
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake128_squeeze_first_three_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
        ) {
            s.state.squeeze_first_three_blocks::<168>(out0, out1)
        }

        /// Squeeze five blocks
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(840) && // 5 * 168 = 504
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake128_squeeze_first_five_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
        ) {
            s.state.squeeze_first_five_blocks::<168>(out0, out1);
        }

        /// Squeeze block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(136) &&
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake256_squeeze_first_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_first_block::<136>(out0, out1);
        }

        /// Squeeze next block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(136) &&
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake256_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_next_block::<136>(out0, out1, 0);
        }

        /// Squeeze 2 times the next block in parallel in the
        /// [`KeccakState`] and return the output in `out0` and `out1`.
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(168) &&
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake128_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_next_block::<168>(out0, out1, 0)
        }
    }
}
