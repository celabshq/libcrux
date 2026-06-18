use libcrux_secrets::U8;
use libcrux_traits::aead::arrayref::{Aead, DecryptError, EncryptError, KeyGenError};

use crate::{KEY_LEN, NONCE_LEN, TAG_LEN};

/// The ChaCha20Poly1305 AEAD algorithm.
///
/// Note: Plaintext, ciphertext and aad need to be at most [`u32::MAX`] long, due to limitations of
/// the implementation.
#[derive(Clone, Copy, PartialEq)]
pub struct ChaCha20Poly1305;

/// The extended ChaCha20Poly1305 AEAD algorithm, which uses longer nonces than ChaCha20Poly1305.
///
/// Note: Plaintext, ciphertext and aad need to be at most [`u32::MAX`] long, due to limitations of
/// the implementation.
#[derive(Clone, Copy, PartialEq)]
pub struct XChaCha20Poly1305;

mod impl_chachapoly {
    use libcrux_secrets::DeclassifyRef;
    use libcrux_secrets::DeclassifyRefMut;
    use libcrux_traits::aead::consts::AeadConsts;

    use super::*;

    impl AeadConsts for ChaCha20Poly1305 {
        const KEY_LEN: usize = KEY_LEN;

        const TAG_LEN: usize = TAG_LEN;

        const NONCE_LEN: usize = NONCE_LEN;
    }

    impl Aead<KEY_LEN, TAG_LEN, NONCE_LEN> for ChaCha20Poly1305 {
        fn encrypt(
            ciphertext: &mut [u8],
            tag: &mut [U8; TAG_LEN],
            key: &[U8; KEY_LEN],
            nonce: &[U8; NONCE_LEN],
            aad: &[u8],
            plaintext: &[U8],
        ) -> Result<(), EncryptError> {
            if plaintext.len() != ciphertext.len() {
                return Err(EncryptError::WrongCiphertextLength);
            }

            let ptxt_len: u32 = plaintext
                .len()
                .try_into()
                .map_err(|_| EncryptError::PlaintextTooLong)?;
            let aad_len: u32 = aad.len().try_into().map_err(|_| EncryptError::AadTooLong)?;

            crate::hacl::aead_chacha20poly1305::encrypt(
                ciphertext,
                tag.declassify_ref_mut(),
                plaintext.declassify_ref(),
                ptxt_len,
                aad,
                aad_len,
                key.declassify_ref(),
                nonce.declassify_ref(),
            );

            Ok(())
        }

        fn decrypt(
            plaintext: &mut [U8],
            key: &[U8; KEY_LEN],
            nonce: &[U8; NONCE_LEN],
            aad: &[u8],
            ciphertext: &[u8],
            tag: &[U8; TAG_LEN],
        ) -> Result<(), DecryptError> {
            if plaintext.len() != ciphertext.len() {
                return Err(DecryptError::WrongPlaintextLength);
            }

            let aad_len: u32 = aad.len().try_into().map_err(|_| DecryptError::AadTooLong)?;
            let ctxt_len: u32 = ciphertext
                .len()
                .try_into()
                .map_err(|_| DecryptError::PlaintextTooLong)?;

            // this call should only ever produce 0 or 1, where 0 is success and 1 is error
            match crate::hacl::aead_chacha20poly1305::decrypt(
                plaintext.declassify_ref_mut(),
                ciphertext,
                ctxt_len,
                aad,
                aad_len,
                key.declassify_ref(),
                nonce.declassify_ref(),
                tag.declassify_ref(),
            ) {
                0 => Ok(()),
                _ => Err(DecryptError::InvalidTag),
            }
        }

        fn keygen(key: &mut [U8; KEY_LEN], rand: &[U8; KEY_LEN]) -> Result<(), KeyGenError> {
            *key = *rand;
            Ok(())
        }
    }

    // Implements the traits in libcrux_traits::aead::slice based on
    // libcrux_traits::aead::arrayref:Aead.
    // This way we can use slices as keys, tags and nonces instead of array references.
    libcrux_traits::aead::slice::impl_aead_slice_trait!(ChaCha20Poly1305 => KEY_LEN, TAG_LEN, NONCE_LEN);

    // Implements the traits in libcrux_traits::aead::typed_owned based on
    // libcrux_traits::aead::arrayref:Aead.
    // This way we can use the user-facing key-centric APIs.
    libcrux_traits::aead::typed_owned::impl_aead_typed_owned!(
        ChaCha20Poly1305,
        KEY_LEN,
        TAG_LEN,
        NONCE_LEN
    );
}

