//! NIST feedback mode KDF.

use core::cmp;

use libcrux_hmac::HmacState;

use crate::KdfError;

/// NIST [SP 800-108r1-upd1][sp] KDF in feedback mode using HMAC.
///
/// The parameter `L`, the requested number of bits, is implicitly passed as the
/// length of `k_out`. This KDF can only output whole bytes. The maximum number of **bytes**
/// that can be requested is `u32::MAX * OUTLEN`.
///
/// On empty `k_out`, this returns `Ok(())`.
///
/// # Fixed Info
///
/// The fixed info slices are concatenated as-is. No length prefixes or separators are
/// inserted. The caller is responsible for an unambiguous encoding of the data within
/// the fixed info.
///
/// # Encoding
///
/// - The counter `i` is encoded as a 32-bit big-endian integer.
///
/// # Zeroization
///
/// Zeroization, i.e. destruction of intermediate data, is currently not performed.
///
/// # Errors
///
/// - If `k_out.len().div_ceil(OUTLEN) > u32::MAX`.
/// - If `k_in`, `iv` or any `fixed_info` exceeds the maximum input size of the HMAC.
///
/// # Example
///
/// ```
/// use libcrux_hmac::HmacSha256;
/// use libcrux_nist_kdf::feedback;
///
/// let k_in = [0x42; 32];
/// let iv = [0x13; 32];
/// let mut k_out = [0; 42];
///
/// // Encode the fixed info as `label || 0x00 || context || L`, with `L`, the
/// // requested number of bits, encoded as a 32-bit big-endian integer.
/// let label = b"example label";
/// let context = b"example context";
/// let l = u32::try_from(k_out.len() * 8).unwrap().to_be_bytes();
///
/// feedback::kdf::<32, HmacSha256>(&mut k_out, &k_in, &iv, &[label, &[0x00], context, &l])
///     .unwrap();
/// ```
///
/// [sp]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-108r1-upd1.pdf
pub fn kdf<const OUTLEN: usize, H: HmacState<OUTLEN>>(
    mut k_out: &mut [u8],
    k_in: &[u8],
    iv: &[u8],
    fixed_info: &[&[u8]],
) -> Result<(), KdfError> {
    // SP 800-108 computes n := ceil(L/h) where L and h are bits.
    // Because k_out.len() * 8 = L and OUTLEN = h / 8, i.e. both are in bytes, we can compute
    // n as follow:
    let n = k_out.len().div_ceil(OUTLEN);
    // Equivalent to checking whether n > u32::MAX.
    let n = u32::try_from(n).map_err(|_| KdfError)?;
    let mut k_i_minus_one = iv;
    // Extend the lifetime of k_i_block so that we can assign
    // the slice to k_i_minus_one
    let mut k_i_block;

    for i in 1_u32..=n {
        let i_encoded = &i.to_be_bytes();
        k_i_block =
            prf::<OUTLEN, H>(k_in, k_i_minus_one, i_encoded, fixed_info).map_err(|_| KdfError)?;
        let end = cmp::min(k_out.len(), OUTLEN);
        // Panic-safety:
        // end = min(k_out.len(), OUTLEN) <= k_out.len()
        // end = min(k_out.len(), OUTLEN) <= OUTLEN == k_i_block.len()
        // Therefore the indexes are always safe.
        k_out[..end].copy_from_slice(&k_i_block[..end]);
        k_out = &mut k_out[end..];
        k_i_minus_one = &k_i_block[..];
    }
    Ok(())
}

fn prf<const OUTLEN: usize, H: HmacState<OUTLEN>>(
    k_in: &[u8],
    k_i_minus_one: &[u8],
    i_encoded: &[u8],
    fixed_info: &[&[u8]],
) -> Result<[u8; OUTLEN], libcrux_hmac::Error> {
    let mut hmac = H::new(k_in)?;
    hmac.update(k_i_minus_one)?;
    hmac.update(i_encoded)?;
    for info in fixed_info {
        hmac.update(info)?;
    }
    let mut mac = [0; _];
    hmac.finalize(&mut mac);
    Ok(mac)
}
