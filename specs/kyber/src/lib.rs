// This module is declared here since otherwise, hax reports the following error:
//
// The THIR body of item
// DefId(0:313 ~ hacspec_kyber[4591]::parameters::KyberFieldElement::{constant#0})
// was stolen.
//
// This is being tracked in https://github.com/hacspec/hacspec-v2/issues/27
mod parameters;

use hacspec_lib::{ArrayConversion, UpdatingArray2};
use parameters::{
    hash_functions::{G, H, H_DIGEST_SIZE, J},
    CPA_PKE_KEY_GENERATION_SEED_SIZE, CPA_PKE_MESSAGE_SIZE, CPA_PKE_PUBLIC_KEY_SIZE,
    CPA_PKE_SECRET_KEY_SIZE,
};

mod compress;
mod ind_cpa;
mod matrix;
mod ntt;
mod sampling;
mod serialize;

pub const KYBER768_SHARED_SECRET_SIZE: usize = CPA_PKE_MESSAGE_SIZE;

pub const KYBER768_KEY_GENERATION_SEED_SIZE: usize =
    CPA_PKE_KEY_GENERATION_SEED_SIZE + KYBER768_SHARED_SECRET_SIZE;

pub const KYBER768_PUBLIC_KEY_SIZE: usize = CPA_PKE_PUBLIC_KEY_SIZE;

pub const KYBER768_SECRET_KEY_SIZE: usize =
    CPA_PKE_SECRET_KEY_SIZE + CPA_PKE_PUBLIC_KEY_SIZE + H_DIGEST_SIZE + KYBER768_SHARED_SECRET_SIZE;

pub const KYBER768_CIPHERTEXT_SIZE: usize = parameters::CPA_PKE_CIPHERTEXT_SIZE;

pub type PublicKey = [u8; KYBER768_PUBLIC_KEY_SIZE];
pub type PrivateKey = [u8; KYBER768_SECRET_KEY_SIZE];

pub type Ciphertext = [u8; KYBER768_CIPHERTEXT_SIZE];
pub type SharedSecret = [u8; KYBER768_SHARED_SECRET_SIZE];

#[derive(Debug)]
pub struct BadRejectionSamplingRandomnessError;

#[derive(Debug)]
pub struct KeyPair {
    pk: PublicKey,
    sk: PrivateKey,
}

impl KeyPair {
    pub fn new(pk: PublicKey, sk: PrivateKey) -> Self {
        Self { pk, sk }
    }

    pub fn pk(&self) -> &PublicKey {
        &self.pk
    }

    pub fn sk(&self) -> &PrivateKey {
        &self.sk
    }
}

/// This function implements most of <strong>Algorithm 15</strong> of the
/// NIST FIPS 203 specification; this is the Kyber CCA-KEM key generation algorithm.
///
/// We say "most of" since Algorithm 15 samples the required randomness within
/// the function itself, whereas this implementation expects it to be provided
/// through the `randomness` parameter.
///
/// Algorithm 15 is reproduced below:
///
/// ```plaintext
/// Output: encapsulation key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Output: decapsulation key dkₚₖₑ ∈ 𝔹^{768k+96}.
///
/// z ←$ 𝔹³²
/// (ekₚₖₑ, dkₚₖₑ) ← K-PKE.KeyGen()
/// ek ← ekₚₖₑ
/// dk ← (dkₚₖₑ ‖ ek ‖ H(ek) ‖ z)
/// return (ek, dk)
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
pub fn generate_keypair(
    randomness: [u8; KYBER768_KEY_GENERATION_SEED_SIZE],
) -> Result<KeyPair, BadRejectionSamplingRandomnessError> {
    let ind_cpa_keypair_randomness = &randomness[0..parameters::CPA_PKE_KEY_GENERATION_SEED_SIZE];
    let implicit_rejection_value = &randomness[parameters::CPA_PKE_KEY_GENERATION_SEED_SIZE..];

    // (ekₚₖₑ, dkₚₖₑ) ← K-PKE.KeyGen()
    let ind_cpa_key_pair = ind_cpa::generate_keypair(&ind_cpa_keypair_randomness.as_len_array())?;

    // dk ← (dkₚₖₑ ‖ ek ‖ H(ek) ‖ z)
    let secret_key_serialized =
        ind_cpa_key_pair.serialize_secret_key(&implicit_rejection_value.as_len_array());

    // return (ek, dk)
    let key_pair = KeyPair::new(ind_cpa_key_pair.pk(), secret_key_serialized);
    Ok(key_pair)
}

