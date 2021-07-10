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
use brix_errors::BrixError;

pub type ParserList = Vec<Box<dyn ConfigParser>>;
type CommandList = Vec<(Box<dyn Command>, ProcessedCommandParams)>;

pub struct ConfigLoader<'a> {
    parsers: Vec<Box<dyn ConfigParser>>,
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

    pub fn run(&self) -> Result<CommandList, BrixError> {
        let mut parser: Option<&Box<dyn ConfigParser>> = None;

        for parser_opt in self.parsers.iter() {
            if parser_opt.matches(self.config_file.as_ref().unwrap()) {
                parser = Some(parser_opt);
                break;
            }
        }

        let contents = fs::read_to_string(self.config_file.as_ref().unwrap())?;
        let config = parser.unwrap().parse(&contents)?;
        self.process(&config)
    }
}

#[derive(Debug)]
struct Config {
    context: Option<HashMap<String, String>>,
    commands: Vec<(String, RawCommandParams)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawConfig {
    context: Option<HashMap<String, String>>,
    commands: Vec<HashMap<String, RawCommandParams>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawCommandParams {
    source: Option<String>,
    destination: Option<String>,
    overwrite: Option<bool>,
    search: Option<String>,
    replace: Option<String>,
    context: Option<HashMap<String, String>>,
}
