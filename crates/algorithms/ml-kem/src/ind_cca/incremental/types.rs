use core::array::from_fn;

use ind_cpa::unpacked::IndCpaPublicKeyUnpacked;

use super::*;
use crate::{
    ind_cca::unpacked::MlKemKeyPairUnpacked,
    ind_cpa::{deserialize_vector, serialize_vector},
    polynomial::{vec_from_bytes, vec_to_bytes},
};

/// Errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid input byte length
    InvalidInputLength,

    /// Invalid output byte length
    InvalidOutputLength,

    /// The public key is not consistent.
    InvalidPublicKey,

    /// Insufficient randomness.
    InsufficientRandomness,
}

/// Incremental trait for unpacked key pairs.
//<const K: usize, Vector: Operations>
pub trait IncrementalKeyPair {
    /// Get the [`PublicKey1`] from this key pair as bytes.
    ///
    /// The output `bytes` have to be at least 64 bytes long.
    fn pk1_bytes(&self, bytes: &mut [u8]) -> Result<(), Error>;

    /// Get the [`PublicKey2`] from this key pair as bytes.
    ///
    /// The output `bytes` have to be at least K * 16 * 32 bytes long.
    ///
    /// **PANICS:** if the output `bytes` are too short.
    fn pk2_bytes(&self, bytes: &mut [u8]);
}

impl<const K: usize, Vector: Operations> IncrementalKeyPair for MlKemKeyPairUnpacked<K, Vector> {
    fn pk1_bytes(&self, bytes: &mut [u8]) -> Result<(), Error> {
        debug_assert!(bytes.len() >= 64);
        if bytes.len() < 64 {
            return Err(Error::InvalidOutputLength);
        }

        bytes[0..32].copy_from_slice(&self.public_key().ind_cpa_public_key.seed_for_A);
        bytes[32..64].copy_from_slice(&self.public_key().public_key_hash);

        Ok(())
    }

    fn pk2_bytes(&self, bytes: &mut [u8]) {
        serialize_vector(&self.public_key.ind_cpa_public_key.t_as_ntt, bytes);
    }
}

/// The incremental public key that allows generating [`Ciphertext1`].
#[derive(Default)]
pub struct PublicKey1 {
    pub(super) seed: [u8; 32],
    pub(super) hash: [u8; 32],
}

impl PublicKey1 {
    /// Get the size of the first public key in bytes.
    pub const fn len() -> usize {
        32 + 32
    }
}

impl TryFrom<&[u8]> for PublicKey1 {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 64 {
            return Err(Error::InvalidInputLength);
        }

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&value[0..32]);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&value[32..64]);
        Ok(Self { seed, hash })
    }
}

impl From<&[u8; 64]> for PublicKey1 {
    fn from(value: &[u8; 64]) -> Self {
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&value[0..32]);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&value[32..64]);
        Self { seed, hash }
    }
}

/// The incremental public key that allows generating [`Ciphertext2`].
///
/// This public key is serialized to safe bytes on the wire.
#[repr(transparent)]
pub struct PublicKey2<const LEN: usize> {
    pub(super) t_as_ntt: [u8; LEN],
}

impl<const LEN: usize> PublicKey2<LEN> {
    /// Get the size of the second public key in bytes.
    pub const fn len() -> usize {
        LEN
    }

    /// Deserialize the public key.
    pub(crate) fn deserialize<const K: usize, Vector: Operations>(
        &self,
    ) -> [PolynomialRingElement<Vector>; K] {
        let mut out = from_fn(|_| PolynomialRingElement::<Vector>::ZERO());
        deserialize_vector(&self.t_as_ntt, &mut out);
        out
    }
}

#[cfg(feature = "alloc")]
pub(crate) mod alloc {
    use super::*;
    use core::any::Any;

    /// Trait container for multiplexing over platform dependent [`MlKemKeyPairUnpacked`].
    pub trait Keys: IncrementalKeyPair {
        fn as_any(&self) -> &dyn Any;
    }
    impl<const K: usize, Vector: Operations + 'static> Keys for MlKemKeyPairUnpacked<K, Vector> {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    /// Trait container for multiplexing over platform dependent [`EncapsState`].
    pub trait State {
        fn as_any(&self) -> &dyn Any;
    }

