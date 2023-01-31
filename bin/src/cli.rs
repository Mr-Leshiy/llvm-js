use ast::{js_ast::Module, CompilerError, LexerError, PrecompilerError};
use clap::Parser;
use compiler::predefined_functions::test::{AssertEqFn, AssertFn, PrintFn};
use std::path::PathBuf;

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
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None, rename_all = "kebab-case")]
pub struct Cli {
    /// Path to the input file
    #[clap(long)]
    input: PathBuf,

    /// Path to the output file
    #[clap(long)]
    output: PathBuf,
}

impl Cli {
    pub fn exec(self) -> Result<(), Error> {
        let in_file = std::fs::File::open(self.input).map_err(Error::CannotOpenFile)?;
        let mut out_file = std::fs::File::create(self.output).map_err(Error::CannotCreateFile)?;

        let extern_functions = vec![
            PrintFn::NAME.to_string(),
            AssertFn::NAME.to_string(),
            AssertEqFn::NAME.to_string(),
        ];

        Module::new("module_1".to_string(), in_file)?
            .precompile(extern_functions.into_iter().map(Into::into))?
            .compile_to(&mut out_file)?;
        Ok(())
    }
}
