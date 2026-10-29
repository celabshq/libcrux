//! ACVP test harness for libcrux.
//!
//! Reads an ACVP vector set (`prompt.json`) and writes the responses of libcrux in the
//! format of `expectedResults.json`, so that they can be validated by the ACVP server.
//! A vector set in the format of the ACVP web API, `[{"acvVersion": ..}, {..}]`, is answered in
//! the same format.
//!
//! ```sh
//! acvp-harness [--backend default|portable|neon|avx2] <prompt.json> <response.json>
//! ```
//!
//! The backend selects the implementation for algorithms that have several (SHA-3, SHAKE,
//! AES, ML-KEM, ML-DSA). `default` uses the platform-multiplexed API. Algorithms with a single
//! implementation ignore it.
//!
//! Test cases that libcrux does not implement, e.g. ML-DSA with external mu, are left out of the
//! response and listed in `notImplemented.json` next to it, as `[{"tgId", "tcId", "reason"}]`.
//!
//! Exits with code 2 if the algorithm is not implemented for the backend, and with code 1 on any
//! other error.

use std::{
    cell::{Cell, RefCell},
    env, fs, panic,
    path::Path,
    process::exit,
    sync::OnceLock,
};

use libcrux_kats::acvp::schema_common::{
    Prompts, TestCase, TestGroup, TestGroupResults, VectorSet,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

mod aes;
mod ccm;
mod curves;
mod drbg;
mod ecdsa;
mod ecdsa_sig;
mod eddsa;
mod hash;
mod hkdf;
mod hmac;
mod kas;
mod kbkdf;
mod kda;
mod kmac;
mod mldsa;
mod mlkem;
mod xecdh;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Backend {
    Default,
    Portable,
    Neon,
    Avx2,
}

static VS_ID: OnceLock<u64> = OnceLock::new();

thread_local! {
    /// The `(tgId, tcId)` being answered, for error messages.
    static CONTEXT: Cell<(Option<usize>, Option<usize>)> = const { Cell::new((None, None)) };
    /// The test cases that are not implemented, see [`not_implemented`].
    static NOT_IMPLEMENTED: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) };
}

/// Leave the test case being answered out of the response, because libcrux does not implement
/// `reason`.
pub(crate) fn not_implemented<R>(reason: impl std::fmt::Display) -> Option<R> {
    let (tg_id, tc_id) = CONTEXT.get();
    NOT_IMPLEMENTED.with_borrow_mut(|n| {
        n.push(json!({ "tgId": tg_id, "tcId": tc_id, "reason": reason.to_string() }))
    });
    None
}

/// The location of an error: the vector set, test group and test case being answered.
fn location() -> String {
    let mut location = match VS_ID.get() {
        Some(vs_id) => format!("vsId {vs_id}"),
        None => "vsId ?".to_string(),
    };
    let (tg_id, tc_id) = CONTEXT.get();
    if let Some(tg_id) = tg_id {
        location += &format!(", tgId {tg_id}");
    }
    if let Some(tc_id) = tc_id {
        location += &format!(", tcId {tc_id}");
    }
    location
}

fn fail(message: impl std::fmt::Display) -> ! {
    eprintln!("error: {}: {message}", location());
    exit(1)
}

/// Run `f` with `(tg_id, tc_id)` as the location reported if it panics.
pub(crate) fn within<T>(tg_id: usize, tc_id: Option<usize>, f: impl FnOnce() -> T) -> T {
    let outer = CONTEXT.replace((Some(tg_id), tc_id));
    let result = f();
    CONTEXT.set(outer);
    result
}

/// Answer each test case of the test group `tg_id` with `f`.
pub(crate) fn answer<T: TestCase, R>(
    tg_id: usize,
    tests: &[T],
    mut f: impl FnMut(&T) -> R,
) -> Vec<R> {
    tests
        .iter()
        .map(|t| within(tg_id, Some(t.tc_id()), || f(t)))
        .collect()
}

/// Answer each test case of the test group `tg_id` with [`not_implemented`], because of
/// `reason`. The result has no test cases, so callers should leave it out of the response.
pub(crate) fn not_implemented_group<T: TestCase, R>(
    tg_id: usize,
    tests: &[T],
    reason: impl std::fmt::Display,
) -> TestGroupResults<R> {
    for t in tests {
        within(tg_id, Some(t.tc_id()), || not_implemented::<()>(&reason));
    }
    TestGroupResults {
        tgId: tg_id,
        tests: Vec::new(),
    }
}

/// Answer each test case of the test group `tg_id` with `f`, which may return
/// [`not_implemented`].
pub(crate) fn answer_implemented<T: TestCase, R>(
    tg_id: usize,
    tests: &[T],
    f: impl FnMut(&T) -> Option<R>,
) -> Vec<R> {
    answer(tg_id, tests, f).into_iter().flatten().collect()
}

