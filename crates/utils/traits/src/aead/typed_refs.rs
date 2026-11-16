//! This module contains the trait and related errors for an Authenticated
//! Encryption with Associated Data (AEAD) scheme that takes array references
//! as arguments and writes outputs to mutable array references.

use libcrux_secrets::U8;

use super::slice::KeyGenError;

/// Error that can occur during encryption.
#[derive(Debug, PartialEq, Eq)]
pub enum EncryptError {
    /// The ciphertext buffer has the wrong length.
    WrongCiphertextLength,

    /// The plaintext is too long for this algorithm or implementation.
    PlaintextTooLong,

    /// The AAD is too long for this algorithm or implementation.
    AadTooLong,

    /// The key is for the wrong algorithm.
    WrongKey,

    /// The tag is for the wrong algorithm.
    WrongTag,

    /// The nonce is for the wrong algorithm.
    WrongNonce,

    /// An unknown error occurred during encryption.
    Unknown,
}

/// Error that can occur during decryption.
#[derive(Debug, PartialEq, Eq)]
pub enum DecryptError {
    /// The authentication tag is invalid; the ciphertext has been tampered with
    /// or the key/nonce/aad is incorrect.
    InvalidTag,

    /// The plaintext buffer has the wrong length.
    WrongPlaintextLength,

    /// The plaintext is too long for this algorithm or implementation.
    PlaintextTooLong,

    /// The AAD is too long for this algorithm or implementation.
    AadTooLong,

    /// The key is for the wrong algorithm.
    WrongKey,

    /// The tag is for the wrong algorithm.
    WrongTag,

    /// The nonce is for the wrong algorithm.
    WrongNonce,

    /// An unknown error occurred during decryption.
    Unknown,
}

impl From<super::arrayref::EncryptError> for EncryptError {
    fn from(value: super::arrayref::EncryptError) -> Self {
        match value {
            super::arrayref::EncryptError::WrongCiphertextLength => {
                EncryptError::WrongCiphertextLength
            }
            super::arrayref::EncryptError::PlaintextTooLong => EncryptError::PlaintextTooLong,
            super::arrayref::EncryptError::AadTooLong => EncryptError::AadTooLong,
            super::arrayref::EncryptError::Unknown => EncryptError::Unknown,
        }
    }
}

impl From<super::arrayref::DecryptError> for DecryptError {
    fn from(value: super::arrayref::DecryptError) -> Self {
        match value {
            super::arrayref::DecryptError::InvalidTag => DecryptError::InvalidTag,
            super::arrayref::DecryptError::WrongPlaintextLength => {
                DecryptError::WrongPlaintextLength
            }
            super::arrayref::DecryptError::PlaintextTooLong => DecryptError::PlaintextTooLong,
            super::arrayref::DecryptError::AadTooLong => DecryptError::AadTooLong,
            super::arrayref::DecryptError::Unknown => DecryptError::Unknown,
        }
    }
}

impl core::fmt::Display for EncryptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            EncryptError::WrongCiphertextLength => "ciphertext buffer has wrong length",
            EncryptError::PlaintextTooLong => {
                "plaintext is too long for algorithm or implementation"
            }
            EncryptError::AadTooLong => "aad is too long for algorithm or implementation",
            EncryptError::Unknown => "an unknown error occurred",
            EncryptError::WrongKey => "key is for wrong algorithm",
            EncryptError::WrongTag => "tag is for wrong algorithm",
            EncryptError::WrongNonce => "nonce is for wrong algorithm",
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for DecryptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            DecryptError::InvalidTag => "invalid authentication tag",
            DecryptError::WrongPlaintextLength => "plaintext buffer has wrong length",
            DecryptError::PlaintextTooLong => {
                "plaintext is too long for algorithm or implementation"
            }
            DecryptError::AadTooLong => "aad is too long for algorithm or implementation",
            DecryptError::Unknown => "an unknown error occurred",
            DecryptError::WrongKey => "key is for wrong algorithm",
            DecryptError::WrongTag => "tag is for wrong algorithm",
            DecryptError::WrongNonce => "nonce is for wrong algorithm",
        };

        f.write_str(text)
    }
}

