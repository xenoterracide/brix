use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod context;
mod parsers;
mod process;
use parsers::ConfigParser;
pub use parsers::YamlConfigParser;

use brix_commands::{Command, ProcessedCommandParams};
use brix_errors::BrixError;

pub type ParserList = Vec<Box<dyn ConfigParser>>;
type CommandList = Vec<(Box<dyn Command>, ProcessedCommandParams)>;

pub struct ConfigLoader<'a> {
    parsers: Vec<Box<dyn ConfigParser>>,
    config_dir: Option<PathBuf>,
    cli_config: &'a brix_cli::Config,
}

impl<'a> ConfigLoader<'a> {
    pub fn new(parsers: ParserList, cli_config: &'a brix_cli::Config) -> Self {
        Self {
            parsers,
            cli_config,
            config_dir: None,
        }
    }

    pub fn load(
        &mut self,
        config_file: &PathBuf,
        processed: &str,
    ) -> Result<CommandList, BrixError> {
        self.config_dir = Some(config_file.parent().unwrap().to_path_buf());
        let mut parser: Option<&Box<dyn ConfigParser>> = None;

        for parser_opt in self.parsers.iter() {
            if parser_opt.matches(&config_file.to_path_buf()) {
                parser = Some(parser_opt);
                break;
            }
        }

        if let None = parser {
            return Err(BrixError::with(&format!(
                "the file extension '.{}' is not supported for configs",
                config_file.extension().unwrap().to_string_lossy()
            )));
        }

        let config = parser.unwrap().parse(processed)?;
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
