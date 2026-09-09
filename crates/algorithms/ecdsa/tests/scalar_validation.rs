use libcrux_ecdsa::{p256::PrivateKey, Error};

// Ensure that private keys that are too long are not truncated silently.
#[test]
fn private_key_rejects_over_length_scalar() {
    let over_length = vec![0x42u8; 40];
    assert!(matches!(
        PrivateKey::try_from(over_length.as_slice()),
        Err(Error::InvalidScalar)
    ));
}

#[test]
fn private_key_rejects_empty_scalar() {
    assert!(matches!(
        PrivateKey::try_from([].as_slice()),
        Err(Error::InvalidScalar)
    ));
}
