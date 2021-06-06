use std::process;

use colour::red_ln;

use brix_errors::BrixError;

pub fn error_and_quit(err: &str) {
    red_ln!(err);
    process::exit(2);
}

pub fn brix_error(err: BrixError) {
    red_ln!("{}", err);
    process::exit(2);
}
