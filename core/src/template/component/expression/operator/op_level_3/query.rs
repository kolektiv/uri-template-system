use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Query
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Query<'t> {
    raw: &'t str,
}

impl<'t> Query<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static QUERY_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('?'),
    sep: '&',
    named: true,
    ifemp: Some('='),
    allow: Allow::Unreserved,
};

impl<'t> Query<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &QUERY_BEHAVIOUR
    }
}
