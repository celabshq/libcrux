use core::array::from_fn;

#[allow(unused_imports)]
use hax_lib::{int::ToInt, prop::ToProp};
use ind_cpa::unpacked::IndCpaPublicKeyUnpacked;

use super::*;
use crate::{
    ind_cca::unpacked::MlKemKeyPairUnpacked,
    ind_cpa::{deserialize_vector, serialize_vector},
    polynomial::{
        matrix_within_field_bound, poly_within_field_bound, polyvec_within_field_bound,
        vec_from_bytes, vec_to_bytes,
    },
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

    /// Input bytes failed validation (coefficient out of range).
    InvalidInput,
}

/// Incremental trait for unpacked key pairs.
//<const K: usize, Vector: Operations>
#[hax_lib::attributes]
pub trait IncrementalKeyPair {
    /// Get the [`PublicKey1`] from this key pair as bytes.
    ///
    /// The output `bytes` have to be at least 64 bytes long.
    #[requires(bytes.len() >= 64)]
    fn pk1_bytes(&self, bytes: &mut [u8]) -> Result<(), Error>;

    /// Get the [`PublicKey2`] from this key pair as bytes.
    ///
    /// The output `bytes` have to be at least K * 16 * 32 bytes long.
    ///
    /// **PANICS:** if the output `bytes` are too short.
    fn pk2_bytes(&self, bytes: &mut [u8]);
}

#[hax_lib::attributes]
impl<const K: usize, Vector: Operations> IncrementalKeyPair for MlKemKeyPairUnpacked<K, Vector> {
    #[requires(bytes.len() >= 64)]
    fn pk1_bytes(&self, bytes: &mut [u8]) -> Result<(), Error> {
        debug_assert!(bytes.len() >= 64);
        if bytes.len() < 64 {
            return Err(Error::InvalidOutputLength);
        }

        bytes[0..32].copy_from_slice(&self.public_key().ind_cpa_public_key.seed_for_A);
        bytes[32..64].copy_from_slice(&self.public_key().public_key_hash);

        Ok(())
    }

    #[requires(
        (hacspec_ml_kem::parameters::is_rank(K)
        && bytes.len() == hacspec_ml_kem::parameters::ranked_bytes_per_ring_element(K))
            .to_prop()
        & crate::polynomial::spec::is_bounded_polynomial_vector(
            3328,
            &self.public_key.ind_cpa_public_key.t_as_ntt,
        )
    )]
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

#[hax_lib::attributes]
impl PublicKey1 {
    /// Get the size of the first public key in bytes.
    #[ensures(|result| result == 64)]
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

#[hax_lib::attributes]
impl<const LEN: usize> PublicKey2<LEN> {
    /// Get the size of the second public key in bytes.
    #[ensures(|result| result == LEN)]
    pub const fn len() -> usize {
        LEN
    }

    /// Deserialize the public key.
    #[requires(hacspec_ml_kem::parameters::is_rank(K)
        && LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K))]
    // Honest bound: `deserialize_vector` runs the non-reduced ByteDecode_12
    // (lanes [0,4095] = is_i16b 4096), so this public-key vector is 4096-,
    // not 3328-bounded; the encap consumer (compute_ring_element_v) accepts it.
    #[ensures(|result| crate::polynomial::spec::is_bounded_polynomial_vector(4096, &result))]
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

#[hax_lib::attributes]
impl<const K: usize, Vector: Operations> EncapsState<K, Vector> {
    /// Get the number of bytes, required for the state.
    #[requires(K <= 4)]
    #[ensures(|result| result == K * 512 + 512 + 32)]
    pub const fn num_bytes() -> usize {
        vec_len_bytes::<K, Vector>() + PolynomialRingElement::<Vector>::num_bytes() + 32
    }

    /// Get the state as bytes
    #[requires(K <= 4 && state.len() >= K * 512 + 512 + 32)]
    #[ensures(|result| result.is_ok() && future(state).len() == state.len())]
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

