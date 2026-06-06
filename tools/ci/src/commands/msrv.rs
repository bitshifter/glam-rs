use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};
use crate::toolchain;

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
            toolchain::cargo(sh, tc)
                .arg("check")
                .arg("--features")
                .arg(features)
        };

        vec![
            PreparedCommand {
                name: format!("msrv check: {}", super::MSRV_FEATURES),
                command: a(super::MSRV_FEATURES),
                failure_message: "msrv check failed",
            },
            PreparedCommand {
                name: format!("msrv check: scalar-math {}", super::MSRV_FEATURES),
                command: a(&format!("scalar-math {}", super::MSRV_FEATURES)),
                failure_message: "msrv check (scalar-math) failed",
            },
            PreparedCommand {
                name: "msrv check: libm scalar-math (no-default-features)".into(),
                command: toolchain::cargo(sh, tc)
                    .arg("check")
                    .arg("--no-default-features")
                    .arg("--features")
                    .arg(format!("libm scalar-math {}", super::MSRV_FEATURES)),
                failure_message: "msrv check (libm, no_std) failed",
            },
        ]
    }
}
