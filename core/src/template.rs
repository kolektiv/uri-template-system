pub mod expand;
pub mod parse;

use crate::{
    template::{
        expand::{
            Expand,
            ExpandError,
        },
        parse::{
            ParseError,
            TryParse,
        },
    },
    value::Values,
};

// =============================================================================
// Template
// =============================================================================

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

// Component

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'t> {
    Literal(Literal<'t>),
    Expression(Expression<'t>),
}

// -----------------------------------------------------------------------------

// Expression

#[derive(Debug, Eq, PartialEq)]
pub struct Expression<'t> {
    pub operator: Option<Operator>,
    pub variable_list: VariableList<'t>,
}

impl<'t> Expression<'t> {
    pub const fn new(operator: Option<Operator>, variable_list: VariableList<'t>) -> Self {
        Self {
            operator,
            variable_list,
        }
    }
}

// -----------------------------------------------------------------------------

// Operator

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    Level2(OpLevel2),
    Level3(OpLevel3),
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel2 {
    Fragment,
    Reserved,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel3 {
    Label,
    Path,
    PathParameter,
    Query,
    QueryContinuation,
}

// -----------------------------------------------------------------------------

// Variables

pub type VariableList<'t> = Vec<VariableSpecification<'t>>;

pub type VariableSpecification<'t> = (VariableName<'t>, Option<Modifier>);

#[derive(Debug, Eq, PartialEq)]
pub struct VariableName<'t> {
    name: &'t str,
}

impl<'t> VariableName<'t> {
    pub const fn new(name: &'t str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &str {
        self.name
    }
}

// -----------------------------------------------------------------------------

// Modifier

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier {
    Explode,
    Prefix(usize),
}

// -----------------------------------------------------------------------------

// Literal

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<'t> {
    pub value: &'t str,
}

impl<'t> Literal<'t> {
    pub const fn new(value: &'t str) -> Self {
        Self { value }
    }
}
