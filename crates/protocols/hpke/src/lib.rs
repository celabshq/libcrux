#![doc = include_str!("../Readme.md")]
//! # Examples
//!
//! Oneshot HPKE encryption.
//!
//! ```
//! use hpke_rs::{*, hpke_types::*};
//! use hpke_rs_libcrux::HpkeLibcrux;
//! use hpke_rs_crypto::{HpkeCrypto, Rng};
//!
//! // Set up hpke mode.
//! let mut hpke = Hpke::<HpkeLibcrux>::new(Mode::Base, KemAlgorithm::DhKem25519,
//!    KdfAlgorithm::HkdfSha256, AeadAlgorithm::ChaCha20Poly1305);
//!
//! // Generate keys. The other parties public key must be received in some way.
//! let (sk_r, pk_r) = hpke.generate_key_pair().unwrap().into_keys();
//! let (sk_s, pk_s) = hpke.generate_key_pair().unwrap().into_keys();
//!
//! // Set the input. Only `plain_text` is required
//! let info = b"HPKE demo info";
//! let aad = b"HPKE demo aad";
//! let plaintext = b"HPKE demo plain text";
//! let exporter_context = b"HPKE demo exporter context";
//!
//! // We don't use authentication or PSKs here.
//! let psk = None;
//! let psk_id = None;
//! let sk_s = None;
//! let pk_s = None;
//!
//! // Encrypt the `plaintext` to the receiver.
//! let (enc, ctxt) = hpke
//!     .seal(&pk_r, info, aad, plaintext, psk, psk_id, sk_s)
//!     .unwrap();
//!
//! // Decrypt the ciphertext on the receiver.
//! let ptxt = hpke
//!     .open(&enc, &sk_r, info, aad, &ctxt, psk, psk_id, pk_s)
//!     .unwrap();
//!
//! assert_eq!(ptxt, plaintext);
//! ```
//!
//! Encryption context.
//!
//! ```
//! use hpke_rs::{*, hpke_types::*};
//! use hpke_rs_libcrux::HpkeLibcrux;
//! use hpke_rs_crypto::{HpkeCrypto, Rng};
//!
//! // Set up hpke mode.
//! let mut hpke = Hpke::<HpkeLibcrux>::new(Mode::Base, KemAlgorithm::DhKem25519,
//!    KdfAlgorithm::HkdfSha256, AeadAlgorithm::ChaCha20Poly1305);
//!
//! // Generate keys. The other parties public key must be received in some way.
//! let (sk_r, pk_r) = hpke.generate_key_pair().unwrap().into_keys();
//! let (sk_s, pk_s) = hpke.generate_key_pair().unwrap().into_keys();
//!
//! // Set the input. Only `plain_text` is required
//! let info = b"HPKE demo info";
//! let aad = b"HPKE demo aad";
//! let plaintext = b"HPKE demo plain text";
//! let exporter_context = b"HPKE demo exporter context";
//!
//! // We don't use authentication or PSKs here.
//! let psk = None;
//! let psk_id = None;
//! let sk_s = None;
//! let pk_s = None;
//!
//! // Set up the context on both sides.
//! let (enc, mut sender_context) = hpke
//!     .setup_sender(&pk_r, info, psk, psk_id, sk_s)
//!     .unwrap();
//!
//! // Share `enc` with the receiver.
//! let mut receiver_context = hpke
//!     .setup_receiver(&enc, &sk_r, info, psk, psk_id, pk_s)
//!     .unwrap();
//!
//! // Encrypt the plaintext to the receiver, using the context.
//! let ctxt = sender_context.seal(aad, plaintext).unwrap();
//!
//! // Decrypt the ciphertext on the receiver, using the context.
//! let ptxt = receiver_context.open(aad, &ctxt).unwrap();
//!
//! assert_eq!(ptxt, plaintext);
//! ```

#![forbid(unsafe_code, unused_must_use, unstable_features)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    missing_docs,
    unused_import_braces,
    unused_extern_crates,
    unused_qualifications
)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use hpke_rs_crypto::TryRng;
use hpke_rs_crypto::{
    types::{
        AeadAlgorithm, KdfAlgorithm, KemAlgorithm, SingleStageKdfAlgorithm, TwoStageKdfAlgorithm,
    },
    HpkeCrypto, HpkeDefaultPrng,
};
use prelude::kdf::{labeled_derive, labeled_expand, labeled_extract, length_prefixed};

/// Re-export of the HPKE types from the [`hpke_rs_crypto`] crate.
pub use hpke_rs_crypto::types as hpke_types;

/// Re-export of the RustCrypto crate.
#[cfg(feature = "rustcrypto")]
pub use hpke_rs_rust_crypto as rustcrypto;

/// Re-export of the libcrux crate.
#[cfg(feature = "libcrux-no-sys-rng")]
pub use hpke_rs_libcrux as libcrux;

#[cfg(feature = "serialization")]
pub(crate) use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

mod dh_kem;
pub(crate) mod kdf;
mod kem;
pub mod prelude;

mod util;

#[cfg(test)]
mod test_aead;
#[cfg(test)]
mod test_kdf;

#[deprecated(
    since = "0.0.7",
    note = "Please use HpkeError instead. This alias will be removed with the first stable  0.1 release."
)]
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
type HPKEError = HpkeError;

/// HPKE Error types.
#[derive(Debug, Clone, PartialEq)]
pub enum HpkeError {
    /// Error opening an HPKE ciphertext.
    OpenError,

    /// Invalid configuration or arguments.
    InvalidConfig,

    /// Invalid input.
    InvalidInput,

