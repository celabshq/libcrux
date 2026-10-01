use libcrux_ml_dsa::{ml_dsa_44, ml_dsa_65, ml_dsa_87};

macro_rules! impl_sign_mut_dirty_buffer_test {
    ($name:ident, $platform:path, $signature:ty $(, $signing_key_view:ident)?) => {
        #[test]
        fn $name() {
            use $platform as platform;

            let key_pair = platform::generate_key_pair([7; 32]);
            let message = b"sign_mut output buffer";
            let randomness = [3; 32];

            let expected = platform::sign(&key_pair.signing_key, message, b"", randomness).unwrap();

            for fill in [0x00, 0x5a, 0xff] {
                let mut signature = [fill; <$signature>::len()];
                platform::sign_mut(
                    &key_pair.signing_key$(.$signing_key_view())?,
                    message,
                    b"",
                    randomness,
                    &mut signature,
                )
                .unwrap();

                assert_eq!(&signature, expected.as_ref());
                platform::verify(
                    &key_pair.verification_key,
                    message,
                    b"",
                    &<$signature>::new(signature),
                )
                .unwrap();
            }
        }
    };
}

impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_44,
    ml_dsa_44::portable,
    ml_dsa_44::MLDSA44Signature
);
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_65,
    ml_dsa_65::portable,
    ml_dsa_65::MLDSA65Signature,
    as_ref
);
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_87,
    ml_dsa_87::portable,
    ml_dsa_87::MLDSA87Signature
);

#[cfg(feature = "simd128")]
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_44_simd128,
    ml_dsa_44::neon,
    ml_dsa_44::MLDSA44Signature
);
#[cfg(feature = "simd128")]
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_65_simd128,
    ml_dsa_65::neon,
    ml_dsa_65::MLDSA65Signature,
    as_ref
);
#[cfg(feature = "simd128")]
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_87_simd128,
    ml_dsa_87::neon,
    ml_dsa_87::MLDSA87Signature
);

#[cfg(feature = "simd256")]
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_44_simd256,
    ml_dsa_44::avx2,
    ml_dsa_44::MLDSA44Signature
);
#[cfg(feature = "simd256")]
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_65_simd256,
    ml_dsa_65::avx2,
    ml_dsa_65::MLDSA65Signature,
    as_ref
);
#[cfg(feature = "simd256")]
impl_sign_mut_dirty_buffer_test!(
    sign_mut_dirty_buffer_87_simd256,
    ml_dsa_87::avx2,
    ml_dsa_87::MLDSA87Signature
);
