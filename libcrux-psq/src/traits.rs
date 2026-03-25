//! # Common Traits
//!
//! This module provides common internal traits used in the PSQ crate.

use crate::session::{Session, SessionError};

/// A trait for state machines which can transition into a `Session`.
pub trait IntoSession {
    /// Consumes the state machine, yielding a `Session`.
    fn into_session(self) -> Result<Session, SessionError>;
    /// Indicates whether the current state allows derivation of a `Session`.
    fn is_handshake_finished(&self) -> bool;
}

/// A common interface for writing and reading messsages to and from a
/// peer.
pub trait Channel<Error, Message> {
    /// Prepare the next message to be sent, leaving wire serialization to the
    /// caller.
    fn write_message_external_encoding(&mut self, payload: &[u8]) -> Result<Message, Error>;

    /// Process the next message, deserialized from the wire by the caller.
    fn read_message_external_encoding(&mut self, message: &Message) -> Result<Vec<u8>, Error>;

    /// Write a message to `out`.
    ///
    /// The message may include a `payload`. Returns the number of
    /// bytes written to `out`.  If the internal state is not ready to
    /// write a message, nothing is written to `out` and `Ok(0)` is
    /// returned.
    fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<usize, Error>;

    /// Reads the bytes in `message`, and writes any payload bytes
    /// contained to `payload`.
    ///
    /// Returns a pair of `(bytes_deserialized, bytes_written)`, where
    /// `bytes_deserialized` is the number of bytes read from
    /// `message` and `bytes_written` is the number of bytes written
    /// to `payload`. If the internal state is not ready to read a
    /// message, nothing is written to `payload` and `Ok((0,0))` is
    /// returned.
    fn read_message(&mut self, message: &[u8], payload: &mut [u8])
        -> Result<(usize, usize), Error>;
}
