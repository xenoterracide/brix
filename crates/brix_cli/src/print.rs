// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[allow(unused_imports)]
use colored::*;

#[macro_export]
macro_rules! error {
    ($start: expr, $($args:tt)*) => {{
        eprintln!("{}", format!($start, $($args)*).red())
    }};
}
