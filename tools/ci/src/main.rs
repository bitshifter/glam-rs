mod args;
mod commands;
mod prepare;
mod toolchain;

use argh::FromArgs;
use xshell::Shell;

use args::Args;
use commands::check_features::CheckFeatures;
use commands::ci::Ci;
use commands::clippy::Clippy;
use commands::codegen::Codegen;
use commands::core_simd::CoreSimd;
use commands::doc::Doc;
use commands::fmt::Fmt;
use commands::lints::Lints;
use commands::msrv::Msrv;
use commands::pre_push::PrePush;
use commands::test_features::TestFeatures;
use commands::wasm32::Wasm32;
use commands::wasm32_chrome::Wasm32Chrome;
use commands::wasm32_firefox::Wasm32Firefox;
use commands::wasm32_node::Wasm32Node;
use commands::wasm64::Wasm64;
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
    Ci(Ci),
    Clippy(Clippy),
    Codegen(Codegen),
    CoreSimd(CoreSimd),
    Doc(Doc),
    Fmt(Fmt),
    Lints(Lints),
    Msrv(Msrv),
    PrePush(PrePush),
    TestFeatures(TestFeatures),
    Wasm32(Wasm32),
    Wasm32Chrome(Wasm32Chrome),
    Wasm32Firefox(Wasm32Firefox),
    Wasm32Node(Wasm32Node),
    Wasm64(Wasm64),
}

impl Prepare for Subcommand {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        match self {
            Subcommand::CheckFeatures(cmd) => cmd.prepare(sh, args),
            Subcommand::Ci(cmd) => cmd.prepare(sh, args),
            Subcommand::Clippy(cmd) => cmd.prepare(sh, args),
            Subcommand::Codegen(cmd) => cmd.prepare(sh, args),
            Subcommand::CoreSimd(cmd) => cmd.prepare(sh, args),
            Subcommand::Doc(cmd) => cmd.prepare(sh, args),
            Subcommand::Fmt(cmd) => cmd.prepare(sh, args),
            Subcommand::Lints(cmd) => cmd.prepare(sh, args),
            Subcommand::Msrv(cmd) => cmd.prepare(sh, args),
            Subcommand::PrePush(cmd) => cmd.prepare(sh, args),
            Subcommand::TestFeatures(cmd) => cmd.prepare(sh, args),
            Subcommand::Wasm32(cmd) => cmd.prepare(sh, args),
            Subcommand::Wasm32Chrome(cmd) => cmd.prepare(sh, args),
            Subcommand::Wasm32Firefox(cmd) => cmd.prepare(sh, args),
            Subcommand::Wasm32Node(cmd) => cmd.prepare(sh, args),
            Subcommand::Wasm64(cmd) => cmd.prepare(sh, args),
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
        None => PrePush::default().prepare(&sh, &args),
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
