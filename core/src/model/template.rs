mod component;

use std::fmt::{
    self,
    Formatter,
};

use crate::{
    model::{
        template::component::Component,
        value::Values,
    },
    process::{
        expand::{
            Expand,
            Expansion,
        },
        parse::{
            ParseError,
            // ParseRef,
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
    /// Creates a new `Expansion` using the given `Values`, which may then be
    /// rendered using the `to_string` function provided by the `Display` trait.
    ///
    /// ```
    /// # use uri_template_system_core::{ Template, Values, Value };
    /// #
    /// let template = Template::parse("hello/{name}!").unwrap();
    /// let values = Values::default().add("name", Value::item("world"));
    ///
    /// assert_eq!("hello/world!", template.expand(&values).to_string());
    #[must_use]
    pub const fn expand<'e>(&'e self, values: &'e Values) -> Expansion<'e, 't> {
        Expansion::new(self, values)
    }

    /// Parses a string representing a potential template, and returns a new
    /// template instance if valid. See <https://datatracker.ietf.org/doc/html/rfc6570>
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
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> fmt::Result {
        self.components
            .iter()
            .try_for_each(|component| component.expand(values, f))
    }
}
