#[cfg(feature = "iri-string")]
pub mod iri_string;
#[cfg(feature = "uritemplate-next")]
pub mod uri_template_next;
pub mod uri_template_system;

use std::fmt::Debug;

use crate::fixtures::Variable;

// =============================================================================
// Harnesses
// =============================================================================

// Traits

pub trait Harness
where
    Self::Values: Debug,
{
    type Values;

    fn prepare(&self, variables: Vec<(String, Variable)>) -> Self::Values;
    fn test(&self, template: &str, values: &Self::Values) -> String;
}
