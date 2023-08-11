// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Contains the [ToUpperHelper], [ToLowerHelper], and [ToTitleHelper] helpers.

use crate::*;
use convert_case::{Boundary, Case, Casing, Converter, Pattern};

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

#[derive(Clone, Copy)]
pub struct ToFlatHelper;

impl HelperDef for ToFlatHelper {
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

        out.write(&rendered.to_case(Case::Flat))?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct ToJavaPackageHelper;

impl HelperDef for ToJavaPackageHelper {
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

        let conv = Converter::new()
            .set_pattern(Pattern::Camel)
            .remove_boundaries(&[
                Boundary::UpperDigit,
                Boundary::LowerDigit,
                Boundary::DigitUpper,
                Boundary::DigitLower,
            ])
            .set_delim(".");

        out.write(&conv.convert(rendered).to_lowercase())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct ToJavaPackagePathHelper;

impl HelperDef for ToJavaPackagePathHelper {
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

        let conv = Converter::new()
            .set_pattern(Pattern::Camel)
            .remove_boundaries(&[
                Boundary::UpperDigit,
                Boundary::LowerDigit,
                Boundary::DigitUpper,
                Boundary::DigitLower,
            ])
            .set_delim("/");

        out.write(&conv.convert(rendered).to_lowercase())?;
        Ok(())
    }
}
