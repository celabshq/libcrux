/// AES-GCM 256 key length.
pub const KEY_LEN: usize = 32;

/// The AES-GCM 256 state
pub(crate) type State<T, U> = super::aes_gcm::State<T, U, 15>;

use super::aes_gcm::type_aliases;

type_aliases!(AesGcm256, "AES-GCM 256");

/// # Portable implementation of AES-GCM 256
///
/// To use the portable implementation, `Key`, `Nonce`, and `Tag` types
/// must be explicitely parameterized by the portable implementation.
///
/// Example:
/// ```rust
/// // Using the portable implementation.
/// use libcrux_aes::AeadConsts as _;
/// use libcrux_aes::{NONCE_LEN, TAG_LEN, aes_gcm_256::portable::{PortableAesGcm256, Key, Tag, Nonce}};
///
/// let k: Key<PortableAesGcm256> = [0; PortableAesGcm256::KEY_LEN].into();
/// let nonce: Nonce<PortableAesGcm256> = [0; NONCE_LEN].into();
/// let mut tag: Tag<PortableAesGcm256> = [0; TAG_LEN].into();
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
    pub use crate::implementations::PortableAesGcm256;
    pub use libcrux_traits::aead::typed_owned::{Key, Nonce, Tag};
    pub use libcrux_traits::aead::typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef};
}
#[cfg(feature = "simd128")]
/// ARM NEON-optimized implementation of AES-GCM 256
///
/// To use the NEON-optimized implementation, `Key`, `Nonce`, and `Tag` types
/// must be explicitely parameterized by the NEON implementation.
///
/// Example:
/// ```rust
/// // Using the NEON implementation.
/// use libcrux_aes::AeadConsts as _;
/// use libcrux_aes::{NONCE_LEN, TAG_LEN, aes_gcm_256::neon::{NeonAesGcm256, Key, Tag, Nonce}};
///
/// let k: Key<NeonAesGcm256> = [0; NeonAesGcm256::KEY_LEN].into();
/// let nonce: Nonce<NeonAesGcm256> = [0; NONCE_LEN].into();
/// let mut tag: Tag<NeonAesGcm256> = [0; TAG_LEN].into();
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
    pub use crate::implementations::NeonAesGcm256;

    pub use libcrux_traits::aead::typed_owned::{Key, Nonce, Tag};
    pub use libcrux_traits::aead::typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef};
}
#[cfg(feature = "simd256")]
/// AES-NI-optimized implementation of AES-GCM 256
///
/// To use the AES-NI-optimized implementation, `Key`, `Nonce`, and `Tag` types
/// must be explicitely parameterized by the AES-NI implementation.
///
/// Example:
/// ```rust
/// // Using the AES-NI implementation.
/// use libcrux_aes::AeadConsts as _;
/// use libcrux_aes::{NONCE_LEN, TAG_LEN, aes_gcm_256::x64::{X64AesGcm256, Key, Tag, Nonce}};
///
/// let k: Key<X64AesGcm256> = [0; X64AesGcm256::KEY_LEN].into();
/// let nonce: Nonce<X64AesGcm256> = [0; NONCE_LEN].into();
/// let mut tag: Tag<X64AesGcm256> = [0; TAG_LEN].into();
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
    pub use crate::implementations::X64AesGcm256;
    pub use libcrux_traits::aead::typed_owned::{Key, Nonce, Tag};
    pub use libcrux_traits::aead::typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef};
}
