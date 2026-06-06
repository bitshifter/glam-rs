use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "wasm32-firefox")]
/// Test with wasm-pack in headless firefox
pub struct Wasm32Firefox {}

impl Prepare for Wasm32Firefox {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        vec![
            {
                let cmd = cmd!(sh, "wasm-pack test --headless --firefox")
                    .env("WASM_BINDGEN_USE_BROWSER", "1")
                    .env("RUSTFLAGS", "-Ctarget-feature=+simd128");
                PreparedCommand {
                    name: "wasm32 firefox +simd128".into(),
                    command: cmd,
                    failure_message: "wasm32 firefox (+simd128) failed",
                }
            },
            {
                let cmd = cmd!(sh, "wasm-pack test --headless --firefox")
                    .env("WASM_BINDGEN_USE_BROWSER", "1");
                PreparedCommand {
                    name: "wasm32 firefox".into(),
                    command: cmd,
                    failure_message: "wasm32 firefox failed",
                }
            },
        ]
    }
}
