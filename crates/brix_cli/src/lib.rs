use std::error::Error;
use std::process;

use colour::red_ln;

pub fn error_and_quit(err: &str) {
    red_ln!(err);
    process::exit(2);
}

pub fn std_error_and_quit(err: Box<dyn Error>) {
    red_ln!("{}", err);
    process::exit(2);
}