    impl<const K: usize, Vector: Operations + 'static> State for EncapsState<K, Vector> {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}

/// The partial ciphertext c1 - first part.
#[repr(transparent)]
pub struct Ciphertext1<const LEN: usize> {
    pub value: [u8; LEN],
}

impl<const LEN: usize> Ciphertext1<LEN> {
    /// The size of the ciphertext.
    pub const fn len() -> usize {
        LEN
    }
}

/// The partial ciphertext c2 - second part.
#[repr(transparent)]
pub struct Ciphertext2<const LEN: usize> {
    pub value: [u8; LEN],
}

impl<const LEN: usize> Ciphertext2<LEN> {
    /// The size of the ciphertext.
    pub const fn len() -> usize {
        LEN
    }
}

/// The incremental state for encapsulate.
pub struct EncapsState<const K: usize, Vector: Operations> {
    pub(super) r_as_ntt: [PolynomialRingElement<Vector>; K],
    pub(super) error2: PolynomialRingElement<Vector>,
    pub(super) randomness: [u8; 32],
}

impl<const K: usize, Vector: Operations> EncapsState<K, Vector> {
    /// Get the number of bytes, required for the state.
    pub const fn num_bytes() -> usize {
        vec_len_bytes::<K, Vector>() + PolynomialRingElement::<Vector>::num_bytes() + 32
    }

    /// Get the state as bytes
    pub fn to_bytes(self, state: &mut [u8]) -> Result<(), Error> {
        debug_assert!(state.len() >= Self::num_bytes());
        if state.len() < Self::num_bytes() {
            return Err(Error::InvalidOutputLength);
        }

        let mut offset = 0;
        vec_to_bytes(&self.r_as_ntt, &mut state[offset..]);
        offset += vec_len_bytes::<K, Vector>();

        self.error2.to_bytes(&mut state[offset..]);
        offset += PolynomialRingElement::<Vector>::num_bytes();

        state[offset..offset + 32].copy_from_slice(&self.randomness);

        Ok(())
    }

    /// Build a state from bytes
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        debug_assert!(bytes.len() >= Self::num_bytes());
        if bytes.len() < Self::num_bytes() {
            return Err(Error::InvalidInputLength);
        }

        let mut offset = 0;
        let mut r_as_ntt = from_fn(|_| PolynomialRingElement::<Vector>::ZERO());
        vec_from_bytes(&bytes[offset..], &mut r_as_ntt);
        offset += vec_len_bytes::<K, Vector>();

        let error2 = PolynomialRingElement::<Vector>::from_bytes(&bytes[offset..]);
        offset += PolynomialRingElement::<Vector>::num_bytes();

        let mut randomness = [0u8; 32];
        randomness.copy_from_slice(&bytes[offset..offset + 32]);

        Ok(Self {
            r_as_ntt,
            error2,
            randomness,
        })
    }

    /// Build a state from bytes
    pub fn from_bytes<const STATE_LEN: usize>(bytes: &[u8; STATE_LEN]) -> Self {
        // Unwrapping here is safe because we know it's the correct size.
        Self::try_from_bytes(bytes).unwrap()
    }
}

// === Implementations

/// Convert [`MlKemPublicKeyUnpacked`] to a [`PublicKey1`]
impl<const K: usize, Vector: Operations> From<&MlKemPublicKeyUnpacked<K, Vector>> for PublicKey1 {
    fn from(pk: &MlKemPublicKeyUnpacked<K, Vector>) -> Self {
        Self {
            seed: pk.ind_cpa_public_key.seed_for_A,
            hash: pk.public_key_hash,
        }
    }
}

