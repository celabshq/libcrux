use alloc::vec::Vec;

use hpke_rs_crypto::{
    error::Error,
    types::{SingleStageKdfAlgorithm, TwoStageKdfAlgorithm},
    HpkeCrypto,
};

use crate::util::{concat, SecretBytes};

pub(crate) const HPKE_VERSION: &[u8] = b"HPKE-v1";

#[inline]
/// requires: x.len() <= u16::MAX.
pub(crate) fn length_prefixed(x: &[u8]) -> Vec<u8> {
    concat(&[&(x.len() as u16).to_be_bytes(), x])
}

#[inline]
/// `LabeledDerive` for single-stage KDFs (draft-ietf-hpke-pq):
/// `Derive(ikm ‖ "HPKE-v1" ‖ suite_id ‖ I2OSP(len(label),2) ‖ label ‖ I2OSP(L,2) ‖ context, L)`.
///
/// `Derive(input, L)` is computed by the provider's single-stage `kdf_derive`,
/// which is `SHAKE(input, 8*L)` for the SHAKE KDFs.
pub(crate) fn labeled_derive<Crypto: HpkeCrypto>(
    alg: SingleStageKdfAlgorithm,
    suite_id: &[u8],
    ikm: &[u8],
    label: &str,
    context: &[u8],
    len: usize,
) -> Result<Vec<u8>, Error> {
    if len > u16::MAX.into() {
        return Err(Error::HpkeInvalidOutputLength);
    }

    // `ikm ‖ "HPKE-v1" ‖ suite_id ‖ I2OSP(len(label),2) ‖ label ‖ I2OSP(L,2) ‖ context`.
    let labeled_ikm = SecretBytes::from(concat(&[
        ikm,
        HPKE_VERSION,
        suite_id,
        &length_prefixed(label.as_bytes()),
        &(len as u16).to_be_bytes(),
        context,
    ]));
    Crypto::kdf_derive(alg, &labeled_ikm, len)
}

#[inline]
pub(crate) fn labeled_extract<Crypto: HpkeCrypto>(
    alg: TwoStageKdfAlgorithm,
    salt: &[u8],
    suite_id: &[u8],
    label: &str,
    ikm: &[u8],
) -> Result<SecretBytes, Error> {
    let labeled_ikm = SecretBytes::from(concat(&[HPKE_VERSION, suite_id, label.as_bytes(), ikm]));
    Crypto::kdf_extract(alg, salt, &labeled_ikm).map(SecretBytes::from)
}

#[inline]
pub(crate) fn labeled_expand<Crypto: HpkeCrypto>(
    alg: TwoStageKdfAlgorithm,
    prk: &[u8],
    suite_id: &[u8],
    label: &'static str,
    info: &[u8],
    len: usize,
) -> Result<Vec<u8>, Error> {
    if len > u16::MAX.into() {
        return Err(Error::HpkeInvalidOutputLength);
    }

    let len_bytes = (len as u16).to_be_bytes();
    let labeled_info = concat(&[&len_bytes, HPKE_VERSION, suite_id, label.as_bytes(), info]);
    Crypto::kdf_expand(alg, prk, &labeled_info, len)
}
