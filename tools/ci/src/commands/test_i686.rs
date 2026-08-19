use argh::FromArgs;
use xshell::{cmd, Shell};

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

/// Test feature combinations on the 32-bit i686 target to catch layout
/// differences that only exist on 32-bit targets, e.g. usize/isize sizing
/// and i64/u64 alignment. Without --index, tests all sets.
#[derive(FromArgs, Default)]
#[argh(subcommand, name = "test-i686")]
/// Build and test feature combinations on the 32-bit i686 target.
pub struct TestI686 {
    #[argh(option, description = "test a single feature set by 1-based index")]
    index: Option<usize>,

    #[argh(switch, description = "list available feature sets and exit")]
    list: bool,
}

impl Prepare for TestI686 {
    fn prepare<'a>(&self, sh: &'a Shell, _args: &Args) -> Vec<PreparedCommand<'a>> {
        if self.list {
            crate::features::print_feature_sets();
            return Vec::new();
        }

        let mut cmds = Vec::new();

        let sets = crate::features::resolve_sets(self.index);
        let total = crate::features::FEATURE_SETS.len();

        // Benchmarks are skipped with `--tests` because criterion and
        // gungraun are only relevant on the host architecture and add
        // significant cross-compilation time on the 32-bit target.
        for (i, features) in sets.iter().enumerate() {
            let idx = self.index.unwrap_or(i + 1);
            let cmd = cmd!(
                sh,
                "cargo test --tests --no-default-features --features {features} --target i686-unknown-linux-gnu"
            );
            cmds.push(PreparedCommand {
                name: format!("test i686 [{idx}/{total}]: {features}"),
                command: cmd,
                failure_message: "i686 test feature set failed",
            });
        }

        cmds
    }
}
