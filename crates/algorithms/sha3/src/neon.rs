use crate::generic_keccak::simd128::keccak2;

/// A SHA3 224 implementation.
#[allow(unused_variables)]
#[inline(always)]
pub fn sha224(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 28];
    keccak2::<144, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHA3 256 implementation.
#[allow(unused_variables)]
#[inline(always)]
pub fn sha256(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 32];
    keccak2::<136, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHA3 384 implementation.
#[allow(unused_variables)]
#[inline(always)]
pub fn sha384(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 48];
    keccak2::<104, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHA3 512 implementation.
#[allow(unused_variables)]
#[inline(always)]
pub fn sha512(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 64];
    keccak2::<72, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHAKE128 implementation.
#[allow(unused_variables)]
#[inline(always)]
pub fn shake128<const LEN: usize>(digest: &mut [u8; LEN], data: &[u8]) {
    let mut dummy = [0u8; LEN];
    keccak2::<168, 0x1fu8>(&[data, data], digest, &mut dummy);
}

/// A SHAKE256 implementation.
#[allow(unused_variables)]
#[inline(always)]
pub fn shake256<const LEN: usize>(digest: &mut [u8; LEN], data: &[u8]) {
    let mut dummy = [0u8; LEN];
    keccak2::<136, 0x1fu8>(&[data, data], digest, &mut dummy);
}

/// Performing 2 operations in parallel
pub mod x2 {
    use super::*;

    /// Run SHAKE256 on both inputs in parallel.
    ///
    /// Writes the two results into `out0` and `out1`
    #[allow(unused_variables)]
    #[inline(always)]
    pub fn shake256(input0: &[u8], input1: &[u8], out0: &mut [u8], out1: &mut [u8]) {
        keccak2::<136, 0x1fu8>(&[input0, input1], out0, out1);
    }

    /// An incremental API to perform 2 operations in parallel
    pub mod incremental {
        use crate::generic_keccak::KeccakState as GenericState;

        /// The Keccak state for the incremental API.
        pub struct KeccakState {
            state: GenericState<2, crate::simd::arm64::uint64x2_t>,
        }

        #[cfg(not(eurydice))]
        #[cfg_attr(hax, hax_lib::exclude)]
        impl Drop for KeccakState {
            fn drop(&mut self) {
                use libcrux_secrets::Zeroize;
                self.state.zeroize();
            }
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
        #[inline(always)]
        pub fn shake128_absorb_final(s: &mut KeccakState, data0: &[u8], data1: &[u8]) {
            s.state
                .absorb_final::<168, 0x1fu8>(&[data0, data1], 0, data0.len());
        }

        /// Shake256 absorb `data0` and `data1` in the [`KeccakState`] `s`.
        #[inline(always)]
        pub fn shake256_absorb_final(s: &mut KeccakState, data0: &[u8], data1: &[u8]) {
            s.state
                .absorb_final::<136, 0x1fu8>(&[data0, data1], 0, data0.len());
        }

        /// Squeeze 2 times the first three blocks in parallel in the
        /// [`KeccakState`] and return the output in `out0` and `out1`.
        #[inline(always)]
        pub fn shake128_squeeze_first_three_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
        ) {
            s.state.squeeze_first_three_blocks::<168>(out0, out1)
        }

        /// Squeeze five blocks
        #[inline(always)]
        pub fn shake128_squeeze_first_five_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
        ) {
            s.state.squeeze_first_five_blocks::<168>(out0, out1);
        }

        /// Squeeze block
        #[inline(always)]
        pub fn shake256_squeeze_first_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_first_block::<136>(out0, out1);
        }

        /// Squeeze next block
        #[inline(always)]
        pub fn shake256_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_next_block::<136>(out0, out1, 0);
        }

        /// Squeeze 2 times the next block in parallel in the
        /// [`KeccakState`] and return the output in `out0` and `out1`.
        #[inline(always)]
        pub fn shake128_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_next_block::<168>(out0, out1, 0)
        }
    }
}
