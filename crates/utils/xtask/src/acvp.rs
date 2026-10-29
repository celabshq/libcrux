//! ACVP tasks, using the ACVP-Server gen/val image in `.docker/acvp-server`.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use clap::Subcommand;
use colored::*;

use crate::{run, workspace_root, GlobalOpts};

/// Where `docker-acvp-server.yml` publishes the image, tagged with the git tree hash of
/// `.docker/acvp-server`.
const IMAGE_REPOSITORY: &str = "ghcr.io/celabshq/libcrux-acvp-server";
const IMAGE_CONTEXT: &str = ".docker/acvp-server";

/// Exit code of `acvp-harness` for an algorithm that has no implementation for the backend.
const HARNESS_UNSUPPORTED: i32 = 2;

/// Backends that implement every algorithm, so that no vector set may be skipped.
const COMPLETE_BACKENDS: [&str; 2] = ["portable", "default"];

#[derive(Subcommand)]
pub(crate) enum AcvpTask {
    /// Generate vector sets with the ACVP server, answer them with libcrux, and validate the
    /// answers with the server.
    ///
    /// Fails unless every vector set passes.
    Run {
        /// Backends to test: default, portable, neon, avx2. Can be repeated. Defaults to
        /// portable, default, and the hardware backend of the host (neon or avx2).
        #[arg(long = "backend")]
        backends: Vec<String>,
        /// Directory for vector sets, responses and validations. Must be mountable by Docker.
        #[arg(long)]
        work_dir: Option<PathBuf>,
        /// The ACVP-Server image. Defaults to the published image for `.docker/acvp-server`.
        #[arg(long)]
        image: Option<String>,
        #[arg(help = registrations_help())]
        registrations: Vec<String>,
    },

    /// Generate vector sets for the file-based tests in `crates/testing/kats/acvp`.
    ///
    /// The prompt and expected results are written next to each registration and minified.
    Generate {
        /// Keep the `internalProjection.json`, which contains values the prompt and expected
        /// results do not, e.g. the private keys of the IUT in sample KAS vector sets.
        #[arg(long)]
        keep_projection: bool,
        /// The ACVP-Server image. Defaults to the published image for `.docker/acvp-server`.
        #[arg(long)]
        image: Option<String>,
        /// Registrations, each in its own directory below the workspace.
        #[arg(required = true)]
        registrations: Vec<PathBuf>,
    },

    /// Build the ACVP-Server image.
    Image {
        /// The image tag. Defaults to the published image for `.docker/acvp-server`.
        #[arg(long)]
        image: Option<String>,
    },
}

pub(crate) fn acvp(global: GlobalOpts, task: AcvpTask) -> Result<(), String> {
    match task {
        AcvpTask::Run {
            backends,
            work_dir,
            image,
            registrations,
        } => {
            let registrations = registrations
                .iter()
                .map(|r| registration(r))
                .collect::<Result<_, _>>()?;
            let image = ensure_image(&global, image)?;
            run_vector_sets(&global, backends, work_dir, &image, registrations)
        }
        AcvpTask::Generate {
            keep_projection,
            image,
            registrations,
        } => {
            let image = ensure_image(&global, image)?;
            generate(&global, keep_projection, &image, registrations)
        }
        AcvpTask::Image { image } => {
            let image = match image {
                Some(image) => image,
                None => default_image()?.0,
            };
            build_image(&global, &image)
        }
    }
}

