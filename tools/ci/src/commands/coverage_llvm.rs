use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "coverage-llvm")]
/// Generate code coverage via cargo-llvm-cov for all math profiles
pub struct CoverageLlvm {
    #[argh(
        option,
        short = 'p',
        description = "profile: sse2_math, scalar_math, or core_simd"
    )]
    profile: Option<String>,

    #[argh(switch, description = "output in LCOV format")]
    lcov: bool,

    #[argh(
        option,
        short = 'o',
        description = "LCOV output path (default: ./coverage/lcov.info)"
    )]
    output_path: Option<String>,
}

impl Prepare for CoverageLlvm {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();

        let profiles: &[(&str, &str, &str)] = &[
            (
                "sse2_math",
                "arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy zerocopy debug-glam-assert",
                "stable",
            ),
            (
                "scalar_math",
                "scalar-math arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy zerocopy debug-glam-assert",
                "stable",
            ),
            (
                "core_simd",
                "core-simd arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy debug-glam-assert",
                toolchain::NIGHTLY,
            ),
        ];

        for &(name, features, tc) in profiles {
            if let Some(ref filter) = self.profile {
                if name != filter.as_str() {
                    continue;
                }
            }

            let mut cmd = toolchain::cargo(sh, tc)
                .arg("llvm-cov")
                .arg("--features")
                .arg(features);

            if self.lcov {
                let path = self
                    .output_path
                    .clone()
                    .unwrap_or_else(|| "./coverage/lcov.info".to_string());
                cmd = cmd.arg("--lcov").arg("--output-path").arg(path);
            }

            cmds.push(PreparedCommand {
                name: format!("coverage: {name}"),
                command: cmd,
                failure_message: "coverage generation failed",
            });
        }

        if cmds.is_empty() {
            panic!("unknown profile: {}", self.profile.as_deref().unwrap_or(""));
        }

        cmds
    }
}
