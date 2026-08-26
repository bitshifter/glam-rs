use std::fs;
use std::path::{Path, PathBuf};

use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

/// Where baselines are committed, relative to this crate.
const BASELINE_HOME: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../benches/gungraun-baselines"
);

/// Lockfile the benchmarks are built with. Instruction counts are only
/// reproducible for an identical dependency resolution (crate disambiguators
/// affect codegen), so the benchmarks pin their own lockfile instead of using
/// the repository's untracked, floating `Cargo.lock`.
const BENCH_LOCK: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../benches/gungraun-baselines/Cargo.lock"
);

/// Where the ci tool installs `gungraun-runner` (kept off PATH). The runner
/// version must exactly match the `gungraun` library version, so it is
/// managed here instead of relying on a pre-existing installation.
const RUNNER_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/gungraun-runner");

fn workspace_path(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(file)
}

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

/// The display name for a backend: like its baseline name but using dashes
/// (`scalar_math` -> `scalar-math`), matching how cargo features are spelled.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn display_name(backend: &Backend) -> String {
    backend.name.replace('_', "-")
}

/// Resolve a `--backend` argument to a `Backend` available on this host,
/// matching the baseline name with `-` and `_` treated interchangeably
/// (so `scalar-math` and `scalar_math` both resolve, like cargo feature names).
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn resolve_backend<'a>(backends: &'a [Backend], name: &str) -> Option<&'a Backend> {
    let normalized = name.replace('-', "_");
    backends.iter().find(|b| b.name == normalized)
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

/// The bench lockfile swapped in as `Cargo.lock`: `Some(Some(backup))` if an
/// original `Cargo.lock` was moved aside, `Some(None)` if none existed, or
/// `None` if no swap happened.
struct BenchLockGuard(Option<Option<PathBuf>>);

/// Swap the committed bench lockfile in as `Cargo.lock`; the original (if
/// any) is restored on drop.
fn swap_in_bench_lock() -> BenchLockGuard {
    let bench_lock = Path::new(BENCH_LOCK);
    if !bench_lock.exists() {
        return BenchLockGuard(None);
    }
    let root = workspace_path("Cargo.lock");
    let backup = workspace_path("Cargo.lock.bench-backup");
    if !root.exists() && backup.exists() {
        // Recover from a previous run that was killed mid-swap.
        let _ = fs::rename(&backup, &root);
    }
    let inner = if root.exists() {
        fs::rename(&root, &backup).expect("failed to back up Cargo.lock");
        Some(backup)
    } else {
        None
    };
    fs::copy(bench_lock, &root).expect("failed to copy the bench lockfile");
    BenchLockGuard(Some(inner))
}

impl Drop for BenchLockGuard {
    fn drop(&mut self) {
        let root = workspace_path("Cargo.lock");
        match &self.0 {
            None => {}
            Some(Some(backup)) => {
                let _ = fs::rename(backup, &root);
            }
            Some(None) => {
                let _ = fs::remove_file(&root);
            }
        }
    }
}

/// The resolved `gungraun` version from `Cargo.lock`. The runner binary must
/// be exactly this version (the runner rejects anything older or newer).
fn gungraun_version(sh: &Shell, locked: bool) -> String {
    let lock_path = workspace_path("Cargo.lock");
    if !lock_path.exists() {
        // Fresh checkout: generate Cargo.lock first
        let mut cmd = sh.cmd("cargo");
        cmd = cmd.arg("fetch");
        if locked {
            cmd = cmd.arg("--locked");
        }
        cmd.run().expect("cargo fetch failed");
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
        description = "save new baselines and the bench lockfile instead of comparing"
    )]
    pub save: bool,

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[argh(
        option,
        description = "backend to bench (repeatable); defaults to all backends available on this host"
    )]
    pub backend: Vec<String>,

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[argh(switch, description = "list backends available on this host and exit")]
    pub list_backends: bool,
}

