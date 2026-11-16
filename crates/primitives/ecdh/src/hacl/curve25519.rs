#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    InvalidInput,
}

/// Compute the ECDH with the `private_key` and `public_key`.
///
/// Returns the 32 bytes shared key.
#[inline(always)]
pub fn ecdh(
    private_key: impl AsRef<[u8; 32]>,
    public_key: impl AsRef<[u8; 32]>,
) -> Result<[u8; 32], Error> {
    let mut shared = [0u8; 32];
    libcrux_curve25519::ecdh(&mut shared, public_key.as_ref(), private_key.as_ref())
        .map(|_| shared)
        .map_err(|_| Error::InvalidInput)
}

/// Compute the public key for the provided `private_key` (scalar multiplication
/// with the base point).
///
/// Returns the 32 bytes shared key.
#[must_use]
#[inline(always)]
pub fn secret_to_public(private_key: impl AsRef<[u8; 32]>) -> [u8; 32] {
    let mut public = [0u8; 32];
    libcrux_curve25519::secret_to_public(&mut public, private_key.as_ref());
    public
}
