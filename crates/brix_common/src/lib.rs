// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! # Brix Common
//! Common crate currently used for storing [AppContext] and other common functions in the future.

mod app_context;
pub mod context;

pub use app_context::AppContext;