fn git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(workspace_root())
        .output()
        .map_err(|e| format!("failed to run git: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// The published image for the committed `.docker/acvp-server`, and whether the directory has
/// uncommitted changes. In that case the image is tagged `dirty` and must be built locally.
fn default_image() -> Result<(String, bool), String> {
    let dirty = !git(&["status", "--porcelain", "--", IMAGE_CONTEXT])?.is_empty();
    let tag = if dirty {
        "dirty".to_string()
    } else {
        git(&["rev-parse", &format!("HEAD:{IMAGE_CONTEXT}")])?
    };
    Ok((format!("{IMAGE_REPOSITORY}:{tag}"), dirty))
}

fn build_image(global: &GlobalOpts, image: &str) -> Result<(), String> {
    let context = workspace_root().join(IMAGE_CONTEXT);
    run(
        Command::new("docker")
            .arg("build")
            .arg(&context)
            .args(["-t", image]),
        global.verbose,
    )
}

/// Make sure the image exists locally for the architecture of the Docker host: pull it if it is
/// published, build it otherwise. An image for another architecture would run emulated.
fn ensure_image(global: &GlobalOpts, image: Option<String>) -> Result<String, String> {
    let (image, dirty) = match image {
        Some(image) => (image, false),
        None => default_image()?,
    };
    if dirty {
        println!("{IMAGE_CONTEXT} has uncommitted changes, building {image}");
        build_image(global, &image)?;
        return Ok(image);
    }

    // The output of a successful docker command.
    let docker = |args: &[&str]| -> Result<Option<String>, String> {
        let output = Command::new("docker")
            .args(args)
            .output()
            .map_err(|e| format!("failed to run docker: {e}"))?;
        Ok(output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string()))
    };
    // Docker reports the host as e.g. `aarch64`, and images as e.g. `arm64`.
    let arch = match docker(&["info", "--format", "{{.Architecture}}"])?.as_deref() {
        Some("aarch64" | "arm64") => "arm64",
        Some("x86_64" | "amd64") => "amd64",
        Some(a) => return Err(format!("unsupported Docker host architecture {a}")),
        None => return Err("failed to run docker info".to_string()),
    };

    match docker(&["image", "inspect", "--format", "{{.Architecture}}", &image])? {
        Some(a) if a == arch => return Ok(image),
        Some(a) => println!("{image} is for {a}, not {arch}, replacing it"),
        None => (),
    }
    crate::info!(global.verbose, "Pulling {image} for linux/{arch}");
    let platform = format!("linux/{arch}");
    if docker(&["pull", "--platform", &platform, &image])?.is_some() {
        return Ok(image);
    }
    println!("{image} is not published for {platform}, building it");
    build_image(global, &image)?;
    Ok(image)
}

/// `--user uid:gid`, so that the files the container writes to the mounted directory are owned
/// by the caller rather than root.
#[cfg(unix)]
fn docker_user() -> Result<Vec<String>, String> {
    let id = |flag: &str| -> Result<String, String> {
        let output = Command::new("id")
            .arg(flag)
            .output()
            .map_err(|e| format!("failed to run id: {e}"))?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    };
    Ok(vec![
        "--user".to_string(),
        format!("{}:{}", id("-u")?, id("-g")?),
    ])
}

#[cfg(not(unix))]
fn docker_user() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

/// Run the image with `dir` mounted at `/work`, and the output written to `log`.
fn docker(
    global: &GlobalOpts,
    image: &str,
    dir: &Path,
    args: &[String],
    log: &Path,
) -> Result<(), String> {
    let mut cmd = Command::new("docker");
    cmd.args(["run", "--rm"])
        .args(docker_user()?)
        .arg("-v")
        .arg(format!("{}:/work", dir.display()))
        .arg(image)
        .args(args);
    crate::info!(global.verbose, "Running: {cmd:?}");
    let output = cmd
        .output()
        .map_err(|e| format!("failed to run docker: {e}"))?;
    let mut text = output.stdout;
    text.extend(output.stderr);
    fs::write(log, &text).map_err(|e| format!("failed to write {}: {e}", log.display()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "docker {} failed, see {}:\n{}",
            args[0],
            log.display(),
            String::from_utf8_lossy(&text)
        ))
    }
}

/// `path` inside the container, where `root` is mounted at `/work`.
fn container_path(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| format!("{} is not below {}", path.display(), root.display()))?;
    Ok(format!("/work/{}", relative.display()))
}

fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let data =
        fs::read_to_string(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    serde_json::from_str(&data).map_err(|e| format!("failed to parse {}: {e}", path.display()))
}

fn copy(from: &Path, to: &Path) -> Result<(), String> {
    fs::copy(from, to)
        .map(|_| ())
        .map_err(|e| format!("failed to copy {} to {}: {e}", from.display(), to.display()))
}

