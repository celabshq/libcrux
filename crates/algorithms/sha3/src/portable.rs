use crate::generic_keccak::{self, portable::keccak1};
use hax_lib;

#[cfg(hax)]
use hax_lib::int::*;

use generic_keccak::KeccakState as GenericState;

/// The Keccak state for the incremental API.
#[derive(Clone, Copy)]
pub struct KeccakState {
    state: GenericState<1, u64>,
}

/// A portable SHA3 224 implementation.
#[inline(always)]
pub fn sha224(digest: &mut [u8], data: &[u8]) {
    keccak1::<144, 0x06u8>(data, digest);
}

/// A portable SHA3 256 implementation.
#[inline(always)]
pub fn sha256(digest: &mut [u8], data: &[u8]) {
    keccak1::<136, 0x06u8>(data, digest);
}

/// A portable SHA3 384 implementation.
#[inline(always)]
pub fn sha384(digest: &mut [u8], data: &[u8]) {
    keccak1::<104, 0x06u8>(data, digest);
}

/// A portable SHA3 512 implementation.
#[inline(always)]
pub fn sha512(digest: &mut [u8], data: &[u8]) {
    keccak1::<72, 0x06u8>(data, digest);
}

/// A portable SHAKE128 implementation.
#[inline(always)]
pub fn shake128(digest: &mut [u8], data: &[u8]) {
    keccak1::<168, 0x1fu8>(data, digest);
}

/// A portable SHAKE256 implementation.
#[inline(always)]
pub fn shake256(digest: &mut [u8], data: &[u8]) {
    keccak1::<136, 0x1fu8>(data, digest);
}

/// An incremental API for SHAKE
pub mod incremental {
    use generic_keccak::xof::KeccakXofState;

    #[cfg(hax)]
    use crate::proof_utils::keccak_xof_state_inv;

    mod private {
        pub trait Sealed {}

        #[hax_lib::fstar::replace(
            "
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1__from__private: t_Sealed t_Shake128Xof = { __marker_trait_t_Sealed = () }
        "
        )]
        impl Sealed for super::Shake128Xof {}
        #[hax_lib::fstar::replace(
            "
