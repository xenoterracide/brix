use crate::*;

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
