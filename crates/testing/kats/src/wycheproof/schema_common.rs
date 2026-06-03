use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestResult {
    Valid,
    Invalid,
    Acceptable,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub version: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesEntry {
    /// The type of the bug tested for
    pub bug_type: String,

    /// A description of the flag
    pub description: Option<String>,

    /// The expected effect of failing the test vector
    pub effect: Option<String>,

    /// A list of potentially related references
    #[serde(default)]
    pub links: Vec<String>,

    /// A list of potentially related CVEs
    #[serde(default)]
    pub cves: Vec<String>,
}
