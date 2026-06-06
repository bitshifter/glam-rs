use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs)]
#[argh(subcommand, name = "fmt")]
/// Check code formatting with rustfmt
pub struct Fmt {}

impl Prepare for Fmt {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let cmd = cmd!(sh, "cargo fmt -- --check --color always");
        vec![PreparedCommand {
            name: "rustfmt".into(),
            command: cmd,
            failure_message: "rustfmt check failed",
        }]
    }
}
