use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Path Parameter
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct PathParameter<'t> {
    raw: &'t str,
}

impl<'t> PathParameter<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static PATH_PARAMETER_BEHAVIOUR: Behaviour = Behaviour {
    first: Some(';'),
    sep: ';',
    named: true,
    ifemp: None,
    allow: Allow::Unreserved,
};

impl<'t> PathParameter<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &PATH_PARAMETER_BEHAVIOUR
    }
}
