use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

const CLIPPY_FEATURES: &str =
    "std,debug-glam-assert,approx,bytemuck,mint,rand,serde,rkyv,speedy,encase,zerocopy,arbitrary";

#[derive(FromArgs)]
#[argh(subcommand, name = "clippy")]
/// Lint with clippy
pub struct Clippy {}

impl Prepare for Clippy {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let cmd = cmd!(
            sh,
            "cargo clippy --workspace --all-targets --features {CLIPPY_FEATURES} -- -D warnings"
        );
        vec![PreparedCommand {
            name: "clippy".into(),
            command: cmd,
            failure_message: "clippy check failed",
        }]
    }
}
