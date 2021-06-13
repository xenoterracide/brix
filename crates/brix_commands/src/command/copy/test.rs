use std::error::Error;
use std::ops::Deref;
use std::path::PathBuf;

use dialoguer::console::Term;
use simple_error::SimpleError;
use spectral::assert_that;
use spectral::result::{ContainingResultAssertions, ResultAssertions};

use super::super::{Command, ProcessedCommandParams};
use super::CopyCommand;

#[test]
fn run_invalid_param_source() {
    let command = CopyCommand {
        term: Term::stdout(),
    };

    let params = ProcessedCommandParams {
        source: Option::Some(PathBuf::new()),
        destination: None,
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    };

    assert_that!(command.run(params)).is_err();
}

#[test]
fn run_invalid_param_dest() {
    let command = CopyCommand {
        term: Term::stdout(),
    };

    let params = ProcessedCommandParams {
        source: None,
        destination: Option::Some(PathBuf::new()),
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    };

    assert_that!(command.run(params)).is_err();
}
