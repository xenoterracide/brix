// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Macros relating to printing to the console.

#[allow(unused_imports)]
use colored::*;

#[macro_export]
/// Macro very similar to `eprintln!` in the standard library, just prints in red.
macro_rules! error {
    ($start: expr, $($args:tt)*) => {{
        eprintln!("{}", format!($start, $($args)*).red())
    }};
}
