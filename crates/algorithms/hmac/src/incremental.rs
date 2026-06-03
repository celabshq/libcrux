use crate::{Error, HmacState};
use libcrux_traits::digest::{arrayref, InitializeDigestState};

/// Generic streaming HMAC state, parametrized by output length, block size,
/// and an incremental digest implementation `D`.
///
/// Use the type aliases [`HmacSha256`], [`HmacSha384`], or [`HmacSha512`]
/// rather than spelling out the type parameters directly.
pub struct HmacStream<const OUTLEN: usize, const BLOCK: usize, D>
where
    D: arrayref::DigestIncremental<OUTLEN>,
    D::IncrementalState: InitializeDigestState,
{
    /// Inner hash state already primed with K xor ipad.
    pub(crate) inner: D::IncrementalState,
    pub(crate) opad: [u8; BLOCK],
}

impl<const OUTLEN: usize, const BLOCK: usize, Digest> HmacState<OUTLEN>
    for HmacStream<OUTLEN, BLOCK, Digest>
where
    Digest: arrayref::DigestIncremental<OUTLEN>,
    Digest::IncrementalState: InitializeDigestState,
{
    /// Initialize a new streaming HMAC computation with the given key.
    fn new(key: &[u8]) -> Result<Self, Error> {
        debug_assert!(OUTLEN <= BLOCK);
        debug_assert!(BLOCK <= u32::MAX as usize);

        if key.len() > u32::MAX as usize {
            return Err(Error::InvalidInputLength);
        }

        let mut key_block = [0u8; BLOCK];
        if key.len() <= BLOCK {
            key_block[..key.len()].copy_from_slice(key);
        } else {
            // Key longer than one block: hash it down to OUTLEN bytes.
            let mut tmp = Digest::IncrementalState::new();
            Digest::update(&mut tmp, key)
                .expect("This can't fail because we checked that key.len() <= u32::MAX");
            let mut hashed = [0u8; OUTLEN];
            Digest::finish(&mut tmp, &mut hashed);
            key_block[..OUTLEN].copy_from_slice(&hashed);
        }

        let mut k_xor_ipad = [0u8; BLOCK];
        let mut k_xor_opad = [0u8; BLOCK];

        for i in 0..BLOCK {
            k_xor_ipad[i] = key_block[i] ^ 0x36;
            k_xor_opad[i] = key_block[i] ^ 0x5c;
        }

        // Prime the inner state with `k_xor_ipad` so subsequent updates feed data directly.
        // The update should only fail if we submit too much data, and `k_xor_ipad` is just a single
        // block long, so short enough.
        let mut inner = Digest::IncrementalState::new();
        Digest::update(&mut inner, &k_xor_ipad).expect("HMAC: ipad update failed");

        Ok(Self {
            inner,
            opad: k_xor_opad,
        })
    }

    fn update(&mut self, data: &[u8]) -> Result<(), Error> {
        Digest::update(&mut self.inner, data).map_err(|_| Error::InvalidInputLength)
    }

    fn finalize(mut self, dst: &mut [u8; OUTLEN]) {
        // Inner hash: finalize H((K xor ipad) || data).
        let mut inner_hash = [0u8; OUTLEN];
        Digest::finish(&mut self.inner, &mut inner_hash);

        // Outer hash: H((K xor opad) || inner_hash).
        let mut outer = Digest::IncrementalState::new();
        Digest::update(&mut outer, &self.opad).expect("self.opad.len() is always <= u32::MAX");
        Digest::update(&mut outer, &inner_hash).expect("inner_hash.len() is always <= u32::MAX");
        Digest::finish(&mut outer, dst);
    }
}

/// Streaming HMAC-SHA-256 state.  Initialize with [`HmacSha256::new`].
pub type HmacSha256 = HmacStream<32, 64, libcrux_sha2::Sha256Hash>;

/// Streaming HMAC-SHA-384 state.  Initialize with [`HmacSha384::new`].
pub type HmacSha384 = HmacStream<48, 128, libcrux_sha2::Sha384Hash>;

/// Streaming HMAC-SHA-512 state.  Initialize with [`HmacSha512::new`].
pub type HmacSha512 = HmacStream<64, 128, libcrux_sha2::Sha512Hash>;

pub fn hmac_slices<const OUTLEN: usize, Hmac: HmacState<OUTLEN>>(
    dst: &mut [u8; OUTLEN],
    key: &[u8],
    slices: &[&[u8]],
) -> Result<(), Error> {
    let mut state = Hmac::new(key)?;

    for &s in slices {
        state.update(s)?;
    }

    state.finalize(dst);
    Ok(())
}

/// Compute HMAC-SHA-256 over logically concatenated, non-contiguous data slices.
#[inline]
pub fn hmac_sha2_256_slices(dst: &mut [u8; 32], key: &[u8], slices: &[&[u8]]) -> Result<(), Error> {
    hmac_slices::<32, HmacSha256>(dst, key, slices)
}

/// Compute HMAC-SHA-384 over logically concatenated, non-contiguous data slices.
#[inline]
pub fn hmac_sha2_384_slices(dst: &mut [u8; 48], key: &[u8], slices: &[&[u8]]) -> Result<(), Error> {
    hmac_slices::<48, HmacSha384>(dst, key, slices)
}

/// Compute HMAC-SHA-512 over logically concatenated, non-contiguous data slices.
#[inline]
pub fn hmac_sha2_512_slices(dst: &mut [u8; 64], key: &[u8], slices: &[&[u8]]) -> Result<(), Error> {
    hmac_slices::<64, HmacSha512>(dst, key, slices)
}