    /// Unknown HPKE mode.
    UnknownMode,

    /// Inconsistent PSK input.
    InconsistentPsk,

    /// PSK input is required but missing.
    MissingPsk,

    /// PSK input is provided but not needed.
    UnnecessaryPsk,

    /// PSK input is too short (needs to be at least 32 bytes).
    InsecurePsk,

    /// An error in the crypto library occurred.
    CryptoError(String),

    /// The message limit for this AEAD, key, and nonce.
    MessageLimitReached,

    /// Unable to collect enough randomness.
    InsufficientRandomness,
}

#[cfg(feature = "std")]
impl std::error::Error for HpkeError {}

impl core::fmt::Display for HpkeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HPKE Error: {:?}", self)
    }
}

#[deprecated(
    since = "0.0.7",
    note = "Please use HpkePublicKey instead. This alias will be removed with the first stable  0.1 release."
)]
#[allow(clippy::upper_case_acronyms)]
#[allow(missing_docs)]
pub type HPKEPublicKey = HpkePublicKey;

/// An HPKE public key is a byte vector.
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct HpkePublicKey {
    value: Vec<u8>,
}

#[deprecated(
    since = "0.0.7",
    note = "Please use HpkePrivateKey instead. This alias will be removed with the first stable  0.1 release."
)]
#[allow(clippy::upper_case_acronyms)]
#[allow(missing_docs)]
pub type HPKEPrivateKey = HpkePrivateKey;

/// An HPKE private key is a byte vector.
#[derive(Default, Zeroize)]
#[zeroize(drop)] // XXX: Change to `ZeroizeOnDrop` when moving to 1.5
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hazmat", derive(Clone))]
pub struct HpkePrivateKey {
    value: Vec<u8>,
}

#[deprecated(
    since = "0.0.7",
    note = "Please use HpkeKeyPair instead. This alias will be removed with the first stable  0.1 release."
)]
#[allow(clippy::upper_case_acronyms)]
#[allow(missing_docs)]
pub type HPKEKeyPair = HpkeKeyPair;

/// An HPKE key pair has an HPKE private and public key.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hazmat", derive(Clone))]
pub struct HpkeKeyPair {
    private_key: HpkePrivateKey,
    public_key: HpkePublicKey,
}

/// HPKE supports four modes.
#[derive(PartialEq, Copy, Clone, Debug, Zeroize)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Mode {
    /// HPKE Base mode.
    Base = 0x00,

    /// HPKE with PSK.
    Psk = 0x01,

    /// Authenticated HPKE.
    Auth = 0x02,

    /// Authenticated HPKE with PSK.
    AuthPsk = 0x03,
}

impl core::fmt::Display for Mode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Mode {
    type Error = HpkeError;
    fn try_from(x: u8) -> Result<Mode, HpkeError> {
        match x {
            0x00 => Ok(Mode::Base),
            0x01 => Ok(Mode::Psk),
            0x02 => Ok(Mode::Auth),
            0x03 => Ok(Mode::AuthPsk),
            _ => Err(HpkeError::UnknownMode),
        }
    }
}

/// Type alias for encapsulated secrets.
/// A byte vector.
type EncapsulatedSecret = Vec<u8>;

/// Type alias for ciphertexts.
/// A byte vector.
type Ciphertext = Vec<u8>;

/// Type alias for plain text.
/// A byte vector.
type Plaintext = Vec<u8>;

/// The HPKE context.
/// Note that the RFC currently doesn't define this.
/// Also see <https://github.com/cfrg/draft-irtf-cfrg-hpke/issues/161>.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Context<Crypto: 'static + HpkeCrypto> {
    key: Vec<u8>,
    nonce: Vec<u8>,
    exporter_secret: Vec<u8>,
    sequence_number: u64,
    kem_id: KemAlgorithm,
    kdf_id: KdfAlgorithm,
    aead_id: AeadAlgorithm,
    #[zeroize(skip)]
    _crypto: core::marker::PhantomData<Crypto>,
}

#[cfg(feature = "hazmat")]
impl<Crypto: HpkeCrypto> core::fmt::Debug for Context<Crypto> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Context {{\n  key: {:?}\n  nonce: {:?}\n exporter_secret: {:?}\n seq no: {:?}\n}}",
            self.key, self.nonce, self.exporter_secret, self.sequence_number
        )
    }
}

#[cfg(not(feature = "hazmat"))]
impl<Crypto: HpkeCrypto> core::fmt::Debug for Context<Crypto> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Context {{\n  key: {:?}\n  nonce: {:?}\n exporter_secret: {:?}\n seq no: {:?}\n}}",
            &"***", &"***", &"***", &"***"
        )
    }
}

impl<Crypto: HpkeCrypto> Context<Crypto> {
    /// 5.2. Encryption and Decryption
    ///
    /// Takes the associated data and the plain text as byte slices and returns
    /// the ciphertext or an error.
    ///
    /// The context can be re-used up to the AEAD's message limit. However, we
    /// use only a 64 bit counter, which technically limits context
    /// re-use to 0xffffffffffffffff (2^64 - 1) uses.
    ///
    /// ```text
    /// def Context.Seal(aad, pt):
    ///   ct = Seal(self.key, self.ComputeNonce(self.seq), aad, pt)
    ///   self.IncrementSeq()
    ///   return ct
    /// ```
    pub fn seal(&mut self, aad: &[u8], plain_txt: &[u8]) -> Result<Ciphertext, HpkeError> {
        if self.aead_id == AeadAlgorithm::HpkeExport {
            return Err(HpkeError::InvalidConfig);
        }

        let ctxt = Crypto::aead_seal(
            self.aead_id,
            &self.key,
            &self.compute_nonce(),
            aad,
            plain_txt,
        )?;
        self.increment_seq()?;
        Ok(ctxt)
    }

