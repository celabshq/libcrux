mod blake2b;
mod blake2s;
mod error;
mod lengths;

pub use blake2b::{Blake2b, Blake2bBuilder};
pub use blake2s::{Blake2s, Blake2sBuilder};
pub use error::Error;
pub use lengths::{SupportsKeyLen, SupportsOutLen};

/// Type that holds the constants in case both key length and digest length are known at compile
/// time.
pub struct ConstKeyLenConstDigestLen<const KEY_LEN: usize, const OUT_LEN: usize>;

/// Type that holds the constant in case just the key length is known at compile time.
pub struct ConstKeyLen<const KEY_LEN: usize>;

/// Type that holds the constant in case just the digest length is known at compile time.
pub struct ConstDigestLen<const OUT_LEN: usize>;

/// Type that holds the constant in case neither the key length nor the digest length is known at
/// compile time.
pub struct Dynamic;

/// Type that is used to bound the implementations to valid key and output lengths.
///
/// We implement [`SupportsKeyLen`] and [`SupportsOutLen`] for [`Blake2s<LengthBounds>`] and
/// [`Blake2b<LengthBounds>`] with the appropriate lengths. This can be used by callers to bound
/// their own implementations to only accept valid lengths.
///
/// # Example
///
/// ```
/// use libcrux_blake2::{Blake2b, Blake2bBuilder, LengthBounds, SupportsKeyLen, SupportsOutLen};
///
/// // A function that does a keyed Blake2b and only accepts valid key lengths
/// fn keyed_hash<const KEY_LEN: usize>(key: &[u8; KEY_LEN], msg: &[u8]) -> [u8; 32]
/// where
///     Blake2b<LengthBounds>: SupportsKeyLen<KEY_LEN>,
/// {
///     let mut hasher = Blake2bBuilder::new_keyed_const(key).build_const_digest_len::<32>();
///     hasher.update(msg);
///     let mut output = [0u8; 32];
///     hasher.finalize(&mut output);
///     output
/// }
///
/// // This compiles because Blake2b supports 32 byte keys
/// let key = [0; 32]; // this should actually be random
/// let msg = b"a test message";
/// let result = keyed_hash(&key, msg);
/// ```
pub struct LengthBounds;
