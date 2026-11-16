#![allow(non_snake_case)]

/// Abstraction and platform multiplexing for SHAKE 256
pub(crate) mod shake256 {
    pub(crate) const BLOCK_SIZE: usize = 136;

    /// An ML-DSA specific Xof trait
    /// This trait is not actually a full Xof implementation but opererates only
    /// on multiple of blocks. The only real Xof API for SHAKE256 is [`Xof`].
    pub(crate) trait DsaXof {
        fn shake256<const OUTPUT_LENGTH: usize>(input: &[u8], out: &mut [u8; OUTPUT_LENGTH]);
        fn init_absorb_final(input: &[u8]) -> Self;
        // TODO: There should only be a `squeeze_block`
        fn squeeze_first_block(&mut self) -> [u8; BLOCK_SIZE];
        fn squeeze_next_block(&mut self) -> [u8; BLOCK_SIZE];
    }

    pub(crate) trait XofX4 {
        fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self;
        fn squeeze_first_block_x4(
            &mut self,
        ) -> (
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
        );
        fn squeeze_next_block_x4(
            &mut self,
        ) -> (
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
        );
        fn shake256_x4<const OUT_LEN: usize>(
            input0: &[u8],
            input1: &[u8],
            input2: &[u8],
            input3: &[u8],
            out0: &mut [u8; OUT_LEN],
            out1: &mut [u8; OUT_LEN],
            out2: &mut [u8; OUT_LEN],
            out3: &mut [u8; OUT_LEN],
        );
    }

    /// A generic Xof trait
    pub(crate) trait Xof {
        /// Initialize the state
        fn init() -> Self;

        /// Absorb
        fn absorb(&mut self, input: &[u8]);

        /// Absorb final input
        fn absorb_final(&mut self, input: &[u8]);

        /// Squeeze output bytes
        fn squeeze(&mut self, out: &mut [u8]);
    }
}

/// Abstraction and platform multiplexing for SHAKE 128
pub(crate) mod shake128 {
    pub(crate) const BLOCK_SIZE: usize = 168;
    pub(crate) const FIVE_BLOCKS_SIZE: usize = BLOCK_SIZE * 5;

    pub(crate) trait Xof {
        fn shake128(input: &[u8], out: &mut [u8]);
    }

    /// When sampling matrix A we always want to do 4 absorb/squeeze calls in
    /// parallel.
    pub(crate) trait XofX4 {
        fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self;
        fn squeeze_first_five_blocks(
            &mut self,
            out0: &mut [u8; FIVE_BLOCKS_SIZE],
            out1: &mut [u8; FIVE_BLOCKS_SIZE],
            out2: &mut [u8; FIVE_BLOCKS_SIZE],
            out3: &mut [u8; FIVE_BLOCKS_SIZE],
        );
        fn squeeze_next_block(
            &mut self,
        ) -> (
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
            [u8; BLOCK_SIZE],
        );
    }
}

/// A portable implementation of [`shake128::Xof`] and [`shake256::Xof`].
pub(crate) mod portable {
    use super::{shake128, shake256};
    use libcrux_sha3::portable::{
        incremental::{self, Xof},
        KeccakState,
    };

