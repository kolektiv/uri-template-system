use thiserror::Error;

// =============================================================================
// Parse
// =============================================================================

// Traits

pub trait Parse<'t>
where
    Self: Sized,
{
    fn parse(raw: &'t str, global: usize) -> (usize, Self);
}

#[allow(clippy::module_name_repetitions)]
pub trait TryParse<'t>
where
    Self: Sized,
{
    fn try_parse(raw: &'t str, base: usize) -> Result<(usize, Self), ParseError>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseRef<'t> {
    pub start: usize,
    pub end: usize,
    pub raw: &'t str,
}

impl<'t> ParseRef<'t> {
    pub const fn new(start: usize, end: usize, raw: &'t str) -> Self {
        Self { start, end, raw }
    }
}

// -----------------------------------------------------------------------------

// Errors

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{message} at position: {position}. expected: {expected}.")]
    UnexpectedInput {
        position: usize,
        message: String,
        expected: String,
    },
}
