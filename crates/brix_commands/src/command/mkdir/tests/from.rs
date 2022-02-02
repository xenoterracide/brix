// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::PathBuf;

use crate::command::Command;
use crate::{MkdirCommand, ProcessedCommandParams};
use brix_common::AppContext;
use brix_errors::BrixErrorKind;
use brix_processor::ProcessorCore;

macro_rules! run {
    ($args:expr) => {{
        let processor = ProcessorCore::new();
        let config = brix_cli::Config::default();
        let command = MkdirCommand::new();
        let context = AppContext {
            processor,
            config: &config,
        };
        // Ensure that it is a validation error that
        assert_eq!(
            command.run($args, &context).unwrap_err().kind.unwrap(),
            BrixErrorKind::Validation
        )
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
#[should_panic]
fn valid() {
    run!(ProcessedCommandParams {
        source: None,
        destination: Some(PathBuf::from("/tmp")),
        overwrite: None,
        search: None,
        replace: None,
        commands: None,
        stdout: None,
        context: None,
    })
}
