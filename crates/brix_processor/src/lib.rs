use tinytemplate::TinyTemplate;

mod context;
use context::Context;

use brix_errors::BrixError;

pub fn process(module: String, content: String) -> Result<String, BrixError> {
    let mut tt = TinyTemplate::new();
    tt.add_template(&module, &content)?;

    let result = tt.render(&module, &Context::new(module.clone()))?;
    Ok(result)
}
