// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::{read_to_string, remove_file};
use std::path::PathBuf;

use maplit::*;

use crate::command::Command;
use crate::{ProcessedCommandParams, TemplateCommand};

use brix_common::AppContext;
use brix_processor::ProcessorCore;

macro_rules! do_test {
    ($source:expr, $context:expr, $assertion:expr) => {{
        let processor = ProcessorCore::new();
        let context = AppContext { processor };

        let path = PathBuf::from("src/command/template").join($source);
        let temp_dir = "src/command/template/temp/";
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let destination = format!("{}{}{}", temp_dir, file_stem, "_output.txt");

        let args = create_args!(path.clone(), destination.clone(), $context);
        let command = TemplateCommand::new();
        command.run(args, &context).unwrap();

        let result = read_to_string(PathBuf::from(destination.clone())).unwrap();
        // Ensure that the file is removed from the temp directory
        remove_file(destination).unwrap();
        assert_eq!(result, $assertion);
    }};
}

macro_rules! create_args {
    ($source:expr, $destination:expr, $context:expr) => {
        ProcessedCommandParams {
            source: Some(PathBuf::from($source)),
            destination: Some(PathBuf::from($destination)),
            overwrite: Some(true),
            search: None,
            replace: None,
            context: Some($context),
        }
    };
}

macro_rules! s {
    ($st:expr) => {
        String::from($st)
    };
}

#[test]
fn simple_context() {
    do_test!(
        "simple_context.hbs",
        hashmap! {
            s!("word") => s!("templated")
        },
        "this is templated text\n"
    );
}

#[test]
fn multi_value_context() {
    do_test!(
        "multi_value_context.hbs",
        hashmap! {
            s!("foo") => s!("bar"),
            s!("bar") => s!("baz")
        },
        "a bar walks into a baz\n"
    );
}

#[test]
fn escaped() {
    do_test!(
        "escaped.hbs",
        hashmap! {
            s!("foo") => s!("(SEE?)")
        },
        "This is not (SEE?) escaped {{this}} is\n"
    );
}

#[test]
fn sub_extension() {
    do_test!(
        "sub_extension.rs.hbs",
        hashmap! {
            s!("foo") => s!("bar"),
            s!("bar") => s!("baz")
        },
        "bar baz\n"
    );
}

#[test]
fn unchanged() {
    do_test!(
        "unchanged.hbs",
        hashmap! {
            s!("foo") => s!("bar")
        },
        "This file will remain unchanged\n"
    );
}

#[test]
fn empty_tags() {
    do_test!("empty_tags.hbs", hashmap! {}, "The  here will be \n");
}
