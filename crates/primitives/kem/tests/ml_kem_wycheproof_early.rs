use libcrux_kem::{
    self, MlKem1024PrivateKey, MlKem1024PublicKey, MlKem512PrivateKey, MlKem512PublicKey,
    MlKem768PrivateKey, MlKem768PublicKey, MlKemCiphertext,
};

use std::path::Path;

use std::{fs::File, io::BufRead, io::BufReader, panic};

fn parse_bytes<const N: usize>(s: &str, val: &str) -> [u8; N] {
    let value_start = match s.find(val) {
        Some(val) => val,
        None => {
            panic!("Didn't find '{val}' on '{s}'");
        }
    };
    let value_start = value_start + val.len() + 3;
    let value = hex::decode(&s[value_start..]).unwrap();
    value.try_into().unwrap()
}

macro_rules! impl_known_answer_test {
    ($name:ident,
     $parameter_set: literal,
     $key_gen_derand:expr,
     $encapsulate_derand:expr,
     $decapsulate_derand: expr,
     $validate_pk: expr,
     $pk: ty,
     $sk: ty,
     $ct_len: literal,
     $pk_len: literal,
     $sk_len: literal) => {
        #[test]
        fn $name() {
            // Encaps
            let katfile_path = Path::new("tests")
                .join("kats")
                .join("wycheproof_early")
                .join(format!("encaps{}draft", $parameter_set));
            let katfile = File::open(katfile_path).expect("Could not open KAT file.");
            let reader = BufReader::new(katfile);
            let mut lines = reader.lines();

            while let (
                Some(comment),
                Some(entropy),
                Some(public_key),
                Some(expected_result),
                Some(expected_ciphertext),
                Some(expected_shared_secret),
                _, // Always read the empty line after the test.
            ) = (
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
            ) {
                let comment = &comment.unwrap()["comment".len() + 3..];
                println!("{}", comment);
                let pass = expected_result.unwrap() == "pass";

                let entropy: [u8; 32] = parse_bytes(entropy.as_ref().unwrap(), "entropy");

                let pk = public_key.as_ref().unwrap();
                if !pass
                    && (comment == "Public key is too short" || comment == "Public key is too long")
                {
                    // The parsing fails already and we require the sized array.
                    let value_start = pk.find("public_key").unwrap();
                    let value_start = value_start + "public_key".len() + 3;
                    let pk = hex::decode(&pk[value_start..]).unwrap();
                    assert_ne!(pk.len(), $pk_len);
                    continue;
                }

                let pk: [u8; $pk_len] = parse_bytes(&public_key.unwrap(), "public_key");
                let (my_ciphertext, my_shared_secret) =
                    $encapsulate_derand(&<$pk>::from(pk), entropy);

                if pass {
                    let expected_ciphertext: [u8; $ct_len] =
                        parse_bytes(&expected_ciphertext.unwrap(), "expected_ciphertext");
                    let expected_shared_secret: [u8; 32] =
                        parse_bytes(&expected_shared_secret.unwrap(), "expected_shared_secret");

                    assert_eq!(my_ciphertext.as_ref(), expected_ciphertext);
                    assert_eq!(my_shared_secret.as_ref(), expected_shared_secret);
                } else {
                    if comment == "Public key not reduced" {
                        assert!(!$validate_pk(&<$pk>::from(pk)));
                    }
                }
            }

            // Decaps
            let katfile_path = Path::new("tests")
                .join("kats")
                .join("wycheproof_early")
                .join(format!("decaps{}draft", $parameter_set));
            let katfile = File::open(katfile_path).expect("Could not open KAT file.");
            let reader = BufReader::new(katfile);
            let mut lines = reader.lines();

            while let (
                Some(comment),
                Some(private_key),
                Some(ciphertext),
                Some(expected_result),
                Some(expected_shared_secret),
                _, // Always read the empty line after the test.
            ) = (
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
                lines.next(),
            ) {
                let comment = &comment.unwrap()["comment".len() + 3..];
                println!("{}", comment);
                let pass = expected_result.unwrap() == "pass";

                let private_key = private_key.as_ref().unwrap();
                let ciphertext = ciphertext.as_ref().unwrap();
                if !pass {
                    if comment == "Private key too short" || comment == "Private key too long" {
                        // The parsing fails already and we require the sized array.
                        let value_start = private_key.find("private_key").unwrap();
                        let value_start = value_start + "private_key".len() + 3;
                        let sk = hex::decode(&private_key[value_start..]).unwrap();
                        assert_ne!(sk.len(), $sk_len);
                        continue;
                    }
                    if comment == "Ciphertext too short" || comment == "Ciphertext too long" {
                        // The parsing fails already and we require the sized array.
                        let value_start = ciphertext.find("ciphertext").unwrap();
                        let value_start = value_start + "ciphertext".len() + 3;
                        let ct = hex::decode(&ciphertext[value_start..]).unwrap();
                        assert_ne!(ct.len(), $ct_len);
                        continue;
                    }
                }

                let ciphertext: [u8; $ct_len] = parse_bytes(ciphertext, "ciphertext");
                let private_key: [u8; $sk_len] = parse_bytes(private_key, "private_key");
                let my_shared_secret = $decapsulate_derand(
                    &<$sk>::from(private_key),
                    &MlKemCiphertext::<$ct_len>::from(ciphertext),
                );

                if pass {
                    let expected_shared_secret: [u8; 32] =
                        parse_bytes(&expected_shared_secret.unwrap(), "expected_shared_secret");
                    assert_eq!(my_shared_secret, expected_shared_secret);
                } else {
                    if comment == "Private key not reduced" {
                        // We don't check the private key.
                    }
                }
            }
        }
    };
}

impl_known_answer_test!(
    ml_kem512_wycheproof_early_kat,
    512,
    libcrux_kem::deterministic::mlkem512_generate_keypair_derand,
    libcrux_kem::deterministic::mlkem512_encapsulate_derand,
    libcrux_kem::deterministic::mlkem512_decapsulate_derand,
    libcrux_kem::ml_kem512_validate_public_key,
    MlKem512PublicKey,
    MlKem512PrivateKey,
    768,
    800,
    1632
);
impl_known_answer_test!(
    ml_kem768_wycheproof_early_kat,
    768,
    libcrux_kem::deterministic::mlkem768_generate_keypair_derand,
    libcrux_kem::deterministic::mlkem768_encapsulate_derand,
    libcrux_kem::deterministic::mlkem768_decapsulate_derand,
    libcrux_kem::ml_kem768_validate_public_key,
    MlKem768PublicKey,
    MlKem768PrivateKey,
    1088,
    1184,
    2400
);
impl_known_answer_test!(
    ml_kem1024_wycheproof_early_kat,
    1024,
    libcrux_kem::deterministic::mlkem1024_generate_keypair_derand,
    libcrux_kem::deterministic::mlkem1024_encapsulate_derand,
    libcrux_kem::deterministic::mlkem1024_decapsulate_derand,
    libcrux_kem::ml_kem1024_validate_public_key,
    MlKem1024PublicKey,
    MlKem1024PrivateKey,
    1568,
    1568,
    3168
);
