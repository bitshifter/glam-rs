use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "wasm64")]
/// Check compilation with wasm64 target (requires nightly + wasmtime)
pub struct Wasm64 {
    #[argh(option, description = "rust toolchain")]
    toolchain: Option<String>,
}

impl Prepare for Wasm64 {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let tc = self.toolchain.as_deref().unwrap_or(toolchain::NIGHTLY);

        let runner = "wasmtime --wasm memory64";

        let base_cmd = || {
            toolchain::cargo(sh, tc)
                .arg("check")
                .arg("--target")
                .arg("wasm64-unknown-unknown")
                .arg("--no-default-features")
                .arg("--features")
                .arg("libm,all-types")
                .arg("-Zbuild-std=std,panic_abort")
                .arg("-Zpanic-abort-tests")
                .env("CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUNNER", runner)
        };

        vec![
            {
                let cmd = base_cmd().env("RUSTFLAGS", "-Ctarget-feature=+simd128");
                PreparedCommand {
                    name: "wasm64 +simd128".into(),
                    command: cmd,
                    failure_message: "wasm64 check (+simd128) failed",
                }
            },
            {
                let cmd = base_cmd();
                PreparedCommand {
                    name: "wasm64".into(),
                    command: cmd,
                    failure_message: "wasm64 check failed",
                }
            },
        ]
    }
}
