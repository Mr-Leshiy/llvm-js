use crate::Error;
use std::{path::PathBuf, process::Command};

pub fn compile_binary(in_file_path: &PathBuf, out_file_name: &str) -> Result<(), Error> {
    let out_arg = format!("-o{out_file_name}");
    let lib_dir_arg = "-L../build/lib/".to_string();
    let llvm_lib_name_arg = "-lllvm-js".to_string();
    let fmt_lib_name_arg = "-lfmt".to_string();

    let out = Command::new("clang++")
        .args([
            lib_dir_arg.as_str(),
            out_arg.as_str(),
            in_file_path.to_str().unwrap(),
            llvm_lib_name_arg.as_str(),
            fmt_lib_name_arg.as_str(),
        ])
        .output()?;
    if out.status.success() {
        Ok(())
    } else {
        Err(Error::LinkerError(
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap(),
        ))
    }
}
