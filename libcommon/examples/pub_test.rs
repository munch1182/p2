use std::process::Command;

use libcommon::{ext::PrettyStringExt, log::log_setup};
use log::info;

pub fn main() {
    log_setup();
    let mut cmd = Command::new("cargo");
    cmd.arg("-v");
    let op = cmd.output().and_then(|o| Ok(o.status.success()));
    info!("cmd : {}: {op:?}", &cmd.to_string_pretty())
}