fn public_key_modulus_check(public_key: &PublicKey) -> bool {
    let encoded_ring_elements = &public_key[0..KYBER768_PUBLIC_KEY_SIZE - 32];
    let decoded_ring_elements =
        serialize::vector_decode_12(encoded_ring_elements.try_into().unwrap());

    encoded_ring_elements == serialize::vector_encode_12(decoded_ring_elements)
}

/// This function implements most of <strong>Algorithm 16</strong> of the
/// NIST FIPS 203 specification; this is the Kyber CCA-KEM encapsulation algorithm.
///
/// We say "most of" since Algorithm 16 samples the required randomness within
/// the function itself, whereas this implementation expects it to be provided
/// through the `randomness` parameter.
///
/// Algorithm 16 is reproduced below:
///
/// ```plaintext
/// Validated input: encapsulation key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Output: shared key K ∈ 𝔹³².
/// Output: ciphertext c ∈ 𝔹^{32(dᵤk+dᵥ)}.
///
/// Input validation step 1. (Type check.) If ek is not an array of bytes of length
/// 384k + 32 for the value of k specifed by the relevant parameter set, the input
/// is invalid.
///
/// Input validation step 2. (Modulus check.) Perform the computation
/// ek' ← ByteEncode₁₂(ByteDecode₁₂(ek)). If ek' ≠ ek, the input is invalid.
///
/// m ←$ 𝔹³²
/// (K,r) ← G(m ‖ H(ek))
/// c ← K-PKE.Encrypt(ek, m, r)
/// return(K,c)
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
pub fn encapsulate(
    // Input validation step 1 is performed by specifying the type of
    // |public_key| to be |PublicKey|
    public_key: PublicKey,
    randomness: [u8; KYBER768_SHARED_SECRET_SIZE],
) -> Result<(Ciphertext, SharedSecret), BadRejectionSamplingRandomnessError> {
    // Input validation step 2
    assert!(public_key_modulus_check(&public_key));

    // (K,r) ← G(m ‖ H(ek))
    let to_hash: [u8; 2 * H_DIGEST_SIZE] = randomness.push(&H(&public_key));

    let hashed = G(&to_hash);
    let (shared_secret, pseudorandomness) = hashed.split_at(KYBER768_SHARED_SECRET_SIZE);

    // c ← K-PKE.Encrypt(ek, m, r)
    let ciphertext = ind_cpa::encrypt(&public_key, randomness, &pseudorandomness.as_len_array())?;

    // return(K,c)
    Ok((ciphertext, shared_secret.as_len_array()))
}