    /// Build a state from bytes.
    ///
    /// Returns [`Error::InvalidInput`] if any decoded coefficient is out of
    /// field range.
    #[requires(K <= 4 && bytes.len() >= K * 512 + 512 + 32)]
    #[ensures(|result| match result {
        Ok(state) => crate::polynomial::spec::is_bounded_polynomial_vector(3328, &state.r_as_ntt)
            & crate::polynomial::spec::is_bounded_poly(3328, &state.error2),
        Err(_) => true.to_prop(),
    })]
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

        // Validate the raw-decoded coefficients: arbitrary bytes decode to
        // arbitrary i16 values, but the encapsulation arithmetic is only
        // overflow-safe for coefficients in [-3328, 3328].
        if !polyvec_within_field_bound(&r_as_ntt) || !poly_within_field_bound(&error2) {
            return Err(Error::InvalidInput);
        }

        Ok(Self {
            r_as_ntt,
            error2,
            randomness,
        })
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
        // PROOF GAP (admitted): `Core_models.Convert.t_From` forces every
        // instance precondition to be trivial (`pred: Type0{true ==> pred}`),
        // but `serialize_vector` requires `is_rank(K)`,
        // `LEN == ranked_bytes_per_ring_element(K)` and
        // `is_bounded_polynomial_vector(3328, t_as_ntt)`.
        hax_lib::fstar!("admit ()");
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
        // PROOF GAP (admitted): `Core_models.Convert.t_From` forces every
        // instance precondition to be trivial (`pred: Type0{true ==> pred}`),
        // so `into_unpacked`'s requires (`is_rank(K)`,
        // `PK2_LEN == cpa_private_key_size(K)`, coefficient bounds) cannot
        // be discharged here.  Annotated callers should use
        // `KeyPair::into_unpacked` directly.
        hax_lib::fstar!("admit ()");
        value.into_unpacked()
    }
}

/// Write `value` into `out` at `offset`.
#[inline(always)]
#[hax_lib::requires(value.len() <= out.len() && *offset <= out.len() - value.len())]
#[hax_lib::ensures(|_| future(out).len() == out.len()
    && *future(offset) == *offset + value.len())]
fn write(out: &mut [u8], value: &[u8], offset: &mut usize) {
    let new_offset = *offset + value.len();
    out[*offset..new_offset].copy_from_slice(value);
    *offset = new_offset;
}