// These are the types for the arguments to AEAD. I wonder if it makes sense to do stuff like
// implementing a random constructor for Nonce for all LEN in 24..64 or so
//
// There also is the question whether we want these to be byte-oriented or whether we want to just
// make these associated types. That would mean that we'd have to define them separately for all
// implementations.

/// A Key with the given algorithm. The bytes are borrowed. Contains a marker for which
/// algorithm the key is to be used with.
#[derive(Clone, Copy)]
pub struct KeyRef<'a, Algo> {
    algorithm: Algo,
    key: &'a [U8],
}

/// A Key with the given algorithm. The bytes are borrowed mutably. Contains a marker for
/// which algorithm the key is to be used with.
pub struct KeyMut<'a, Algo> {
    algorithm: Algo,
    key: &'a mut [U8],
}

/// A tag with the given algorithm. The bytes are borrowed. Contains a marker for which
/// algorithm the key is to be used with.
#[derive(Clone, Copy)]
pub struct TagRef<'a, Algo> {
    algorithm: Algo,
    tag: &'a [U8],
}

/// A mutable tag with the given algorithm. The bytes are borrowed mutably. Contains a marker
/// for which algorithm the key is to be used with.
pub struct TagMut<'a, Algo> {
    algorithm: Algo,
    tag: &'a mut [U8],
}

/// A nonce with the given algorithm. The bytes are borrowed. Contains a marker for which
/// algorithm the key is to be used with.
#[derive(Clone, Copy)]
pub struct NonceRef<'a, Algo> {
    algorithm: Algo,
    nonce: &'a [U8],
}

impl<'a, Algo: Aead + core::fmt::Debug> core::fmt::Debug for KeyRef<'a, Algo> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Key")
            .field(&self.algorithm)
            .field(&"***")
            .finish()
    }
}

/// An Authenticated Encryption with Associated Data (AEAD) scheme. This trait
/// is low-level and is mostly used for implementing other, more usable APIs.
///
/// Some implementors of this trait may impose stronger restrictions on the inputs than described
/// here. Check the documentation of the types implementing this trait to make sure which inputs
/// are valid.
pub trait Aead: Copy + PartialEq {
    /// Returns the length of keys for this algorithm in bytes.
    fn key_len(&self) -> usize;
    /// Returns the length of authentication tags for this algorithm in bytes.
    fn tag_len(&self) -> usize;
    /// Returns the length of nonces for this algorithm in bytes.
    fn nonce_len(&self) -> usize;

    /// Generate a new key for this algorithm using the provided randomness.
    fn keygen<'a>(&self, key: KeyMut<'a, Self>, rand: &[U8]) -> Result<(), KeyGenError>;

    /// Encrypt a plaintext message, producing a ciphertext and an authentication tag.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn encrypt<'a>(
        &self,
        ciphertext: &mut [u8],
        tag: TagMut<'a, Self>,
        key: KeyRef<'a, Self>,
        nonce: NonceRef<'a, Self>,
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError>;

    /// Decrypt a ciphertext, verifying its authenticity.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn decrypt<'a>(
        &self,
        plaintext: &mut [U8],
        key: KeyRef<'a, Self>,
        nonce: NonceRef<'a, Self>,
        aad: &[u8],
        ciphertext: &[u8],
        tag: TagRef<'a, Self>,
    ) -> Result<(), DecryptError>;

    /// Creates a new key given the algorithm.
    fn new_key<'a>(self, key: &'a [U8]) -> Result<KeyRef<'a, Self>, WrongLengthError> {
        KeyRef::new_for_algo(self, key)
    }
    /// Creates a new mutable key given the algorithm.
    fn new_key_mut<'a>(self, key: &'a mut [U8]) -> Result<KeyMut<'a, Self>, WrongLengthError> {
        KeyMut::new_for_algo(self, key)
    }
    /// Creates a new tag given the algorithm.
    fn new_tag<'a>(self, tag: &'a [U8]) -> Result<TagRef<'a, Self>, WrongLengthError> {
        TagRef::new_for_algo(self, tag)
    }
    /// Creates a new mutable tag given the algorithm.
    fn new_tag_mut<'a>(self, tag_mut: &'a mut [U8]) -> Result<TagMut<'a, Self>, WrongLengthError> {
        TagMut::new_for_algo(self, tag_mut)
    }
    /// Creates a new nonce given the algorithm.
    fn new_nonce<'a>(self, nonce: &'a [U8]) -> Result<NonceRef<'a, Self>, WrongLengthError> {
        NonceRef::new_for_algo(self, nonce)
    }
}

