use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

const FEATURES: &str = "all-types arbitrary approx mint speedy debug-glam-assert";

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "msrv")]
/// Check compilation with MSRV toolchain
pub struct Msrv {
    #[argh(option, description = "rust toolchain")]
    toolchain: Option<String>,
}

impl Prepare for Msrv {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let tc = self.toolchain.as_deref().unwrap_or(toolchain::MSRV);

        let a = |features: &str| {
            let cmd = toolchain::cargo(sh, tc)
                .arg("check")
                .arg("--features")
                .arg(features);
            cmd
        };

        vec![
            PreparedCommand {
                name: format!("msrv check: {FEATURES}"),
                command: a(FEATURES),
                failure_message: "msrv check failed",
            },
            PreparedCommand {
                name: format!("msrv check: scalar-math {FEATURES}"),
                command: a(&format!("scalar-math {FEATURES}")),
                failure_message: "msrv check (scalar-math) failed",
            },
            PreparedCommand {
                name: "msrv check: libm scalar-math (no-default-features)".into(),
                command: toolchain::cargo(sh, tc)
                    .arg("check")
                    .arg("--no-default-features")
                    .arg("--features")
                    .arg(format!("libm scalar-math {FEATURES}")),
                failure_message: "msrv check (libm, no_std) failed",
            },
        ]
    }
}
