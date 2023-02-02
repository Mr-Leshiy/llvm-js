use crate::{Error, CORE_LIB, FMT_LIB};
use std::{env::current_exe, path::PathBuf, process::Command};

pub fn compile_binary(in_file_path: &PathBuf, out_file_path: &PathBuf) -> Result<(), Error> {
    let out_arg = format!("-o{}", out_file_path.to_str().unwrap());
    let lib_dir_arg = format!(
        "-L{}",
        // path to the binary
        current_exe().unwrap().parent().unwrap().to_str().unwrap()
    );
    let llvm_lib_name_arg = format!("-l{CORE_LIB}");
    let fmt_lib_name_arg = format!("-l{FMT_LIB}");

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
