#![no_main]

use libcrux_p384::PublicKey;
use libfuzzer_sys::fuzz_target;

// Any successfully parsed key must round-trip through re-encoding in the
// same form `TryFrom` dispatched on (49 bytes -> compressed, 97 bytes ->
// uncompressed). This also exercises the `assert!` inside
// `AffinePoint::from_compressed` that guards against a zero Weierstrass RHS
// on attacker-controlled input.
fuzz_target!(|data: &[u8]| {
    let Ok(pk) = PublicKey::try_from(data) else {
        return;
    };

    match data.len() {
        49 => {
            let mut out = [0u8; 49];
            pk.to_compressed(&mut out);
            assert_eq!(out.as_slice(), data);
        }
        97 => {
            let mut out = [0u8; 97];
            pk.to_uncompressed(&mut out);
            assert_eq!(out.as_slice(), data);
        }
        _ => unreachable!("TryFrom only succeeds for len 49 or 97"),
    }
});
