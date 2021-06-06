use brix_errors::BrixError;
use std::path::PathBuf;
use regex::Regex;

pub trait Command {
    fn run(&self) -> Result<(), BrixError>;
}

pub struct CopyCommand {
    // pub source: PathBuf,
    // pub dest: PathBuf,
    // pub overwrite: bool,
}

impl CopyCommand {
    pub fn new(_args: ProcessedCommandParams) -> Self {
        Self { } // Fix
    }
}

impl Command for CopyCommand {
    fn run(&self) -> Result<(), BrixError> {
        // TODO: implementation
        Ok(())
    }
}

pub struct ProcessedCommandParams {
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
    pub overwrite: Option<bool>,
    pub search: Option<Regex>,
    pub replace: Option<String>,
}