/// This trait captures the lengths of keys, tags and nonces used by an AEAD.
pub trait AeadConsts {
    /// The length of the key for this AEAD, in bytes.
    const KEY_LEN: usize;
    /// The length of the tag for this AEAD, in bytes.
    const TAG_LEN: usize;
    /// The length of the nonce for this AEAD, in bytes.
    const NONCE_LEN: usize;
}
