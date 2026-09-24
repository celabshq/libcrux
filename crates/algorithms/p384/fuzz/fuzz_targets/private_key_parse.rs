#![no_main]

use libcrux_p384::PrivateKey;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = PrivateKey::try_from(data);
});