impl<
        const KEY_LEN: usize,
        const TAG_LEN: usize,
        const NONCE_LEN: usize,
        Algo: super::typed_owned::Aead<
                Key = [U8; KEY_LEN],
                Tag = [U8; TAG_LEN],
                Nonce = [U8; NONCE_LEN],
                Rand = [U8; KEY_LEN],
            > + Copy
            + PartialEq,
    > Aead for Algo
{
    fn key_len(&self) -> usize {
        KEY_LEN
    }

    fn tag_len(&self) -> usize {
        TAG_LEN
    }

    fn nonce_len(&self) -> usize {
        NONCE_LEN
    }

    fn keygen<'a>(&self, key: KeyMut<'a, Self>, rand: &[U8]) -> Result<(), KeyGenError> {
        if rand.len() < KEY_LEN {
            return Err(KeyGenError::InsufficientRandomness);
        }

        key.key.copy_from_slice(rand);

        Ok(())
    }

    fn encrypt<'a>(
        &self,
        ciphertext: &mut [u8],
        mut tag: TagMut<'a, Self>,
        key: KeyRef<'a, Self>,
        nonce: NonceRef<'a, Self>,
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError> {
        if key.algo() != self {
            return Err(EncryptError::WrongKey);
        }
        if tag.algo() != self {
            return Err(EncryptError::WrongTag);
        }
        if nonce.algo() != self {
            return Err(EncryptError::WrongNonce);
        }

        // now we expect the lengths to be correct, so later mismatches are unknown errors
        let key: &[U8; KEY_LEN] = key.as_ref().try_into().map_err(|_| EncryptError::Unknown)?;

        let tag_raw: &mut [U8; TAG_LEN] =
            tag.as_mut().try_into().map_err(|_| EncryptError::Unknown)?;

        let nonce: &[U8; NONCE_LEN] = nonce
            .as_ref()
            .try_into()
            .map_err(|_| EncryptError::Unknown)?;

        <Self as super::typed_owned::Aead>::encrypt(
            ciphertext,
            tag_raw.into(),
            key.into(),
            nonce.into(),
            aad,
            plaintext,
        )
        .map_err(EncryptError::from)
    }

    fn decrypt<'a>(
        &self,
        plaintext: &mut [U8],
        key: KeyRef<'a, Self>,
        nonce: NonceRef<'a, Self>,
        aad: &[u8],
        ciphertext: &[u8],
        tag: TagRef<'a, Self>,
    ) -> Result<(), DecryptError> {
        if key.algo() != self {
            return Err(DecryptError::WrongKey);
        }
        if tag.algo() != self {
            return Err(DecryptError::WrongTag);
        }
        if nonce.algo() != self {
            return Err(DecryptError::WrongNonce);
        }

        // now we expect the lengths to be correct, so later mismatches are unknown errors
        let key: &[U8; KEY_LEN] = key.as_ref().try_into().map_err(|_| DecryptError::Unknown)?;

        let tag: &[U8; TAG_LEN] = tag.as_ref().try_into().map_err(|_| DecryptError::Unknown)?;

        let nonce: &[U8; NONCE_LEN] = nonce
            .as_ref()
            .try_into()
            .map_err(|_| DecryptError::Unknown)?;

        <Self as super::typed_owned::Aead>::decrypt(
            plaintext,
            key.into(),
            nonce.into(),
            aad,
            ciphertext,
            tag.into(),
        )
        .map_err(DecryptError::from)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WrongLengthError;

impl<'a, Algo: Aead> KeyRef<'a, Algo> {
    /// Creates a new key for the provided algorithm. Checks that the length is correct.
    pub fn new_for_algo(algo: Algo, key: &'a [U8]) -> Result<Self, WrongLengthError> {
        (key.len() == algo.key_len())
            .then_some(KeyRef {
                algorithm: algo,
                key,
            })
            .ok_or(WrongLengthError)
    }

    /// Performs AEAD encryption. If algo is multiplexed, it checks that the algorithms specified
    /// in the key, nonce and tag are consistent.
    pub fn encrypt(
        &self,
        ciphertext: &mut [u8],
        tag: TagMut<'a, Algo>,
        nonce: NonceRef<'a, Algo>,
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError> {
        self.algorithm
            .encrypt(ciphertext, tag, *self, nonce, aad, plaintext)
    }

    /// Performs AEAD decryption. If algo is multiplexed, it checks that the algorithms specified
    /// in the key, nonce and tag are consistent.
    pub fn decrypt(
        &self,
        plaintext: &mut [U8],
        nonce: NonceRef<'a, Algo>,
        aad: &[u8],
        ciphertext: &[u8],
        tag: TagRef<'a, Algo>,
    ) -> Result<(), DecryptError> {
        self.algorithm
            .decrypt(plaintext, *self, nonce, aad, ciphertext, tag)
    }
}

impl<'a, Algo: Aead> AsRef<[U8]> for KeyRef<'a, Algo> {
    fn as_ref(&self) -> &[U8] {
        self.key
    }
}

impl<'a, Algo: Aead> AsMut<[U8]> for KeyMut<'a, Algo> {
    fn as_mut(&mut self) -> &mut [U8] {
        self.key
    }
}

impl<'a, Algo> KeyRef<'a, Algo> {
    /// Returns the algorithm this key should be used in
    pub fn algo(&self) -> &Algo {
        &self.algorithm
    }
}

impl<'a, Algo: Aead> KeyMut<'a, Algo> {
    /// Creates a new mutable key for the provided algorithm. Checks that the length is correct.
    pub fn new_for_algo(algo: Algo, key: &'a mut [U8]) -> Result<Self, WrongLengthError> {
        (key.len() == algo.key_len())
            .then_some(KeyMut {
                algorithm: algo,
                key,
            })
            .ok_or(WrongLengthError)
    }
}

impl<'a, Algo> KeyMut<'a, Algo> {
    /// Returns the algorithm this key should be used in
    pub fn algo(&self) -> &Algo {
        &self.algorithm
    }
}

impl<'a, Algo: Aead> TagRef<'a, Algo> {
    /// Creates a new tag for the provided algorithm. Checks that the length is correct.
    pub fn new_for_algo(algo: Algo, tag: &'a [U8]) -> Result<Self, WrongLengthError> {
        (tag.len() == algo.tag_len())
            .then_some(TagRef {
                algorithm: algo,
                tag,
            })
            .ok_or(WrongLengthError)
    }
}

impl<'a, Algo: Aead> AsRef<[U8]> for TagRef<'a, Algo> {
    fn as_ref(&self) -> &[U8] {
        self.tag
    }
}

impl<'a, Algo> TagRef<'a, Algo> {
    /// Returns the algorithm this tag should be used in
    pub fn algo(&self) -> &Algo {
        &self.algorithm
    }
}

impl<'a, Algo: Aead> TagMut<'a, Algo> {
    /// Creates a new mutable tag for the provided algorithm. Checks that the length is correct.
    pub fn new_for_algo(algo: Algo, tag: &'a mut [U8]) -> Result<Self, WrongLengthError> {
        (tag.len() == algo.tag_len())
            .then_some(TagMut {
                algorithm: algo,
                tag,
            })
            .ok_or(WrongLengthError)
    }
}

impl<'a, Algo: Aead> From<TagMut<'a, Algo>> for TagRef<'a, Algo> {
    fn from(tag: TagMut<'a, Algo>) -> Self {
        TagRef {
            algorithm: tag.algorithm,
            tag: tag.tag,
        }
    }
}

impl<'a, Algo> TagMut<'a, Algo> {
    /// Returns the algorithm this mutable tag should be used in
    pub fn algo(&self) -> &Algo {
        &self.algorithm
    }
}

impl<'a, Algo: Aead> NonceRef<'a, Algo> {
    /// Creates a new nonce for the provided algorithm. Checks that the length is correct.
    pub fn new_for_algo(algo: Algo, nonce: &'a [U8]) -> Result<Self, WrongLengthError> {
        (nonce.len() == algo.nonce_len())
            .then_some(NonceRef {
                algorithm: algo,
                nonce,
            })
            .ok_or(WrongLengthError)
    }
}

impl<'a, Algo> NonceRef<'a, Algo> {
    /// Returns the algorithm this nonce should be used in
    pub fn algo(&self) -> &Algo {
        &self.algorithm
    }
}

impl<'a, Algo: Aead> AsRef<[U8]> for NonceRef<'a, Algo> {
    fn as_ref(&self) -> &[U8] {
        self.nonce
    }
}

impl<'a, Algo: Aead> AsMut<[U8]> for TagMut<'a, Algo> {
    fn as_mut(&mut self) -> &mut [U8] {
        self.tag
    }
}

/// This trait is implemented by the multiplexing enum, once per actual algorithm.
/// It allows users (and internal code) to convert between structs with the actual algorithm and
/// the multiplexing enum.
pub trait Multiplexes<Algo>: Aead + Sized {
    /// If self is actually algorithm `Algo`, return that.
    fn mux_algo(&self) -> Option<Algo>;

    /// Create a new multiplexed algorithm value that has `algo` as actual algorithm.
    fn wrap_algo(algo: Algo) -> Self;

    /// Tries unwrapping the algorithm in a key
    fn mux_key<'a>(key: KeyRef<'a, Self>) -> Option<KeyRef<'a, Algo>> {
        let KeyRef { algorithm, key } = key;
        algorithm
            .mux_algo()
            .map(|algorithm| KeyRef { algorithm, key })
    }

    /// Wraps the algorithm in a key
    fn wrap_key<'a>(k: KeyRef<'a, Algo>) -> KeyRef<'a, Self> {
        KeyRef {
            algorithm: Self::wrap_algo(k.algorithm),
            key: k.key,
        }
    }

    /// Tries unwrapping the algorithm in a mutable key
    fn mux_key_mut<'a>(key: KeyMut<'a, Self>) -> Option<KeyMut<'a, Algo>> {
        let KeyMut { algorithm, key } = key;
        algorithm
            .mux_algo()
            .map(|algorithm| KeyMut { algorithm, key })
    }

    /// Wraps the algorithm in a mutable key
    fn wrap_key_mut<'a>(k: KeyMut<'a, Algo>) -> KeyMut<'a, Self> {
        KeyMut {
            algorithm: Self::wrap_algo(k.algorithm),
            key: k.key,
        }
    }

    /// Tries unwrapping the algorithm in a tag
    fn mux_tag<'a>(tag: TagRef<'a, Self>) -> Option<TagRef<'a, Algo>> {
        let TagRef { algorithm, tag } = tag;
        algorithm
            .mux_algo()
            .map(|algorithm| TagRef { algorithm, tag })
    }

    /// Wraps the algorithm in a tag
    fn wrap_tag<'a>(tag: TagRef<'a, Algo>) -> TagRef<'a, Self> {
        TagRef {
            algorithm: Self::wrap_algo(tag.algorithm),
            tag: tag.tag,
        }
    }

    /// Tries unwrapping the algorithm in a mutable tag
    fn mux_tag_mut<'a>(tag: TagMut<'a, Self>) -> Option<TagMut<'a, Algo>> {
        let TagMut { algorithm, tag } = tag;
        algorithm
            .mux_algo()
            .map(|algorithm| TagMut { algorithm, tag })
    }

    /// Wraps the algorithm in a mutable tag
    fn wrap_tag_mut<'a>(tag: TagMut<'a, Algo>) -> TagMut<'a, Self> {
        TagMut {
            algorithm: Self::wrap_algo(tag.algorithm),
            tag: tag.tag,
        }
    }

    /// Tries unwrapping the algorithm in a nonce
    fn mux_nonce<'a>(nonce: NonceRef<'a, Self>) -> Option<NonceRef<'a, Algo>> {
        let NonceRef { algorithm, nonce } = nonce;
        algorithm
            .mux_algo()
            .map(|algorithm| NonceRef { algorithm, nonce })
    }

    /// Wraps the algorithm in a nonce
    fn wrap_nonce<'a>(nonce: NonceRef<'a, Algo>) -> NonceRef<'a, Self> {
        NonceRef {
            algorithm: Self::wrap_algo(nonce.algorithm),
            nonce: nonce.nonce,
        }
    }
}
