#[derive(Debug)]
pub enum Command {
    /// Template and copy command (source, destination)
    TemplateAndCopy(String, String),
    Empty,
}
