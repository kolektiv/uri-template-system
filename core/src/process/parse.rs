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
