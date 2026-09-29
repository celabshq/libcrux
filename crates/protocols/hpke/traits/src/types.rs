//! # HPKE Algorithm Identifiers
//!
//! Algorithm definitions for the [`crate::HpkeCrypto`] trait.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::error;

/// KEM Modes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u16)]
pub enum KemAlgorithm {
    /// DH KEM on P256
    DhKemP256 = 0x0010,

    /// DH KEM on P384
    DhKemP384 = 0x0011,

    /// DH KEM on P521
    DhKemP521 = 0x0012,

    /// DH KEM on secp256k1
    DhKemK256 = 0x0016,

    /// DH KEM on x25519
    DhKem25519 = 0x0020,

    /// DH KEM on x448
    DhKem448 = 0x0021,

    /// X-WING
    ///
    /// This is XWing draft 06, but uses an obsolete code point. You should use `XWingDraft06` instead.
    #[deprecated(
        since = "0.4.0",
        note = "This uses an obsolete code point, use `XWingDraft06` instead for the correct code point."
    )]
    XWingDraft06Obsolete = 0x004D,

    /// X-WING
    ///
    /// This is the X-Wing construction (ML-KEM-768 + X25519). The authoritative
    /// reference, `draft-ietf-hpke-pq`, registers this code point under the name
    /// `MLKEM768-X25519` (see its §8.2).
    ///
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05>
    /// <https://datatracker.ietf.org/doc/html/draft-connolly-cfrg-xwing-kem-06>
    XWingDraft06 = 0x647a,

    /// ML-KEM-512
    ///
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04>
    MlKem512 = 0x0040,

    /// ML-KEM-768
    ///
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04>
    MlKem768 = 0x0041,

    /// ML-KEM-1024
    ///
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04>
    MlKem1024 = 0x0042,

    /// ML-KEM-768 + P-256 hybrid KEM
    ///
    /// Defined by `draft-ietf-hpke-pq` (authoritative for the HPKE integration
    /// and code point) on top of the `MLKEM768-P256` instance of
    /// `draft-irtf-cfrg-concrete-hybrid-kems`.
    ///
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04>
    /// <https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-concrete-hybrid-kems-03>
    MlKem768P256 = 0x0050,

    /// ML-KEM-1024 + P-384 hybrid KEM
    ///
    /// Defined by `draft-ietf-hpke-pq` (authoritative for the HPKE integration
    /// and code point) on top of the `MLKEM1024-P384` instance of
    /// `draft-irtf-cfrg-concrete-hybrid-kems`.
    ///
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04>
    /// <https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-concrete-hybrid-kems-03>
    MlKem1024P384 = 0x0051,
}

impl Zeroize for KemAlgorithm {
    fn zeroize(&mut self) {
        // Nothing to do here.
    }
}

impl core::fmt::Display for KemAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::convert::TryFrom<u16> for KemAlgorithm {
    type Error = error::Error;
    fn try_from(x: u16) -> Result<KemAlgorithm, Self::Error> {
        match x {
            0x0010 => Ok(KemAlgorithm::DhKemP256),
            0x0011 => Ok(KemAlgorithm::DhKemP384),
            0x0012 => Ok(KemAlgorithm::DhKemP521),
            0x0016 => Ok(KemAlgorithm::DhKemK256),
            0x0020 => Ok(KemAlgorithm::DhKem25519),
            0x0021 => Ok(KemAlgorithm::DhKem448),
            #[allow(deprecated)]
            0x004D => Ok(KemAlgorithm::XWingDraft06Obsolete),
            0x647a => Ok(KemAlgorithm::XWingDraft06),
            0x0040 => Ok(KemAlgorithm::MlKem512),
            0x0041 => Ok(KemAlgorithm::MlKem768),
            0x0042 => Ok(KemAlgorithm::MlKem1024),
            0x0050 => Ok(KemAlgorithm::MlKem768P256),
            0x0051 => Ok(KemAlgorithm::MlKem1024P384),
            _ => Err(Self::Error::UnknownKemAlgorithm),
        }
    }
}

