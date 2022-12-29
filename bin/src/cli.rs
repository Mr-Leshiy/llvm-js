use ast::js_ast::Module;
use clap::Parser;
use compiler::predefined_functions::{
    test::{AssertEqFn, AssertFn, GbVariablesCount, PrintFn},
    PredefineFunctionName,
};
use std::path::PathBuf;

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
    pub fn exec(self) {
        let in_file = std::fs::File::open(self.input).unwrap();
        let mut out_file = std::fs::File::create(self.output).unwrap();

        let extern_functions = vec![
            PrintFn::NAME.to_string(),
            AssertFn::NAME.to_string(),
            AssertEqFn::NAME.to_string(),
            GbVariablesCount::NAME.to_string(),
        ];

        Module::new("module_1".to_string(), in_file)
            .unwrap()
            .precompile(extern_functions.into_iter().map(|e| e.into()))
            .unwrap()
            .compile_to(&mut out_file)
            .unwrap();
    }
}
