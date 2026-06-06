use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

use super::wasm32_node::Wasm32Node;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "wasm32")]
/// Test with wasm-pack in node
pub struct Wasm32 {}

impl Prepare for Wasm32 {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        Wasm32Node {}.prepare(sh, args)
    }
}
