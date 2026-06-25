macro_rules! impl_generic_struct {
    ($name:ident, $doc:expr) => {
        #[derive(Clone)]
        #[doc = $doc]
        pub struct $name<const SIZE: usize> {
            pub(crate) value: [u8; SIZE],
        }

        impl<const SIZE: usize> Default for $name<SIZE> {
            fn default() -> Self {
                Self { value: [0u8; SIZE] }
            }
        }

        #[hax_lib::attributes]
        impl<const SIZE: usize> AsRef<[u8]> for $name<SIZE> {
            #[ensures(|result| fstar!(r#"$result = ${self_}.f_value"#))]
            fn as_ref(&self) -> &[u8] {
                &self.value
            }
        }

        #[hax_lib::attributes]
        impl<const SIZE: usize> From<[u8; SIZE]> for $name<SIZE> {
            #[ensures(|result| fstar!(r#"${result}.f_value = $value"#))]
            fn from(value: [u8; SIZE]) -> Self {
                Self { value }
            }
        }

        impl<const SIZE: usize> From<&[u8; SIZE]> for $name<SIZE> {
            fn from(value: &[u8; SIZE]) -> Self {
                Self {
                    value: value.clone(),
                }
            }
        }

        impl<const SIZE: usize> From<$name<SIZE>> for [u8; SIZE] {
            fn from(value: $name<SIZE>) -> Self {
                value.value
            }
        }

        impl<const SIZE: usize> TryFrom<&[u8]> for $name<SIZE> {
            type Error = core::array::TryFromSliceError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                match value.try_into() {
                    Ok(value) => Ok(Self { value }),
                    Err(e) => Err(e),
                }
            }
        }

        #[hax_lib::attributes]
        impl<const SIZE: usize> $name<SIZE> {
            /// A reference to the raw byte slice.
            #[ensures(|result| fstar!(r#"$result == self.f_value"#))]
            pub fn as_slice(&self) -> &[u8; SIZE] {
                &self.value
            }

            // This is only used for some of the macro callers.
            // #[allow(dead_code)]
            // /// Split this value and return the raw byte slices.
            // pub(crate) fn split_at(&self, mid: usize) -> (&[u8], &[u8]) {
            //     self.value.split_at(mid)
            // }

            /// The number of bytes
            pub const fn len() -> usize {
                SIZE
            }
        }
    };
}
macro_rules! impl_index_impls_for_generic_struct {
    ($name:ident) => {
        #[hax_lib::attributes]
        impl<const SIZE: usize> core::ops::Index<usize> for $name<SIZE> {
            type Output = u8;

            #[requires(index < SIZE)]
            fn index(&self, index: usize) -> &Self::Output {
                &self.value[index]
            }
        }

        #[hax_lib::attributes]
        impl<const SIZE: usize> core::ops::Index<core::ops::Range<usize>> for $name<SIZE> {
            type Output = [u8];

            #[requires(range.start <= range.end && range.end <= SIZE)]
            fn index(&self, range: core::ops::Range<usize>) -> &Self::Output {
                &self.value[range]
            }
        }

        #[hax_lib::attributes]
        impl<const SIZE: usize> core::ops::Index<core::ops::RangeTo<usize>> for $name<SIZE> {
            type Output = [u8];

            #[requires(range.end <= SIZE)]
            fn index(&self, range: core::ops::RangeTo<usize>) -> &Self::Output {
                &self.value[range]
            }
        }

        #[hax_lib::attributes]
        impl<const SIZE: usize> core::ops::Index<core::ops::RangeFrom<usize>> for $name<SIZE> {
            type Output = [u8];

            #[requires(range.start <= SIZE)]
            fn index(&self, range: core::ops::RangeFrom<usize>) -> &Self::Output {
                &self.value[range]
            }
        }
    };
}

