use handlebars::Handlebars;
use serde_json::json;
use serde_json::value::{Map, Value as Json};
use std::collections::HashMap;

use brix_errors::BrixError;

pub fn process(text: String, context: Map<String, Json>) -> Result<String, BrixError> {
    let handlebars = Handlebars::new();
    let result = handlebars.render_template(&text, &context)?;
    Ok(result)
}

pub fn create_context(data: HashMap<String, String>) -> Map<String, Json> {
    let mut res = Map::new();
    for (key, value) in data.into_iter() {
        res.insert(key, json!(value));
    }
    res
}
