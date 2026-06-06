mod args;
mod commands;
mod prepare;

use argh::FromArgs;
use xshell::Shell;

use args::Args;
use commands::check_features::CheckFeatures;
use commands::clippy::Clippy;
use commands::codegen::Codegen;
use commands::doc::Doc;
use commands::fmt::Fmt;
use commands::lints::Lints;
use commands::test_features::TestFeatures;
use prepare::{Prepare, PreparedCommand};

#[derive(FromArgs)]
/// CI tool for glam-rs.
struct Cli {
    #[argh(switch, description = "continue on failure")]
    keep_going: bool,

    #[argh(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    CheckFeatures(CheckFeatures),
    Clippy(Clippy),
    Codegen(Codegen),
    Doc(Doc),
    Fmt(Fmt),
    Lints(Lints),
    TestFeatures(TestFeatures),
}

impl Prepare for Subcommand {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        match self {
            Subcommand::CheckFeatures(cmd) => cmd.prepare(sh, args),
            Subcommand::Clippy(cmd) => cmd.prepare(sh, args),
            Subcommand::Codegen(cmd) => cmd.prepare(sh, args),
            Subcommand::Doc(cmd) => cmd.prepare(sh, args),
            Subcommand::Fmt(cmd) => cmd.prepare(sh, args),
            Subcommand::Lints(cmd) => cmd.prepare(sh, args),
            Subcommand::TestFeatures(cmd) => cmd.prepare(sh, args),
        }
    }
}

fn main() {
    let cli: Cli = argh::from_env();
    let sh = Shell::new().unwrap();
    let args = Args {
        keep_going: cli.keep_going,
    };

    let cmds: Vec<PreparedCommand> = match cli.subcommand {
        Some(ref subcommand) => subcommand.prepare(&sh, &args),
        None => {
            let mut all = Vec::new();
            all.extend(TestFeatures::default().prepare(&sh, &args));
            all.extend(Lints {}.prepare(&sh, &args));
            all
        }
    };

    let mut failures: Vec<(&str, xshell::Error)> = Vec::new();
    for cmd in &cmds {
        eprintln!("=== {} ===", cmd.name);
        match cmd.command.run() {
            Ok(()) => {}
            Err(e) => {
                if args.keep_going {
                    failures.push((cmd.failure_message, e));
                } else {
                    panic!("{}: {e}", cmd.failure_message);
                }
            }
        }
    }

    if !failures.is_empty() {
        let msg = failures
            .iter()
            .map(|(msg, e)| format!("{msg}: {e}"))
            .collect::<Vec<_>>()
            .join("\n");
        panic!("{msg}");
    }
}
