use super::super::schema_common::*;
use serde::{Deserialize, Serialize};

/// A test case. Which keys are present depends on the scheme, the role of the IUT and the
/// test type; absent keys are empty.
///
/// Public keys are given as affine coordinates, each a big-endian integer of the size of the
/// field.
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KasPrompt {
    pub tcId: usize,

    #[serde(default, with = "hex::serde")]
    pub ephemeralPublicServerX: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub ephemeralPublicServerY: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub staticPublicServerX: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub staticPublicServerY: Vec<u8>,

    /// VAL; AFT in the internal projection only.
    #[serde(default, with = "hex::serde")]
    pub ephemeralPrivateIut: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub ephemeralPublicIutX: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub ephemeralPublicIutY: Vec<u8>,

    /// VAL; AFT in the internal projection only.
    #[serde(default, with = "hex::serde")]
    pub staticPrivateIut: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub staticPublicIutX: Vec<u8>,
    #[serde(default, with = "hex::serde")]
    pub staticPublicIutY: Vec<u8>,

    /// The shared secret to check; VAL only.
    #[serde(default, with = "hex::serde")]
    pub z: Vec<u8>,
}

/// `ephemeral` if `is_ephemeral`, `static_` otherwise. The other one must be empty.
fn select<'a>(is_ephemeral: bool, ephemeral: &'a [u8], static_: &'a [u8]) -> &'a [u8] {
    let (selected, other) = if is_ephemeral {
        (ephemeral, static_)
    } else {
        (static_, ephemeral)
    };
    assert!(
        other.is_empty(),
        "unexpected {} key",
        if is_ephemeral { "static" } else { "ephemeral" }
    );
    selected
}

impl KasPrompt {
    /// The public key `(x, y)` of the server, see [`KasPromptTestGroup::server_ephemeral`].
    pub fn server_public_key(&self, ephemeral: bool) -> (&[u8], &[u8]) {
        (
            select(
                ephemeral,
                &self.ephemeralPublicServerX,
                &self.staticPublicServerX,
            ),
            select(
                ephemeral,
                &self.ephemeralPublicServerY,
                &self.staticPublicServerY,
            ),
        )
    }

    /// The private key of the IUT, see [`KasPromptTestGroup::iut_ephemeral`].
    pub fn iut_private_key(&self, ephemeral: bool) -> &[u8] {
        select(ephemeral, &self.ephemeralPrivateIut, &self.staticPrivateIut)
    }

    /// The public key `(x, y)` of the IUT, see [`KasPromptTestGroup::iut_ephemeral`].
    pub fn iut_public_key(&self, ephemeral: bool) -> (&[u8], &[u8]) {
        (
            select(ephemeral, &self.ephemeralPublicIutX, &self.staticPublicIutX),
            select(ephemeral, &self.ephemeralPublicIutY, &self.staticPublicIutY),
        )
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct KasPromptTestGroup {
    pub tgId: usize,
    /// `"AFT"` or `"VAL"`.
    pub testType: String,
    pub scheme: String,
    /// `"initiator"` or `"responder"`.
    pub kasRole: String,
    /// The curve, e.g. `"P-384"`.
    pub domainParameterGenerationMode: String,
    pub tests: Vec<KasPrompt>,
}

impl KasPromptTestGroup {
    /// Whether the IUT contributes an ephemeral (rather than a static) key pair.
    ///
    /// In `ephemeralUnified` both parties use ephemeral keys. In `onePassDh` the initiator uses
    /// an ephemeral key and the responder a static one.
    pub fn iut_ephemeral(&self) -> bool {
        match (self.scheme.as_str(), self.kasRole.as_str()) {
            ("ephemeralUnified", "initiator" | "responder") | ("onePassDh", "initiator") => true,
            ("onePassDh", "responder") => false,
            (s, r) => panic!("unexpected scheme {s} with kasRole {r}"),
        }
    }

    /// Whether the server contributes an ephemeral (rather than a static) key pair.
    pub fn server_ephemeral(&self) -> bool {
        match (self.scheme.as_str(), self.kasRole.as_str()) {
            ("ephemeralUnified", "initiator" | "responder") | ("onePassDh", "responder") => true,
            ("onePassDh", "initiator") => false,
            (s, r) => panic!("unexpected scheme {s} with kasRole {r}"),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct KasResult {
    pub tcId: usize,

    /// AFT only, if the IUT key is ephemeral.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ephemeralPublicIutX: Option<Vec<u8>>,
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ephemeralPublicIutY: Option<Vec<u8>>,
    /// AFT only, if the IUT key is static.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub staticPublicIutX: Option<Vec<u8>>,
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub staticPublicIutY: Option<Vec<u8>>,

    /// The shared secret; AFT only.
    #[serde(
        default,
        with = "upper_hex::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub z: Option<Vec<u8>>,

    /// VAL only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub testPassed: Option<bool>,
}

impl KasResult {
    /// The AFT result for the IUT public key `(x, y)` and the shared secret `z`.
    pub fn aft(tc_id: usize, iut_ephemeral: bool, x: Vec<u8>, y: Vec<u8>, z: Vec<u8>) -> Self {
        let (x, y) = (Some(x), Some(y));
        let (ephemeral, static_) = if iut_ephemeral {
            ((x, y), (None, None))
        } else {
            ((None, None), (x, y))
        };
        Self {
            tcId: tc_id,
            ephemeralPublicIutX: ephemeral.0,
            ephemeralPublicIutY: ephemeral.1,
            staticPublicIutX: static_.0,
            staticPublicIutY: static_.1,
            z: Some(z),
            testPassed: None,
        }
    }

    /// The public key `(x, y)` of the IUT in an AFT result, see
    /// [`KasPromptTestGroup::iut_ephemeral`].
    pub fn iut_public_key(&self, ephemeral: bool) -> (&[u8], &[u8]) {
        let (x, y, other) = if ephemeral {
            (
                &self.ephemeralPublicIutX,
                &self.ephemeralPublicIutY,
                &self.staticPublicIutX,
            )
        } else {
            (
                &self.staticPublicIutX,
                &self.staticPublicIutY,
                &self.ephemeralPublicIutX,
            )
        };
        assert!(other.is_none(), "tcId {}: unexpected IUT key", self.tcId);
        (
            x.as_deref().expect("missing IUT public key"),
            y.as_deref().expect("missing IUT public key"),
        )
    }
}

pub type ResultKasTestGroup = TestGroupResults<KasResult>;

impl_test_case!(KasPrompt, KasResult);
impl_test_group!(KasPromptTestGroup => KasPrompt);