    /// Portable SHAKE 128 x4 state.
    ///
    /// We're using a portable implementation so this is actually sequential.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake128X4 {
        state0: KeccakState,
        state1: KeccakState,
        state2: KeccakState,
        state3: KeccakState,
    }

    #[inline(always)]
    fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Shake128X4 {
        let mut state0 = incremental::shake128_init();
        incremental::shake128_absorb_final(&mut state0, input0);

        let mut state1 = incremental::shake128_init();
        incremental::shake128_absorb_final(&mut state1, input1);

        let mut state2 = incremental::shake128_init();
        incremental::shake128_absorb_final(&mut state2, input2);

        let mut state3 = incremental::shake128_init();
        incremental::shake128_absorb_final(&mut state3, input3);

        Shake128X4 {
            state0,
            state1,
            state2,
            state3,
        }
    }

    #[inline(always)]
    fn squeeze_first_five_blocks(
        state: &mut Shake128X4,
        out0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    ) {
        incremental::shake128_squeeze_first_five_blocks(&mut state.state0, out0);
        incremental::shake128_squeeze_first_five_blocks(&mut state.state1, out1);
        incremental::shake128_squeeze_first_five_blocks(&mut state.state2, out2);
        incremental::shake128_squeeze_first_five_blocks(&mut state.state3, out3);
    }

    #[inline(always)]
    fn squeeze_next_block(
        state: &mut Shake128X4,
    ) -> (
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake128::BLOCK_SIZE];
        incremental::shake128_squeeze_next_block(&mut state.state0, &mut out0);
        let mut out1 = [0u8; shake128::BLOCK_SIZE];
        incremental::shake128_squeeze_next_block(&mut state.state1, &mut out1);
        let mut out2 = [0u8; shake128::BLOCK_SIZE];
        incremental::shake128_squeeze_next_block(&mut state.state2, &mut out2);
        let mut out3 = [0u8; shake128::BLOCK_SIZE];
        incremental::shake128_squeeze_next_block(&mut state.state3, &mut out3);

        (out0, out1, out2, out3)
    }

    impl shake128::XofX4 for Shake128X4 {
        #[inline(always)]
        fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self {
            init_absorb(input0, input1, input2, input3)
        }

        #[inline(always)]
        fn squeeze_first_five_blocks(
            &mut self,
            out0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        ) {
            squeeze_first_five_blocks(self, out0, out1, out2, out3);
        }

        #[inline(always)]
        fn squeeze_next_block(
            &mut self,
        ) -> (
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
        ) {
            squeeze_next_block(self)
        }
    }

    /// Portable SHAKE 128 state
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake128 {}

    #[inline(always)]
    fn shake128(input: &[u8], out: &mut [u8]) {
        libcrux_sha3::portable::shake128(out, input);
    }

    impl shake128::Xof for Shake128 {
        #[inline(always)]
        fn shake128(input: &[u8], out: &mut [u8]) {
            shake128(input, out);
        }
    }

    /// Portable SHAKE 256 state
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake256 {
        state: KeccakState,
    }

    #[inline(always)]
    fn shake256<const OUTPUT_LENGTH: usize>(input: &[u8], out: &mut [u8; OUTPUT_LENGTH]) {
        libcrux_sha3::portable::shake256(out, input);
    }

    #[inline(always)]
    fn init_absorb_final_shake256(input: &[u8]) -> Shake256 {
        let mut state = incremental::shake256_init();
        incremental::shake256_absorb_final(&mut state, input);
        Shake256 { state }
    }

    #[inline(always)]
    fn squeeze_first_block_shake256(state: &mut Shake256) -> [u8; shake256::BLOCK_SIZE] {
        let mut out = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_first_block(&mut state.state, &mut out);
        out
    }

    #[inline(always)]
    fn squeeze_next_block_shake256(state: &mut Shake256) -> [u8; shake256::BLOCK_SIZE] {
        let mut out = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_next_block(&mut state.state, &mut out);
        out
    }

    impl shake256::DsaXof for Shake256 {
        #[inline(always)]
        fn shake256<const OUTPUT_LENGTH: usize>(input: &[u8], out: &mut [u8; OUTPUT_LENGTH]) {
            shake256(input, out);
        }

        #[inline(always)]
        fn init_absorb_final(input: &[u8]) -> Self {
            init_absorb_final_shake256(input)
        }

        #[inline(always)]
        fn squeeze_first_block(&mut self) -> [u8; shake256::BLOCK_SIZE] {
            squeeze_first_block_shake256(self)
        }

        #[inline(always)]
        fn squeeze_next_block(&mut self) -> [u8; shake256::BLOCK_SIZE] {
            squeeze_next_block_shake256(self)
        }
    }

    /// Portable SHAKE 256 x4 state.
    ///
    /// We're using a portable implementation so this is actually sequential.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake256X4 {
        state0: KeccakState,
        state1: KeccakState,
        state2: KeccakState,
        state3: KeccakState,
    }

    #[inline(always)]
    fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Shake256X4 {
        let mut state0 = incremental::shake256_init();
        incremental::shake256_absorb_final(&mut state0, input0);

        let mut state1 = incremental::shake256_init();
        incremental::shake256_absorb_final(&mut state1, input1);

        let mut state2 = incremental::shake256_init();
        incremental::shake256_absorb_final(&mut state2, input2);

        let mut state3 = incremental::shake256_init();
        incremental::shake256_absorb_final(&mut state3, input3);

        Shake256X4 {
            state0,
            state1,
            state2,
            state3,
        }
    }

    #[inline(always)]
    fn squeeze_first_block_x4(
        state: &mut Shake256X4,
    ) -> (
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_first_block(&mut state.state0, &mut out0);
        let mut out1 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_first_block(&mut state.state1, &mut out1);
        let mut out2 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_first_block(&mut state.state2, &mut out2);
        let mut out3 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_first_block(&mut state.state3, &mut out3);

        (out0, out1, out2, out3)
    }

    #[inline(always)]
    fn squeeze_next_block_x4(
        state: &mut Shake256X4,
    ) -> (
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_next_block(&mut state.state0, &mut out0);
        let mut out1 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_next_block(&mut state.state1, &mut out1);
        let mut out2 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_next_block(&mut state.state2, &mut out2);
        let mut out3 = [0u8; shake256::BLOCK_SIZE];
        incremental::shake256_squeeze_next_block(&mut state.state3, &mut out3);

        (out0, out1, out2, out3)
    }

    impl shake256::XofX4 for Shake256X4 {
        #[inline(always)]
        fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self {
            init_absorb_x4(input0, input1, input2, input3)
        }

        #[inline(always)]
        fn squeeze_first_block_x4(
            &mut self,
        ) -> (
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
        ) {
            squeeze_first_block_x4(self)
        }

        #[inline(always)]
        fn squeeze_next_block_x4(
            &mut self,
        ) -> (
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
        ) {
            squeeze_next_block_x4(self)
        }

        #[inline(always)]
        fn shake256_x4<const OUT_LEN: usize>(
            input0: &[u8],
            input1: &[u8],
            input2: &[u8],
            input3: &[u8],
            out0: &mut [u8; OUT_LEN],
            out1: &mut [u8; OUT_LEN],
            out2: &mut [u8; OUT_LEN],
            out3: &mut [u8; OUT_LEN],
        ) {
            shake256(input0, out0);
            shake256(input1, out1);
            shake256(input2, out2);
            shake256(input3, out3);
        }
    }

    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake256Xof {
        state: incremental::Shake256Xof,
    }

    impl shake256::Xof for Shake256Xof {
        fn init() -> Self {
            Shake256Xof {
                state: incremental::Shake256Xof::new(),
            }
        }

        fn absorb(&mut self, input: &[u8]) {
            self.state.absorb(input);
        }

        fn absorb_final(&mut self, input: &[u8]) {
            self.state.absorb_final(input);
        }

        fn squeeze(&mut self, out: &mut [u8]) {
            self.state.squeeze(out)
        }
    }
}