fn generate(
    global: &GlobalOpts,
    keep_projection: bool,
    image: &str,
    registrations: Vec<PathBuf>,
) -> Result<(), String> {
    let root = workspace_root();
    let registrations = registrations
        .iter()
        .map(|r| fs::canonicalize(r).map_err(|e| format!("{}: {e}", r.display())))
        .collect::<Result<Vec<_>, _>>()?;

    let mut args = vec!["generate".to_string()];
    for reg in &registrations {
        args.push(container_path(&root, reg)?);
    }
    let log = root.join("target/acvp-generate.log");
    fs::create_dir_all(log.parent().unwrap()).map_err(|e| e.to_string())?;
    docker(global, image, &root, &args, &log)?;

    for reg in &registrations {
        let dir = reg.parent().unwrap();
        for log in ["Check.log", "Generate.log"] {
            let _ = fs::remove_file(dir.join(log));
        }
        let projection = dir.join("internalProjection.json");
        if !keep_projection {
            fs::remove_file(&projection)
                .map_err(|e| format!("failed to remove {}: {e}", projection.display()))?;
        }
        for file in [
            "prompt.json",
            "expectedResults.json",
            "internalProjection.json",
        ] {
            let path = dir.join(file);
            if path.exists() {
                let minified = serde_json::to_string(&read_json(&path)?).unwrap() + "\n";
                fs::write(&path, minified)
                    .map_err(|e| format!("failed to write {}: {e}", path.display()))?;
            }
        }
        println!("Generated {}", dir.display());
    }
    Ok(())
}

fn default_backends() -> Vec<String> {
    let mut backends = vec!["portable".to_string(), "default".to_string()];
    match std::env::consts::ARCH {
        "aarch64" => backends.push("neon".to_string()),
        "x86_64" => backends.push("avx2".to_string()),
        _ => (),
    }
    backends
}

fn registrations_dir() -> PathBuf {
    workspace_root().join("crates/testing/acvp-harness/registrations")
}

/// The names of the registrations in `crates/testing/acvp-harness/registrations`.
fn harness_registrations() -> Vec<String> {
    let mut names: Vec<_> = fs::read_dir(registrations_dir())
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .filter_map(|p| Some(p.file_stem()?.to_string_lossy().to_string()))
        .collect();
    names.sort();
    names
}

fn registrations_help() -> String {
    format!(
        "Registrations to test, by name or path. Defaults to all of: {}",
        harness_registrations().join(", ")
    )
}

/// The registration file for `name`, which is either a path or the name of a registration in
/// `crates/testing/acvp-harness/registrations`.
fn registration(name: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(name);
    if path.is_file() {
        return Ok(path);
    }
    let path = registrations_dir().join(format!("{name}.json"));
    if path.is_file() {
        return Ok(path);
    }
    Err(format!(
        "unknown registration {name}, expected a path or one of: {}",
        harness_registrations().join(", ")
    ))
}

/// Build the harness and return the path of its executable.
fn build_harness(global: &GlobalOpts, root: &Path) -> Result<PathBuf, String> {
    let mut cmd = Command::new("cargo");
    cmd.args([
        "build",
        "--release",
        "--message-format=json-render-diagnostics",
    ])
    .arg("--manifest-path")
    .arg(root.join("crates/testing/acvp-harness/Cargo.toml"))
    .stderr(Stdio::inherit());
    crate::info!(global.verbose, "Running: {cmd:?}");
    let output = cmd
        .output()
        .map_err(|e| format!("failed to run cargo: {e}"))?;
    if !output.status.success() {
        return Err("failed to build acvp-harness".to_string());
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|m| m["reason"] == "compiler-artifact" && m["target"]["name"] == "acvp-harness")
        .find_map(|m| m["executable"].as_str().map(PathBuf::from))
        .ok_or_else(|| "cargo did not report the acvp-harness executable".to_string())
}

/// The disposition of a validation, the number of test cases, and the ids of the failed ones.
fn disposition(validation: &Path) -> Result<(String, usize, Vec<u64>), String> {
    let v = read_json(validation)?;
    let v = match &v {
        serde_json::Value::Array(a) => a.last().cloned().unwrap_or_default(),
        v => v.clone(),
    };
    let disposition = v["disposition"]
        .as_str()
        .ok_or_else(|| format!("{} has no disposition", validation.display()))?
        .to_string();
    let tests = v["tests"]
        .as_array()
        .ok_or_else(|| format!("{} has no tests", validation.display()))?;
    let failed = tests
        .iter()
        .filter(|t| t["result"] != "passed")
        .map(|t| {
            t["tcId"]
                .as_u64()
                .ok_or_else(|| format!("{} has a test without tcId", validation.display()))
        })
        .collect::<Result<_, _>>()?;
    Ok((disposition, tests.len(), failed))
}

