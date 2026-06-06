use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

#[derive(FromArgs)]
#[argh(subcommand, name = "test-features")]
/// Build and test all feature combinations
pub struct TestFeatures {}

impl Prepare for TestFeatures {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();

        let sets = super::feature_sets();
        for (i, features) in sets.iter().enumerate() {
            let cmd = cmd!(sh, "cargo test --no-default-features --features {features}");
            cmds.push(PreparedCommand {
                name: format!("test features [{}/{}]", i + 1, sets.len()),
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