/// A SIMD256 implementation of [`shake128::XofX4`] and [`shake256::Xof`] for AVX2.
#[cfg(feature = "simd256")]
pub(crate) mod simd256 {

    use super::{shake128, shake256};
    use libcrux_sha3::avx2::x4;

    /// AVX2 SHAKE 128 state
    ///
    /// This only implements the XofX4 API. For the single Xof, the portable
    /// version is used.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake128x4 {
        state: x4::incremental::KeccakState,
    }

    /// Init the state and absorb 4 blocks in parallel.
    #[inline(always)]
    fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Shake128x4 {
        let mut state = x4::incremental::init();
        x4::incremental::shake128_absorb_final(&mut state, input0, input1, input2, input3);
        Shake128x4 { state }
    }

    #[inline(always)]
    fn squeeze_first_five_blocks(
        state: &mut Shake128x4,
        out0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    ) {
        x4::incremental::shake128_squeeze_first_five_blocks(
            &mut state.state,
            out0,
            out1,
            out2,
            out3,
        );
    }

    #[inline(always)]
    fn squeeze_next_block(
        state: &mut Shake128x4,
    ) -> (
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake128::BLOCK_SIZE];
        let mut out1 = [0u8; shake128::BLOCK_SIZE];
        let mut out2 = [0u8; shake128::BLOCK_SIZE];
        let mut out3 = [0u8; shake128::BLOCK_SIZE];
        x4::incremental::shake128_squeeze_next_block(
            &mut state.state,
            &mut out0,
            &mut out1,
            &mut out2,
            &mut out3,
        );

        (out0, out1, out2, out3)
    }

    impl shake128::XofX4 for Shake128x4 {
        /// Init the state and absorb 4 blocks in parallel.
        #[inline(always)]
        fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self {
            init_absorb(input0, input1, input2, input3)
        }

        #[inline(always)]
        fn squeeze_first_five_blocks(
            &mut self,
            out0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        ) {
            squeeze_first_five_blocks(self, out0, out1, out2, out3);
        }

        #[inline(always)]
        fn squeeze_next_block(
            &mut self,
        ) -> (
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
        ) {
            squeeze_next_block(self)
        }
    }

    /// AVX2 SHAKE 256 state
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake256 {
        state: libcrux_sha3::portable::KeccakState,
    }

    #[inline(always)]
    fn shake256<const OUTPUT_LENGTH: usize>(input: &[u8], out: &mut [u8; OUTPUT_LENGTH]) {
        libcrux_sha3::portable::shake256(out, input);
    }

    #[inline(always)]
    fn init_absorb_final_shake256(input: &[u8]) -> Shake256 {
        let mut state = libcrux_sha3::portable::incremental::shake256_init();
        libcrux_sha3::portable::incremental::shake256_absorb_final(&mut state, input);

        Shake256 { state }
    }

    #[inline(always)]
    fn squeeze_first_block_shake256(state: &mut Shake256) -> [u8; shake256::BLOCK_SIZE] {
        let mut out = [0u8; shake256::BLOCK_SIZE];
        libcrux_sha3::portable::incremental::shake256_squeeze_first_block(
            &mut state.state,
            &mut out,
        );
        out
    }

    #[inline(always)]
    fn squeeze_next_block_shake256(state: &mut Shake256) -> [u8; shake256::BLOCK_SIZE] {
        let mut out = [0u8; shake256::BLOCK_SIZE];
        libcrux_sha3::portable::incremental::shake256_squeeze_next_block(
            &mut state.state,
            &mut out,
        );
        out
    }

    impl shake256::DsaXof for Shake256 {
        #[inline(always)]
        fn shake256<const OUTPUT_LENGTH: usize>(input: &[u8], out: &mut [u8; OUTPUT_LENGTH]) {
            shake256(input, out)
        }

        #[inline(always)]
        fn init_absorb_final(input: &[u8]) -> Self {
            init_absorb_final_shake256(input)
        }

        #[inline(always)]
        fn squeeze_first_block(&mut self) -> [u8; shake256::BLOCK_SIZE] {
            squeeze_first_block_shake256(self)
        }

        #[inline(always)]
        fn squeeze_next_block(&mut self) -> [u8; shake256::BLOCK_SIZE] {
            squeeze_next_block_shake256(self)
        }
    }

    /// AVX2 SHAKE 256 x4 state.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake256x4 {
        state: x4::incremental::KeccakState,
    }

    #[inline(always)]
    fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Shake256x4 {
        let mut state = x4::incremental::init();
        x4::incremental::shake256_absorb_final(&mut state, input0, input1, input2, input3);
        Shake256x4 { state }
    }

    #[inline(always)]
    fn squeeze_first_block_x4(
        state: &mut Shake256x4,
    ) -> (
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake256::BLOCK_SIZE];
        let mut out1 = [0u8; shake256::BLOCK_SIZE];
        let mut out2 = [0u8; shake256::BLOCK_SIZE];
        let mut out3 = [0u8; shake256::BLOCK_SIZE];
        x4::incremental::shake256_squeeze_first_block(
            &mut state.state,
            &mut out0,
            &mut out1,
            &mut out2,
            &mut out3,
        );

        (out0, out1, out2, out3)
    }

    #[inline(always)]
    fn squeeze_next_block_x4(
        state: &mut Shake256x4,
    ) -> (
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake256::BLOCK_SIZE];
        let mut out1 = [0u8; shake256::BLOCK_SIZE];
        let mut out2 = [0u8; shake256::BLOCK_SIZE];
        let mut out3 = [0u8; shake256::BLOCK_SIZE];
        x4::incremental::shake256_squeeze_next_block(
            &mut state.state,
            &mut out0,
            &mut out1,
            &mut out2,
            &mut out3,
        );

        (out0, out1, out2, out3)
    }

    #[inline(always)]
    fn shake256_x4<const OUT_LEN: usize>(
        input0: &[u8],
        input1: &[u8],
        input2: &[u8],
        input3: &[u8],
        out0: &mut [u8; OUT_LEN],
        out1: &mut [u8; OUT_LEN],
        out2: &mut [u8; OUT_LEN],
        out3: &mut [u8; OUT_LEN],
    ) {
        x4::shake256(input0, input1, input2, input3, out0, out1, out2, out3);
    }

    impl shake256::XofX4 for Shake256x4 {
        #[inline(always)]
        fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self {
            init_absorb_x4(input0, input1, input2, input3)
        }

        #[inline(always)]
        fn squeeze_first_block_x4(
            &mut self,
        ) -> (
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
        ) {
            squeeze_first_block_x4(self)
        }

        #[inline(always)]
        fn squeeze_next_block_x4(
            &mut self,
        ) -> (
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
        ) {
            squeeze_next_block_x4(self)
        }

        #[inline(always)]
        fn shake256_x4<const OUT_LEN: usize>(
            input0: &[u8],
            input1: &[u8],
            input2: &[u8],
            input3: &[u8],
            out0: &mut [u8; OUT_LEN],
            out1: &mut [u8; OUT_LEN],
            out2: &mut [u8; OUT_LEN],
            out3: &mut [u8; OUT_LEN],
        ) {
            shake256_x4(input0, input1, input2, input3, out0, out1, out2, out3);
        }
    }
}

