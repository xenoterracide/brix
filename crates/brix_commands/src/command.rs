use std::iter::Map;
use std::path::PathBuf;

use regex::Regex;

use brix_errors::BrixError;

pub trait Command {
    fn run(&self, params: ProcessedCommandParams) -> Result<(), BrixError>;
}

pub struct ProcessedCommandParams {
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
    pub overwrite: Option<bool>,
    pub search: Option<Regex>,
    pub replace: Option<String>,
    pub context: Option<Map<String, String>>,
}
