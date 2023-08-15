use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Path
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Path<'t> {
    raw: &'t str,
}

impl<'t> Path<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static PATH_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('/'),
    sep: '/',
    named: false,
    ifemp: None,
    allow: Allow::Unreserved,
};

impl<'t> Path<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &PATH_BEHAVIOUR
    }
}