/// This function implements <strong>Algorithm 17</strong> of the
/// NIST FIPS 203 specification; this is the Kyber CCA-KEM encapsulation algorithm.
///
/// Algorithm 17 is reproduced below:
///
/// ```plaintext
/// Validated input: ciphertext c ∈ 𝔹^{32(dᵤk+dᵥ)}.
/// Validated input: decapsulation key dkₚₖₑ ∈ 𝔹^{768k+96}.
/// Output: shared key K ∈ 𝔹^{384k+32}.
///
/// Input validation step 1. (Ciphertext type check.) If c is not a byte array
/// of length 32(dᵤk+dᵥ) for the values of dᵤ, dᵥ, and k specifed by the relevant
/// parameter set, the input is invalid.
///
/// Input validation step 2. (Decapsulation key type check.) If dk is not a byte
/// array of length 768k + 96 for the value of k specifed by the relevant
/// parameter set, the input is invalid.
///
/// dkₚₖₑ ← dk[0 : 384k]
/// ekₚₖₑ ← dk[384k : 768k + 32]
/// h ← dk[768k + 32 : 768k + 64]
/// z ← dk[768k + 64 : 768k + 96]
/// m′ ← K-PKE.Decrypt(dkₚₖₑ,c)
/// (K′,r′) ← G(m′ ‖ h)
/// K̃ ← J(z‖c, 32)
/// c′ ← K-PKE.Encrypt(ekₚₖₑ, m′, r′)
/// if c ≠ c′ then
///     K′ ← K̃
/// end if
/// return K′
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
pub fn decapsulate(
    // Input validation step 1 is performed by specifying the type of
    // |ciphertext| to be |Ciphertext|
    ciphertext: Ciphertext,
    // Input validation step 2 is performed by specifying the type of
    // |secret_key| to be |PrivateKey|
    secret_key: PrivateKey,
) -> SharedSecret {
    // dkₚₖₑ ← dk[0 : 384k]
    let (ind_cpa_secret_key, secret_key) = secret_key.split_at(CPA_PKE_SECRET_KEY_SIZE);

    // ekₚₖₑ ← dk[384k : 768k + 32]
    let (ind_cpa_public_key, secret_key) = secret_key.split_at(CPA_PKE_PUBLIC_KEY_SIZE);

    // h ← dk[768k + 32 : 768k + 64]
    // z ← dk[768k + 64 : 768k + 96]
    let (ind_cpa_public_key_hash, implicit_rejection_value) = secret_key.split_at(H_DIGEST_SIZE);

    // m′ ← K-PKE.Decrypt(dkₚₖₑ,c)
    let decrypted = ind_cpa::decrypt(&ind_cpa_secret_key.as_len_array(), &ciphertext);

    // (K′,r′) ← G(m′ ‖ h)
    let to_hash: [u8; CPA_PKE_MESSAGE_SIZE + H_DIGEST_SIZE] =
        decrypted.push(ind_cpa_public_key_hash);
    let hashed = G(&to_hash);
    let (success_shared_secret, pseudorandomness) = hashed.split_at(KYBER768_SHARED_SECRET_SIZE);

    // K̃ ← J(z‖c, 32)
    let to_hash: [u8; KYBER768_SHARED_SECRET_SIZE + KYBER768_CIPHERTEXT_SIZE] =
        implicit_rejection_value.push(&ciphertext);
    let rejection_shared_secret: [u8; KYBER768_SHARED_SECRET_SIZE] = J(&to_hash);

    // c′ ← K-PKE.Encrypt(ekₚₖₑ, m′, r′)
    let reencrypted_ciphertext = ind_cpa::encrypt(
        &ind_cpa_public_key.as_len_array(),
        decrypted,
        &pseudorandomness.as_len_array(),
    );

    // if c ≠ c′ then
    //     K′ ← K̃
    // end if
    // return K′
    if let Ok(reencrypted) = reencrypted_ciphertext {
        if ciphertext == reencrypted {
            success_shared_secret.as_len_array()
        } else {
            rejection_shared_secret
        }
    } else {
        rejection_shared_secret
    }
}

#[cfg(test)]
mod tests {
    use proptest::collection::vec;
    use proptest::prelude::*;

    use super::*;

    use parameters::{
        hash_functions::{H_DIGEST_SIZE, J},
        CPA_PKE_PUBLIC_KEY_SIZE, CPA_PKE_SECRET_KEY_SIZE,
    };

    const IMPLICIT_REJECTION_VALUE_POSITION: usize =
        CPA_PKE_SECRET_KEY_SIZE + CPA_PKE_PUBLIC_KEY_SIZE + H_DIGEST_SIZE;

    pub fn calculate_rejection_secret(
        secret_key: PrivateKey,
        ciphertext: Ciphertext,
    ) -> SharedSecret {
        let mut to_hash = secret_key[IMPLICIT_REJECTION_VALUE_POSITION..].to_vec();
        to_hash.extend_from_slice(&ciphertext);

        J(&to_hash)
    }

