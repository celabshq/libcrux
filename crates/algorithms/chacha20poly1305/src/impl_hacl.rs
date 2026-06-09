use crate::{AeadError, KEY_LEN, NONCE_LEN, TAG_LEN};

const DETACHED: bool = true;
const NOT_DETACHED: bool = false;

/// The ChaCha20-Poly1305 AEAD encryption function with detached tag.
pub fn encrypt_detached<'a>(
    key: &[u8; KEY_LEN],
    ptxt: &[u8],
    ctxt: &'a mut [u8],
    tag: &'a mut [u8; TAG_LEN],
    aad: &[u8],
    nonce: &[u8; NONCE_LEN],
) -> Result<(&'a [u8], &'a [u8; TAG_LEN]), AeadError> {
    let (ptxt_len, aad_len) = encrypt_checks(ptxt, ctxt, aad, DETACHED)?;

    crate::hacl::aead_chacha20poly1305::encrypt(
        ctxt, tag, ptxt, ptxt_len, aad, aad_len, key, nonce,
    );

    Ok((ctxt, tag))
}

#[inline(always)]
fn encrypt_checks(
    ptxt: &[u8],
    ctxt: &mut [u8],
    aad: &[u8],
    detached: bool,
) -> Result<(u32, u32), AeadError> {
    let ptxt_len: u32 = ptxt
        .len()
        .try_into()
        .map_err(|_| AeadError::PlaintextTooLarge)?;
    let aad_len: u32 = aad.len().try_into().map_err(|_| AeadError::AadTooLarge)?;

    if !detached && (ctxt.len() as u64) < (ptxt.len() as u64) + (TAG_LEN as u64) {
        return Err(AeadError::CiphertextTooShort);
    }

    if detached && (ctxt.len() as u64) < (ptxt.len() as u64) {
        return Err(AeadError::CiphertextTooShort);
    }

    Ok((ptxt_len, aad_len))
}

/// The ChaCha20-Poly1305 AEAD encryption function. Writes the concatenation of the ciphertext
/// produced by ChaCha20 and the MAC tag into `ctxt` and returns the two pieces separately.
///
/// This implementation is backed by hacl-rs and can only handle inputs up to a length of `u32::MAX`.
/// When provided longer values, this function will return an error.
pub fn encrypt<'a>(
    key: &[u8; KEY_LEN],
    ptxt: &[u8],
    ctxt: &'a mut [u8],
    aad: &[u8],
    nonce: &[u8; NONCE_LEN],
) -> Result<(&'a [u8], &'a [u8; TAG_LEN]), AeadError> {
    let (ptxt_len, aad_len) = encrypt_checks(ptxt, ctxt, aad, NOT_DETACHED)?;

    let (ctxt_cpa, rest) = ctxt.split_at_mut(ptxt_len as usize);
    // The ciphertext buffer may be longer than ptxt_len + TAG_LEN.
    let (tag, _rest) = rest.split_at_mut(TAG_LEN);
    let tag: &mut [u8; TAG_LEN] = tag.try_into().unwrap();

    crate::hacl::aead_chacha20poly1305::encrypt(
        ctxt_cpa, tag, ptxt, ptxt_len, aad, aad_len, key, nonce,
    );

    Ok((ctxt_cpa, tag))
}

/// The ChaCha20-Poly1305 AEAD decryption function. Writes the result of the decryption to `ptxt`,
/// and returns the slice of appropriate length.
///
/// This implementation is backed by hacl-rs and can only handle inputs up to a length of `u32::MAX`.
/// When provided longer values, this function will return an error.
pub fn decrypt<'a>(
    key: &[u8; KEY_LEN],
    ptxt: &'a mut [u8],
    ctxt: &[u8],
    aad: &[u8],
    nonce: &[u8; NONCE_LEN],
) -> Result<&'a [u8], AeadError> {
    if ctxt.len() < TAG_LEN {
        return Err(AeadError::InvalidCiphertext);
    }

    // we know that ctxt.len() >= TAG_LEN, so we can subtract
    if ptxt.len() < ctxt.len() - TAG_LEN {
        return Err(AeadError::PlaintextTooShort);
    }

    let aad_len: u32 = aad.len().try_into().map_err(|_| AeadError::AadTooLarge)?;

    let (ctxt_cpa, tag) = ctxt.split_at(ctxt.len() - TAG_LEN);
    let ptxt = &mut ptxt[..ctxt_cpa.len()];

    let ctxt_cpa_len: u32 = ctxt_cpa
        .len()
        .try_into()
        .map_err(|_| AeadError::CiphertextTooLarge)?;

    // this call should only ever produce 0 or 1, where 0 is success and 1 is error
    match crate::hacl::aead_chacha20poly1305::decrypt(
        ptxt,
        ctxt_cpa,
        ctxt_cpa_len,
        aad,
        aad_len,
        key,
        nonce,
        tag,
    ) {
        0 => Ok(ptxt),
        _ => Err(AeadError::InvalidCiphertext),
    }
}

/// The ChaCha20-Poly1305 AEAD decryption function. Writes the result of the decryption to `ptxt`,
/// and returns the slice of appropriate length.
///
/// This implementation is backed by hacl-rs and can only handle inputs up to a length of `u32::MAX`.
/// When provided longer values, this function will return an error.
pub fn decrypt_detached<'a>(
    key: &[u8; KEY_LEN],
    ptxt: &'a mut [u8],
    ctxt: &[u8],
    tag: &[u8; TAG_LEN],
    aad: &[u8],
    nonce: &[u8; NONCE_LEN],
) -> Result<&'a [u8], AeadError> {
    if ptxt.len() < ctxt.len() {
        return Err(AeadError::PlaintextTooShort);
    }

    let ctxt_len: u32 = ctxt
        .len()
        .try_into()
        .map_err(|_| AeadError::CiphertextTooLarge)?;

    let aad_len: u32 = aad.len().try_into().map_err(|_| AeadError::AadTooLarge)?;

    let ptxt = &mut ptxt[..ctxt.len()];

    // this call should only ever produce 0 or 1, where 0 is success and 1 is error
    match crate::hacl::aead_chacha20poly1305::decrypt(
        ptxt, ctxt, ctxt_len, aad, aad_len, key, nonce, tag,
    ) {
        0 => Ok(ptxt),
        _ => Err(AeadError::InvalidCiphertext),
    }
}
