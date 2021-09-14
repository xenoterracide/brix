// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::BrixError;
use crate::RawConfig;
use std::path::PathBuf;

mod yaml;
pub use yaml::YamlConfigParser;

pub trait ConfigParser {
    fn parse(&self, contents: &str) -> Result<RawConfig, BrixError>;
    fn matches(&self, path: &PathBuf) -> bool;
    fn extensions(&self) -> Vec<&str>;
}