/// Convert [`MlKemPublicKeyUnpacked`] to a [`PublicKey2`].
impl<const K: usize, const LEN: usize, Vector: Operations> From<&MlKemPublicKeyUnpacked<K, Vector>>
    for PublicKey2<LEN>
{
    fn from(pk: &MlKemPublicKeyUnpacked<K, Vector>) -> Self {
        let mut out = Self {
            t_as_ntt: [0u8; LEN],
        };
        serialize_vector(&pk.ind_cpa_public_key.t_as_ntt, &mut out.t_as_ntt);
        out
    }
}

/// Convert a byte slice `&[u8]` to a [`PublicKey2`].
impl<const LEN: usize> TryFrom<&[u8]> for PublicKey2<LEN> {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < LEN {
            return Err(Error::InvalidInputLength);
        }

        let mut t_as_ntt = [0u8; LEN];
        t_as_ntt.copy_from_slice(&value[0..LEN]);
        Ok(Self { t_as_ntt })
    }
}

/// Convert bytes `&[u8; LEN]` to a [`PublicKey2`].
impl<const LEN: usize> From<&[u8; LEN]> for PublicKey2<LEN> {
    fn from(value: &[u8; LEN]) -> Self {
        let mut t_as_ntt = [0u8; LEN];
        t_as_ntt.copy_from_slice(&value[0..LEN]);
        Self { t_as_ntt }
    }
}

// The key pair struct that we (de)serialize.
pub struct KeyPair<const K: usize, const PK2_LEN: usize, Vector: Operations> {
    pk1: PublicKey1,
    pk2: PublicKey2<PK2_LEN>,
    sk: MlKemPrivateKeyUnpacked<K, Vector>,
    matrix: [[PolynomialRingElement<Vector>; K]; K],
}

impl<const K: usize, const PK2_LEN: usize, Vector: Operations> From<MlKemKeyPairUnpacked<K, Vector>>
    for KeyPair<K, PK2_LEN, Vector>
{
    fn from(kp: MlKemKeyPairUnpacked<K, Vector>) -> Self {
        KeyPair {
            pk1: PublicKey1::from(kp.public_key()),
            pk2: PublicKey2::from(kp.public_key()),
            sk: kp.private_key,
            matrix: kp.public_key.ind_cpa_public_key.A,
        }
    }
}

impl<const K: usize, const PK2_LEN: usize, Vector: Operations> From<KeyPair<K, PK2_LEN, Vector>>
    for MlKemKeyPairUnpacked<K, Vector>
{
    fn from(value: KeyPair<K, PK2_LEN, Vector>) -> Self {
        let mut t_as_ntt = from_fn(|_| PolynomialRingElement::<Vector>::ZERO());
        deserialize_vector(&value.pk2.t_as_ntt, &mut t_as_ntt);

        MlKemKeyPairUnpacked {
            private_key: value.sk,
            public_key: MlKemPublicKeyUnpacked {
                ind_cpa_public_key: IndCpaPublicKeyUnpacked {
                    t_as_ntt,
                    seed_for_A: value.pk1.seed,
                    A: value.matrix,
                },
                public_key_hash: value.pk1.hash,
            },
        }
    }
}

/// Write `value` into `out` at `offset`.
#[inline(always)]
fn write(out: &mut [u8], value: &[u8], offset: &mut usize) {
    let new_offset = *offset + value.len();
    out[*offset..new_offset].copy_from_slice(value);
    *offset = new_offset;
}

impl<const K: usize, const PK2_LEN: usize, Vector: Operations> KeyPair<K, PK2_LEN, Vector> {
    /// Get [`PublicKey1`] as bytes.
    pub fn pk1_bytes(&self, pk1: &mut [u8]) -> Result<(), Error> {
        debug_assert!(pk1.len() >= PublicKey1::len());
        if pk1.len() < PublicKey1::len() {
            return Err(Error::InvalidOutputLength);
        }

        pk1[0..32].copy_from_slice(&self.pk1.seed);
        pk1[32..64].copy_from_slice(&self.pk1.hash);

        Ok(())
    }

    /// Get [`PublicKey2`] as bytes.
    pub fn pk2_bytes(&self, pk2: &mut [u8]) -> Result<(), Error> {
        if pk2.len() < PK2_LEN {
            return Err(Error::InvalidOutputLength);
        }

        pk2[0..PK2_LEN].copy_from_slice(&self.pk2.t_as_ntt);

        Ok(())
    }

