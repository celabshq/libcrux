//! NIST two-step KDF.
use libcrux_hmac::HmacState;

use crate::{feedback, KdfError};

/// [SP 800-56Cr2][56] two-step KDF.
///
/// An implementation of the two-step KDF using HMAC for the randomness extraction
/// and a [feedback mode kdf][`feedback::kdf`] using the same HMAC algorithm for
/// the key expansion.
///
/// # Salt
///
/// SP 800-56Cr2 requires a non-null salt and, in the absence of an agreed-upon
/// alternative, a default salt consisting of an all-zero byte string of the length
/// of a single input block of the hash function. An empty `salt` is accepted and is
/// equivalent to this default salt, because HMAC pads keys shorter than the block
/// length with zeros.
///
/// # Fixed Info
///
/// The fixed info slices are concatenated as-is. No length prefixes or separators are
/// inserted. The caller is responsible for an unambiguous encoding of the data within
/// the fixed info.
///
/// # Encoding
///
/// - The counter `i` during the key expansion is encoded as a 32-bit big-endian integer.
///
/// # Zeroization
///
/// Zeroization, i.e. destruction of intermediate data, is currently not performed.
///
/// # Errors
///
/// - If `k_out.is_empty()`.
/// - If `shared_secret` or `salt` exceed the maximum input size of the HMAC.
/// - If [`feedback::kdf`] returns an error.
///
/// # Example
///
/// ```
/// use libcrux_hmac::HmacSha256;
/// use libcrux_nist_kdf::two_step;
///
/// let shared_secret = [0x42; 32];
/// let salt = [0x07; 32];
/// let iv = [0x13; 32];
/// let mut k_out = [0; 42];
///
/// // Encode the fixed info as `label || 0x00 || context || L`, with `L`, the
/// // requested number of bits, encoded as a 32-bit big-endian integer.
/// let label = b"example label";
/// let context = b"example context";
/// let l = u32::try_from(k_out.len() * 8).unwrap().to_be_bytes();
///
/// two_step::kdf::<32, HmacSha256>(
///     &mut k_out,
///     &shared_secret,
///     &salt,
///     &iv,
///     &[label, &[0x00], context, &l],
/// )
/// .unwrap();
/// ```
///
/// [56]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Cr2.pdf
pub fn kdf<const OUTLEN: usize, H: HmacState<OUTLEN>>(
    k_out: &mut [u8],
    shared_secret: &[u8],
    salt: &[u8],
    iv: &[u8],
    fixed_info: &[&[u8]],
) -> Result<(), KdfError> {
    // `L` must be a **positive** integer.
    if k_out.is_empty() {
        return Err(KdfError);
    }
    let mut kdk = [0; OUTLEN];
    libcrux_hmac::hmac_slices::<OUTLEN, H>(&mut kdk, salt, &[shared_secret])
        .map_err(|_| KdfError)?;

    feedback::kdf::<OUTLEN, H>(k_out, &kdk, iv, fixed_info)
}