/// Remove the test cases the harness listed in `notImplemented.json` from the projection in
/// `dir`, so that only the implemented ones are validated. Returns the number of test cases per
/// reason, and the number of test cases left.
fn remove_not_implemented(dir: &Path) -> Result<(BTreeMap<String, usize>, usize), String> {
    let projection_path = dir.join("internalProjection.json");
    let mut projection = read_json(&projection_path)?;
    let not_implemented_path = dir.join("notImplemented.json");
    let not_implemented = if not_implemented_path.exists() {
        read_json(&not_implemented_path)?
    } else {
        serde_json::Value::Array(Vec::new())
    };

    let mut reasons = BTreeMap::new();
    let mut ids = BTreeSet::new();
    for n in not_implemented.as_array().into_iter().flatten() {
        let reason = n["reason"].as_str().unwrap_or("?").to_string();
        *reasons.entry(reason).or_insert(0) += 1;
        ids.insert((n["tgId"].as_u64(), n["tcId"].as_u64()));
    }

    let vector_set = match &mut projection {
        serde_json::Value::Array(a) => a
            .last_mut()
            .ok_or_else(|| format!("{} is empty", projection_path.display()))?,
        v => v,
    };
    let groups = vector_set["testGroups"]
        .as_array_mut()
        .ok_or_else(|| format!("{} has no testGroups", projection_path.display()))?;
    for group in groups.iter_mut() {
        let tg_id = group["tgId"].as_u64();
        if let Some(tests) = group["tests"].as_array_mut() {
            tests.retain(|t| !ids.contains(&(tg_id, t["tcId"].as_u64())));
        }
    }
    groups.retain(|g| g["tests"].as_array().is_some_and(|t| !t.is_empty()));
    let remaining = groups
        .iter()
        .map(|g| g["tests"].as_array().map_or(0, Vec::len))
        .sum();

    if !ids.is_empty() {
        fs::write(&projection_path, projection.to_string())
            .map_err(|e| format!("failed to write {}: {e}", projection_path.display()))?;
    }
    Ok((reasons, remaining))
}

/// Results of the vector sets of one backend.
#[derive(Default)]
struct Summary {
    passed_sets: usize,
    failed_sets: usize,
    skipped_sets: usize,
    not_implemented_sets: usize,
    passed_tests: usize,
    failed_tests: usize,
    not_implemented_tests: usize,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let failed_sets = format!("{} failed", self.failed_sets);
        let failed_tests = format!("{} failed", self.failed_tests);
        let (failed_sets, failed_tests) = if self.failed_sets > 0 {
            (failed_sets.red(), failed_tests.red())
        } else {
            (failed_sets.normal(), failed_tests.normal())
        };
        write!(
            f,
            "vector sets: {} passed, {failed_sets}, {} skipped, {} not implemented; \
             tests: {} passed, {failed_tests}, {} not implemented",
            self.passed_sets,
            self.skipped_sets,
            self.not_implemented_sets,
            self.passed_tests,
            self.not_implemented_tests
        )
    }
}

