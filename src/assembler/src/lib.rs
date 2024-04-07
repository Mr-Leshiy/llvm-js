#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

use std::{
    env::current_exe,
    path::Path,
    process::{Command, ExitStatus},
};

pub type AssemblerError = Error;

const CORE_LIB: &str = "core";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Command error: {0}")]
    CommandError(#[from] std::io::Error),
    #[error("clang error, status code: {0}, stdout: {1}, stderr: {2}")]
    ClangError(ExitStatus, String, String),
}

pub fn compile_binary(in_file_path: &Path, out_file_path: &Path) -> Result<(), Error> {
    let out_arg = format!("-o{}", out_file_path.to_str().unwrap());
    let lib_dir_arg = format!(
        "-L{}",
        // path to the binary
        current_exe().unwrap().parent().unwrap().to_str().unwrap()
    );
    let llvm_lib_name_arg = format!("-l{CORE_LIB}");

    let out = Command::new("clang++")
        .args([
            lib_dir_arg.as_str(),
            out_arg.as_str(),
            in_file_path.to_str().unwrap(),
            llvm_lib_name_arg.as_str(),
        ])
        .output()?;
    if out.status.success() {
        Ok(())
    } else {
        Err(Error::ClangError(
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap(),
        ))
    }
}
