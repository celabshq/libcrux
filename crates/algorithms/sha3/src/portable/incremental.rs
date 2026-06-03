use super::*;
use generic_keccak::xof::KeccakXofState;

#[cfg(hax)]
use crate::proof_utils::keccak_xof_state_inv;

mod cshake;
pub use cshake::{left_encode, left_encode_byte, right_encode};

mod private {
    pub trait Sealed {}

    impl Sealed for super::Shake128Xof {}
    impl Sealed for super::Shake256Xof {}
    impl Sealed for super::CShake128 {}
    impl Sealed for super::CShake256 {}
}

/// SHAKE128 Xof state
pub struct Shake128Xof {
    state: KeccakXofState<1, 168, u64>,
}

/// SHAKE256 Xof state
pub struct Shake256Xof {
    state: KeccakXofState<1, 136, u64>,
}

#[hax_lib::attributes]
/// A trait for portable, incremental CSHAKE implementations
// XXX: The names here have the `_cshake` suffix to work around an F* extraction name clash bug.
pub trait CShake<const RATE: usize>: private::Sealed {
    /// Create new absorb state
    #[requires(RATE == 136 || RATE == 168)]
    fn new_cshake(name: &[u8], customization: &[u8]) -> Self;

    /// Absorb input
    #[requires(true)]
    fn absorb_cshake(&mut self, input: &[u8]);

    #[requires(true)]
    /// Absorb final input (may be empty)
    fn absorb_final_cshake(&mut self, input: &[u8]);

    /// Squeeze output bytes
    #[requires(true)]
    fn squeeze_cshake(&mut self, out: &mut [u8]);
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

impl Xof<168> for CShake128 {
    /// CShake128 new state
    fn new() -> Self {
        Self {
            state: KeccakXofState::<1, 168, u64>::new(),
        }
    }

    /// CShake128 absorb
    fn absorb(&mut self, input: &[u8]) {
        self.state.absorb(&[input]);
    }

    /// CShake128 absorb final
    // fn absorb_finalize(&mut self, input: &[u8], out: &mut [u8]) {
    //     self.state.absorb_final::<0x4u8>(&[input]);
    //     self.state.squeeze(out);
    // }

    fn absorb_final(&mut self, input: &[u8]) {
        self.state.absorb_final::<0x4u8>(&[input]);
    }

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

impl Xof<136> for CShake256 {
    /// CShake256 new state
    fn new() -> Self {
        Self {
            state: KeccakXofState::<1, 136, u64>::new(),
        }
    }

    /// CShake256 absorb
    fn absorb(&mut self, input: &[u8]) {
        self.state.absorb(&[input]);
    }

    // /// CShake256 absorb final
    // pub fn absorb_finalize(&mut self, input: &[u8], out: &mut [u8]) {
    //     self.state.absorb_final::<0x4u8>(&[input]);
    //     self.state.squeeze(out);
    // }

    fn absorb_final(&mut self, input: &[u8]) {
        self.state.absorb_final::<0x4u8>(&[input]);
    }

    fn squeeze(&mut self, out: &mut [u8]) {
        self.state.squeeze(out);
    }
}

/// Create a new SHAKE-128 state object.
#[inline(always)]
pub fn shake128_init() -> KeccakState {
    KeccakState::init()
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

#[hax_lib::attributes]
impl<const RATE: usize> CShake<RATE> for CShakeIncremental<RATE>
where
    CShakeIncremental<RATE>: private::Sealed,
{
    #[requires(RATE == 136 || RATE == 168)]
    fn new_cshake(name: &[u8], customization: &[u8]) -> Self {
        let mut state = KeccakXofState::<1, RATE, u64>::new();

        let zeros = [0u8; RATE];
        let name_bits = name.len() << 3;
        let customization_bits = customization.len() << 3;
        let mut b = [0u8; 9];

        // Left bytepad
        state.absorb(&[&left_encode_byte(RATE as u8)]);
        // Encode name string
        let name_bits_encoding = left_encode(name_bits, &mut b);
        let name_bits_encoding_len = name_bits_encoding.len();
        state.absorb(&[name_bits_encoding]);
        state.absorb(&[name]);

        // Encode customization string
        let customization_encoding = left_encode(customization_bits, &mut b);
        let customization_encoding_len = customization_encoding.len();
        state.absorb(&[customization_encoding]);
        state.absorb(&[customization]);

        // Pad zeros
        let buffer_len = 2
            + name.len()
            + name_bits_encoding_len
            + customization_encoding_len
            + (customization.len() % RATE);
        let n_zeros = (RATE - (buffer_len % RATE)) % RATE;
        debug_assert!(n_zeros < RATE);
        state.absorb(&[&zeros[..n_zeros]]);

        Self { state }
    }

    fn absorb_cshake(&mut self, input: &[u8]) {
        self.state.absorb(&[input]);
    }

    fn absorb_final_cshake(&mut self, input: &[u8]) {
        self.state.absorb_final::<0x4u8>(&[input]);
    }

    fn squeeze_cshake(&mut self, out: &mut [u8]) {
        self.state.squeeze(out);
    }
}

/// A portable, incremental implementation of CSHAKE for a given absorption rate.
pub struct CShakeIncremental<const RATE: usize> {
    pub(crate) state: KeccakXofState<1, RATE, u64>,
}

/// A portable, incremental implementation of CSHAKE-128.
pub type CShake128 = CShakeIncremental<168>;
/// A portable, incremental implementation of CSHAKE-256.
pub type CShake256 = CShakeIncremental<136>;