#[hax_lib::attributes]
impl<const K: usize, const PK2_LEN: usize, Vector: Operations> KeyPair<K, PK2_LEN, Vector> {
    /// Get [`PublicKey1`] as bytes.
    #[requires(pk1.len() >= 64)]
    #[ensures(|result| result.is_ok() && future(pk1).len() == pk1.len())]
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
    #[ensures(|result| hax_lib::implies(pk2.len() >= PK2_LEN, result.is_ok())
        & (future(pk2).len() == pk2.len()))]
    pub fn pk2_bytes(&self, pk2: &mut [u8]) -> Result<(), Error> {
        if pk2.len() < PK2_LEN {
            return Err(Error::InvalidOutputLength);
        }

        pk2[0..PK2_LEN].copy_from_slice(&self.pk2.t_as_ntt);

        Ok(())
    }

    /// The byte size of this key pair.
    #[requires(K <= 4 && PK2_LEN <= 1536)]
    #[ensures(|result| result == 64 + PK2_LEN + K * 512 + 32 + K * K * 512)]
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
    // The ground `K == 2 || ..` domain (rather than `K <= 4`) lets Z3
    // case-split K to literals, linearizing the `i * (K * 512)` products
    // in the matrix-loop invariant.
    #[requires((K == 2 || K == 3 || K == 4) && PK2_LEN <= 1536
        && key.len() >= 64 + PK2_LEN + K * 512 + 32 + K * K * 512)]
    #[ensures(|result| result.is_ok() && future(key).len() == key.len())]
    pub fn to_bytes(&self, key: &mut [u8]) -> Result<(), Error> {
        debug_assert!(key.len() >= Self::num_bytes());
        if key.len() < Self::num_bytes() {
            return Err(Error::InvalidInputLength);
        }

        #[cfg(hax)]
        let _key_len = key.len();

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
            // The leading `i <= K` conjunct bounds the (rebound, bare-usize)
            // loop index so the `i * (K * 512)` machine product is provably
            // overflow-free (lazy && checks later conjuncts under earlier
            // ones).
            hax_lib::loop_invariant!(|i: usize| {
                key.len() == _key_len
                    && i <= K
                    && offset == 64 + PK2_LEN + K * 512 + 32 + i * (K * 512)
            });
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
        // PROOF GAP (admitted): `serialize_vector` requires
        // `out.len() == ranked_bytes_per_ring_element(K)` exactly, but is
        // handed the full KEY_SIZE buffer here; it also requires
        // `is_bounded_polynomial_vector(3328, secret_as_ntt)`, which is not
        // available from this type.
        hax_lib::fstar!("admit ()");
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
    /// `key` must be at least of length `num_bytes()`.
    ///
    /// Returns [`Error::InvalidInput`] if any decoded coefficient is out of
    /// field range.
    // Ground K domain for the same reason as in `to_bytes`.
    #[requires((K == 2 || K == 3 || K == 4) && PK2_LEN <= 1536
        && key.len() >= 64 + PK2_LEN + K * 512 + 32 + K * K * 512)]
    #[ensures(|result| match result {
        Ok(kp) => crate::polynomial::spec::is_bounded_polynomial_vector(
            3328,
            &kp.sk.ind_cpa_private_key.secret_as_ntt,
        ) & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, &kp.matrix),
        Err(_) => true.to_prop(),
    })]
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
            // `i <= K` bounds the loop index for the same reason as in
            // `to_bytes` above.
            hax_lib::loop_invariant!(|i: usize| {
                i <= K && offset == 64 + PK2_LEN + K * 512 + 32 + i * (K * 512)
            });
            vec_from_bytes(&key[offset..], &mut matrix[i]);
            offset += vec_len_bytes::<K, Vector>();
        }

        // Validate the raw-decoded coefficients: arbitrary bytes decode to
        // arbitrary i16 values, but the decapsulation arithmetic is only
        // overflow-safe for coefficients in [-3328, 3328].  (pk2 stays
        // byte-encoded here; its 12-bit decode is bounded by construction.)
        if !polyvec_within_field_bound(&sk.ind_cpa_private_key.secret_as_ntt)
            || !matrix_within_field_bound(&matrix)
        {
            return Err(Error::InvalidInput);
        }

        Ok(Self {
            pk1,
            pk2,
            sk,
            matrix,
        })
    }

    /// Convert this key pair into an unpacked key pair.
    ///
    /// This is the annotated home of the `From<KeyPair> for
    /// MlKemKeyPairUnpacked` conversion: hax forces trivial preconditions on
    /// `From` instances, so the contract lives here and the instance
    /// delegates.
    #[requires(
        (hacspec_ml_kem::parameters::is_rank(K)
        && PK2_LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K))
            .to_prop()
        & crate::polynomial::spec::is_bounded_polynomial_vector(
            3328,
            &self.sk.ind_cpa_private_key.secret_as_ntt,
        )
        & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, &self.matrix)
    )]
    #[ensures(|result|
        crate::polynomial::spec::is_bounded_polynomial_vector(
            3328,
            &result.private_key.ind_cpa_private_key.secret_as_ntt,
        )
        & crate::polynomial::spec::is_bounded_polynomial_matrix(
            3328,
            &result.public_key.ind_cpa_public_key.A,
        )
        // t_as_ntt is deserialized non-reduced (ByteDecode_12, lanes [0,4095]):
        // honest 4096 bound.  secret_as_ntt (from validated self.sk) and A
        // (keygen-sampled) remain genuinely 3328.
        & crate::polynomial::spec::is_bounded_polynomial_vector(
            4096,
            &result.public_key.ind_cpa_public_key.t_as_ntt,
        )
    )]
    pub(crate) fn into_unpacked(self) -> MlKemKeyPairUnpacked<K, Vector> {
        let mut t_as_ntt = from_fn(|_| PolynomialRingElement::<Vector>::ZERO());
        deserialize_vector(&self.pk2.t_as_ntt, &mut t_as_ntt);

        MlKemKeyPairUnpacked {
            private_key: self.sk,
            public_key: MlKemPublicKeyUnpacked {
                ind_cpa_public_key: IndCpaPublicKeyUnpacked {
                    t_as_ntt,
                    seed_for_A: self.pk1.seed,
                    A: self.matrix,
                },
                public_key_hash: self.pk1.hash,
            },
        }
    }
}
