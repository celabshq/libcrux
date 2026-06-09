//! # Key Encapsulation Mechanism
//!
//! A KEM interface.
//!
//! For ECDH structs, check the [`libcrux_ecdh`] crate.
//!
//! Available algorithms:
//! * [`Algorithm::X25519`]\: x25519 ECDH KEM. Also see [`libcrux::ecdh#x25519`].
//! * [`Algorithm::Secp256r1`]\: NIST P256 ECDH KEM. Also see [`libcrux::ecdh#P256`].
//! * [`Algorithm::MlKem512`]\: ML-KEM 512 from [FIPS 203].
//! * [`Algorithm::MlKem768`]\: ML-KEM 768 from [FIPS 203].
//! * [`Algorithm::MlKem1024`]\: ML-KEM 1024 from [FIPS 203].
//! * [`Algorithm::X25519MlKem768Draft00`]\: Hybrid x25519 - ML-KEM 768 [draft kem for hpke](https://www.ietf.org/archive/id/draft-westerbaan-cfrg-hpke-xyber768d00-00.html).
//! * [`Algorithm::XWingKemDraft06`]\: Hybrid x25519 - ML-KEM 768 [draft xwing kem for hpke](https://www.ietf.org/archive/id/draft-connolly-cfrg-xwing-kem-06.html).
//!
//! ```
//! use libcrux_kem::*;
//! use rand::rngs::SysRng;
//! use rand_core::UnwrapErr;
//!
//! let mut rng = UnwrapErr(SysRng);
//!
//! let (sk_a, pk_a) = key_gen(Algorithm::MlKem768, &mut rng).unwrap();
//! let received_pk = pk_a.encode();
//!
//! let pk = PublicKey::decode(Algorithm::MlKem768, &received_pk).unwrap();
//! let (ss_b, ct_b) = pk.encapsulate(&mut rng).unwrap();
//! let received_ct = ct_b.encode();
//!
//! let ct_a = Ct::decode(Algorithm::MlKem768, &received_ct).unwrap();
//! let ss_a = ct_a.decapsulate(&sk_a).unwrap();
//! assert_eq!(ss_b.encode(), ss_a.encode());
//! ```
//!
//! [FIPS 203]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.ipd.pdf
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
#[cfg(feature = "codec")]
use std::format;

use libcrux_ecdh::{
    p256_derive, p256_secret_to_public, x25519_derive, x25519_secret_to_public, P256PrivateKey,
    P256PublicKey, P256SharedSecret, X25519PrivateKey, X25519PublicKey, X25519SharedSecret,
};
use libcrux_ml_kem::{mlkem1024, mlkem512, mlkem768};
use libcrux_sha3 as sha3;
use rand::CryptoRng;
#[cfg(feature = "codec")]
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

#[cfg(feature = "codec")]
extern crate std;

// TODO: These functions are currently exposed simply in order to make NIST KAT
// testing possible without an implementation of the NIST AES-CTR DRBG. Remove them
// (and change the visibility of the exported functions to pub(crate)) the
// moment we have an implementation of one. This is tracked by:
// https://github.com/cryspen/libcrux/issues/36
#[cfg(feature = "tests")]
pub mod deterministic {
    pub use libcrux_ml_kem::{
        mlkem1024::{
            decapsulate as mlkem1024_decapsulate_derand,
            encapsulate as mlkem1024_encapsulate_derand,
            generate_key_pair as mlkem1024_generate_keypair_derand,
        },
        mlkem512::{
            decapsulate as mlkem512_decapsulate_derand, encapsulate as mlkem512_encapsulate_derand,
            generate_key_pair as mlkem512_generate_keypair_derand,
        },
        mlkem768::{
            decapsulate as mlkem768_decapsulate_derand, encapsulate as mlkem768_encapsulate_derand,
            generate_key_pair as mlkem768_generate_keypair_derand,
        },
    };
}

pub use libcrux_curve25519::X25519;
use libcrux_ml_kem::MlKemSharedSecret;
#[cfg(feature = "tests")]
pub use libcrux_ml_kem::{
    mlkem1024::validate_public_key as ml_kem1024_validate_public_key,
    mlkem512::validate_public_key as ml_kem512_validate_public_key,
    mlkem768::validate_public_key as ml_kem768_validate_public_key,
};
pub use libcrux_ml_kem::{
    mlkem1024::{MlKem1024, MlKem1024Ciphertext, MlKem1024PrivateKey, MlKem1024PublicKey},
    mlkem512::{MlKem512, MlKem512Ciphertext, MlKem512PrivateKey, MlKem512PublicKey},
    mlkem768::{MlKem768, MlKem768Ciphertext, MlKem768PrivateKey, MlKem768PublicKey},
    MlKemCiphertext, MlKemKeyPair,
};
pub use libcrux_p256::P256;
pub use xwing::XWing;
use xwing::XWingSharedSecret;

/// KEM Algorithms
///
/// This includes named elliptic curves or dedicated KEM algorithms like ML-KEM.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Algorithm {
    X25519,
    X448,
    Secp256r1,
    Secp384r1,
    Secp521r1,
    MlKem512,
    MlKem768,
    X25519MlKem768Draft00,
    XWingKemDraft06,
    MlKem1024,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EcDhError(libcrux_ecdh::Error),
    KeyGen,
    Encapsulate,
    Decapsulate,
    UnsupportedAlgorithm,
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidCiphertext,
}

impl TryFrom<Algorithm> for libcrux_ecdh::Algorithm {
    type Error = &'static str;

