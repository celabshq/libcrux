//! Minimal DER (ASN.1) codec for the ECDSA-Sig-Value structure.
//!
//! ```asn1
//! ECDSA-Sig-Value ::= SEQUENCE { r INTEGER, s INTEGER }
//! ```
//!
//! Signatures are commonly transported in this form, while `libcrux-ecdsa`
//! works with the raw `(r, s)` scalars. These helpers convert between the two
//! representations. Only the small encodings produced by P-256 (each integer is
//! at most 33 content bytes, the sequence at most ~70 bytes) are relevant, so
//! short-form lengths are all that is supported. This module deals purely with
//! encoding, on stack buffers only, since this crate is `no_std` without
//! `alloc`.

const INTEGER_TAG: u8 = 0x02;
const SEQUENCE_TAG: u8 = 0x30;

const HIGH_BIT: u8 = 0x80;

/// Maximum encoded size of a P-256 `ECDSA-Sig-Value`: 2-byte SEQUENCE header
/// plus two INTEGERs of at most 2 (tag + length) + 1 (sign byte) + 32
/// (content) bytes each.
pub const MAX_DER_LEN: usize = 2 + 2 * (2 + 1 + 32);

/// A DER-encoded `ECDSA-Sig-Value`, held in a fixed-capacity stack buffer.
///
/// Derefs to the encoded bytes, so it can be used anywhere a `&[u8]` is
/// expected. The encoding is never longer than [`MAX_DER_LEN`].
#[derive(Clone, Debug)]
pub struct DerSignature {
    buf: [u8; MAX_DER_LEN],
    len: usize,
}

