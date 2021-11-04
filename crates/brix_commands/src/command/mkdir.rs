// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fs::create_dir_all;
use std::path::PathBuf;
use validator::Validate;

use crate::command::{Command, ProcessedCommandParams};
use brix_common::AppContext;
use brix_errors::BrixError;

#[cfg(test)]
mod tests {
    mod from;
    mod run;
}

#[derive(Debug)]
pub struct MkdirParams {
    destination: PathBuf,
}

impl PartialEq for MkdirParams {
    fn eq(&self, other: &Self) -> bool {
        self.destination == other.destination
    }
}

#[derive(Debug, Validate)]
struct Params {
    #[validate(required)]
    destination: Option<PathBuf>,
}

pub struct MkdirCommand {}

impl MkdirCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for MkdirCommand {
    fn run(&self, pcp: ProcessedCommandParams, _app_context: &AppContext) -> Result<(), BrixError> {
        let cp = Params {
            destination: pcp.destination,
        };
        cp.validate()?;

        let dest = cp.destination.unwrap();
        create_dir_all(dest)?;

        Ok(())
    }

    fn name(&self) -> String {
        String::from("mkdir")
    }
}