    fn try_from(value: Algorithm) -> Result<Self, Self::Error> {
        match value {
            Algorithm::X25519 => Ok(libcrux_ecdh::Algorithm::X25519),
            Algorithm::X448 => Ok(libcrux_ecdh::Algorithm::X448),
            Algorithm::Secp256r1 => Ok(libcrux_ecdh::Algorithm::P256),
            Algorithm::Secp384r1 => Ok(libcrux_ecdh::Algorithm::P384),
            Algorithm::Secp521r1 => Ok(libcrux_ecdh::Algorithm::P521),
            Algorithm::X25519MlKem768Draft00 => Ok(libcrux_ecdh::Algorithm::X25519),
            Algorithm::XWingKemDraft06 => Ok(libcrux_ecdh::Algorithm::X25519),
            _ => Err("provided algorithm is not an ECDH algorithm"),
        }
    }
}

impl From<libcrux_ecdh::Error> for Error {
    fn from(value: libcrux_ecdh::Error) -> Self {
        Error::EcDhError(value)
    }
}

/// An ML-KEM768-x25519 private key.
pub struct X25519MlKem768Draft00PrivateKey {
    pub mlkem: MlKem768PrivateKey,
    pub x25519: X25519PrivateKey,
}

impl X25519MlKem768Draft00PrivateKey {
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            mlkem: bytes[..2400]
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)?,
            x25519: bytes[2400..]
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)?,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = self.mlkem.as_ref().to_vec();
        out.extend_from_slice(&self.x25519.0);
        out
    }
}

/// An X-Wing private key.
pub struct XWingKemDraft06PrivateKey {
    pub seed: [u8; 32],
}

impl XWingKemDraft06PrivateKey {
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            seed: bytes.try_into().map_err(|_| Error::InvalidPrivateKey)?,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        self.seed.as_ref().to_vec()
    }
}

/// A KEM private key.
pub enum PrivateKey {
    X25519(X25519PrivateKey),
    P256(P256PrivateKey),
    MlKem512(MlKem512PrivateKey),
    MlKem768(MlKem768PrivateKey),
    X25519MlKem768Draft00(X25519MlKem768Draft00PrivateKey),
    XWingKemDraft06(XWingKemDraft06PrivateKey),
    MlKem1024(MlKem1024PrivateKey),
}

/// An ML-KEM768-x25519 public key.
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
pub struct X25519MlKem768Draft00PublicKey {
    pub mlkem: MlKem768PublicKey,
    pub x25519: X25519PublicKey,
}

impl X25519MlKem768Draft00PublicKey {
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            mlkem: {
                let key = MlKem768PublicKey::try_from(&bytes[..1184])
                    .map_err(|_| Error::InvalidPublicKey)?;
                if !mlkem768::validate_public_key(&key) {
                    return Err(Error::InvalidPublicKey);
                }
                key
            },
            x25519: bytes[1184..]
                .try_into()
                .map_err(|_| Error::InvalidPublicKey)?,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = self.mlkem.as_ref().to_vec();
        out.extend_from_slice(&self.x25519.0);
        out
    }
}

/// An X-Wing public key.
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
pub struct XWingKemDraft06PublicKey {
    pub pk_m: MlKem768PublicKey,
    pub pk_x: X25519PublicKey,
}

impl XWingKemDraft06PublicKey {
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            pk_m: {
                let key = MlKem768PublicKey::try_from(&bytes[0..1184])
                    .map_err(|_| Error::InvalidPublicKey)?;
                if !mlkem768::validate_public_key(&key) {
                    return Err(Error::InvalidPublicKey);
                }
                key
            },
            pk_x: bytes[1184..]
                .try_into()
                .map_err(|_| Error::InvalidPublicKey)?,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = self.pk_m.as_ref().to_vec();
        out.extend_from_slice(self.pk_x.0.to_vec().as_ref());
        out
    }
}

/// A KEM public key.
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
#[repr(u8)]
pub enum PublicKey {
    X25519(X25519PublicKey),
    P256(P256PublicKey),
    MlKem512(MlKem512PublicKey),
    MlKem768(MlKem768PublicKey),
    X25519MlKem768Draft00(X25519MlKem768Draft00PublicKey),
    XWingKemDraft06(XWingKemDraft06PublicKey),
    MlKem1024(MlKem1024PublicKey),
}

/// A KEM ciphertext
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
#[repr(u8)]
pub enum Ct {
    X25519(X25519PublicKey),
    P256(P256PublicKey),
    MlKem512(MlKem512Ciphertext),
    MlKem768(MlKem768Ciphertext),
    X25519MlKem768Draft00(MlKem768Ciphertext, X25519PublicKey),
    XWingKemDraft06(MlKem768Ciphertext, X25519PublicKey),
    MlKem1024(MlKem1024Ciphertext),
}

