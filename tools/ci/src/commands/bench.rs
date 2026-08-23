use std::fs;
use std::path::Path;

use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

/// Where baselines are committed, relative to this crate.
const BASELINE_HOME: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../benches/gungraun-baselines"
);

/// Where the ci tool installs `gungraun-runner` (kept off PATH). The runner
/// version must exactly match the `gungraun` library version, so it is
/// managed here instead of relying on a pre-existing installation.
const RUNNER_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/gungraun-runner");

struct Backend {
    /// Baseline name passed to `--save-baseline` / `--baseline`.
    name: &'static str,
    /// Features to enable on top of the default features.
    features: Option<&'static str>,
}

/// Instruction counts are only comparable between identical toolchains and
/// target triples, so baselines are tracked for x86_64 and aarch64 Linux
/// only (valgrind doesn't run on macOS at all).
fn host_backends() -> Vec<Backend> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => vec![
            Backend {
                name: "sse2",
                features: None,
            },
            Backend {
                name: "scalar_math",
                features: Some("scalar-math"),
            },
        ],
        ("linux", "aarch64") => vec![Backend {
            name: "neon",
            features: None,
        }],
        (os, arch) => {
            eprintln!("skipping gungraun benches: no baselines are tracked for {arch}-{os}");
            Vec::new()
        }
    }
}

/// Delete saved baseline files for `name` below `dir`, so that removed or
/// renamed benchmarks don't leave stale baselines behind.
fn remove_stale_baselines(dir: &Path, name: &str) {
    remove_with_suffix(dir, &format!(".base@{name}"));
}

fn remove_with_suffix(dir: &Path, suffix: &str) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            remove_with_suffix(&path, suffix);
        } else if path
            .file_name()
            .map_or(false, |n| n.to_string_lossy().ends_with(suffix))
        {
            let _ = fs::remove_file(&path);
        }
    }
}

/// Install the `gungraun-runner` binary matching the `gungraun` version in
/// `Cargo.lock` (skipped if that exact version is already installed).
///
/// The runner rejects any version mismatch with the `gungraun` library, so
/// the exact resolved version is needed here. Cargo.toml only contains a
/// version requirement, making Cargo.lock the canonical source.
fn gungraun_version(sh: &Shell) -> String {
    let lock_path = Path::new(BASELINE_HOME).join("../../Cargo.lock");
    if !lock_path.exists() {
        // Fresh checkout: generate Cargo.lock first
        sh.cmd("cargo")
            .arg("fetch")
            .run()
            .expect("cargo fetch failed");
    }
    let lockfile = cargo_lock::Lockfile::load(&lock_path).expect("failed to read Cargo.lock");
    lockfile
        .packages
        .iter()
        .find(|package| package.name.as_str() == "gungraun")
        .expect("gungraun not found in Cargo.lock")
        .version
        .to_string()
}

/// True if `gungraun-runner` with exactly `version` is already installed in
/// `RUNNER_ROOT`.
fn runner_installed(sh: &Shell, version: &str) -> bool {
    sh.cmd(format!("{RUNNER_ROOT}/bin/gungraun-runner"))
        .arg("--version")
        .read()
        .map_or(false, |stdout| {
            stdout.trim() == format!("gungraun-runner {version}")
        })
}

#[derive(FromArgs)]
#[argh(subcommand, name = "bench")]
/// Run gungraun benchmarks against the committed baselines (requires
/// valgrind; only x86_64 and aarch64 Linux hosts have tracked baselines)
pub struct Bench {
    #[argh(
        switch,
        description = "save new baselines for this host's backends instead of comparing"
    )]
    pub save: bool,
}

impl Prepare for Bench {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let backends = host_backends();
        if backends.is_empty() {
            return Vec::new();
        }

        let version = gungraun_version(sh);
        let runner_bin = format!("{RUNNER_ROOT}/bin/gungraun-runner");

        let mut cmds = Vec::new();
        if !runner_installed(sh, &version) {
            cmds.push(PreparedCommand {
                name: format!("install gungraun-runner {version}"),
                command: sh
                    .cmd("cargo")
                    .arg("install")
                    .arg("gungraun-runner")
                    .arg("--locked")
                    .arg("--force")
                    .arg("--version")
                    .arg(format!("={version}"))
                    .arg("--root")
                    .arg(RUNNER_ROOT),
                failure_message: "failed to install gungraun-runner",
            });
        }
        for backend in backends {
            let mut cmd = sh.cmd("cargo");
            cmd = cmd
                .env("GUNGRAUN_RUNNER", &runner_bin)
                .arg("bench")
                .arg("--bench")
                .arg("gungraun");
            if let Some(features) = backend.features {
                cmd = cmd.arg("--features").arg(features);
            }
            cmd = cmd.arg("--").arg(format!("--home={BASELINE_HOME}"));

            let (name, failure_message): (String, &'static str);
            if self.save {
                remove_stale_baselines(Path::new(BASELINE_HOME), backend.name);
                cmd = cmd.arg(format!("--save-baseline={}", backend.name));
                name = format!("bench (save {})", backend.name);
                failure_message = "failed to save gungraun baselines";
            } else {
                cmd = cmd
                    .arg(format!("--baseline={}", backend.name))
                    .arg("--callgrind-limits=ir=0%");
                name = format!("bench (check {})", backend.name);
                failure_message = "gungraun benchmarks regressed against the saved baselines";
            }

            cmds.push(PreparedCommand {
                name,
                command: cmd,
                failure_message,
            });
        }
        cmds
    }
}