/// A SIMD256 implementation of [`shake128::Xof`] and [`shake256::Xof`] for Neon.
#[cfg(feature = "simd128")]
pub(crate) mod neon {

    use super::{shake128, shake256};
    use libcrux_sha3::neon::x2;
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) type KeccakState = x2::incremental::KeccakState;

    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake128x4 {
        state: [KeccakState; 2],
    }

    /// Init the state and absorb 4 blocks in parallel.
    fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Shake128x4 {
        let mut state = [x2::incremental::init(), x2::incremental::init()];
        x2::incremental::shake128_absorb_final(&mut state[0], &input0, &input1);
        x2::incremental::shake128_absorb_final(&mut state[1], &input2, &input3);
        Shake128x4 { state }
    }

    fn squeeze_first_five_blocks(
        state: &mut Shake128x4,
        out0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        out3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
    ) {
        x2::incremental::shake128_squeeze_first_five_blocks(&mut state.state[0], out0, out1);
        x2::incremental::shake128_squeeze_first_five_blocks(&mut state.state[1], out2, out3);
    }

    fn squeeze_next_block(
        state: &mut Shake128x4,
    ) -> (
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
        [u8; shake128::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake128::BLOCK_SIZE];
        let mut out1 = [0u8; shake128::BLOCK_SIZE];
        let mut out2 = [0u8; shake128::BLOCK_SIZE];
        let mut out3 = [0u8; shake128::BLOCK_SIZE];
        x2::incremental::shake128_squeeze_next_block(&mut state.state[0], &mut out0, &mut out1);
        x2::incremental::shake128_squeeze_next_block(&mut state.state[1], &mut out2, &mut out3);

        (out0, out1, out2, out3)
    }

    impl shake128::XofX4 for Shake128x4 {
        /// Init the state and absorb 4 blocks in parallel.
        fn init_absorb(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self {
            init_absorb(input0, input1, input2, input3)
        }

        fn squeeze_first_five_blocks(
            &mut self,
            out0: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out1: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out2: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
            out3: &mut [u8; shake128::FIVE_BLOCKS_SIZE],
        ) {
            squeeze_first_five_blocks(self, out0, out1, out2, out3);
        }

        fn squeeze_next_block(
            &mut self,
        ) -> (
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
            [u8; shake128::BLOCK_SIZE],
        ) {
            squeeze_next_block(self)
        }
    }

    /// Neon SHAKE 256 x4 state
    #[cfg_attr(hax, hax_lib::opaque)]
    pub(crate) struct Shake256x4 {
        state: [KeccakState; 2],
    }

    fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Shake256x4 {
        let mut state = [x2::incremental::init(), x2::incremental::init()];
        x2::incremental::shake256_absorb_final(&mut state[0], &input0, &input1);
        x2::incremental::shake256_absorb_final(&mut state[1], &input2, &input3);
        Shake256x4 { state }
    }

    fn squeeze_first_block_x4(
        state: &mut Shake256x4,
    ) -> (
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake256::BLOCK_SIZE];
        let mut out1 = [0u8; shake256::BLOCK_SIZE];
        let mut out2 = [0u8; shake256::BLOCK_SIZE];
        let mut out3 = [0u8; shake256::BLOCK_SIZE];
        x2::incremental::shake256_squeeze_first_block(&mut state.state[0], &mut out0, &mut out1);
        x2::incremental::shake256_squeeze_first_block(&mut state.state[1], &mut out2, &mut out3);

        (out0, out1, out2, out3)
    }

    fn squeeze_next_block_x4(
        state: &mut Shake256x4,
    ) -> (
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
        [u8; shake256::BLOCK_SIZE],
    ) {
        let mut out0 = [0u8; shake256::BLOCK_SIZE];
        let mut out1 = [0u8; shake256::BLOCK_SIZE];
        let mut out2 = [0u8; shake256::BLOCK_SIZE];
        let mut out3 = [0u8; shake256::BLOCK_SIZE];
        x2::incremental::shake256_squeeze_next_block(&mut state.state[0], &mut out0, &mut out1);
        x2::incremental::shake256_squeeze_next_block(&mut state.state[1], &mut out2, &mut out3);

        (out0, out1, out2, out3)
    }

    fn shake256_x4<const OUT_LEN: usize>(
        input0: &[u8],
        input1: &[u8],
        input2: &[u8],
        input3: &[u8],
        out0: &mut [u8; OUT_LEN],
        out1: &mut [u8; OUT_LEN],
        out2: &mut [u8; OUT_LEN],
        out3: &mut [u8; OUT_LEN],
    ) {
        x2::shake256(input0, input1, out0, out1);
        x2::shake256(input2, input3, out2, out3);
    }

    impl shake256::XofX4 for Shake256x4 {
        fn init_absorb_x4(input0: &[u8], input1: &[u8], input2: &[u8], input3: &[u8]) -> Self {
            init_absorb_x4(input0, input1, input2, input3)
        }

        fn squeeze_first_block_x4(
            &mut self,
        ) -> (
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
        ) {
            squeeze_first_block_x4(self)
        }

        fn squeeze_next_block_x4(
            &mut self,
        ) -> (
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
            [u8; shake256::BLOCK_SIZE],
        ) {
            squeeze_next_block_x4(self)
        }

        fn shake256_x4<const OUT_LEN: usize>(
            input0: &[u8],
            input1: &[u8],
            input2: &[u8],
            input3: &[u8],
            out0: &mut [u8; OUT_LEN],
            out1: &mut [u8; OUT_LEN],
            out2: &mut [u8; OUT_LEN],
            out3: &mut [u8; OUT_LEN],
        ) {
            shake256_x4(input0, input1, input2, input3, out0, out1, out2, out3);
        }
    }
}
