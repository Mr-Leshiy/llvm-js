use assembler::{compile_binary, AssemblerError};
use ast::{js_ast::Module, CompilerError, LexerError, PrecompilerError};
use clap::Parser;
use compiler::predefined_functions::test::{AssertEqFn, AssertFn, PrintFn};
use std::{
    fs::remove_file,
    path::{Path, PathBuf},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CannotOpenFile(std::io::Error),
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

        let ll_file_path = if let Some(parent) = self.input.parent() {
            parent.join(ll_file_name)
        } else {
            ll_file_name.into()
        };

        let extern_functions = vec![
            PrintFn::NAME.to_string(),
            AssertFn::NAME.to_string(),
            AssertEqFn::NAME.to_string(),
        ];

        Module::new(file_name.to_string(), in_file)?
            .precompile(extern_functions.into_iter().map(Into::into))?
            .compile_to(&ll_file_path)?;
        compile_binary(&ll_file_path, Path::new(&self.binary_name))?;
        if self.clean {
            remove_file(&ll_file_path).unwrap();
        }
        Ok(())
    }
}
