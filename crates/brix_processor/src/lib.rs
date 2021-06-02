use tinytemplate::TinyTemplate;

mod context;
use context::Context;

pub fn process(module: String, content: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template(&module, &content)?;

    let result = tt.render(&module, &Context::new(module.clone()))?;
    Ok(result)
}