impl KemAlgorithm {
    /// Get the length of the private key for the KEM in bytes.
    pub const fn private_key_len(&self) -> usize {
        match self {
            KemAlgorithm::DhKemP256 => 32,
            KemAlgorithm::DhKemP384 => 48,
            KemAlgorithm::DhKemP521 => 66,
            KemAlgorithm::DhKemK256 => 32,
            KemAlgorithm::DhKem25519 => 32,
            KemAlgorithm::DhKem448 => 56,
            #[allow(deprecated)]
            KemAlgorithm::XWingDraft06 | KemAlgorithm::XWingDraft06Obsolete => 32,
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => 64,
            // Hybrid KEMs derive a key pair from a 32-byte seed
            // (`SHAKE256.LabeledDerive(ikm, "DeriveKeyPair", "", 32)`).
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => 32,
        }
    }

    /// The length of the explicit randomness a derandomized encapsulation
    /// (`HpkeCrypto::kem_encaps_derand`) needs for this KEM, or `None` if
    /// `self` doesn't use that path. DH-based KEMs derive their ephemeral
    /// randomness length from [`Self::private_key_len`] instead and don't go
    /// through `kem_encaps_derand`.
    pub const fn encaps_randomness_len(&self) -> Option<usize> {
        match self {
            KemAlgorithm::DhKemP256
            | KemAlgorithm::DhKemP384
            | KemAlgorithm::DhKemP521
            | KemAlgorithm::DhKemK256
            | KemAlgorithm::DhKem25519
            | KemAlgorithm::DhKem448 => None,
            #[allow(deprecated)]
            KemAlgorithm::XWingDraft06 | KemAlgorithm::XWingDraft06Obsolete => Some(64),
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => Some(32),
            // ML-KEM randomness (32) + the nominal group's seed
            // (`T::SEED_SIZE`, per `draft-irtf-cfrg-concrete-hybrid-kems`).
            KemAlgorithm::MlKem768P256 => Some(32 + 128),
            KemAlgorithm::MlKem1024P384 => Some(32 + 48),
        }
    }

