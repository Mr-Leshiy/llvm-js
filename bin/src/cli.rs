use assembler::{linker::compile_binary, llc::compile_llvm_ir, AssemblerError};
use ast::{js_ast::Module, CompilerError, LexerError, PrecompilerError};
use clap::Parser;
use compiler::predefined_functions::test::{AssertEqFn, AssertFn, PrintFn};
use std::{fs::remove_file, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CannotOpenFile(std::io::Error),
    #[error(transparent)]
    CannotCreateFile(std::io::Error),
    #[error(transparent)]
    Lexer(#[from] LexerError),
    #[error(transparent)]
    Precompiler(#[from] PrecompilerError),
    #[error(transparent)]
    Compiler(#[from] CompilerError),
    #[error(transparent)]
    Assembler(#[from] AssemblerError),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None, rename_all = "kebab-case")]
pub struct Cli {
    /// Path to the input file
    #[clap(long)]
    input: PathBuf,

    /// Binary name
    #[clap(long, default_value = "run")]
    binary_name: String,

    #[clap(long, default_value_t = false)]
    clean: bool,
}

impl Cli {
    pub fn exec(self) -> Result<(), Error> {
        let in_file = std::fs::File::open(&self.input).map_err(Error::CannotOpenFile)?;
        let file_name = self.input.file_stem().unwrap().to_str().unwrap();

        let ll_file_name = format!("{file_name}.ll");
        let object_file_name = format!("{file_name}.o");

        let (ll_file_path, object_file_path) = if let Some(parent) = self.input.parent() {
            (parent.join(ll_file_name), parent.join(object_file_name))
        } else {
            (ll_file_name.into(), object_file_name.into())
        };

        let mut ll_file = std::fs::File::create(&ll_file_path).map_err(Error::CannotCreateFile)?;

        let extern_functions = vec![
            PrintFn::NAME.to_string(),
            AssertFn::NAME.to_string(),
            AssertEqFn::NAME.to_string(),
        ];

        Module::new(file_name.to_string(), in_file)?
            .precompile(extern_functions.into_iter().map(Into::into))?
            .compile_to(&mut ll_file)?;
        compile_llvm_ir(&ll_file_path, &object_file_path)?;
        compile_binary(&object_file_path, &self.binary_name.into())?;
        if self.clean {
            remove_file(&ll_file_path).unwrap();
            remove_file(&object_file_path).unwrap();
        }
        Ok(())
    }
}
