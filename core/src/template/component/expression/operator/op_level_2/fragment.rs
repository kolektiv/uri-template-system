use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Fragment
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Fragment<'t> {
    raw: &'t str,
}

impl<'t> Fragment<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static FRAGMENT_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('#'),
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::UnreservedAndReserved,
};

impl<'t> Fragment<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &FRAGMENT_BEHAVIOUR
    }
}
