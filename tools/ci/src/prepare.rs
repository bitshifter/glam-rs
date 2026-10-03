use xshell::{Cmd, Shell, cmd};

use crate::args::Args;

pub trait Prepare {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>>;
}

pub struct PreparedCommand<'a> {
    pub name: String,
    pub command: Cmd<'a>,
    pub failure_message: &'static str,
}

/// Build `cargo test` commands for the given feature sets. `index` is the
/// optional 1-based position used in the command labels.
pub(crate) fn test_feature_commands<'a>(
    sh: &'a Shell,
    sets: &[&str],
    index: Option<usize>,
) -> Vec<PreparedCommand<'a>> {
    let total = sets.len();
    sets.iter()
        .enumerate()
        .map(|(i, features)| PreparedCommand {
            name: format!("test [{}/{}]: {features}", index.unwrap_or(i + 1), total),
            command: cmd!(sh, "cargo test --no-default-features --features {features}"),
            failure_message: "test feature set failed",
        })
        .collect()
}
