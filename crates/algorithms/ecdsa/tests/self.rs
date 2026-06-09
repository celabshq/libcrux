mod util;

#[cfg(feature = "rand")]
mod rand {
    use libcrux_ecdsa::{
        p256::{Nonce, PrivateKey, PublicKey},
        *,
    };
    use rand::rngs::SysRng;
    use rand_core::UnwrapErr;

    use crate::util::*;

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
