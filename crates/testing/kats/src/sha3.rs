pub use crate::cavp::{Sha3 as Hash, ShakeMsg, ShakeVariableOut, TestVector};

macro_rules! sha3_fn {
    ($name:ident, $file:literal) => {
        pub fn $name() -> TestVector<Hash> {
            crate::cavp::read_string(include_str!($file)).expect("failed to parse CAVP test vector")
        }
    };
}

macro_rules! shake_fn {
    ($name:ident, $file:literal) => {
        pub fn $name() -> TestVector<ShakeMsg> {
            crate::cavp::read_string(include_str!($file)).expect("failed to parse CAVP test vector")
        }
    };
}

macro_rules! shake_varout_fn {
    ($name:ident, $file:literal) => {
        pub fn $name() -> TestVector<ShakeVariableOut> {
            crate::cavp::read_string(include_str!($file)).expect("failed to parse CAVP test vector")
        }
    };
}

sha3_fn!(sha3_224_short, "../tv/sha3/SHA3_224ShortMsg.rsp");
sha3_fn!(sha3_224_long, "../tv/sha3/SHA3_224LongMsg.rsp");
sha3_fn!(sha3_256_short, "../tv/sha3/SHA3_256ShortMsg.rsp");
sha3_fn!(sha3_256_long, "../tv/sha3/SHA3_256LongMsg.rsp");
sha3_fn!(sha3_384_short, "../tv/sha3/SHA3_384ShortMsg.rsp");
sha3_fn!(sha3_384_long, "../tv/sha3/SHA3_384LongMsg.rsp");
sha3_fn!(sha3_512_short, "../tv/sha3/SHA3_512ShortMsg.rsp");
sha3_fn!(sha3_512_long, "../tv/sha3/SHA3_512LongMsg.rsp");

shake_fn!(shake128_short, "../tv/sha3/SHAKE128ShortMsg.rsp");
shake_fn!(shake128_long, "../tv/sha3/SHAKE128LongMsg.rsp");
shake_varout_fn!(shake128_variable_out, "../tv/sha3/SHAKE128VariableOut.rsp");

shake_fn!(shake256_short, "../tv/sha3/SHAKE256ShortMsg.rsp");
shake_fn!(shake256_long, "../tv/sha3/SHAKE256LongMsg.rsp");
shake_varout_fn!(shake256_variable_out, "../tv/sha3/SHAKE256VariableOut.rsp");
