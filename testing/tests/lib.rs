#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

mod compiling_tests;

use std::{env::current_dir, fs::remove_file, process::Command};

fn run_test(source_code_path: &str, test_name: &str) {
    let test = CompileSuite::new(source_code_path, test_name);
    let cleanup = |e| {
        test.cleanup();
        panic!("error: {e}");
    };
    test.compile().unwrap_or_else(cleanup);
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap_or_else(cleanup);
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap_or_else(cleanup);
    test.cleanup();
}

pub struct CompileSuite {
    source_code_path: String,
    binary_out_file: String,
}

impl CompileSuite {
    pub fn new(source_code_path: &str, test_name: &str) -> Self {
        let binary_out_file = format!("{test_name}_run");
        Self {
            source_code_path: source_code_path.to_string(),
            binary_out_file,
        }
    }

    pub fn compile(&self) -> Result<(), String> {
        let out = Command::new("../target/debug/jsc")
            .args([
                format!("--input={}", &self.source_code_path),
                format!("--binary-name={}", &self.binary_out_file),
                "--clean".to_string(),
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

    pub fn run(&self) -> Result<(), String> {
        let cur_dir = current_dir().unwrap();

        let in_arg = format!("{}/{}", cur_dir.to_str().unwrap(), &self.binary_out_file);

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

    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    pub fn run_with_valgrind(&self) -> Result<(), String> {
        let cur_dir = current_dir().unwrap();

        let in_arg = format!("{}/{}", cur_dir.to_str().unwrap(), &self.binary_out_file);

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

    pub fn cleanup(&self) {
        remove_file(self.binary_out_file.clone()).unwrap();
    }
}
