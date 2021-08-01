use std::path::PathBuf;

use crate::command::Command;
use crate::{ProcessedCommandParams, SearchReplaceCommand};
use brix_common::AppContext;
use brix_errors::BrixErrorKind;
use brix_processor::ProcessorCore;

macro_rules! run {
    ($args:expr) => {{
        let processor = ProcessorCore::new();
        let command = SearchReplaceCommand::new();
        let context = AppContext { processor };
        // Ensure it is a validation error
        assert_eq!(
            command.run($args, &context).unwrap_err().kind.unwrap(),
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
fn search() {
    run!(ProcessedCommandParams {
        source: None,
        destination: None,
        overwrite: None,
        search: Some(String::new()),
        replace: None,
        context: None,
    })
}

#[test]
fn replace() {
    run!(ProcessedCommandParams {
        source: None,
        destination: None,
        overwrite: None,
        search: None,
        replace: Some(String::new()),
        context: None,
    })
}

#[test]
fn destination_search() {
    run!(ProcessedCommandParams {
        source: None,
        destination: Some(PathBuf::new()),
        overwrite: None,
        search: Some(String::new()),
        replace: None,
        context: None,
    })
}

#[test]
fn destination_replace() {
    run!(ProcessedCommandParams {
        source: None,
        destination: Some(PathBuf::new()),
        overwrite: None,
        search: None,
        replace: Some(String::new()),
        context: None,
    })
}

#[test]
fn search_replace() {
    run!(ProcessedCommandParams {
        source: None,
        destination: None,
        overwrite: None,
        search: Some(String::new()),
        replace: Some(String::new()),
        context: None,
    })
}

#[test]
#[should_panic]
fn valid() {
    run!(ProcessedCommandParams {
        source: None,
        destination: Some(PathBuf::new()),
        overwrite: None,
        search: Some(String::new()),
        replace: Some(String::new()),
        context: None,
    })
}
