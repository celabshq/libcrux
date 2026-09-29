use std::time::{Duration, Instant};

use hpke_rs::{prelude::*, test_util::hex_to_bytes};
use hpke_rs_crypto::{
    types::{AeadAlgorithm, KdfAlgorithm, KemAlgorithm},
    HpkeDefaultPrng,
};
use hpke_rs_libcrux::HpkeLibcrux;
use hpke_rs_rust_crypto::*;
use rand::Rng;

fn duration(d: Duration) -> f64 {
    ((d.as_secs() as f64) + (d.subsec_nanos() as f64 * 1e-9)) * 1000000f64
}

const MODES: [Mode; 4] = [
    HpkeMode::Base,
    HpkeMode::Auth,
    HpkeMode::Psk,
    HpkeMode::AuthPsk,
];
const AEAD_IDS: [AeadAlgorithm; 3] = [
    AeadAlgorithm::Aes128Gcm,
    AeadAlgorithm::Aes256Gcm,
    AeadAlgorithm::ChaCha20Poly1305,
];
const KDF_IDS: [KdfAlgorithm; 3] = [
    KdfAlgorithm::HkdfSha256,
    KdfAlgorithm::HkdfSha384,
    KdfAlgorithm::HkdfSha512,
];
const KEM_IDS: [KemAlgorithm; 6] = [
    KemAlgorithm::DhKemP256,
    KemAlgorithm::DhKemK256,
    KemAlgorithm::DhKemP384,
    KemAlgorithm::DhKemP521,
    KemAlgorithm::DhKem25519,
    KemAlgorithm::DhKem448,
];

const AEAD_PAYLOAD: usize = 128;
const AEAD_AAD: usize = 48;

const ITERATIONS: usize = 1;

