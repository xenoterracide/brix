use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub struct BrixError {
    kind: Option<BrixErrorKind>,
    message: String,
}

impl BrixError {
    pub fn with(error: &str) -> Self {
        Self {
            kind: None,
            message: String::from(error),
        }
    }
}

#[derive(Debug)]
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

impl From<tinytemplate::error::Error> for BrixError {
    fn from(err: tinytemplate::error::Error) -> BrixError {
        BrixError {
            kind: Some(BrixErrorKind::Template),
            message: err.to_string(),
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

impl From<regex::Error> for BrixError {
    fn from(err: regex::Error) -> BrixError {
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
