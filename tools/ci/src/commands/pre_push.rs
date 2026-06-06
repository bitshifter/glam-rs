use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

use super::lints::Lints;
use super::test_features::TestFeatures;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "pre-push")]
/// Run tests and lints locally before pushing (no special toolchains required)
pub struct PrePush {}

impl Prepare for PrePush {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();
        cmds.extend(Lints {}.prepare(sh, args));
        cmds.extend(TestFeatures::default().prepare(sh, args));
        cmds
    }
}