    /// The byte size of this key pair.
    pub const fn num_bytes() -> usize {
        PublicKey1::len() + PublicKey2::<PK2_LEN>::len()
        // sk length
        + vec_len_bytes::<K, Vector>() + 32
        // matrix length
        + K * vec_len_bytes::<K, Vector>()
    }

    /// Write this key pair into the `key` bytes.
    ///
    /// `key` must be at least of length `num_bytes()`
    pub fn to_bytes(&self, key: &mut [u8]) -> Result<(), Error> {
        debug_assert!(key.len() >= Self::num_bytes());
        if key.len() < Self::num_bytes() {
            return Err(Error::InvalidInputLength);
        }

        let mut offset = 0;

        // PK1
        write(key, &self.pk1.seed, &mut offset);
        write(key, &self.pk1.hash, &mut offset);

        // PK2
        write(key, &self.pk2.t_as_ntt, &mut offset);

        // SK
        vec_to_bytes(
            &self.sk.ind_cpa_private_key.secret_as_ntt,
            &mut key[offset..],
        );
        offset += vec_len_bytes::<K, Vector>();
        write(key, &self.sk.implicit_rejection_value, &mut offset);

        // Matrix
        for i in 0..self.matrix.len() {
            vec_to_bytes(&self.matrix[i], &mut key[offset..]);
            offset += vec_len_bytes::<K, Vector>();
        }

        Ok(())
    }

    /// Write this key pair into the `key` bytes.
    /// This is the compressed private key.
    ///
    /// `key` must be at least of length secret key size
    ///
    /// Layout: dk | ek | H(ek) | z
    pub fn to_bytes_compressed<const KEY_SIZE: usize, const VEC_SIZE: usize>(
        &self,
        key: &mut [u8; KEY_SIZE],
    ) {
        // Write the private key.
        // This is a manual version of serialize_kem_secret_key_mut that skips
        // the hash.
        serialize_vector(&self.sk.ind_cpa_private_key.secret_as_ntt, key);
        let mut offset = VEC_SIZE;

        // ek = t | ⍴
        write(key, &self.pk2.t_as_ntt, &mut offset);
        write(key, &self.pk1.seed, &mut offset);

        write(key, &self.pk1.hash, &mut offset);
        write(key, &self.sk.implicit_rejection_value, &mut offset);
    }

    /// Read a key pair from the `key` bytes.
    ///
    /// `key` must be at least of length `num_bytes()`
    pub fn from_bytes(key: &[u8]) -> Result<Self, Error> {
        debug_assert!(key.len() >= Self::num_bytes());
        if key.len() < Self::num_bytes() {
            return Err(Error::InvalidInputLength);
        }

        // PK1
        let pk1 = PublicKey1::try_from(key)?;
        let mut offset = PublicKey1::len();

        // PK2
        let pk2 = PublicKey2::try_from(&key[offset..])?;
        offset += PublicKey2::<PK2_LEN>::len();

        // SK
        let implicit_rejection_value = [0u8; 32];
        let mut sk: MlKemPrivateKeyUnpacked<K, Vector> = MlKemPrivateKeyUnpacked {
            ind_cpa_private_key: IndCpaPrivateKeyUnpacked::default(),
            implicit_rejection_value,
        };
        vec_from_bytes(&key[offset..], &mut sk.ind_cpa_private_key.secret_as_ntt);
        offset += vec_len_bytes::<K, Vector>();
        sk.implicit_rejection_value
            .copy_from_slice(&key[offset..offset + 32]);
        offset += sk.implicit_rejection_value.len();

        // Matrix
        let mut matrix = [[PolynomialRingElement::<Vector>::ZERO(); K]; K];
        for i in 0..matrix.len() {
            vec_from_bytes(&key[offset..], &mut matrix[i]);
            offset += vec_len_bytes::<K, Vector>();
        }

        Ok(Self {
            pk1,
            pk2,
            sk,
            matrix,
        })
    }
}
