#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]
#![allow(clippy::needless_range_loop)]
#![warn(missing_docs)]
// Enable doc cfg feature for doc builds. They use nightly.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", feature = "incremental"))]
extern crate alloc;

/// Feature gating helper macros
#[macro_use]
mod cfg;

pub(crate) mod hax_utils;

// This module is declared here since otherwise, hax reports the following error:
//
// The THIR body of item
// DefId(0:986 ~ libcrux[92b3]::kem::kyber768::parameters::COEFFICIENTS_IN_RING_ELEMENT)
// was stolen.
//
// This is being tracked in https://github.com/hacspec/hacspec-v2/issues/27
pub(crate) mod constants;

/// Helpers for verification and extraction
mod helper;

mod constant_time_ops;
mod hash_functions;
mod ind_cca;
mod ind_cpa;
mod invert_ntt;
mod matrix;
mod mlkem;
mod ntt;
mod polynomial;
mod sampling;
mod serialize;
mod types;
mod utils;
mod variant;
mod vector;

#[cfg(feature = "pqcp")]
pub(crate) mod pqcp;

#[cfg(feature = "mlkem512")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlkem512")))]
pub mod mlkem512;

#[cfg(feature = "mlkem768")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlkem768")))]
pub mod mlkem768;

#[cfg(feature = "mlkem1024")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlkem1024")))]
pub mod mlkem1024;

pub use constants::SHARED_SECRET_SIZE;

pub use ind_cca::{MlKemSharedSecret, ENCAPS_SEED_SIZE, KEY_GENERATION_SEED_SIZE};

// These types all have type aliases for the different variants.
pub use types::{MlKemCiphertext, MlKemKeyPair, MlKemPrivateKey, MlKemPublicKey};

cfg_kyber! {
    #[cfg(feature = "mlkem512")]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "kyber", feature = "mlkem512"))))]
    pub mod kyber512 {
        //! Kyber 512 (NIST PQC Round 3)
        cfg_no_eurydice! {
            pub use crate::mlkem512::kyber::Kyber512;
            pub use crate::mlkem512::kyber::generate_key_pair;
            pub use crate::mlkem512::kyber::decapsulate;
            pub use crate::mlkem512::kyber::encapsulate;
            pub use crate::mlkem512::validate_public_key;
            pub use crate::mlkem512::validate_private_key;
        }
    }

    #[cfg(feature = "mlkem768")]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "kyber", feature = "mlkem768"))))]
    pub mod kyber768 {
        //! Kyber 768 (NIST PQC Round 3)
        cfg_no_eurydice! {
            pub use crate::mlkem768::kyber::Kyber768;
            pub use crate::mlkem768::kyber::generate_key_pair;
            pub use crate::mlkem768::kyber::decapsulate;
            pub use crate::mlkem768::kyber::encapsulate;
            pub use crate::mlkem768::validate_public_key;
            pub use crate::mlkem768::validate_private_key;
        }
    }

    #[cfg(feature = "mlkem1024")]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "kyber", feature = "mlkem1024"))))]
    pub mod kyber1024 {
        //! Kyber 1024 (NIST PQC Round 3)
        cfg_no_eurydice! {
            pub use crate::mlkem1024::kyber::Kyber1024;
            pub use crate::mlkem1024::kyber::generate_key_pair;
            pub use crate::mlkem1024::kyber::decapsulate;
            pub use crate::mlkem1024::kyber::encapsulate;
            pub use crate::mlkem1024::validate_public_key;
            pub use crate::mlkem1024::validate_private_key;
        }
    }
}

macro_rules! impl_kem_trait {
    ($variant:ty, $pk:ty, $sk:ty, $ct:ty) => {
        #[hax_lib::exclude]
        impl
            libcrux_traits::kem::arrayref::Kem<
                CPA_PKE_PUBLIC_KEY_SIZE,
                SECRET_KEY_SIZE,
                CPA_PKE_CIPHERTEXT_SIZE,
                SHARED_SECRET_SIZE,
                KEY_GENERATION_SEED_SIZE,
                SHARED_SECRET_SIZE,
            > for $variant
        {
            fn keygen(
                ek: &mut [u8; CPA_PKE_PUBLIC_KEY_SIZE],
                dk: &mut [u8; SECRET_KEY_SIZE],
                rand: &[u8; KEY_GENERATION_SEED_SIZE],
            ) -> Result<(), libcrux_traits::kem::owned::KeyGenError> {
                let key_pair = generate_key_pair(*rand);
                ek.copy_from_slice(key_pair.pk());
                dk.copy_from_slice(key_pair.sk());

                Ok(())
            }

            fn encaps(
                ct: &mut [u8; CPA_PKE_CIPHERTEXT_SIZE],
                ss: &mut [u8; SHARED_SECRET_SIZE],
                ek: &[u8; CPA_PKE_PUBLIC_KEY_SIZE],
                rand: &[u8; SHARED_SECRET_SIZE],
            ) -> Result<(), libcrux_traits::kem::owned::EncapsError> {
                let public_key: $pk = ek.into();

                let (ct_, ss_) = encapsulate(&public_key, *rand);
                ct.copy_from_slice(ct_.as_slice());
                ss.copy_from_slice(ss_.as_slice());

                Ok(())
            }

            fn decaps(
                ss: &mut [u8; SHARED_SECRET_SIZE],
                ct: &[u8; CPA_PKE_CIPHERTEXT_SIZE],
                dk: &[u8; SECRET_KEY_SIZE],
            ) -> Result<(), libcrux_traits::kem::owned::DecapsError> {
                let secret_key: $sk = dk.into();
                let ciphertext: $ct = ct.into();

                let ss_ = decapsulate(&secret_key, &ciphertext);

                ss.copy_from_slice(ss_.as_slice());

                Ok(())
            }
        }

    #[cfg(not(hax))]
    libcrux_traits::kem::slice::impl_trait!($variant =>
        CPA_PKE_PUBLIC_KEY_SIZE, SECRET_KEY_SIZE,
        CPA_PKE_CIPHERTEXT_SIZE, SHARED_SECRET_SIZE,
        KEY_GENERATION_SEED_SIZE, SHARED_SECRET_SIZE);
    };
}

use impl_kem_trait;
