#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

use std::process::ExitStatus;

pub mod linker;
pub mod llc;

pub type AssemblerError = Error;

pub const CORE_LIB: &str = "core";
pub const FMT_LIB: &str = "fmtd";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Command error: {0}")]
    CommandError(#[from] std::io::Error),
    #[error("llc error, status code: {0}, stdout: {1}, stderr: {2}")]
    LlcError(ExitStatus, String, String),
    #[error("linker error, status code: {0}, stdout: {1}, stderr: {2}")]
    LinkerError(ExitStatus, String, String),
}
