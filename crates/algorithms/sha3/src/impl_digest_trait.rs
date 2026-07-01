use libcrux_traits::digest::{DigestIncrementalBase, InitializeDigestState, UpdateError};

use crate::{generic_keccak::xof::KeccakXofState, *};

const SHA3_224_LEN: usize = 28;
const SHA3_256_LEN: usize = 32;
const SHA3_384_LEN: usize = 48;
const SHA3_512_LEN: usize = 64;

#[doc(hidden)]
/// Incremental hasher state
///
/// We implement the `libcrux-traits` traits `InitializeDigestState`
/// and `DigestIncrementalState` generically for any `RATE`, but
/// `libcrux_traits::digest::arrayref::DigestIncremental`,
/// i.e. `finish`, is only implemented for the supported digest
/// lengths.
pub struct HasherState<const RATE: usize, const OUTLEN: usize> {
    // We use a portable XOF state here, but AVX2 and NEON variants
    // would be possible in principle.  However, `PARALLEL_LANES = 1`
    // is necessary until `squeeze` is implemented for `PARALLEL_LANES
    // > 1`.
    inner: KeccakXofState<1, RATE, u64>,
    absorb_state: AbsorbState<OUTLEN>,
}

enum AbsorbState<const OUTLEN: usize> {
    Absorbing,
    Finished([u8; OUTLEN]),
}

impl<const RATE: usize, const OUTLEN: usize> InitializeDigestState for HasherState<RATE, OUTLEN> {
    fn new() -> Self {
        Self {
            inner: KeccakXofState::<1, RATE, u64>::new(),
            absorb_state: AbsorbState::<OUTLEN>::Absorbing,
        }
    }
}

impl<const RATE: usize, const OUTLEN: usize> DigestIncrementalBase for HasherState<RATE, OUTLEN> {
    type IncrementalState = Self;

    fn reset(state: &mut Self::IncrementalState) {
        *state = Self::IncrementalState::new();
    }

    fn update(state: &mut Self::IncrementalState, payload: &[u8]) -> Result<(), UpdateError> {
        match state.absorb_state {
            AbsorbState::Absorbing => {
                state.inner.absorb(&[payload]);
                Ok(())
            }
            AbsorbState::Finished(_) => Err(UpdateError::Unknown),
        }
    }
}

macro_rules! impl_hash_traits {
    ($type:ident, $hasher:ident, $len:expr, $rate:expr, $method:expr) => {
        #[doc = concat!("A struct that implements [`libcrux_traits::digest`] traits.")]
        #[doc = concat!("\n\n")]
        #[doc = concat!("[`",stringify!($hasher), "`] is a convenient hasher for this struct.")]
        pub struct $type;

        #[doc = concat!("A hasher for [`",stringify!($type), "`].")]
        pub type $hasher = libcrux_traits::digest::Hasher<$len, HasherState<$rate, $len>>;

        // Squeeze is only implemented for the correct digest lengths.
        impl libcrux_traits::digest::arrayref::DigestIncremental<$len>
            for HasherState<$rate, $len>
        {
            fn finish(state: &mut Self::IncrementalState, digest: &mut [u8; $len]) {
                match state.absorb_state {
                    AbsorbState::Absorbing => {
                        state.inner.absorb_final::<0x06u8>(&[&[]]);
                        state.inner.squeeze(digest);
                        state.absorb_state = AbsorbState::Finished(digest.clone());
                    }
                    AbsorbState::Finished(existing_digest) => *digest = existing_digest.clone(),
                }
            }
        }

        impl libcrux_traits::digest::arrayref::Hash<$len> for $type {
            #[inline(always)]
            fn hash(
                digest: &mut [u8; $len],
                payload: &[u8],
            ) -> Result<(), libcrux_traits::digest::arrayref::HashError> {
                if payload.len() > u32::MAX as usize {
                    return Err(libcrux_traits::digest::arrayref::HashError::InvalidPayloadLength);
                }

                $method(digest, payload);

                Ok(())
            }
        }
    };
}

impl_hash_traits!(
    Sha3_224,
    Sha3_224Hasher,
    SHA3_224_LEN,
    144,
    portable::sha224
);
impl_hash_traits!(
    Sha3_256,
    Sha3_256Hasher,
    SHA3_256_LEN,
    136,
    portable::sha256
);
impl_hash_traits!(
    Sha3_384,
    Sha3_384Hasher,
    SHA3_384_LEN,
    104,
    portable::sha384
);
impl_hash_traits!(Sha3_512, Sha3_512Hasher, SHA3_512_LEN, 72, portable::sha512);

// Implement the slice hash trait
// This is excluded for the hax extraction
#[cfg_attr(hax, hax_lib::exclude)]
mod slice {
    use super::*;

    libcrux_traits::digest::slice::impl_hash_trait!(Sha3_224 => SHA3_224_LEN);
    libcrux_traits::digest::slice::impl_hash_trait!(Sha3_256 => SHA3_256_LEN);
    libcrux_traits::digest::slice::impl_hash_trait!(Sha3_384 => SHA3_384_LEN);
    libcrux_traits::digest::slice::impl_hash_trait!(Sha3_512 => SHA3_512_LEN);
}
