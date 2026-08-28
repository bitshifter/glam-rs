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

        // core-simd has to keep working without std, where std::simd::StdFloat isn't
        // available and the libm backend is used instead.
        let nostd_cmd = toolchain::cargo(sh, tc)
            .arg("test")
            .arg("--no-default-features")
            .arg("--features")
            .arg("core-simd,libm,all-types");

        // libm "prefers it over std", so it must also build with std enabled.
        let libm_cmd = toolchain::cargo(sh, tc)
            .arg("test")
            .arg("--features")
            .arg("core-simd,libm");

        vec![
            PreparedCommand {
                name: "core-simd".into(),
                command: cmd,
                failure_message: "core-simd test failed",
            },
            PreparedCommand {
                name: "core-simd-nostd".into(),
                command: nostd_cmd,
                failure_message: "core-simd no-std test failed",
            },
            PreparedCommand {
                name: "core-simd-libm".into(),
                command: libm_cmd,
                failure_message: "core-simd libm test failed",
            },
        ]
    }
}