    /// 5.2. Encryption and Decryption
    ///
    /// Takes the associated data and the ciphertext as byte slices and returns
    /// the plain text or an error.
    ///
    /// The context can be re-used up to the AEAD's message limit. However, we
    /// use only a 64 bit counter, which technically limits context
    /// re-use to 0xffffffffffffffff (2^64 - 1) uses.
    ///
    /// ```text
    /// def Context.Open(aad, ct):
    ///   pt = Open(self.key, self.ComputeNonce(self.seq), aad, ct)
    ///   if pt == OpenError:
    ///     raise OpenError
    ///   self.IncrementSeq()
    ///   return pt
    /// ```
    pub fn open(&mut self, aad: &[u8], cipher_txt: &[u8]) -> Result<Plaintext, HpkeError> {
        if self.aead_id == AeadAlgorithm::HpkeExport {
            return Err(HpkeError::InvalidConfig);
        }

        let ptxt = Crypto::aead_open(
            self.aead_id,
            &self.key,
            &self.compute_nonce(),
            aad,
            cipher_txt,
        )?;
        self.increment_seq()?;
        Ok(ptxt)
    }

    /// 5.3. Secret Export
    ///
    /// Takes a serialised exporter context as byte slice and a length for the
    /// output secret and returns an exporter secret as byte vector.
    ///
    /// ```text
    /// def Context.Export(exporter_context, L):
    ///  return LabeledExpand(self.exporter_secret, "sec", exporter_context, L)
    ///```
    pub fn export(&self, exporter_context: &[u8], length: usize) -> Result<Vec<u8>, HpkeError> {
        const LABEL: &str = "sec";

        if let Ok(kdf) = SingleStageKdfAlgorithm::try_from(self.kdf_id) {
            return labeled_derive::<Crypto>(
                kdf,
                &self.ciphersuite(),
                &self.exporter_secret,
                LABEL,
                exporter_context,
                length,
            )
            .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)));
        }

        labeled_expand::<Crypto>(
            self.two_stage_kdf()?,
            &self.exporter_secret,
            &self.ciphersuite(),
            LABEL,
            exporter_context,
            length,
        )
        .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)))
    }

    /// The two-stage KDF for this ciphersuite, or an error if it is single-stage.
    #[inline]
    fn two_stage_kdf(&self) -> Result<TwoStageKdfAlgorithm, HpkeError> {
        TwoStageKdfAlgorithm::try_from(self.kdf_id)
            .map_err(|_| HpkeError::CryptoError("Unsupported KDF".to_string()))
    }

    #[inline]
    fn ciphersuite(&self) -> Vec<u8> {
        util::concat(&[
            b"HPKE",
            &(self.kem_id as u16).to_be_bytes(),
            &(self.kdf_id as u16).to_be_bytes(),
            &(self.aead_id as u16).to_be_bytes(),
        ])
    }

    /// def Context<ROLE>.ComputeNonce(seq):
    ///     seq_bytes = I2OSP(seq, Nn)
    ///     return xor(self.base_nonce, seq_bytes)
    fn compute_nonce(&self) -> Vec<u8> {
        let seq = self.sequence_number.to_be_bytes();
        let mut enc_seq = vec![0u8; self.nonce.len() - seq.len()];
        enc_seq.extend_from_slice(&seq);
        util::xor_bytes(&enc_seq, &self.nonce)
    }

    /// def Context<ROLE>.IncrementSeq():
    ///     if self.seq >= (1 << (8*Nn)) - 1:
    ///       raise MessageLimitReached
    ///     self.seq += 1
    fn increment_seq(&mut self) -> Result<(), HpkeError> {
        if u128::from(self.sequence_number)
            >= ((1u128 << (8 * Crypto::aead_nonce_length(self.aead_id))) - 1)
        {
            // The limit is 0xffffffffffffffffffffffff for all currently implemented
            // ciphersuites.
            return Err(HpkeError::MessageLimitReached);
        }
        self.sequence_number = self
            .sequence_number
            // We use a u64, which is lower than the AEAD limit
            .checked_add(1)
            .ok_or(HpkeError::MessageLimitReached)?;
        Ok(())
    }
}

/// The HPKE configuration struct.
/// This holds the configuration for HPKE but no state.
/// To use HPKE first instantiate the configuration with
/// `let hpke = Hpke::new(mode, kem_mode, kdf_mode, aead_mode)`.
/// Now one can use the `hpke` configuration.
///
/// Note that cloning does NOT clone the PRNG state.
#[derive(Debug, Zeroize)]
pub struct Hpke<Crypto: 'static + HpkeCrypto> {
    mode: Mode,
    kem_id: KemAlgorithm,
    kdf_id: KdfAlgorithm,
    aead_id: AeadAlgorithm,
    // `HpkeCrypto::HpkePrng` is no longer required to implement `Zeroize` (most Rng types
    // don't), so this field is skipped here; a provider's PRNG is responsible for zeroizing
    // its own sensitive state, if any (e.g. via its own `Drop` impl).
    #[zeroize(skip)]
    prng: Crypto::HpkePrng,
}

