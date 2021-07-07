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
