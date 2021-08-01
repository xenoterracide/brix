use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
    RenderError,
};
use serde_json::json;
use serde_json::value::{Map, Value as Json};
use std::collections::HashMap;

use brix_errors::BrixError;
mod helpers;

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
        Self { handlebars }
    }

    pub fn process(&self, text: String, context: Map<String, Json>) -> Result<String, BrixError> {
        let result = self.handlebars.render_template(&text, &context)?;
        Ok(result)
    }
}

pub fn create_context(data: HashMap<String, String>) -> Map<String, Json> {
    let mut res = Map::new();
    for (key, value) in data.into_iter() {
        res.insert(key, json!(value));
    }
    res
}
