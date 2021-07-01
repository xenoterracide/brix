use std::fs::copy;
use std::path::PathBuf;

use dialoguer::console::Term;
use log::debug;
use simple_error::{try_with, SimpleError};
use validator::{Validate, ValidationErrors};

use crate::command::{OverwritableCommand, OverwritableParams, ProcessedCommandParams};

#[cfg(test)]
mod tests {
    mod from;
}

#[derive(Debug)]
pub struct CopyParams {
    source: PathBuf,
    destination: PathBuf,
    overwrite: Option<bool>,
}

impl PartialEq for CopyParams {
    fn eq(&self, other: &Self) -> bool {
        return self.source == other.source
            && self.destination == other.destination
            && self.overwrite == other.overwrite;
    }
}

impl OverwritableParams for CopyParams {
    fn source(&self) -> PathBuf {
        self.source.clone()
    }

    fn destination(&self) -> PathBuf {
        self.destination.clone()
    }

    fn overwrite(&self) -> Option<bool> {
        self.overwrite
    }
}

#[derive(Debug, Validate)]
struct Params {
    #[validate(required)]
    source: Option<PathBuf>,
    #[validate(required)]
    destination: Option<PathBuf>,
    overwrite: Option<bool>,
}

pub struct CopyCommand {
    term: Term,
}

impl CopyCommand {
    pub fn new() -> Self {
        Self {
            term: Term::stderr(),
        } // TODO: Control over stderr or stdout
    }
}

impl OverwritableCommand for CopyCommand {
    type Params = CopyParams;

    fn term(&self) -> Term {
        self.term.clone()
    }

    fn from(&self, pcp: ProcessedCommandParams) -> Result<CopyParams, ValidationErrors> {
        let cp = Params {
            source: pcp.source,
            destination: pcp.destination,
            overwrite: pcp.overwrite,
        };
        cp.validate()?;
        Ok(Self::Params {
            source: cp.source.unwrap(),
            destination: cp.destination.unwrap(),
            overwrite: cp.overwrite,
        })
    }

    fn write_impl(&self, params: CopyParams) -> Result<(), SimpleError> {
        debug!(
            "copying '{}' to '{}'",
            params.source.display(),
            params.destination.display()
        );

        try_with!(copy(params.source, params.destination), "copy");
        Ok(())
    }
}
