use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand, test_feature_commands};

use super::clippy::Clippy;
use super::codegen::Codegen;
use super::fmt::Fmt;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "pre-push")]
/// Minimal checks to run locally before pushing: fmt, clippy, codegen and a
/// reduced feature set. The full suite runs on CI.
pub struct PrePush {}

impl Prepare for PrePush {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();
        cmds.extend(Fmt {}.prepare(sh, args));
        cmds.extend(Clippy {}.prepare(sh, args));
        cmds.extend(Codegen {}.prepare(sh, args));
        cmds.extend(test_feature_commands(
            sh,
            crate::features::PRE_PUSH_FEATURE_SETS,
            None,
        ));
        cmds
    }
}
