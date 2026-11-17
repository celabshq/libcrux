#![no_std]

extern crate alloc;

#[cfg(not(feature = "expose-hacl"))]
mod hacl {
    pub(crate) mod rsapss;

    use libcrux_hacl_rs::streaming_types;
    use libcrux_sha2::hacl as hash_sha2;
}

/// The hacl-rs code for RSA signatures
#[cfg(feature = "expose-hacl")]
pub mod hacl {
    /// The RSA-PSS signature code.
    pub mod rsapss;

    use libcrux_hacl_rs::streaming_types;
    use libcrux_sha2::hacl as hash_sha2;
}

/// Entry points for the valgrind constant-time tests in `ctgrind-test`.
#[cfg(valgrind_ct_test)]
#[doc(hidden)]
pub mod ct_test {
    /// The private key validation of [`crate::sign`], without the branch on its result.
    #[inline(never)]
    pub fn load_skey(
        mod_bits: u32,
        e_bits: u32,
        d_bits: u32,
        n: &[u8],
        e: &[u8],
        d: &[u8],
        skey: &mut [u64],
    ) -> bool {
        crate::hacl::rsapss::load_skey(mod_bits, e_bits, d_bits, n, e, d, skey)
    }
}

/// The hash algorithm used for signing or verifying.
#[derive(Clone, Copy, Debug)]
pub enum DigestAlgorithm {
    /// The SHA256 hash algorithm
    Sha2_256,

    /// The SHA384 hash algorithm
    Sha2_384,

    /// The SHA512 hash algorithm
    Sha2_512,
}

impl DigestAlgorithm {
    // using u8 so it can be safely coerced into any uint type
    const fn hash_len(&self) -> u8 {
        match self {
            DigestAlgorithm::Sha2_256 => 32,
            DigestAlgorithm::Sha2_384 => 48,
            DigestAlgorithm::Sha2_512 => 64,
        }
    }
}

/// Represents errors that occurred during signing or verifying.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Indicates that the salt is too large.
    SaltTooLarge,

    /// Indicates that the message is too large.
    MessageTooLarge,

    /// Indicates that the verification of a signature failed.
    VerificationFailed,

    /// Indicates that signing a message failed.
    SigningFailed,

    /// The lengths of the public and private parts of the key do not match
    KeyLengthMismatch,

    /// The length of the provided key is invalid
    InvalidKeyLength,

    /// The length of the provided signature is invalid
    InvalidSignatureLength,
}

impl alloc::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Error::SaltTooLarge => "Indicates that the salt is too large.",
            Error::MessageTooLarge => "Indicates that the message is too large.",
            Error::VerificationFailed => "Indicates that the verification of a signature failed.",
            Error::SigningFailed => "Indicates that signing a message failed.",
            Error::KeyLengthMismatch => {
                "The lengths of the public and private parts of the key do not match"
            }
            Error::InvalidKeyLength => "The length of the provided key is invalid",
            Error::InvalidSignatureLength => "The length of the provided signature is invalid",
        };

        f.write_str(text)
    }
}

impl core::error::Error for Error {}

mod impl_hacl;

pub use impl_hacl::*;
