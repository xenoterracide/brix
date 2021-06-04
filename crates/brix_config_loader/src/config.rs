use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub file: Option<FileProperties>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileProperties {
    source: Option<String>,
    destination: Option<String>,
}

impl Config {
    pub fn get_command(&self) -> Command {
        if let Some(file) = self.file.as_ref() {
            return file.get_command();
        }

        Command::Empty
    }
}

impl FileProperties {
    pub fn get_command(&self) -> Command {
        if self.source.is_some() && self.destination.is_some() {
            let source = String::from(self.source.as_ref().unwrap());
            let dest = String::from(self.destination.as_ref().unwrap());
            return Command::TemplateAndCopy(source, dest);
        }

        Command::Empty
    }
}
