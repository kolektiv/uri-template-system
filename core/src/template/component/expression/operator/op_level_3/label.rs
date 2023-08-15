use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Label
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Label<'t> {
    raw: &'t str,
}

impl<'t> Label<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static LABEL_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('.'),
    sep: '.',
    named: false,
    ifemp: None,
    allow: Allow::Unreserved,
};

impl<'t> Label<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &LABEL_BEHAVIOUR
    }
}
