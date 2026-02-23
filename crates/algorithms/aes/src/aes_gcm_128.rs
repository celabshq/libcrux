use crate::{
    aes::AES_BLOCK_LEN,
    aes_gcm::aesgcm,
    ctr::AesGcm128CtrContext,
    gf128::GF128State,
    platform::{AESState, GF128FieldElement},
    DecryptError, NONCE_LEN, TAG_LEN,
};

/// AES-GCM 128 key length.
pub const KEY_LEN: usize = 16;
pub(crate) const GCM_KEY_LEN: usize = 16;

/// The AES-GCM 128 state
pub(crate) struct State<T: AESState, U: GF128FieldElement> {
    pub(crate) aes_state: AesGcm128CtrContext<T>,
    pub(crate) gcm_state: GF128State<U>,
    pub(crate) tag_mix: [u8; TAG_LEN],
}

aesgcm!(State<T, U>, AesGcm128CtrContext);

use super::aes_gcm::type_aliases;

type_aliases!(AesGcm128, "AES-GCM 128");

/// # Portable implementation of AES-GCM 128
///
/// To use the portable implementation, `Key`, `Nonce`, and `Tag` types
/// must be explicitely parameterized by the portable implementation.
///
/// Example:
/// ```rust
/// // Using the portable implementation.
/// use libcrux_aes::AeadConsts as _;
/// use libcrux_aes::{NONCE_LEN, TAG_LEN, aes_gcm_128::portable::{PortableAesGcm128, Key, Tag, Nonce}};
///
/// let k: Key<PortableAesGcm128> = [0; PortableAesGcm128::KEY_LEN].into();
/// let nonce: Nonce<PortableAesGcm128> = [0; NONCE_LEN].into();
/// let mut tag: Tag<PortableAesGcm128> = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod portable {
    pub use crate::implementations::PortableAesGcm128;
    pub use libcrux_traits::aead::typed_owned::{Key, Nonce, Tag};
    pub use libcrux_traits::aead::typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef};
}
#[cfg(feature = "simd128")]
/// ARM NEON-optimized implementation of AES-GCM 128
///
/// To use the NEON-optimized implementation, `Key`, `Nonce`, and `Tag` types
/// must be explicitely parameterized by the NEON implementation.
///
/// Example:
/// ```rust
/// // Using the NEON implementation.
/// use libcrux_aes::AeadConsts as _;
/// use libcrux_aes::{NONCE_LEN, TAG_LEN, aes_gcm_128::neon::{NeonAesGcm128, Key, Tag, Nonce}};
///
/// let k: Key<NeonAesGcm128> = [0; NeonAesGcm128::KEY_LEN].into();
/// let nonce: Nonce<NeonAesGcm128> = [0; NONCE_LEN].into();
/// let mut tag: Tag<NeonAesGcm128> = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod neon {
    pub use crate::implementations::NeonAesGcm128;

    pub use libcrux_traits::aead::typed_owned::{Key, Nonce, Tag};
    pub use libcrux_traits::aead::typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef};
}
#[cfg(feature = "simd256")]
/// AES-NI-optimized implementation of AES-GCM 128
///
/// To use the AES-NI-optimized implementation, `Key`, `Nonce`, and `Tag` types
/// must be explicitely parameterized by the AES-NI implementation.
///
/// Example:
/// ```rust
/// // Using the AES-NI implementation.
/// use libcrux_aes::AeadConsts as _;
/// use libcrux_aes::{NONCE_LEN, TAG_LEN, aes_gcm_128::x64::{X64AesGcm128, Key, Tag, Nonce}};
///
/// let k: Key<X64AesGcm128> = [0; X64AesGcm128::KEY_LEN].into();
/// let nonce: Nonce<X64AesGcm128> = [0; NONCE_LEN].into();
/// let mut tag: Tag<X64AesGcm128> = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod x64 {
    pub use crate::implementations::X64AesGcm128;
    pub use libcrux_traits::aead::typed_owned::{Key, Nonce, Tag};
    pub use libcrux_traits::aead::typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef};
}
