use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "coverage-tarpaulin")]
/// Generate code coverage via cargo-tarpaulin for all math profiles (requires nightly)
pub struct CoverageTarpaulin {
    #[argh(
        option,
        short = 'p',
        description = "profile: sse2_math, scalar_math, or core_simd"
    )]
    profile: Option<String>,

    #[argh(option, default = "120", description = "test timeout in seconds")]
    timeout: u64,

    #[argh(
        option,
        default = "String::from(\"lcov\")",
        description = "output format"
    )]
    out: String,

    #[argh(
        option,
        default = "String::from(\"./coverage\")",
        description = "output directory"
    )]
    output_dir: String,
}

impl Prepare for CoverageTarpaulin {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();
        let profile_names: &[&str] = &["sse2_math", "scalar_math", "core_simd"];

        for &name in profile_names {
            if let Some(ref filter) = self.profile {
                if name != filter.as_str() {
                    continue;
                }
            }

            let cmd = toolchain::cargo(sh, "nightly")
                .arg("tarpaulin")
                .arg("--config")
                .arg(name)
                .arg("--timeout")
                .arg(self.timeout.to_string())
                .arg("--out")
                .arg(&self.out)
                .arg("--output-dir")
                .arg(&self.output_dir);

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
