//! Wycheproof ML-KEM Known Answer Tests
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::wycheproof::mlkem::{ParameterSet, MlKemTests};
//!
//! // load the tests for the ML-KEM-512 parameter set
//! let tests = MlKemTests::load(ParameterSet::MlKem512);
//!
//! for test_group in tests.keygen_and_decaps_tests() {
//!     for test in &test_group.tests {
//!         // ...
//!     }
//! }
//! ```

pub mod schema;

pub use schema::*;

pub enum TestGroupType {
    MlKemTest,
    MlKemEncapsTest,
    MlKemDecapsValidationTest,
}

macro_rules! impl_tests {
    ($name:ident, $name_encaps:ident, $name_decaps:ident) => {
        impl MlKemTests {
            fn $name() -> Self {
                let data: &str = include_str!(concat!(
                    "../../wycheproof/",
                    stringify!($name),
                    "_test.json"
                ));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }

            fn $name_encaps() -> Self {
                let data: &str = include_str!(concat!(
                    "../../wycheproof/",
                    stringify!($name_encaps),
                    "_test.json"
                ));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }

            fn $name_decaps() -> Self {
                let data: &str = include_str!(concat!(
                    "../../wycheproof/",
                    stringify!($name_decaps),
                    "_test.json"
                ));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }
        }
    };
}

impl_tests!(mlkem_512, mlkem_512_encaps, mlkem_512_semi_expanded_decaps);
impl_tests!(mlkem_768, mlkem_768_encaps, mlkem_768_semi_expanded_decaps);
impl_tests!(
    mlkem_1024,
    mlkem_1024_encaps,
    mlkem_1024_semi_expanded_decaps
);

impl schema::MlKemTests {
    pub fn load(parameter_set: ParameterSet, group_type: TestGroupType) -> Self {
        match (parameter_set, group_type) {
            (ParameterSet::MlKem512, TestGroupType::MlKemTest) => Self::mlkem_512(),
            (ParameterSet::MlKem512, TestGroupType::MlKemEncapsTest) => Self::mlkem_512_encaps(),
            (ParameterSet::MlKem512, TestGroupType::MlKemDecapsValidationTest) => {
                Self::mlkem_512_semi_expanded_decaps()
            }
            (ParameterSet::MlKem768, TestGroupType::MlKemTest) => Self::mlkem_768(),
            (ParameterSet::MlKem768, TestGroupType::MlKemEncapsTest) => Self::mlkem_768_encaps(),
            (ParameterSet::MlKem768, TestGroupType::MlKemDecapsValidationTest) => {
                Self::mlkem_768_semi_expanded_decaps()
            }
            (ParameterSet::MlKem1024, TestGroupType::MlKemTest) => Self::mlkem_1024(),
            (ParameterSet::MlKem1024, TestGroupType::MlKemEncapsTest) => Self::mlkem_1024_encaps(),
            (ParameterSet::MlKem1024, TestGroupType::MlKemDecapsValidationTest) => {
                Self::mlkem_1024_semi_expanded_decaps()
            }
        }
    }

    pub fn keygen_and_decaps_tests(&self) -> impl Iterator<Item = &MlKemTestGroup> {
        self.test_groups.iter().filter_map(|g| {
            if let TestGroup::MlKemTestGroup(g) = g {
                Some(g)
            } else {
                None
            }
        })
    }

    pub fn encaps_tests(&self) -> impl Iterator<Item = &MlKemEncapsTestGroup> {
        self.test_groups.iter().filter_map(|g| {
            if let TestGroup::MlKemEncapsTestGroup(g) = g {
                Some(g)
            } else {
                None
            }
        })
    }

    pub fn semi_expanded_decaps_tests(
        &self,
    ) -> impl Iterator<Item = &MlKemDecapsValidationTestGroup> {
        self.test_groups.iter().filter_map(|g| {
            if let TestGroup::MlKemDecapsValidationTestGroup(g) = g {
                Some(g)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // test that data is loaded successfully
    #[test]
    fn test_load() {
        MlKemTests::load(ParameterSet::MlKem512, TestGroupType::MlKemTest);
        MlKemTests::load(ParameterSet::MlKem768, TestGroupType::MlKemTest);
        MlKemTests::load(ParameterSet::MlKem1024, TestGroupType::MlKemTest);

        MlKemTests::load(ParameterSet::MlKem512, TestGroupType::MlKemEncapsTest);
        MlKemTests::load(ParameterSet::MlKem768, TestGroupType::MlKemEncapsTest);
        MlKemTests::load(ParameterSet::MlKem1024, TestGroupType::MlKemEncapsTest);

        MlKemTests::load(
            ParameterSet::MlKem512,
            TestGroupType::MlKemDecapsValidationTest,
        );
        MlKemTests::load(
            ParameterSet::MlKem768,
            TestGroupType::MlKemDecapsValidationTest,
        );
        MlKemTests::load(
            ParameterSet::MlKem1024,
            TestGroupType::MlKemDecapsValidationTest,
        );
    }
}