/// Cloning gives the clone a fresh default PRNG ([`HpkeDefaultPrng::try_prng`], i.e. newly seeded
/// from the system), never a copy of this instance's PRNG state: two copies of a PRNG would
/// produce identical randomness, and thus identical ephemeral keys. This also holds if this
/// instance was built with [`Hpke::new_with_rng`]; the caller's PRNG is not carried over.
///
/// # Panics
///
/// Panics if the default PRNG can't be constructed (e.g. the system RNG fails).
impl<Crypto: 'static + HpkeDefaultPrng> Clone for Hpke<Crypto> {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            kem_id: self.kem_id,
            kdf_id: self.kdf_id,
            aead_id: self.aead_id,
            prng: Crypto::try_prng().expect("failed to construct the default PRNG"),
        }
    }
}

impl<Crypto: HpkeCrypto> core::fmt::Display for Hpke<Crypto> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}_{}_{}_{}",
            self.mode.to_string().to_lowercase(),
            self.kem_id.to_string().to_lowercase(),
            self.kdf_id.to_string().to_lowercase(),
            self.aead_id.to_string().to_lowercase()
        )
    }
}

impl<Crypto: HpkeDefaultPrng> Hpke<Crypto> {
    /// Set up the configuration for HPKE using the provider's default PRNG.
    ///
    /// # Panics
    ///
    /// Panics if the default PRNG can't be constructed (e.g. the system RNG fails). Use
    /// [`Hpke::try_new`] to get a `Result` instead, or [`Hpke::new_with_rng`] to supply a PRNG
    /// explicitly.
    pub fn new(
        mode: Mode,
        kem_id: KemAlgorithm,
        kdf_id: KdfAlgorithm,
        aead_id: AeadAlgorithm,
    ) -> Self {
        Self::try_new(mode, kem_id, kdf_id, aead_id).expect("failed to construct the default PRNG")
    }

    /// Like [`Hpke::new`], but returns an error instead of panicking when the default PRNG
    /// can't be constructed.
    pub fn try_new(
        mode: Mode,
        kem_id: KemAlgorithm,
        kdf_id: KdfAlgorithm,
        aead_id: AeadAlgorithm,
    ) -> Result<Self, HpkeError> {
        Ok(Self {
            mode,
            kem_id,
            kdf_id,
            aead_id,
            prng: Crypto::try_prng()?,
        })
    }
}

impl<Crypto: HpkeCrypto> Hpke<Crypto> {
    /// Set up the configuration for HPKE with a caller-provided PRNG.
    ///
    /// This is like [`Hpke::new`] but stores the given PRNG instead of
    /// constructing one via [`HpkeDefaultPrng::try_prng`]. Use this to bring your own
    /// randomness source — any type implementing `TryCryptoRng` — instead of a provider's
    /// default, or on platforms without a system RNG (e.g. wasm), where the caller has to
    /// construct the provider PRNG itself.
    ///
    /// # Example
    ///
    /// ```
    /// use hpke_rs::{*, hpke_types::*};
    /// use hpke_rs_libcrux::HpkeLibcrux;
    /// use hpke_rs_crypto::HpkeDefaultPrng;
    ///
    /// // Set up hpke mode with an explicit PRNG instance.
    /// let mut hpke = Hpke::<HpkeLibcrux>::new_with_rng(
    ///     Mode::Base,
    ///     KemAlgorithm::DhKem25519,
    ///     KdfAlgorithm::HkdfSha256,
    ///     AeadAlgorithm::ChaCha20Poly1305,
    ///     HpkeLibcrux::try_prng().unwrap(),
    /// );
    ///
    /// let (sk_r, pk_r) = hpke.generate_key_pair().unwrap().into_keys();
    /// let info = b"HPKE demo info";
    /// let aad = b"HPKE demo aad";
    /// let plaintext = b"HPKE demo plain text";
    ///
    /// let (enc, ctxt) = hpke.seal(&pk_r, info, aad, plaintext, None, None, None).unwrap();
    /// let ptxt = hpke.open(&enc, &sk_r, info, aad, &ctxt, None, None, None).unwrap();
    ///
    /// assert_eq!(ptxt, plaintext);
    /// ```
    pub fn new_with_rng(
        mode: Mode,
        kem_id: KemAlgorithm,
        kdf_id: KdfAlgorithm,
        aead_id: AeadAlgorithm,
        prng: Crypto::HpkePrng,
    ) -> Self {
        Self {
            mode,
            kem_id,
            kdf_id,
            aead_id,
            prng,
        }
    }

    /// Set up an HPKE sender.
    ///
    /// For the base and PSK modes this encapsulates the public key `pk_r`
    /// of the receiver.
    /// For the Auth and AuthPSK modes this encapsulates and authenticates
    /// the public key `pk_r` of the receiver with the senders secret key `sk_s`.
    ///
    /// **Note** that this API expects the public key to be encoded.
    /// This differs from the RFC.
    /// But the public keys will be present in encoded form rather than raw form
    /// such that it doesn't make sense to deserialize before passing it in.
    ///
    /// The encapsulated secret is returned together with the context.
    /// If the secret key is missing in an authenticated mode, an error is returned.
    pub fn setup_sender(
        &mut self,
        pk_r: &HpkePublicKey,
        info: &[u8],
        psk: Option<&[u8]>,
        psk_id: Option<&[u8]>,
        sk_s: Option<&HpkePrivateKey>,
    ) -> Result<(EncapsulatedSecret, Context<Crypto>), HpkeError> {
        let (zz, enc) = match self.mode {
            Mode::Base | Mode::Psk => kem::encaps::<Crypto>(self, pk_r.value.as_slice())?,
            Mode::Auth | Mode::AuthPsk => {
                let sk_s = match sk_s {
                    Some(s) => &s.value,
                    None => return Err(HpkeError::InvalidInput),
                };
                kem::auth_encaps::<Crypto>(self, pk_r.value.as_slice(), sk_s)?
            }
        };
        Ok((
            enc,
            self.key_schedule(
                &zz,
                info,
                psk.unwrap_or_default(),
                psk_id.unwrap_or_default(),
            )?,
        ))
    }

