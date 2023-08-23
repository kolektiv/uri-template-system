#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![allow(clippy::missing_errors_doc)] // TODO: Remove

mod string;
mod template;
mod value;

use std::fmt::{
    Error,
    Write,
};

use thiserror::Error;

// =============================================================================
// URI Template
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

#[derive(Debug, Error)]
pub enum ExpandError {
    #[error("formatting failed")]
    Format(#[from] Error),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{message} at position: {position}. expected: {expected}.")]
    UnexpectedInput {
        position: usize,
        message: String,
        expected: String,
    },
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use crate::{
    template::Template,
    value::{
        Value,
        Values,
    },
};