    proptest! {
        #[test]
        fn consistency(key_generation_randomness in vec(any::<u8>(), KYBER768_KEY_GENERATION_SEED_SIZE), encapsulation_randomness in vec(any::<u8>(), KYBER768_SHARED_SECRET_SIZE)) {
            if let Ok(key_pair) = generate_keypair(key_generation_randomness.try_into().unwrap()) {
                if let Ok((ciphertext, shared_secret)) = encapsulate(key_pair.pk, encapsulation_randomness.try_into().unwrap()) {
                    let shared_secret_decapsulated = decapsulate(ciphertext, key_pair.sk);

                    assert_eq!(shared_secret, shared_secret_decapsulated);
                }
            }
            // If the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }

        #[test]
        fn modified_ciphertext(
            key_generation_randomness in vec(any::<u8>(), KYBER768_KEY_GENERATION_SEED_SIZE),
            encapsulation_randomness in vec(any::<u8>(), KYBER768_SHARED_SECRET_SIZE),

            ciphertext_position in 0usize..KYBER768_CIPHERTEXT_SIZE,
            random_byte in 1u8..u8::MAX
            ) {
            if let Ok(key_pair) = generate_keypair(key_generation_randomness.try_into().unwrap()) {
                if let Ok((mut ciphertext, shared_secret)) = encapsulate(key_pair.pk, encapsulation_randomness.try_into().unwrap()) {
                    ciphertext[ciphertext_position] ^= random_byte;
                    let shared_secret_decapsulated = decapsulate(ciphertext, key_pair.sk);

                    assert_ne!(shared_secret, shared_secret_decapsulated);

                    let implicit_rejection_secret = calculate_rejection_secret(key_pair.sk, ciphertext);
                    assert_eq!(shared_secret_decapsulated, implicit_rejection_secret);

                }
            }
            // If the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }

        #[test]
        fn modified_secret_key(
            key_generation_randomness in vec(any::<u8>(), KYBER768_KEY_GENERATION_SEED_SIZE),
            encapsulation_randomness in vec(any::<u8>(), KYBER768_SHARED_SECRET_SIZE),

            secret_key_position in 0usize..(KYBER768_SECRET_KEY_SIZE - KYBER768_SHARED_SECRET_SIZE),
            random_byte in 1u8..u8::MAX
            ) {
            if let Ok(mut key_pair) = generate_keypair(key_generation_randomness.try_into().unwrap()) {
                if let Ok((ciphertext, shared_secret)) = encapsulate(key_pair.pk, encapsulation_randomness.try_into().unwrap()) {
                    key_pair.sk[secret_key_position] ^= random_byte;
                    let shared_secret_decapsulated = decapsulate(ciphertext, key_pair.sk);

                    assert_ne!(shared_secret, shared_secret_decapsulated);
                }
            }
            // If the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }

        #[test]
        fn modified_ciphertext_and_implicit_rejection_value(
            key_generation_randomness in vec(any::<u8>(), KYBER768_KEY_GENERATION_SEED_SIZE),
            encapsulation_randomness in vec(any::<u8>(), KYBER768_SHARED_SECRET_SIZE),

            secret_key_position in IMPLICIT_REJECTION_VALUE_POSITION..KYBER768_SECRET_KEY_SIZE,
            random_byte_for_secret_key in 1u8..u8::MAX,

            ciphertext_position in 0usize..KYBER768_CIPHERTEXT_SIZE,
            random_byte_for_ciphertext in 1u8..u8::MAX
            ) {
            if let Ok(mut key_pair) = generate_keypair(key_generation_randomness.try_into().unwrap()) {
                if let Ok((mut ciphertext, _)) = encapsulate(key_pair.pk, encapsulation_randomness.try_into().unwrap()) {
                    ciphertext[ciphertext_position] ^= random_byte_for_ciphertext;
                    let shared_secret_decapsulated = decapsulate(ciphertext, key_pair.sk);

                    key_pair.sk[secret_key_position] ^= random_byte_for_secret_key;
                    let shared_secret_decapsulated_2 = decapsulate(ciphertext, key_pair.sk);

                    assert_ne!(shared_secret_decapsulated, shared_secret_decapsulated_2);
                }
            }
            // If the randomness was not enough for the rejection sampling step
            // in key-generation and encapsulation, simply return without
            // failing.
        }
    }
}