    /// Set up an HPKE receiver.
    ///
    /// For the base and PSK modes this decapsulates `enc` with the secret key
    /// `sk_r` of the receiver.
    /// For the Auth and AuthPSK modes this decapsulates and authenticates `enc`
    /// with the secret key `sk_r` of the receiver and the senders public key `pk_s`.
    ///
    /// **Note** that this API expects the public key to be encoded.
    /// This differs from the RFC.
    /// But the public keys will be present in encoded form rather than raw form
    /// such that it doesn't make sense to deserialize before passing it in.
    ///
    /// The context based on the decapsulated values and, if present, the PSK is
    /// returned.
    /// If the secret key is missing in an authenticated mode, an error is returned.
    pub fn setup_receiver(
        &self,
        enc: &[u8],
        sk_r: &HpkePrivateKey,
        info: &[u8],
        psk: Option<&[u8]>,
        psk_id: Option<&[u8]>,
        pk_s: Option<&HpkePublicKey>,
    ) -> Result<Context<Crypto>, HpkeError> {
        let zz = match self.mode {
            Mode::Base | Mode::Psk => kem::decaps::<Crypto>(self.kem_id, enc, &sk_r.value)?,
            Mode::Auth | Mode::AuthPsk => {
                let pk_s = match pk_s {
                    Some(s) => s.value.as_slice(),
                    None => return Err(HpkeError::InvalidInput),
                };
                kem::auth_decaps::<Crypto>(self.kem_id, enc, &sk_r.value, pk_s)?
            }
        };
        self.key_schedule(
            &zz,
            info,
            psk.unwrap_or_default(),
            psk_id.unwrap_or_default(),
        )
    }

    /// 6. Single-Shot APIs
    ///
    /// 6.1. Encryption and Decryption
    ///
    /// Single shot API to encrypt the bytes in `plain_text` to the public key
    /// `pk_r`.
    ///
    /// **Note** that this API expects the public key to be encoded.
    /// This differs from the RFC.
    /// But the public keys will be present in encoded form rather than raw form
    /// such that it doesn't make sense to deserialize before passing it in.
    ///
    /// Returns the encapsulated secret and the ciphertext, or an error.
    #[allow(clippy::too_many_arguments)]
    pub fn seal(
        &mut self,
        pk_r: &HpkePublicKey,
        info: &[u8],
        aad: &[u8],
        plain_txt: &[u8],
        psk: Option<&[u8]>,
        psk_id: Option<&[u8]>,
        sk_s: Option<&HpkePrivateKey>,
    ) -> Result<(EncapsulatedSecret, Ciphertext), HpkeError> {
        let (enc, mut context) = self.setup_sender(pk_r, info, psk, psk_id, sk_s)?;
        let ctxt = context.seal(aad, plain_txt)?;
        Ok((enc, ctxt))
    }

