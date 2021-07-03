use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod parsers;
use parsers::ConfigParser;
pub use parsers::YamlConfigParser;

use brix_commands::{Command, ProcessedCommandParams};
use brix_commands::{CopyCommand, SearchReplaceCommand};
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
            return Err(BrixError::with(&format!(
                "the file extension '.{}' is not supported for configs",
                config_file.extension().unwrap().to_string_lossy()
            )));
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
            let command: Box<dyn Command> = match key.to_lowercase().as_str() {
                "copy" => Box::new(CopyCommand::new()),
                "search_replace" => Box::new(SearchReplaceCommand::new()),
                _ => panic!("Command `{}` not found!", key),
            };

            list.push((command, args));
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
        let mut left_brace = None;
        let mut right_brace = None;

        if let Some(raw_source) = &raw.source {
            source = Some(config.join(raw_source)); // Source is relative to config
        };
        if let Some(raw_destination) = &raw.destination {
            destination = Some(PathBuf::from(raw_destination)); // Dest is absolute path
        };
        if let Some(raw_overwrite) = raw.overwrite {
            overwrite = Some(raw_overwrite);
        };
        if let Some(raw_search) = &raw.search {
            search = Some(raw_search.clone());
        };
        if let Some(raw_replace) = &raw.replace {
            replace = Some(raw_replace.clone());
        };
        if let Some(raw_left_brace) = &raw.left_brace {
            left_brace = Some(raw_left_brace.clone());
        }
        if let Some(raw_right_brace) = &raw.right_brace {
            right_brace = Some(raw_right_brace.clone());
        }

        Ok(ProcessedCommandParams {
            source,
            destination,
            overwrite,
            search,
            replace,
            left_brace,
            right_brace,
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
    left_brace: Option<String>,
    right_brace: Option<String>,
    context: Option<HashMap<String, String>>,
}
