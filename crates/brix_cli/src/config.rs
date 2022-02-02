// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Contains the `Config` struct and helper functions.

use crate::error;
use clap::ArgMatches;
use colored::*;
use log::LevelFilter;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use crate::app;

/// The core config struct used within Brix's lifecycle.
/// Mostly the direct output of the CLI args and flags,
/// but includes some additional properties as well.
pub struct Config {
    pub language: String,
    pub config_name: String,
    pub project: String,
    pub module: String,

    pub config_dir: Option<PathBuf>,
    pub workdir: PathBuf,
    pub home_dir: Option<PathBuf>,
    pub log_level: log::LevelFilter,

    pub raw_matches: ArgMatches<'static>,
}

impl Config {
    /// Creates the config given the home directory and argument matches from `clap`.
    pub fn new(home_dir: Option<PathBuf>, matches: ArgMatches<'static>) -> Self {
        let current_dir: Result<PathBuf, ()> = std::env::current_dir().or_else(|_| {
            error!(
                "
    Something went wrong. Your current working directory is invalid.
    This is either due to the directory being deleted or insufficient permissions.",
            );
            std::process::exit(2);
        });

        let language = matches.value_of_lossy(app::LANGUAGE).unwrap().to_string();
        let config_name = matches
            .value_of_lossy(app::CONFIG_NAME)
            .unwrap()
            .to_string();
        let project = matches.value_of_lossy(app::PROJECT).unwrap().to_string();
        let module = matches.value_of_lossy(app::MODULE).unwrap().to_string();

        let config_dir = matches
            .value_of_lossy(app::CONFIG_DIR)
            .and_then(|s| Some(PathBuf::from(s.to_string())));

        let workdir = PathBuf::from(
            matches
                .value_of_lossy(app::WORKDIR)
                .unwrap_or(Cow::from(current_dir.unwrap().to_str().unwrap()))
                .to_string(),
        );
        let log_level = matches
            .value_of_lossy(app::LOG_LEVEL)
            .unwrap_or(Cow::from("off"))
            .to_string();

        Self {
            raw_matches: matches,
            language,
            config_name,
            project,
            config_dir,
            workdir,
            home_dir,
            log_level: log_level_to_struct(&log_level),
            module,
        }
    }
}

/// Returns a `log::LevelFilter` given a string.
fn log_level_to_struct(level: &str) -> LevelFilter {
    match level {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => unimplemented!(),
    }
}

#[doc(hidden)]
macro_rules! s {
    () => {
        String::new()
    };
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            language: s!(),
            config_name: s!(),
            project: s!(),
            module: s!(),
            config_dir: None,
            workdir: std::env::current_dir().unwrap(),
            home_dir: None,
            log_level: LevelFilter::Off,
            raw_matches: ArgMatches::default(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "[LANGUAGE: {}, CONFIG_NAME: {}, PROJECT: {}, MODULE: {}, CONFIG_DIR: {:?}]",
            self.language, self.config_name, self.project, self.module, self.config_dir
        )
    }
}