impl Prepare for Bench {
    // The benchmarks run inside `prepare` so that the lockfile swap is
    // guaranteed to be reverted (via the guard's `Drop`) no matter how the
    // commands exit.
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        let backends = host_backends();
        if backends.is_empty() {
            return Vec::new();
        }

        // Only x86_64 Linux has more than one backend to choose between; on
        // every other host the backend-selection options don't exist, so just
        // bench everything available.
        let selected: Vec<&Backend> = {
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            {
                if self.list_backends {
                    for backend in &backends {
                        println!("  {}", display_name(backend));
                    }
                    return Vec::new();
                }
                if self.backend.is_empty() {
                    backends.iter().collect()
                } else {
                    self.backend
                        .iter()
                        .map(|name| {
                            resolve_backend(&backends, name).unwrap_or_else(|| {
                                let available = backends
                                    .iter()
                                    .map(display_name)
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                panic!(
                                    "unknown backend `{name}`; available backends on this host: {available}"
                                )
                            })
                        })
                        .collect()
                }
            }
            #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
            {
                backends.iter().collect()
            }
        };

        let _lock = swap_in_bench_lock();

        // The committed bench lockfile may lag behind the manifest (e.g. if a PR adds a
        // dependency). `cargo fetch` adds only the missing entries and keeps every existing pin, so
        // instruction counts should be unchanged unless the manifest change actually affects the
        // benches. The `--save` path runs unlocked instead so the regenerated lock and baselines
        // are committed together.
        if !self.save {
            sh.cmd("cargo")
                .arg("fetch")
                .run()
                .expect("failed to reconcile the bench lockfile");
        }

        let version = gungraun_version(sh, !self.save);
        let runner_bin = format!("{RUNNER_ROOT}/bin/gungraun-runner");

        let mut failure: Option<&'static str> = None;

        if !runner_installed(sh, &version) {
            eprintln!("=== install gungraun-runner {version} ===");
            let result = sh
                .cmd("cargo")
                .arg("install")
                .arg("gungraun-runner")
                .arg("--locked")
                .arg("--force")
                .arg("--version")
                .arg(format!("={version}"))
                .arg("--root")
                .arg(RUNNER_ROOT)
                .run();
            if result.is_err() {
                failure = Some("failed to install gungraun-runner");
            }
        }

        'benches: {
            if failure.is_some() {
                break 'benches;
            }
            for backend in selected {
                let (action, failure_message): (&str, &'static str) = if self.save {
                    ("save", "failed to save gungraun baselines")
                } else {
                    (
                        "check",
                        "gungraun benchmarks regressed against the saved baselines",
                    )
                };
                eprintln!("=== bench ({action} {}) ===", backend.name);

                if self.save {
                    remove_stale_baselines(Path::new(BASELINE_HOME), backend.name);
                }

                let mut cmd = sh.cmd("cargo");
                cmd = cmd
                    .env("GUNGRAUN_RUNNER", &runner_bin)
                    .arg("bench")
                    .arg("--bench")
                    .arg("gungraun");
                if let Some(features) = backend.features {
                    cmd = cmd.arg("--features").arg(features);
                }
                if !self.save {
                    cmd = cmd.arg("--locked");
                }
                cmd = cmd.arg("--").arg(format!("--home={BASELINE_HOME}"));
                cmd = if self.save {
                    cmd.arg(format!("--save-baseline={}", backend.name))
                } else {
                    cmd.arg(format!("--baseline={}", backend.name))
                        .arg("--callgrind-limits=ir=0%")
                };

                if let Err(e) = cmd.run() {
                    eprintln!("{failure_message}: {e}");
                    failure = Some(failure_message);
                    if !args.keep_going {
                        break;
                    }
                }
            }
        }

        if self.save && failure.is_none() {
            // Persist the lockfile the benchmarks were built with.
            fs::copy(workspace_path("Cargo.lock"), BENCH_LOCK)
                .expect("failed to update the bench lockfile");
        }

        match failure {
            Some(failure_message) => vec![PreparedCommand {
                name: "bench".into(),
                command: sh.cmd("false"),
                failure_message,
            }],
            None => Vec::new(),
        }
    }
}
