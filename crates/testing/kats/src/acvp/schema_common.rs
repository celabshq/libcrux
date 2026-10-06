use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Prompts<TG> {
    pub vsId: usize,
    pub algorithm: String,
    /// Not present in all ACVP files, e.g. `KDF-1.0`.
    #[serde(default)]
    pub mode: String,
    pub revision: String,
    pub isSample: bool,
    pub testGroups: Vec<TG>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Results<TG> {
    pub vsId: usize,
    pub algorithm: String,
    /// Not present in all ACVP files, e.g. `KDF-1.0`.
    #[serde(default)]
    pub mode: String,
    pub revision: String,
    pub isSample: bool,
    pub testGroups: Vec<TG>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TestGroupResults<R> {
    pub tgId: usize,
    pub tests: Vec<R>,
}

pub trait TestResult {
    fn tc_id(&self) -> usize;
}

impl<T: TestResult> Results<TestGroupResults<T>> {
    #[allow(non_snake_case)]
    pub fn find_expected_result(&self, tgId: usize, tcId: usize) -> &T {
        self.testGroups
            .iter()
            .find(|tg| tg.tgId == tgId)
            .unwrap_or_else(|| panic!("testGroup {} should have been deserialized", tgId))
            .tests
            .iter()
            .find(|t| t.tc_id() == tcId)
            .unwrap_or_else(|| panic!("testCase {} should have been deserialized", tcId))
    }
}
