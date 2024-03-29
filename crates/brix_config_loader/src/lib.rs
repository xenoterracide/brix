// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! # Brix Config Loader
//! The config loader is responsible for loading the declaration file and dispatching it
//! to the appropriate parser depending on the extension. It is also responsible for converting
//! the declaration file into a parsed list of [commands](`brix_commands::Command`) and arguments.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod parsers;
mod process;
use parsers::ConfigParser;
pub use parsers::YamlConfigParser;

use brix_cli::select::do_select;
use brix_commands::{Command, ProcessedCommandParams};
use brix_common::AppContext;
use brix_errors::BrixError;

#[allow(rustdoc::private_intra_doc_links)]
/// Defines a parser list as a vec of trait objects implementing [ConfigParser].
pub type ParserList = Vec<Box<dyn ConfigParser>>;
/// Defines a command list as a vec of tuples containing a [Command] trait object and [ProcessedCommandParams].
type CommandList = Vec<(Box<dyn Command>, ProcessedCommandParams)>;

/// Struct that holds current information about the loaded configs and parsers.
pub struct ConfigLoader<'a> {
    parsers: ParserList,
    config_file: Option<PathBuf>,
    config_dir: Option<PathBuf>,
    cli_config: &'a brix_cli::Config,
}

impl<'a> ConfigLoader<'a> {
    pub fn new(parsers: ParserList, cli_config: &'a brix_cli::Config) -> Self {
        Self {
            parsers,
            cli_config,
            config_file: None,
            config_dir: None,
        }
    }

    /// Selects the proper configuration file knowing the supported extension for config files.
    /// May possibly use a [select prompt](`brix_cli::select::do_select`) to ask the user to choose a config file.
    pub fn load(&mut self, config_files: Vec<PathBuf>) -> Result<PathBuf, BrixError> {
        self.config_dir = Some(config_files[0].parent().unwrap().to_path_buf());
        let mut all_extensions = Vec::new();

        for supported_parser in self.parsers.iter() {
            all_extensions.extend(supported_parser.extensions())
        }

        let possible_config_files: Vec<&PathBuf> = config_files
            .iter()
            .filter(|f| all_extensions.contains(&f.extension().unwrap().to_str().unwrap()))
            .collect();

        if possible_config_files.len() == 0 {
            return Err(BrixError::with(&format!(
                "the file extension '.{}' is not supported for configs",
                config_files[0].extension().unwrap().to_string_lossy()
            )));
        }

        self.config_file = Some(possible_config_files[0].to_path_buf());

        if possible_config_files.len() > 1 {
            let names: Vec<&str> = possible_config_files
                .clone()
                .into_iter()
                .map(|f| f.file_name().unwrap().to_str().unwrap())
                .collect();

            println!("found multiple config files, which one would you like to use?");
            let result = do_select(names)?;

            self.config_file = Some(possible_config_files.get(result).unwrap().to_path_buf());
        }

        Ok(self.config_file.as_ref().unwrap().clone())
    }

    #[allow(rustdoc::private_intra_doc_links)]
    /// Actually parsers the config file for errors depending on the correct parser for the file.
    /// Sends resulting parsed output to [process] to be processed into commands.
    pub fn run(&self, app_context: &AppContext) -> Result<CommandList, BrixError> {
        let mut parser: Option<&Box<dyn ConfigParser>> = None;

        // Loop over each of the valid parsers this instance is configured with and see
        // if the extension for the config file matches with the parser we are testing
        for parser_opt in self.parsers.iter() {
            if parser_opt.matches(self.config_file.as_ref().unwrap()) {
                parser = Some(parser_opt);
                break;
            }
        }

        // Read the contents of the file to a string and parse it into the raw struct
        let contents = fs::read_to_string(self.config_file.as_ref().unwrap())?;
        let config = parser.unwrap().parse(&contents)?;
        // Send it over to be processed (./process.rs)
        self.process(&config, app_context)
    }
}

/// The preferred config with a Vec of command tuples instead
/// of a Vec of HashMaps.
#[derive(Debug)]
struct Config {
    context: Option<HashMap<String, String>>,
    commands: Vec<(String, RawCommandParams)>,
}

/// The raw struct used for an entire config.
/// Accurately describes the fields in a valid config file.
#[derive(Serialize, Deserialize, Debug)]
pub struct RawConfig {
    context: Option<HashMap<String, String>>,
    commands: Vec<HashMap<String, RawCommandParams>>,
}

/// The raw output for any config parser.
/// Defines all fields and their inital (not preferred) types for all commands.
#[derive(Serialize, Deserialize, Debug)]
struct RawCommandParams {
    source: Option<String>,
    destination: Option<String>,
    overwrite: Option<bool>,
    search: Option<String>,
    replace: Option<String>,
    commands: Option<Vec<String>>,
    stdout: Option<bool>,
    context: Option<HashMap<String, String>>,
}
