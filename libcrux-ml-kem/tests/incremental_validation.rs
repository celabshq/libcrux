#![cfg(feature = "incremental")]

use rand::{rngs::SysRng, TryRng};

fn random_array<const L: usize>() -> [u8; L] {
    let mut rng = SysRng;
    let mut seed = [0; L];
    rng.try_fill_bytes(&mut seed).unwrap();
    seed
}

macro_rules! impl_incremental_validation {
    ($encaps:ident, $decaps:ident, $modp:ident) => {
        #[test]
        fn $encaps() {
            use libcrux_ml_kem::$modp::incremental::*;

            let mut key_pair_bytes = [0u8; key_pair_len()];
            generate_key_pair(random_array(), &mut key_pair_bytes).unwrap();

            let mut state = [0u8; encaps_state_len()];
            let mut shared_secret = [0u8; shared_secret_size()];
            encapsulate1(
                pk1(&key_pair_bytes),
                random_array(),
                &mut state,
                &mut shared_secret,
            )
            .unwrap();

            let mut pk2_bytes = [0u8; pk2_len()];
            pk2_bytes.copy_from_slice(pk2(&key_pair_bytes));
            assert!(encapsulate2(&state, &pk2_bytes).is_ok());

            // The state starts with the raw 16-bit coefficients of `r_as_ntt`.
            state[0] = 0xff;
            state[1] = 0x7f;
            let err = encapsulate2(&state, &pk2_bytes).err().unwrap();
            assert_eq!(format!("{err:?}"), "InvalidInput");
        }

        #[test]
        fn $decaps() {
            use libcrux_ml_kem::$modp::incremental::*;

            let mut key_pair_bytes = [0u8; key_pair_len()];
            generate_key_pair(random_array(), &mut key_pair_bytes).unwrap();

            let mut state = [0u8; encaps_state_len()];
            let mut shared_secret = [0u8; shared_secret_size()];
            let ct1 = encapsulate1(
                pk1(&key_pair_bytes),
                random_array(),
                &mut state,
                &mut shared_secret,
            )
            .unwrap();
            let mut pk2_bytes = [0u8; pk2_len()];
            pk2_bytes.copy_from_slice(pk2(&key_pair_bytes));
            let ct2 = encapsulate2(&state, &pk2_bytes).unwrap();

            assert_eq!(
                decapsulate_incremental_key(&key_pair_bytes, &ct1, &ct2).unwrap(),
                shared_secret
            );

            // The key pair is `pk1 | pk2 | raw 16-bit secret coefficients | ...`.
            let offset = pk1_len() + pk2_len();
            key_pair_bytes[offset] = 0xff;
            key_pair_bytes[offset + 1] = 0x7f;
            let err = decapsulate_incremental_key(&key_pair_bytes, &ct1, &ct2).unwrap_err();
            assert_eq!(format!("{err:?}"), "InvalidInput");
        }
    };
}

#[cfg(feature = "mlkem512")]
impl_incremental_validation!(
    encapsulate2_rejects_invalid_state_512,
    decapsulate_rejects_invalid_key_512,
    mlkem512
);
#[cfg(feature = "mlkem768")]
impl_incremental_validation!(
    encapsulate2_rejects_invalid_state_768,
    decapsulate_rejects_invalid_key_768,
    mlkem768
);
#[cfg(feature = "mlkem1024")]
impl_incremental_validation!(
    encapsulate2_rejects_invalid_state_1024,
    decapsulate_rejects_invalid_key_1024,
    mlkem1024
);
