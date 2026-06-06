use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "wasm32-chrome")]
/// Test with wasm-pack in headless chrome
pub struct Wasm32Chrome {}

impl Prepare for Wasm32Chrome {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        vec![
            {
                let cmd = cmd!(sh, "wasm-pack test --headless --chrome")
                    .env("WASM_BINDGEN_USE_BROWSER", "1")
                    .env("RUSTFLAGS", "-Ctarget-feature=+simd128");
                PreparedCommand {
                    name: "wasm32 chrome +simd128".into(),
                    command: cmd,
                    failure_message: "wasm32 chrome (+simd128) failed",
                }
            },
            {
                let cmd = cmd!(sh, "wasm-pack test --headless --chrome")
                    .env("WASM_BINDGEN_USE_BROWSER", "1");
                PreparedCommand {
                    name: "wasm32 chrome".into(),
                    command: cmd,
                    failure_message: "wasm32 chrome failed",
                }
            },
        ]
    }
}
