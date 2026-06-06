use argh::FromArgs;
use xshell::Shell;

use crate::args::Args;
use crate::prepare::{Prepare, PreparedCommand};

use super::wasm32_chrome::Wasm32Chrome;
use super::wasm32_firefox::Wasm32Firefox;
use super::wasm32_node::Wasm32Node;

#[derive(FromArgs, Default)]
#[argh(subcommand, name = "wasm32")]
/// Test with wasm-pack in all browsers (node, firefox, chrome)
pub struct Wasm32 {}

impl Prepare for Wasm32 {
    fn prepare<'a>(&self, sh: &'a Shell, args: &Args) -> Vec<PreparedCommand<'a>> {
        let mut cmds = Vec::new();
        cmds.extend(Wasm32Node {}.prepare(sh, args));
        cmds.extend(Wasm32Firefox {}.prepare(sh, args));
        cmds.extend(Wasm32Chrome {}.prepare(sh, args));
        cmds
    }
}
