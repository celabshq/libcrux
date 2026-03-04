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

/// Type alias for the AES 128 ctr context.
pub(crate) type AesGcm128CtrContext<T> =
    AesCtrContext<T, NUM_KEYS, AES_GCM_CTR_LEN, AES_GCM_NONCE_START>;
pub(crate) type AesCcm128CtrContext<T> =
    AesCtrContext<T, NUM_KEYS, AES_CCM_CTR_LEN, AES_CCM_NONCE_START>;

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

    #[inline]
    pub(crate) fn set_nonce(&mut self, nonce: &[u8]) {
        debug_assert!(nonce.len() == NONCE_LEN);

        self.aes_ctr_set_nonce(nonce);
    }

    #[inline]
    pub(crate) fn key_block(&self, ctr: u32, out: &mut [u8]) {
        debug_assert!(out.len() == GCM_KEY_LEN);

        self.aes_ctr_key_block(ctr, out);
    }

    #[inline]
    pub(crate) fn update(&self, ctr: u32, inp: &[u8], out: &mut [u8]) {
        debug_assert!(inp.len() == out.len());

        self.aes_ctr_update(ctr, inp, out);
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
