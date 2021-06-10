use std::fs::copy;
use std::path::PathBuf;

use log::debug;
use simple_error::{try_with, SimpleError};
use validator::{Validate, ValidationErrors};

use crate::command::{OverwritableCommand, OverwritableParams, ProcessedCommandParams};

struct CopyParams {
    source: PathBuf,
    destination: PathBuf,
    overwrite: Option<bool>,
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
    #[validate(required)]
    overwrite: Option<bool>,
}

struct CopyCommand {}

impl OverwritableCommand for CopyCommand {
    type Params = CopyParams;
    fn validate(&self, pcp: ProcessedCommandParams) -> Result<CopyParams, ValidationErrors> {
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
