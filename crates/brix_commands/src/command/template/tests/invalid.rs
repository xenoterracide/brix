use std::path::PathBuf;

use crate::command::Command;
use crate::{ProcessedCommandParams, TemplateCommand};
use brix_errors::BrixErrorKind;

macro_rules! run {
    ($args:expr) => {{
        let command = TemplateCommand::new();
        // Ensure it is a validation error
        assert_eq!(
            command.run($args).unwrap_err().kind.unwrap(),
            BrixErrorKind::Validation
        );
    }};
}

#[test]
fn nothing() {
    run!(ProcessedCommandParams {
        source: None,
        destination: None,
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    })
}

#[test]
fn source() {
    run!(ProcessedCommandParams {
        source: Some(PathBuf::new()),
        destination: None,
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    })
}

#[test]
fn destination() {
    run!(ProcessedCommandParams {
        source: None,
        destination: Some(PathBuf::new()),
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    })
}

#[test]
#[should_panic]
fn valid() {
    run!(ProcessedCommandParams {
        source: Some(PathBuf::new()),
        destination: Some(PathBuf::new()),
        overwrite: None,
        search: None,
        replace: None,
        context: None,
    })
}
