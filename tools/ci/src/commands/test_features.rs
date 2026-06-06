use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

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
            super::print_feature_sets();
            return Vec::new();
        }

        let mut cmds = Vec::new();

        let sets = super::resolve_sets(self.index);
        let total = super::FEATURE_SETS.len();

        for (i, features) in sets.iter().enumerate() {
            let idx = self.index.unwrap_or(i + 1);
            let cmd = cmd!(sh, "cargo test --no-default-features --features {features}");
            cmds.push(PreparedCommand {
                name: format!("test [{idx}/{total}]: {features}"),
                command: cmd,
                failure_message: "test feature set failed",
            });
        }

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
