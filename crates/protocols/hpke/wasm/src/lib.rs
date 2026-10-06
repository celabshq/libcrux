//! # HPKE for WASM
//!
//! A minimal [HPKE](https://www.rfc-editor.org/rfc/rfc9180.html) API exposed to
//! WASM. It offers single-shot key generation, seal, and open in HPKE Base mode
//! (no PSK, no sender authentication).
//!
//! The ciphersuite (mode, KEM, KDF, AEAD) is chosen by the caller via
//! [`HpkeConfig`], using the RFC 9180 code points.
//!
//! All functions panic on any error.

use hpke_rs::{
    hpke_types::*,
    libcrux::{HpkeLibcrux, HpkeLibcruxPrng},
    *,
};
use wasm_bindgen::prelude::*;

/// An HPKE ciphersuite configuration.
///
/// The values are the RFC 9180 code points for the mode, KEM, KDF, and AEAD.
#[wasm_bindgen]
pub struct HpkeConfig {
    mode: Mode,
    kem: KemAlgorithm,
    kdf: KdfAlgorithm,
    aead: AeadAlgorithm,
}

#[wasm_bindgen]
impl HpkeConfig {
    /// Create a new HPKE configuration from RFC 9180 code points.
    ///
    /// * `mode` — e.g. `0x00` for Base.
    /// * `kem` — e.g. `0x0020` for DHKEM(X25519, HKDF-SHA256).
    /// * `kdf` — e.g. `0x0001` for HKDF-SHA256.
    /// * `aead` — e.g. `0x0003` for ChaCha20Poly1305.
    ///
    /// Panics if any code point is unknown.
    #[wasm_bindgen(constructor)]
    pub fn new(mode: u8, kem: u16, kdf: u16, aead: u16) -> HpkeConfig {
        HpkeConfig {
            mode: Mode::try_from(mode).unwrap(),
            kem: KemAlgorithm::try_from(kem).unwrap(),
            kdf: KdfAlgorithm::try_from(kdf).unwrap(),
            aead: AeadAlgorithm::try_from(aead).unwrap(),
        }
    }
}

impl HpkeConfig {
    /// Build an [`Hpke`] instance with a freshly seeded, wasm-friendly PRNG.
    fn hpke(&self) -> Hpke<HpkeLibcrux> {
        Hpke::new_with_rng(self.mode, self.kem, self.kdf, self.aead, new_prng())
    }
}

/// An HPKE key pair, holding the raw private and public key bytes.
#[wasm_bindgen]
pub struct KeyPair {
    sk: Vec<u8>,
    pk: Vec<u8>,
}

#[wasm_bindgen]
impl KeyPair {
    /// The raw private key bytes.
    #[wasm_bindgen(getter)]
    pub fn sk(&self) -> Vec<u8> {
        self.sk.clone()
    }

    /// The raw public key bytes.
    #[wasm_bindgen(getter)]
    pub fn pk(&self) -> Vec<u8> {
        self.pk.clone()
    }
}

/// An HPKE ciphertext, holding the encapsulated secret and the encrypted bytes.
#[wasm_bindgen]
pub struct Ciphertext {
    enc: Vec<u8>,
    ct: Vec<u8>,
}

#[wasm_bindgen]
impl Ciphertext {
    /// The encapsulated secret.
    #[wasm_bindgen(getter)]
    pub fn enc(&self) -> Vec<u8> {
        self.enc.clone()
    }

    /// The encrypted bytes.
    #[wasm_bindgen(getter)]
    pub fn ct(&self) -> Vec<u8> {
        self.ct.clone()
    }
}

/// Construct a provider PRNG seeded with 32 bytes of system randomness.
///
/// This keeps all wasm-/RNG-specific concerns in this crate: the seed is
/// gathered via `getrandom` (with its wasm backend on `wasm32`) and used to
/// seed the HMAC-DRBG provider PRNG.
fn new_prng() -> HpkeLibcruxPrng {
    let mut seed = [0u8; 32];
    getrandom::fill(&mut seed).unwrap();
    HpkeLibcruxPrng::try_from_seed(seed).unwrap()
}

/// Generate an HPKE key pair for the given ciphersuite.
#[wasm_bindgen]
pub fn hpke_key_gen(config: &HpkeConfig) -> KeyPair {
    let mut hpke = config.hpke();
    let (sk, pk) = hpke.generate_key_pair().unwrap().into_keys();
    KeyPair {
        sk: sk.as_slice().to_vec(),
        pk: pk.as_slice().to_vec(),
    }
}

/// Single-shot HPKE seal (Base mode).
///
/// Encrypts `pt` to the receiver public key `pk_r`, returning the encapsulated
/// secret and the ciphertext.
#[wasm_bindgen]
pub fn hpke_seal(
    config: &HpkeConfig,
    pk_r: &[u8],
    info: &[u8],
    aad: &[u8],
    pt: &[u8],
) -> Ciphertext {
    let mut hpke = config.hpke();
    let (enc, ct) = hpke
        .seal(
            &HpkePublicKey::new(pk_r.to_vec()),
            info,
            aad,
            pt,
            None,
            None,
            None,
        )
        .unwrap();
    Ciphertext { enc, ct }
}

/// Single-shot HPKE open (Base mode).
///
/// Decrypts `ct` (with encapsulated secret `enc`) using the receiver private
/// key `sk_r`, returning the plaintext.
#[wasm_bindgen]
pub fn hpke_open(
    config: &HpkeConfig,
    enc: &[u8],
    sk_r: &[u8],
    info: &[u8],
    aad: &[u8],
    ct: &[u8],
) -> Vec<u8> {
    let hpke = config.hpke();
    hpke.open(
        enc,
        &HpkePrivateKey::new(sk_r.to_vec()),
        info,
        aad,
        ct,
        None,
        None,
        None,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let configs = vec![
            // Base / DHKEM(X25519, HKDF-SHA256) / HKDF-SHA256 / ChaCha20Poly1305.
            HpkeConfig::new(0x00, 0x0020, 0x0001, 0x0003),
            // Base / ML-KEM-768 + X25519 / SHAKE256 / ChaCha20Poly1305
            HpkeConfig::new(0x00, 0x647A, 0x0011, 0x0003),
            // Base / ML-KEM-768 + P-256 / SHAKE256 / AES-256-GCM
            HpkeConfig::new(0x00, 0x0050, 0x0011, 0x0002),
        ];

        for config in configs {
            let kp = hpke_key_gen(&config);

            let info = b"HPKE demo info";
            let aad = b"HPKE demo aad";
            let plaintext = b"HPKE demo plain text";

            let ciphertext = hpke_seal(&config, &kp.pk(), info, aad, plaintext);
            let recovered = hpke_open(
                &config,
                &ciphertext.enc(),
                &kp.sk(),
                info,
                aad,
                &ciphertext.ct(),
            );

            assert_eq!(recovered, plaintext);
        }
    }
}
