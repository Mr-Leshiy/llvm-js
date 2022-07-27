use ast::js_ast::Module;
use compiler::predefined_functions::{
    abort::AbortFn, assert::AssertFn, assert_eq::AssertEqFn, printf::PrintFn, PredefineFunctionName,
};
use std::{
    env::current_dir,
    fs::{remove_file, File},
    path::Path,
    process::Command,
};

pub struct CompileSuite {
    source_code_path: &'static str,
    module_name: String,
    llvm_ir_out_file: String,
    object_out_file: String,
    binary_out_file: String,
}

impl CompileSuite {
    pub fn new(source_code_path: &'static str, test_name: &'static str) -> Self {
        let module_name = format!("module_{}", test_name);
        let llvm_ir_out_file = format!("{}.ll", test_name);
        let object_out_file = format!("{}.o", test_name);
        let binary_out_file = format!("{}_run", test_name);
        Self {
            source_code_path,
            module_name,
            llvm_ir_out_file,
            object_out_file,
            binary_out_file,
        }
    }

    pub fn compile(self) -> Self {
        compile_js(
            self.source_code_path,
            self.llvm_ir_out_file.clone(),
            self.module_name.clone(),
        );
        compile_llvm_ir(self.llvm_ir_out_file.clone(), self.object_out_file.clone());
        compile_binary(self.object_out_file.clone(), self.binary_out_file.clone());
        self
    }

    pub fn run(self) -> Result<Self, String> {
        if run_binary(self.binary_out_file.clone()) {
            Ok(self)
        } else {
            // TODO provide info
            Err("run failuire".to_string())
        }
    }

    pub fn cleanup(&self) {
        remove_file(self.llvm_ir_out_file.clone()).unwrap();
        remove_file(self.object_out_file.clone()).unwrap();
        remove_file(self.binary_out_file.clone()).unwrap();
    }
}

fn compile_js<P1: AsRef<Path>, P2: AsRef<Path>>(
    in_file_path: P1,
    out_file_path: P2,
    module_name: String,
) {
    let in_file = File::open(in_file_path).unwrap();
    let mut out_file = File::create(out_file_path).unwrap();
    let js_module = Module::new(module_name, in_file).unwrap();
    let extern_functions = vec![
        PrintFn::NAME.to_string(),
        AbortFn::NAME.to_string(),
        AssertFn::NAME.to_string(),
        AssertEqFn::NAME.to_string(),
    ];

    let llvm_module = js_module
        .precompile(extern_functions.clone().into_iter().map(|e| e.into()))
        .unwrap();

    llvm_module
        .compile_to(&mut out_file, extern_functions.into_iter())
        .unwrap();
}

fn compile_llvm_ir(in_file_path: String, out_file_name: String) {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{}", cur_dir.to_str().unwrap(), in_file_path.as_str());
    let out_arg = format!("-o={}", out_file_name,);

    Command::new("llc")
        .args(["-filetype=obj", out_arg.as_str(), in_arg.as_str()])
        .output()
        .expect("failed to execute process");
}

fn compile_binary(in_file_path: String, out_file_name: String) {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{}", cur_dir.to_str().unwrap(), in_file_path.as_str());
    let out_arg = format!("-o{}", out_file_name,);

    Command::new("clang")
        .args([out_arg.as_str(), in_arg.as_str()])
        .output()
        .expect("failed to execute process");
}

fn run_binary(in_file_path: String) -> bool {
    let cur_dir = current_dir().unwrap();

    let in_arg = format!("{}/{}", cur_dir.to_str().unwrap(), in_file_path.as_str());

    let output = Command::new(in_arg)
        .output()
        .expect("failed to execute process");
    output.status.success()
}