    /// Get the length of the shared secret for the KEM in bytes.
    pub const fn shared_secret_len(&self) -> usize {
        match self {
            KemAlgorithm::DhKemP256 => 32,
            KemAlgorithm::DhKemP384 => 48,
            KemAlgorithm::DhKemP521 => 64,
            KemAlgorithm::DhKemK256 => 32,
            KemAlgorithm::DhKem25519 => 32,
            KemAlgorithm::DhKem448 => 64,
            #[allow(deprecated)]
            KemAlgorithm::XWingDraft06 | KemAlgorithm::XWingDraft06Obsolete => 32,
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => 32,
            // SHA3-256 combiner output
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => 32,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
/// AEAD types
pub enum AeadAlgorithm {
    /// AES GCM 128
    Aes128Gcm = 0x0001,

    /// AES GCM 256
    Aes256Gcm = 0x0002,

    /// ChaCha20 Poly1305
    ChaCha20Poly1305 = 0x0003,

    /// HPKE Export-only
    HpkeExport = 0xFFFF,
}

impl Zeroize for AeadAlgorithm {
    fn zeroize(&mut self) {
        // Nothing to do here.
    }
}

impl core::fmt::Display for AeadAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::convert::TryFrom<u16> for AeadAlgorithm {
    type Error = error::Error;
    fn try_from(x: u16) -> Result<AeadAlgorithm, Self::Error> {
        match x {
            0x0001 => Ok(AeadAlgorithm::Aes128Gcm),
            0x0002 => Ok(AeadAlgorithm::Aes256Gcm),
            0x0003 => Ok(AeadAlgorithm::ChaCha20Poly1305),
            0xFFFF => Ok(AeadAlgorithm::HpkeExport),
            _ => Err(Self::Error::UnknownAeadAlgorithm),
        }
    }
}

impl AeadAlgorithm {
    /// Get the tag size of the [`AeadAlgorithm`] in bytes.
    ///
    /// Note that the function returns `0` for unknown lengths such as the
    /// [`AeadAlgorithm::HpkeExport`] type.
    pub const fn tag_length(&self) -> usize {
        match self {
            AeadAlgorithm::Aes128Gcm => 16,
            AeadAlgorithm::Aes256Gcm => 16,
            AeadAlgorithm::ChaCha20Poly1305 => 16,
            AeadAlgorithm::HpkeExport => 0,
        }
    }

    /// Get the key size of the [`AeadAlgorithm`] in bytes.
    ///
    /// Note that the function returns `0` for unknown lengths such as the
    /// [`AeadAlgorithm::HpkeExport`] type.
    pub const fn key_length(&self) -> usize {
        match self {
            AeadAlgorithm::Aes128Gcm => 16,
            AeadAlgorithm::Aes256Gcm => 32,
            AeadAlgorithm::ChaCha20Poly1305 => 32,
            AeadAlgorithm::HpkeExport => 0,
        }
    }

    /// Get the nonce size of the [`AeadAlgorithm`] in bytes.
    ///
    /// Note that the function returns `0` for unknown lengths such as the
    /// [`AeadAlgorithm::HpkeExport`] type.
    ///
    /// Further note that while the AEAD mechanisms generally allow for different
    /// nonce lengths, this HPKE implementation expects the most common nonce size.
    pub const fn nonce_length(&self) -> usize {
        match self {
            AeadAlgorithm::Aes128Gcm => 12,
            AeadAlgorithm::Aes256Gcm => 12,
            AeadAlgorithm::ChaCha20Poly1305 => 12,
            AeadAlgorithm::HpkeExport => 0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
/// KDF types
///
/// Note that Shake types are not standardized yet and may change in future.
pub enum KdfAlgorithm {
    /// HKDF SHA 256
    HkdfSha256 = 0x0001,

    /// HKDF SHA 384
    HkdfSha384 = 0x0002,

    /// HKDF SHA 512
    HkdfSha512 = 0x0003,

    /// SHAKE128 single-stage KDF
    ///
    /// Used by the post-quantum HPKE ciphersuites.
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05>
    Shake128 = 0x0010,

    /// SHAKE256 single-stage KDF
    ///
    /// Used by the post-quantum HPKE ciphersuites.
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05>
    Shake256 = 0x0011,

    /// SHAKE128 single-stage KDF
    /// Not supported by any official provider yet.
    ///
    /// Used by the post-quantum HPKE ciphersuites.
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05>
    TurboShake128 = 0x0012,

    /// SHAKE256 single-stage KDF
    /// Not supported by any official provider yet.
    ///
    /// Used by the post-quantum HPKE ciphersuites.
    /// <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05>
    TurboShake256 = 0x0013,
}

/// A single-stage (XOF) KDF, per `draft-ietf-hpke-pq`.
///
/// Single-stage KDFs offer a single `Derive(ikm, L)` operation (via
/// [`HpkeCrypto::kdf_derive`](crate::HpkeCrypto::kdf_derive)) and a different
/// key-schedule shape than the two-stage HKDF KDFs.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SingleStageKdfAlgorithm {
    /// SHAKE128 single-stage KDF (`KdfAlgorithm::Shake128`).
    Shake128,

    /// SHAKE256 single-stage KDF (`KdfAlgorithm::Shake256`).
    Shake256,

    /// TurboSHAKE128 single-stage KDF (`KdfAlgorithm::TurboShake128`).
    TurboShake128,

    /// TurboSHAKE256 single-stage KDF (`KdfAlgorithm::TurboShake256`).
    TurboShake256,
}

/// A two-stage (extract-then-expand) HKDF-based KDF.
///
/// Two-stage KDFs offer separate `Extract` and `Expand` operations (via
/// [`HpkeCrypto::kdf_extract`](crate::HpkeCrypto::kdf_extract) /
/// [`HpkeCrypto::kdf_expand`](crate::HpkeCrypto::kdf_expand)).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TwoStageKdfAlgorithm {
    /// HKDF SHA-256 (`KdfAlgorithm::HkdfSha256`).
    HkdfSha256,

    /// HKDF SHA-384 (`KdfAlgorithm::HkdfSha384`).
    HkdfSha384,

    /// HKDF SHA-512 (`KdfAlgorithm::HkdfSha512`).
    HkdfSha512,
}

/// The two-stage view of a [`KdfAlgorithm`]. Errors on the single-stage
/// (SHAKE) and TurboSHAKE identifiers.
impl core::convert::TryFrom<KdfAlgorithm> for TwoStageKdfAlgorithm {
    type Error = error::Error;
    fn try_from(alg: KdfAlgorithm) -> Result<Self, Self::Error> {
        match alg {
            KdfAlgorithm::HkdfSha256 => Ok(Self::HkdfSha256),
            KdfAlgorithm::HkdfSha384 => Ok(Self::HkdfSha384),
            KdfAlgorithm::HkdfSha512 => Ok(Self::HkdfSha512),
            _ => Err(error::Error::UnknownKdfAlgorithm),
        }
    }
}

/// The single-stage view of a [`KdfAlgorithm`]. Errors on the two-stage (HKDF)
/// identifiers and on the TurboSHAKE variants, which no provider derives yet.
impl core::convert::TryFrom<KdfAlgorithm> for SingleStageKdfAlgorithm {
    type Error = error::Error;
    fn try_from(alg: KdfAlgorithm) -> Result<Self, Self::Error> {
        match alg {
            KdfAlgorithm::Shake128 => Ok(Self::Shake128),
            KdfAlgorithm::Shake256 => Ok(Self::Shake256),
            KdfAlgorithm::TurboShake128 => Ok(Self::TurboShake128),
            KdfAlgorithm::TurboShake256 => Ok(Self::TurboShake256),
            _ => Err(error::Error::UnknownKdfAlgorithm),
        }
    }
}

impl Zeroize for KdfAlgorithm {
    fn zeroize(&mut self) {
        // Nothing to do here.
    }
}

impl core::fmt::Display for KdfAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::convert::TryFrom<u16> for KdfAlgorithm {
    type Error = error::Error;
    fn try_from(x: u16) -> Result<KdfAlgorithm, Self::Error> {
        match x {
            0x0001 => Ok(KdfAlgorithm::HkdfSha256),
            0x0002 => Ok(KdfAlgorithm::HkdfSha384),
            0x0003 => Ok(KdfAlgorithm::HkdfSha512),
            0x0010 => Ok(KdfAlgorithm::Shake128),
            0x0011 => Ok(KdfAlgorithm::Shake256),
            _ => Err(Self::Error::UnknownKdfAlgorithm),
        }
    }
}

impl From<KemAlgorithm> for KdfAlgorithm {
    fn from(kem: KemAlgorithm) -> Self {
        match kem {
            KemAlgorithm::DhKemP256 => KdfAlgorithm::HkdfSha256,
            KemAlgorithm::DhKemP384 => KdfAlgorithm::HkdfSha384,
            KemAlgorithm::DhKemP521 => KdfAlgorithm::HkdfSha512,
            KemAlgorithm::DhKemK256 => KdfAlgorithm::HkdfSha256,
            KemAlgorithm::DhKem25519 => KdfAlgorithm::HkdfSha256,
            KemAlgorithm::DhKem448 => KdfAlgorithm::HkdfSha512,
            #[allow(deprecated)]
            KemAlgorithm::XWingDraft06 | KemAlgorithm::XWingDraft06Obsolete => {
                KdfAlgorithm::HkdfSha512
            }
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => {
                KdfAlgorithm::HkdfSha256
            }
            // Post-quantum hybrid KEMs default to the SHAKE256 KDF, per
            // draft-ietf-hpke-pq. Note that callers construct HPKE with an
            // explicit `kdf_id`, so this mapping is only a default.
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => KdfAlgorithm::Shake256,
        }
    }
}