fn run_vector_sets(
    global: &GlobalOpts,
    backends: Vec<String>,
    work_dir: Option<PathBuf>,
    image: &str,
    registrations: Vec<PathBuf>,
) -> Result<(), String> {
    let root = workspace_root();
    let backends = if backends.is_empty() {
        default_backends()
    } else {
        backends
    };
    let registrations = if registrations.is_empty() {
        harness_registrations()
            .iter()
            .map(|r| registration(r))
            .collect::<Result<_, _>>()?
    } else {
        registrations
    };
    let work = work_dir.unwrap_or_else(|| root.join("target/acvp"));

    let harness = build_harness(global, &root)?;

    // One directory per vector set, as the server writes its files next to the registration.
    if work.exists() {
        fs::remove_dir_all(&work)
            .map_err(|e| format!("failed to remove {}: {e}", work.display()))?;
    }
    let vectors = work.join("vectors");
    let mut names = Vec::new();
    for reg in &registrations {
        let name = reg
            .file_stem()
            .ok_or_else(|| format!("invalid registration {}", reg.display()))?
            .to_string_lossy()
            .to_string();
        fs::create_dir_all(vectors.join(&name)).map_err(|e| e.to_string())?;
        copy(reg, &vectors.join(&name).join("registration.json"))?;
        names.push(name);
    }

    println!(
        "{}",
        format!("Generating {} vector sets", names.len()).bold()
    );
    let mut args = vec!["generate".to_string()];
    args.extend(
        names
            .iter()
            .map(|n| format!("/work/vectors/{n}/registration.json")),
    );
    docker(global, image, &work, &args, &work.join("generate.log"))?;

    let mut failed = false;
    let mut summaries = Vec::new();
    // The test cases per reason that are not implemented, per vector set.
    let mut not_implemented = BTreeMap::new();
    for backend in &backends {
        println!("{}", format!("Running libcrux ({backend})").bold());
        let mut summary = Summary::default();
        let complete = COMPLETE_BACKENDS.contains(&backend.as_str());
        // The server writes validation.json next to the answers, so each backend gets its
        // own copy of the projection.
        let mut answered = Vec::new();
        let mut skipped = Vec::new();
        let mut crashed = Vec::new();
        let mut unimplemented = Vec::new();
        let mut not_implemented_tests = BTreeMap::new();
        for name in &names {
            let dir = work.join(backend).join(name);
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            copy(
                &vectors.join(name).join("internalProjection.json"),
                &dir.join("internalProjection.json"),
            )?;
            let status = Command::new(&harness)
                .args(["--backend", backend])
                .arg(vectors.join(name).join("prompt.json"))
                .arg(dir.join("response.json"))
                .status()
                .map_err(|e| format!("failed to run {}: {e}", harness.display()))?;
            match status.code() {
                Some(0) => {
                    let (reasons, remaining) = remove_not_implemented(&dir)?;
                    let count: usize = reasons.values().sum();
                    summary.not_implemented_tests += count;
                    not_implemented_tests.insert(name, count);
                    if !reasons.is_empty() {
                        not_implemented.entry(name.clone()).or_insert(reasons);
                    }
                    if remaining == 0 {
                        unimplemented.push(name);
                    } else {
                        answered.push(name);
                    }
                }
                Some(HARNESS_UNSUPPORTED) => skipped.push(name),
                _ => crashed.push((name, status)),
            }
        }

        println!("{}", format!("Validating ({backend})").bold());
        let mut args = vec!["validate".to_string()];
        for name in &answered {
            args.push(format!("/work/{backend}/{name}/internalProjection.json"));
            args.push(format!("/work/{backend}/{name}/response.json"));
        }
        // A failed validation is reported per vector set below.
        if let Err(e) = docker(
            global,
            image,
            &work,
            &args,
            &work.join(backend).join("validate.log"),
        ) {
            failed = true;
            eprintln!("{}", e.red());
        }

        for (name, status) in crashed {
            summary.failed_sets += 1;
            println!(
                "{backend:<10} {name:<40} {}",
                format!("acvp-harness failed ({status})").red()
            );
        }
        for name in skipped {
            if complete {
                summary.failed_sets += 1;
                println!(
                    "{backend:<10} {name:<40} {}",
                    "skipped, but the backend must implement every algorithm".red()
                );
            } else {
                summary.skipped_sets += 1;
                println!(
                    "{backend:<10} {name:<40} {}",
                    format!("skipped (not implemented for {backend})").yellow()
                );
            }
        }
        for name in unimplemented {
            summary.not_implemented_sets += 1;
            println!(
                "{backend:<10} {name:<40} {}",
                format!("not implemented ({} tests)", not_implemented_tests[name]).yellow()
            );
        }
        for name in answered {
            let not_implemented = match not_implemented_tests[name] {
                0 => String::new(),
                n => format!(", {}", format!("{n} not implemented").yellow()),
            };
            match disposition(&work.join(backend).join(name).join("validation.json")) {
                Ok((disposition, total, failures)) => {
                    summary.passed_tests += total - failures.len();
                    summary.failed_tests += failures.len();
                    if disposition == "passed" && failures.is_empty() {
                        summary.passed_sets += 1;
                        println!(
                            "{backend:<10} {name:<40} {} ({total} tests{not_implemented})",
                            disposition.green()
                        );
                    } else {
                        summary.failed_sets += 1;
                        println!(
                            "{backend:<10} {name:<40} {} ({} of {total} tests failed{not_implemented}: {failures:?})",
                            disposition.red(),
                            failures.len()
                        );
                    }
                }
                Err(e) => {
                    summary.failed_sets += 1;
                    println!("{backend:<10} {name:<40} {}", e.red());
                }
            }
        }
        failed |= summary.failed_sets > 0;
        summaries.push((backend, summary));
    }

    if !not_implemented.is_empty() {
        println!("{}", "Not implemented (not validated)".bold());
        for (name, reasons) in &not_implemented {
            for (reason, count) in reasons {
                println!("{name:<40} {reason:<40} {count:>5} tests");
            }
        }
    }

    println!("{}", "Summary".bold());
    for (backend, summary) in summaries {
        println!("{backend:<10} {summary}");
    }

    if failed {
        Err(format!(
            "not all vector sets passed, see {}",
            work.display()
        ))
    } else {
        Ok(())
    }
}
