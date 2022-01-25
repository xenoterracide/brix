// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Contains the [ToCaseHelper] helper.

use crate::*;
use convert_case::{Case, Casing};

/// Helper that is used to convert a parameter from one case to another.
/// Supports the following cases:
/// - toggle
/// - pascal
/// - camel
/// - upper-camel
/// - snake
/// - upper-snake
/// - screaming-snake
/// - kebab
/// - cobol
/// - train
/// - flat
/// - upper-flat
/// - alternating
#[derive(Clone, Copy)]
pub struct ToCaseHelper;

impl HelperDef for ToCaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let case_type = h.param(0).ok_or(RenderError::new(
            "case type (e.g. 'snake' or 'camel') not specified",
        ))?;
        // In case user wants to specify their case in the actual case itself
        let modified_case_type = case_type.render().to_case(Case::Kebab);

        let case = match modified_case_type.as_str() {
            "toggle" => Ok(Case::Toggle),
            "pascal" => Ok(Case::Pascal),
            "camel" => Ok(Case::Camel),
            "upper-camel" => Ok(Case::UpperCamel),
            "snake" => Ok(Case::Snake),
            "upper-snake" => Ok(Case::UpperSnake),
            "screaming-snake" => Ok(Case::ScreamingSnake),
            "kebab" => Ok(Case::Kebab),
            "cobol" => Ok(Case::Cobol),
            "train" => Ok(Case::Train),
            "flat" => Ok(Case::Flat),
            "upper-flat" => Ok(Case::UpperFlat),
            "alternating" => Ok(Case::Alternating),
            "" => Err(RenderError::new(
                "try putting the name of your desired case in quotation marks e.g. \"snake\"",
            )),
            _ => Err(RenderError::new(format!(
                "case '{}' not supported",
                case_type.render()
            ))),
        }?;

        let value = h
            .param(1)
            .ok_or(RenderError::new("argument to process not specified"))?;

        let rendered = value.value().render();
        let output = rendered.to_case(case);

        out.write(&output)?;
        Ok(())
    }
}
