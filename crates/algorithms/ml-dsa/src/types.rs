//! Common types

macro_rules! impl_struct {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone)]
        pub struct $name<const SIZE: usize> {
            pub(crate) value: [u8; SIZE],
        }

        impl<const SIZE: usize> $name<SIZE> {
            /// Init with zero
            pub const fn zero() -> Self {
                Self { value: [0u8; SIZE] }
            }

            /// Build
            pub const fn new(value: [u8; SIZE]) -> Self {
                Self { value }
            }

            /// A reference to the raw byte slice.
            pub const fn as_slice(&self) -> &[u8] {
                &self.value
            }

            /// A reference to the raw byte array.
            pub const fn as_ref(&self) -> &[u8; SIZE] {
                &self.value
            }

            /// The number of bytes
            pub const fn len() -> usize {
                SIZE
            }
        }
    };
}

impl_struct!(MLDSASigningKey, "An ML-DSA signature key.");
impl_struct!(MLDSAVerificationKey, "An ML-DSA verification key.");
impl_struct!(MLDSASignature, "An ML-DSA signature.");

macro_rules! impl_non_hax_types {
    ($name:ident) => {
        impl<const SIZE: usize> $name<SIZE> {
            /// A mutable reference to the raw byte slice.
            pub fn as_mut_slice(&mut self) -> &mut [u8] {
                &mut self.value
            }

            /// A mutable reference to the raw byte array.
            pub fn as_ref_mut(&mut self) -> &mut [u8; SIZE] {
                &mut self.value
            }
        }
    };
}

// Hax can't handle these.
mod non_hax_impls {
    use super::*;
    impl_non_hax_types!(MLDSASigningKey);
    impl_non_hax_types!(MLDSAVerificationKey);
    impl_non_hax_types!(MLDSASignature);
}

/// An ML-DSA key pair.
pub struct MLDSAKeyPair<const VERIFICATION_KEY_SIZE: usize, const SIGNING_KEY_SIZE: usize> {
    pub signing_key: MLDSASigningKey<SIGNING_KEY_SIZE>,
    pub verification_key: MLDSAVerificationKey<VERIFICATION_KEY_SIZE>,
}

#[cfg_attr(not(eurydice), derive(Debug))]
pub enum VerificationError {
    MalformedHintError,
    SignerResponseExceedsBoundError,
    CommitmentHashesDontMatchError,
    // FIXME: Eurydice can't handle enum variants with the same name
    // https://github.com/AeneasVerif/eurydice/issues/102
    VerificationContextTooLongError,
}

#[cfg_attr(not(eurydice), derive(Debug))]
pub enum SigningError {
    RejectionSamplingError,
    ContextTooLongError,
}

#[cfg(feature = "codec")]
mod codec {
    use super::*;

    macro_rules! impl_tls_codec_for_generic_struct {
        ($name:ident) => {
            // XXX: `tls_codec::{Serialize, Deserialize}` are only
            // available for feature `std`. For `no_std` scenarios, we
            // need to implement `tls_codec::{SerializeBytes,
            // DeserializeBytes}`, but `SerializeBytes` is not
            // implemented for `VLByteSlice`.
            impl<const SIZE: usize> tls_codec::DeserializeBytes for $name<SIZE> {
                fn tls_deserialize_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), tls_codec::Error> {
                    let (bytes, remainder) = tls_codec::VLBytes::tls_deserialize_bytes(bytes)?;
                    Ok((
                        Self {
                            value: bytes
                                .as_ref()
                                .try_into()
                                .map_err(|_| tls_codec::Error::InvalidInput)?,
                        },
                        remainder,
                    ))
                }
            }

            #[cfg(feature = "std")]
            impl<const SIZE: usize> tls_codec::Serialize for $name<SIZE> {
                fn tls_serialize<W: std::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> Result<usize, tls_codec::Error> {
                    let out = tls_codec::VLByteSlice(self.as_ref());
                    out.tls_serialize(writer)
                }
            }

            #[cfg(feature = "std")]
            impl<const SIZE: usize> tls_codec::Serialize for &$name<SIZE> {
                fn tls_serialize<W: std::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> Result<usize, tls_codec::Error> {
                    (*self).tls_serialize(writer)
                }
            }

            #[cfg(feature = "std")]
            impl<const SIZE: usize> tls_codec::Deserialize for $name<SIZE> {
                fn tls_deserialize<R: std::io::Read>(
                    bytes: &mut R,
                ) -> Result<Self, tls_codec::Error> {
                    let bytes = tls_codec::VLBytes::tls_deserialize(bytes)?;
                    Ok(Self {
                        value: bytes
                            .as_ref()
                            .try_into()
                            .map_err(|_| tls_codec::Error::InvalidInput)?,
                    })
                }
            }

            impl<const SIZE: usize> tls_codec::Size for $name<SIZE> {
                fn tls_serialized_len(&self) -> usize {
                    tls_codec::VLByteSlice(self.as_ref()).tls_serialized_len()
                }
            }

            impl<const SIZE: usize> tls_codec::Size for &$name<SIZE> {
                fn tls_serialized_len(&self) -> usize {
                    (*self).tls_serialized_len()
                }
            }
        };
    }

    impl_tls_codec_for_generic_struct!(MLDSAVerificationKey);
    impl_tls_codec_for_generic_struct!(MLDSASignature);
}
