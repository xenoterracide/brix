// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! # Brix Processor
//! Brix processor is a wrapper around [handlrbars](https://crates.io/crates/handlebars)
//! that allows for more complex context handling and adds custom helper functions.

use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
    RenderError,
};
use serde_json::json;
use serde_json::value::{Map, Value as Json};
use std::collections::HashMap;

use brix_errors::BrixError;
mod helpers;

/// Struct that contains an inner handlebars object with registered helpers.
/// May contain other templating engines in the future.
pub struct ProcessorCore<'a> {
    handlebars: handlebars::Handlebars<'a>,
}

impl<'a> ProcessorCore<'a> {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("to-upper", Box::new(helpers::ToUpperHelper));
        handlebars.register_helper("to-lower", Box::new(helpers::ToLowerHelper));
        handlebars.register_helper("to-title", Box::new(helpers::ToTitleHelper));
        handlebars.register_helper("to-case", Box::new(helpers::ToCaseHelper));
        handlebars.register_helper("to-flat", Box::new(helpers::ToFlatHelper));
        handlebars.register_helper("to-java-package", Box::new(helpers::ToJavaPackageHelper));
        handlebars.register_helper(
            "to-java-package-path",
            Box::new(helpers::ToJavaPackagePathHelper),
        );
        Self { handlebars }
    }

    /// Render text with the provided context.
    pub fn process(&self, text: String, context: Map<String, Json>) -> Result<String, BrixError> {
        let result = self.handlebars.render_template(&text, &context)?;
        Ok(result)
    }
}

/// Create a valid context map by serializing into JSON.
pub fn create_context(data: HashMap<String, String>) -> Map<String, Json> {
    let mut res = Map::new();
    for (key, value) in data.into_iter() {
        res.insert(key, json!(value));
    }
    res
}
