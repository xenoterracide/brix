use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use dialoguer::console::Term;
use log::debug;
use validator::{Validate, ValidationErrors};

use crate::command::{OverwritableCommand, OverwritableParams, ProcessedCommandParams};
use brix_errors::BrixError;

#[derive(Debug)]
pub struct TemplateParams {
    source: PathBuf,
    destination: PathBuf,
    overwrite: Option<bool>,
    context: Option<HashMap<String, String>>,
}

impl PartialEq for TemplateParams {
    fn eq(&self, other: &Self) -> bool {
        return self.source == other.source
            && self.destination == other.destination
            && self.overwrite == other.overwrite
            && self.context == other.context;
    }
}

impl OverwritableParams for TemplateParams {
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

pub struct TemplateCommand {
    term: Term,
}

impl TemplateCommand {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            term: Term::stderr(),
        } // TODO: Control over stderr or stdout
    }
}

#[derive(Debug, Validate)]
struct Params {
    #[validate(required)]
    source: Option<PathBuf>,
    #[validate(required)]
    destination: Option<PathBuf>,
    overwrite: Option<bool>,
    context: Option<HashMap<String, String>>,
}

impl OverwritableCommand for TemplateCommand {
    type Params = TemplateParams;

    fn term(&self) -> Term {
        self.term.clone()
    }

    fn from(&self, pcp: ProcessedCommandParams) -> Result<TemplateParams, ValidationErrors> {
        let cp = Params {
            source: pcp.source,
            destination: pcp.destination,
            overwrite: pcp.overwrite,
            context: pcp.context,
        };
        cp.validate()?;
        Ok(Self::Params {
            source: cp.source.unwrap(),
            destination: cp.destination.unwrap(),
            overwrite: cp.overwrite,
            context: cp.context,
        })
    }

    fn write_impl(&self, params: TemplateParams) -> Result<(), BrixError> {
        let mut file = File::open(&params.source)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        debug!("templating '{}'", params.source.display());
        let context = params.context.unwrap();
        let processed_context = brix_processor::create_context(context);
        let result = brix_processor::process(contents, processed_context)?;

        std::fs::write(params.destination, result)?;

        Ok(())
    }

    fn name_inner(&self) -> String {
        String::from("template")
    }
}
