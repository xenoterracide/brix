use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use fancy_regex::Regex;
use log::info;
use validator::Validate;

use crate::command::{Command, ProcessedCommandParams};
use brix_common::AppContext;
use brix_errors::BrixError;

#[cfg(test)]
mod tests {
    mod invalid;
    mod run;
}

#[derive(Debug)]
pub struct SearchReplaceParams {
    destination: PathBuf,
    search: String,
    replace: String,
}

impl PartialEq for SearchReplaceParams {
    fn eq(&self, other: &Self) -> bool {
        return self.destination == other.destination
            && self.search.as_str() == other.search.as_str()
            && self.replace == other.replace;
    }
}

#[derive(Debug, Validate)]
struct Params {
    #[validate(required)]
    destination: Option<PathBuf>,
    #[validate(required)]
    search: Option<String>,
    #[validate(required)]
    replace: Option<String>,
}

pub struct SearchReplaceCommand {}

impl SearchReplaceCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for SearchReplaceCommand {
    fn run(&self, pcp: ProcessedCommandParams, _app_context: &AppContext) -> Result<(), BrixError> {
        let cp = Params {
            destination: pcp.destination,
            search: pcp.search,
            replace: pcp.replace,
        };
        cp.validate()?;

        let dest = cp.destination.unwrap();
        info!("reading to string from '{}'", dest.clone().display());
        let data = fs::read_to_string(dest.clone()).or_else(|err| {
            return Err(BrixError::with(&format!(
                "unable to read file '{}': {}",
                dest.display(),
                err
            )));
        })?;

        info!("creating regex '{}'", &cp.search.clone().unwrap());
        let re = Regex::new(&cp.search.unwrap())?;
        let result = re.replace_all(&data, cp.replace.unwrap());

        info!("writing changes");
        let mut write = File::create(dest.clone()).unwrap();
        write.write_all(result.as_bytes()).or_else(|_err| {
            return Err(BrixError::with(&format!(
                "unable to write to file '{}'",
                dest.display()
            )));
        })?;

        Ok(())
    }

    fn name(&self) -> String {
        String::from("search and replace")
    }
}