impl Ct {
    /// Decapsulate the shared secret in `ct` using the private key `sk`.
    pub fn decapsulate(&self, sk: &PrivateKey) -> Result<Ss, Error> {
        match self {
            Ct::X25519(ct) => {
                let sk = if let PrivateKey::X25519(k) = sk {
                    k
                } else {
                    return Err(Error::InvalidPrivateKey);
                };
                x25519_derive(ct, sk).map_err(|e| e.into()).map(Ss::X25519)
            }
            Ct::P256(ct) => {
                let sk = if let PrivateKey::P256(k) = sk {
                    k
                } else {
                    return Err(Error::InvalidPrivateKey);
                };
                p256_derive(ct, sk).map_err(|e| e.into()).map(Ss::P256)
            }
            Ct::MlKem512(ct) => {
                let sk = if let PrivateKey::MlKem512(k) = sk {
                    k
                } else {
                    return Err(Error::InvalidPrivateKey);
                };
                let ss = libcrux_ml_kem::mlkem512::decapsulate(sk, ct);

                Ok(Ss::MlKem768(ss))
            }
            Ct::MlKem768(ct) => {
                let sk = if let PrivateKey::MlKem768(k) = sk {
                    k
                } else {
                    return Err(Error::InvalidPrivateKey);
                };
                let ss = mlkem768::decapsulate(sk, ct);

                Ok(Ss::MlKem768(ss))
            }
            Ct::X25519MlKem768Draft00(kct, xct) => {
                let (ksk, xsk) =
                    if let PrivateKey::X25519MlKem768Draft00(X25519MlKem768Draft00PrivateKey {
                        mlkem: kk,
                        x25519: xk,
                    }) = sk
                    {
                        (kk, xk)
                    } else {
                        return Err(Error::InvalidPrivateKey);
                    };
                let kss = mlkem768::decapsulate(ksk, kct);
                let xss = x25519_derive(xct, xsk)?;

                Ok(Ss::X25519MlKem768Draft00(kss, xss))
            }

            Ct::XWingKemDraft06(ct_m, ct_x) => {
                let seed =
                    if let PrivateKey::XWingKemDraft06(XWingKemDraft06PrivateKey { seed }) = sk {
                        seed
                    } else {
                        return Err(Error::InvalidPrivateKey);
                    };

                let (kp_m, pk_x, sk_x) = xwing::expand_decap_key(seed)?;

                let ss_m = mlkem768::decapsulate(kp_m.private_key(), ct_m);
                let ss_x = x25519_derive(ct_x, &sk_x)?;

                Ok(Ss::XWingKemDraft06(xwing::combiner(
                    &ss_m,
                    ss_x.as_ref(),
                    ct_x.as_ref(),
                    pk_x.as_ref(),
                )))
            }

            Ct::MlKem1024(ct) => {
                let sk = if let PrivateKey::MlKem1024(k) = sk {
                    k
                } else {
                    return Err(Error::InvalidPrivateKey);
                };
                let ss = libcrux_ml_kem::mlkem1024::decapsulate(sk, ct);

                Ok(Ss::MlKem1024(ss))
            }
        }
    }
}

/// A KEM shared secret
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
#[repr(u8)]
pub enum Ss {
    X25519(X25519SharedSecret),
    P256(P256SharedSecret),
    MlKem512(MlKemSharedSecret),
    MlKem768(MlKemSharedSecret),
    X25519MlKem768Draft00(MlKemSharedSecret, X25519SharedSecret),
    XWingKemDraft06(XWingSharedSecret),
    MlKem1024(MlKemSharedSecret),
}

impl PrivateKey {
    /// Encode a private key.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            PrivateKey::X25519(k) => k.0.to_vec(),
            PrivateKey::P256(k) => k.0.to_vec(),
            PrivateKey::MlKem512(k) => k.as_slice().to_vec(),
            PrivateKey::MlKem768(k) => k.as_slice().to_vec(),
            PrivateKey::X25519MlKem768Draft00(k) => k.encode(),
            PrivateKey::XWingKemDraft06(k) => k.encode(),
            PrivateKey::MlKem1024(k) => k.as_slice().to_vec(),
        }
    }

    /// Decode a private key.
    pub fn decode(alg: Algorithm, bytes: &[u8]) -> Result<Self, Error> {
        match alg {
            Algorithm::X25519 => bytes
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)
                .map(Self::X25519),
            Algorithm::Secp256r1 => bytes
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)
                .map(Self::P256),
            Algorithm::MlKem512 => bytes
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)
                .map(Self::MlKem512),
            Algorithm::MlKem768 => bytes
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)
                .map(Self::MlKem768),
            Algorithm::X25519MlKem768Draft00 => X25519MlKem768Draft00PrivateKey::decode(bytes)
                .map_err(|_| Error::InvalidPrivateKey)
                .map(Self::X25519MlKem768Draft00),
            Algorithm::XWingKemDraft06 => {
                let pk = XWingKemDraft06PrivateKey::decode(bytes)
                    .map_err(|_| Error::InvalidPrivateKey)?;
                Ok(Self::XWingKemDraft06(pk))
            }

            Algorithm::MlKem1024 => bytes
                .try_into()
                .map_err(|_| Error::InvalidPrivateKey)
                .map(Self::MlKem1024),
            _ => Err(Error::UnsupportedAlgorithm),
        }
    }
}

impl PublicKey {
    /// Encapsulate a shared secret to the provided `pk` and return the `(Key, Enc)` tuple.
    pub fn encapsulate(&self, rng: &mut impl CryptoRng) -> Result<(Ss, Ct), Error> {
        match self {
            PublicKey::X25519(pk) => {
                let (new_sk, new_pk) = libcrux_ecdh::x25519_key_gen(rng)?;
                let gxy = x25519_derive(pk, &new_sk)?;
                Ok((Ss::X25519(gxy), Ct::X25519(new_pk)))
            }
            PublicKey::P256(pk) => {
                let (new_sk, new_pk) = libcrux_ecdh::p256_key_gen(rng)?;
                let gxy = p256_derive(pk, &new_sk)?;
                Ok((Ss::P256(gxy), Ct::P256(new_pk)))
            }

            PublicKey::MlKem512(pk) => {
                let seed = mlkem_rand(rng)?;
                let (ct, ss) = libcrux_ml_kem::mlkem512::encapsulate(pk, seed);
                Ok((Ss::MlKem512(ss), Ct::MlKem512(ct)))
            }

            PublicKey::MlKem768(pk) => {
                let seed = mlkem_rand(rng)?;
                let (ct, ss) = mlkem768::encapsulate(pk, seed);
                Ok((Ss::MlKem768(ss), Ct::MlKem768(ct)))
            }

            PublicKey::MlKem1024(pk) => {
                let seed = mlkem_rand(rng)?;
                let (ct, ss) = mlkem1024::encapsulate(pk, seed);
                Ok((Ss::MlKem1024(ss), Ct::MlKem1024(ct)))
            }

            PublicKey::X25519MlKem768Draft00(X25519MlKem768Draft00PublicKey {
                mlkem: kpk,
                x25519: xpk,
            }) => {
                let seed = mlkem_rand(rng)?;
                let (mlkem_ct, mlkem_ss) = mlkem768::encapsulate(kpk, seed);
                let (x_sk, x_pk) = libcrux_ecdh::x25519_key_gen(rng)?;
                let x_ss = x25519_derive(xpk, &x_sk)?;

                Ok((
                    Ss::X25519MlKem768Draft00(mlkem_ss, x_ss),
                    Ct::X25519MlKem768Draft00(mlkem_ct, x_pk),
                ))
            }

            PublicKey::XWingKemDraft06(XWingKemDraft06PublicKey { pk_m, pk_x }) => {
                let seed = mlkem_rand(rng)?;
                let (ct_m, ss_m) = mlkem768::encapsulate(pk_m, seed);
                let (ek_x, ct_x) = libcrux_ecdh::x25519_key_gen(rng)?;
                let ss_x = x25519_derive(pk_x, &ek_x)?;

                Ok((
                    Ss::XWingKemDraft06(xwing::combiner(
                        &ss_m,
                        ss_x.as_ref(),
                        ct_x.as_ref(),
                        pk_x.as_ref(),
                    )),
                    Ct::XWingKemDraft06(ct_m, X25519PublicKey(ct_x.0)),
                ))
            }
        }
    }

