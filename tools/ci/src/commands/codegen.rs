use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs)]
#[argh(subcommand, name = "codegen")]
/// Check that codegen output matches committed source
pub struct Codegen {}

impl Prepare for Codegen {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let cmd = cmd!(sh, "cargo run --release -p codegen -- --check **");
        vec![PreparedCommand {
            name: "codegen".into(),
            command: cmd,
            failure_message: "codegen check failed",
        }]
    }
}
