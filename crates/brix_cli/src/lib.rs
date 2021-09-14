// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use brix_errors::BrixError;

mod app;
mod args;
mod config;

pub mod select;

pub use args::clap_matches;
pub use clap::ArgMatches;
pub use config::Config;
