use xshell::{Cmd, Shell};

pub const MSRV: &str = "1.68.2";
pub const NIGHTLY: &str = "nightly";

pub fn cargo<'a>(sh: &'a Shell, toolchain: &str) -> Cmd<'a> {
    sh.cmd("rustup").arg("run").arg(toolchain).arg("cargo")
}
