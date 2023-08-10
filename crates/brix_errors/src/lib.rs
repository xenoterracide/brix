// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! # Brix Errors
//! Common crate for the [BrixError] type used to help wrap and format many
//! different types of errors that may occur within Brix.

use std::fmt::{self, Display};
use std::io;

/// The Brix error, that supports an optional [BrixErrorKind] kind, and a message.
#[derive(Debug)]
pub struct BrixError {
    pub kind: Option<BrixErrorKind>,
    pub message: String,
}

impl BrixError {
    /// Construct an error with only a message.
    pub fn with(error: &str) -> Self {
        Self {
            kind: None,
            message: String::from(error),
        }
    }
}

/// Enum that lists all types of Brix errors.
#[derive(Debug, PartialEq, Eq)]
pub enum BrixErrorKind {
    Io,
    Cli,
    Template,
    Validation,
}

impl From<io::Error> for BrixError {
    fn from(err: io::Error) -> BrixError {
        BrixError {
            kind: Some(BrixErrorKind::Io),
            message: err.to_string(),
        }
    }
}

impl From<clap::Error> for BrixError {
    fn from(err: clap::Error) -> BrixError {
        BrixError {
            kind: Some(BrixErrorKind::Cli),
            message: err.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for BrixError {
    fn from(err: serde_yaml::Error) -> BrixError {
        BrixError {
            kind: None,
            message: format!("{}", err),
        }
    }
}

impl From<handlebars::RenderError> for BrixError {
    fn from(err: handlebars::RenderError) -> BrixError {
        BrixError {
            kind: Some(BrixErrorKind::Template),
            message: format!("{}", err),
        }
    }
}

impl From<validator::ValidationErrors> for BrixError {
    fn from(err: validator::ValidationErrors) -> BrixError {
        let mut message = String::new();
        for (field, _errors) in err.field_errors().into_iter() {
            message.push_str(&format!("\nField '{}' is required!", field))
        }

        BrixError {
            kind: Some(BrixErrorKind::Validation),
            message,
        }
    }
}

impl From<fancy_regex::Error> for BrixError {
    fn from(err: fancy_regex::Error) -> BrixError {
        BrixError {
            kind: None,
            message: format!("{}", err),
        }
    }
}

impl From<fs_extra::error::Error> for BrixError {
    fn from(err: fs_extra::error::Error) -> BrixError {
        BrixError {
            kind: None,
            message: format!("{}", err),
        }
    }
}

impl Display for BrixErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let formatted = match self {
            Self::Io => "IO",
            Self::Cli => "CLI",
            Self::Template => "Template",
            Self::Validation => "Validation",
        };

        write!(fmt, "{}", formatted)
    }
}

impl Display for BrixError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(kind) = &self.kind {
            write!(fmt, "{} error: {}", kind, self.message)
        } else {
            write!(fmt, "{}", self.message)
        }
    }
}
