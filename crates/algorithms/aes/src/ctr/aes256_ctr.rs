//! AES256 ctr mode, generic over the platform [`AESState`].

use core::array::from_fn;

use super::{AesCtrContext, AES_GCM_CTR_LEN, AES_GCM_NONCE_START};
use crate::{
    aes::*,
    aes_gcm_256::KEY_LEN,
    ctr::{AES_CCM_CTR_LEN, AES_CCM_NONCE_START},
    platform::AESState,
    NONCE_LEN,
};

pub(crate) const NUM_KEYS: usize = 15;

impl<T: AESState, const CTR_LEN: usize, const NONCE_START: usize>
    AesCtrContext<T, NUM_KEYS, CTR_LEN, NONCE_START>
{
    #[inline]
    pub(crate) fn init(key: &[u8], nonce: &[u8]) -> Self {
        debug_assert_eq!(nonce.len(), NONCE_LEN);
        debug_assert_eq!(key.len(), 32);
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

/// 256 - Key expansion
#[inline]
fn key_expansion<T: AESState>(key: &[u8]) -> ExtendedKey<T, NUM_KEYS> {
    debug_assert!(key.len() == KEY_LEN);

    let mut keyex = from_fn(|_| T::new());
    keyex[0].load_block(&key[0..16]);
    keyex[1].load_block(&key[16..32]);

    macro_rules! expansion_step256 {
        ($i:expr,$rcon:expr) => {
            // // Split at $i to get the one we currently look at and the previous
            // // blocks.
            // let (prev, current) = keyex.split_at_mut($i);

            // // Split again to get the $i and $i + 1 states to operate on.
            // let (c0, c1) = current.split_at_mut(1);
            // let key_i = &mut c0[0];
            // let key_i_plus_1 = &mut c1[0];

            // key_i.aes_keygen_assist0::<$rcon>(&prev[$i - 1]);
            // key_i.key_expansion_step(&prev[$i - 2]);

            // key_i_plus_1.aes_keygen_assist1(&key_i);
            // key_i_plus_1.key_expansion_step(&prev[$i - 1]);

            // The following is what goes through hax right now. But it
            // requires copies that are really not necessary.
            let prev0 = keyex[$i - 2].clone();
            let prev1 = keyex[$i - 1].clone();

            keyex[$i].aes_keygen_assist0::<$rcon>(&prev1);
            keyex[$i].key_expansion_step(&prev0);

            let next0 = keyex[$i].clone();
            keyex[$i + 1].aes_keygen_assist1(&next0);
            keyex[$i + 1].key_expansion_step(&prev1);
        };
    }

    expansion_step256!(2, 0x01);
    expansion_step256!(4, 0x02);
    expansion_step256!(6, 0x04);
    expansion_step256!(8, 0x08);
    expansion_step256!(10, 0x10);
    expansion_step256!(12, 0x20);

    // let (prev0, tmp) = keyex.split_at_mut(13);
    // let (prev1, last) = tmp.split_at_mut(1);
    // let prev0 = &mut prev0[12];
    // let prev1 = &mut prev1[0];
    // let last = &mut last[0];
    // To get through hax right now have to clone instead.
    let prev0 = keyex[12].clone();
    let prev1 = keyex[13].clone();
    // let last = &mut keyex[NUM_KEYS - 1];
    keyex[NUM_KEYS - 1].aes_keygen_assist0::<0x40>(&prev1);
    keyex[NUM_KEYS - 1].key_expansion_step(&prev0);

    keyex
}
