use std::path::PathBuf;

use crate::parsers::ConfigParser;
use crate::BrixError;
use crate::RawConfig;

pub struct YamlConfigParser;

const SUPPORTED_EXTENSIONS: [&str; 2] = ["yaml", "yml"];

impl ConfigParser for YamlConfigParser {
    fn parse(&self, contents: &str) -> Result<RawConfig, BrixError> {
        let config: RawConfig = serde_yaml::from_str(&contents).unwrap(); // TODO: Better handling
        Ok(config)
    }

    fn matches(&self, path: &PathBuf) -> bool {
        let ext = path.extension().unwrap();
        SUPPORTED_EXTENSIONS.contains(&ext.to_str().unwrap())
    }
}