/// The response to `prompts`, with the test groups `test_groups`.
pub(crate) fn response<TG, RG: Serialize>(prompts: &VectorSet<TG>, test_groups: Vec<RG>) -> Value {
    serde_json::to_value(VectorSet {
        vsId: prompts.vsId,
        algorithm: prompts.algorithm.clone(),
        mode: prompts.mode.clone(),
        revision: prompts.revision.clone(),
        isSample: prompts.isSample,
        testGroups: test_groups,
    })
    .unwrap()
}

/// The response to `prompts`, with `f` answering each test case.
pub(crate) fn respond<TG: TestGroup, R: Serialize>(
    prompts: &Prompts<TG>,
    mut f: impl FnMut(&TG, &TG::Test) -> R,
) -> Value {
    respond_implemented(prompts, |g, t| Some(f(g, t)))
}

/// The response to `prompts`, with `f` answering each test case, or returning
/// [`not_implemented`]. Test groups without any answered test case are left out.
pub(crate) fn respond_implemented<TG: TestGroup, R: Serialize>(
    prompts: &Prompts<TG>,
    mut f: impl FnMut(&TG, &TG::Test) -> Option<R>,
) -> Value {
    let test_groups = prompts
        .testGroups
        .iter()
        .map(|g| TestGroupResults {
            tgId: g.tg_id(),
            tests: answer(g.tg_id(), g.tests(), |t| f(g, t))
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        })
        .filter(|g| !g.tests.is_empty())
        .collect();
    response(prompts, test_groups)
}

/// The response to a vector set of which libcrux implements nothing: every test case is
/// [`not_implemented`] because of `reason`. Needs no schema, only `tgId` and `tcId`.
fn all_not_implemented(prompt: &Value, reason: &str) -> Value {
    let ids = |v: &Value, field| {
        v[field]
            .as_u64()
            .and_then(|id| usize::try_from(id).ok())
            .unwrap_or_else(|| fail(format!("missing {field}")))
    };
    let Some(groups) = prompt["testGroups"].as_array() else {
        fail("the prompt has no testGroups")
    };
    for g in groups {
        let tg_id = ids(g, "tgId");
        let Some(tests) = g["tests"].as_array() else {
            within(tg_id, None, || fail("the test group has no tests"))
        };
        for t in tests {
            let tc_id = ids(t, "tcId");
            within(tg_id, Some(tc_id), || not_implemented::<()>(reason));
        }
    }
    let mut response = json!({
        "vsId": prompt["vsId"],
        "algorithm": prompt["algorithm"],
        "revision": prompt["revision"],
        "isSample": prompt["isSample"],
        "testGroups": [],
    });
    if let Some(mode) = prompt.get("mode") {
        response["mode"] = mode.clone();
    }
    response
}

/// Parse the prompt into the schema of [`libcrux_kats`].
pub(crate) fn parse<T: DeserializeOwned>(prompt: Value) -> T {
    serde_json::from_value(prompt).unwrap_or_else(|e| fail(format!("invalid prompt: {e}")))
}

pub(crate) fn unsupported(what: &str, backend: Backend) -> ! {
    eprintln!("{what} is not supported on backend {backend:?}");
    exit(2)
}

fn usage() -> ! {
    eprintln!(
        "usage: acvp-harness [--backend default|portable|neon|avx2] <prompt.json> <response.json>"
    );
    exit(1)
}

