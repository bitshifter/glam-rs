use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

use super::clippy::Clippy;
use super::codegen::Codegen;
use super::doc::Doc;
use super::fmt::Fmt;

#[derive(FromArgs)]
#[argh(subcommand, name = "lints")]
/// Run all linting checks (fmt, clippy, codegen, doc)
pub struct Lints {}

impl Prepare for Lints {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();
        cmds.extend(Fmt {}.prepare(sh, args));
        cmds.extend(Clippy {}.prepare(sh, args));
        cmds.extend(Codegen {}.prepare(sh, args));
        cmds.extend(Doc {}.prepare(sh, args));
        cmds
    }
}
