// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

/// Current type used for contexts.
type StdContext = HashMap<String, String>;

/// Struct containing all types of contexts used in commands.
pub struct ContextMap {
    pub cli_positional: StdContext,
    pub config_global: StdContext,
    pub command_local: StdContext,
}

impl ContextMap {
    /// Merges all contexts together into a single map respecting the priority of each.
    pub fn do_merge(&self) -> StdContext {
        let mut merged = StdContext::new();
        // Start with cli positional args
        for (key, value) in self.cli_positional.clone().into_iter() {
            merged.insert(key, value);
        }
        // Global is lowest priority
        for (key, value) in self.config_global.clone().into_iter() {
            merged.insert(key, value);
        }
        // Local overrides global context
        for (key, value) in self.command_local.clone().into_iter() {
            merged.insert(key, value);
        }

        merged
    }
}

/// Creates the initial context based off of CLI parameters.
pub fn cli_config_to_map(config: &brix_cli::Config) -> StdContext {
    macro_rules! s {
        ($st:expr) => {
            String::from($st)
        };
    }

    let mut map = HashMap::new();
    map.insert(s!("language"), s!(&config.language));
    map.insert(s!("module"), s!(&config.module));
    map.insert(s!("project"), s!(&config.project));
    map
}
