use argh::FromArgs;
use xshell::{Shell, cmd};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand, test_feature_commands};

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "test-features")]
/// Build and test feature combinations. Without --index, tests all sets.
pub struct TestFeatures {
    #[argh(option, description = "test a single feature set by 1-based index")]
    index: Option<usize>,

    #[argh(switch, description = "list available feature sets and exit")]
    list: bool,
}

impl Prepare for TestFeatures {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        if self.list {
            crate::features::print_feature_sets();
            return Vec::new();
        }

        let mut cmds =
            test_feature_commands(sh, crate::features::resolve_sets(self.index), self.index);

        let cmd = cmd!(sh, "cargo check").env("RUSTFLAGS", "-C target-feature=+fma");
        cmds.push(PreparedCommand {
            name: "check FMA".into(),
            command: cmd,
            failure_message: "FMA check failed",
        });

        let cmd = cmd!(sh, "cargo check -p glam-no_std");
        cmds.push(PreparedCommand {
            name: "check glam-no_std".into(),
            command: cmd,
            failure_message: "no_std check failed",
        });

        cmds
    }
}
