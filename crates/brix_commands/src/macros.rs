// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[macro_export]
macro_rules! dir {
    ($config: expr, $path: expr) => {
        $config.join($path)
    };
}
