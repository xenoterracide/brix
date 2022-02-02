// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Handles the core `clap_matches()` function.

use clap;
use std::env;
use std::io::{self, Write};
use std::process;

use crate::app;

/// Executes the core `clap` function to parse argumensts from `env::args_os()`.
pub fn clap_matches() -> Result<clap::ArgMatches<'static>, crate::BrixError> {
    let err = match app::app().get_matches_from_safe(env::args_os()) {
        Ok(matches) => return Ok(matches),
        Err(err) => err,
    };
    if err.use_stderr() {
        return Err(err.into());
    }

    let _ = write!(io::stdout(), "{}", err);
    process::exit(0);
}
