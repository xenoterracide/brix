use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod command;
mod config;

use brix_commands::Command;
use brix_errors::BrixError;

type CommandList = Vec<Box<dyn Command>>;

pub struct ConfigLoader {
    parsers: Vec<Box<dyn ConfigParser>>,
}

impl ConfigLoader {
    pub fn new(parsers: Vec<Box<dyn ConfigParser>>) -> Self {
        Self { parsers }
    }

    pub fn load(&self) -> Result<CommandList, BrixError> {
        let config_file = std::path::Path::new("");
        let parser: Option<Box<dyn ConfigParser>>;

        for parser_opt in self.parsers {
            if parser_opt.matches(&config_file.to_path_buf()) {
                parser = Some(parser_opt);
                break;
            }
        }

        if let None = parser {
            panic!("File format not supported: {:?}", config_file);
        }

        let config = parser.unwrap().parse(&config_file.to_path_buf())?;
        self.process(&config)
    }

    fn process(config: &Config) -> Result<CommandList, BrixError> {
        // Go through and **process all templates**
        // might even delegate stuff to templates itself
        // Converting strings to PathBufs
        // Converting strings to patterns

        for command in config.commands.iter() {
            println!("{:?}", command);
        }

        Ok(CommandList::new())
    }
}

trait ConfigParser {
    fn parse(&self, path: &PathBuf) -> Result<Config, BrixError>;
    fn matches(&self, path: &PathBuf) -> bool;
}

struct YamlConfigParser {}

impl ConfigParser for YamlConfigParser {
    fn parse(&self, path: &PathBuf) -> Result<Config, BrixError> {}

    fn matches(&self, path: &PathBuf) -> bool {}
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