    /// Encapsulate a shared secret to the provided `pk` and return the `(Key, Enc)` tuple.
    pub fn encapsulate_derand(&self, seed: &[u8]) -> Result<(Ss, Ct), Error> {
        match self {
            PublicKey::X25519(pk) => {
                let new_sk = X25519PrivateKey::try_from(seed)?; // clamps
                let new_pk = x25519_secret_to_public(&new_sk)?;
                let gxy = x25519_derive(pk, &new_sk)?;
                Ok((Ss::X25519(gxy), Ct::X25519(new_pk.try_into().unwrap())))
            }

            PublicKey::P256(pk) => {
                let new_sk = P256PrivateKey::try_from(seed)?;
                let new_pk = p256_secret_to_public(&new_sk)?;

                let gxy = p256_derive(pk, &new_sk)?;
                Ok((Ss::P256(gxy), Ct::P256(new_pk)))
            }

            PublicKey::MlKem512(pk) => {
                let (ct, ss) = libcrux_ml_kem::mlkem512::encapsulate(
                    pk,
                    seed.try_into().map_err(|_| Error::KeyGen)?,
                );
                Ok((Ss::MlKem512(ss), Ct::MlKem512(ct)))
            }

            PublicKey::MlKem768(pk) => {
                let (ct, ss) =
                    mlkem768::encapsulate(pk, seed.try_into().map_err(|_| Error::KeyGen)?);
                Ok((Ss::MlKem768(ss), Ct::MlKem768(ct)))
            }

            PublicKey::MlKem1024(pk) => {
                let (ct, ss) =
                    mlkem1024::encapsulate(pk, seed.try_into().map_err(|_| Error::KeyGen)?);
                Ok((Ss::MlKem1024(ss), Ct::MlKem1024(ct)))
            }

            PublicKey::X25519MlKem768Draft00(X25519MlKem768Draft00PublicKey {
                mlkem: kpk,
                x25519: xpk,
            }) => {
                // seed = mlkem_seed || x_sk
                let (mlkem_ct, mlkem_ss) =
                    mlkem768::encapsulate(kpk, seed[0..32].try_into().map_err(|_| Error::KeyGen)?);

                let x_sk = X25519PrivateKey::try_from(&seed[32..])?; // clamps
                let x_pk = x25519_secret_to_public(&x_sk)?;

                let x_ss = x25519_derive(xpk, &x_sk)?;

                Ok((
                    Ss::X25519MlKem768Draft00(mlkem_ss, x_ss),
                    Ct::X25519MlKem768Draft00(mlkem_ct, x_pk),
                ))
            }

            PublicKey::XWingKemDraft06(XWingKemDraft06PublicKey { pk_m, pk_x }) => {
                let (ct_m, ss_m) =
                    mlkem768::encapsulate(pk_m, seed[0..32].try_into().map_err(|_| Error::KeyGen)?);

                let ek_x = X25519PrivateKey::try_from(&seed[32..])?; // clamps
                let ct_x = x25519_secret_to_public(&ek_x)?;

                let ss_x = x25519_derive(pk_x, &ek_x)?;

                Ok((
                    Ss::XWingKemDraft06(xwing::combiner(
                        &ss_m,
                        ss_x.as_ref(),
                        ct_x.as_ref(),
                        pk_x.as_ref(),
                    )),
                    Ct::XWingKemDraft06(ct_m, X25519PublicKey(ct_x.0)),
                ))
            }
        }
    }

