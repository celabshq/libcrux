//! ACVP AES Known Answer Tests
//!
//! These test vectors were generated with the `GenValAppRunner` of
//! <https://github.com/usnistgov/ACVP-Server> (see `.docker/acvp-server`), as of commit
//! [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d),
//! using the `registration.json` stored next to each `prompt.json`.
//!
//! - `ACVP-AES-ECB` (revision 1.0): AES-128 and AES-256 encryption, AFT and MCT. This is the
//!   prerequisite for AES-GCM.
//! - `ACVP-AES-GCM` (revision 1.0): AES-128 and AES-256 encryption and decryption, AFT, with a 96-bit
//!   externally generated IV, a 128-bit tag, and byte-oriented payload and AAD lengths.
//! - `ACVP-AES-CCM` (revision 1.0): AES-128 and AES-256 encryption and decryption, AFT, with a
//!   96-bit nonce, 64-bit (short) and 128-bit tags, and payloads of up to 256 bits. Ciphertexts
//!   include the tag.
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::acvp::aes::{EcbTests, GcmTests};
//!
//! let tests = GcmTests::aes256_iv96_tag128();
//!
//! for test_group in tests.prompts.testGroups {
//!     for test in test_group.tests {
//!         // retrieve the expected result for that test
//!         let expected_result = tests.results.find_expected_result(test_group.tgId, test.tcId);
//!     }
//! }
//! ```

pub mod ccm_schema;
pub mod ecb_schema;
pub mod gcm_schema;

/// The AES-ECB Monte Carlo test (MCT, SP 800-38A AESAVS 6.4.1) for encryption, starting from
/// `key` and the plaintext `pt`, with `encrypt` the AES block cipher. Returns the 100 outer
/// iterations.
pub fn ecb_mct<const KEY_LEN: usize>(
    key: &[u8],
    pt: &[u8],
    mut encrypt: impl FnMut(&[u8; KEY_LEN], &mut [u8; 16]),
) -> Vec<ecb_schema::EcbMctResult> {
    let mut key: [u8; KEY_LEN] = key.try_into().expect("wrong key length");
    let mut pt: [u8; 16] = pt.try_into().expect("wrong block length");
    let mut results = Vec::with_capacity(100);
    for _ in 0..100 {
        // CT[j] = AES(Key[i], PT[j]); PT[j+1] = CT[j]
        let mut prev = [0u8; 16];
        let mut block = pt;
        for _ in 0..1000 {
            prev = block;
            encrypt(&key, &mut block);
        }
        results.push(ecb_schema::EcbMctResult {
            key: key.to_vec(),
            pt: pt.to_vec(),
            ct: block.to_vec(),
        });

        // Key[i+1] = Key[i] xor (rightmost key length bits of CT[998] || CT[999])
        let cts = [prev, block].concat();
        for (k, c) in key.iter_mut().zip(&cts[cts.len() - KEY_LEN..]) {
            *k ^= c;
        }
        pt = block;
    }
    results
}

pub struct EcbTests {
    pub prompts: super::schema_common::Prompts<ecb_schema::EcbPromptTestGroup>,
    pub results: super::schema_common::Results<ecb_schema::ResultEcbTestGroup>,
}

impl EcbTests {
    pub fn aes128_encrypt() -> Self {
        load!("aes-ecb-1_0", "aes-128-encrypt")
    }

    pub fn aes256_encrypt() -> Self {
        load!("aes-ecb-1_0", "aes-256-encrypt")
    }
}

pub struct GcmTests {
    pub prompts: super::schema_common::Prompts<gcm_schema::GcmPromptTestGroup>,
    pub results: super::schema_common::Results<gcm_schema::ResultGcmTestGroup>,
}

impl GcmTests {
    pub fn aes128_iv96_tag128() -> Self {
        load!("aes-gcm-1_0", "aes-128-iv96-tag128")
    }

    pub fn aes256_iv96_tag128() -> Self {
        load!("aes-gcm-1_0", "aes-256-iv96-tag128")
    }
}

pub struct CcmTests {
    pub prompts: super::schema_common::Prompts<ccm_schema::CcmPromptTestGroup>,
    pub results: super::schema_common::Results<ccm_schema::ResultCcmTestGroup>,
}

impl CcmTests {
    pub fn aes128_256_iv96() -> Self {
        load!("aes-ccm-1_0", "aes-128-256-iv96")
    }
}
