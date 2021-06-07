use std::fmt::{self, Display};
use std::io;

use clap;

#[derive(Debug)]
pub struct BrixError {
    kind: BrixErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum BrixErrorKind {
    Io,
    Cli,
    Template,
}

impl From<io::Error> for BrixError {
    fn from(err: io::Error) -> BrixError {
        BrixError {
            kind: BrixErrorKind::Io,
            message: err.to_string(),
        }
    }
}

impl From<clap::Error> for BrixError {
    fn from(err: clap::Error) -> BrixError {
        BrixError {
            kind: BrixErrorKind::Cli,
            message: err.to_string(),
        }
    }
}

impl From<tinytemplate::error::Error> for BrixError {
    fn from(err: tinytemplate::error::Error) -> BrixError {
        BrixError {
            kind: BrixErrorKind::Template,
            message: err.to_string(),
        }
    }
}

impl Display for BrixErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let formatted = match self {
            Self::Io => "IO",
            Self::Cli => "CLI",
            Self::Template => "Template",
        };

        write!(fmt, "{}", formatted)
    }
}

impl Display for BrixError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{} error: {}", self.kind, self.message)
    }
}