    /// Encode public key.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            PublicKey::X25519(k) => k.0.to_vec(),
            PublicKey::P256(k) => k.0.to_vec(),
            PublicKey::MlKem512(k) => k.as_ref().to_vec(),
            PublicKey::MlKem768(k) => k.as_ref().to_vec(),
            PublicKey::X25519MlKem768Draft00(k) => k.encode(),
            PublicKey::XWingKemDraft06(k) => k.encode(),
            PublicKey::MlKem1024(k) => k.as_ref().to_vec(),
        }
    }

    /// Decode a public key.
    pub fn decode(alg: Algorithm, bytes: &[u8]) -> Result<Self, Error> {
        match alg {
            Algorithm::X25519 => bytes
                .try_into()
                .map(Self::X25519)
                .map_err(|_| Error::InvalidPublicKey),
            Algorithm::Secp256r1 => bytes
                .try_into()
                .map(Self::P256)
                .map_err(|_| Error::InvalidPublicKey),
            Algorithm::MlKem512 => {
                let key =
                    MlKem512PublicKey::try_from(bytes).map_err(|_| Error::InvalidPublicKey)?;
                if !mlkem512::validate_public_key(&key) {
                    return Err(Error::InvalidPublicKey);
                }
                Ok(Self::MlKem512(key))
            }
            Algorithm::MlKem768 => {
                let key =
                    MlKem768PublicKey::try_from(bytes).map_err(|_| Error::InvalidPublicKey)?;
                if !mlkem768::validate_public_key(&key) {
                    return Err(Error::InvalidPublicKey);
                }
                Ok(Self::MlKem768(key))
            }
            Algorithm::X25519MlKem768Draft00 => {
                X25519MlKem768Draft00PublicKey::decode(bytes).map(Self::X25519MlKem768Draft00)
            }
            Algorithm::XWingKemDraft06 => {
                XWingKemDraft06PublicKey::decode(bytes).map(Self::XWingKemDraft06)
            }
            Algorithm::MlKem1024 => {
                let key =
                    MlKem1024PublicKey::try_from(bytes).map_err(|_| Error::InvalidPublicKey)?;
                if !mlkem1024::validate_public_key(&key) {
                    return Err(Error::InvalidPublicKey);
                }
                Ok(Self::MlKem1024(key))
            }
            _ => Err(Error::UnsupportedAlgorithm),
        }
    }
}

impl Ss {
    /// Encode a shared secret.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Ss::X25519(k) => k.0.to_vec(),
            Ss::P256(k) => k.0.to_vec(),
            Ss::MlKem512(k) => k.as_ref().to_vec(),
            Ss::MlKem768(k) => k.as_ref().to_vec(),
            Ss::X25519MlKem768Draft00(kk, xk) => {
                let mut out = kk.to_vec();
                out.extend_from_slice(xk.0.as_ref());
                out
            }
            Ss::XWingKemDraft06(ss) => ss.value.into(),
            Ss::MlKem1024(k) => k.as_ref().to_vec(),
        }
    }
}

impl Ct {
    /// Encode a ciphertext.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Ct::X25519(k) => k.0.to_vec(),
            Ct::P256(k) => k.0.to_vec(),
            Ct::MlKem512(k) => k.as_ref().to_vec(),
            Ct::MlKem768(k) => k.as_ref().to_vec(),
            Ct::X25519MlKem768Draft00(kk, xk) => {
                let mut out = kk.as_ref().to_vec();
                out.extend_from_slice(xk.0.as_ref());
                out
            }
            Ct::XWingKemDraft06(ct_m, ct_x) => {
                let mut out = ct_m.as_ref().to_vec();
                out.extend_from_slice(ct_x.as_ref());
                out
            }
            Ct::MlKem1024(k) => k.as_ref().to_vec(),
        }
    }

    /// Decode a ciphertext.
    pub fn decode(alg: Algorithm, bytes: &[u8]) -> Result<Self, Error> {
        match alg {
            Algorithm::X25519 => bytes
                .try_into()
                .map_err(|_| Error::InvalidCiphertext)
                .map(Self::X25519),
            Algorithm::Secp256r1 => bytes
                .try_into()
                .map_err(|_| Error::InvalidCiphertext)
                .map(Self::P256),
            Algorithm::MlKem512 => bytes
                .try_into()
                .map_err(|_| Error::InvalidCiphertext)
                .map(Self::MlKem512),
            Algorithm::MlKem768 => bytes
                .try_into()
                .map_err(|_| Error::InvalidCiphertext)
                .map(Self::MlKem768),
            Algorithm::X25519MlKem768Draft00 => {
                let key: [u8; MlKem768Ciphertext::len() + 32] =
                    bytes.try_into().map_err(|_| Error::InvalidCiphertext)?;
                let (kct, xct) = key.split_at(1088);
                Ok(Self::X25519MlKem768Draft00(
                    kct.try_into().map_err(|_| Error::InvalidCiphertext)?,
                    xct.try_into().map_err(|_| Error::InvalidCiphertext)?,
                ))
            }
            Algorithm::XWingKemDraft06 => {
                let key: [u8; MlKem768Ciphertext::len() + 32] =
                    bytes.try_into().map_err(|_| Error::InvalidCiphertext)?;
                let (ct_m, ct_x) = key.split_at(MlKem768Ciphertext::len());
                Ok(Self::XWingKemDraft06(
                    ct_m.try_into().map_err(|_| Error::InvalidCiphertext)?,
                    ct_x.try_into().map_err(|_| Error::InvalidCiphertext)?,
                ))
            }
            Algorithm::MlKem1024 => bytes
                .try_into()
                .map_err(|_| Error::InvalidCiphertext)
                .map(Self::MlKem1024),
            _ => Err(Error::UnsupportedAlgorithm),
        }
    }
}

/// Compute the public key for a private key of the given [`Algorithm`].
/// Applicable only to X25519 and secp256r1.
pub fn secret_to_public(alg: Algorithm, sk: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
    match alg {
        Algorithm::X25519 | Algorithm::Secp256r1 => {
            libcrux_ecdh::secret_to_public(alg.try_into().unwrap(), sk.as_ref())
                .map_err(|e| e.into())
        }
        _ => Err(Error::UnsupportedAlgorithm),
    }
}

fn gen_mlkem768(
    rng: &mut impl CryptoRng,
) -> Result<(MlKem768PrivateKey, MlKem768PublicKey), Error> {
    Ok(mlkem768::generate_key_pair(random_array(rng)?).into_parts())
}

