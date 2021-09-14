// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

use clap::Shell;

#[path = "crates/brix_cli/src/app.rs"]
mod app;

fn main() {
    let out = match env::var_os("OUT_DIR") {
        Some(o) => o,
        None => panic!("OUT_DIR environment variable not defined!"),
    };
    fs::create_dir_all(&out).unwrap();

    let mut app = app::app();
    app.gen_completions("brix", Shell::Bash, &out);
    app.gen_completions("brix", Shell::Fish, &out);
    app.gen_completions("brix", Shell::PowerShell, &out);
}