fn main() {
    let mut args = env::args().skip(1);
    let mut backend = Backend::Default;
    let mut paths = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--backend" {
            backend = match args.next().as_deref() {
                Some("default") => Backend::Default,
                Some("portable") => Backend::Portable,
                Some("neon") => Backend::Neon,
                Some("avx2") => Backend::Avx2,
                _ => usage(),
            };
        } else {
            paths.push(arg);
        }
    }
    let [prompt_path, response_path] = paths.as_slice() else {
        usage()
    };

    panic::set_hook(Box::new(|info| fail(info)));

    let prompt = fs::read_to_string(prompt_path)
        .unwrap_or_else(|e| fail(format!("could not read {prompt_path}: {e}")));
    let prompt: Value = serde_json::from_str(&prompt)
        .unwrap_or_else(|e| fail(format!("{prompt_path} is not JSON: {e}")));
    // The ACVP web API wraps the vector set as `[{"acvVersion": ..}, vector set]`.
    let (acv_version, prompt) = match prompt {
        Value::Array(a) => match <[Value; 2]>::try_from(a) {
            Ok([version, prompt]) if version.get("acvVersion").is_some() => (Some(version), prompt),
            _ => fail("expected a vector set, or [{\"acvVersion\": ..}, vector set]"),
        },
        prompt => (None, prompt),
    };
    if let Some(vs_id) = prompt["vsId"].as_u64() {
        VS_ID.set(vs_id).unwrap();
    }
    let Some(algorithm) = prompt["algorithm"].as_str().map(str::to_string) else {
        fail("the prompt has no algorithm")
    };
    let mode = prompt["mode"].as_str().unwrap_or_default().to_string();

    let response = match (algorithm.as_str(), mode.as_str()) {
        (
            "SHA2-224" | "SHA2-256" | "SHA2-384" | "SHA2-512" | "SHA3-224" | "SHA3-256"
            | "SHA3-384" | "SHA3-512",
            _,
        ) => hash::sha(prompt, &algorithm, backend),
        ("SHAKE-128" | "SHAKE-256", _) => hash::shake(prompt, &algorithm, backend),
        (
            "HMAC-SHA2-256" | "HMAC-SHA2-384" | "HMAC-SHA2-512" | "HMAC-SHA3-224" | "HMAC-SHA3-256"
            | "HMAC-SHA3-384" | "HMAC-SHA3-512",
            _,
        ) => hmac::respond(prompt, &algorithm),
        ("ACVP-AES-ECB", _) => aes::ecb(prompt, backend),
        ("ACVP-AES-GCM", _) => aes::gcm(prompt, backend),
        ("KDA", "TwoStep") => kda::respond(prompt),
        ("KAS-ECC-SSC", _) => kas::respond(prompt),
        ("ECDSA", "keyGen") => ecdsa::keygen(prompt),
        ("ECDSA", "keyVer") => ecdsa::keyver(prompt),
        ("ECDSA", "sigGen") => ecdsa_sig::siggen(prompt),
        ("ECDSA", "sigVer") => ecdsa_sig::sigver(prompt),
        ("ACVP-AES-CCM", _) => ccm::ccm(prompt, backend),
        ("ML-KEM", "keyGen") => mlkem::keygen(prompt, backend),
        ("ML-KEM", "encapDecap") => mlkem::encap_decap(prompt, backend),
        ("hmacDRBG", _) => drbg::respond(prompt),
        ("EDDSA", "keyGen") => eddsa::keygen(prompt),
        ("EDDSA", "sigGen") => eddsa::siggen(prompt),
        ("EDDSA", "sigVer") => eddsa::sigver(prompt),
        ("XECDH", "keyGen") => xecdh::keygen(prompt),
        ("XECDH", "keyVer") => xecdh::keyver(prompt),
        ("XECDH", "SSC") => xecdh::ssc(prompt),
        ("ML-DSA", "keyGen") => mldsa::keygen(prompt, backend),
        ("ML-DSA", "sigGen") => mldsa::siggen(prompt, backend),
        ("ML-DSA", "sigVer") => mldsa::sigver(prompt, backend),
        ("KDF", "KMAC") => all_not_implemented(&prompt, "KDF in KMAC mode"),
        ("KDF", _) => kbkdf::respond(prompt),
        ("KDA", "HKDF") => hkdf::respond(prompt),
        ("KMAC-128" | "KMAC-256", _) => kmac::respond(prompt, &algorithm),
        (
            "SHA-1" | "SHA2-512/224" | "SHA2-512/256" | "HMAC-SHA-1" | "HMAC-SHA2-224"
            | "HMAC-SHA2-512/224" | "HMAC-SHA2-512/256",
            _,
        ) => all_not_implemented(&prompt, &algorithm),
        ("DetECDSA", "sigGen") => all_not_implemented(&prompt, "deterministic ECDSA"),
        ("EDDSA", "keyVer") => all_not_implemented(&prompt, "EdDSA keyVer"),
        ("KDA", "OneStep" | "OneStepNoCounter") => {
            all_not_implemented(&prompt, &format!("KDA {mode}"))
        }
        _ => fail(format!(
            "unknown vector set: algorithm {algorithm:?}, mode {mode:?}"
        )),
    };
    let response = match acv_version {
        Some(version) => json!([version, response]),
        None => response,
    };

    fs::write(
        response_path,
        serde_json::to_string_pretty(&response).unwrap() + "\n",
    )
    .unwrap_or_else(|e| fail(format!("could not write {response_path}: {e}")));

    let not_implemented = NOT_IMPLEMENTED.take();
    if !not_implemented.is_empty() {
        let path = Path::new(response_path).with_file_name("notImplemented.json");
        fs::write(
            &path,
            serde_json::to_string_pretty(&not_implemented).unwrap() + "\n",
        )
        .unwrap_or_else(|e| fail(format!("could not write {}: {e}", path.display())));
    }
}
