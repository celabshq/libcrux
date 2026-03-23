//! AES128 ctr mode, generic over the platform [`AESState`].

use core::array::from_fn;

use super::{AesCtrContext, AES_CCM_CTR_LEN, AES_GCM_CTR_LEN};
use crate::{
    aes::*,
    aes_gcm_128::GCM_KEY_LEN,
    ctr::{AES_CCM_NONCE_START, AES_GCM_NONCE_START},
    platform::AESState,
    NONCE_LEN,
};

pub(super) const NUM_KEYS: usize = 11;

impl<T: AESState, const CTR_LEN: usize, const NONCE_START: usize>
    AesCtrContext<T, NUM_KEYS, CTR_LEN, NONCE_START>
{
    #[inline]
    pub(crate) fn init(key: &[u8], nonce: &[u8]) -> Self {
        debug_assert_eq!(nonce.len(), NONCE_LEN);
        debug_assert_eq!(key.len(), GCM_KEY_LEN);
        debug_assert!(CTR_LEN <= 8 && CTR_LEN > 1);

        let mut ctr_nonce = [0u8; 16];
        if NONCE_START == 1 {
            // write flags into the first byte
            ctr_nonce[0] = (CTR_LEN - 1) as u8;
        }
        ctr_nonce[NONCE_START..NONCE_START + NONCE_LEN].copy_from_slice(nonce);

        Self {
            extended_key: key_expansion(key),
            ctr_nonce,
        }
    }
}

impl<T: AESState> super::CcmInit
    for AesCtrContext<T, NUM_KEYS, AES_CCM_CTR_LEN, AES_CCM_NONCE_START>
{
    fn ccm_init(key: &[u8]) -> Self {
        Self::init(key, &[0u8; NONCE_LEN])
    }
}

impl<T: AESState> super::GcmInit
    for AesCtrContext<T, NUM_KEYS, AES_GCM_CTR_LEN, AES_GCM_NONCE_START>
{
    fn gcm_init(key: &[u8]) -> Self {
        Self::init(key, &[0u8; NONCE_LEN])
    }
}

/// 128 - Key expansion
#[inline]
fn key_expansion<T: AESState>(key: &[u8]) -> ExtendedKey<T, NUM_KEYS> {
    debug_assert!(key.len() == GCM_KEY_LEN);

    let mut keyex = from_fn(|_| T::new());
    keyex[0].load_block(key);

    macro_rules! expansion_step128 {
        ($i:expr,$rcon:expr) => {
            // For hax need to clone here.
            let prev = keyex[$i - 1].clone();
            // let (prev, current) = keyex.split_at_mut($i);
            keyex[$i].aes_keygen_assist0::<$rcon>(&prev);
            keyex[$i].key_expansion_step(&prev);
        };
    }

    expansion_step128!(1, 0x01);
    expansion_step128!(2, 0x02);
    expansion_step128!(3, 0x04);
    expansion_step128!(4, 0x08);
    expansion_step128!(5, 0x10);
    expansion_step128!(6, 0x20);
    expansion_step128!(7, 0x40);
    expansion_step128!(8, 0x80);
    expansion_step128!(9, 0x1b);
    expansion_step128!(10, 0x36);

    keyex
}
