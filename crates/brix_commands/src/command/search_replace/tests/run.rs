// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::read_to_string;
use std::path::PathBuf;

use crate::command::Command;
use crate::{ProcessedCommandParams, SearchReplaceCommand};
use brix_common::AppContext;
use brix_processor::ProcessorCore;

macro_rules! do_test {
    ($path:expr, $search:expr, $replace:expr, $assertion:expr) => {{
        let processor = ProcessorCore::new();
        let config = brix_cli::Config::default();
        let command = SearchReplaceCommand::new();
        let context = AppContext {
            processor,
            config: &config,
        };

        let path = PathBuf::from("src/command/search_replace").join($path);
        let contents = read_to_string(path.clone()).unwrap();
        let args = create_args!(path.clone(), $search, $replace);
        command.run(args, &context).unwrap();

        let result = read_to_string(path.clone()).unwrap();
        // Ensure file is reverted to its original state
        std::fs::write(path, contents).unwrap();
        assert_eq!(result, $assertion);
    }};
}

macro_rules! create_args {
    ($dest:expr, $search:expr, $replace:expr) => {
        ProcessedCommandParams {
            source: None,
            destination: Some(PathBuf::from($dest)),
            overwrite: None,
            search: Some(String::from($search)),
            replace: Some(String::from($replace)),
            commands: None,
            stdout: None,
            context: None,
        }
    };
}

#[test]
fn basic() {
    do_test!("basic.txt", "es", "ES", "ES\n");
}

#[test]
fn empty() {
    do_test!("empty.txt", "foo", "bar", "");
}

#[test]
fn no_matches() {
    do_test!("no_matches.txt", "z", "foo", "Lorem ipsum dolor sit amet\n");
}

#[test]
fn basic_regex() {
    do_test!(
        "basic_regex.txt",
        "[a-zA-Z]",
        "0",
        "00002390070029003929300000\n"
    );
}

#[test]
fn full_match() {
    do_test!("full_match_one.txt", "^[0-9]{10}\\n$", "!", "59023485\n");
    do_test!("full_match_two.txt", "^[0-9]{10}\\n$", "!", "!");
}

#[test]
fn diff_extension() {
    do_test!(
        "diff_extension.peb",
        "foo",
        "bar",
        "bar bar bar bar bar bar\n"
    )
}

#[test]
fn non_english() {
    do_test!(
        "non_english.txt",
        "это не английские буквы\n",
        "这些不是英文字母",
        "这些不是英文字母"
    );
}