[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl__from__private: t_Sealed t_Shake256Xof = { __marker_trait_t_Sealed = () }
        "
        )]
        impl Sealed for super::Shake256Xof {}
    }
    use super::*;

    /// SHAKE128 Xof state
    pub struct Shake128Xof {
        state: KeccakXofState<1, 168, u64>,
    }

    /// SHAKE256 Xof state
    pub struct Shake256Xof {
        state: KeccakXofState<1, 136, u64>,
    }

    /// An XOF
    pub trait Xof<const RATE: usize>: private::Sealed {
        /// Create new absorb state
        fn new() -> Self;

        /// Absorb input
        fn absorb(&mut self, input: &[u8]);

        /// Absorb final input (may be empty)
        fn absorb_final(&mut self, input: &[u8]);

        /// Squeeze output bytes
        fn squeeze(&mut self, out: &mut [u8]);
    }

    #[hax_lib::attributes]
    impl Xof<168> for Shake128Xof {
        #[hax_lib::ensures(|result|keccak_xof_state_inv(168, result.state.buf_len))]
        fn new() -> Self {
            Self {
                state: KeccakXofState::<1, 168, u64>::new(),
            }
        }

        #[hax_lib::requires(keccak_xof_state_inv(168, self.state.buf_len))]
        #[hax_lib::ensures(|_| keccak_xof_state_inv(168, future(self).state.buf_len))]
        fn absorb(&mut self, input: &[u8]) {
            self.state.absorb(&[input]);
        }

        #[hax_lib::requires(keccak_xof_state_inv(168, self.state.buf_len))]
        #[hax_lib::ensures(|_| keccak_xof_state_inv(168, future(self).state.buf_len))]
        fn absorb_final(&mut self, input: &[u8]) {
            self.state.absorb_final::<0x1fu8>(&[input]);
        }

        /// Shake128 squeeze
        #[hax_lib::requires(keccak_xof_state_inv(168, self.state.buf_len))]
        #[hax_lib::ensures(|_|
            keccak_xof_state_inv(168, self.state.buf_len) &&
            future(out).len() == out.len()
        )]
        fn squeeze(&mut self, out: &mut [u8]) {
            self.state.squeeze(out);
        }
    }

    /// Shake256 XOF in absorb state
    #[hax_lib::attributes]
    impl Xof<136> for Shake256Xof {
        /// Shake256 new state
        #[hax_lib::ensures(|result| keccak_xof_state_inv(136, result.state.buf_len))]
        fn new() -> Self {
            Self {
                state: KeccakXofState::<1, 136, u64>::new(),
            }
        }

        /// Shake256 absorb
        #[hax_lib::requires(keccak_xof_state_inv(136, self.state.buf_len))]
        #[hax_lib::ensures(|_| keccak_xof_state_inv(136, future(self).state.buf_len))]
        fn absorb(&mut self, input: &[u8]) {
            self.state.absorb(&[input]);
        }

        /// Shake256 absorb final
        #[hax_lib::requires(keccak_xof_state_inv(136, self.state.buf_len))]
        #[hax_lib::ensures(|_| keccak_xof_state_inv(136, future(self).state.buf_len))]
        fn absorb_final(&mut self, input: &[u8]) {
            self.state.absorb_final::<0x1fu8>(&[input]);
        }

        /// Shake256 squeeze
        #[hax_lib::requires(keccak_xof_state_inv(136, self.state.buf_len))]
        #[hax_lib::ensures(|_|
            keccak_xof_state_inv(136, self.state.buf_len) &&
            future(out).len() == out.len()
        )]
        fn squeeze(&mut self, out: &mut [u8]) {
            self.state.squeeze(out);
        }
    }

    /// Create a new SHAKE-128 state object.
    #[inline(always)]
    pub fn shake128_init() -> KeccakState {
        KeccakState {
            state: GenericState::<1, u64>::new(),
        }
    }

    /// Absorb
    #[inline(always)]
    #[hax_lib::requires(
        data0.len().to_int() < hax_lib::int!(168)
    )]
    pub fn shake128_absorb_final(s: &mut KeccakState, data0: &[u8]) {
        s.state
            .absorb_final::<168, 0x1fu8>(&[data0], 0, data0.len());
    }

    /// Squeeze three blocks
    #[inline(always)]
    #[hax_lib::requires(
        out0.len().to_int() >= hax_lib::int!(504) // 3 * 168 = 504
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len())]
    pub fn shake128_squeeze_first_three_blocks(s: &mut KeccakState, out0: &mut [u8]) {
        s.state.squeeze_first_three_blocks::<168>(out0);
    }

    /// Squeeze five blocks
    #[inline(always)]
    #[hax_lib::requires(
        out0.len().to_int() >= hax_lib::int!(840) // 5 * 168 = 840
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len())]
    pub fn shake128_squeeze_first_five_blocks(s: &mut KeccakState, out0: &mut [u8]) {
        s.state.squeeze_first_five_blocks::<168>(out0);
    }

    /// Squeeze another block
    #[inline(always)]
    #[hax_lib::requires(
        out0.len().to_int() >= hax_lib::int!(168)
    )]
    #[hax_lib::ensures(|_| future(out0).len() == out0.len())]
    pub fn shake128_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8]) {
        s.state.squeeze_next_block::<168>(out0, 0)
    }

    /// Create a new SHAKE-256 state object.
    #[inline(always)]
    pub fn shake256_init() -> KeccakState {
        KeccakState {
            state: GenericState::<1, u64>::new(),
        }
    }

    /// Absorb some data for SHAKE-256 for the last time
    #[inline(always)]
    #[hax_lib::requires(
        data.len().to_int() < hax_lib::int!(136)
    )]
    pub fn shake256_absorb_final(s: &mut KeccakState, data: &[u8]) {
        s.state.absorb_final::<136, 0x1fu8>(&[data], 0, data.len());
    }

    /// Squeeze the first SHAKE-256 block
    #[inline(always)]
    #[hax_lib::requires(
        out.len().to_int() >= hax_lib::int!(136)
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub fn shake256_squeeze_first_block(s: &mut KeccakState, out: &mut [u8]) {
        s.state.squeeze_first_block::<136>(out);
    }

    /// Squeeze the next SHAKE-256 block
    #[inline(always)]
    #[hax_lib::requires(
        out.len().to_int() >= hax_lib::int!(136)
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub fn shake256_squeeze_next_block(s: &mut KeccakState, out: &mut [u8]) {
        s.state.squeeze_next_block::<136>(out, 0);
    }
}
