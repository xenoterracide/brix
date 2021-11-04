// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::PathBuf;

use crate::command::Command;
use crate::{MkdirCommand, ProcessedCommandParams};
use brix_common::AppContext;
use brix_processor::ProcessorCore;

macro_rules! do_test {
    ($destination:expr) => {{
        let processor = ProcessorCore::new();
        let command = MkdirCommand::new();
        let context = AppContext { processor };

        let path = PathBuf::from("src/command/mkdir/temp").join($destination);

        let params = ProcessedCommandParams {
            source: None,
            destination: Some(path.clone()),
            overwrite: None,
            search: None,
            replace: None,
            commands: None,
            stdout: None,
            context: None,
        };

        command.run(params, &context).unwrap();
        let exists = path.exists();

        std::fs::remove_dir_all(&path).unwrap();

        assert!(exists);
    }};
}

#[test]
fn basic() {
    do_test!("basic");
}

#[test]
fn nested() {
    do_test!("nested/one/two");
}

#[test]
fn multiple_stages() {
    do_test!("multiple_stages");
    do_test!("multiple_stages/next");
}

#[test]
fn in_parent_dir() {
    do_test!("../in_parent");
}
