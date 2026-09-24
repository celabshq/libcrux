use std::{
    env, fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "xtask", about = "Build and development tasks for libcrux")]
struct Cli {
    #[command(flatten)]
    global: GlobalOpts,
    #[command(subcommand)]
    command: Task,
}

#[derive(clap::Args, Clone)]
struct GlobalOpts {
    /// Enable verbose output.
    #[arg(long, short, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Task {
    /// Run tests with coverage instrumentation and generate an HTML report.
    Coverage {
        /// Directory to write the coverage report into.
        #[arg(long, default_value = "coverage")]
        output_dir: PathBuf,
        /// Run coverage over the entire workspace instead of the current crate.
        #[arg(long)]
        workspace: bool,
        /// Remove profraw files after generating the report.
        #[arg(long)]
        clean: bool,
        /// Extra arguments forwarded to `cargo test`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        cargo_args: Vec<String>,
    },
    /// Remove profraw files left behind by a previous coverage run.
    CoverageClean {
        /// Clean the entire workspace instead of the current crate.
        #[arg(long)]
        workspace: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Task::Coverage {
            output_dir,
            workspace,
            clean,
            cargo_args,
        } => coverage(cli.global, output_dir, workspace, clean, cargo_args),
        Task::CoverageClean { workspace } => {
            let run_dir = if workspace {
                Ok(workspace_root())
            } else {
                env::current_dir().map_err(|e| format!("failed to get current dir: {e}"))
            };
            run_dir.and_then(|d| cleanup_profraw(&d, cli.global.verbose))
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Coverage
// ---------------------------------------------------------------------------

fn coverage(
    global: GlobalOpts,
    output_dir: PathBuf,
    workspace: bool,
    clean: bool,
    cargo_args: Vec<String>,
) -> Result<(), String> {
    let run_dir = if workspace {
        workspace_root()
    } else {
        env::current_dir().map_err(|e| format!("failed to get current dir: {e}"))?
    };

    if clean {
        return cleanup_profraw(&run_dir, global.verbose);
    }

    fs::create_dir_all(&output_dir).map_err(|e| format!("failed to create output dir: {e}"))?;

    let workspace_flag = if workspace { "--workspace" } else { "" };

    // Build & test with source-based coverage instrumentation.
    run(
        cmd(&format!("cargo test {workspace_flag}"))
            .args(&cargo_args)
            .current_dir(&run_dir)
            .env("CARGO_INCREMENTAL", "0")
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", "cargo-test-%p-%m.profraw"),
        global.verbose,
    )?;

    // Generate the HTML report with grcov.
    run(
        cmd(&format!(
            r#"grcov .
                --binary-path {}/target/debug/deps
                --source-dir .
                --output-types html
                --branch
                --ignore-not-existing
                --ignore '../*'
                --ignore "/*"
                --output-path {}"#,
            workspace_root().display(),
            output_dir.display(),
        ))
        .current_dir(&run_dir),
        global.verbose,
    )?;

    println!(
        "Coverage report written to: {}",
        output_dir.join("html/index.html").display()
    );
    Ok(())
}

fn cleanup_profraw(root: &PathBuf, verbose: bool) -> Result<(), String> {
    let pattern = format!("{}/**/*.profraw", root.display());
    debug!(verbose, "Deleting all files in {pattern} ...");
    let files: Vec<_> = glob::glob(&pattern)
        .unwrap_or_else(|e| panic!("invalid glob pattern: {e}"))
        .flatten()
        .collect();
    info!(verbose, "Removing {} profraw file(s)", files.len());
    debug!(verbose, "  {:?}", files);
    for path in files {
        fs::remove_file(&path).map_err(|e| format!("failed to remove {}: {e}", path.display()))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a whitespace-separated command string into a [`Command`].
fn cmd(s: &str) -> Command {
    let mut parts = s.split_whitespace().filter(|p| !p.is_empty());
    let program = parts.next().expect("empty command string");
    let mut c = Command::new(program);
    c.args(parts);
    c
}

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points to the xtask crate (crates/utils/xtask);
    // go up three levels to reach the workspace root.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().expect("current dir"));
    manifest_dir
        .ancestors()
        .nth(3)
        .unwrap_or(&manifest_dir)
        .to_path_buf()
}

fn run(cmd: &mut Command, verbose: bool) -> Result<(), String> {
    info!(verbose, "Running: {cmd:?} with env: {:?}", cmd.get_envs());
    let status: ExitStatus = cmd
        .status()
        .map_err(|e| format!("failed to run {:?}: {e}", cmd.get_program()))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "{:?} exited with status {}",
            cmd.get_program(),
            status
        ))
    }
}

macro_rules! info {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            eprintln!("{}", format_args!($($arg)*).to_string().blue());
        }
    };
}
use info;

macro_rules! debug {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            eprintln!("{}", format_args!($($arg)*).to_string().red());
        }
    };
}
use debug;
