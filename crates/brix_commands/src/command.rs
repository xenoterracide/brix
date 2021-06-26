use std::format;
use std::fs::create_dir_all;
use std::iter::Map;
use std::path::{Path, PathBuf};

use dialoguer::console::Term;
use dialoguer::Confirm;
use log::{debug, error, info};
use regex::Regex;
use simple_error::{simple_error, SimpleError};
use validator::ValidationErrors;

pub mod copy;

pub trait Command {
    fn run(&self, pcp: ProcessedCommandParams) -> Result<(), SimpleError>;
}

pub trait OverwritableCommand {
    type Params: OverwritableParams + 'static;

    fn term(&self) -> Term;

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

    fn write(&self, params: Self::Params) -> Result<(), SimpleError> {
        info!("writing: '{}'", params.destination().display());
        self.write_impl(params)
    }

    fn skip_write(&self, path: &Path) -> Result<(), SimpleError> {
        info!("skipping: '{}'", path.display());
        Ok(())
    }

    fn from(&self, pcp: ProcessedCommandParams) -> Result<Self::Params, ValidationErrors>;

    fn write_impl(&self, params: Self::Params) -> Result<(), SimpleError>;
}

impl<T> Command for T
where
    T: OverwritableCommand,
{
    fn run(&self, pcp: ProcessedCommandParams) -> Result<(), SimpleError> {
        let params = self
            .from(pcp)
            .map_err(|err| SimpleError::with("validate", err))?;

        if !params.source().exists() {
            return Err(simple_error!(format!(
                "source '{}' does not exist",
                params.source().display()
            )));
        }

        let dest = &params.destination();
        let parent = &dest.parent();
        if !(parent.is_some() && parent.unwrap().exists()) && parent.is_some() {
            debug!("creating directory '{}'", parent.unwrap().display());
            create_dir_all(parent.unwrap()).map_err(|err| {
                SimpleError::with(
                    &*format!("unable to create '{}'", parent.unwrap().display()),
                    err,
                )
            })?;
        }

        if params.overwrite().is_some() {
            let overwrite = params.overwrite().unwrap();
            if overwrite {
                return self.write(params);
            } else if dest.exists() {
                return self.skip_write(dest);
            }
        }
        if self.ask_to_write(dest) {
            return self.write(params);
        }
        self.write(params)
    }
}

pub trait OverwritableParams {
    fn source(&self) -> PathBuf;
    fn destination(&self) -> PathBuf;
    fn overwrite(&self) -> Option<bool>;
}

pub struct ProcessedCommandParams {
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
    pub overwrite: Option<bool>,
    pub search: Option<Regex>,
    pub replace: Option<String>,
    pub context: Option<Map<String, String>>,
}
