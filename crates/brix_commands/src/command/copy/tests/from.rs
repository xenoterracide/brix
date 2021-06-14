#[cfg(test)]
mod params {
    use std::path::PathBuf;

    use dialoguer::console::Term;
    use spectral::assert_that;
    use spectral::result::ResultAssertions;

    use crate::command::copy::OverwritableCommand;
    use crate::{CopyCommand, ProcessedCommandParams};

    #[test]
    fn valid() {
        let command = CopyCommand {
            term: Term::stdout(),
        };
        let params = ProcessedCommandParams {
            source: Option::Some(PathBuf::new()),
            destination: Option::Some(PathBuf::new()),
            overwrite: None,
            search: None,
            replace: None,
            context: None,
        };
        assert_that!(command.from(params)).is_ok();
    }

    #[test]
    fn invalid() {
        let command = CopyCommand {
            term: Term::stdout(),
        };

        let params = ProcessedCommandParams {
            source: None,
            destination: None,
            overwrite: None,
            search: None,
            replace: None,
            context: None,
        };

        let error = command.from(params).err().unwrap();
        let errors = error.into_errors();
        let keys = errors.keys();
        let mut vec = keys.collect::<Vec<_>>();

        vec.sort();

        assert_eq!(vec, [&"destination", &"source"]);
    }
}
