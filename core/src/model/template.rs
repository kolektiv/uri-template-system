mod component;

use std::fmt::Write;

use crate::{
    model::{
        template::component::Component,
        value::Values,
    },
    process::{
        expand::{
            Expand,
            ExpandError,
        },
        parse::{
            ParseError,
            TryParse,
        },
    },
};

// =============================================================================
// Template
// =============================================================================

// Types

/// The `Template` type is the basis for most simple tasks. Parsing and
/// expansion are both template functions.
#[derive(Debug, Eq, PartialEq)]
pub struct Template<'t> {
    components: Vec<Component<'t>>,
}

impl<'t> Template<'t> {
    /// Expands the template using the given `Values`, returning a string if
    /// expansion was successful.
    ///
    /// ```
    /// # use uri_template_system_core::{ Template, Values, Value };
    /// #
    /// let template = Template::parse("hello/{name}!").unwrap();
    /// let values = Values::default().add("name", Value::item("world"));
    ///
    /// assert_eq!("hello/world!", template.expand(&values).unwrap());
    pub fn expand(&self, values: &Values) -> Result<String, ExpandError> {
        let mut expanded = String::default();

        Expand::expand(self, values, &mut expanded)?;

        Ok(expanded)
    }

    /// Parses a string representing a potential template, and returns a new
    /// `Template` instance if valid. See <https://datatracker.ietf.org/doc/html/rfc6570>
    /// for the grammar of a valid URI Template. `uri-template-system` supports
    /// all operators and modifiers up-to and including Level 4.
    ///
    /// ```
    /// # use uri_template_system_core::Template;
    /// #
    /// let template = Template::parse("my/valid/{template}");
    ///
    /// assert!(template.is_ok());
    /// ```
    pub fn parse(raw: &'t str) -> Result<Self, ParseError> {
        Self::try_parse(raw, 0).map(|(_, template)| template)
    }

    const fn new(components: Vec<Component<'t>>) -> Self {
        Self { components }
    }
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Template<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        Vec::<Component<'t>>::try_parse(raw, global)
            .map(|(position, components)| (position, Self::new(components)))
    }
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Expand for Template<'t> {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        self.components
            .iter()
            .try_for_each(|component| component.expand(values, write))
    }
}
