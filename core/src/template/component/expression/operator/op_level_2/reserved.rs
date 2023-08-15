use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Reserved
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Reserved<'t> {
    raw: &'t str,
}

impl<'t> Reserved<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static RESERVED_BEHAVIOUR: Behaviour = Behaviour {
    first: None,
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::UnreservedAndReserved,
};

impl<'t> Reserved<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &RESERVED_BEHAVIOUR
    }
}
