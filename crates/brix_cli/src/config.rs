// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use clap::ArgMatches;
use log::LevelFilter;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use crate::app;

pub struct Config {
    pub language: String,
    pub config_name: String,
    pub project: String,
    pub module: String,

    pub config_dir: String,
    pub home_dir: Option<PathBuf>,
    pub log_level: log::LevelFilter,
    // TODO: Add flags
    pub raw_matches: ArgMatches<'static>,
}

impl Config {
    /// Parses matches and sets into config
    pub fn new(home_dir: Option<PathBuf>, matches: ArgMatches<'static>) -> Self {
        let language = matches.value_of_lossy(app::LANGUAGE).unwrap().to_string();
        let config_name = matches
            .value_of_lossy(app::CONFIG_NAME)
            .unwrap()
            .to_string();
        let project = matches.value_of_lossy(app::PROJECT).unwrap().to_string();
        let module = matches.value_of_lossy(app::MODULE).unwrap().to_string();

        let config_dir = matches
            .value_of_lossy(app::CONFIG_DIR)
            .unwrap_or(Cow::from(".config/brix"))
            .to_string();
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
            home_dir,
            log_level: log_level_to_struct(&log_level),
            module,
        }
    }
}

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

impl Display for Config {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "[LANGUAGE: {}, CONFIG_NAME: {}, PROJECT: {}, MODULE: {}, CONFIG_DIR: {}]",
            self.language, self.config_name, self.project, self.module, self.config_dir
        )
    }
}