// IndexMut impls are not extracted to F*: hax cannot represent &mut
// indexing (issue hacspec/hax#420), so the extracted bodies were
// failure stubs that no longer typecheck against the new
// `Core_models.Ops.Index.IndexMut.t_IndexMut` shape. They have no
// other F* users, so we hide them from hax extraction here.
#[cfg(not(hax))]
macro_rules! impl_index_mut_impls_for_generic_struct {
    ($name:ident) => {
        impl<const SIZE: usize> core::ops::IndexMut<usize> for $name<SIZE> {
            fn index_mut(&mut self, range: usize) -> &mut Self::Output {
                &mut self.value[range]
            }
        }

        impl<const SIZE: usize> core::ops::IndexMut<core::ops::Range<usize>> for $name<SIZE> {
            fn index_mut(&mut self, range: core::ops::Range<usize>) -> &mut Self::Output {
                &mut self.value[range]
            }
        }

        impl<const SIZE: usize> core::ops::IndexMut<core::ops::RangeTo<usize>> for $name<SIZE> {
            fn index_mut(&mut self, range: core::ops::RangeTo<usize>) -> &mut Self::Output {
                &mut self.value[range]
            }
        }

        impl<const SIZE: usize> core::ops::IndexMut<core::ops::RangeFrom<usize>> for $name<SIZE> {
            fn index_mut(&mut self, range: core::ops::RangeFrom<usize>) -> &mut Self::Output {
                &mut self.value[range]
            }
        }
    };
}

impl_generic_struct!(MlKemCiphertext, "An ML-KEM Ciphertext");
impl_generic_struct!(MlKemPrivateKey, "An ML-KEM Private key");
impl_generic_struct!(MlKemPublicKey, "An ML-KEM Public key");

// These traits are used only in `ind_cpa` for kyber cipher text.
mod index_impls {
    use super::*;
    impl_index_impls_for_generic_struct!(MlKemCiphertext);
    impl_index_impls_for_generic_struct!(MlKemPrivateKey);
    impl_index_impls_for_generic_struct!(MlKemPublicKey);

    #[cfg(not(hax))]
    impl_index_mut_impls_for_generic_struct!(MlKemCiphertext);
    #[cfg(not(hax))]
    impl_index_mut_impls_for_generic_struct!(MlKemPrivateKey);
    #[cfg(not(hax))]
    impl_index_mut_impls_for_generic_struct!(MlKemPublicKey);
}

#[cfg(all(feature = "codec", feature = "alloc"))]
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

    impl_tls_codec_for_generic_struct!(MlKemCiphertext);
    impl_tls_codec_for_generic_struct!(MlKemPublicKey);

    #[cfg(test)]
    mod test {
        use tls_codec::{Deserialize, Serialize, Size};

        use super::*;

        #[test]
        #[cfg(feature = "std")]
        fn ser_de() {
            use tls_codec::DeserializeBytes;

            const SIZE: usize = 1568;
            let test_struct = MlKemCiphertext::<SIZE>::default();

            assert_eq!(test_struct.tls_serialized_len(), SIZE + 2);
            let test_struct_serialized = test_struct.tls_serialize_detached().unwrap();
            assert_eq!(
                test_struct_serialized.len(),
                test_struct.tls_serialized_len()
            );

            let test_struct_deserialized =
                MlKemCiphertext::<SIZE>::tls_deserialize_exact(&test_struct_serialized).unwrap();

            let test_struct_deserialized_bytes =
                MlKemCiphertext::<SIZE>::tls_deserialize_exact_bytes(&test_struct_serialized)
                    .unwrap();

            assert_eq!(test_struct.as_ref(), test_struct_deserialized.as_ref());
            assert_eq!(
                test_struct.as_ref(),
                test_struct_deserialized_bytes.as_ref()
            )
        }
    }
}

/// An ML-KEM key pair
pub struct MlKemKeyPair<const PRIVATE_KEY_SIZE: usize, const PUBLIC_KEY_SIZE: usize> {
    pub(crate) sk: MlKemPrivateKey<PRIVATE_KEY_SIZE>,
    pub(crate) pk: MlKemPublicKey<PUBLIC_KEY_SIZE>,
}

