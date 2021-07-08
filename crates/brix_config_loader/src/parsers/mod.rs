use crate::BrixError;
use crate::RawConfig;
use std::path::PathBuf;

mod yaml;
pub use yaml::YamlConfigParser;

pub trait ConfigParser {
    fn parse(&self, contents: &str) -> Result<RawConfig, BrixError>;
    fn matches(&self, path: &PathBuf) -> bool;
}
