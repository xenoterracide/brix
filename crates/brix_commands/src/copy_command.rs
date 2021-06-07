use brix_errors::BrixError;

use crate::command::Command;
use crate::command::ProcessedCommandParams;

struct CopyCommand;

impl CopyCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for CopyCommand {
    fn run(&self, params: ProcessedCommandParams) -> Result<(), BrixError> {
        // TODO: implementation
        Ok(())
    }
}
