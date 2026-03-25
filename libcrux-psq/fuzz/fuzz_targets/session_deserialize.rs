//! Fuzz `Session::deserialize`.
//!
//! Serialized sessions may be stored in persistent storage and later loaded
//! back.  Feed arbitrary bytes to the deserializer to verify that malformed
//! or adversarially crafted blobs cannot cause panics or undefined behaviour.
#![no_main]

use libcrux_psq::session::Session;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // No session binding — exercises the raw TLS-codec deserialization path
    // without any MAC/binding verification gating.
    let _ = Session::deserialize(data, None);
});
