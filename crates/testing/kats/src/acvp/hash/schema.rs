use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

/// A message for the large data tests (LDT), to be expanded to `fullLength` bits.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct LargeMessage {
    #[serde(with = "hex::serde")]
    pub content: Vec<u8>,
    /// Length of `content` in bits.
    pub contentLength: usize,
    /// Length of the expanded message in bits.
    pub fullLength: u64,
    /// Only `"repeating"` is defined: `content` is repeated until `fullLength` bits are reached.
    pub expansionTechnique: String,
}

impl LargeMessage {
    /// Length of the expanded message in bytes.
    fn full_len(&self) -> u64 {
        assert_eq!(self.expansionTechnique, "repeating");
        assert_eq!(self.contentLength, self.content.len() * 8);
        assert!(!self.content.is_empty());
        assert_eq!(self.fullLength % 8, 0, "only byte-oriented messages");
        self.fullLength / 8
    }

    /// The expanded message.
    pub fn expand(&self) -> Vec<u8> {
        let full_len = usize::try_from(self.full_len()).unwrap();
        self.content
            .iter()
            .copied()
            .cycle()
            .take(full_len)
            .collect()
    }

    /// Call `f` with consecutive chunks of the expanded message, without expanding it in memory.
    pub fn for_each_chunk(&self, mut f: impl FnMut(&[u8])) {
        const CHUNK_LEN: usize = 1 << 23;
        let mut remaining = self.full_len();
        // A whole number of repetitions, so that each chunk starts with `content`.
        let chunk = self.content.repeat((CHUNK_LEN / self.content.len()).max(1));
        while remaining > 0 {
            let len = remaining.min(chunk.len() as u64) as usize;
            f(&chunk[..len]);
            remaining -= len as u64;
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct HashPrompt {
    pub tcId: usize,

    /// Omitted in the LDT.
    #[serde(default, with = "hex::serde")]
    pub msg: Vec<u8>,
    /// Length of `msg` in bits. Omitted in the LDT.
    #[serde(default)]
    pub len: usize,

    /// Output length in bits; SHAKE AFT and VOT only.
    pub outLen: Option<usize>,

    /// LDT only.
    pub largeMsg: Option<LargeMessage>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct HashPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"`, `"MCT"`, `"LDT"` (SHA-3) or `"VOT"` (SHAKE).
    pub testType: String,
    /// MCT only.
    pub mctVersion: Option<String>,
    /// Smallest output length in bits; SHAKE MCT only.
    pub minOutLen: Option<usize>,
    /// Largest output length in bits; SHAKE MCT only.
    pub maxOutLen: Option<usize>,
    pub tests: Vec<HashPrompt>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct MctResult {
    #[serde(with = "upper_hex")]
    pub md: Vec<u8>,
    /// Output length in bits; SHAKE only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outLen: Option<usize>,
}

impl MctResult {
    /// A SHA-2 or SHA-3 result, without `outLen`.
    pub fn new(md: Vec<u8>) -> Self {
        Self { md, outLen: None }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct HashResult {
    pub tcId: usize,

    /// Omitted in the MCT.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub md: Option<Vec<u8>>,

    /// Output length in bits; SHAKE only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outLen: Option<usize>,

    /// MCT only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resultsArray: Vec<MctResult>,
}

pub type ResultHashTestGroup = TestGroupResults<HashResult>;

impl_test_case!(HashPrompt, HashResult);
impl_test_group!(HashPromptTestGroup => HashPrompt);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_message() {
        let large = LargeMessage {
            content: vec![1, 2, 3],
            contentLength: 24,
            fullLength: 8 * 7,
            expansionTechnique: "repeating".to_string(),
        };
        assert_eq!(large.expand(), [1, 2, 3, 1, 2, 3, 1]);
        let mut streamed = Vec::new();
        large.for_each_chunk(|c| streamed.extend_from_slice(c));
        assert_eq!(streamed, large.expand());
    }
}
