#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

mod compiling_tests;
#[cfg(target_os = "linux")]
#[cfg(feature = "mem-check")]
mod memory_tests;

use std::{env::current_dir, fs::remove_file, process::Command};

pub struct CompileSuite {
    source_code_path: String,
    llvm_ir_out_file: String,
    object_out_file: String,
    binary_out_file: String,
}

impl CompileSuite {
    pub fn new(source_code_path: &'static str, test_name: &'static str) -> Self {
        let llvm_ir_out_file = format!("{test_name}.ll");
        let object_out_file = format!("{test_name}.o");
        let binary_out_file = format!("{test_name}_run");
        Self {
            source_code_path: source_code_path.to_string(),
            llvm_ir_out_file,
            object_out_file,
            binary_out_file,
        }
    }

    pub fn compile(self) -> Result<Self, String> {
        compile_js(
            self.source_code_path.as_str(),
            self.llvm_ir_out_file.as_str(),
        )?;
        compile_llvm_ir(
            self.llvm_ir_out_file.as_str(),
            self.object_out_file.as_str(),
        )?;
        compile_binary(self.object_out_file.as_str(), self.binary_out_file.as_str())?;
        Ok(self)
    }

    pub fn run(self) -> Result<Self, String> {
        run_binary(self.binary_out_file.as_str())?;
        Ok(self)
    }

    #[cfg(target_os = "linux")]
    pub fn run_with_valgrind(self) -> Result<Self, String> {
        run_binary_with_valgrind(self.binary_out_file.as_str())?;
        Ok(self)
    }

    pub fn cleanup(&self) {
        remove_file(self.llvm_ir_out_file.clone()).unwrap();
        remove_file(self.object_out_file.clone()).unwrap();
        remove_file(self.binary_out_file.clone()).unwrap();
    }
}

fn compile_js(in_file_path: &str, out_file_path: &str) -> Result<(), String> {
    let out = Command::new("../target/debug/llvm-js-compiler")
        .args([
            format!("--input={in_file_path}"),
            format!("--output={out_file_path}"),
        ])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "status code: {} \n, stdout: {} \n, stderr: {}",
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap()
        ))
    }
}

fn compile_llvm_ir(in_file_path: &str, out_file_name: &str) -> Result<(), String> {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{in_file_path}", cur_dir.to_str().unwrap());
    let out_arg = format!("-o={out_file_name}",);

    let out = Command::new("llc")
        .args(["-filetype=obj", out_arg.as_str(), in_arg.as_str()])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "status code: {} \n, stdout: {} \n, stderr: {}",
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap()
        ))
    }
}

fn compile_binary(in_file_path: &str, out_file_name: &str) -> Result<(), String> {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{in_file_path}", cur_dir.to_str().unwrap());
    let out_arg = format!("-o{out_file_name}");
    let lib_dir_arg = "-L../build/lib/".to_string();
    let llvm_lib_name_arg = "-lllvm-js".to_string();
    let fmt_lib_name_arg = "-lfmt".to_string();

    let out = Command::new("clang++")
        .args([
            lib_dir_arg.as_str(),
            out_arg.as_str(),
            in_arg.as_str(),
            llvm_lib_name_arg.as_str(),
            fmt_lib_name_arg.as_str(),
        ])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "status code: {} \n, stdout: {} \n, stderr: {}",
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap()
        ))
    }
}

fn run_binary(in_file_path: &str) -> Result<(), String> {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{in_file_path}", cur_dir.to_str().unwrap());

    let out = Command::new(in_arg).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "status code: {} \n, stdout: {} \n, stderr: {}",
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap()
        ))
    }
}

#[cfg(target_os = "linux")]
fn run_binary_with_valgrind(in_file_path: &str) -> Result<(), String> {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{in_file_path}", cur_dir.to_str().unwrap());

    let out = Command::new("valgrind")
        .args(["--leak-check=full", "--error-exitcode=1", in_arg.as_str()])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "status code: {} \n, stdout: {} \n, stderr: {}",
            out.status,
            String::from_utf8(out.stdout).unwrap(),
            String::from_utf8(out.stderr).unwrap()
        ))
    }
}
