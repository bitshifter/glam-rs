use xshell::{Cmd, Shell};

use crate::args::Args;

pub trait Prepare {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>>;
}

pub struct PreparedCommand<'a> {
    pub name: String,
    pub command: Cmd<'a>,
    pub failure_message: &'static str,
}
