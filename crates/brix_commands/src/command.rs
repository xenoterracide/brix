// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::format;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use dialoguer::console::Term;
use dialoguer::Confirm;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

use brix_common::AppContext;
use brix_errors::BrixError;

pub mod copy;
pub mod exec;
pub mod mkdir;
pub mod search_replace;
pub mod template;

pub trait Command {
    fn run(&self, pcp: ProcessedCommandParams, app_context: &AppContext) -> Result<(), BrixError>;
    fn name(&self) -> String;
}

pub trait OverwritableCommand: Command {
    type Params: OverwritableParams + 'static;

    fn term(&self) -> Term;

    fn name_inner(&self) -> String;

    fn ask_to_write(&self, path: &Path) -> bool {
        let res = Confirm::new()
            .with_prompt(format!("overwrite '{}'", path.display()))
            .default(false)
            .interact_on(&Term::stdout());
        match res {
            Ok(b) => b,
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    }

    fn write(&self, params: Self::Params, app_context: &AppContext) -> Result<(), BrixError> {
        info!("writing: '{}'", params.destination().display());
        self.write_impl(params, app_context)
    }

    fn skip_write(&self, path: &Path) -> Result<(), BrixError> {
        info!("skipping: '{}'", path.display());
        Ok(())
    }

    fn from(&self, pcp: ProcessedCommandParams) -> Result<Self::Params, ValidationErrors>;

    fn write_impl(&self, params: Self::Params, app_context: &AppContext) -> Result<(), BrixError>;
}

impl<T> Command for T
where
    T: OverwritableCommand,
{
    fn run(&self, pcp: ProcessedCommandParams, app_context: &AppContext) -> Result<(), BrixError> {
        let params = self.from(pcp)?;

        if !params.source().exists() {
            return Err(BrixError::with(&format!(
                "source '{}' does not exist",
                &params.source().display()
            )));
        }

        let dest = &params.destination();
        let parent = &dest.parent();
        if !(parent.is_some() && parent.unwrap().exists()) && parent.is_some() {
            debug!("creating directory '{}'", parent.unwrap().display());
            if let Err(e) = create_dir_all(parent.unwrap()) {
                return Err(BrixError::with(&format!(
                    "unable to create '{}': {}",
                    parent.unwrap().display(),
                    e
                )));
            }
        }

        if params.overwrite().is_some() {
            let overwrite = params.overwrite().unwrap();
            if overwrite {
                return self.write(params, app_context);
            } else if dest.exists() {
                return self.skip_write(dest);
            }
        }
        if self.ask_to_write(dest) {
            return self.write(params, app_context);
        };
        return self.skip_write(dest);
    }

    fn name(&self) -> String {
        self.name_inner()
    }
}

pub trait OverwritableParams {
    fn source(&self) -> PathBuf;
    fn destination(&self) -> PathBuf;
    fn overwrite(&self) -> Option<bool>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedCommandParams {
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
    pub overwrite: Option<bool>,
    pub search: Option<String>,
    pub replace: Option<String>,
    pub commands: Option<Vec<String>>,
    pub stdout: Option<bool>,
    pub context: Option<HashMap<String, String>>,
}
