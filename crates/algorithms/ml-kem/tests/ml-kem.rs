//! Tests for ML-KEM

/// These tests are from https://github.com/C2SP/CCTV/
#[allow(dead_code)]
fn test_invalid_modulus(p: &str) {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let kat_file_path = file_name(p);
    let kat_file = File::open(kat_file_path).unwrap();
    let reader = BufReader::new(kat_file);
    for line in reader.lines() {
        let line = line.unwrap();
        let pk = hex::decode(line).unwrap();
        #[allow(unused_variables)]
        let pk = pk.as_slice();
        match p {
            #[cfg(feature = "mlkem512")]
            "512" => assert!(!libcrux_ml_kem::mlkem512::validate_public_key(
                &pk.try_into().unwrap()
            )),
            #[cfg(feature = "mlkem768")]
            "768" => assert!(!libcrux_ml_kem::mlkem768::validate_public_key(
                &pk.try_into().unwrap()
            )),
            #[cfg(feature = "mlkem1024")]
            "1024" => assert!(!libcrux_ml_kem::mlkem1024::validate_public_key(
                &pk.try_into().unwrap()
            )),
            _ => unreachable!(),
        };
    }
}

#[test]
#[cfg(feature = "mlkem512")]
fn invalid_modulus_512() {
    test_invalid_modulus("512");
}

#[test]
#[cfg(feature = "mlkem768")]
fn invalid_modulus_768() {
    test_invalid_modulus("768");
}

#[test]
#[cfg(feature = "mlkem1024")]
fn invalid_modulus_1024() {
    test_invalid_modulus("1024");
}

fn file_name(p: &str) -> std::path::PathBuf {
    std::path::Path::new("tests")
        .join("kats")
        .join("invalid_modulus")
        .join(format!("ML-KEM-{}.txt", p))
}

#[allow(dead_code)]
fn test_invalid_dk(p: &str) {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn dk_file(p: &str) -> std::path::PathBuf {
        std::path::Path::new("tests")
            .join("kats")
            .join("invalid_dk")
            .join(format!("ML-KEM-{}.txt", p))
    }

    let kat_file_path = dk_file(p);
    let kat_file = File::open(kat_file_path).unwrap();
    let reader = BufReader::new(kat_file);
    let mut lines = reader.lines();
    while let (Some(dk_string), Some(ct_string)) = (lines.next(), lines.next()) {
        let dk = hex::decode(dk_string.unwrap()).unwrap();
        #[allow(unused_variables)]
        let dk = dk.as_slice();
        let ct = hex::decode(ct_string.unwrap()).unwrap();
        #[allow(unused_variables)]
        let ct = ct.as_slice();
        match p {
            #[cfg(feature = "mlkem512")]
            "512" => assert!(!libcrux_ml_kem::mlkem512::validate_private_key(
                &dk.try_into().unwrap(),
                &ct.try_into().unwrap(),
            )),
            #[cfg(feature = "mlkem768")]
            "768" => assert!(!libcrux_ml_kem::mlkem768::validate_private_key(
                &dk.try_into().unwrap(),
                &ct.try_into().unwrap(),
            )),
            #[cfg(feature = "mlkem1024")]
            "1024" => assert!(!libcrux_ml_kem::mlkem1024::validate_private_key(
                &dk.try_into().unwrap(),
                &ct.try_into().unwrap(),
            )),
            _ => unreachable!(),
        };
    }
}

#[test]
#[cfg(feature = "mlkem512")]
fn invalid_dk_512() {
    test_invalid_dk("512");
}

#[test]
#[cfg(feature = "mlkem768")]
fn invalid_dk_768() {
    test_invalid_dk("768");
}

#[test]
#[cfg(feature = "mlkem1024")]
fn invalid_dk_1024() {
    test_invalid_dk("1024");
}
