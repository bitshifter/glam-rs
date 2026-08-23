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
        let mut cmds = Vec::new();
        for backend in host_backends() {
            let mut cmd = sh.cmd("cargo");
            cmd = cmd.arg("bench").arg("--bench").arg("gungraun");
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
