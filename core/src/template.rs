pub mod component;

use std::fmt::{
    Error,
    Write,
};

use thiserror::Error;

use crate::{
    template::component::Component,
    value::Values,
};

// =============================================================================
// Template
// =============================================================================

// Traits

trait Expand {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError>;
}

trait Parse<'t>
where
    Self: Sized,
{
    fn parse(raw: &'t str, global: usize) -> (usize, Self);
}

trait TryParse<'t>
where
    Self: Sized,
{
    fn try_parse(raw: &'t str, base: usize) -> Result<(usize, Self), ParseError>;
}

// -----------------------------------------------------------------------------

// Errors

/// An [`Error`](std::error::Error) compatible type which may be the result of a
/// failure of [`Template::expand`] (given a valid [`Template`] and provided
/// [`Values`]).
#[derive(Debug, Error)]
pub enum ExpandError {
    /// Formatting for this expansion failed due to an internal error in
    /// [`std::fmt::Write`], which is not recoverable.
    #[error("formatting failed")]
    Format(#[from] Error),
}

/// An [`Error`](std::error::Error) compatible type which may be the result of a
/// failure of [`Template::parse`], likely due to an invalid URI Template format
/// (as defined by the grammar given in [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570)).
#[derive(Debug, Error)]
pub enum ParseError {
    /// The input given contained an unexpected value according to the URI
    /// Template value grammar, causing parsing to fail. See the grammar at
    /// [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570) for the definition of a
    /// valid URI Template.
    #[error("{message} at position: {position}. expected: {expected}.")]
    UnexpectedInput {
        /// The position (in bytes) of the input at which the unexpected input
        /// occurs.
        position: usize,
        /// A message giving more detail about which grammatical element failed
        /// to parse the given input.
        message: String,
        /// An indication of what (valid) input was expected by the parser.
        expected: String,
    },
}

// -----------------------------------------------------------------------------

// Types

/// The [`Template`] type is the basis for most simple tasks. Parsing and
/// expansion are both template functions.
#[derive(Debug, Eq, PartialEq)]
pub struct Template<'t> {
    components: Vec<Component<'t>>,
}

impl<'t> Template<'t> {
    /// Expands the template using the given [`Values`], returning a [`String`]
    /// if expansion was successful.
    ///
    /// # Errors
    ///
    /// This function may fail due to internal formatting errors
    /// ([`std::fmt::Write`] is an abstraction which allows for underlying
    /// failures) though this is very unlikely given [`String`] output.
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

    /// Parses a [`&str`] representing a potential template, and returns a new
    /// [`Template`] instance if valid. See [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570)
    /// for the grammar of a valid URI Template. `uri-template-system` supports
    /// all operators and modifiers up-to and including Level 4.
    ///
    /// # Errors
    ///
    /// This function may fail when the given input is not a valid URI Template
    /// according the RFC-defined grammar. The resultant [`ParseError`]
    /// should give useful information about where the parser failed.
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
