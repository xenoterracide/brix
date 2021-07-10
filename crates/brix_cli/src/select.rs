use dialoguer::console::Term;
use dialoguer::Select;

use brix_errors::BrixError;

pub fn do_select(items: Vec<&str>) -> Result<usize, BrixError> {
    let selection = Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stdout())?;

    match selection {
        Some(index) => Ok(index),
        None => Err(BrixError::with("no option selected!")),
    }
}
