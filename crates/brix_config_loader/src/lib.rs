use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

mod parsers;
use parsers::ConfigParser;
pub use parsers::YamlConfigParser;

use brix_commands::CopyCommand;
use brix_commands::{Command, ProcessedCommandParams};
use brix_errors::BrixError;

pub type ParserList = Vec<Box<dyn ConfigParser>>;
type CommandList = Vec<(Box<dyn Command>, ProcessedCommandParams)>;

pub struct ConfigLoader {
    parsers: Vec<Box<dyn ConfigParser>>,
    config_dir: Option<PathBuf>,
}

impl ConfigLoader {
    pub fn new(parsers: ParserList) -> Self {
        Self {
            parsers,
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
            panic!("File format not supported: {:?}", config_file);
        }

        let config = parser.unwrap().parse(processed)?;
        self.process(&config)
    }

    fn process(&self, config: &RawConfig) -> Result<CommandList, BrixError> {
        let mut list = CommandList::new();

        for command in config.commands.iter() {
            let key = command.keys().next().unwrap();
            let value = command.values().next().unwrap();
            let args = self.create_processed_args(value)?;
            let command = match key.to_lowercase().as_str() {
                "copy" => CopyCommand::new(),
                _ => panic!("Command `{}` not found!", key),
            };

            list.push((Box::new(command), args));
        }

        Ok(list)
    }

    fn create_processed_args(
        &self,
        raw: &RawCommandParams,
    ) -> Result<ProcessedCommandParams, BrixError> {
        let config = self.config_dir.as_ref().unwrap();

        let mut source = None;
        let mut destination = None;
        let mut overwrite = None;
        let mut search = None;
        let mut replace = None;

        if let Some(raw_source) = &raw.source {
            source = Some(config.join(raw_source)); // Source is relative to config
        };
        if let Some(raw_destination) = &raw.destination {
            destination = Some(Path::new(raw_destination).to_path_buf()); // Dest is absolute path
        };
        if let Some(raw_overwrite) = raw.overwrite {
            overwrite = Some(raw_overwrite);
        };
        if let Some(raw_search) = &raw.search {
            search = Some(Regex::new(&raw_search).unwrap());
        };
        if let Some(raw_replace) = &raw.replace {
            replace = Some(raw_replace.clone());
        };

        Ok(ProcessedCommandParams {
            source: source,
            destination: destination,
            overwrite: overwrite,
            search: search,
            replace: replace,
            context: None,
        })
    }
}

#[derive(Debug)]
struct Config {
    commands: Vec<(String, RawCommandParams)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawConfig {
    commands: Vec<HashMap<String, RawCommandParams>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawCommandParams {
    source: Option<String>,
    destination: Option<String>,
    overwrite: Option<bool>,
    search: Option<String>,
    replace: Option<String>,
}