    /// 6. Single-Shot APIs
    ///
    /// 6.1. Encryption and Decryption
    ///
    /// Single shot API to decrypt the bytes in `ct` with the private key `sk_r`.
    ///
    /// **Note** that this API expects the public key to be encoded.
    /// This differs from the RFC.
    /// But the public keys will be present in encoded form rather than raw form
    /// such that it doesn't make sense to deserialize before passing it in.
    ///
    /// Returns the decrypted plain text, or an error.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        &self,
        enc: &[u8],
        sk_r: &HpkePrivateKey,
        info: &[u8],
        aad: &[u8],
        ct: &[u8],
        psk: Option<&[u8]>,
        psk_id: Option<&[u8]>,
        pk_s: Option<&HpkePublicKey>,
    ) -> Result<Plaintext, HpkeError> {
        let mut context = self.setup_receiver(enc, sk_r, info, psk, psk_id, pk_s)?;
        context.open(aad, ct)
    }

    /// 6. Single-Shot APIs
    ///
    /// 6.2. Secret Export
    ///
    /// Single shot API to derive an exporter secret for receiver with public key
    /// `pk_r`.
    ///
    /// **Note** that this API expects the public key to be encoded.
    /// This differs from the RFC.
    /// But the public keys will be present in encoded form rather than raw form
    /// such that it doesn't make sense to deserialize before passing it in.
    ///
    /// Returns the encapsulated secret and the exporter secret for the given
    /// exporter context and length.
    #[allow(clippy::too_many_arguments)]
    pub fn send_export(
        &mut self,
        pk_r: &HpkePublicKey,
        info: &[u8],
        psk: Option<&[u8]>,
        psk_id: Option<&[u8]>,
        sk_s: Option<&HpkePrivateKey>,
        exporter_context: &[u8],
        length: usize,
    ) -> Result<(EncapsulatedSecret, Vec<u8>), HpkeError> {
        let (enc, context) = self.setup_sender(pk_r, info, psk, psk_id, sk_s)?;
        Ok((enc, context.export(exporter_context, length)?))
    }

    /// 6. Single-Shot APIs
    ///
    /// 6.2. Secret Export
    ///
    /// Single shot API to derive an exporter secret for receiver with private key
    /// `sk_r`.
    ///
    /// **Note** that this API expects the public key to be encoded.
    /// This differs from the RFC.
    /// But the public keys will be present in encoded form rather than raw form
    /// such that it doesn't make sense to deserialize before passing it in.
    ///
    /// Returns the exporter secret for the given exporter context and length.
    #[allow(clippy::too_many_arguments)]
    pub fn receiver_export(
        &self,
        enc: &[u8],
        sk_r: &HpkePrivateKey,
        info: &[u8],
        psk: Option<&[u8]>,
        psk_id: Option<&[u8]>,
        pk_s: Option<&HpkePublicKey>,
        exporter_context: &[u8],
        length: usize,
    ) -> Result<Vec<u8>, HpkeError> {
        let context = self.setup_receiver(enc, sk_r, info, psk, psk_id, pk_s)?;
        context.export(exporter_context, length)
    }

    /// Verify PSKs.
    #[inline(always)]
    fn verify_psk_inputs(&self, psk: &[u8], psk_id: &[u8]) -> Result<(), HpkeError> {
        let got_psk = !psk.is_empty();
        let got_psk_id = !psk_id.is_empty();
        if (got_psk && !got_psk_id) || (!got_psk && got_psk_id) {
            return Err(HpkeError::InconsistentPsk);
        }

        if got_psk && (self.mode == Mode::Base || self.mode == Mode::Auth) {
            return Err(HpkeError::UnnecessaryPsk);
        }
        if !got_psk && (self.mode == Mode::Psk || self.mode == Mode::AuthPsk) {
            return Err(HpkeError::MissingPsk);
        }

        // The PSK MUST have at least 32 bytes of entropy and SHOULD be of length Nh bytes or longer.
        if (self.mode == Mode::Psk || self.mode == Mode::AuthPsk) && psk.len() < 32 {
            return Err(HpkeError::InsecurePsk);
        }

        Ok(())
    }

    #[inline]
    fn ciphersuite(&self) -> Vec<u8> {
        util::concat(&[
            b"HPKE",
            &(self.kem_id as u16).to_be_bytes(),
            &(self.kdf_id as u16).to_be_bytes(),
            &(self.aead_id as u16).to_be_bytes(),
        ])
    }

    /// The two-stage KDF for this ciphersuite, or an error if it is single-stage.
    #[inline]
    fn two_stage_kdf(&self) -> Result<TwoStageKdfAlgorithm, HpkeError> {
        TwoStageKdfAlgorithm::try_from(self.kdf_id)
            .map_err(|_| HpkeError::CryptoError("Unsupported KDF".to_string()))
    }

    #[inline]
    fn key_schedule_context(
        &self,
        kdf: TwoStageKdfAlgorithm,
        info: &[u8],
        psk_id: &[u8],
        suite_id: &[u8],
    ) -> Result<Vec<u8>, HpkeError> {
        let psk_id_hash = labeled_extract::<Crypto>(kdf, &[0], suite_id, "psk_id_hash", psk_id)?;
        let info_hash = labeled_extract::<Crypto>(kdf, &[0], suite_id, "info_hash", info)?;
        Ok(util::concat(&[
            &[self.mode as u8],
            &psk_id_hash,
            &info_hash,
        ]))
    }

    /// Creating the Encryption Context
    /// Generate the HPKE context from the given input.
    pub fn key_schedule(
        &self,
        shared_secret: &[u8],
        info: &[u8],
        psk: &[u8],
        psk_id: &[u8],
    ) -> Result<Context<Crypto>, HpkeError> {
        self.verify_psk_inputs(psk, psk_id)?;
        let suite_id = self.ciphersuite();

        // Single-stage (SHAKE) KDFs use a different key-schedule shape: a single
        // `LabeledDerive` producing key ‖ base_nonce ‖ exporter_secret, with the
        // PSK/info length-prefixed. See draft-ietf-hpke-pq.
        if let Ok(kdf) = SingleStageKdfAlgorithm::try_from(self.kdf_id) {
            let nk = Crypto::aead_key_length(self.aead_id);
            let nn = Crypto::aead_nonce_length(self.aead_id);
            let nh = Crypto::kdf_digest_length(self.kdf_id);

            // Every value below is emitted with a 2-byte length prefix; reject
            // anything that would not fit rather than silently truncating it.
            for field in [psk, shared_secret, psk_id, info] {
                if field.len() > u16::MAX as usize {
                    return Err(HpkeError::InvalidInput);
                }
            }

            let secrets = util::concat(&[&length_prefixed(psk), &length_prefixed(shared_secret)]);
            let context = util::concat(&[
                &[self.mode as u8],
                &length_prefixed(psk_id),
                &length_prefixed(info),
            ]);
            let secret = labeled_derive::<Crypto>(
                kdf,
                &suite_id,
                &secrets,
                "secret",
                &context,
                nk + nn + nh,
            )
            .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)))?;

            return Ok(Context {
                key: secret[..nk].to_vec(),
                nonce: secret[nk..nk + nn].to_vec(),
                exporter_secret: secret[nk + nn..].to_vec(),
                sequence_number: 0,
                kem_id: self.kem_id,
                kdf_id: self.kdf_id,
                aead_id: self.aead_id,
                _crypto: core::marker::PhantomData,
            });
        }

        let kdf = self.two_stage_kdf()?;
        let key_schedule_context = self.key_schedule_context(kdf, info, psk_id, &suite_id)?;
        let secret = labeled_extract::<Crypto>(kdf, shared_secret, &suite_id, "secret", psk)
            .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)))?;

        let key = labeled_expand::<Crypto>(
            kdf,
            &secret,
            &suite_id,
            "key",
            &key_schedule_context,
            Crypto::aead_key_length(self.aead_id),
        )
        .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)))?;
        let base_nonce = labeled_expand::<Crypto>(
            kdf,
            &secret,
            &suite_id,
            "base_nonce",
            &key_schedule_context,
            Crypto::aead_nonce_length(self.aead_id),
        )
        .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)))?;
        let exporter_secret = labeled_expand::<Crypto>(
            kdf,
            &secret,
            &suite_id,
            "exp",
            &key_schedule_context,
            Crypto::kdf_digest_length(self.kdf_id),
        )
        .map_err(|e| HpkeError::CryptoError(format!("Crypto error: {}", e)))?;

        Ok(Context {
            key,
            nonce: base_nonce,
            exporter_secret,
            sequence_number: 0,
            kem_id: self.kem_id,
            kdf_id: self.kdf_id,
            aead_id: self.aead_id,
            _crypto: core::marker::PhantomData,
        })
    }

    /// 4. Cryptographic Dependencies
    ///
    /// Randomized algorithm to generate a key pair `(skX, pkX)` for the KEM.
    /// This is equivalent to `derive_key_pair(random_vector(sk.len()))`
    ///
    /// Returns an `HpkeKeyPair`.
    pub fn generate_key_pair(&mut self) -> Result<HpkeKeyPair, HpkeError> {
        let (sk, pk) = kem::key_gen::<Crypto>(self.kem_id, &mut self.prng)?;
        Ok(HpkeKeyPair::new(sk.0.clone(), pk))
    }

    /// 7.1.2. DeriveKeyPair
    /// Derive a key pair for the used KEM with the given input key material.
    ///
    /// Returns an `HpkeKeyPair` result or an `HpkeError` if key derivation fails.
    pub fn derive_key_pair(&self, ikm: &[u8]) -> Result<HpkeKeyPair, HpkeError> {
        let (pk, sk) = kem::derive_key_pair::<Crypto>(self.kem_id, ikm)?;
        Ok(HpkeKeyPair::new(sk.0.clone(), pk))
    }

    #[inline]
    pub(crate) fn random(&mut self, len: usize) -> Result<zeroize::Zeroizing<Vec<u8>>, HpkeError> {
        let prng = &mut self.prng;
        let mut out = zeroize::Zeroizing::new(vec![0u8; len]);

        prng.try_fill_bytes(&mut out)
            .map_err(|_| HpkeError::InsufficientRandomness)?;

        Ok(out)
    }
}

