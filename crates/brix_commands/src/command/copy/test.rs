use std::error::Error;
use std::ops::Deref;

use dialoguer::console::Term;
use pretty_assertions::assert_eq;
use simple_error::SimpleError;
use spectral::result::{ContainingResultAssertions, ResultAssertions};
use spectral::{assert_that, Spec};

use super::super::{Command, ProcessedCommandParams};
use super::CopyCommand;

#[test]
fn run() {
    let command = CopyCommand {
        term: Term::stdout(),
    };

    let params = ProcessedCommandParams {
        source: None,
        destination: None,
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    };

    assert_that!(command.run(params));
}

#[test]
fn run_invalid_params() {
    let command = CopyCommand {
        term: Term::stdout(),
    };

    let params = ProcessedCommandParams {
        source: None,
        destination: None,
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    };

    let err = command.run(params).err().unwrap();

    assert_eq!(err.as_str(), "validate");
}
