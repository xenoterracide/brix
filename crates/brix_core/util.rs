// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/// Utility function for displaying Windows paths more or less the same as Unix paths.
pub fn display_path(path: &str) -> String {
    let path = path.replace("//", "/");
    let path = path.replace("\\\\", "/");
    path.replace("\\", "/")
}
