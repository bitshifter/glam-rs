use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

const DOC_FEATURES: &str = "std,debug-glam-assert,approx,bytemuck,mint,rand,serde,rkyv,speedy,rkyv/pointer_width_32,encase,zerocopy,arbitrary";

#[derive(FromArgs)]
#[argh(subcommand, name = "doc")]
/// Build documentation
pub struct Doc {}

impl Prepare for Doc {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let cmd = cmd!(
            sh,
            "cargo doc --all --no-deps --document-private-items --features {DOC_FEATURES}"
        )
        .env("RUSTDOCFLAGS", "-Dwarnings");
        vec![PreparedCommand {
            name: "doc".into(),
            command: cmd,
            failure_message: "doc check failed",
        }]
    }
}
