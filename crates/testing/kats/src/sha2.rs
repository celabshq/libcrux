pub use crate::cavp::{Sha3 as Hash, TestVector};

macro_rules! load_fn {
    ($name:ident, $file:literal) => {
        pub fn $name() -> TestVector<Hash> {
            crate::cavp::read_string(include_str!($file)).expect("failed to parse CAVP test vector")
        }
    };
}

load_fn!(sha224_short, "../tv/sha2/SHA224ShortMsg.rsp");
load_fn!(sha224_long, "../tv/sha2/SHA224LongMsg.rsp");
load_fn!(sha256_short, "../tv/sha2/SHA256ShortMsg.rsp");
load_fn!(sha256_long, "../tv/sha2/SHA256LongMsg.rsp");
load_fn!(sha384_short, "../tv/sha2/SHA384ShortMsg.rsp");
load_fn!(sha384_long, "../tv/sha2/SHA384LongMsg.rsp");
load_fn!(sha512_short, "../tv/sha2/SHA512ShortMsg.rsp");
load_fn!(sha512_long, "../tv/sha2/SHA512LongMsg.rsp");