#[hax_lib::attributes]
impl<const PRIVATE_KEY_SIZE: usize, const PUBLIC_KEY_SIZE: usize>
    MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>
{
    /// Creates a new [`MlKemKeyPair`].
    pub fn new(sk: [u8; PRIVATE_KEY_SIZE], pk: [u8; PUBLIC_KEY_SIZE]) -> Self {
        Self {
            sk: sk.into(),
            pk: pk.into(),
        }
    }

    /// Create a new [`MlKemKeyPair`] from the secret and public key.
    #[ensures(|result| fstar!(r#"${result}.f_sk == $sk /\ ${result}.f_pk == $pk"#))]
    pub fn from(
        sk: MlKemPrivateKey<PRIVATE_KEY_SIZE>,
        pk: MlKemPublicKey<PUBLIC_KEY_SIZE>,
    ) -> Self {
        Self { sk, pk }
    }

    /// Get a reference to the [`MlKemPublicKey<PUBLIC_KEY_SIZE>`].
    pub fn public_key(&self) -> &MlKemPublicKey<PUBLIC_KEY_SIZE> {
        &self.pk
    }

    /// Get a reference to the [`MlKemPrivateKey<PRIVATE_KEY_SIZE>`].
    pub fn private_key(&self) -> &MlKemPrivateKey<PRIVATE_KEY_SIZE> {
        &self.sk
    }

    /// Get a reference to the raw public key bytes.
    pub fn pk(&self) -> &[u8; PUBLIC_KEY_SIZE] {
        self.pk.as_slice()
    }

    /// Get a reference to the raw private key bytes.
    pub fn sk(&self) -> &[u8; PRIVATE_KEY_SIZE] {
        self.sk.as_slice()
    }

    /// Separate this key into the public and private key.
    pub fn into_parts(
        self,
    ) -> (
        MlKemPrivateKey<PRIVATE_KEY_SIZE>,
        MlKemPublicKey<PUBLIC_KEY_SIZE>,
    ) {
        (self.sk, self.pk)
    }
}

/// Unpack an incoming private key into it's different parts.
///
/// We have this here in types to extract into a common core for C.
#[hax_lib::requires(fstar!(r#"Seq.length private_key >= 
                            v v_CPA_SECRET_KEY_SIZE + v v_PUBLIC_KEY_SIZE + 
                            v Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE"#))]
#[hax_lib::ensures(|result| fstar!(r#"
           let (ind_cpa_secret_key_s,rest) = split $private_key $CPA_SECRET_KEY_SIZE in
           let (ind_cpa_public_key_s,rest) = split rest $PUBLIC_KEY_SIZE in
           let (ind_cpa_public_key_hash_s,implicit_rejection_value_s) = split rest Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE in
           let (ind_cpa_secret_key,ind_cpa_public_key,ind_cpa_public_key_hash,implicit_rejection_value)
               = result in
           ind_cpa_secret_key_s == ind_cpa_secret_key /\
           ind_cpa_public_key_s == ind_cpa_public_key /\
           ind_cpa_public_key_hash_s == ind_cpa_public_key_hash /\
           implicit_rejection_value_s == implicit_rejection_value /\
           Seq.length ind_cpa_secret_key == v v_CPA_SECRET_KEY_SIZE /\
           Seq.length ind_cpa_public_key == v v_PUBLIC_KEY_SIZE /\
           Seq.length ind_cpa_public_key_hash == v Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE /\
           Seq.length implicit_rejection_value == 
           Seq.length private_key - 
             (v v_CPA_SECRET_KEY_SIZE + v v_PUBLIC_KEY_SIZE + v Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE)
           "#))]
pub(crate) fn unpack_private_key<const CPA_SECRET_KEY_SIZE: usize, const PUBLIC_KEY_SIZE: usize>(
    private_key: &[u8], // len: SECRET_KEY_SIZE
) -> (&[u8], &[u8], &[u8], &[u8]) {
    let (ind_cpa_secret_key, secret_key) = private_key.split_at(CPA_SECRET_KEY_SIZE);
    let (ind_cpa_public_key, secret_key) = secret_key.split_at(PUBLIC_KEY_SIZE);
    let (ind_cpa_public_key_hash, implicit_rejection_value) =
        secret_key.split_at(crate::constants::H_DIGEST_SIZE);
    (
        ind_cpa_secret_key,
        ind_cpa_public_key,
        ind_cpa_public_key_hash,
        implicit_rejection_value,
    )
}