fn benchmark<Crypto: HpkeDefaultPrng + 'static>() {
    for hpke_mode in MODES {
        for aead_mode in AEAD_IDS {
            if Crypto::supports_aead(aead_mode).is_err() {
                continue;
            }
            for kdf_mode in KDF_IDS {
                if Crypto::supports_kdf(kdf_mode).is_err() {
                    continue;
                }
                for kem_mode in KEM_IDS {
                    if Crypto::supports_kem(kem_mode).is_err() {
                        continue;
                    }
                    let mut hpke = Hpke::<Crypto>::new(hpke_mode, kem_mode, kdf_mode, aead_mode);
                    let label = format!(
                        "{} {} {} {} {}",
                        Crypto::name(),
                        hpke_mode,
                        kem_mode,
                        kdf_mode,
                        aead_mode
                    );
                    println!("{}", label);

                    let kp_r = hpke.generate_key_pair().unwrap();
                    let sk_rm = kp_r.private_key();
                    let pk_rm = kp_r.public_key();
                    let info = hex_to_bytes("4f6465206f6e2061204772656369616e2055726e");
                    let psk = if hpke_mode == HpkeMode::AuthPsk || hpke_mode == HpkeMode::Psk {
                        Some(hex_to_bytes(
                            "0247fd33b913760fa1fa51e1892d9f307fbe65eb171e8132c2af18555a738b82",
                        ))
                    } else {
                        None
                    };
                    let psk_id = if hpke_mode == HpkeMode::AuthPsk || hpke_mode == HpkeMode::Psk {
                        Some(hex_to_bytes("456e6e796e20447572696e206172616e204d6f726961"))
                    } else {
                        None
                    };
                    let (pk_sm, sk_sm) =
                        if hpke_mode == HpkeMode::AuthPsk || hpke_mode == HpkeMode::Auth {
                            let kp = hpke.generate_key_pair().unwrap();
                            (
                                Some(kp.public_key().clone()),
                                Some(kp.private_key().clone()),
                            )
                        } else {
                            (None, None)
                        };

                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        let mut hpke =
                            Hpke::<Crypto>::new(hpke_mode, kem_mode, kdf_mode, aead_mode);
                        let _sender = hpke
                            .setup_sender(
                                pk_rm,
                                &info,
                                psk.as_ref().map(Vec::as_ref),
                                psk_id.as_ref().map(Vec::as_ref),
                                sk_sm.as_ref(),
                            )
                            .unwrap();
                    }
                    let end = Instant::now();
                    let time = duration(end.duration_since(start));
                    println!("\tSetup Sender: {:.4}μs", time / (ITERATIONS as f64));

                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        let mut hpke =
                            Hpke::<Crypto>::new(hpke_mode, kem_mode, kdf_mode, aead_mode);
                        let (encapsulation, _) = hpke
                            .setup_sender(
                                pk_rm,
                                &info,
                                psk.as_ref().map(Vec::as_ref),
                                psk_id.as_ref().map(Vec::as_ref),
                                sk_sm.as_ref(),
                            )
                            .unwrap();
                        let _receiver = hpke
                            .setup_receiver(
                                &encapsulation,
                                sk_rm,
                                &info,
                                psk.as_ref().map(Vec::as_ref),
                                psk_id.as_ref().map(Vec::as_ref),
                                pk_sm.as_ref(),
                            )
                            .unwrap();
                    }
                    let end = Instant::now();
                    let time = duration(end.duration_since(start));
                    println!("\tSetup Receiver: {:.4}μs", time / (ITERATIONS as f64));

                    let (enc, mut context) = hpke
                        .setup_sender(
                            pk_rm,
                            &info,
                            psk.as_ref().map(Vec::as_ref),
                            psk_id.as_ref().map(Vec::as_ref),
                            sk_sm.as_ref(),
                        )
                        .unwrap();
                    let mut aad = vec![0u8; AEAD_AAD];
                    rand::rng().fill_bytes(&mut aad);
                    let mut ptxt = vec![0u8; AEAD_PAYLOAD];
                    rand::rng().fill_bytes(&mut ptxt);

                    let mut ctxts = Vec::with_capacity((AEAD_PAYLOAD + 16) * ITERATIONS);
                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        let ctxt = context.seal(&aad, &ptxt).unwrap();
                        ctxts.push(ctxt);
                    }
                    let end = Instant::now();
                    let time = duration(end.duration_since(start));
                    println!(
                        "\tSeal {}({}): {:.4}μs",
                        AEAD_PAYLOAD,
                        AEAD_AAD,
                        time / (ITERATIONS as f64)
                    );

                    let mut context = hpke
                        .setup_receiver(
                            &enc,
                            sk_rm,
                            &info,
                            psk.as_ref().map(Vec::as_ref),
                            psk_id.as_ref().map(Vec::as_ref),
                            pk_sm.as_ref(),
                        )
                        .unwrap();

                    let mut ptxts = Vec::with_capacity((AEAD_PAYLOAD + 16) * ITERATIONS);
                    let start = Instant::now();
                    for ctxt in ctxts.iter() {
                        let ptxt_out = context.open(&aad, ctxt).unwrap();
                        ptxts.push(ptxt_out);
                    }
                    let end = Instant::now();
                    let time = duration(end.duration_since(start));
                    println!(
                        "\tOpen {}({}): {:.4}μs",
                        AEAD_PAYLOAD,
                        AEAD_AAD,
                        time / (ITERATIONS as f64)
                    );
                    assert_eq!(ptxts[0], ptxt);

                    let mut aad = vec![0u8; AEAD_AAD];
                    rand::rng().fill_bytes(&mut aad);
                    let mut ptxt = vec![0u8; AEAD_PAYLOAD];
                    rand::rng().fill_bytes(&mut ptxt);

                    let mut enc = Vec::<u8>::new();
                    let mut ctxt = Vec::<u8>::new();
                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        let (new_enc, new_ctxt) = hpke
                            .seal(
                                pk_rm,
                                &info,
                                &aad,
                                &ptxt,
                                psk.as_ref().map(Vec::as_ref),
                                psk_id.as_ref().map(Vec::as_ref),
                                sk_sm.as_ref(),
                            )
                            .unwrap();
                        enc = new_enc;
                        ctxt = new_ctxt;
                    }
                    let end = Instant::now();
                    let time = duration(end.duration_since(start));
                    println!(
                        "\tSingle-Shot Seal {}({}): {:.4}μs",
                        AEAD_PAYLOAD,
                        AEAD_AAD,
                        time / (ITERATIONS as f64)
                    );

                    let mut ptxt_out = Vec::<u8>::new();
                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        ptxt_out = hpke
                            .open(
                                &enc,
                                sk_rm,
                                &info,
                                &aad,
                                &ctxt,
                                psk.as_ref().map(Vec::as_ref),
                                psk_id.as_ref().map(Vec::as_ref),
                                pk_sm.as_ref(),
                            )
                            .unwrap();
                    }
                    let end = Instant::now();
                    let time = duration(end.duration_since(start));
                    println!(
                        "\tSingle-Shot Open {}({}): {:.4}μs",
                        AEAD_PAYLOAD,
                        AEAD_AAD,
                        time / (ITERATIONS as f64)
                    );
                    assert_eq!(ptxt_out, ptxt);
                }
            }
        }
    }
}

fn main() {
    benchmark::<HpkeLibcrux>();
    benchmark::<HpkeRustCrypto>();
}
