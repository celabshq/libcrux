use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// A vector set: the `prompt.json`, `expectedResults.json` or `internalProjection.json`.
///
/// The serialization is the format of the response to a vector set.
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct VectorSet<TG> {
    pub vsId: usize,
    pub algorithm: String,
    /// Not present in all ACVP files, e.g. `KDF-1.0`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mode: String,
    pub revision: String,
    pub isSample: bool,
    pub testGroups: Vec<TG>,
}

pub type Prompts<TG> = VectorSet<TG>;
pub type Results<TG> = VectorSet<TG>;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct TestGroupResults<R> {
    pub tgId: usize,
    pub tests: Vec<R>,
}

/// A test case, identified by its `tcId`.
pub trait TestCase {
    fn tc_id(&self) -> usize;
}

pub use TestCase as TestResult;

/// A test group, identified by its `tgId`.
pub trait TestGroup {
    type Test: TestCase;

    fn tg_id(&self) -> usize;
    fn tests(&self) -> &[Self::Test];
}

/// The `(tgId, tcId)` of all test cases of a test group.
///
/// Implemented for every [`TestGroup`], and for groups whose test cases are not a single list.
pub trait TestIds {
    fn test_ids(&self) -> Vec<(usize, usize)>;
}

impl<G: TestGroup> TestIds for G {
    fn test_ids(&self) -> Vec<(usize, usize)> {
        let tg_id = self.tg_id();
        self.tests().iter().map(|t| (tg_id, t.tc_id())).collect()
    }
}

impl<R: TestCase> TestGroup for TestGroupResults<R> {
    type Test = R;

    fn tg_id(&self) -> usize {
        self.tgId
    }

    fn tests(&self) -> &[R] {
        &self.tests
    }
}

impl<TG: TestGroup> VectorSet<TG> {
    pub fn find_group(&self, tg_id: usize) -> &TG {
        self.testGroups
            .iter()
            .find(|tg| tg.tg_id() == tg_id)
            .unwrap_or_else(|| panic!("testGroup {tg_id} should have been deserialized"))
    }

    pub fn find_test(&self, tg_id: usize, tc_id: usize) -> &TG::Test {
        self.find_group(tg_id)
            .tests()
            .iter()
            .find(|t| t.tc_id() == tc_id)
            .unwrap_or_else(|| panic!("testCase {tc_id} should have been deserialized"))
    }

    #[allow(non_snake_case)]
    pub fn find_expected_result(&self, tgId: usize, tcId: usize) -> &TG::Test {
        self.find_test(tgId, tcId)
    }
}

impl<TG: TestIds> VectorSet<TG> {
    /// The `(tgId, tcId)` of all test cases. Panics on duplicates.
    pub fn test_ids(&self) -> BTreeSet<(usize, usize)> {
        let mut ids = BTreeSet::new();
        for id in self.testGroups.iter().flat_map(TestIds::test_ids) {
            assert!(ids.insert(id), "duplicate test case (tgId, tcId) {id:?}");
        }
        ids
    }

    pub fn test_count(&self) -> usize {
        self.test_ids().len()
    }
}

/// Assert that `prompts` and `results` belong to the same vector set and contain the same test
/// cases, so that every expected result is checked when every prompt is.
pub fn assert_covers<P: TestIds, R: TestIds>(prompts: &VectorSet<P>, results: &VectorSet<R>) {
    assert_eq!(prompts.vsId, results.vsId, "vsId");
    assert_eq!(prompts.algorithm, results.algorithm, "algorithm");
    assert_eq!(prompts.mode, results.mode, "mode");
    assert_eq!(prompts.revision, results.revision, "revision");
    assert_eq!(
        prompts.test_ids(),
        results.test_ids(),
        "the prompts and expected results contain different test cases"
    );
}

/// Counts the test cases a test ran or skipped per category, e.g. the `testType`, to assert
/// that no category silently disappears.
#[derive(Default, Debug)]
pub struct Tally(BTreeMap<String, usize>);

impl Tally {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, category: impl Into<String>) {
        self.add_n(category, 1);
    }

    pub fn add_n(&mut self, category: impl Into<String>, n: usize) {
        *self.0.entry(category.into()).or_default() += n;
    }

    pub fn total(&self) -> usize {
        self.0.values().sum()
    }

    /// Assert the exact counts per category. Categories with an expected count of 0 must not
    /// have been added.
    pub fn assert_counts<K: Into<String>>(&self, expected: impl IntoIterator<Item = (K, usize)>) {
        let mut want = BTreeMap::new();
        for (category, n) in expected {
            if n > 0 {
                *want.entry(category.into()).or_default() += n;
            }
        }
        assert_eq!(self.0, want, "test cases per category");
    }
}

/// Upper case hex (de)serialization, as used by ACVP responses. Decoding accepts both cases.
pub mod upper_hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub use hex::serde::deserialize;

    pub fn serialize<S: Serializer, T: AsRef<[u8]>>(
        data: T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&hex::encode_upper(data))
    }

    /// For optional fields, which need `#[serde(default)]` to be absent.
    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer, T: AsRef<[u8]>>(
            data: &Option<T>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match data {
                Some(data) => serializer.serialize_some(&hex::encode_upper(data)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<Vec<u8>>, D::Error> {
            Option::<String>::deserialize(deserializer)?
                .map(|s| hex::decode(s).map_err(serde::de::Error::custom))
                .transpose()
        }
    }

    /// For optional lists, which need `#[serde(default)]` to be absent.
    pub mod option_vec {
        use super::*;

        pub fn serialize<S: Serializer>(
            data: &Option<Vec<Vec<u8>>>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match data {
                Some(data) => serializer
                    .serialize_some(&data.iter().map(hex::encode_upper).collect::<Vec<_>>()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<Vec<Vec<u8>>>, D::Error> {
            Option::<Vec<String>>::deserialize(deserializer)?
                .map(|v| {
                    v.into_iter()
                        .map(|s| hex::decode(s).map_err(serde::de::Error::custom))
                        .collect()
                })
                .transpose()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct Hex {
        #[serde(with = "upper_hex")]
        a: Vec<u8>,
        #[serde(with = "upper_hex")]
        b: [u8; 2],
        #[serde(
            default,
            with = "upper_hex::option",
            skip_serializing_if = "Option::is_none"
        )]
        c: Option<Vec<u8>>,
        #[serde(
            default,
            with = "upper_hex::option_vec",
            skip_serializing_if = "Option::is_none"
        )]
        d: Option<Vec<Vec<u8>>>,
    }

    #[test]
    fn upper_hex_round_trip() {
        let v: Hex = serde_json::from_str(r#"{"a":"abCD","b":"0f10","d":["aa",""]}"#).unwrap();
        assert_eq!(
            v,
            Hex {
                a: vec![0xab, 0xcd],
                b: [0x0f, 0x10],
                c: None,
                d: Some(vec![vec![0xaa], vec![]]),
            }
        );
        assert_eq!(
            serde_json::to_string(&v).unwrap(),
            r#"{"a":"ABCD","b":"0F10","d":["AA",""]}"#
        );
    }

    #[test]
    fn tally() {
        let mut t = Tally::new();
        t.add("AFT");
        t.add_n("MCT", 2);
        t.assert_counts([("AFT", 1), ("MCT", 2), ("LDT", 0)]);
        assert_eq!(t.total(), 3);
    }

    #[test]
    #[should_panic(expected = "test cases per category")]
    fn tally_missing_category() {
        let mut t = Tally::new();
        t.add("AFT");
        t.assert_counts([("AFT", 1), ("MCT", 1)]);
    }
}
