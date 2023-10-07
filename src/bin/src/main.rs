#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

use clap::Parser;
use cli::Cli;
use std::error::Error;

mod cli;

fn main() {
    Cli::parse().exec().unwrap_or_else(|e| report_error(&e));
}

fn report_error(error: &cli::Error) {
    eprintln!("{error}");
    let mut source = error.source();
    while let Some(sub_error) = source {
        eprintln!("  |-> {sub_error}");
        source = sub_error.source();
    }
    std::process::exit(1)
}