impl DerSignature {
    /// The encoded bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl core::ops::Deref for DerSignature {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for DerSignature {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Append `data` to `buf` at `*pos`, advancing `*pos`.
fn push(buf: &mut [u8; MAX_DER_LEN], pos: &mut usize, data: &[u8]) {
    debug_assert!(*pos + data.len() <= MAX_DER_LEN);
    buf[*pos..*pos + data.len()].copy_from_slice(data);
    *pos += data.len();
}

/// DER-encode a big-endian unsigned integer as an ASN.1 `INTEGER` into `buf`
/// at `*pos`, advancing `*pos` past the written bytes.
fn encode_integer(buf: &mut [u8; MAX_DER_LEN], pos: &mut usize, value: &[u8; 32]) {
    // Strip leading zero bytes, but keep at least one byte.
    let mut start = 0;
    while start + 1 < value.len() && value[start] == 0 {
        start += 1;
    }
    let content = &value[start..];

    // ASN.1 integers are signed. If the high bit is set, prepend a zero byte
    // so the value is interpreted as positive.
    let needs_sign_byte = content.first().is_some_and(|b| b & HIGH_BIT != 0);
    let content_len = content.len() + if needs_sign_byte { 1 } else { 0 };

    push(buf, pos, &[INTEGER_TAG, content_len as u8]); // content length always < 128 for P-256
    if needs_sign_byte {
        push(buf, pos, &[0x00]);
    }
    push(buf, pos, content);
}

/// Encode a raw `(r, s)` P-256 signature (each 32 bytes) as a DER
/// `ECDSA-Sig-Value`.
pub(crate) fn raw_to_der(r: &[u8; 32], s: &[u8; 32]) -> DerSignature {
    let mut buf = [0u8; MAX_DER_LEN];
    // Encode the integers into a scratch area first so we know the body
    // length before writing the SEQUENCE header.
    let mut body = [0u8; MAX_DER_LEN];
    let mut body_len = 0;
    encode_integer(&mut body, &mut body_len, r);
    encode_integer(&mut body, &mut body_len, s);

    let mut pos = 0;
    push(&mut buf, &mut pos, &[SEQUENCE_TAG, body_len as u8]); // body length always < 128 for P-256
    push(&mut buf, &mut pos, &body[..body_len]);

    DerSignature { buf, len: pos }
}

/// Read a DER short-form length starting at `*pos`, advancing `*pos` past the
/// length octet.
///
/// Long-form lengths are rejected outright. DER requires the minimal encoding,
/// so the long form only ever encodes a length of at least 128 — which no
/// P-256 `ECDSA-Sig-Value` can reach: an INTEGER that long has more than 32
/// content bytes, and a SEQUENCE that long cannot be filled by two such
/// INTEGERs.
fn read_len(bytes: &[u8], pos: &mut usize) -> Option<usize> {
    let first = *bytes.get(*pos)?;
    *pos += 1;
    if first < 0x80 {
        Some(first as usize)
    } else {
        None
    }
}

/// Read a DER `INTEGER` starting at `*pos` and return it left-padded to 32
/// bytes, advancing `*pos` past the integer.
///
/// Rejects non-canonical encodings: a redundant leading zero byte (one not
/// needed to keep the value non-negative), and a negative value (ECDSA `r`
/// and `s` are always non-negative).
fn read_integer_32(bytes: &[u8], pos: &mut usize) -> Option<[u8; 32]> {
    if *bytes.get(*pos)? != INTEGER_TAG {
        return None;
    }
    *pos += 1;

    let len = read_len(bytes, pos)?;
    if len == 0 {
        return None;
    }

    let content = bytes.get(*pos..*pos + len)?;
    *pos += len;

    let value = if content[0] == 0x00 {
        if content.len() > 1 && content[1] & HIGH_BIT == 0 {
            // Redundant zero-padding byte: not the minimal DER encoding.
            return None;
        }
        &content[1..]
    } else {
        if content[0] & HIGH_BIT != 0 {
            // High bit set with no zero-padding byte encodes a negative value.
            return None;
        }
        content
    };

    if value.len() > 32 {
        return None;
    }

    let mut out = [0u8; 32];
    out[32 - value.len()..].copy_from_slice(value);
    Some(out)
}

/// Decode a DER `ECDSA-Sig-Value` into raw `(r, s)` scalars, each 32 bytes.
///
/// Returns `None` if the input is not a well-formed P-256 signature.
pub(crate) fn der_to_raw(der: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    let mut pos = 0;
    if *der.get(pos)? != SEQUENCE_TAG {
        return None;
    }
    pos += 1;
    let seq_len = read_len(der, &mut pos)?;
    if pos + seq_len != der.len() {
        return None;
    }
    let r = read_integer_32(der, &mut pos)?;
    let s = read_integer_32(der, &mut pos)?;
    if pos != der.len() {
        return None;
    }
    Some((r, s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_high_bit() {
        let r = [0x80u8; 32]; // high bit set → needs sign padding
        let s = [0x01u8; 32];
        let der = raw_to_der(&r, &s);
        let (r2, s2) = der_to_raw(der.as_bytes()).unwrap();
        assert_eq!(r, r2);
        assert_eq!(s, s2);
    }

    #[test]
    fn roundtrip_leading_zeros() {
        let mut r = [0u8; 32];
        r[31] = 0x2a; // small value with many leading zeros
        let s = [0xffu8; 32];
        let der = raw_to_der(&r, &s);
        let (r2, s2) = der_to_raw(der.as_bytes()).unwrap();
        assert_eq!(r, r2);
        assert_eq!(s, s2);
    }

    #[test]
    fn rejects_garbage() {
        assert!(der_to_raw(&[0x00, 0x01, 0x02]).is_none());
        assert!(der_to_raw(&[]).is_none());
    }

    #[test]
    fn rejects_redundant_leading_zero_byte() {
        // Our strict (DER, not BER-tolerant) decoder must reject this, since it
        // isn't the unique minimal encoding.
        // Wycheproof ecdsa_secp256r1_sha256_test.json (libcrux-kats), tcId 6
        // ("Legacy: ASN encoding of s misses leading 0"), result "invalid" with
        // flag "MissingZero": `s`'s high bit is not set, so it does not need a
        // leading zero byte, but the encoding includes one anyway.
        let sig = [
            0x30, 0x44, 0x02, 0x20, 0x2b, 0xa3, 0xa8, 0xbe, 0x6b, 0x94, 0xd5, 0xec, 0x80, 0xa6,
            0xd9, 0xd1, 0x19, 0x0a, 0x43, 0x6e, 0xff, 0xe5, 0x0d, 0x85, 0xa1, 0xee, 0xe8, 0x59,
            0xb8, 0xcc, 0x6a, 0xf9, 0xbd, 0x5c, 0x2e, 0x18, 0x02, 0x20, 0xb3, 0x29, 0xf4, 0x79,
            0xa2, 0xbb, 0xd0, 0xa5, 0xc3, 0x84, 0xee, 0x14, 0x93, 0xb1, 0xf5, 0x18, 0x6a, 0x87,
            0x13, 0x9c, 0xac, 0x5d, 0xf4, 0x08, 0x7c, 0x13, 0x4b, 0x49, 0x15, 0x68, 0x47, 0xdb,
        ];
        assert!(der_to_raw(&sig).is_none());
    }

    #[test]
    fn rejects_negative_integer() {
        // `s = 0x81` with no zero-padding byte: the high bit is set, so this
        // encodes a negative integer, which is invalid for an ECDSA scalar.
        let mut r = [0u8; 32];
        r[31] = 0x01;
        let der = raw_to_der(&r, &r);
        let mut sig = der.as_bytes().to_vec();
        let s_content_start = sig.len() - 1;
        sig[s_content_start] = 0x81;
        assert!(der_to_raw(&sig).is_none());
    }

    #[test]
    fn rejects_non_minimal_length_encoding() {
        // Long-form length (0x81 0x01) for a length that fits in short form:
        // BER permits this, but DER requires the minimal-length encoding.
        let sig = [
            SEQUENCE_TAG,
            0x08,
            INTEGER_TAG,
            0x81,
            0x01,
            0x01,
            INTEGER_TAG,
            0x01,
            0x01,
        ];
        assert!(der_to_raw(&sig).is_none());
    }
}
