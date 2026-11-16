use libcrux_secrets::{Classify, DeclassifyRef};

pub fn simple<
    const EK_LEN: usize,
    const DK_LEN: usize,
    const CT_LEN: usize,
    const SS_LEN: usize,
    const RAND_KEYGEN_LEN: usize,
    const RAND_ENCAPS_LEN: usize,
    Kem: super::arrayref::Kem<EK_LEN, DK_LEN, CT_LEN, SS_LEN, RAND_KEYGEN_LEN, RAND_ENCAPS_LEN>,
>() {
    let alice_keygen_rand = [1.classify(); RAND_KEYGEN_LEN];
    let bob_keygen_rand = [2.classify(); RAND_KEYGEN_LEN];

    let mut alice_dk = [0.classify(); DK_LEN];
    let mut alice_ek = [0; EK_LEN];

    let mut bob_dk = [0.classify(); DK_LEN];
    let mut bob_ek = [0; EK_LEN];

    Kem::keygen(&mut alice_ek, &mut alice_dk, &alice_keygen_rand)
        .expect("error generating alice's key pair");
    Kem::keygen(&mut bob_ek, &mut bob_dk, &bob_keygen_rand)
        .expect("error generating bob's key pair");

    let mut ct = [0; CT_LEN];
    let mut ss_alice = [0.classify(); SS_LEN];
    let rand_encaps = [3.classify(); RAND_ENCAPS_LEN];

    Kem::encaps(&mut ct, &mut ss_alice, &bob_ek, &rand_encaps).expect("error encapsulating");

    let mut ss_bob = [0.classify(); SS_LEN];

    Kem::decaps(&mut ss_bob, &ct, &bob_dk).expect("error decapsulating");

    assert_eq!(ss_alice.declassify_ref(), ss_bob.declassify_ref())
}
