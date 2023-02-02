use crate::Error;
use std::{path::PathBuf, process::Command};

pub fn compile_llvm_ir(in_file_path: &PathBuf, out_file_name: &str) -> Result<(), Error> {
    let out_arg = format!("-o={out_file_name}");

    let out = Command::new("llc")
        .args([
            "-filetype=obj",
            out_arg.as_str(),
            in_file_path.to_str().unwrap(),
        ])
        .output()?;
    if out.status.success() {
        Ok(())
    } else {
        Err(Error::LlcError(
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap(),
        ))
    }
}
