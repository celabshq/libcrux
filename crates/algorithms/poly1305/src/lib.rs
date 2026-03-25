#![no_std]

/// The length of Poly1305 keys (r: 16 bytes + s: 16 bytes).
pub const KEY_LEN: usize = 32;

/// The length of Poly1305 MAC tags.
pub const TAG_LEN: usize = 16;

/// Describes the error conditions of the Poly1305 MAC.
pub enum Error {
    /// Indicates that the message argument is too large for the library to handle.
    MessageTooLarge,

    /// Indicates that the MAC tag is invalid for that key and message.
    InvalidMacTag,
}

#[cfg(not(feature = "expose-hacl"))]
mod hacl {
    pub(crate) mod mac_poly1305;
}

#[cfg(feature = "expose-hacl")]
pub mod hacl {
    pub mod mac_poly1305;
}

mod impl_hacl;

pub use impl_hacl::*;