fn random_array<const L: usize>(rng: &mut impl CryptoRng) -> Result<[u8; L], Error> {
    let mut seed = [0; L];
    rng.try_fill_bytes(&mut seed).map_err(|_| Error::KeyGen)?;
    Ok(seed)
}

/// Generate a key pair for the [`Algorithm`] using the provided rng.
///
/// The function returns a fresh key or a [`Error::KeyGen`] error if
/// * not enough entropy was available
/// * it was not possible to generate a valid key within a reasonable amount of iterations.
pub fn key_gen(alg: Algorithm, rng: &mut impl CryptoRng) -> Result<(PrivateKey, PublicKey), Error> {
    match alg {
        Algorithm::X25519 => libcrux_ecdh::x25519_key_gen(rng)
            .map_err(|e| e.into())
            .map(|(private, public)| (PrivateKey::X25519(private), PublicKey::X25519(public))),
        Algorithm::Secp256r1 => libcrux_ecdh::p256_key_gen(rng)
            .map_err(|e| e.into())
            .map(|(private, public)| (PrivateKey::P256(private), PublicKey::P256(public))),
        Algorithm::MlKem512 => {
            let (sk, pk) = mlkem512::generate_key_pair(random_array(rng)?).into_parts();
            Ok((PrivateKey::MlKem512(sk), PublicKey::MlKem512(pk)))
        }
        Algorithm::MlKem768 => {
            let (sk, pk) = mlkem768::generate_key_pair(random_array(rng)?).into_parts();
            Ok((PrivateKey::MlKem768(sk), PublicKey::MlKem768(pk)))
        }
        Algorithm::MlKem1024 => {
            let (sk, pk) = mlkem1024::generate_key_pair(random_array(rng)?).into_parts();
            Ok((PrivateKey::MlKem1024(sk), PublicKey::MlKem1024(pk)))
        }
        Algorithm::X25519MlKem768Draft00 => {
            let (mlkem_private, mlkem_public) = gen_mlkem768(rng)?;
            let (x25519_private, x25519_public) = libcrux_ecdh::x25519_key_gen(rng)?;

            Ok((
                PrivateKey::X25519MlKem768Draft00(X25519MlKem768Draft00PrivateKey {
                    mlkem: mlkem_private,
                    x25519: x25519_private,
                }),
                PublicKey::X25519MlKem768Draft00(X25519MlKem768Draft00PublicKey {
                    mlkem: mlkem_public,
                    x25519: x25519_public,
                }),
            ))
        }

        Algorithm::XWingKemDraft06 => {
            let mut seed = [0u8; 32];
            rng.fill_bytes(&mut seed);

            let (kp_m, pk_x, _) = xwing::expand_decap_key(&seed)?;

            Ok((
                PrivateKey::XWingKemDraft06(XWingKemDraft06PrivateKey { seed }),
                PublicKey::XWingKemDraft06(XWingKemDraft06PublicKey {
                    pk_m: kp_m.pk().into(),
                    // unwrap is ok here because it comes from the secret to pub above
                    pk_x,
                }),
            ))
        }
        _ => Err(Error::UnsupportedAlgorithm),
    }
}

/// Generate a key pair for the [`Algorithm`] using the provided rng.
///
/// The function returns a fresh key or a [`Error::KeyGen`] error if
/// * the `seed` wasn't long enough
/// * it was not possible to generate a valid key within a reasonable amount of iterations.
pub fn key_gen_derand(alg: Algorithm, seed: &[u8]) -> Result<(PrivateKey, PublicKey), Error> {
    match alg {
        Algorithm::X25519 => {
            let sk = X25519PrivateKey::try_from(seed)?;
            let pk = x25519_secret_to_public(&sk)?;
            Ok((PrivateKey::X25519(sk), PublicKey::X25519(pk)))
        }

        Algorithm::Secp256r1 => {
            let sk = P256PrivateKey::try_from(seed)?;
            let pk = p256_secret_to_public(&sk)?;
            Ok((PrivateKey::P256(sk), PublicKey::P256(pk)))
        }

        Algorithm::MlKem512 => {
            let (sk, pk) = mlkem512::generate_key_pair(seed.try_into().map_err(|_| Error::KeyGen)?)
                .into_parts();
            Ok((PrivateKey::MlKem512(sk), PublicKey::MlKem512(pk)))
        }

        Algorithm::MlKem768 => {
            let (sk, pk) = mlkem768::generate_key_pair(seed.try_into().map_err(|_| Error::KeyGen)?)
                .into_parts();
            Ok((PrivateKey::MlKem768(sk), PublicKey::MlKem768(pk)))
        }

        Algorithm::MlKem1024 => {
            let (sk, pk) =
                mlkem1024::generate_key_pair(seed.try_into().map_err(|_| Error::KeyGen)?)
                    .into_parts();
            Ok((PrivateKey::MlKem1024(sk), PublicKey::MlKem1024(pk)))
        }

        Algorithm::X25519MlKem768Draft00 => {
            let (mlkem_private, mlkem_public) =
                mlkem768::generate_key_pair(seed.try_into().map_err(|_| Error::KeyGen)?)
                    .into_parts();
            let x25519_private = X25519PrivateKey::try_from(seed)?;
            let x25519_public = x25519_secret_to_public(&x25519_private)?;

            Ok((
                PrivateKey::X25519MlKem768Draft00(X25519MlKem768Draft00PrivateKey {
                    mlkem: mlkem_private,
                    x25519: x25519_private,
                }),
                PublicKey::X25519MlKem768Draft00(X25519MlKem768Draft00PublicKey {
                    mlkem: mlkem_public,
                    x25519: x25519_public,
                }),
            ))
        }

        Algorithm::XWingKemDraft06 => {
            let seed: [u8; 32] = seed.try_into().map_err(|_| Error::KeyGen)?;
            let (kp_m, pk_x, _) = xwing::expand_decap_key(&seed)?;

            Ok((
                PrivateKey::XWingKemDraft06(XWingKemDraft06PrivateKey { seed }),
                PublicKey::XWingKemDraft06(XWingKemDraft06PublicKey {
                    pk_m: kp_m.pk().into(),
                    // unwrap is ok here because it comes from the secret to pub above
                    pk_x,
                }),
            ))
        }

        _ => Err(Error::UnsupportedAlgorithm),
    }
}

