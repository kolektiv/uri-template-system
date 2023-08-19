use std::fmt::{
    Error,
    Write,
};

use thiserror::Error;

use crate::model::value::Values;

// =============================================================================
// Expand
// =============================================================================

// Traits

pub trait Expand {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError>;
}

// -----------------------------------------------------------------------------

// Errors

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum ExpandError {
    #[error("formatting failed")]
    Format(#[from] Error),
}
