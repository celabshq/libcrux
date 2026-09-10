mod util;

#[cfg(feature = "rand")]
mod rand {
    use libcrux_ecdsa::{
        p256::{Nonce, PrivateKey, PublicKey},
        *,
    };
    use rand::rngs::SysRng;
    use rand_core::{TryRng, UnwrapErr};

    use crate::util::*;

    /// An RNG that always fails, to exercise the fallible-`TryCryptoRng`
    /// error path.
    struct AlwaysFailingRng;

    #[derive(Debug)]
    struct AlwaysFailingError;

    impl core::fmt::Display for AlwaysFailingError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "always fails")
        }
    }

    impl core::error::Error for AlwaysFailingError {}

    impl TryRng for AlwaysFailingRng {
        type Error = AlwaysFailingError;

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Err(AlwaysFailingError)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Err(AlwaysFailingError)
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Err(AlwaysFailingError)
        }
    }

    impl rand::TryCryptoRng for AlwaysFailingRng {}

    #[test]
    fn generate_key_pair_and_sign_verify() {
        let mut rng = UnwrapErr(SysRng);

        let (sk, pk) = p256::rand::generate_key_pair(&mut rng).unwrap();
        assert_eq!(&sk.public_key().unwrap().0, &pk.0);

        let msg = b"a message to sign";
        let sig = p256::rand::sign(DigestAlgorithm::Sha256, msg, &sk, &mut rng).unwrap();
        p256::verify(DigestAlgorithm::Sha256, msg, &sig, &pk).unwrap();
    }

    #[test]
    fn fallible_rng_error_propagates() {
        let mut rng = AlwaysFailingRng;
        let error = match p256::rand::generate_key_pair(&mut rng) {
            Ok(_) => panic!("the RNG always fails, so key generation must fail too"),
            Err(error) => error,
        };
        assert!(matches!(error, Error::RandError));
    }

    #[test]
    fn test_self() {
        // From https://tools.ietf.org/html/rfc6979#appendix-A.2.5
        const PK_HEX: &str = "0460FED4BA255A9D31C961EB74C6356D68C049B8923B61FA6CE669622E60F29FB67903FE1008B8BC99A41AE9E95628BC64F2F1B20C2D7E9F5177A3C294D4462299";
        const SK_HEX: &str = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";

        let mut rng = UnwrapErr(SysRng);

        let pk = hex_str_to_bytes(PK_HEX);
        let pk = PublicKey::try_from(pk.as_slice()).unwrap();
        let sk: [u8; 32] = hex_str_to_array(SK_HEX);
        let sk = PrivateKey::try_from(&sk).unwrap();
        let nonce = Nonce::random(&mut rng).unwrap();
        let msg = b"sample";

        let sig = p256::sign(DigestAlgorithm::Sha256, &msg[..], &sk, &nonce).unwrap();
        p256::verify(DigestAlgorithm::Sha256, &msg[..], &sig, &pk).unwrap();

        let new_msg = b"a different message";
        let sig = p256::sign(DigestAlgorithm::Sha256, &new_msg[..], &sk, &nonce).unwrap();
        let error = p256::verify(DigestAlgorithm::Sha256, &msg[..], &sig, &pk)
            .expect_err("The message is wrong for the signature");
        assert!(matches!(error, Error::InvalidSignature));
    }
}
