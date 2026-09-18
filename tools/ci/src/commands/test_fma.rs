use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "test-fma")]
/// Test with fused multiply-add enabled (requires an FMA capable CPU)
pub struct TestFma {}

impl Prepare for TestFma {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let test = cmd!(sh, "cargo test").env("RUSTFLAGS", "-C target-feature=+fma");
        let deprecated = cmd!(sh, "cargo check --features fast-math");

        vec![
            PreparedCommand {
                name: "test FMA".into(),
                command: test,
                failure_message: "FMA test failed",
            },
            PreparedCommand {
                name: "check deprecated fast-math feature".into(),
                command: deprecated,
                failure_message: "deprecated fast-math feature check failed",
            },
        ]
    }
}