/// The XWing KEM combiner with ML-KEM 768 and x25519
mod xwing {
    use libcrux_ecdh::X25519PrivateKey;

    use super::*;

    const MLKEM768_EK_LEN: usize = libcrux_ml_kem::mlkem768::MlKem768PublicKey::len();
    const MLKEM768_DK_LEN: usize = libcrux_ml_kem::mlkem768::MlKem768PrivateKey::len();
    const MLKEM768_CT_LEN: usize = libcrux_ml_kem::mlkem768::MlKem768Ciphertext::len();
    const MLKEM768_SS_LEN: usize = libcrux_ml_kem::SHARED_SECRET_SIZE;
    const MLKEM768_RAND_KEYGEN_LEN: usize = libcrux_ml_kem::KEY_GENERATION_SEED_SIZE;
    const MLKEM768_RAND_ENCAPS_LEN: usize = MLKEM768_SS_LEN;

    const X25519_EK_LEN: usize = libcrux_curve25519::EK_LEN;
    const X25519_DK_LEN: usize = libcrux_curve25519::DK_LEN;
    const X25519_CT_LEN: usize = X25519_EK_LEN;
    const X25519_RAND_KEYGEN_LEN: usize = X25519_DK_LEN;
    const X25519_RAND_ENCAPS_LEN: usize = X25519_DK_LEN;

    const EK_LEN: usize = MLKEM768_EK_LEN + X25519_EK_LEN;
    const DK_LEN: usize = MLKEM768_DK_LEN + X25519_DK_LEN;
    const CT_LEN: usize = MLKEM768_CT_LEN + X25519_CT_LEN;
    const SS_LEN: usize = 32;
    const RAND_KEYGEN_LEN: usize = 32; // gets expanded later
    const RAND_ENCAPS_LEN: usize = MLKEM768_RAND_ENCAPS_LEN + X25519_RAND_ENCAPS_LEN;

    use libcrux_curve25519::X25519;
    use libcrux_ml_kem::mlkem768::MlKem768;

    pub struct XWing;

    impl
        libcrux_traits::kem::arrayref::Kem<
            EK_LEN,
            DK_LEN,
            CT_LEN,
            SS_LEN,
            RAND_KEYGEN_LEN,
            RAND_ENCAPS_LEN,
        > for XWing
    {
        fn keygen(
            ek: &mut [u8; EK_LEN],
            dk: &mut [u8; DK_LEN],
            rand: &[u8; RAND_KEYGEN_LEN],
        ) -> Result<(), libcrux_traits::kem::owned::KeyGenError> {
            let expanded: [u8; MLKEM768_RAND_KEYGEN_LEN + X25519_RAND_KEYGEN_LEN] =
                libcrux_sha3::shake256(rand);

            let (rand_m, rand_x) = expanded.split_at(MLKEM768_RAND_KEYGEN_LEN);
            let rand_m: &[u8; MLKEM768_RAND_KEYGEN_LEN] = rand_m.try_into().unwrap();
            let rand_x: &[u8; X25519_RAND_KEYGEN_LEN] = rand_x.try_into().unwrap();

            let (ek_m, ek_x) = ek.split_at_mut(MLKEM768_EK_LEN);
            let ek_m: &mut [u8; MLKEM768_EK_LEN] = ek_m.try_into().unwrap();
            let ek_x: &mut [u8; X25519_EK_LEN] = ek_x.try_into().unwrap();

            let (dk_m, dk_x) = dk.split_at_mut(MLKEM768_DK_LEN);
            let dk_m: &mut [u8; MLKEM768_DK_LEN] = dk_m.try_into().unwrap();
            let dk_x: &mut [u8; X25519_DK_LEN] = dk_x.try_into().unwrap();

            MlKem768::keygen(ek_m, dk_m, rand_m)?;
            X25519::keygen(ek_x, dk_x, rand_x)?;

            Ok(())
        }

        fn encaps(
            ct: &mut [u8; CT_LEN],
            ss: &mut [u8; SS_LEN],
            ek: &[u8; EK_LEN],
            rand: &[u8; RAND_ENCAPS_LEN],
        ) -> Result<(), libcrux_traits::kem::owned::EncapsError> {
            let (rand_m, rand_x) = rand.split_at(MLKEM768_RAND_ENCAPS_LEN);
            let rand_m: &[u8; MLKEM768_RAND_ENCAPS_LEN] = rand_m.try_into().unwrap();
            let rand_x: &[u8; X25519_RAND_ENCAPS_LEN] = rand_x.try_into().unwrap();

            let (ek_m, ek_x) = ek.split_at(MLKEM768_EK_LEN);
            let ek_m: &[u8; MLKEM768_EK_LEN] = ek_m.try_into().unwrap();
            let ek_x: &[u8; X25519_EK_LEN] = ek_x.try_into().unwrap();

            let (ct_m, ct_x) = ct.split_at_mut(MLKEM768_CT_LEN);
            let ct_m: &mut [u8; MLKEM768_CT_LEN] = ct_m.try_into().unwrap();
            let ct_x: &mut [u8; X25519_CT_LEN] = ct_x.try_into().unwrap();

            let mut hash_buffer = [0u8; 32 + 32 + X25519_CT_LEN + X25519_EK_LEN + 6];
            hash_buffer[96..128].copy_from_slice(ek_x);
            hash_buffer[128..134].copy_from_slice(&[0x5c, 0x2e, 0x2f, 0x2f, 0x5e, 0x5c]);

            let ss_m: &mut [u8; 32] = (&mut hash_buffer[0..32]).try_into().unwrap();
            MlKem768::encaps(ct_m, ss_m, ek_m, rand_m)?;

            let ss_x: &mut [u8; 32] = (&mut hash_buffer[32..64]).try_into().unwrap();
            X25519::encaps(ct_x, ss_x, ek_x, rand_x)?;
            hash_buffer[64..96].copy_from_slice(ct_x);
            sha3::sha256_ema(ss, &hash_buffer);

            Ok(())
        }

        fn decaps(
            ss: &mut [u8; SS_LEN],
            ct: &[u8; CT_LEN],
            dk: &[u8; DK_LEN],
        ) -> Result<(), libcrux_traits::kem::owned::DecapsError> {
            let (dk_m, dk_x) = dk.split_at(MLKEM768_DK_LEN);
            let dk_m: &[u8; MLKEM768_DK_LEN] = dk_m.try_into().unwrap();
            let dk_x: &[u8; X25519_DK_LEN] = dk_x.try_into().unwrap();

            let (ct_m, ct_x) = ct.split_at(MLKEM768_CT_LEN);
            let ct_m: &[u8; MLKEM768_CT_LEN] = ct_m.try_into().unwrap();
            let ct_x: &[u8; X25519_CT_LEN] = ct_x.try_into().unwrap();

            let mut ek_x = [0u8; X25519_EK_LEN];
            libcrux_curve25519::secret_to_public(&mut ek_x, dk_x);

            let mut hash_buffer = [0u8; 32 + 32 + X25519_CT_LEN + X25519_EK_LEN + 6];
            hash_buffer[64..96].copy_from_slice(ct_x);
            hash_buffer[96..128].copy_from_slice(&ek_x);
            hash_buffer[128..134].copy_from_slice(&[0x5c, 0x2e, 0x2f, 0x2f, 0x5e, 0x5c]);

            let ss_m: &mut [u8; 32] = (&mut hash_buffer[0..32]).try_into().unwrap();
            MlKem768::decaps(ss_m, ct_m, dk_m)?;

            let ss_x: &mut [u8; 32] = (&mut hash_buffer[32..64]).try_into().unwrap();
            X25519::decaps(ss_x, ct_x, dk_x)?;
            sha3::sha256_ema(ss, &hash_buffer);

            Ok(())
        }
    }

