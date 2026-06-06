use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "wasm32-node")]
/// Test with wasm-pack in node
pub struct Wasm32Node {}

impl Prepare for Wasm32Node {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        vec![
            {
                let cmd =
                    cmd!(sh, "wasm-pack test --node").env("RUSTFLAGS", "-Ctarget-feature=+simd128");
                PreparedCommand {
                    name: "wasm32 node +simd128".into(),
                    command: cmd,
                    failure_message: "wasm32 node (+simd128) failed",
                }
            },
            {
                let cmd = cmd!(sh, "wasm-pack test --node");
                PreparedCommand {
                    name: "wasm32 node".into(),
                    command: cmd,
                    failure_message: "wasm32 node failed",
                }
            },
        ]
    }
}