impl HpkeKeyPair {
    /// Create a new HPKE key pair.
    /// Consumes the private and public key bytes.
    pub fn new(sk: Vec<u8>, pk: Vec<u8>) -> Self {
        Self {
            private_key: HpkePrivateKey::new(sk),
            public_key: HpkePublicKey::new(pk),
        }
    }

    /// Get a reference to the HPKE private key of this key pair.
    pub fn private_key(&self) -> &HpkePrivateKey {
        &self.private_key
    }

    /// Get a reference to the HPKE public key of this key pair.
    pub fn public_key(&self) -> &HpkePublicKey {
        &self.public_key
    }

    /// Split the key pair into the two keys
    pub fn into_keys(self) -> (HpkePrivateKey, HpkePublicKey) {
        (self.private_key, self.public_key)
    }

    /// Build a key pair from two keys
    pub fn from_keys(private_key: HpkePrivateKey, public_key: HpkePublicKey) -> Self {
        Self {
            private_key,
            public_key,
        }
    }
}

impl From<(Vec<u8>, Vec<u8>)> for HpkeKeyPair {
    fn from((sk, pk): (Vec<u8>, Vec<u8>)) -> Self {
        Self::new(sk, pk)
    }
}

impl From<(&[u8], &[u8])> for HpkeKeyPair {
    fn from((sk, pk): (&[u8], &[u8])) -> Self {
        Self::new(sk.to_vec(), pk.to_vec())
    }
}

impl HpkePrivateKey {
    /// Create a new HPKE private key.
    /// Consumes the private key bytes.
    pub fn new(b: Vec<u8>) -> Self {
        Self { value: b }
    }

    /// Get the raw key as byte slice.
    #[cfg(feature = "hazmat")]
    pub fn as_slice(&self) -> &[u8] {
        &self.value
    }
}

impl From<Vec<u8>> for HpkePrivateKey {
    fn from(b: Vec<u8>) -> Self {
        Self::new(b)
    }
}

impl From<&[u8]> for HpkePrivateKey {
    fn from(b: &[u8]) -> Self {
        Self::new(b.to_vec())
    }
}

/// Constant-time comparison of private key values.
impl PartialEq for HpkePrivateKey {
    fn eq(&self, other: &Self) -> bool {
        use subtle::ConstantTimeEq;
        self.value.ct_eq(&other.value).into()
    }
}

#[cfg(not(feature = "hazmat"))]
impl core::fmt::Debug for HpkePrivateKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.debug_struct("HpkePrivateKey")
            .field("value", &"***")
            .finish()
    }
}

#[cfg(feature = "hazmat")]
impl core::fmt::Debug for HpkePrivateKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.debug_struct("HpkePrivateKey")
            .field("value", &self.value)
            .finish()
    }
}

impl HpkePublicKey {
    /// Create a new HPKE public key.
    /// Consumes the public key bytes.
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    /// Get the raw key as byte slice.
    pub fn as_slice(&self) -> &[u8] {
        self.value.as_slice()
    }
}

impl From<Vec<u8>> for HpkePublicKey {
    fn from(b: Vec<u8>) -> Self {
        Self::new(b)
    }
}

impl From<&[u8]> for HpkePublicKey {
    fn from(b: &[u8]) -> Self {
        Self::new(b.to_vec())
    }
}