    libcrux_traits::kem::slice::impl_trait!(XWing => EK_LEN, DK_LEN, CT_LEN, SS_LEN, RAND_KEYGEN_LEN, RAND_ENCAPS_LEN);

    #[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
    pub struct XWingSharedSecret {
        pub(super) value: [u8; 32],
    }

    /// Expand the `seed` to the ML-KEM and x25519 key pairs.
    pub(super) fn expand_decap_key(
        seed: &[u8; 32],
    ) -> Result<(MlKemKeyPair<2400, 1184>, X25519PublicKey, X25519PrivateKey), Error> {
        let expanded: [u8; 96] = libcrux_sha3::shake256(seed);
        let kp_m =
            mlkem768::generate_key_pair(expanded[..64].try_into().map_err(|_| Error::KeyGen)?);
        let mut sk_x = [0u8; 32];
        sk_x.copy_from_slice(&expanded[64..]);
        let sk_x = X25519PrivateKey::from(&sk_x); // clamps
        let pk_x = x25519_secret_to_public(&sk_x)?;
        Ok((kp_m, pk_x, sk_x))
    }

    pub(super) fn combiner(
        ss_m: &[u8],
        ss_x: &[u8],
        ct_x: &[u8],
        pk_x: &[u8],
    ) -> XWingSharedSecret {
        // label:
        // \./
        // /^\
        // 5c2e2f2f5e5c
        let mut input = ss_m.to_vec();
        input.extend_from_slice(ss_x);
        input.extend_from_slice(ct_x);
        input.extend_from_slice(pk_x);
        input.extend_from_slice(&[0x5c, 0x2e, 0x2f, 0x2f, 0x5e, 0x5c]);
        XWingSharedSecret {
            value: sha3::sha256(&input),
        }
    }

    impl<'a> TryFrom<&'a [u8]> for XWingSharedSecret {
        type Error = <[u8; 32] as TryFrom<&'a [u8]>>::Error;

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            Ok(Self {
                value: value.try_into()?,
            })
        }
    }
}

fn mlkem_rand(rng: &mut impl CryptoRng) -> Result<[u8; libcrux_ml_kem::SHARED_SECRET_SIZE], Error> {
    let mut seed = [0; libcrux_ml_kem::SHARED_SECRET_SIZE];
    rng.try_fill_bytes(&mut seed).map_err(|_| Error::KeyGen)?;
    Ok(seed)
}

impl TryInto<libcrux_ecdh::X25519PublicKey> for PublicKey {
    type Error = libcrux_ecdh::Error;

    fn try_into(self) -> Result<libcrux_ecdh::X25519PublicKey, libcrux_ecdh::Error> {
        if let PublicKey::X25519(k) = self {
            Ok(k)
        } else {
            Err(libcrux_ecdh::Error::InvalidPoint)
        }
    }
}

impl TryInto<libcrux_ecdh::X25519PrivateKey> for PrivateKey {
    type Error = libcrux_ecdh::Error;

    fn try_into(self) -> Result<libcrux_ecdh::X25519PrivateKey, libcrux_ecdh::Error> {
        if let PrivateKey::X25519(k) = self {
            Ok(k)
        } else {
            Err(libcrux_ecdh::Error::InvalidPoint)
        }
    }
}
