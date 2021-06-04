use serde::Serialize;

#[derive(Serialize)]
pub struct Context {
    module: String,
}

impl Context {
    pub fn new(module: String) -> Self {
        Self { module }
    }
}
