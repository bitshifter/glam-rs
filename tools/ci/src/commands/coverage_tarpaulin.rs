use std::path::Path;

use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

const EXCLUDE_FILES: &[&str] = &[
    "tools/*",
    "src/neon.rs",
    "src/bool/neon/*",
    "src/f32/neon/*",
    "src/swizzles/neon/*",
    "src/wasm.rs",
    "src/bool/wasm/*",
    "src/f32/wasm/*",
    "src/swizzles/wasm/*",
    "benches/*",
    "tests/support.rs",
];

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "coverage-tarpaulin")]
/// Generate code coverage via cargo-tarpaulin for all math profiles (requires nightly)
pub struct CoverageTarpaulin {
    #[argh(
        option,
        short = 'p',
        description = "profile: native_simd, scalar_math, or core_simd"
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

        let scalar_features = format!("scalar-math {}", crate::features::ALL_FEATURES);

        let profiles: &[(&str, &str)] = &[
            ("native_simd", crate::features::ALL_FEATURES),
            ("scalar_math", &scalar_features),
            ("core_simd", crate::features::CORE_SIMD_FEATURES),
        ];

        for &(name, features) in profiles {
            if let Some(ref filter) = self.profile {
                if name != filter.as_str() {
                    continue;
                }
            }

            let mut cmd = toolchain::cargo(sh, toolchain::NIGHTLY)
                .arg("tarpaulin")
                .arg("--features")
                .arg(features);
            for &pat in EXCLUDE_FILES {
                cmd = cmd.arg("--exclude-files").arg(pat);
            }
            cmd = cmd
                .arg("--timeout")
                .arg(self.timeout.to_string())
                .arg("--out")
                .arg(&self.out)
                .arg("--output-dir")
                .arg(&self.output_dir);

            if let Some(parent) = Path::new(&self.output_dir).parent() {
                std::fs::create_dir_all(parent).unwrap();
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