#[cfg(feature = "serialization")]
impl tls_codec::Size for HpkePublicKey {
    #[inline(always)]
    fn tls_serialized_len(&self) -> usize {
        tls_codec::VLByteSlice(self.as_slice()).tls_serialized_len()
    }
}

#[cfg(feature = "serialization")]
impl tls_codec::Serialize for HpkePublicKey {
    #[inline(always)]
    fn tls_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, tls_codec::Error> {
        tls_codec::VLByteSlice(self.as_slice()).tls_serialize(writer)
    }
}

#[cfg(feature = "serialization")]
impl tls_codec::Size for &HpkePublicKey {
    #[inline(always)]
    fn tls_serialized_len(&self) -> usize {
        tls_codec::VLByteSlice(self.as_slice()).tls_serialized_len()
    }
}

#[cfg(feature = "serialization")]
impl tls_codec::Serialize for &HpkePublicKey {
    #[inline(always)]
    fn tls_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<usize, tls_codec::Error> {
        tls_codec::VLByteSlice(self.as_slice()).tls_serialize(writer)
    }
}

#[cfg(feature = "serialization")]
impl tls_codec::Deserialize for HpkePublicKey {
    #[inline(always)]
    fn tls_deserialize<R: std::io::Read>(bytes: &mut R) -> Result<Self, tls_codec::Error> {
        Ok(Self {
            value: tls_codec::VLBytes::tls_deserialize(bytes)?.into(),
        })
    }
}

#[cfg(feature = "serialization")]
impl tls_codec::Deserialize for &HpkePublicKey {
    #[inline(always)]
    fn tls_deserialize<R: std::io::Read>(_: &mut R) -> Result<Self, tls_codec::Error> {
        Err(tls_codec::Error::DecodingError(
            "Error trying to deserialize a reference.".to_string(),
        ))
    }
}

/// Test util module. Should be moved really.
#[cfg(feature = "hpke-test")]
pub mod test_util {
    use alloc::{format, string::String, vec, vec::Vec};

    use hpke_rs_crypto::HpkeCrypto;

    impl<Crypto: HpkeCrypto> super::Context<Crypto> {
        /// Get a reference to the key in the context.
        #[doc(hidden)]
        pub fn key(&self) -> &[u8] {
            &self.key
        }
        /// Get a reference to the nonce in the context.
        #[doc(hidden)]
        pub fn nonce(&self) -> &[u8] {
            &self.nonce
        }
        /// Get a reference to the exporter secret in the context.
        #[doc(hidden)]
        pub fn exporter_secret(&self) -> &[u8] {
            &self.exporter_secret
        }
        /// Get a reference to the sequence number in the context.
        #[doc(hidden)]
        pub fn sequence_number(&self) -> u64 {
            self.sequence_number
        }
    }

    /// Convert `bytes` to a hex string.
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        let mut hex = String::new();
        for &b in bytes {
            hex += &format!("{:02X}", b);
        }
        hex
    }

    /// Convert a hex string to a byte vector.
    pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
        assert!(hex.len().is_multiple_of(2));
        let mut bytes = Vec::new();
        for i in 0..(hex.len() / 2) {
            bytes.push(u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
        }
        bytes
    }

    /// Convert a hex string to a byte vector.
    /// If the input is `None`, this returns an empty vector.
    pub fn hex_to_bytes_option(hex: Option<String>) -> Vec<u8> {
        match hex {
            Some(s) => hex_to_bytes(&s),
            None => vec![],
        }
    }

    /// Convert a byte slice into byte slice option.
    /// Returns `Nonce` if the byte slice is empty and `Some(v)` otherwise.
    pub fn vec_to_option_slice(v: &[u8]) -> Option<&[u8]> {
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }
}

impl From<hpke_rs_crypto::error::Error> for HpkeError {
    fn from(e: hpke_rs_crypto::error::Error) -> Self {
        match e {
            hpke_rs_crypto::error::Error::AeadOpenError => HpkeError::OpenError,
            hpke_rs_crypto::error::Error::AeadInvalidNonce
            | hpke_rs_crypto::error::Error::AeadInvalidCiphertext => HpkeError::InvalidInput,
            hpke_rs_crypto::error::Error::UnknownAeadAlgorithm => HpkeError::UnknownMode,
            hpke_rs_crypto::error::Error::CryptoLibraryError(s) => HpkeError::CryptoError(s),
            hpke_rs_crypto::error::Error::HpkeInvalidOutputLength => {
                HpkeError::CryptoError("Invalid HPKE output length".to_string())
            }
            hpke_rs_crypto::error::Error::UnknownKdfAlgorithm => {
                HpkeError::CryptoError("Unknown KDF algorithm.".to_string())
            }
            hpke_rs_crypto::error::Error::KemInvalidSecretKey => {
                HpkeError::CryptoError("Invalid KEM secret key".to_string())
            }
            hpke_rs_crypto::error::Error::KemInvalidPublicKey => {
                HpkeError::CryptoError("Invalid KEM public key".to_string())
            }
            hpke_rs_crypto::error::Error::UnknownKemAlgorithm => {
                HpkeError::CryptoError("Unknown KEM algorithm".to_string())
            }
            hpke_rs_crypto::error::Error::InsufficientRandomness => {
                HpkeError::InsufficientRandomness
            }
            hpke_rs_crypto::error::Error::UnsupportedKemOperation => HpkeError::InvalidConfig,
            hpke_rs_crypto::error::Error::KemInvalidCiphertext => HpkeError::InvalidInput,
        }
    }
}
