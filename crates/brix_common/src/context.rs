// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

type StdContext = HashMap<String, String>;

pub struct ContextMap {
    pub cli_positional: StdContext,
    pub config_global: StdContext,
    pub command_local: StdContext,
}

impl ContextMap {
    pub fn do_merge(&self) -> StdContext {
        let mut merged = HashMap::new();
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

pub fn cli_config_to_map(config: &brix_cli::Config) -> HashMap<String, String> {
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
