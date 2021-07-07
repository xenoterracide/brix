use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde_json::json;

mod context;
mod parsers;
use context::ContextMap;
use parsers::ConfigParser;
pub use parsers::YamlConfigParser;

use brix_commands::{Command, ProcessedCommandParams};
use brix_commands::{CopyCommand, SearchReplaceCommand, TemplateCommand};
use brix_errors::BrixError;

pub type ParserList = Vec<Box<dyn ConfigParser>>;
type CommandList = Vec<(Box<dyn Command>, ProcessedCommandParams)>;

lazy_static! {
    static ref SUPPORTED_COMMANDS: Vec<&'static str> = vec!["copy", "search_replace"];
}

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

    fn process(&self, config: &RawConfig) -> Result<CommandList, BrixError> {
        let mut list = CommandList::new();

        for command in config.commands.iter() {
            let key = command.keys().next().unwrap();
            let value = command.values().next().unwrap();
            let command: Box<dyn Command> = match key.to_lowercase().as_str() {
                "copy" => Box::new(CopyCommand::new()),
                "search_replace" => Box::new(SearchReplaceCommand::new()),
                "template" => Box::new(TemplateCommand::new()),
                _ => {
                    let matches =
                        difflib::get_close_matches(key, SUPPORTED_COMMANDS.to_vec(), 1, 0.6);
                    if let Some(closest) = matches.get(0) {
                        return Err(BrixError::with(&format!(
                            "command '{}' not found... did you mean '{}'?",
                            key, closest
                        )));
                    } else {
                        return Err(BrixError::with(&format!("command '{}' not found", key)));
                    }
                }
            };

            // Serialize the data into json
            let json = json!(value);
            // Read context
            let local_context = value.context.clone().unwrap_or(HashMap::new());
            // Create context map and populate accordingly
            let context_map = ContextMap {
                cli_positional: cli_config_to_map(self.cli_config),
                config_global: config.context.clone().unwrap_or(HashMap::new()),
                command_local: local_context,
            };
            // Merge contexts together
            let context = context_map.do_merge();

            let processor_context = brix_processor::create_context(context);
            let res = brix_processor::process(json.to_string(), processor_context)?;
            let raw_args: RawCommandParams = serde_json::from_str(&res).unwrap();
            let args = self.create_processed_args(&raw_args)?;

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
        let mut context = None;

        if let Some(raw_source) = &raw.source {
            source = Some(config.join(raw_source)); // Source is relative to config
        }
        if let Some(raw_destination) = &raw.destination {
            destination = Some(PathBuf::from(raw_destination)); // Dest is absolute path
        }
        if let Some(raw_overwrite) = raw.overwrite {
            overwrite = Some(raw_overwrite);
        }
        if let Some(raw_search) = &raw.search {
            search = Some(raw_search.clone());
        }
        if let Some(raw_replace) = &raw.replace {
            replace = Some(raw_replace.clone());
        }
        if let Some(raw_context) = &raw.context {
            context = Some(raw_context.clone());
        }

        Ok(ProcessedCommandParams {
            source,
            destination,
            overwrite,
            search,
            replace,
            context,
        })
    }
}

fn cli_config_to_map(config: &brix_cli::Config) -> HashMap<String, String> {
    macro_rules! s {
        ($st:expr) => {
            String::from($st)
        };
    }

    let mut map = HashMap::new();
    map.insert(s!("language"), s!(&config.language));
    map.insert(s!("module"), s!(&config.module));
    map.insert(s!("project"), s!(&config.project));
    map
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
