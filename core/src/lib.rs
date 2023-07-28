mod common;
mod expression;
mod literal;
mod template;
mod value;

use anyhow::{
    Error,
    Result,
};

use crate::template::Template;

// =============================================================================
// URI Template
// =============================================================================

// Traits

trait Expand {
    type Context;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()>;
}

// -----------------------------------------------------------------------------

// Types

#[derive(Debug, PartialEq)]
pub struct URITemplate {
    template: Template,
}

impl URITemplate {
    pub fn parse(input: impl Into<String>) -> Result<Self> {
        let template = template::parse(&input.into())
            .map(|(_, template)| template)
            .map_err(|_| Error::msg("uri template parse failed"))?; // TODO: Proper Error

        Ok(Self { template: template })
    }

    pub fn expand(&self, values: &Values) -> Result<String> {
        let mut output = String::new();
        let _ = self.template.expand(&mut output, values, &())?;

        Ok(output)
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::value::{
    Value,
    Values,
};
