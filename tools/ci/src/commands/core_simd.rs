use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "core-simd")]
/// Test with experimental portable-simd (requires nightly)
pub struct CoreSimd {
    #[argh(option, description = "rust toolchain")]
    toolchain: Option<String>,
}

impl Prepare for CoreSimd {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let tc = self.toolchain.as_deref().unwrap_or(toolchain::NIGHTLY);

        let cmd = toolchain::cargo(sh, tc)
            .arg("test")
            .arg("--features")
            .arg("core-simd");

        vec![PreparedCommand {
            name: "core-simd".into(),
            command: cmd,
            failure_message: "core-simd test failed",
        }]
    }
}
