use crate::Error;
use std::{path::Path, process::Command};

pub fn compile_llvm_ir(in_file_path: &Path, out_file_path: &Path) -> Result<(), Error> {
    let out_arg = format!("-o={}", out_file_path.to_str().unwrap());

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
