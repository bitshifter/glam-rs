use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

use super::OPTIONAL_DEPS;

#[derive(FromArgs)]
#[argh(subcommand, name = "doc")]
/// Build documentation
pub struct Doc {}

impl Prepare for Doc {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let cmd = cmd!(
            sh,
            "cargo doc --all --no-deps --document-private-items --features {OPTIONAL_DEPS}"
        )
        .env("RUSTDOCFLAGS", "-Dwarnings");
        vec![PreparedCommand {
            name: "doc".into(),
            command: cmd,
            failure_message: "doc check failed",
        }]
    }
}
