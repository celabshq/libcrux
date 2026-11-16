use libcrux_ecdh::Algorithm;
use libcrux_kats::acvp::{
    ec::{pad, raw_point},
    kas_ecc::KasEccSscTests,
    schema_common::{assert_covers, Tally},
};

const FIELD_LEN: usize = 32;

fn private_key(d: &[u8]) -> [u8; FIELD_LEN] {
    pad(d, FIELD_LEN).unwrap().try_into().unwrap()
}

/// The point (x, y) as `X || Y`.
fn point(x: &[u8], y: &[u8]) -> Vec<u8> {
    raw_point(x, y, FIELD_LEN).unwrap()
}

/// The shared secret `Z` of SP 800-56A, i.e. the x-coordinate of the shared point.
///
/// [`libcrux_ecdh::derive`] returns the shared point `X || Y` for P-256.
fn derive_z(server_public_key: Vec<u8>, sk: [u8; FIELD_LEN]) -> Option<Vec<u8>> {
    let point = libcrux_ecdh::derive(Algorithm::P256, server_public_key, sk).ok()?;
    Some(point[..FIELD_LEN].to_vec())
}

#[test]
fn kas_ecc_ssc_p256() {
    let tests = KasEccSscTests::p256();
    assert_eq!(tests.prompts.algorithm, "KAS-ECC-SSC");
    assert_eq!(tests.prompts.revision, "Sp800-56Ar3");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.domainParameterGenerationMode, "P-256");
        let (iut_ephemeral, server_ephemeral) = (group.iut_ephemeral(), group.server_ephemeral());

        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let (server_x, server_y) = test.server_public_key(server_ephemeral);
            let server_public_key = point(server_x, server_y);
            match group.testType.as_str() {
                "AFT" => {
                    // The IUT key pair is chosen by the IUT, the projection contains the one
                    // the expected results were computed with.
                    let projection = tests.find_projection(group.tgId, test.tcId);
                    let sk = private_key(projection.iut_private_key(iut_ephemeral));

                    let (x, y) = expected.iut_public_key(iut_ephemeral);
                    let pk = libcrux_ecdh::secret_to_public(Algorithm::P256, sk).unwrap();
                    assert_eq!(pk, point(x, y), "tcId {}", test.tcId);

                    let z = derive_z(server_public_key, sk).unwrap();
                    assert_eq!(
                        Some(z),
                        pad(expected.z.as_ref().unwrap(), FIELD_LEN),
                        "tcId {}",
                        test.tcId
                    );
                    tally.add(format!("AFT {}", group.scheme));
                }
                "VAL" => {
                    let sk = private_key(test.iut_private_key(iut_ephemeral));
                    let (x, y) = test.iut_public_key(iut_ephemeral);

                    let passed = libcrux_ecdh::secret_to_public(Algorithm::P256, sk)
                        .is_ok_and(|derived| derived == point(x, y))
                        && derive_z(server_public_key, sk)
                            .is_some_and(|z| Some(z) == pad(&test.z, FIELD_LEN));
                    assert_eq!(Some(passed), expected.testPassed, "tcId {}", test.tcId);
                    tally.add(format!(
                        "VAL {} {}",
                        group.scheme,
                        if passed { "pass" } else { "fail" }
                    ));
                }
                t => panic!("unexpected testType {t}"),
            }
        }
    }
    tally.assert_counts([
        ("AFT ephemeralUnified", 5),
        ("AFT onePassDh", 5),
        ("VAL ephemeralUnified pass", 4),
        ("VAL ephemeralUnified fail", 1),
        ("VAL onePassDh pass", 4),
        ("VAL onePassDh fail", 1),
    ]);
}