mod impl_xchachapoly {
    use libcrux_secrets::{Classify, DeclassifyRef, DeclassifyRefMut};
    use libcrux_traits::aead::consts::AeadConsts;

    use crate::xchacha20_poly1305::NONCE_LEN as XNONCE_LEN;

    use super::*;

    impl AeadConsts for XChaCha20Poly1305 {
        const KEY_LEN: usize = KEY_LEN;

        const TAG_LEN: usize = TAG_LEN;

        const NONCE_LEN: usize = XNONCE_LEN;
    }

    impl Aead<KEY_LEN, TAG_LEN, XNONCE_LEN> for XChaCha20Poly1305 {
        fn encrypt(
            ciphertext: &mut [u8],
            tag: &mut [U8; TAG_LEN],
            key: &[U8; KEY_LEN],
            nonce: &[U8; XNONCE_LEN],
            aad: &[u8],
            plaintext: &[U8],
        ) -> Result<(), EncryptError> {
            let mut subkey = [0u8.classify(); KEY_LEN];
            let mut new_nonce = [0u8.classify(); super::NONCE_LEN];

            crate::xchacha20_poly1305::derive(
                key.declassify_ref(),
                nonce.declassify_ref(),
                subkey.declassify_ref_mut(),
                new_nonce.declassify_ref_mut(),
            );
            ChaCha20Poly1305::encrypt(ciphertext, tag, &subkey, &new_nonce, aad, plaintext)
        }

        fn decrypt(
            plaintext: &mut [U8],
            key: &[U8; KEY_LEN],
            nonce: &[U8; XNONCE_LEN],
            aad: &[u8],
            ciphertext: &[u8],
            tag: &[U8; TAG_LEN],
        ) -> Result<(), DecryptError> {
            let mut subkey = [0u8.classify(); KEY_LEN];
            let mut new_nonce = [0u8.classify(); super::NONCE_LEN];

            crate::xchacha20_poly1305::derive(
                key.declassify_ref(),
                nonce.declassify_ref(),
                &mut subkey.declassify_ref_mut(),
                &mut new_nonce.declassify_ref_mut(),
            );
            ChaCha20Poly1305::decrypt(plaintext, &subkey, &new_nonce, aad, ciphertext, tag)
        }

        fn keygen(key: &mut [U8; KEY_LEN], rand: &[U8; KEY_LEN]) -> Result<(), KeyGenError> {
            *key = *rand;
            Ok(())
        }
    }

    libcrux_traits::aead::slice::impl_aead_slice_trait!(XChaCha20Poly1305 => KEY_LEN, TAG_LEN, XNONCE_LEN);
    libcrux_traits::aead::typed_owned::impl_aead_typed_owned!(
        XChaCha20Poly1305,
        KEY_LEN,
        TAG_LEN,
        XNONCE_LEN
    );
}

#[cfg(test)]
mod tests {
    use libcrux_secrets::Classify;
    use libcrux_secrets::ClassifyRef;
    use libcrux_secrets::DeclassifyRef;
    use libcrux_traits::aead::typed_owned;
    use libcrux_traits::aead::typed_refs;

    type Key = typed_owned::Key<super::ChaCha20Poly1305>;
    type Nonce = typed_owned::Nonce<super::ChaCha20Poly1305>;
    type Tag = typed_owned::Tag<super::ChaCha20Poly1305>;

    #[test]
    fn test_key_centric_owned() {
        let k: Key = [0; 32].classify().into();
        let nonce: Nonce = [0; 12].classify().into();
        let mut tag: Tag = [0; 16].classify().into();

        let pt = b"the quick brown fox jumps over the lazy dog".classify_ref();
        let mut ct = [0; 43];
        let mut pt_out = [0.classify(); 43];

        k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        assert_eq!(pt.declassify_ref(), pt_out.declassify_ref());
    }

    #[test]
    fn test_key_centric_refs() {
        use typed_refs::Aead as _;

        let mut tag_bytes = [0.classify(); 16];
        let key_bytes = [0.classify(); 32];
        let nonce_bytes = [0.classify(); 12];

        let algo = super::ChaCha20Poly1305;
        let key = algo.new_key(&key_bytes).unwrap();
        let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
        let nonce = algo.new_nonce(&nonce_bytes).unwrap();

        let pt = b"the quick brown fox jumps over the lazy dog".classify_ref();
        let mut ct = [0; 43];
        let mut pt_out = [0.classify(); 43];

        key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
        let tag = algo.new_tag(&tag_bytes).unwrap();
        key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
        assert_eq!(pt.declassify_ref(), pt_out.declassify_ref());
    }
}
