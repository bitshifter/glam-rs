use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

use super::OPTIONAL_DEPS;

#[derive(FromArgs)]
#[argh(subcommand, name = "clippy")]
/// Lint with clippy
pub struct Clippy {}

impl Prepare for Clippy {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let cmd = cmd!(
            sh,
            "cargo clippy --workspace --all-targets --features {OPTIONAL_DEPS} -- -D warnings"
        );
        vec![PreparedCommand {
            name: "clippy".into(),
            command: cmd,
            failure_message: "clippy check failed",
        }]
    }
}
