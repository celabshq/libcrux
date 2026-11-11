//! Schema and Monte Carlo test drivers shared by the ACVP SHA-2, SHA-3 and SHAKE vector sets.
//!
//! See [`super::sha2`] and [`super::sha3`] for the vector sets.

pub mod schema;

use schema::MctResult;

pub struct HashTests {
    pub prompts: super::schema_common::Prompts<schema::HashPromptTestGroup>,
    pub results: super::schema_common::Results<schema::ResultHashTestGroup>,
}

/// The SHA-2 Monte Carlo test (`mctVersion` `"standard"`) from `seed`, with `hash` the hash
/// function. Returns the 100 outer iterations.
pub fn sha2_mct(seed: &[u8], mut hash: impl FnMut(&[u8]) -> Vec<u8>) -> Vec<MctResult> {
    let mut seed = seed.to_vec();
    (0..100)
        .map(|_| {
            // A = B = C = SEED; MSG = A || B || C; A = B; B = C; C = SHA(MSG)
            let (mut a, mut b, mut c) = (seed.clone(), seed.clone(), seed.clone());
            for _ in 0..1000 {
                let md = hash(&[a, b.clone(), c.clone()].concat());
                (a, b, c) = (b, c, md);
            }
            seed = c;
            MctResult::new(seed.clone())
        })
        .collect()
}

/// The SHA-3 Monte Carlo test (`mctVersion` `"standard"`) from `seed`, with `hash` the hash
/// function. Returns the 100 outer iterations.
pub fn sha3_mct(seed: &[u8], mut hash: impl FnMut(&[u8]) -> Vec<u8>) -> Vec<MctResult> {
    let mut md = seed.to_vec();
    (0..100)
        .map(|_| {
            // MD[i] = SHA3(MD[i-1])
            for _ in 0..1000 {
                md = hash(&md);
            }
            MctResult::new(md.clone())
        })
        .collect()
}

/// The SHAKE Monte Carlo test (`mctVersion` `"standard"`) from `seed`, for output lengths from
/// `min_out_len` to `max_out_len` bits, with `xof(output, msg)` the XOF. Returns the 100 outer
/// iterations.
pub fn shake_mct(
    seed: &[u8],
    min_out_len: usize,
    max_out_len: usize,
    mut xof: impl FnMut(&mut [u8], &[u8]),
) -> Vec<MctResult> {
    assert_eq!(min_out_len % 8, 0, "only byte-oriented output lengths");
    assert_eq!(max_out_len % 8, 0, "only byte-oriented output lengths");
    let min_out_bytes = min_out_len / 8;
    let max_out_bytes = max_out_len / 8;
    let range = max_out_bytes - min_out_bytes + 1;
    let mut out_len = max_out_bytes;

    let mut md = seed.to_vec();
    (0..100)
        .map(|_| {
            for _ in 0..1000 {
                // The 128 leftmost bits of the previous output, padded with zeros.
                let mut msg = [0u8; 16];
                let n = md.len().min(16);
                msg[..n].copy_from_slice(&md[..n]);

                md = vec![0u8; out_len];
                xof(&mut md, &msg);

                let rightmost = u16::from_be_bytes([md[md.len() - 2], md[md.len() - 1]]);
                out_len = min_out_bytes + (rightmost as usize % range);
            }
            MctResult {
                outLen: Some(md.len() * 8),
                md: md.clone(),
            }
        })
        .collect()
}
