mod common;
mod expression;
mod literal;
mod template;

use anyhow::Error;

use crate::template::Template;

// =============================================================================
// URI Template
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct URITemplate {
    _template: Template,
}

impl URITemplate {
    pub fn parse(input: impl Into<String>) -> Result<Self, Error> {
        let template = template::parse(&input.into())
            .map(|(_, template)| template)
            .map_err(|_| Error::msg("uri template parse failed"))?; // TODO: Proper Error

        Ok(Self {
            _template: template,
        })
    }
}
