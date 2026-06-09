pub mod util {
    pub fn randombytes(n: usize) -> Vec<u8> {
        use rand::{rngs::SysRng, TryRng};

        let mut bytes = vec![0u8; n];
        SysRng.try_fill_bytes(&mut bytes).unwrap();
        bytes
    }

    pub fn fmt(x: usize) -> String {
        let base = (x as f64).log(1024f64).floor() as usize;
        let suffix = ["", "KB", "MB", "GB"];
        format!("{} {}", x >> (10 * base), suffix[base])
    }

    pub fn hex_str_to_bytes(val: &str) -> Vec<u8> {
        let b: Result<Vec<u8>, std::num::ParseIntError> = (0..val.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
            .collect();
        b.expect("Error parsing hex string")
    }

    pub fn hex_str_to_array<A>(val: &str) -> A
    where
        A: Default + AsMut<[u8]>,
    {
        let b: Result<Vec<u8>, std::num::ParseIntError> = (0..val.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
            .collect();
        let b = b.expect("Error parsing hex string");
        let mut out = A::default();
        A::as_mut(&mut out).clone_from_slice(&b);
        out
    }
}
