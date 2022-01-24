// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::PathBuf;

use crate::command::Command;
use crate::{ProcessedCommandParams, TemplateCommand};
use brix_common::AppContext;
use brix_errors::BrixErrorKind;
use brix_processor::ProcessorCore;

macro_rules! run {
    ($args:expr) => {{
        let processor = ProcessorCore::new();
        let config = brix_cli::Config::default();
        let command = TemplateCommand::new();
        let context = AppContext {
            processor,
            config: &config,
        };
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
        commands: None,
        stdout: None,
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
        commands: None,
        stdout: None,
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
        commands: None,
        stdout: None,
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
        commands: None,
        stdout: None,
        context: None,
    })
}
