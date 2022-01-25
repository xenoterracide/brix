// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Module responsible for processing a [RawConfig] into a list of commands.

use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

use brix_commands::{
    CopyCommand, ExecCommand, MkdirCommand, SearchReplaceCommand, TemplateCommand,
};
use brix_common::context::{cli_config_to_map, ContextMap};
use brix_common::AppContext;
use brix_errors::BrixError;

use crate::ConfigLoader;
use crate::{Command, CommandList, RawConfig};
use crate::{ProcessedCommandParams, RawCommandParams};

lazy_static! {
    static ref SUPPORTED_COMMANDS: Vec<&'static str> =
        vec!["copy", "exec", "mkdir", "search_replace", "template"];
}

impl<'a> ConfigLoader<'a> {
    /// Converts the [RawConfig] into a list of commands or returns an error.
    pub fn process(
        &self,
        config: &RawConfig,
        app_context: &AppContext,
    ) -> Result<CommandList, BrixError> {
        let mut list = CommandList::new();

        for command in config.commands.iter() {
            let key = command.keys().next().unwrap();
            let value = command.values().next().unwrap();
            let command: Box<dyn Command> = match key.to_lowercase().as_str() {
                "copy" => Box::new(CopyCommand::new()),
                "exec" => Box::new(ExecCommand::new()),
                "mkdir" => Box::new(MkdirCommand::new()),
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

            // After the merge, template the actual context itself in case it includes context
            // For instance, the context might be something like `path: temp/{{module}}`
            let processor_context = brix_processor::create_context(context.clone());
            let mut processed_processor_context = HashMap::new();
            // TODO: perhaps templating each individual context line isn't really that performant...
            for (key, raw_line) in processor_context.iter() {
                // Replace the raw quotation marks
                let context_line = raw_line.to_string().replace("\"", "");
                let processed = app_context
                    .processor
                    .process(context_line, processor_context.clone())?;
                processed_processor_context.insert(String::from(key), processed);
            }

            let res = app_context.processor.process(
                json.to_string(),
                brix_processor::create_context(processed_processor_context.clone()),
            )?;
            let raw_args: RawCommandParams = serde_json::from_str(&res).unwrap();
            let mut args = self.create_processed_args(&raw_args)?;
            args.context = Some(processed_processor_context);

            list.push((command, args));
        }

        Ok(list)
    }

    /// Formats all the raw types (such as strings) into their preferred types
    /// for the given field (such as a path).
    fn create_processed_args(
        &self,
        raw: &RawCommandParams,
    ) -> Result<ProcessedCommandParams, BrixError> {
        macro_rules! lf {
            ($val:expr) => {{
                let mut val = $val;
                val = val.replace("\\n", "\n");
                val.replace("\\t", "\t")
            }};
        }

        let config = self.config_dir.as_ref().unwrap();

        let mut source = None;
        let mut destination = None;
        let mut overwrite = None;
        let mut search = None;
        let mut replace = None;
        let mut commands = None;
        let mut stdout = None;
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
            search = Some(lf!(raw_search.clone()));
        }
        if let Some(raw_replace) = &raw.replace {
            replace = Some(lf!(raw_replace.clone()));
        }
        if let Some(raw_commands) = &raw.commands {
            commands = Some(raw_commands.clone());
        }
        if let Some(raw_stdout) = &raw.stdout {
            stdout = Some(raw_stdout.clone());
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
            commands,
            stdout,
            context,
        })
    }
}
