//! Wycheproof ML-DSA Known Answer Tests
//!
//! ### Example usage
//! ```rust
//! use libcrux_kats::wycheproof::mldsa::{ParameterSet, MlDsaSignTestsNoSeed};
//!
//! // load the tests for the ML-DSA-44 parameter set
//! let signing_tests = MlDsaSignTestsNoSeed::load(ParameterSet::MlDsa44);
//!
//! for test_group in signing_tests.test_groups {
//!     for test in test_group.tests {
//!         // ...
//!     }
//! }
//! ```

pub mod sign_common;
pub mod sign_noseed_schema;
pub mod sign_seed_schema;

pub mod verify_schema;

pub use sign_noseed_schema::MlDsaSignTestsNoSeed;
pub use sign_seed_schema::MlDsaSignTestsWithSeed;
pub use verify_schema::MlDsaVerifyTests;

/// Parameter sets for ML-DSA
pub enum ParameterSet {
    MlDsa44,
    MlDsa65,
    MlDsa87,
}

macro_rules! impl_sign_noseed {
    ($name:ident, $parameter_set:literal) => {
        impl MlDsaSignTestsNoSeed {
            fn $name() -> Self {
                let data: &str = include_str!(concat!(
                    "../../wycheproof/mldsa_",
                    $parameter_set,
                    "_sign_noseed_test.json"
                ));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }
        }
    };
}

impl_sign_noseed!(sign_44, 44);
impl_sign_noseed!(sign_65, 65);
impl_sign_noseed!(sign_87, 87);

macro_rules! impl_sign_seed {
    ($name:ident, $parameter_set:literal) => {
        impl MlDsaSignTestsWithSeed {
            fn $name() -> Self {
                let data: &str = include_str!(concat!(
                    "../../wycheproof/mldsa_",
                    $parameter_set,
                    "_sign_seed_test.json"
                ));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }
        }
    };
}
impl_sign_seed!(sign_44, 44);
impl_sign_seed!(sign_65, 65);
impl_sign_seed!(sign_87, 87);

macro_rules! impl_verify {
    ($name:ident, $parameter_set:literal) => {
        impl MlDsaVerifyTests {
            fn $name() -> Self {
                let data: &str = include_str!(concat!(
                    "../../wycheproof/mldsa_",
                    $parameter_set,
                    "_verify_test.json"
                ));
                serde_json::from_str(data).expect("Could not deserialize KAT file.")
            }
        }
    };
}

impl_verify!(verify_44, 44);
impl_verify!(verify_65, 65);
impl_verify!(verify_87, 87);

impl MlDsaSignTestsNoSeed {
    /// Load the [`MlDsaSignTestsNoSeed`] for the given [`ParameterSet`].
    pub fn load(parameter_set: ParameterSet) -> Self {
        match parameter_set {
            ParameterSet::MlDsa44 => Self::sign_44(),
            ParameterSet::MlDsa65 => Self::sign_65(),
            ParameterSet::MlDsa87 => Self::sign_87(),
        }
    }
}
impl MlDsaSignTestsWithSeed {
    /// Load the [`MlDsaSignTestsWithSeed`] for the given [`ParameterSet`].
    pub fn load(parameter_set: ParameterSet) -> Self {
        match parameter_set {
            ParameterSet::MlDsa44 => Self::sign_44(),
            ParameterSet::MlDsa65 => Self::sign_65(),
            ParameterSet::MlDsa87 => Self::sign_87(),
        }
    }
}

impl MlDsaVerifyTests {
    /// Load the [`MlDsaVerifyTests`] for the given [`ParameterSet`].
    pub fn load(parameter_set: ParameterSet) -> Self {
        match parameter_set {
            ParameterSet::MlDsa44 => Self::verify_44(),
            ParameterSet::MlDsa65 => Self::verify_65(),
            ParameterSet::MlDsa87 => Self::verify_87(),
        }
    }
}
