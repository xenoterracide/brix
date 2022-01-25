// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! YAML config parser.

use std::path::PathBuf;

use crate::parsers::ConfigParser;
use crate::BrixError;
use crate::RawConfig;

/// The config parser for .yaml and .yml files.
pub struct YamlConfigParser;

const SUPPORTED_EXTENSIONS: [&str; 2] = ["yaml", "yml"];

impl ConfigParser for YamlConfigParser {
    fn parse(&self, contents: &str) -> Result<RawConfig, BrixError> {
        let config: RawConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    fn matches(&self, path: &PathBuf) -> bool {
        let ext = path.extension().unwrap();
        SUPPORTED_EXTENSIONS.contains(&ext.to_str().unwrap())
    }

    fn extensions(&self) -> Vec<&str> {
        SUPPORTED_EXTENSIONS.to_vec()
    }
}
