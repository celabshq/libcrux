use super::*;

/// Header from a single-section HMAC_DRBG .rsp file.
///
/// Each section begins with lines like `[SHA-256]`, `[PredictionResistance = False]`, etc.
#[derive(Debug, Clone, Default)]
pub struct HmacDrbgHeader {
    /// Hash algorithm: "SHA-256", "SHA-384", or "SHA-512".
    pub hash: String,
    pub prediction_resistance: bool,
    pub entropy_input_len: usize,
    pub nonce_len: usize,
    pub personalization_string_len: usize,
    pub additional_input_len: usize,
    pub returned_bits_len: usize,
}

/// One HMAC_DRBG test case.
///
/// NIST CAVP vectors for each COUNT have two generate calls; only the second output
/// (`returned_bits`) is checked.
#[derive(Debug, Clone, Default)]
pub struct HmacDrbgTest {
    /// Hash algorithm inherited from the section header: "SHA-256", "SHA-384", "SHA-512", etc.
    pub hash: String,
    /// Whether prediction resistance is enabled for this section.
    pub prediction_resistance: bool,
    pub count: usize,
    pub entropy_input: Vec<u8>,
    pub nonce: Vec<u8>,
    pub personalization_string: Vec<u8>,
    /// The two `AdditionalInput` lines (first for generate call 1, second for call 2).
    pub additional_input: Vec<Vec<u8>>,
    pub returned_bits: Vec<u8>,
    /// Present only in reseed test cases.
    pub entropy_input_reseed: Vec<u8>,
    /// Present only in reseed test cases.
    pub additional_input_reseed: Vec<u8>,
}

/// Marker type for HMAC_DRBG test vectors.
#[derive(Debug, Clone, Default)]
pub struct HmacDrbg {}

const VECTORS: &str = include_str!("../kats/HMAC_DRBG.rsp");

impl HmacDrbg {
    pub fn new() -> Result<TestVector<Self>, Error> {
        crate::read_string(VECTORS)
    }
}

impl Tv for HmacDrbg {
    type T = HmacDrbgTest;
    type H = HmacDrbgHeader;
}

impl Header for HmacDrbgHeader {
    fn algorithm(&self) -> &str {
        &self.hash
    }

    fn prediction_resistance(&self) -> bool {
        self.prediction_resistance
    }

    fn read_header<Ty: Tv<H = Self>>(line: &str, header: &mut Self) -> Result<(), Error>
    where
        Self: Sized,
    {
        // Lines like `SHA-256` (no `=`) set the hash algorithm.
        if !line.contains('=') {
            // Any section name that looks like a hash algorithm updates the hash field,
            // including unsupported ones (SHA-1, SHA-224, SHA-512/224, SHA-512/256).
            // This prevents tests in later sections from inheriting a previous section's hash.
            header.hash = line.trim().to_string();
            return Ok(());
        }

        let mut kv = line.splitn(2, '=');
        let key = kv.next().map(|v| v.trim()).ok_or(Error::Read)?;
        let val = kv.next().map(|v| v.trim()).unwrap_or("");

        match key {
            "PredictionResistance" => {
                header.prediction_resistance = val.eq_ignore_ascii_case("true");
            }
            "EntropyInputLen" => {
                header.entropy_input_len = val.parse().map_err(|_| Error::Read)?;
            }
            "NonceLen" => {
                header.nonce_len = val.parse().map_err(|_| Error::Read)?;
            }
            "PersonalizationStringLen" => {
                header.personalization_string_len = val.parse().map_err(|_| Error::Read)?;
            }
            "AdditionalInputLen" => {
                header.additional_input_len = val.parse().map_err(|_| Error::Read)?;
            }
            "ReturnedBitsLen" => {
                header.returned_bits_len = val.parse().map_err(|_| Error::Read)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Test for HmacDrbgTest {
    fn read_test<Ty: Tv<T = Self>>(
        line: &str,
        test: &mut Self,
        tv: &mut TestVector<Ty>,
    ) -> Result<(), Error>
    where
        Self: Sized,
    {
        let mut kv = line.splitn(2, '=');
        let key = kv.next().map(|v| v.trim()).ok_or(Error::Read)?;
        let val = kv.next().map(|v| v.trim()).unwrap_or("");

        match key {
            "COUNT" => {
                test.count = val.parse().map_err(|_| Error::Read)?;
            }
            "EntropyInput" => {
                test.entropy_input = hex::decode(val).map_err(|_| Error::Read)?;
            }
            "Nonce" => {
                test.nonce = hex::decode(val).map_err(|_| Error::Read)?;
            }
            "PersonalizationString" => {
                test.personalization_string = if val.is_empty() {
                    vec![]
                } else {
                    hex::decode(val).map_err(|_| Error::Read)?
                };
            }
            "AdditionalInput" => {
                let bytes = if val.is_empty() {
                    vec![]
                } else {
                    hex::decode(val).map_err(|_| Error::Read)?
                };
                test.additional_input.push(bytes);
            }
            "EntropyInputReseed" => {
                test.entropy_input_reseed = hex::decode(val).map_err(|_| Error::Read)?;
            }
            "AdditionalInputReseed" => {
                test.additional_input_reseed = if val.is_empty() {
                    vec![]
                } else {
                    hex::decode(val).map_err(|_| Error::Read)?
                };
            }
            "ReturnedBits" => {
                test.returned_bits = hex::decode(val).map_err(|_| Error::Read)?;
                test.hash = tv.header.algorithm().to_string();
                test.prediction_resistance = tv.header.prediction_resistance();
                tv.tests.push(test.clone());
                *test = Self::default();
            }
            _ => {}
        }
        Ok(())
    }
}
