// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Contains the [ToUpperHelper], [ToLowerHelper], and [ToTitleHelper] helpers.

use crate::*;

/// Converts the specified text to all uppercase characters.
#[derive(Clone, Copy)]
pub struct ToUpperHelper;

impl HelperDef for ToUpperHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).ok_or(RenderError::new(
            "this function requires an argument to process",
        ))?;
        let rendered = param.value().render();

        out.write(&rendered.to_uppercase())?;
        Ok(())
    }
}

/// Converts the specified text to all lowercase characters.
#[derive(Clone, Copy)]
pub struct ToLowerHelper;

impl HelperDef for ToLowerHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).ok_or(RenderError::new(
            "this function requires an argument to process",
        ))?;
        let rendered = param.value().render();

        out.write(&rendered.to_lowercase())?;
        Ok(())
    }
}

/// Converts the specified text to title case.
#[derive(Clone, Copy)]
pub struct ToTitleHelper;

impl HelperDef for ToTitleHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).ok_or(RenderError::new(
            "this function requires an argument to process",
        ))?;
        let rendered = param.value().render();

        out.write(&titlecase::titlecase(&rendered))?;
        Ok(())
    }
}
